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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5745() {
    rusty_monitor::set_test_id(5745);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_0: u8 = 32u8;
    let mut i32_0: i32 = -129i32;
    let mut i32_1: i32 = 33i32;
    let mut i64_0: i64 = 44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i64_1: i64 = -220i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_2: i32 = 70i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i32_3: i32 = 60i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i128_0: i128 = 56i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_4: i32 = 26i32;
    let mut i64_2: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i32_5: i32 = -39i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_7);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i32_6: i32 = -202i32;
    let mut i64_3: i64 = 72i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_6);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut u32_0: u32 = 86u32;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 67u8;
    let mut u8_3: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}
}