//! Functions with a common interface that rely on system calls.

#![allow(unsafe_code)] // We're interfacing with system calls.

#[cfg(feature = "local-offset")]
mod local_offset_at;

#[cfg(feature = "local-offset")]
pub(crate) use local_offset_at::local_offset_at;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6828() {
//    rusty_monitor::set_test_id(6828);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i32_0: i32 = -56i32;
    let mut i32_1: i32 = 9999i32;
    let mut i64_0: i64 = 12i64;
    let mut i32_2: i32 = 252i32;
    let mut i64_1: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_2);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut f32_0: f32 = 32.238716f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 31u8;
    let mut i32_3: i32 = 139i32;
    let mut i64_4: i64 = 23i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut i32_4: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = 9999i32;
    let mut i64_5: i64 = 60i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut i64_6: i64 = 3600i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_7: i64 = 86400i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_6: i32 = 50i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_14, i32_6);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut i64_8: i64 = 9223372036854775807i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut i32_7: i32 = 1000000i32;
    let mut i64_9: i64 = 2147483647i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_18);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i32_8: i32 = 6i32;
    let mut i64_10: i64 = -294i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_8);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_19);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::abs(duration_20);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_8, duration_21);
    let mut i64_11: i64 = 604800i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i32_9: i32 = 71i32;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_11, i32_0);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut u16_0: u16 = 60u16;
    let mut i32_10: i32 = 88i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_9, u16_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_10);
    let mut u8_2: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut option_1: std::option::Option<crate::date::Date> = crate::date::Date::checked_add(date_2, duration_23);
    let mut option_2: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_1, duration_12);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_9);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_0, u8_1, u32_0);
//    panic!("From RustyUnit with love");
}
}