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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8129() {
    rusty_monitor::set_test_id(8129);
    let mut u32_0: u32 = 87u32;
    let mut u8_0: u8 = 45u8;
    let mut u8_1: u8 = 66u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -27i8;
    let mut i8_1: i8 = 71i8;
    let mut i8_2: i8 = -11i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_0: i32 = 88i32;
    let mut i64_0: i64 = 19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i128_0: i128 = -95i128;
    let mut i64_1: i64 = -181i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_1);
    let mut i32_1: i32 = 68i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_3: i64 = 136i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_4: i64 = -32i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_2: i32 = 39i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_5: i64 = 51i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_6: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_7, utcoffset_3);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: std::time::Instant = crate::instant::Instant::into_inner(instant_9);
    let mut i64_7: i64 = -86i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i64_8: i64 = 193i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11_ref_0: &crate::instant::Instant = &mut instant_11;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_11_ref_0, instant_8_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_6);
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_0, i32_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2515() {
    rusty_monitor::set_test_id(2515);
    let mut i32_0: i32 = 68i32;
    let mut f32_0: f32 = -5.878817f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_0: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_4);
    let mut f32_1: f32 = 23.356745f32;
    let mut i8_0: i8 = -93i8;
    let mut i8_1: i8 = 75i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i128_0: i128 = -78i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_6);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_1: i64 = 12i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_7);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = -65i8;
    let mut i8_5: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_6: i8 = -110i8;
    let mut i8_7: i8 = -107i8;
    let mut i8_8: i8 = 72i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 38i8;
    let mut i8_10: i8 = 75i8;
    let mut i8_11: i8 = 55i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = 27i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = 95i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_9);
    let mut i8_12: i8 = -54i8;
    let mut i8_13: i8 = -46i8;
    let mut i8_14: i8 = -43i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 86u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut str_0: &str = "XGKtV09KiGcZe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 48u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_4: i64 = 28i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_12);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut u32_2: u32 = 15u32;
    let mut u8_6: u8 = 91u8;
    let mut u8_7: u8 = 81u8;
    let mut u8_8: u8 = 71u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_1, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2058() {
    rusty_monitor::set_test_id(2058);
    let mut i128_0: i128 = -95i128;
    let mut i64_0: i64 = -181i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i32_0: i32 = 68i32;
    let mut i64_1: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_2: i64 = 136i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_3: i64 = -32i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i32_1: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_4: i64 = 42i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_5: i64 = -28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i8_0: i8 = -21i8;
    let mut i8_1: i8 = 14i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_8);
    let mut i64_6: i64 = -86i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i64_7: i64 = 193i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10_ref_0: &crate::instant::Instant = &mut instant_10;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_10_ref_0, instant_7_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_5);
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5955() {
    rusty_monitor::set_test_id(5955);
    let mut i8_0: i8 = 36i8;
    let mut i8_1: i8 = -36i8;
    let mut i8_2: i8 = 82i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -76i8;
    let mut i8_4: i8 = -36i8;
    let mut i8_5: i8 = 45i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = -132i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut f32_0: f32 = -125.373180f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -94i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 81u8;
    let mut u8_1: u8 = 39u8;
    let mut u8_2: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut i64_1: i64 = 7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut f64_0: f64 = 103.091441f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_4);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_2_ref_0: &std::time::Instant = &mut instant_2;
    let mut f64_1: f64 = -21.519830f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_5);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut i64_2: i64 = -119i64;
    let mut i128_0: i128 = -95i128;
    let mut i64_3: i64 = -181i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_6);
    let mut i32_1: i32 = 68i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_1);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Add::add(instant_5, duration_7);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut i64_5: i64 = 136i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = std::ops::Add::add(instant_7, duration_8);
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut i64_6: i64 = -32i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_2: i32 = 39i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_9);
    let mut i8_6: i8 = -48i8;
    let mut i8_7: i8 = -75i8;
    let mut i8_8: i8 = 124i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_7: i64 = 51i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_10);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = std::ops::Sub::sub(instant_10, instant_9);
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_8: i64 = -28i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i8_9: i8 = -21i8;
    let mut i8_10: i8 = 14i8;
    let mut i8_11: i8 = 60i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_7, utcoffset_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut instant_13: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_13_ref_0: &crate::instant::Instant = &mut instant_13;
    let mut instant_14: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_15: std::time::Instant = crate::instant::Instant::into_inner(instant_14);
    let mut i64_9: i64 = -86i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_14_ref_0: &crate::duration::Duration = &mut duration_14;
    let mut instant_16: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_16_ref_0: &crate::instant::Instant = &mut instant_16;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_16_ref_0, instant_13_ref_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_11);
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_8_ref_0, instant_6_ref_0);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_4_ref_0, instant_2_ref_0);
    let mut option_1: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_3, duration_2);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_129() {
    rusty_monitor::set_test_id(129);
    let mut i128_0: i128 = -91i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut f64_0: f64 = -145.166867f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i32_0: i32 = -107i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 62u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut f32_0: f32 = 112.935957f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_0: i64 = -109i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_0: i8 = 17i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_1: i32 = 138i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_5);
    let mut u32_1: u32 = 9u32;
    let mut u8_3: u8 = 64u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 92u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_2);
    let mut f64_1: f64 = -38.371873f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Sub::sub(instant_5, duration_8);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut instant_7: crate::instant::Instant = std::clone::Clone::clone(instant_6_ref_0);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_3_ref_0, instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3838() {
    rusty_monitor::set_test_id(3838);
    let mut u16_0: u16 = 70u16;
    let mut i32_0: i32 = -154i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 32i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_1: i32 = -103i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut u32_0: u32 = 71u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 35u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = -177i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -164i32;
    let mut i64_1: i64 = 88i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i64_2: i64 = 4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6052() {
    rusty_monitor::set_test_id(6052);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 35i32;
    let mut i64_0: i64 = 120i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 151i32;
    let mut i64_1: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_2: i64 = -18i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 11i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = 3i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_8);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_5: i64 = -74i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_2: i32 = 175i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_9);
    let mut i64_6: i64 = -32i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_3: i32 = 39i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_10);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_7: i64 = 51i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_11);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_8: i64 = -28i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut instant_7: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut i64_9: i64 = -86i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_8_ref_0, instant_6_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_1);
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_5);
    let mut i64_10: i64 = crate::duration::Duration::whole_seconds(duration_4);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_3, duration_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1528() {
    rusty_monitor::set_test_id(1528);
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 35i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 151i32;
    let mut i64_2: i64 = 86i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut f64_1: f64 = -2.857879f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_3: i64 = -18i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 11i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = 3i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_10);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_6: i64 = -74i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i32_2: i32 = 175i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_11);
    let mut i64_7: i64 = -32i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i32_3: i32 = 39i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_12);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_8: i64 = 51i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_13);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_9: i64 = -28i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_7);
    let mut i64_10: i64 = -86i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_10);
    let mut duration_14_ref_0: &crate::duration::Duration = &mut duration_14;
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10_ref_0: &crate::instant::Instant = &mut instant_10;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_10_ref_0, instant_8_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_1);
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_3, duration_7);
    let mut i64_11: i64 = crate::duration::Duration::whole_seconds(duration_6);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_5, duration_2);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3541() {
    rusty_monitor::set_test_id(3541);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut i32_0: i32 = 5i32;
    let mut i64_0: i64 = 89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_1: i32 = -89i32;
    let mut u32_0: u32 = 28u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 38u8;
    let mut f32_0: f32 = 3.760251f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = 42i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -88i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_1: u32 = 73u32;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 20u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 35i32;
    let mut i64_3: i64 = 120i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut i32_3: i32 = 151i32;
    let mut i64_4: i64 = 86i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_5: i64 = -18i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i64_6: i64 = 11i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i64_7: i64 = 3i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_8: i64 = -74i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut i32_4: i32 = 175i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_15);
    let mut i64_9: i64 = -32i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut i32_5: i32 = 39i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_16);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_11_ref_0: &mut crate::duration::Duration = &mut duration_11;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_1, month_0);
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(instant_2_ref_0, instant_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6071() {
    rusty_monitor::set_test_id(6071);
    let mut i8_0: i8 = -94i8;
    let mut i8_1: i8 = -55i8;
    let mut i8_2: i8 = -6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 81u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 88u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 35i32;
    let mut i64_0: i64 = 120i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i32_2: i32 = 151i32;
    let mut i64_1: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_2: i64 = -18i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 11i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = 3i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_8);
    let mut i8_3: i8 = 81i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 17i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_5: i64 = -74i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_3: i32 = 175i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_9);
    let mut i64_6: i64 = -32i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_4: i32 = 39i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_10);
    let mut i8_6: i8 = -48i8;
    let mut i8_7: i8 = -75i8;
    let mut i8_8: i8 = 124i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_7: i64 = 51i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_11);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_8: i64 = -28i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i8_9: i8 = -21i8;
    let mut i8_10: i8 = 14i8;
    let mut i8_11: i8 = 60i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_4);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut instant_7: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut i64_9: i64 = -86i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_8_ref_0, instant_6_ref_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_5);
    let mut i64_10: i64 = crate::duration::Duration::whole_seconds(duration_4);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_3, duration_0);
    let mut tuple_2: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3197() {
    rusty_monitor::set_test_id(3197);
    let mut u32_0: u32 = 94u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 35u8;
    let mut u8_2: u8 = 82u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 27u16;
    let mut i32_0: i32 = 80i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i128_0: i128 = 252i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = 182i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i128_1: i128 = -95i128;
    let mut i64_1: i64 = -181i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut i32_1: i32 = 68i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Add::add(instant_4, duration_5);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut i64_3: i64 = 136i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = std::ops::Add::add(instant_6, duration_6);
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut i64_4: i64 = -32i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_2: i32 = 39i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_7);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 124i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_5: i64 = 51i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_8);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(instant_9, instant_8);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_6: i64 = -28i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = 14i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut instant_12: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12_ref_0: &crate::instant::Instant = &mut instant_12;
    let mut instant_13: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_14: std::time::Instant = crate::instant::Instant::into_inner(instant_13);
    let mut i64_7: i64 = -86i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i64_8: i64 = 193i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut instant_15: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_15_ref_0: &crate::instant::Instant = &mut instant_15;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_15_ref_0, instant_12_ref_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_9);
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_7_ref_0, instant_5_ref_0);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_1);
    let mut ordering_1: std::cmp::Ordering = std::cmp::Ord::cmp(instant_3_ref_0, instant_1_ref_0);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2487() {
    rusty_monitor::set_test_id(2487);
    let mut u16_0: u16 = 82u16;
    let mut i32_0: i32 = 89i32;
    let mut f32_0: f32 = -85.722118f32;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_0: i64 = 21i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i64_1: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 3u16;
    let mut i32_1: i32 = -57i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut u32_1: u32 = 29u32;
    let mut u8_3: u8 = 35u8;
    let mut u8_4: u8 = 64u8;
    let mut u8_5: u8 = 73u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_2: i32 = 3i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut i128_0: i128 = -36i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 64i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_3: i32 = 31i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_3, u16_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_1, duration_6);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_0);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = 61u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 86u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i8_0: i8 = -31i8;
    let mut i64_0: i64 = -61i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i16_0: i16 = -27i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -4i32;
    let mut i64_1: i64 = -14i64;
    let mut i64_2: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_1: i32 = 109i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u16_0: u16 = 39u16;
    let mut i32_2: i32 = 27i32;
    let mut month_0: month::Month = crate::month::Month::September;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_3: crate::date::Date = crate::primitive_date_time::PrimitiveDateTime::date(primitivedatetime_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3173() {
    rusty_monitor::set_test_id(3173);
    let mut i64_0: i64 = -57i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut i32_0: i32 = 0i32;
    let mut i64_1: i64 = -58i64;
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = -65i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -110i8;
    let mut i8_4: i8 = -107i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 38i8;
    let mut i8_7: i8 = 75i8;
    let mut i8_8: i8 = 55i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 27i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = 95i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut f32_0: f32 = -35.686560f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i8_9: i8 = -54i8;
    let mut i8_10: i8 = -46i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 86u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut str_0: &str = "XGKtV09KiGcZe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut instant_3: crate::instant::Instant = std::convert::From::from(instant_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2654() {
    rusty_monitor::set_test_id(2654);
    let mut i8_0: i8 = 91i8;
    let mut i8_1: i8 = -83i8;
    let mut i8_2: i8 = 34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 35i32;
    let mut i64_0: i64 = 120i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 151i32;
    let mut i64_1: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_2: i64 = -18i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 11i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = 3i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_8);
    let mut i8_3: i8 = 81i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 17i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_5: i64 = -74i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_2: i32 = 175i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_9);
    let mut i64_6: i64 = -32i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_3: i32 = 39i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_10);
    let mut i64_7: i64 = 51i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_11);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_8: i64 = -28i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut instant_7: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_8_ref_0, instant_6_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_1);
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_5);
    let mut i64_9: i64 = crate::duration::Duration::whole_seconds(duration_4);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_3, duration_0);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_92() {
    rusty_monitor::set_test_id(92);
    let mut i32_0: i32 = -107i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 62u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f32_0: f32 = 112.935957f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_0: i64 = -109i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_0: i8 = 17i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_1: i32 = 138i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut u32_1: u32 = 9u32;
    let mut u8_3: u8 = 64u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 92u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_2);
    let mut f64_0: f64 = -38.371873f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_5);
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut instant_3: crate::instant::Instant = std::clone::Clone::clone(instant_2_ref_0);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5797() {
    rusty_monitor::set_test_id(5797);
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_0: i32 = -89i32;
    let mut u32_1: u32 = 28u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 38u8;
    let mut f32_0: f32 = 3.760251f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = 42i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_1: i64 = -88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u32_2: u32 = 73u32;
    let mut u8_6: u8 = 0u8;
    let mut u8_7: u8 = 7u8;
    let mut u8_8: u8 = 20u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 35i32;
    let mut i64_2: i64 = 120i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut i32_2: i32 = 151i32;
    let mut i64_3: i64 = 86i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_4: i64 = -18i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = 11i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = 3i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_12);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_7: i64 = -74i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i32_3: i32 = 175i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_13);
    let mut i64_8: i64 = -32i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i32_4: i32 = 39i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_14);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_9: i64 = 51i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_15);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut bool_0: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut u8_9: u8 = crate::util::days_in_year_month(i32_0, month_0);
    let mut u8_10: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_357() {
    rusty_monitor::set_test_id(357);
    let mut i32_0: i32 = 4i32;
    let mut i64_0: i64 = 81i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_1: i64 = -32i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 124i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 51i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = -28i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = 14i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: std::time::Instant = crate::instant::Instant::into_inner(instant_7);
    let mut i64_4: i64 = -86i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i64_5: i64 = 193i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9_ref_0: &crate::instant::Instant = &mut instant_9;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_9_ref_0, instant_6_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut instant_10: &std::time::Instant = std::borrow::Borrow::borrow(instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_848() {
    rusty_monitor::set_test_id(848);
    let mut i64_0: i64 = -181i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i32_0: i32 = 68i32;
    let mut i64_1: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_2: i64 = 136i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_3: i64 = -32i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i32_1: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 124i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 51i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6516() {
    rusty_monitor::set_test_id(6516);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 32i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = -103i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut u32_0: u32 = 71u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 35u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -177i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_2: i32 = -164i32;
    let mut i64_1: i64 = 88i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i64_2: i64 = 4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut i64_3: i64 = -181i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut i32_3: i32 = 68i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Add::add(instant_5, duration_6);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5414() {
    rusty_monitor::set_test_id(5414);
    let mut i8_0: i8 = -19i8;
    let mut i8_1: i8 = 100i8;
    let mut i8_2: i8 = -9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut i32_0: i32 = -105i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut i128_0: i128 = -95i128;
    let mut i64_0: i64 = -181i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_1);
    let mut i32_1: i32 = 68i32;
    let mut i64_1: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Add::add(instant_4, duration_2);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -32i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_3: i64 = 51i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_4);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_8, instant_7);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_4: i64 = -28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_5, utcoffset_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10_ref_0: &crate::instant::Instant = &mut instant_10;
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12: std::time::Instant = crate::instant::Instant::into_inner(instant_11);
    let mut i64_5: i64 = -86i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i64_6: i64 = 193i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut instant_13: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_13_ref_0: &crate::instant::Instant = &mut instant_13;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_13_ref_0, instant_10_ref_0);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_5);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    let mut month_0: month::Month = crate::month::Month::December;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_486() {
    rusty_monitor::set_test_id(486);
    let mut i64_0: i64 = -64i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = -3i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 8i64;
    let mut i64_3: i64 = 21i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut i64_4: i64 = -36i64;
    let mut i32_0: i32 = 72i32;
    let mut i32_1: i32 = 20i32;
    let mut i64_5: i64 = -41i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_1);
    let mut i8_0: i8 = -53i8;
    let mut i8_1: i8 = 41i8;
    let mut i8_2: i8 = -102i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_6: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i64_7: i64 = 229i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 48u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8150() {
    rusty_monitor::set_test_id(8150);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_1);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i128_0: i128 = 8i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 83i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i32_1: i32 = -17i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut f32_0: f32 = 3.760251f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_3);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut i64_0: i64 = 42i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Add::add(instant_5, duration_5);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut i64_1: i64 = -88i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_6);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 20u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 35i32;
    let mut i64_2: i64 = 120i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut i32_3: i32 = 151i32;
    let mut i64_3: i64 = 86i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_4: i64 = -18i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2598() {
    rusty_monitor::set_test_id(2598);
    let mut i8_0: i8 = -77i8;
    let mut i8_1: i8 = -51i8;
    let mut i8_2: i8 = 99i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 15u16;
    let mut i32_0: i32 = -261i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i32_1: i32 = 94i32;
    let mut i32_2: i32 = 33i32;
    let mut i32_3: i32 = -164i32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_3);
    let mut i64_1: i64 = 4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i64_2: i64 = 95i64;
    let mut i128_0: i128 = -95i128;
    let mut i64_3: i64 = -181i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut i32_4: i32 = 68i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_4);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_5: i64 = 136i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_5);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_6: i64 = -32i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_5: i32 = 39i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_6);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_7: i64 = 51i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_7);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_8: i64 = -28i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_5, utcoffset_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: std::time::Instant = crate::instant::Instant::into_inner(instant_9);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_9: i64 = 193i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_11_ref_0: &crate::duration::Duration = &mut duration_11;
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11_ref_0: &crate::instant::Instant = &mut instant_11;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_11_ref_0, instant_6_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_8);
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_314() {
    rusty_monitor::set_test_id(314);
    let mut i64_0: i64 = 52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_2_ref_0: &std::time::Instant = &mut instant_2;
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i32_0: i32 = -70i32;
    let mut f32_0: f32 = -38.496567f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut i64_1: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u32_0: u32 = 79u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = -43i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut offsetdatetime_1_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i32_2: i32 = 75i32;
    let mut i128_0: i128 = 187i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_2: i64 = -189i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_3: i32 = 71i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_6);
    let mut u32_1: u32 = 35u32;
    let mut u8_3: u8 = 67u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 89u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_1: i128 = 43i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_4: i32 = -55i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i32_5: i32 = -38i32;
    let mut str_0: &str = "39JOt2B";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_5);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_2);
    let mut offsetdatetime_3_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(instant_3_ref_0, instant_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4210() {
    rusty_monitor::set_test_id(4210);
    let mut i64_0: i64 = -49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = -77i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_0: i32 = -111i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u32_0: u32 = 17u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 74u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 85i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 88u16;
    let mut i32_1: i32 = -23i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut i8_0: i8 = -19i8;
    let mut i8_1: i8 = 100i8;
    let mut i8_2: i8 = -9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 48u32;
    let mut u8_3: u8 = 89u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 56u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_2);
    let mut i32_2: i32 = -105i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i64_2: i64 = -181i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_7);
    let mut i32_3: i32 = 68i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Add::add(instant_5, duration_8);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut i64_4: i64 = 136i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = std::ops::Add::add(instant_7, duration_9);
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut i64_5: i64 = -32i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i32_4: i32 = 39i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_10);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_387() {
    rusty_monitor::set_test_id(387);
    let mut i8_0: i8 = -111i8;
    let mut i8_1: i8 = -96i8;
    let mut i8_2: i8 = 78i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -20i8;
    let mut i8_4: i8 = 22i8;
    let mut i8_5: i8 = 88i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 40i8;
    let mut i8_7: i8 = 48i8;
    let mut i8_8: i8 = 61i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 7u32;
    let mut u8_0: u8 = 47u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 64u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 4i32;
    let mut i64_0: i64 = -175i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u16_0: u16 = 40u16;
    let mut i32_1: i32 = -27i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = -32i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = 39i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut i8_9: i8 = -48i8;
    let mut i8_10: i8 = -75i8;
    let mut i8_11: i8 = 124i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = 51i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = -28i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_12: i8 = -21i8;
    let mut i8_13: i8 = 14i8;
    let mut i8_14: i8 = 60i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_5);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: std::time::Instant = crate::instant::Instant::into_inner(instant_6);
    let mut i64_4: i64 = -86i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i64_5: i64 = 193i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8_ref_0: &crate::instant::Instant = &mut instant_8;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_8_ref_0, instant_5_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_73() {
    rusty_monitor::set_test_id(73);
    let mut i64_0: i64 = -32i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 124i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 51i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_1, instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -28i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = 14i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut i64_3: i64 = -86i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i64_4: i64 = 193i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_7_ref_0, instant_4_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7410() {
    rusty_monitor::set_test_id(7410);
    let mut i32_0: i32 = 104i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = -66i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i64_0: i64 = 53i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_0);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = 57u32;
    let mut u8_0: u8 = 70u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 16u16;
    let mut i32_2: i32 = -98i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_1);
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_3: i32 = 35i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i32_4: i32 = 151i32;
    let mut i64_2: i64 = 86i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_3: i64 = -18i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 11i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = 3i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Sub::sub(instant_5, duration_10);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_6: i64 = -74i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i32_5: i32 = 175i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_11);
    let mut i64_7: i64 = -32i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i32_6: i32 = 39i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_12);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_8: i64 = 51i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = std::ops::Sub::sub(instant_8, instant_7);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut primitivedatetime_2_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut instant_11: crate::instant::Instant = std::ops::Add::add(instant_3, duration_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1801() {
    rusty_monitor::set_test_id(1801);
    let mut u32_0: u32 = 24u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -76i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 35i32;
    let mut i64_0: i64 = 120i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i32_2: i32 = 151i32;
    let mut i64_1: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_2: i64 = -18i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 11i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = 3i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_8);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_5: i64 = -74i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_3: i32 = 175i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_9);
    let mut i64_6: i64 = -32i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_4: i32 = 39i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6585() {
    rusty_monitor::set_test_id(6585);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 1u32;
    let mut u8_3: u8 = 33u8;
    let mut u8_4: u8 = 47u8;
    let mut u8_5: u8 = 79u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = -56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_1: i32 = -89i32;
    let mut u32_2: u32 = 28u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 50u8;
    let mut u8_8: u8 = 38u8;
    let mut f32_0: f32 = 3.760251f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = 42i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_1: i64 = -88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u32_3: u32 = 73u32;
    let mut u8_9: u8 = 0u8;
    let mut u8_10: u8 = 7u8;
    let mut u8_11: u8 = 20u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_2};
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 35i32;
    let mut i64_2: i64 = 120i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut i32_3: i32 = 151i32;
    let mut i64_3: i64 = 86i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_4: i64 = -18i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = 11i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = 3i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_12);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_7: i64 = -74i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i32_4: i32 = 175i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_13);
    let mut i64_8: i64 = -32i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i32_5: i32 = 39i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_4);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut bool_0: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_8, u8_7, u8_6, u32_2);
    let mut u8_12: u8 = crate::util::days_in_year_month(i32_1, month_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3246() {
    rusty_monitor::set_test_id(3246);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut f64_0: f64 = -107.063493f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = -65i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -110i8;
    let mut i8_4: i8 = -107i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 38i8;
    let mut i8_7: i8 = 75i8;
    let mut i8_8: i8 = 55i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_0: i64 = 27i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = 95i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut f32_0: f32 = -35.686560f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i8_9: i8 = -54i8;
    let mut i8_10: i8 = -46i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 86u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut str_0: &str = "XGKtV09KiGcZe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 48u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_6);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut u32_2: u32 = 15u32;
    let mut u8_6: u8 = 91u8;
    let mut u8_7: u8 = 81u8;
    let mut u8_8: u8 = 71u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u32_3: u32 = 83u32;
    let mut u8_9: u8 = 83u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 9u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i32_0: i32 = 28i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_2);
    let mut f32_1: f32 = 28.517247f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_1: i32 = -29i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_7);
    let mut i64_3: i64 = 39i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i8_12: i8 = -84i8;
    let mut i8_13: i8 = 41i8;
    let mut i8_14: i8 = -102i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_5, utcoffset_5);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8068() {
    rusty_monitor::set_test_id(8068);
    let mut i8_0: i8 = 8i8;
    let mut i8_1: i8 = -59i8;
    let mut i8_2: i8 = -44i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = -95i128;
    let mut i64_0: i64 = -181i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i32_0: i32 = 68i32;
    let mut i64_1: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_2: i64 = 136i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_3: i64 = -32i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i32_1: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = -75i8;
    let mut i8_5: i8 = 124i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = 51i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_5, instant_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_5: i64 = -28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i8_6: i8 = -21i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: std::time::Instant = crate::instant::Instant::into_inner(instant_9);
    let mut i64_6: i64 = -86i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i64_7: i64 = 193i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11_ref_0: &crate::instant::Instant = &mut instant_11;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_11_ref_0, instant_7_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_5);
    let mut bool_1: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut bool_2: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    panic!("From RustyUnit with love");
}
}