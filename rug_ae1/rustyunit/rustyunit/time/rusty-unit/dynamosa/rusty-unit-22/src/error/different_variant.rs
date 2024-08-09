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
fn rusty_test_7807() {
    rusty_monitor::set_test_id(7807);
    let mut i32_0: i32 = 9i32;
    let mut f64_0: f64 = -144.839131f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_0: i64 = -54i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = 46i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 84u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = 18i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut u16_0: u16 = 15u16;
    let mut i32_1: i32 = -252i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut i32_2: i32 = -157i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = -85i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i8_3: i8 = -53i8;
    let mut i8_4: i8 = -35i8;
    let mut i8_5: i8 = 65i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -64i8;
    let mut i8_7: i8 = 117i8;
    let mut i8_8: i8 = 11i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_0: i128 = -55i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_8);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_3: i32 = -44i32;
    let mut i64_3: i64 = -47i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut i64_4: i64 = 48i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut i64_5: i64 = 47i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i64_6: i64 = -97i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut i32_4: i32 = 63i32;
    let mut i64_7: i64 = -49i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_4);
    let mut i32_5: i32 = -101i32;
    let mut i64_8: i64 = -50i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_5);
    let mut i32_6: i32 = 47i32;
    let mut i64_9: i64 = 34i64;
    let mut i32_7: i32 = 137i32;
    let mut i64_10: i64 = 41i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::days(i64_10);
    let mut i8_9: i8 = -40i8;
    let mut i8_10: i8 = 1i8;
    let mut i8_11: i8 = -31i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_11: i64 = 47i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut u32_1: u32 = 61u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 39u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_8: i32 = 177i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut i64_12: i64 = -5i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::weeks(i64_12);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_16, i32_7);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_6);
    let mut i64_13: i64 = crate::duration::Duration::whole_hours(duration_15);
    let mut u8_6: u8 = crate::date::Date::iso_week(date_2);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}
}