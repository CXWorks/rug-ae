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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::convert::AsRef;
	use std::ops::Sub;
	use std::convert::From;
	use std::cmp::Eq;
	use std::borrow::Borrow;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2053() {
    rusty_monitor::set_test_id(2053);
    let mut i64_0: i64 = 171i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -63i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 93i32;
    let mut i64_1: i64 = 103i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_2);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_2_ref_0: &std::time::Instant = &mut instant_2;
    let mut i64_2: i64 = -3i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Add::add(instant_3, duration_4);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_4_ref_0, instant_2_ref_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1904() {
    rusty_monitor::set_test_id(1904);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i8_0: i8 = 27i8;
    let mut i8_1: i8 = -93i8;
    let mut i8_2: i8 = 64i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -188i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i128_0: i128 = -7i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = -151i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_1: i64 = 14i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_5);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut instant_6_ref_0: &std::time::Instant = &mut instant_6;
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_7_ref_0, instant_6_ref_0);
    let mut instant_8: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_4);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_2, duration_1);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_1);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4567() {
    rusty_monitor::set_test_id(4567);
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 84u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 8i8;
    let mut i8_1: i8 = 18i8;
    let mut i8_2: i8 = 8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = 77i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut instant_1: &std::time::Instant = std::convert::AsRef::as_ref(instant_0_ref_0);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1647() {
    rusty_monitor::set_test_id(1647);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_1);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i64_0: i64 = -225i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = std::ops::Sub::sub(instant_7, duration_1);
    let mut i64_1: i64 = -18i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = std::ops::Sub::sub(instant_9, duration_3);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_10, instant_8);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3518() {
    rusty_monitor::set_test_id(3518);
    let mut i32_0: i32 = -140i32;
    let mut i64_0: i64 = -199i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u8_0: u8 = 14u8;
    let mut i64_1: i64 = -19i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 125i8;
    let mut i8_1: i8 = -72i8;
    let mut i8_2: i8 = 35i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -40i64;
    let mut i64_3: i64 = 18i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_3);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::clone::Clone::clone(instant_1_ref_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_1);
    let mut i8_4: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3642() {
    rusty_monitor::set_test_id(3642);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_2);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 241i64;
    let mut i64_1: i64 = 25i64;
    let mut i64_2: i64 = -67i64;
    let mut str_0: &str = "FWZgWOM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut error_0_ref_0: &error::Error = &mut error_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4944() {
    rusty_monitor::set_test_id(4944);
    let mut i64_0: i64 = -21i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut f32_0: f32 = -87.123359f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_0);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = -92i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = -130i32;
    let mut i64_1: i64 = -14i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Add::add(instant_4, duration_4);
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut instant_7: crate::instant::Instant = std::convert::From::from(instant_6);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_0, duration_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2538() {
    rusty_monitor::set_test_id(2538);
    let mut i8_0: i8 = 37i8;
    let mut i8_1: i8 = 22i8;
    let mut i8_2: i8 = 123i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 4u16;
    let mut i32_0: i32 = 29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_1: i64 = 200i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i32_1: i32 = 36i32;
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut f32_0: f32 = -12.912676f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_4);
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut i8_3: i8 = 16i8;
    let mut i8_4: i8 = 20i8;
    let mut i8_5: i8 = -22i8;
    let mut i64_3: i64 = 73i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i32_2: i32 = -71i32;
    let mut i64_4: i64 = -5i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_6, duration_5);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut instant_7: crate::instant::Instant = std::convert::From::from(instant_6);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut bool_0: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1127() {
    rusty_monitor::set_test_id(1127);
    let mut i64_0: i64 = -101i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 27i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_3);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut i32_0: i32 = 125i32;
    let mut u8_0: u8 = 44u8;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i32_1: i32 = 79i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_1, month_1, u8_0);
    let mut date_0: crate::date::Date = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    let mut instant_3: crate::instant::Instant = std::convert::From::from(instant_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2917() {
    rusty_monitor::set_test_id(2917);
    let mut f32_0: f32 = -217.162421f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = -113i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 16i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(instant_4_ref_0, instant_3_ref_0);
    let mut bool_1: bool = crate::duration::Duration::is_positive(duration_1);
    let mut i32_0: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2088() {
    rusty_monitor::set_test_id(2088);
    let mut f32_0: f32 = -52.718857f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = -81i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_1);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut bool_0: bool = true;
    let mut i64_1: i64 = 26i64;
    let mut i64_2: i64 = -21i64;
    let mut i64_3: i64 = -26i64;
    let mut str_0: &str = "iKVjSjGZ1IEB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut month_0: month::Month = crate::month::Month::February;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_3_ref_0, instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3604() {
    rusty_monitor::set_test_id(3604);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = -96i8;
    let mut i8_2: i8 = 40i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -22i8;
    let mut i8_4: i8 = 30i8;
    let mut i8_5: i8 = -36i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 50u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = -130i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut f64_0: f64 = -116.801545f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3415() {
    rusty_monitor::set_test_id(3415);
    let mut i8_0: i8 = -127i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = -26i8;
    let mut i64_0: i64 = 9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 6u8;
    let mut i32_0: i32 = -56i32;
    let mut i128_0: i128 = -31i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = 138i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_3);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_1);
    let mut instant_2: crate::instant::Instant = std::option::Option::unwrap(option_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    let mut instant_2_ref_0: &mut crate::instant::Instant = &mut instant_2;
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2243() {
    rusty_monitor::set_test_id(2243);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i128_0: i128 = -3i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -2i32;
    let mut i64_0: i64 = 49i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_3);
    let mut i64_2: i64 = -70i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = crate::duration::Duration::whole_minutes(duration_4);
    let mut instant_2: crate::instant::Instant = std::ops::Add::add(instant_1, duration_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_753() {
    rusty_monitor::set_test_id(753);
    let mut f64_0: f64 = 43.361851f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i32_0: i32 = -185i32;
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut instant_5: &std::time::Instant = std::borrow::Borrow::borrow(instant_4_ref_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(instant_3_ref_0, instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1136() {
    rusty_monitor::set_test_id(1136);
    let mut u16_0: u16 = 87u16;
    let mut i32_0: i32 = 49i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut i32_1: i32 = -150i32;
    let mut i64_0: i64 = 91i64;
    let mut i64_1: i64 = -35i64;
    let mut i64_2: i64 = -139i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_1_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3207() {
    rusty_monitor::set_test_id(3207);
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_0: i32 = -88i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f64_0: f64 = -25.733745f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_2);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_2_ref_0: &std::time::Instant = &mut instant_2;
    let mut i64_1: i64 = 27i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_4);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(instant_4_ref_0, instant_2_ref_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_0);
    let mut bool_1: bool = crate::util::is_leap_year(i32_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_545() {
    rusty_monitor::set_test_id(545);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 57u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut f64_0: f64 = 107.843176f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut i64_0: i64 = 72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Add::add(instant_3, duration_3);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_664() {
    rusty_monitor::set_test_id(664);
    let mut i32_0: i32 = -7i32;
    let mut i64_0: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut i32_1: i32 = -46i32;
    let mut i64_1: i64 = 9i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Add::add(instant_1, duration_6);
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut i64_2: i64 = 50i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_7);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_4_ref_0, instant_2_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3428() {
    rusty_monitor::set_test_id(3428);
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 50u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 70u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u16_0: u16 = 64u16;
    let mut i32_0: i32 = 71i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = -15i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_3);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_2, duration_1);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_73() {
    rusty_monitor::set_test_id(73);
    let mut i8_0: i8 = 71i8;
    let mut i8_1: i8 = 78i8;
    let mut i8_2: i8 = 10i8;
    let mut i32_0: i32 = 14i32;
    let mut i64_0: i64 = -33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = -61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_3_ref_0, instant_1_ref_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2661() {
    rusty_monitor::set_test_id(2661);
    let mut i64_0: i64 = 75i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_0: u32 = 3u32;
    let mut u8_0: u8 = 45u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 55i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i128_0: i128 = 70i128;
    let mut i32_1: i32 = -133i32;
    let mut i64_1: i64 = 150i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_1_ref_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::nanosecond(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_212() {
    rusty_monitor::set_test_id(212);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_0: u8 = 40u8;
    let mut i32_0: i32 = -16i32;
    let mut i64_0: i64 = -62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_1: i64 = -12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -39i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_4);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut bool_0: bool = std::cmp::PartialEq::ne(instant_3_ref_0, instant_1_ref_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}
}