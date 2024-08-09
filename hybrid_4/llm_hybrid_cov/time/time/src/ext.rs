//! Extension traits.
use core::time::Duration as StdDuration;
use crate::convert::*;
use crate::Duration;
/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}
    impl Sealed for i64 {}
    impl Sealed for u64 {}
    impl Sealed for f64 {}
}
/// Create [`Duration`]s from numeric literals.
///
/// # Examples
///
/// Basic construction of [`Duration`]s.
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
/// assert_eq!(5.microseconds(), Duration::microseconds(5));
/// assert_eq!(5.milliseconds(), Duration::milliseconds(5));
/// assert_eq!(5.seconds(), Duration::seconds(5));
/// assert_eq!(5.minutes(), Duration::minutes(5));
/// assert_eq!(5.hours(), Duration::hours(5));
/// assert_eq!(5.days(), Duration::days(5));
/// assert_eq!(5.weeks(), Duration::weeks(5));
/// ```
///
/// Signed integers work as well!
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Duration::microseconds(-5));
/// assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
/// assert_eq!((-5).seconds(), Duration::seconds(-5));
/// assert_eq!((-5).minutes(), Duration::minutes(-5));
/// assert_eq!((-5).hours(), Duration::hours(-5));
/// assert_eq!((-5).days(), Duration::days(-5));
/// assert_eq!((-5).weeks(), Duration::weeks(-5));
/// ```
///
/// Just like any other [`Duration`], they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalDuration;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point value will be
/// truncated. Keep in mind that floating point numbers are inherently imprecise and have limited
/// capacity.
pub trait NumericalDuration: sealed::Sealed {
    /// Create a [`Duration`] from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a [`Duration`] from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a [`Duration`] from the number of hours.
    fn hours(self) -> Duration;
    /// Create a [`Duration`] from the number of days.
    fn days(self) -> Duration;
    /// Create a [`Duration`] from the number of weeks.
    fn weeks(self) -> Duration;
}
impl NumericalDuration for i64 {
    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self)
    }
    fn microseconds(self) -> Duration {
        Duration::microseconds(self)
    }
    fn milliseconds(self) -> Duration {
        Duration::milliseconds(self)
    }
    fn seconds(self) -> Duration {
        Duration::seconds(self)
    }
    fn minutes(self) -> Duration {
        Duration::minutes(self)
    }
    fn hours(self) -> Duration {
        Duration::hours(self)
    }
    fn days(self) -> Duration {
        Duration::days(self)
    }
    fn weeks(self) -> Duration {
        Duration::weeks(self)
    }
}
impl NumericalDuration for f64 {
    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self as _)
    }
    fn microseconds(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Microsecond) as Self) as _)
    }
    fn milliseconds(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Millisecond) as Self) as _)
    }
    fn seconds(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Second) as Self) as _)
    }
    fn minutes(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Minute) as Self) as _)
    }
    fn hours(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Hour) as Self) as _)
    }
    fn days(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Day) as Self) as _)
    }
    fn weeks(self) -> Duration {
        Duration::nanoseconds((self * Nanosecond.per(Week) as Self) as _)
    }
}
/// Create [`std::time::Duration`]s from numeric literals.
///
/// # Examples
///
/// Basic construction of [`std::time::Duration`]s.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// # use core::time::Duration;
/// assert_eq!(5.std_nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.std_microseconds(), Duration::from_micros(5));
/// assert_eq!(5.std_milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.std_seconds(), Duration::from_secs(5));
/// assert_eq!(5.std_minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.std_hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.std_days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.std_weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other [`std::time::Duration`], they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// assert_eq!(
///     2.std_seconds() + 500.std_milliseconds(),
///     2_500.std_milliseconds()
/// );
/// assert_eq!(
///     2.std_seconds() - 500.std_milliseconds(),
///     1_500.std_milliseconds()
/// );
/// ```
///
/// When called on floating point values, any remainder of the floating point value will be
/// truncated. Keep in mind that floating point numbers are inherently imprecise and have limited
/// capacity.
pub trait NumericalStdDuration: sealed::Sealed {
    /// Create a [`std::time::Duration`] from the number of nanoseconds.
    fn std_nanoseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of microseconds.
    fn std_microseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of milliseconds.
    fn std_milliseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of seconds.
    fn std_seconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of minutes.
    fn std_minutes(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of hours.
    fn std_hours(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of days.
    fn std_days(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of weeks.
    fn std_weeks(self) -> StdDuration;
}
impl NumericalStdDuration for u64 {
    fn std_nanoseconds(self) -> StdDuration {
        StdDuration::from_nanos(self)
    }
    fn std_microseconds(self) -> StdDuration {
        StdDuration::from_micros(self)
    }
    fn std_milliseconds(self) -> StdDuration {
        StdDuration::from_millis(self)
    }
    fn std_seconds(self) -> StdDuration {
        StdDuration::from_secs(self)
    }
    fn std_minutes(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Minute) as Self)
    }
    fn std_hours(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Hour) as Self)
    }
    fn std_days(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Day) as Self)
    }
    fn std_weeks(self) -> StdDuration {
        StdDuration::from_secs(self * Second.per(Week) as Self)
    }
}
impl NumericalStdDuration for f64 {
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as _)
    }
    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Microsecond) as Self) as _)
    }
    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Millisecond) as Self) as _)
    }
    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Second) as Self) as _)
    }
    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Minute) as Self) as _)
    }
    fn std_hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Hour) as Self) as _)
    }
    fn std_days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Day) as Self) as _)
    }
    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * Nanosecond.per(Week) as Self) as _)
    }
}
#[cfg(test)]
mod tests_llm_16_82 {
    use crate::ext::NumericalDuration;
    use crate::Duration;
    #[test]
    fn days_trait_for_f64() {
        let _rug_st_tests_llm_16_82_rrrruuuugggg_days_trait_for_f64 = 0;
        let rug_fuzz_0 = 1.0_f64;
        let rug_fuzz_1 = 0.0_f64;
        let rug_fuzz_2 = 1.0_f64;
        let rug_fuzz_3 = 0.0000001_f64;
        let rug_fuzz_4 = 3.5_f64;
        let rug_fuzz_5 = 86_400.0;
        let duration_from_trait = (rug_fuzz_0).days();
        let expected_duration = Duration::DAY;
        debug_assert_eq!(duration_from_trait, expected_duration);
        let duration_from_trait_zero = (rug_fuzz_1).days();
        debug_assert!(duration_from_trait_zero.is_zero());
        let duration_from_trait_negative = (-rug_fuzz_2).days();
        debug_assert!(duration_from_trait_negative.is_negative());
        let duration_from_trait_positive = (rug_fuzz_3).days();
        debug_assert!(duration_from_trait_positive.is_positive());
        let duration_from_trait_max = (f64::MAX).days();
        debug_assert_eq!(duration_from_trait_max, Duration::MAX);
        let duration_from_trait_min = (f64::MIN).days();
        debug_assert_eq!(duration_from_trait_min, Duration::MIN);
        let days = rug_fuzz_4;
        let expected_seconds = days * rug_fuzz_5;
        let duration_from_trait_random = days.days();
        let duration_as_seconds = duration_from_trait_random.as_seconds_f64();
        debug_assert_eq!(duration_as_seconds, expected_seconds);
        let _rug_ed_tests_llm_16_82_rrrruuuugggg_days_trait_for_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_83 {
    use crate::ext::NumericalDuration;
    use crate::Duration;
    #[test]
    fn one_hour() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_one_hour = 0;
        let rug_fuzz_0 = 1.0f64;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::seconds(3600));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_one_hour = 0;
    }
    #[test]
    fn half_hour() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_half_hour = 0;
        let rug_fuzz_0 = 0.5f64;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::seconds(1800));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_half_hour = 0;
    }
    #[test]
    fn zero_hour() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_zero_hour = 0;
        let rug_fuzz_0 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::seconds(0));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_zero_hour = 0;
    }
    #[test]
    fn negative_hour() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_negative_hour = 0;
        let rug_fuzz_0 = 1.0f64;
        debug_assert_eq!((- rug_fuzz_0).hours(), Duration::seconds(- 3600));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_negative_hour = 0;
    }
    #[test]
    fn multiple_hours() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_multiple_hours = 0;
        let rug_fuzz_0 = 2.5f64;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::seconds(9000));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_multiple_hours = 0;
    }
    #[test]
    fn fractional_hour() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_fractional_hour = 0;
        let rug_fuzz_0 = 1.75f64;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::seconds(6300));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_fractional_hour = 0;
    }
    #[test]
    fn large_hour() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_large_hour = 0;
        let rug_fuzz_0 = 1_000_000.0f64;
        let hours = rug_fuzz_0;
        debug_assert_eq!(hours.hours(), Duration::seconds(hours as i64 * 3600));
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_large_hour = 0;
    }
    #[test]
    #[should_panic(expected = "overflow when adding durations")]
    fn hours_overflow() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_hours_overflow = 0;
        let hours = f64::MAX;
        let _ = hours.hours();
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_hours_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "overflow when adding durations")]
    fn hours_negative_overflow() {
        let _rug_st_tests_llm_16_83_rrrruuuugggg_hours_negative_overflow = 0;
        let hours = f64::MIN;
        let _ = hours.hours();
        let _rug_ed_tests_llm_16_83_rrrruuuugggg_hours_negative_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_84 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn microseconds_positive() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_microseconds_positive = 0;
        let rug_fuzz_0 = 0.5f64;
        let rug_fuzz_1 = 1.0f64;
        let rug_fuzz_2 = 1.5f64;
        debug_assert_eq!(rug_fuzz_0.microseconds(), Duration::nanoseconds(500));
        debug_assert_eq!(rug_fuzz_1.microseconds(), Duration::nanoseconds(1000));
        debug_assert_eq!(rug_fuzz_2.microseconds(), Duration::nanoseconds(1500));
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_microseconds_positive = 0;
    }
    #[test]
    fn microseconds_negative() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_microseconds_negative = 0;
        let rug_fuzz_0 = 0.5f64;
        let rug_fuzz_1 = 1.0f64;
        let rug_fuzz_2 = 1.5f64;
        debug_assert_eq!((- rug_fuzz_0).microseconds(), Duration::nanoseconds(- 500));
        debug_assert_eq!((- rug_fuzz_1).microseconds(), Duration::nanoseconds(- 1000));
        debug_assert_eq!((- rug_fuzz_2).microseconds(), Duration::nanoseconds(- 1500));
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_microseconds_negative = 0;
    }
    #[test]
    fn microseconds_edge_cases() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_microseconds_edge_cases = 0;
        debug_assert_eq!(f64::INFINITY.microseconds(), Duration::MAX);
        debug_assert_eq!(f64::NEG_INFINITY.microseconds(), Duration::MIN);
        debug_assert!(f64::NAN.microseconds().is_zero());
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_microseconds_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_85 {
    use super::*;
    use crate::*;
    use std::convert::TryInto;
    #[test]
    fn milliseconds_positive() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_positive = 0;
        let rug_fuzz_0 = 1500.0_f64;
        let milliseconds = rug_fuzz_0;
        let duration = milliseconds.milliseconds();
        debug_assert_eq!(duration.whole_seconds(), 1);
        debug_assert_eq!(duration.subsec_milliseconds(), 500);
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_positive = 0;
    }
    #[test]
    fn milliseconds_negative() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_negative = 0;
        let rug_fuzz_0 = 1500.0_f64;
        let milliseconds = -rug_fuzz_0;
        let duration = milliseconds.milliseconds();
        debug_assert_eq!(duration.whole_seconds(), - 2);
        debug_assert_eq!(duration.subsec_milliseconds(), - 500);
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_negative = 0;
    }
    #[test]
    fn milliseconds_zero() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_zero = 0;
        let rug_fuzz_0 = 0.0_f64;
        let milliseconds = rug_fuzz_0;
        let duration = milliseconds.milliseconds();
        debug_assert!(duration.is_zero());
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_zero = 0;
    }
    #[test]
    fn milliseconds_fraction_positive() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_fraction_positive = 0;
        let rug_fuzz_0 = 0.7_f64;
        let milliseconds = rug_fuzz_0;
        let duration = milliseconds.milliseconds();
        let expected_nanos = (milliseconds * (Nanosecond.per(Millisecond) as f64))
            as i32;
        debug_assert_eq!(duration.subsec_nanoseconds(), expected_nanos);
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_fraction_positive = 0;
    }
    #[test]
    fn milliseconds_fraction_negative() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_fraction_negative = 0;
        let rug_fuzz_0 = 0.7_f64;
        let milliseconds = -rug_fuzz_0;
        let duration = milliseconds.milliseconds();
        let expected_nanos = (milliseconds * (Nanosecond.per(Millisecond) as f64))
            as i32;
        debug_assert_eq!(duration.subsec_nanoseconds(), expected_nanos);
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_fraction_negative = 0;
    }
    #[test]
    fn milliseconds_max_value() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_max_value = 0;
        let milliseconds = f64::MAX;
        let duration = milliseconds.milliseconds();
        debug_assert_eq!(duration, Duration::MAX);
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_max_value = 0;
    }
    #[test]
    fn milliseconds_min_value() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_milliseconds_min_value = 0;
        let milliseconds = f64::MIN;
        let duration = milliseconds.milliseconds();
        debug_assert!(duration.is_negative());
        debug_assert_eq!(duration, Duration::MIN);
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_milliseconds_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_87 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn nanoseconds_positive() {
        let _rug_st_tests_llm_16_87_rrrruuuugggg_nanoseconds_positive = 0;
        let rug_fuzz_0 = 42.0f64;
        let duration = rug_fuzz_0.nanoseconds();
        debug_assert_eq!(duration, Duration::nanoseconds(42));
        let _rug_ed_tests_llm_16_87_rrrruuuugggg_nanoseconds_positive = 0;
    }
    #[test]
    fn nanoseconds_negative() {
        let _rug_st_tests_llm_16_87_rrrruuuugggg_nanoseconds_negative = 0;
        let rug_fuzz_0 = 42.0f64;
        let duration = (-rug_fuzz_0).nanoseconds();
        debug_assert_eq!(duration, Duration::nanoseconds(- 42));
        let _rug_ed_tests_llm_16_87_rrrruuuugggg_nanoseconds_negative = 0;
    }
    #[test]
    fn nanoseconds_zero() {
        let _rug_st_tests_llm_16_87_rrrruuuugggg_nanoseconds_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        let duration = rug_fuzz_0.nanoseconds();
        debug_assert_eq!(duration, Duration::nanoseconds(0));
        let _rug_ed_tests_llm_16_87_rrrruuuugggg_nanoseconds_zero = 0;
    }
    #[test]
    fn nanoseconds_fractional() {
        let _rug_st_tests_llm_16_87_rrrruuuugggg_nanoseconds_fractional = 0;
        let rug_fuzz_0 = 42.7f64;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 10;
        let duration = rug_fuzz_0.nanoseconds();
        let expected = Duration::nanoseconds(rug_fuzz_1)
            + Duration::nanoseconds(rug_fuzz_2) / rug_fuzz_3;
        debug_assert_eq!(duration, expected);
        let _rug_ed_tests_llm_16_87_rrrruuuugggg_nanoseconds_fractional = 0;
    }
    #[test]
    fn nanoseconds_large() {
        let _rug_st_tests_llm_16_87_rrrruuuugggg_nanoseconds_large = 0;
        let rug_fuzz_0 = 1e9f64;
        let duration = rug_fuzz_0.nanoseconds();
        debug_assert_eq!(duration, Duration::seconds(1));
        let _rug_ed_tests_llm_16_87_rrrruuuugggg_nanoseconds_large = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_88 {
    use super::*;
    use crate::*;
    use crate::{ext::NumericalDuration, Duration};
    #[test]
    fn seconds_positive() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_seconds_positive = 0;
        let rug_fuzz_0 = 5.0f64;
        debug_assert_eq!(rug_fuzz_0.seconds(), Duration::seconds(5));
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_seconds_positive = 0;
    }
    #[test]
    fn seconds_negative() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_seconds_negative = 0;
        let rug_fuzz_0 = 5.0f64;
        debug_assert_eq!((- rug_fuzz_0).seconds(), Duration::seconds(- 5));
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_seconds_negative = 0;
    }
    #[test]
    fn seconds_zero() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_seconds_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.seconds(), Duration::ZERO);
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_seconds_zero = 0;
    }
    #[test]
    fn seconds_fractional() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_seconds_fractional = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 500;
        let rug_fuzz_2 = 1.5f64;
        let expected = Duration::seconds(rug_fuzz_0)
            + Duration::milliseconds(rug_fuzz_1);
        debug_assert_eq!(rug_fuzz_2.seconds(), expected);
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_seconds_fractional = 0;
    }
    #[test]
    #[should_panic(expected = "overflow when adding durations")]
    fn seconds_overflow() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_seconds_overflow = 0;
        let _ = f64::MAX.seconds();
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_seconds_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "overflow when adding durations")]
    fn seconds_underflow() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_seconds_underflow = 0;
        let _ = f64::MIN.seconds();
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_seconds_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use super::*;
    use crate::*;
    use crate::{ext::NumericalDuration, Duration};
    #[test]
    fn test_weeks_positive() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_test_weeks_positive = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 0.5f64;
        let rug_fuzz_2 = 2.0f64;
        debug_assert_eq!(rug_fuzz_0.weeks(), Duration::WEEK);
        debug_assert_eq!(rug_fuzz_1.weeks(), Duration::DAY * 3.5);
        debug_assert_eq!(rug_fuzz_2.weeks(), Duration::WEEK * 2);
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_test_weeks_positive = 0;
    }
    #[test]
    fn test_weeks_negative() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_test_weeks_negative = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 0.5f64;
        let rug_fuzz_2 = 2.0f64;
        debug_assert_eq!((- rug_fuzz_0).weeks(), Duration::WEEK * - 1);
        debug_assert_eq!((- rug_fuzz_1).weeks(), Duration::DAY * - 3.5);
        debug_assert_eq!((- rug_fuzz_2).weeks(), Duration::WEEK * - 2);
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_test_weeks_negative = 0;
    }
    #[test]
    fn test_weeks_zero() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_test_weeks_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.weeks(), Duration::ZERO);
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_test_weeks_zero = 0;
    }
    #[test]
    fn test_weeks_fractional() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_test_weeks_fractional = 0;
        let rug_fuzz_0 = 1.5f64;
        let rug_fuzz_1 = 0.1f64;
        debug_assert_eq!(rug_fuzz_0.weeks(), Duration::DAY * 10.5);
        debug_assert_eq!(rug_fuzz_1.weeks(), Duration::HOUR * 16.8);
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_test_weeks_fractional = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_98 {
    use super::*;
    use crate::*;
    #[test]
    fn test_days_negative() {
        let _rug_st_tests_llm_16_98_rrrruuuugggg_test_days_negative = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!((- rug_fuzz_0).days(), Duration::new(- 1 * 86_400, 0));
        let _rug_ed_tests_llm_16_98_rrrruuuugggg_test_days_negative = 0;
    }
    #[test]
    fn test_days_zero() {
        let _rug_st_tests_llm_16_98_rrrruuuugggg_test_days_zero = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(rug_fuzz_0.days(), Duration::new(0, 0));
        let _rug_ed_tests_llm_16_98_rrrruuuugggg_test_days_zero = 0;
    }
    #[test]
    fn test_days_positive() {
        let _rug_st_tests_llm_16_98_rrrruuuugggg_test_days_positive = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(rug_fuzz_0.days(), Duration::new(1 * 86_400, 0));
        let _rug_ed_tests_llm_16_98_rrrruuuugggg_test_days_positive = 0;
    }
    #[test]
    fn test_days_max() {
        let _rug_st_tests_llm_16_98_rrrruuuugggg_test_days_max = 0;
        debug_assert_eq!(i64::MAX.days(), Duration::days(i64::MAX));
        let _rug_ed_tests_llm_16_98_rrrruuuugggg_test_days_max = 0;
    }
    #[test]
    fn test_days_min() {
        let _rug_st_tests_llm_16_98_rrrruuuugggg_test_days_min = 0;
        debug_assert_eq!(i64::MIN.days(), Duration::days(i64::MIN));
        let _rug_ed_tests_llm_16_98_rrrruuuugggg_test_days_min = 0;
    }
    #[test]
    fn test_days_bounds() {
        let _rug_st_tests_llm_16_98_rrrruuuugggg_test_days_bounds = 0;
        debug_assert_eq!(Duration::MIN, Duration::new(i64::MIN, - 999_999_999));
        debug_assert_eq!(Duration::MAX, Duration::new(i64::MAX, 999_999_999));
        let _rug_ed_tests_llm_16_98_rrrruuuugggg_test_days_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_99 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn hours_zero() {
        let _rug_st_tests_llm_16_99_rrrruuuugggg_hours_zero = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::new(0, 0));
        let _rug_ed_tests_llm_16_99_rrrruuuugggg_hours_zero = 0;
    }
    #[test]
    fn hours_positive() {
        let _rug_st_tests_llm_16_99_rrrruuuugggg_hours_positive = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 24;
        debug_assert_eq!(rug_fuzz_0.hours(), Duration::new(3600, 0));
        debug_assert_eq!(rug_fuzz_1.hours(), Duration::new(7200, 0));
        debug_assert_eq!(rug_fuzz_2.hours(), Duration::new(86400, 0));
        let _rug_ed_tests_llm_16_99_rrrruuuugggg_hours_positive = 0;
    }
    #[test]
    fn hours_negative() {
        let _rug_st_tests_llm_16_99_rrrruuuugggg_hours_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 24;
        debug_assert_eq!((- rug_fuzz_0).hours(), Duration::new(- 3600, 0));
        debug_assert_eq!((- rug_fuzz_1).hours(), Duration::new(- 7200, 0));
        debug_assert_eq!((- rug_fuzz_2).hours(), Duration::new(- 86400, 0));
        let _rug_ed_tests_llm_16_99_rrrruuuugggg_hours_negative = 0;
    }
    #[test]
    fn hours_edge_cases() {
        let _rug_st_tests_llm_16_99_rrrruuuugggg_hours_edge_cases = 0;
        debug_assert_eq!(i64::MAX.hours(), Duration::new(i64::MAX, 0));
        debug_assert_eq!(i64::MIN.hours(), Duration::new(i64::MIN, 0));
        let _rug_ed_tests_llm_16_99_rrrruuuugggg_hours_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_104 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_zero_seconds() {
        let _rug_st_tests_llm_16_104_rrrruuuugggg_test_zero_seconds = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(Duration::seconds(rug_fuzz_0), Duration::ZERO);
        let _rug_ed_tests_llm_16_104_rrrruuuugggg_test_zero_seconds = 0;
    }
    #[test]
    fn test_positive_seconds() {
        let _rug_st_tests_llm_16_104_rrrruuuugggg_test_positive_seconds = 0;
        let rug_fuzz_0 = 5;
        debug_assert_eq!(Duration::seconds(rug_fuzz_0), Duration::new(5, 0));
        let _rug_ed_tests_llm_16_104_rrrruuuugggg_test_positive_seconds = 0;
    }
    #[test]
    fn test_negative_seconds() {
        let _rug_st_tests_llm_16_104_rrrruuuugggg_test_negative_seconds = 0;
        let rug_fuzz_0 = 5;
        debug_assert_eq!(Duration::seconds(- rug_fuzz_0), Duration::new(- 5, 0));
        let _rug_ed_tests_llm_16_104_rrrruuuugggg_test_negative_seconds = 0;
    }
    #[test]
    fn test_large_positive_seconds() {
        let _rug_st_tests_llm_16_104_rrrruuuugggg_test_large_positive_seconds = 0;
        debug_assert_eq!(Duration::seconds(i64::MAX), Duration::new(i64::MAX, 0));
        let _rug_ed_tests_llm_16_104_rrrruuuugggg_test_large_positive_seconds = 0;
    }
    #[test]
    fn test_large_negative_seconds() {
        let _rug_st_tests_llm_16_104_rrrruuuugggg_test_large_negative_seconds = 0;
        debug_assert_eq!(Duration::seconds(i64::MIN), Duration::new(i64::MIN, 0));
        let _rug_ed_tests_llm_16_104_rrrruuuugggg_test_large_negative_seconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_101 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_nanoseconds() {
        let _rug_st_tests_rug_101_rrrruuuugggg_test_nanoseconds = 0;
        let rug_fuzz_0 = 1_000_000_000;
        let p0: i64 = rug_fuzz_0;
        let duration = <i64 as NumericalDuration>::nanoseconds(p0);
        debug_assert_eq!(duration, Duration::nanoseconds(1_000_000_000));
        let _rug_ed_tests_rug_101_rrrruuuugggg_test_nanoseconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_102 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_microseconds() {
        let _rug_st_tests_rug_102_rrrruuuugggg_test_microseconds = 0;
        let rug_fuzz_0 = 100_000;
        let mut p0: i64 = rug_fuzz_0;
        let duration = <i64 as NumericalDuration>::microseconds(p0);
        debug_assert_eq!(duration, Duration::microseconds(100_000));
        let _rug_ed_tests_rug_102_rrrruuuugggg_test_microseconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_103_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1000;
        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(
            < i64 as NumericalDuration > ::milliseconds(p0), Duration::milliseconds(1000)
        );
        let _rug_ed_tests_rug_103_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_104 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::Duration;
    #[test]
    fn test_minutes() {
        let _rug_st_tests_rug_104_rrrruuuugggg_test_minutes = 0;
        let rug_fuzz_0 = 5;
        let p0: i64 = rug_fuzz_0;
        debug_assert_eq!(
            < i64 as NumericalDuration > ::minutes(p0), Duration::minutes(5)
        );
        let _rug_ed_tests_rug_104_rrrruuuugggg_test_minutes = 0;
    }
}
#[cfg(test)]
mod tests_rug_105 {
    use crate::ext::NumericalDuration;
    use crate::Duration;
    #[test]
    fn test_weeks() {
        let _rug_st_tests_rug_105_rrrruuuugggg_test_weeks = 0;
        let rug_fuzz_0 = 2;
        let p0: i64 = rug_fuzz_0;
        let result: Duration = <i64 as NumericalDuration>::weeks(p0);
        debug_assert_eq!(result.whole_weeks(), p0);
        let _rug_ed_tests_rug_105_rrrruuuugggg_test_weeks = 0;
    }
}
#[cfg(test)]
mod tests_rug_106 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_106_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 5.0;
        let p0: f64 = rug_fuzz_0;
        <f64 as NumericalDuration>::minutes(p0);
        let _rug_ed_tests_rug_106_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_nanoseconds() {
        let _rug_st_tests_rug_107_rrrruuuugggg_test_std_nanoseconds = 0;
        let rug_fuzz_0 = 1_000_000_000;
        let p0: u64 = rug_fuzz_0;
        let duration = <u64 as NumericalStdDuration>::std_nanoseconds(p0);
        debug_assert_eq!(duration, StdDuration::from_nanos(p0));
        let _rug_ed_tests_rug_107_rrrruuuugggg_test_std_nanoseconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_108 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_microseconds() {
        let _rug_st_tests_rug_108_rrrruuuugggg_test_std_microseconds = 0;
        let rug_fuzz_0 = 1_000_000;
        let p0: u64 = rug_fuzz_0;
        let duration = <u64 as NumericalStdDuration>::std_microseconds(p0);
        debug_assert_eq!(duration, StdDuration::from_secs(1));
        let _rug_ed_tests_rug_108_rrrruuuugggg_test_std_microseconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_109 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_milliseconds() {
        let _rug_st_tests_rug_109_rrrruuuugggg_test_std_milliseconds = 0;
        let rug_fuzz_0 = 1000;
        let p0: u64 = rug_fuzz_0;
        let result = <u64 as NumericalStdDuration>::std_milliseconds(p0);
        debug_assert_eq!(result, StdDuration::from_millis(1000));
        let _rug_ed_tests_rug_109_rrrruuuugggg_test_std_milliseconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_110 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_seconds() {
        let _rug_st_tests_rug_110_rrrruuuugggg_test_std_seconds = 0;
        let rug_fuzz_0 = 1234;
        let p0: u64 = rug_fuzz_0;
        debug_assert_eq!(
            < u64 as NumericalStdDuration > ::std_seconds(p0),
            StdDuration::from_secs(1234)
        );
        let _rug_ed_tests_rug_110_rrrruuuugggg_test_std_seconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_minutes() {
        let _rug_st_tests_rug_111_rrrruuuugggg_test_std_minutes = 0;
        let rug_fuzz_0 = 60;
        let p0: u64 = rug_fuzz_0;
        debug_assert_eq!(
            < u64 as NumericalStdDuration > ::std_minutes(p0), StdDuration::from_secs(60
            * 60)
        );
        let _rug_ed_tests_rug_111_rrrruuuugggg_test_std_minutes = 0;
    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_hours() {
        let _rug_st_tests_rug_112_rrrruuuugggg_test_std_hours = 0;
        let rug_fuzz_0 = 5;
        let p0: u64 = rug_fuzz_0;
        let result = <u64 as NumericalStdDuration>::std_hours(p0);
        debug_assert_eq!(result, StdDuration::from_secs(5 * 60 * 60));
        let _rug_ed_tests_rug_112_rrrruuuugggg_test_std_hours = 0;
    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_days() {
        let _rug_st_tests_rug_113_rrrruuuugggg_test_std_days = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 86_400;
        let p0: u64 = rug_fuzz_0;
        let expected = StdDuration::from_secs(rug_fuzz_1 * rug_fuzz_2);
        let result = <u64 as NumericalStdDuration>::std_days(p0);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_rug_113_rrrruuuugggg_test_std_days = 0;
    }
}
#[cfg(test)]
mod tests_rug_114 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_weeks() {
        let _rug_st_tests_rug_114_rrrruuuugggg_test_std_weeks = 0;
        let rug_fuzz_0 = 3;
        let p0: u64 = rug_fuzz_0;
        let result = p0.std_weeks();
        debug_assert_eq!(result, StdDuration::from_secs(3 * 604800));
        let _rug_ed_tests_rug_114_rrrruuuugggg_test_std_weeks = 0;
    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_115_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1_000_000.0;
        let p0: f64 = rug_fuzz_0;
        let duration = <f64 as NumericalStdDuration>::std_nanoseconds(p0);
        debug_assert_eq!(duration, StdDuration::from_nanos(1_000_000));
        let _rug_ed_tests_rug_115_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_microseconds() {
        let _rug_st_tests_rug_116_rrrruuuugggg_test_std_microseconds = 0;
        let rug_fuzz_0 = 2500.0;
        let p0: f64 = rug_fuzz_0;
        let result = p0.std_microseconds();
        debug_assert_eq!(result, StdDuration::from_micros(2500));
        let _rug_ed_tests_rug_116_rrrruuuugggg_test_std_microseconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_117 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_117_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1000.0;
        let mut p0: f64 = rug_fuzz_0;
        let duration: StdDuration = <f64 as NumericalStdDuration>::std_milliseconds(p0);
        debug_assert_eq!(duration, StdDuration::from_millis(1000));
        let _rug_ed_tests_rug_117_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_118 {
    use super::*;
    use std::time::Duration as StdDuration;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_std_seconds() {
        let _rug_st_tests_rug_118_rrrruuuugggg_test_std_seconds = 0;
        let rug_fuzz_0 = 3.5;
        let mut p0: f64 = rug_fuzz_0;
        let result = <f64 as NumericalStdDuration>::std_seconds(p0);
        debug_assert_eq!(result, StdDuration::from_secs_f64(3.5));
        let _rug_ed_tests_rug_118_rrrruuuugggg_test_std_seconds = 0;
    }
}
#[cfg(test)]
mod tests_rug_119 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_minutes() {
        let _rug_st_tests_rug_119_rrrruuuugggg_test_std_minutes = 0;
        let rug_fuzz_0 = 60.0;
        let p0: f64 = rug_fuzz_0;
        let duration: StdDuration = p0.std_minutes();
        debug_assert_eq!(duration, StdDuration::from_secs(60 * 60));
        let _rug_ed_tests_rug_119_rrrruuuugggg_test_std_minutes = 0;
    }
}
#[cfg(test)]
mod tests_rug_120 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use core::time::Duration as StdDuration;
    #[test]
    fn test_std_hours() {
        let _rug_st_tests_rug_120_rrrruuuugggg_test_std_hours = 0;
        let rug_fuzz_0 = 2.0;
        let p0: f64 = rug_fuzz_0;
        let duration = <f64 as NumericalStdDuration>::std_hours(p0);
        debug_assert_eq!(duration, StdDuration::from_secs(2 * 3600));
        let _rug_ed_tests_rug_120_rrrruuuugggg_test_std_hours = 0;
    }
}
#[cfg(test)]
mod tests_rug_121 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_days() {
        let _rug_st_tests_rug_121_rrrruuuugggg_test_std_days = 0;
        let rug_fuzz_0 = 1.5;
        let p0: f64 = rug_fuzz_0;
        let duration = <f64 as NumericalStdDuration>::std_days(p0);
        debug_assert_eq!(
            duration, StdDuration::from_secs(1 * 24 * 60 * 60 + 12 * 60 * 60)
        );
        let _rug_ed_tests_rug_121_rrrruuuugggg_test_std_days = 0;
    }
}
#[cfg(test)]
mod tests_rug_122 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_std_weeks() {
        let _rug_st_tests_rug_122_rrrruuuugggg_test_std_weeks = 0;
        let rug_fuzz_0 = 2.5;
        let rug_fuzz_1 = 2_5;
        let rug_fuzz_2 = 7;
        let rug_fuzz_3 = 24;
        let rug_fuzz_4 = 60;
        let rug_fuzz_5 = 60;
        let p0: f64 = rug_fuzz_0;
        let result = <f64 as NumericalStdDuration>::std_weeks(p0);
        let expected = StdDuration::from_secs(
            rug_fuzz_1 * rug_fuzz_2 * rug_fuzz_3 * rug_fuzz_4 * rug_fuzz_5,
        );
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_rug_122_rrrruuuugggg_test_std_weeks = 0;
    }
}
