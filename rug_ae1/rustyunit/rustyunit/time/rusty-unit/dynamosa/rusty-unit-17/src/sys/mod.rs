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
fn rusty_test_7699() {
    rusty_monitor::set_test_id(7699);
    let mut u32_0: u32 = 46u32;
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 112i64;
    let mut i32_1: i32 = 110i32;
    let mut i64_1: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 67i8;
    let mut i8_2: i8 = 94i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i32_2: i32 = 215i32;
    let mut i64_2: i64 = -61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_2);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut i8_3: i8 = -51i8;
    let mut i8_4: i8 = 26i8;
    let mut i8_5: i8 = -56i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_3: i64 = 99i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_3: i32 = 189i32;
    let mut i64_4: i64 = -23i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut u32_1: u32 = 81u32;
    let mut u8_0: u8 = 19u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_5: i64 = 31i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i8_6: i8 = -124i8;
    let mut i8_7: i8 = 68i8;
    let mut i8_8: i8 = 27i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_0: i128 = 30i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_9: i8 = 100i8;
    let mut i8_10: i8 = -90i8;
    let mut i8_11: i8 = 101i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 79i8;
    let mut i8_13: i8 = -22i8;
    let mut i8_14: i8 = -40i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut i8_15: i8 = 4i8;
    let mut i8_16: i8 = 63i8;
    let mut i8_17: i8 = 32i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut f64_0: f64 = -81.360271f64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_6: i64 = 111i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut f32_0: f32 = 17.739734f32;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_7: i64 = -325i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i8_18: i8 = 87i8;
    let mut i8_19: i8 = 101i8;
    let mut i8_20: i8 = -51i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 47u32;
    let mut u8_3: u8 = 43u8;
    let mut u8_4: u8 = 26u8;
    let mut u8_5: u8 = 78u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_8: i64 = 163i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_19);
    let mut u16_1: u16 = 50u16;
    let mut u8_6: u8 = 22u8;
    let mut u8_7: u8 = 39u8;
    let mut u8_8: u8 = 12u8;
    let mut i32_4: i32 = 79i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_18);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_2};
    let mut i64_9: i64 = 92i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_3: u32 = 95u32;
    let mut u8_9: u8 = 61u8;
    let mut u8_10: u8 = 67u8;
    let mut u8_11: u8 = 26u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_10, u8_6, u32_3);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut u8_12: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_6);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_2, duration_17);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_21, duration_11);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_11, u8_8, u8_9, u32_0);
    panic!("From RustyUnit with love");
}
}