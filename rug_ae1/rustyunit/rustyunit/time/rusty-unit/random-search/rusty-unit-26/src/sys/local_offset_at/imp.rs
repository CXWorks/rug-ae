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
fn rusty_test_4278() {
    rusty_monitor::set_test_id(4278);
    let mut i8_0: i8 = 52i8;
    let mut i8_1: i8 = 36i8;
    let mut i8_2: i8 = -1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_0: i64 = -127i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i8_3: i8 = 66i8;
    let mut i8_4: i8 = -89i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -50i8;
    let mut i8_7: i8 = 108i8;
    let mut i8_8: i8 = -70i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = -12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 52i8;
    let mut i8_10: i8 = -8i8;
    let mut i8_11: i8 = 55i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 29i8;
    let mut i8_13: i8 = 12i8;
    let mut i8_14: i8 = 47i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 3i8;
    let mut i8_16: i8 = -114i8;
    let mut i8_17: i8 = -120i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_2: i64 = 128i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u32_1: u32 = 31u32;
    let mut u8_3: u8 = 79u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 35u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = 139i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = -65i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = -19i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut u16_0: u16 = 44u16;
    let mut i32_0: i32 = 118i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_18: i8 = 17i8;
    let mut i8_19: i8 = 12i8;
    let mut i8_20: i8 = 1i8;
    let mut i8_21: i8 = 42i8;
    let mut i8_22: i8 = -8i8;
    let mut i8_23: i8 = -15i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_1: i32 = -23i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u8_6: u8 = crate::date::Date::sunday_based_week(date_0);
    panic!("From RustyUnit with love");
}
}