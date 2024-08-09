//! Extension traits.

use core::time::Duration as StdDuration;

use crate::Duration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}
    impl Sealed for i64 {}
    impl Sealed for u64 {}
    impl Sealed for f64 {}
}

// region: NumericalDuration
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
        Duration::nanoseconds((self * 1_000.) as _)
    }

    fn milliseconds(self) -> Duration {
        Duration::nanoseconds((self * 1_000_000.) as _)
    }

    fn seconds(self) -> Duration {
        Duration::nanoseconds((self * 1_000_000_000.) as _)
    }

    fn minutes(self) -> Duration {
        Duration::nanoseconds((self * 60_000_000_000.) as _)
    }

    fn hours(self) -> Duration {
        Duration::nanoseconds((self * 3_600_000_000_000.) as _)
    }

    fn days(self) -> Duration {
        Duration::nanoseconds((self * 86_400_000_000_000.) as _)
    }

    fn weeks(self) -> Duration {
        Duration::nanoseconds((self * 604_800_000_000_000.) as _)
    }
}
// endregion NumericalDuration

// region: NumericalStdDuration
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
        StdDuration::from_secs(self * 60)
    }

    fn std_hours(self) -> StdDuration {
        StdDuration::from_secs(self * 3_600)
    }

    fn std_days(self) -> StdDuration {
        StdDuration::from_secs(self * 86_400)
    }

    fn std_weeks(self) -> StdDuration {
        StdDuration::from_secs(self * 604_800)
    }
}

impl NumericalStdDuration for f64 {
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as _)
    }

    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000.) as _)
    }

    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000.) as _)
    }

    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000_000.) as _)
    }

    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 60_000_000_000.) as _)
    }

    fn std_hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 3_600_000_000_000.) as _)
    }

    fn std_days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 86_400_000_000_000.) as _)
    }

    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 604_800_000_000_000.) as _)
    }
}
// endregion NumericalStdDuration

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_360() {
//    rusty_monitor::set_test_id(360);
    let mut i64_0: i64 = 2440588i64;
    let mut i64_1: i64 = 1i64;
    let mut i64_2: i64 = 60i64;
    let mut i64_3: i64 = 86400i64;
    let mut i64_4: i64 = 12i64;
    let mut i64_5: i64 = -46i64;
    let mut i64_6: i64 = 3600i64;
    let mut i64_7: i64 = 2147483647i64;
    let mut i64_8: i64 = 12i64;
    let mut i64_9: i64 = 24i64;
    let mut i64_10: i64 = 3600i64;
    let mut i64_11: i64 = 3600i64;
    let mut i64_12: i64 = 104i64;
    let mut i64_13: i64 = 604800i64;
    let mut i64_14: i64 = 24i64;
    let mut i64_15: i64 = 1000i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_15);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_14);
    let mut result_2: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_13);
    let mut result_3: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_12);
    let mut result_4: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_11);
    let mut result_5: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_10);
    let mut result_6: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_9);
    let mut result_7: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_8);
    let mut result_8: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_7);
    let mut result_9: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_6);
    let mut result_10: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_5);
    let mut result_11: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_4);
    let mut result_12: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_3);
    let mut result_13: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut result_14: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_1);
    let mut result_15: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
//    panic!("From RustyUnit with love");
}
}