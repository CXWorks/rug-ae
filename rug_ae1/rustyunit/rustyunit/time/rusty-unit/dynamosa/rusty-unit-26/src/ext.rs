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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6796() {
    rusty_monitor::set_test_id(6796);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 54u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_1: u32 = 77u32;
    let mut u8_3: u8 = 13u8;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 28i8;
    let mut i8_1: i8 = -14i8;
    let mut i8_2: i8 = -13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -60i8;
    let mut i8_4: i8 = -10i8;
    let mut i8_5: i8 = -40i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -38i8;
    let mut i8_7: i8 = -3i8;
    let mut i8_8: i8 = -7i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -105i8;
    let mut i8_10: i8 = -15i8;
    let mut i8_11: i8 = 72i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_2: u32 = 23u32;
    let mut u8_6: u8 = 68u8;
    let mut u8_7: u8 = 42u8;
    let mut u8_8: u8 = 53u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_12: i8 = 63i8;
    let mut i8_13: i8 = -38i8;
    let mut i8_14: i8 = -89i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 65i8;
    let mut i8_16: i8 = -77i8;
    let mut i8_17: i8 = -70i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_18: i8 = 112i8;
    let mut i8_19: i8 = -1i8;
    let mut i8_20: i8 = 27i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = -17i8;
    let mut i8_22: i8 = 73i8;
    let mut i8_23: i8 = -116i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 120i8;
    let mut i8_25: i8 = 111i8;
    let mut i8_26: i8 = -96i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_3: u32 = 89u32;
    let mut u8_9: u8 = 43u8;
    let mut u8_10: u8 = 67u8;
    let mut u8_11: u8 = 31u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut u32_4: u32 = 31u32;
    let mut u8_12: u8 = 55u8;
    let mut u8_13: u8 = 72u8;
    let mut u8_14: u8 = 82u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_27: i8 = 105i8;
    let mut i8_28: i8 = 125i8;
    let mut i8_29: i8 = 112i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = 6i8;
    let mut i8_31: i8 = 90i8;
    let mut i8_32: i8 = 42i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 59i8;
    let mut i8_34: i8 = -84i8;
    let mut i8_35: i8 = 78i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut u32_5: u32 = 10u32;
    let mut u8_15: u8 = 22u8;
    let mut u8_16: u8 = 43u8;
    let mut u8_17: u8 = 38u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i8_36: i8 = -21i8;
    let mut i8_37: i8 = -11i8;
    let mut i8_38: i8 = 3i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i8_39: i8 = 0i8;
    let mut i8_40: i8 = -44i8;
    let mut i8_41: i8 = -13i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i32_0: i32 = 46i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    panic!("From RustyUnit with love");
}
}