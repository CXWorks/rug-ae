//! A fallback for any OS not covered.

use crate::{OffsetDateTime, UtcOffset};

pub(super) fn local_offset_at(_datetime: OffsetDateTime) -> Option<UtcOffset> {
    None
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6173() {
    rusty_monitor::set_test_id(6173);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut u8_0: u8 = 32u8;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 17u32;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 58u8;
    let mut u8_3: u8 = 87u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut f64_0: f64 = -17.397737f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 49i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_1: i64 = -127i64;
    let mut u32_1: u32 = 38u32;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 14u8;
    let mut u8_6: u8 = 82u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = -55i8;
    let mut i8_2: i8 = -38i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -31i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_1: i32 = -22i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut i64_4: i64 = 14i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut i64_5: i64 = -58i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_2: i32 = -181i32;
    let mut i64_6: i64 = 229i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut i128_0: i128 = 143i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut f64_1: f64 = 192.755156f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut i64_7: i64 = 143i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i8_3: i8 = 127i8;
    let mut i8_4: i8 = 7i8;
    let mut i8_5: i8 = 70i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_6: i8 = -93i8;
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 7i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_8: i64 = 157i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut u32_2: u32 = 87u32;
    let mut u8_7: u8 = 41u8;
    let mut u8_8: u8 = 40u8;
    let mut u8_9: u8 = 29u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i64_9: i64 = 141i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i32_3: i32 = 92i32;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u16_0: u16 = 13u16;
    let mut i32_4: i32 = -212i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}
}