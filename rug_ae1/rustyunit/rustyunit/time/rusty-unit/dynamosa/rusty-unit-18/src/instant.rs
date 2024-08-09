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
	use std::cmp::Eq;
	use std::borrow::Borrow;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4366() {
    rusty_monitor::set_test_id(4366);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i32_4: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2698() {
    rusty_monitor::set_test_id(2698);
    let mut i64_0: i64 = 76i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = -107i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_2);
    let mut i32_0: i32 = -91i32;
    let mut i64_2: i64 = -8i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_5);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_3: i64 = 113i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u16_0: u16 = 95u16;
    let mut i32_1: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_4: i64 = -53i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_5: i64 = 36i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i32_2: i32 = 108i32;
    let mut i64_6: i64 = -35i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut instant_5: crate::instant::Instant = std::ops::Add::add(instant_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4846() {
    rusty_monitor::set_test_id(4846);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -97i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i64_1: i64 = -90i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Sub::sub(instant_5, duration_2);
    let mut i32_2: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_3: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_3);
    let mut u16_1: u16 = 66u16;
    let mut i32_4: i32 = -131i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut i32_5: i32 = 37i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_8, instant_7);
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_5);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_6, duration_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1478() {
    rusty_monitor::set_test_id(1478);
    let mut i64_0: i64 = -107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 113i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u16_0: u16 = 95u16;
    let mut i32_0: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_2: i64 = -53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = 36i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_1: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2578() {
    rusty_monitor::set_test_id(2578);
    let mut i8_0: i8 = 13i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = -40i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut i32_4: i32 = 37i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2892() {
    rusty_monitor::set_test_id(2892);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = -107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = std::ops::Sub::sub(instant_3, duration_0);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut i32_0: i32 = -91i32;
    let mut i64_1: i64 = -8i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Sub::sub(instant_5, duration_3);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 113i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 95u16;
    let mut i32_1: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_3: i64 = -53i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = 36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_2: i32 = 108i32;
    let mut i64_5: i64 = -35i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_6_ref_0, instant_4_ref_0);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(instant_2, instant_1);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6378() {
    rusty_monitor::set_test_id(6378);
    let mut i64_0: i64 = -237i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut i8_0: i8 = -28i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = -13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = 167.642094f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_1: i64 = -45i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 27u16;
    let mut i32_0: i32 = 11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i64_2: i64 = 113i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut f32_1: f32 = -129.794601f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_3: i64 = 197i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_1: i32 = 19i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_7);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_6);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2694() {
    rusty_monitor::set_test_id(2694);
    let mut i32_0: i32 = -114i32;
    let mut i64_0: i64 = -107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i32_1: i32 = -91i32;
    let mut i64_1: i64 = -8i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_2: i64 = 113i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -53i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = 36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_2: i32 = 108i32;
    let mut i64_5: i64 = -35i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_3_ref_0, instant_1_ref_0);
    let mut u8_0: u8 = crate::util::weeks_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4212() {
    rusty_monitor::set_test_id(4212);
    let mut i128_0: i128 = 63i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = 33i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 129i32;
    let mut i64_1: i64 = 33i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Add::add(instant_2, duration_3);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i64_2: i64 = -107i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_4);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut i32_1: i32 = -91i32;
    let mut i64_3: i64 = -8i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = std::ops::Sub::sub(instant_6, duration_7);
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 113i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut u16_0: u16 = 95u16;
    let mut i32_2: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_5: i64 = -53i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = 36i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i32_3: i32 = 108i32;
    let mut i64_7: i64 = -35i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_3);
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_7_ref_0, instant_5_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(instant_3_ref_0, instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6076() {
    rusty_monitor::set_test_id(6076);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut u8_0: u8 = 18u8;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i32_0: i32 = -19i32;
    let mut i64_0: i64 = -90i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = std::ops::Sub::sub(instant_1, duration_0);
    let mut i32_1: i32 = 27i32;
    let mut i64_1: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_2: i64 = 41i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Sub::sub(instant_4, instant_3);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    let mut instant_7: crate::instant::Instant = std::clone::Clone::clone(instant_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3939() {
    rusty_monitor::set_test_id(3939);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i32_4: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut instant_6: &std::time::Instant = std::convert::AsRef::as_ref(instant_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7178() {
    rusty_monitor::set_test_id(7178);
    let mut i64_0: i64 = 39i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_0);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2_ref_0: &crate::instant::Instant = &mut instant_2;
    let mut u16_0: u16 = 20u16;
    let mut i32_0: i32 = -117i32;
    let mut i8_0: i8 = -6i8;
    let mut i8_1: i8 = 97i8;
    let mut i8_2: i8 = -93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 73i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 73u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 22u16;
    let mut i32_1: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i8_3: i8 = -123i8;
    let mut i8_4: i8 = 105i8;
    let mut i8_5: i8 = -27i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_2: i32 = 15i32;
    let mut i64_2: i64 = -99i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(instant_2_ref_0, instant_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8684() {
    rusty_monitor::set_test_id(8684);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 41i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut u8_0: u8 = 87u8;
    let mut i32_1: i32 = -156i32;
    let mut i32_2: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_3: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_4: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_4);
    let mut i32_5: i32 = 37i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_414() {
    rusty_monitor::set_test_id(414);
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 101i32;
    let mut i64_2: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut u16_0: u16 = 66u16;
    let mut i32_2: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i32_3: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_4, duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5658() {
    rusty_monitor::set_test_id(5658);
    let mut u32_0: u32 = 84u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -109i8;
    let mut i8_1: i8 = 71i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -6i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i8_3: i8 = 101i8;
    let mut i8_4: i8 = 11i8;
    let mut i8_5: i8 = 55i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = -35i32;
    let mut i64_1: i64 = 28i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut i8_6: i8 = 42i8;
    let mut i8_7: i8 = 101i8;
    let mut i8_8: i8 = 37i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 37u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 85u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = -87i8;
    let mut i8_10: i8 = -21i8;
    let mut i8_11: i8 = 92i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_1: i32 = -186i32;
    let mut i64_2: i64 = 4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_12: i8 = 110i8;
    let mut i8_13: i8 = 49i8;
    let mut i8_14: i8 = -10i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 0i8;
    let mut i8_16: i8 = -33i8;
    let mut i8_17: i8 = 88i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 42u32;
    let mut u8_6: u8 = 5u8;
    let mut u8_7: u8 = 83u8;
    let mut u8_8: u8 = 92u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i64_3: i64 = -19i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = 88i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_2: i32 = -7i32;
    let mut i64_5: i64 = 180i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_2);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_5, duration_8);
    let mut i64_6: i64 = 60i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut i64_7: i64 = 84i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_9);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = std::ops::Add::add(instant_5, duration_11);
    let mut instant_6_ref_0: &crate::instant::Instant = &mut instant_6;
    let mut i32_3: i32 = -75i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_8: i64 = -4i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut u32_3: u32 = 15u32;
    let mut u8_9: u8 = 8u8;
    let mut u8_10: u8 = 12u8;
    let mut u8_11: u8 = 64u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i128_0: i128 = -68i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_13);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_12);
    let mut u16_0: u16 = 85u16;
    let mut i32_4: i32 = 3i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut i32_5: i32 = 4i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut i8_18: i8 = -107i8;
    let mut i8_19: i8 = -34i8;
    let mut i8_20: i8 = -96i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i32_6: i32 = 159i32;
    let mut i64_9: i64 = 86i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_14, i32_6);
    let mut i32_7: i32 = 121i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_7};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_3, duration_15);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_4, utcoffset_7);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_9, primitivedatetime_2);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_10);
    let mut i32_8: i32 = -80i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_8};
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_16: crate::duration::Duration = std::ops::Sub::sub(instant_8, instant_7);
    let mut i64_10: i64 = 34i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::minutes(i64_10);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_6_ref_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6909() {
    rusty_monitor::set_test_id(6909);
    let mut f32_0: f32 = -146.025587f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_2);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut i32_4: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8128() {
    rusty_monitor::set_test_id(8128);
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 41i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut u8_0: u8 = 87u8;
    let mut i32_1: i32 = -156i32;
    let mut i32_2: i32 = 80i32;
    let mut i64_1: i64 = -43i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i64_2: i64 = -90i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_2);
    let mut i32_3: i32 = 27i32;
    let mut i64_3: i64 = 8i64;
    let mut i32_4: i32 = 101i32;
    let mut i64_4: i64 = 41i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_4);
    let mut u16_1: u16 = 66u16;
    let mut i32_5: i32 = -131i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut i32_6: i32 = 37i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_5);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_4, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7794() {
    rusty_monitor::set_test_id(7794);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -57i32;
    let mut f32_0: f32 = -45.622465f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i64_0: i64 = -107i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = std::ops::Sub::sub(instant_2, duration_2);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut i32_1: i32 = -91i32;
    let mut i64_1: i64 = -8i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_5);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 113i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 95u16;
    let mut i32_2: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_3: i64 = -53i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = 36i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_3: i32 = 108i32;
    let mut i64_5: i64 = -35i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_3);
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_5_ref_0, instant_3_ref_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(instant_1_ref_0);
    let mut bool_0: bool = crate::util::is_leap_year(i32_0);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_138() {
    rusty_monitor::set_test_id(138);
    let mut i32_0: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i8_0: i8 = -107i8;
    let mut i8_1: i8 = -34i8;
    let mut i8_2: i8 = -96i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 159i32;
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i32_2: i32 = 121i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 99u32;
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 29u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = -80i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_1, instant_0);
    let mut i64_1: i64 = 34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6340() {
    rusty_monitor::set_test_id(6340);
    let mut u8_0: u8 = 29u8;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = 161i64;
    let mut i8_0: i8 = 25i8;
    let mut i8_1: i8 = -12i8;
    let mut i8_2: i8 = 97i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_3: i8 = -41i8;
    let mut i8_4: i8 = -79i8;
    let mut i8_5: i8 = -4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -25i8;
    let mut i8_7: i8 = -2i8;
    let mut i8_8: i8 = -122i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut f64_0: f64 = -53.040939f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = -14i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = -119i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut i32_2: i32 = -99i32;
    let mut i32_3: i32 = 103i32;
    let mut i64_1: i64 = 86i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_3);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1689() {
    rusty_monitor::set_test_id(1689);
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i32_4: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    let mut i32_5: i32 = crate::duration::Duration::subsec_microseconds(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1585() {
    rusty_monitor::set_test_id(1585);
    let mut i64_0: i64 = -90i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_0);
    let mut i32_0: i32 = 27i32;
    let mut i64_1: i64 = 8i64;
    let mut i64_2: i64 = 41i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_1: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3633() {
    rusty_monitor::set_test_id(3633);
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut i32_3: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5274() {
    rusty_monitor::set_test_id(5274);
    let mut i32_0: i32 = -4i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = 84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Add::add(instant_0, duration_3);
    let mut instant_1_ref_0: &crate::instant::Instant = &mut instant_1;
    let mut i32_1: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_3: i64 = -4i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 64u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = -68i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5709() {
    rusty_monitor::set_test_id(5709);
    let mut i64_0: i64 = 82i64;
    let mut i32_0: i32 = 80i32;
    let mut i64_1: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_1: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_2: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut u16_0: u16 = 66u16;
    let mut i32_3: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i32_4: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4164() {
    rusty_monitor::set_test_id(4164);
    let mut i32_0: i32 = 70i32;
    let mut u16_0: u16 = 89u16;
    let mut u8_0: u8 = 45u8;
    let mut u8_1: u8 = 19u8;
    let mut u8_2: u8 = 17u8;
    let mut i32_1: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_0: i64 = -4i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 15u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 64u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i128_0: i128 = -68i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut u32_1: u32 = 37u32;
    let mut u8_6: u8 = 90u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 27u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut u16_1: u16 = 85u16;
    let mut i32_2: i32 = 3i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_3: i32 = 4i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut i8_0: i8 = -107i8;
    let mut i8_1: i8 = -34i8;
    let mut i8_2: i8 = -96i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_4: i32 = 121i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut i32_5: i32 = -80i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_5};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Sub::sub(instant_1, instant_0);
    let mut i64_2: i64 = 34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6441() {
    rusty_monitor::set_test_id(6441);
    let mut u32_0: u32 = 47u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 13u8;
    let mut i64_0: i64 = -120i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 115i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u8_3: u8 = 18u8;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i32_1: i32 = -19i32;
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = std::ops::Sub::sub(instant_0, duration_1);
    let mut i32_2: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_3: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_3);
    let mut u16_0: u16 = 66u16;
    let mut i32_4: i32 = -131i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_3);
    let mut i32_5: i32 = 37i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_1, month_0, u8_3);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6539() {
    rusty_monitor::set_test_id(6539);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(instant_3, instant_1);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(instant_9, instant_6);
    let mut i64_0: i64 = 15i64;
    let mut i32_0: i32 = 147i32;
    let mut i32_1: i32 = 157i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i8_0: i8 = 41i8;
    let mut i8_1: i8 = -15i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_2: i32 = 22i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i32_3: i32 = 47i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 95u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_10);
    let mut i32_4: i32 = 1i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_0);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2351() {
    rusty_monitor::set_test_id(2351);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = -107i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = std::ops::Sub::sub(instant_4, duration_1);
    let mut instant_5_ref_0: &crate::instant::Instant = &mut instant_5;
    let mut i32_0: i32 = -91i32;
    let mut i64_1: i64 = -8i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = std::ops::Sub::sub(instant_6, duration_4);
    let mut instant_7_ref_0: &crate::instant::Instant = &mut instant_7;
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 113i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 95u16;
    let mut i32_1: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_3: i64 = -53i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = 36i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_2: i32 = 108i32;
    let mut i64_5: i64 = -35i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(instant_7_ref_0, instant_5_ref_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_weeks(duration_0);
    let mut instant_8: &std::time::Instant = std::borrow::Borrow::borrow(instant_0_ref_0);
    panic!("From RustyUnit with love");
}
}