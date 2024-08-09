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
fn rusty_test_4634() {
    rusty_monitor::set_test_id(4634);
    let mut f32_0: f32 = -34.239473f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i8_0: i8 = -69i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = 82i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = -48i8;
    let mut i8_5: i8 = -85i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_1: f32 = -77.703545f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 50u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 28u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = 78i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_6: i8 = -85i8;
    let mut i8_7: i8 = 92i8;
    let mut i8_8: i8 = -17i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 24i8;
    let mut i8_10: i8 = 33i8;
    let mut i8_11: i8 = 35i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f32_2: f32 = 138.347827f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut u32_2: u32 = 24u32;
    let mut u8_6: u8 = 86u8;
    let mut u8_7: u8 = 48u8;
    let mut u8_8: u8 = 42u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -78i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i8_12: i8 = -1i8;
    let mut i8_13: i8 = 63i8;
    let mut i8_14: i8 = -58i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -6i8;
    let mut i8_16: i8 = 80i8;
    let mut i8_17: i8 = -94i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_1);
    panic!("From RustyUnit with love");
}
}