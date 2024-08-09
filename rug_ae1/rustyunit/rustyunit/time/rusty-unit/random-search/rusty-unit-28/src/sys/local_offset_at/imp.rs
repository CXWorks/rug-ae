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
fn rusty_test_4333() {
    rusty_monitor::set_test_id(4333);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_0: i64 = 59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 94u8;
    let mut u8_2: u8 = 78u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i8_0: i8 = -95i8;
    let mut i8_1: i8 = 35i8;
    let mut i8_2: i8 = 21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 19u32;
    let mut u8_3: u8 = 87u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 28u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 44i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = -60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_0: i32 = -157i32;
    let mut i64_3: i64 = -120i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_0);
    let mut i64_4: i64 = 7i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i128_0: i128 = 101i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_3: i8 = -2i8;
    let mut i8_4: i8 = 12i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_1: i128 = 30i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut u8_6: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_1);
    let mut u32_2: u32 = crate::time::Time::nanosecond(time_0);
    let mut month_0: month::Month = crate::month::Month::February;
    panic!("From RustyUnit with love");
}
}