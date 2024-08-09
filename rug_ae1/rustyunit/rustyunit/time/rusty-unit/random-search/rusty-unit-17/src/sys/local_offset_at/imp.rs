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
fn rusty_test_4838() {
    rusty_monitor::set_test_id(4838);
    let mut u32_0: u32 = 18u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 9u16;
    let mut i32_0: i32 = 46i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = 40i8;
    let mut i8_1: i8 = 119i8;
    let mut i8_2: i8 = 14i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 124i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 33i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = 68i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = -25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_2: i32 = 23i32;
    let mut i64_1: i64 = 6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_0);
    let mut date_3_ref_0: &mut crate::date::Date = &mut date_3;
    let mut i8_6: i8 = 27i8;
    let mut i8_7: i8 = -22i8;
    let mut i8_8: i8 = 10i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_2: i64 = -40i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i128_0: i128 = 7i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 43u8;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 11u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = -67i8;
    let mut i8_10: i8 = -11i8;
    let mut i8_11: i8 = 84i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 113i8;
    let mut i8_13: i8 = 109i8;
    let mut i8_14: i8 = -59i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = -5i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -4i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i8_15: i8 = 46i8;
    let mut i8_16: i8 = 94i8;
    let mut i8_17: i8 = 42i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 12u32;
    let mut u8_6: u8 = 48u8;
    let mut u8_7: u8 = 58u8;
    let mut u8_8: u8 = 96u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_18: i8 = -76i8;
    let mut i8_19: i8 = 17i8;
    let mut i8_20: i8 = -20i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_5: i64 = 16i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_3: i32 = 74i32;
    let mut i64_6: i64 = 82i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_3);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i8_21: i8 = -70i8;
    let mut i8_22: i8 = 15i8;
    let mut i8_23: i8 = -68i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = -83i8;
    let mut i8_25: i8 = -127i8;
    let mut i8_26: i8 = -52i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i128_1: i128 = 28i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_7: i64 = -192i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut i8_27: i8 = -34i8;
    let mut i8_28: i8 = 118i8;
    let mut i8_29: i8 = -21i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_9);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_4: i32 = -124i32;
    let mut i64_8: i64 = 144i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new(i64_8, i32_4);
    let mut i64_9: i64 = 29i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut f32_0: f32 = 56.255059f32;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}
}