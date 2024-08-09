//! Different variant error

use core::convert::TryFrom;
use core::fmt;

/// An error type indicating that a [`TryFrom`](core::convert::TryFrom) call failed because the
/// original value was of a different variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentVariant;

impl fmt::Display for DifferentVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was of a different variant than required")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DifferentVariant {}

impl From<DifferentVariant> for crate::Error {
    fn from(err: DifferentVariant) -> Self {
        Self::DifferentVariant(err)
    }
}

impl TryFrom<crate::Error> for DifferentVariant {
    type Error = Self;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::DifferentVariant(err) => Ok(err),
            _ => Err(Self),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8327() {
    rusty_monitor::set_test_id(8327);
    let mut i32_0: i32 = -64i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut i64_0: i64 = -149i64;
    let mut i8_0: i8 = -73i8;
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -43i32;
    let mut i64_1: i64 = 15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i32_2: i32 = 14i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut i64_2: i64 = 27i64;
    let mut i32_3: i32 = -113i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_3);
    let mut i64_3: i64 = 76i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_4: i32 = -70i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i64_4: i64 = -19i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i32_5: i32 = -138i32;
    let mut i64_5: i64 = 4i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut i8_1: i8 = -39i8;
    let mut i8_2: i8 = -30i8;
    let mut i64_6: i64 = -98i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i32_6: i32 = -162i32;
    let mut i32_7: i32 = -31i32;
    let mut i64_7: i64 = 19i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_7);
    let mut i8_3: i8 = 109i8;
    let mut i8_4: i8 = 26i8;
    let mut i8_5: i8 = -5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_4);
    let mut i8_6: i8 = 35i8;
    let mut i8_7: i8 = -107i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_7, i8_1, i8_5);
    let mut i64_8: i64 = 45i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut i8_8: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_0, i8_6);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_2);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_11);
    let mut month_0: month::Month = crate::date::Date::month(date_5);
    let mut i64_9: i64 = -29i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_10: i64 = -38i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}
}