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
fn rusty_test_4059() {
    rusty_monitor::set_test_id(4059);
    let mut i32_0: i32 = -52i32;
    let mut i32_1: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 1i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = 34i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = 64i8;
    let mut i8_1: i8 = 16i8;
    let mut i8_2: i8 = -35i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -23i8;
    let mut i8_4: i8 = -20i8;
    let mut i8_5: i8 = -15i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 52u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 45u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_4: i32 = -301i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_0};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut i32_5: i32 = -66i32;
    let mut i8_6: i8 = 17i8;
    let mut i8_7: i8 = -71i8;
    let mut i8_8: i8 = 66i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 25u32;
    let mut u8_3: u8 = 93u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 55u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 129i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_5);
    let mut i32_6: i32 = -78i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_1);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_4, utcoffset_3);
    let mut i32_7: i32 = 63i32;
    let mut i64_2: i64 = -3i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_7);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_2);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut f64_0: f64 = -4.222348f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_7, duration_3);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut u16_0: u16 = 7u16;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_6, time_2);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_5, date_5);
    let mut i128_0: i128 = -9i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_9: i8 = -125i8;
    let mut i8_10: i8 = 17i8;
    let mut i8_11: i8 = 33i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_9, utcoffset_4);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_10);
    let mut i64_3: i64 = -19i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut i64_4: i64 = -23i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = 3i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_12: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_11, duration_9);
    let mut date_7: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_12);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_add(date_7, duration_6);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_3);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_7, duration_4);
    let mut i64_6: i64 = 19i64;
    let mut i64_7: i64 = 23i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut i64_8: i64 = -61i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut offsetdatetime_13: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_14: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_13, duration_11);
    let mut date_9: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_14);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_9);
    let mut primitivedatetime_10: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_9, duration_10);
    let mut primitivedatetime_10_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_10;
    let mut i64_9: i64 = 28i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut i32_8: i32 = 59i32;
    let mut date_10: crate::date::Date = crate::date::Date {value: i32_8};
    let mut date_11: crate::date::Date = crate::date::Date::saturating_add(date_10, duration_12);
    let mut i64_10: i64 = -334i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut offsetdatetime_15: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_16: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_15, duration_13);
    let mut date_12: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_16);
    let mut i128_1: i128 = 58i128;
    let mut i64_11: i64 = 33i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_15: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_17, i32_3);
    let mut offsetdatetime_17: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_18: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_15);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_17);
    let mut duration_20: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i8_12: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_5);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_8);
    panic!("From RustyUnit with love");
}
}