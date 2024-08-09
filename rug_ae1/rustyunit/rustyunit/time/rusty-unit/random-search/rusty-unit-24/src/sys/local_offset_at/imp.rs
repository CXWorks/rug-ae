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
fn rusty_test_4936() {
    rusty_monitor::set_test_id(4936);
    let mut u16_0: u16 = 32u16;
    let mut i32_0: i32 = -26i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 29i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u32_0: u32 = 62u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 120i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut f64_0: f64 = 22.790940f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i128_0: i128 = -93i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut i64_3: i64 = 143i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 42i8;
    let mut i8_2: i8 = 30i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -46i8;
    let mut i8_4: i8 = -116i8;
    let mut i8_5: i8 = 106i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut u8_3: u8 = crate::date::Date::monday_based_week(date_0);
    panic!("From RustyUnit with love");
}
}