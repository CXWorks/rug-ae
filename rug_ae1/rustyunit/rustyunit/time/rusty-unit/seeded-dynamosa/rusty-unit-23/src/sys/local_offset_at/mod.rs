//! A method to obtain the local offset from UTC.

#[cfg_attr(target_family = "windows", path = "windows.rs")]
#[cfg_attr(target_family = "unix", path = "unix.rs")]
mod imp;

use crate::{OffsetDateTime, UtcOffset};

/// Attempt to obtain the system's UTC offset. If the offset cannot be determined, `None` is
/// returned.
pub(crate) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    imp::local_offset_at(datetime)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1581() {
//    rusty_monitor::set_test_id(1581);
    let mut i128_0: i128 = 1000000i128;
    let mut f32_0: f32 = 65.169352f32;
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut i8_0: i8 = 24i8;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 0i64;
    let mut i32_0: i32 = 5119853i32;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 60i8;
    let mut i8_3: i8 = 82i8;
    let mut i8_4: i8 = -24i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_4, i8_1);
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_1: i32 = 392i32;
    let mut i64_3: i64 = -84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 6u8;
    let mut i64_4: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut f64_1: f64 = 64.323926f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f64_2: f64 = 4768169126130614272.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_5: i64 = 2440588i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i8_6: i8 = 60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_6, i8_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i8_7: i8 = 23i8;
    let mut i8_8: i8 = 0i8;
    let mut i64_6: i64 = 1000000000i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i32_2: i32 = 161i32;
    let mut i64_7: i64 = 60i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut i64_8: i64 = 12i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_7, i8_8);
    let mut i64_9: i64 = 59i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_10: i64 = 253402300799i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::minutes(i64_9);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds(i64_10);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_10);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 100u32;
    let mut u8_2: u8 = 1u8;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_1, u8_3, u8_2, u32_0);
    let mut u32_1: u32 = 49u32;
    let mut u8_5: u8 = 53u8;
    let mut i32_3: i32 = 82i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_0, u8_5, u8_4, u32_1);
//    panic!("From RustyUnit with love");
}
}