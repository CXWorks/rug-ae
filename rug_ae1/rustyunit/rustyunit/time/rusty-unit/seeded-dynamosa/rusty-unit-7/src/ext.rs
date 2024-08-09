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
#[timeout(30000)]fn rusty_test_1674() {
//    rusty_monitor::set_test_id(1674);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = 77.205142f32;
    let mut i128_0: i128 = 1i128;
    let mut i64_0: i64 = 60i64;
    let mut f64_0: f64 = 0.000000f64;
    let mut i64_1: i64 = 1i64;
    let mut i32_0: i32 = 511i32;
    let mut i64_2: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_3: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_1: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 33i8;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_4: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_0);
    let mut i64_5: i64 = -87i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_2: i8 = 2i8;
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_0, i8_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_2);
    let mut i64_6: i64 = 19i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i64_7: i64 = 1i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut month_1: month::Month = crate::month::Month::October;
    let mut month_2: month::Month = crate::month::Month::previous(month_0);
    let mut i32_2: i32 = crate::date::Date::to_julian_day(date_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_246() {
//    rusty_monitor::set_test_id(246);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut i64_1: i64 = 0i64;
    let mut i64_2: i64 = 1000i64;
    let mut str_0: &str = "FX5LJwKa00";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 35i64;
    let mut i64_4: i64 = 604800i64;
    let mut i64_5: i64 = 2440588i64;
    let mut str_1: &str = "Monday";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 604800i64;
    let mut i64_7: i64 = 253402300799i64;
    let mut i64_8: i64 = 86400i64;
    let mut str_2: &str = "ConversionRange";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut bool_3: bool = true;
    let mut i64_9: i64 = 60i64;
    let mut i64_10: i64 = 1000000000i64;
    let mut i64_11: i64 = 1000i64;
    let mut str_3: &str = "Saturday";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut str_4: &str = crate::error::component_range::ComponentRange::name(componentrange_3);
    let mut str_5: &str = crate::error::component_range::ComponentRange::name(componentrange_2);
    let mut str_6: &str = crate::error::component_range::ComponentRange::name(componentrange_1);
    let mut str_7: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
//    panic!("From RustyUnit with love");
}
}