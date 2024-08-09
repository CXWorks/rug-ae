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
fn rusty_test_4764() {
    rusty_monitor::set_test_id(4764);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 70i8;
    let mut i8_1: i8 = -45i8;
    let mut i8_2: i8 = -51i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 75i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i8_3: i8 = 91i8;
    let mut i8_4: i8 = -14i8;
    let mut i8_5: i8 = 12i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_0: i128 = -93i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut f64_0: f64 = -6.648612f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_6: i8 = -17i8;
    let mut i8_7: i8 = 84i8;
    let mut i8_8: i8 = 46i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 3i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut u32_1: u32 = 53u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 28u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = 65i8;
    let mut i8_10: i8 = -24i8;
    let mut i8_11: i8 = -22i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -43i8;
    let mut i8_13: i8 = -65i8;
    let mut i8_14: i8 = 79i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = 53i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i64_4: i64 = 51i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut i64_5: i64 = -282i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::abs(duration_13);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut i64_6: i64 = 135i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i8_15: i8 = -118i8;
    let mut i8_16: i8 = -109i8;
    let mut i8_17: i8 = 7i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 30i8;
    let mut i8_19: i8 = -21i8;
    let mut i8_20: i8 = 21i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i32_0: i32 = 47i32;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_17: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_17, i32_0);
    let mut u32_2: u32 = 39u32;
    let mut u8_6: u8 = 27u8;
    let mut u8_7: u8 = 67u8;
    let mut u8_8: u8 = 23u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut f64_1: f64 = -23.313076f64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_7: i64 = 21i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut f64_2: f64 = -39.646666f64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    panic!("From RustyUnit with love");
}
}