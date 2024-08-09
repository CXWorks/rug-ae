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
fn rusty_test_6325() {
    rusty_monitor::set_test_id(6325);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 105i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 26i32;
    let mut i64_1: i64 = 139i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut i32_1: i32 = -216i32;
    let mut i64_2: i64 = -47i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i8_0: i8 = -33i8;
    let mut i8_1: i8 = -52i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 63i8;
    let mut i8_5: i8 = 107i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_0: f32 = -64.802849f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_2: i32 = 17i32;
    let mut i64_3: i64 = -44i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_2);
    let mut i32_3: i32 = -93i32;
    let mut i64_4: i64 = -30i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_6: i8 = 91i8;
    let mut i8_7: i8 = 108i8;
    let mut i8_8: i8 = 26i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 85u32;
    let mut u8_3: u8 = 68u8;
    let mut u8_4: u8 = 73u8;
    let mut u8_5: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = 25i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_5: i64 = 158i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut i8_9: i8 = -29i8;
    let mut i8_10: i8 = -56i8;
    let mut i8_11: i8 = 118i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 8i8;
    let mut i8_13: i8 = 45i8;
    let mut i8_14: i8 = 74i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_6: i64 = -204i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_13);
    let mut i64_7: i64 = 86i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    panic!("From RustyUnit with love");
}
}