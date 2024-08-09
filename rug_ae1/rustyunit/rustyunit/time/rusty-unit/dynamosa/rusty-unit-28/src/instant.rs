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
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::ops::Sub;
	use std::borrow::Borrow;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_902() {
    rusty_monitor::set_test_id(902);
    let mut u32_0: u32 = 35u32;
    let mut i32_0: i32 = -50i32;
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 107i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = 158i64;
    let mut i64_3: i64 = 79i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i128_0: i128 = -69i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut u16_0: u16 = 91u16;
    let mut i32_1: i32 = 104i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut i64_4: i64 = -34i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i64_5: i64 = -148i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut u32_1: u32 = 5u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 34u8;
    let mut u8_2: u8 = 41u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1749() {
    rusty_monitor::set_test_id(1749);
    let mut f32_0: f32 = 74.586736f32;
    let mut i64_0: i64 = -55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = 117.390492f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_1: i64 = 82i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_2: i64 = 22i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_392() {
    rusty_monitor::set_test_id(392);
    let mut f32_0: f32 = -17.195964f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut f32_1: f32 = 74.586736f32;
    let mut i64_0: i64 = -55i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = 117.390492f64;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_2);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i64_1: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 36u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_2: i64 = 22i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_2: f32 = crate::duration::Duration::as_seconds_f32(duration_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut instant_6: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7622() {
    rusty_monitor::set_test_id(7622);
    let mut i32_0: i32 = 14i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = -80i32;
    let mut i64_1: i64 = -62i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut i64_2: i64 = 92i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = 4i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i64_4: i64 = -1i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_7);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_6);
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 77u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_415() {
    rusty_monitor::set_test_id(415);
    let mut f32_0: f32 = 74.586736f32;
    let mut i64_0: i64 = -55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = 117.390492f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_1: i64 = 82i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 36u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_2: i64 = 22i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 19u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -72i32;
    let mut i64_0: i64 = -44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = -84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i32_1: i32 = 91i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_2: i64 = 19i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u32_1: u32 = 70u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 44u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -79i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_3);
    let mut i8_0: i8 = -93i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = -48i8;
    let mut i32_3: i32 = 133i32;
    let mut i64_3: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_4);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::March;
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut month_1: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_3);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1375() {
    rusty_monitor::set_test_id(1375);
    let mut i32_0: i32 = -162i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = 43i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut f32_0: f32 = 74.586736f32;
    let mut i64_0: i64 = -55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = 117.390492f64;
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: std::time::Instant = crate::instant::Instant::into_inner(instant_7);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_10, instant_8);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_1: i64 = 82i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 36u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_2: i64 = 22i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_108() {
    rusty_monitor::set_test_id(108);
    let mut i64_0: i64 = -82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut f32_0: f32 = 74.586736f32;
    let mut f64_0: f64 = 117.390492f64;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_6, instant_3);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_1: i64 = 82i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 36u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_2: i64 = 22i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4834() {
    rusty_monitor::set_test_id(4834);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_1_ref_0: &std::time::Instant = &mut instant_1;
    let mut i8_0: i8 = -62i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 33i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 19u16;
    let mut i32_0: i32 = 118i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = -4i8;
    let mut i8_4: i8 = 13i8;
    let mut i8_5: i8 = 12i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_3: u8 = 18u8;
    let mut i32_1: i32 = -5i32;
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 75u32;
    let mut u8_4: u8 = 90u8;
    let mut u8_5: u8 = 21u8;
    let mut u8_6: u8 = 66u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i64_1: i64 = -55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut f64_0: f64 = 117.390492f64;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: std::time::Instant = crate::instant::Instant::into_inner(instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_6, instant_3);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = 82i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut u32_2: u32 = 12u32;
    let mut u8_7: u8 = 43u8;
    let mut u8_8: u8 = 75u8;
    let mut u8_9: u8 = 36u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_3: i64 = 22i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_9, u8_8, u8_7, u32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_4);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_4);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_3, weekday_0);
    let mut i32_2: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_1);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_4_ref_0, instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_98() {
    rusty_monitor::set_test_id(98);
    let mut i32_0: i32 = -27i32;
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 19u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -72i32;
    let mut i64_0: i64 = -44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = -84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i32_2: i32 = 91i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_2: i64 = 19i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u32_1: u32 = 70u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 44u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_3);
    let mut i8_0: i8 = -93i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = -48i8;
    let mut i32_3: i32 = 133i32;
    let mut i64_3: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_4);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut month_1: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_3);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut i8_0: i8 = -53i8;
    let mut i8_1: i8 = -47i8;
    let mut i8_2: i8 = -7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 8u16;
    let mut i32_0: i32 = 73i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_1: i32 = 199i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_2);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_1: i64 = -141i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_4);
    let mut instant_4: std::time::Instant = crate::instant::Instant::into_inner(instant_3);
    let mut instant_4_ref_0: &std::time::Instant = &mut instant_4;
    let mut i64_2: i64 = -163i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Add::add(instant_5, duration_5);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_6_ref_0, instant_4_ref_0);
    let mut instant_7: &std::time::Instant = std::borrow::Borrow::borrow(instant_1_ref_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1944() {
    rusty_monitor::set_test_id(1944);
    let mut f32_0: f32 = 74.586736f32;
    let mut i64_0: i64 = -55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_1: i64 = 82i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_2: i64 = 22i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1800() {
    rusty_monitor::set_test_id(1800);
    let mut i32_0: i32 = 79i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i128_0: i128 = -47i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = -33i8;
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = 35i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = -15i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_2);
    let mut i64_0: i64 = 81i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i64_1: i64 = -203i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_2: i32 = crate::date::Date::year(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3354() {
    rusty_monitor::set_test_id(3354);
    let mut i32_0: i32 = -19i32;
    let mut i32_1: i32 = 75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 19u16;
    let mut i32_2: i32 = 118i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -4i8;
    let mut i8_1: i8 = 13i8;
    let mut i8_2: i8 = 12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = -55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = 82i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut u8_3: u8 = crate::util::weeks_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5287() {
    rusty_monitor::set_test_id(5287);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut u16_0: u16 = 55u16;
    let mut i32_0: i32 = -4i32;
    let mut i8_0: i8 = 64i8;
    let mut i8_1: i8 = -86i8;
    let mut i8_2: i8 = 21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 51i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut i32_2: i32 = 14i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i32_3: i32 = -80i32;
    let mut i64_1: i64 = -62i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut i64_2: i64 = 92i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = 4i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i64_4: i64 = -1i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_7);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_6);
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 77u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_5: i64 = 52i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i8_3: i8 = -16i8;
    let mut i8_4: i8 = -102i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -84i8;
    let mut i8_7: i8 = 96i8;
    let mut i8_8: i8 = 13i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_6: i64 = 80i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut i8_9: i8 = 42i8;
    let mut i8_10: i8 = 41i8;
    let mut i8_11: i8 = -10i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 0i8;
    let mut i8_13: i8 = -54i8;
    let mut i8_14: i8 = 9i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_7: i64 = -24i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i64_8: i64 = 116i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut month_3: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}
}