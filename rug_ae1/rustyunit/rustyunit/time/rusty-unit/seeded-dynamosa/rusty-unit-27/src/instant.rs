//! The [`Instant`] struct and its associated `impl`s.

use core::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use core::convert::{TryFrom, TryInto};
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
use std::borrow::Borrow;
use std::time::Instant as StdInstant;

use crate::Duration;

/// A measurement of a monotonically non-decreasing clock. Opaque and useful only with [`Duration`].
///
/// Instants are always guaranteed to be no less than any previously measured instant when created,
/// and are often useful for tasks such as measuring benchmarks or timing how long an operation
/// takes.
///
/// Note, however, that instants are not guaranteed to be **steady**. In other words, each tick of
/// the underlying clock may not be the same length (e.g. some seconds may be longer than others).
/// An instant may jump forwards or experience time dilation (slow down or speed up), but it will
/// never go backwards.
///
/// Instants are opaque types that can only be compared to one another. There is no method to get
/// "the number of seconds" from an instant. Instead, it only allows measuring the duration between
/// two instants (or comparing two instants).
///
/// This implementation allows for operations with signed [`Duration`]s, but is otherwise identical
/// to [`std::time::Instant`].
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(pub StdInstant);

impl Instant {
    // region: delegation
    /// Returns an `Instant` corresponding to "now".
    ///
    /// ```rust
    /// # use time::Instant;
    /// println!("{:?}", Instant::now());
    /// ```
    pub fn now() -> Self {
        Self(StdInstant::now())
    }

    /// Returns the amount of time elapsed since this instant was created. The duration will always
    /// be nonnegative if the instant is not synthetically created.
    ///
    /// ```rust
    /// # use time::{Instant, ext::{NumericalStdDuration, NumericalDuration}};
    /// # use std::thread;
    /// let instant = Instant::now();
    /// thread::sleep(1.std_milliseconds());
    /// assert!(instant.elapsed() >= 1.milliseconds());
    /// ```
    pub fn elapsed(self) -> Duration {
        Self::now() - self
    }
    // endregion delegation

    // region: checked arithmetic
    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, ext::NumericalDuration};
    /// let now = Instant::now();
    /// assert_eq!(now.checked_add(5.seconds()), Some(now + 5.seconds()));
    /// assert_eq!(now.checked_add((-5).seconds()), Some(now + (-5).seconds()));
    /// ```
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        if duration.is_zero() {
            Some(self)
        } else if duration.is_positive() {
            self.0.checked_add(duration.abs_std()).map(Self)
        } else {
            debug_assert!(duration.is_negative());
            self.0.checked_sub(duration.abs_std()).map(Self)
        }
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, ext::NumericalDuration};
    /// let now = Instant::now();
    /// assert_eq!(now.checked_sub(5.seconds()), Some(now - 5.seconds()));
    /// assert_eq!(now.checked_sub((-5).seconds()), Some(now - (-5).seconds()));
    /// ```
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        if duration.is_zero() {
            Some(self)
        } else if duration.is_positive() {
            self.0.checked_sub(duration.abs_std()).map(Self)
        } else {
            debug_assert!(duration.is_negative());
            self.0.checked_add(duration.abs_std()).map(Self)
        }
    }
    // endregion checked arithmetic

    /// Obtain the inner [`std::time::Instant`].
    ///
    /// ```rust
    /// # use time::Instant;
    /// let now = Instant::now();
    /// assert_eq!(now.into_inner(), now.0);
    /// ```
    pub const fn into_inner(self) -> StdInstant {
        self.0
    }
}

// region: trait impls
impl From<StdInstant> for Instant {
    fn from(instant: StdInstant) -> Self {
        Self(instant)
    }
}

impl From<Instant> for StdInstant {
    fn from(instant: Instant) -> Self {
        instant.0
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        match self.0.cmp(&other.0) {
            Ordering::Equal => Duration::ZERO,
            Ordering::Greater => (self.0 - other.0)
                .try_into()
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
            Ordering::Less => -Duration::try_from(other.0 - self.0)
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
        }
    }
}

impl Sub<StdInstant> for Instant {
    type Output = Duration;

    fn sub(self, other: StdInstant) -> Self::Output {
        self - Self(other)
    }
}

impl Sub<Instant> for StdInstant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Self::Output {
        Instant(self) - other
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            Self(self.0 + duration.abs_std())
        } else if duration.is_negative() {
            Self(self.0 - duration.abs_std())
        } else {
            self
        }
    }
}

impl Add<Duration> for StdInstant {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        (Instant(self) + duration).0
    }
}

impl Add<StdDuration> for Instant {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self(self.0 + duration)
    }
}

impl_add_assign!(Instant: Duration, StdDuration);
impl_add_assign!(StdInstant: Duration);

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            Self(self.0 - duration.abs_std())
        } else if duration.is_negative() {
            Self(self.0 + duration.abs_std())
        } else {
            self
        }
    }
}

