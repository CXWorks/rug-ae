//! Functions with a common interface that rely on system calls.

#![allow(unsafe_code)] // We're interfacing with system calls.

#[cfg(feature = "local-offset")]
mod local_offset_at;

#[cfg(feature = "local-offset")]
pub(crate) use local_offset_at::local_offset_at;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7915() {
    rusty_monitor::set_test_id(7915);
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 53u8;
    let mut u16_0: u16 = 52u16;
    let mut i32_0: i32 = 79i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = -85i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u8_2: u8 = 1u8;
    let mut i64_1: i64 = 29i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i8_0: i8 = -42i8;
    let mut i8_1: i8 = -4i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 6i32;
    let mut i64_2: i64 = -46i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_2: i32 = 52i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = -40i8;
    let mut i8_5: i8 = -46i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 124i8;
    let mut i8_7: i8 = -23i8;
    let mut i8_8: i8 = 62i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_1, u8_0, u8_2);
    panic!("From RustyUnit with love");
}
}