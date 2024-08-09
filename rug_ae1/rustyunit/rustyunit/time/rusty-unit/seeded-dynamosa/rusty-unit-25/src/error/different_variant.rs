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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6376() {
//    rusty_monitor::set_test_id(6376);
    let mut i32_0: i32 = 381i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 105i32;
    let mut i128_0: i128 = 1000000i128;
    let mut i64_0: i64 = 3600i64;
    let mut i128_1: i128 = 9223372036854775807i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_1: i64 = 1i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_2: i32 = 398i32;
    let mut i64_2: i64 = -24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = -4i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_3: i32 = 86399i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 53u8;
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 39i8;
    let mut i8_5: i8 = 3i8;
    let mut i64_3: i64 = 604800i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = -11i8;
    let mut i8_8: i8 = 60i8;
    let mut i64_4: i64 = 12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut u32_0: u32 = 85u32;
    let mut u8_2: u8 = 5u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_1, u8_0, u8_2, u32_0);
    let mut i128_2: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_5: i64 = -97i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_2);
    let mut i8_9: i8 = 6i8;
    let mut i8_10: i8 = 79i8;
    let mut i8_11: i8 = 6i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_4, i8_9, i8_6);
    let mut i8_12: i8 = 1i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_7, i8_10);
    let mut i8_13: i8 = 127i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_8, i8_11);
    let mut i64_6: i64 = 1000000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i8_14: i8 = -52i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_13, i8_14, i8_12);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_4);
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut date_2_ref_0: &crate::date::Date = &mut date_2;
//    panic!("From RustyUnit with love");
}
}