impl Sub<Duration> for StdInstant {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        (Instant(self) - duration).0
    }
}

impl Sub<StdDuration> for Instant {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self(self.0 - duration)
    }
}

impl_sub_assign!(Instant: Duration, StdDuration);
impl_sub_assign!(StdInstant: Duration);

impl PartialEq<StdInstant> for Instant {
    fn eq(&self, rhs: &StdInstant) -> bool {
        self.0.eq(rhs)
    }
}

impl PartialEq<Instant> for StdInstant {
    fn eq(&self, rhs: &Instant) -> bool {
        self.eq(&rhs.0)
    }
}

impl PartialOrd<StdInstant> for Instant {
    fn partial_cmp(&self, rhs: &StdInstant) -> Option<Ordering> {
        self.0.partial_cmp(rhs)
    }
}

impl PartialOrd<Instant> for StdInstant {
    fn partial_cmp(&self, rhs: &Instant) -> Option<Ordering> {
        self.partial_cmp(&rhs.0)
    }
}

impl AsRef<StdInstant> for Instant {
    fn as_ref(&self) -> &StdInstant {
        &self.0
    }
}

impl Borrow<StdInstant> for Instant {
    fn borrow(&self) -> &StdInstant {
        &self.0
    }
}
// endregion trait impls

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::ops::Sub;
	use std::cmp::Eq;
	use std::convert::From;
	use std::borrow::Borrow;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5336() {
//    rusty_monitor::set_test_id(5336);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut i64_0: i64 = -39i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 308i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = -167i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut u16_0: u16 = 82u16;
    let mut i32_2: i32 = 76i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut f32_0: f32 = -109.060150f32;
    let mut i64_1: i64 = 86i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = 1000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_3: i32 = 122i32;
    let mut i64_3: i64 = 1i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 59u16;
    let mut i32_4: i32 = 189i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut u8_0: u8 = crate::date::Date::iso_week(date_4);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut instant_3_ref_0: &mut crate::instant::Instant = &mut instant_3;
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_4_ref_0, instant_0_ref_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_218() {
//    rusty_monitor::set_test_id(218);
    let mut i64_0: i64 = -7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Add::add(instant_4, duration_4);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = std::ops::Add::add(instant_6, duration_8);
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: crate::instant::Instant = std::ops::Add::add(instant_8, duration_9);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_10);
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_9);
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_7);
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_5);
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut duration_15: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_203() {
//    rusty_monitor::set_test_id(203);
    let mut i64_0: i64 = -162i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i32_0: i32 = 20i32;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Add::add(instant_4, duration_4);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut i32_1: i32 = 2i32;
    let mut i64_3: i64 = 2147483647i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut i64_4: i64 = 9223372036854775807i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = std::ops::Sub::sub(instant_6, duration_7);
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut instant_8: crate::instant::Instant = std::clone::Clone::clone(instant_7_ref_0);
    let mut instant_9: crate::instant::Instant = std::clone::Clone::clone(instant_5_ref_0);
    let mut instant_10: crate::instant::Instant = std::clone::Clone::clone(instant_3_ref_0);
    let mut instant_11: crate::instant::Instant = std::clone::Clone::clone(instant_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1925() {
//    rusty_monitor::set_test_id(1925);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_2);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = 400i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_0: u8 = 6u8;
    let mut i32_1: i32 = 325i32;
    let mut i64_1: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1000u32;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 87u8;
    let mut u8_3: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 18u8;
    let mut u8_6: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_0);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = 2440588i64;
    let mut i64_3: i64 = 24i64;
    let mut i64_4: i64 = 102i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i32_2: i32 = 161i32;
    let mut i64_5: i64 = -95i64;
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_3, duration_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut bool_1: bool = std::cmp::PartialEq::eq(instant_3_ref_0, instant_1_ref_0);
    let mut month_2: month::Month = crate::month::Month::September;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8206() {
//    rusty_monitor::set_test_id(8206);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 120i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i32_2: i32 = 105i32;
    let mut i64_1: i64 = 1i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_3_ref_0, instant_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_539() {
//    rusty_monitor::set_test_id(539);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_1_ref_0: &std::time::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut i32_0: i32 = 336i32;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Add::add(instant_3, duration_1);
    let mut instant_5: std::time::Instant = crate::instant::Instant::into_inner(instant_4);
    let mut instant_5_ref_0: &std::time::Instant = &mut instant_5;
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut i64_1: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = std::ops::Sub::sub(instant_7, duration_2);
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_8);
    let mut instant_9_ref_0: &std::time::Instant = &mut instant_9;
    let mut i32_1: i32 = 285i32;
    let mut i64_2: i64 = 223i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_1);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11: crate::instant::Instant = std::ops::Sub::sub(instant_10, duration_5);
    let mut instant_11_ref_0: &crate::instant::Instant = &mut instant_11;
    let mut bool_0: bool = std::cmp::PartialEq::eq(instant_11_ref_0, instant_9_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(instant_6_ref_0, instant_5_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(instant_2_ref_0, instant_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5828() {
//    rusty_monitor::set_test_id(5828);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 9i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 120i32;
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Add::add(instant_1, duration_1);
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut i32_2: i32 = 105i32;
    let mut i64_2: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_3);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_4_ref_0, instant_2_ref_0);
    let mut u32_0: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6317() {
//    rusty_monitor::set_test_id(6317);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 9i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 120i32;
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut u32_0: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_172() {
//    rusty_monitor::set_test_id(172);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = 104.833336f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = 99i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Add::add(instant_1, duration_2);
    let mut i32_1: i32 = 51i32;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut i64_4: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Add::add(instant_3, duration_7);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(instant_7, instant_5);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_10, duration_8);
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_4, duration_5);
    let mut option_2: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_2, duration_1);
    let mut option_3: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_530() {
//    rusty_monitor::set_test_id(530);
    let mut u8_0: u8 = 0u8;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_3);
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7154() {
//    rusty_monitor::set_test_id(7154);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 83u16;
    let mut i32_0: i32 = 38i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut u16_1: u16 = 0u16;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 5u8;
    let mut i32_1: i32 = 113i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i8_0: i8 = 86i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 36i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u32_0: u32 = 0u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 11u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i64_0: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 3600i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = 126i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = 4i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_3: i32 = 1721425i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_5, date_3);
    let mut i32_4: i32 = 1000000i32;
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut i64_2: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_4);
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i64_4: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = std::ops::Sub::sub(instant_7, duration_7);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(instant_8, instant_6);
    let mut month_1: month::Month = crate::month::Month::January;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_3, i32_4);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_6);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_2, u8_2, u8_1, u8_0, u16_1);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_69() {
//    rusty_monitor::set_test_id(69);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut i128_0: i128 = 1000i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_0);
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i8_3: i8 = 4i8;
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_4_ref_0, instant_3_ref_0);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_2_ref_0, instant_0_ref_0);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2524() {
//    rusty_monitor::set_test_id(2524);
    let mut i32_0: i32 = 43i32;
    let mut i32_1: i32 = 126i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_2: i32 = 1721425i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut i32_3: i32 = 1000000i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_1);
    let mut instant_3: std::time::Instant = crate::instant::Instant::into_inner(instant_2);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_4);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_3);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_0, i32_3);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_5, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_985() {
//    rusty_monitor::set_test_id(985);
    let mut i32_0: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = -58i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut i32_1: i32 = 331i32;
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = 2147483647i64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut instant_1: &std::time::Instant = std::borrow::Borrow::borrow(instant_0_ref_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_5, i32_1);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2569() {
//    rusty_monitor::set_test_id(2569);
    let mut i32_0: i32 = 126i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 1721425i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut i32_2: i32 = 1000000i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_1);
    let mut instant_3: std::time::Instant = crate::instant::Instant::into_inner(instant_2);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_4);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_3);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_0, i32_2);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_390() {
//    rusty_monitor::set_test_id(390);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 246i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_310() {
//    rusty_monitor::set_test_id(310);
    let mut i16_0: i16 = 32767i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_206() {
//    rusty_monitor::set_test_id(206);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7836() {
//    rusty_monitor::set_test_id(7836);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = 4i8;
    let mut i8_4: i8 = 9i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 120i32;
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i32_2: i32 = 105i32;
    let mut i64_2: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_3_ref_0, instant_1_ref_0);
    let mut u32_0: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4542() {
//    rusty_monitor::set_test_id(4542);
    let mut i64_0: i64 = -43i64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_1_ref_0: &std::time::Instant = &mut instant_1;
    let mut i32_0: i32 = 348i32;
    let mut i64_1: i64 = -39i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = 308i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = -167i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut u16_0: u16 = 82u16;
    let mut i32_3: i32 = 76i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut f32_0: f32 = -109.060150f32;
    let mut i64_2: i64 = 86i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = 1000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_4: i32 = 122i32;
    let mut i64_3: i64 = 1i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 59u16;
    let mut i32_5: i32 = 189i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut u8_0: u8 = crate::date::Date::iso_week(date_4);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_4);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_6_ref_0, instant_1_ref_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3554() {
//    rusty_monitor::set_test_id(3554);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 76i32;
    let mut i32_1: i32 = 274i32;
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = 88i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 9i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_3: i32 = 120i32;
    let mut i64_2: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = std::ops::Add::add(instant_7, duration_4);
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut i32_4: i32 = 105i32;
    let mut i64_3: i64 = 1i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_4);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = std::ops::Sub::sub(instant_9, duration_6);
    let mut instant_10_ref_0: &crate::instant::Instant = &mut instant_10;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_10_ref_0, instant_8_ref_0);
    let mut instant_11: crate::instant::Instant = std::convert::From::from(instant_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_0);
//    panic!("From RustyUnit with love");
}
}