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
fn rusty_test_6183() {
    rusty_monitor::set_test_id(6183);
    let mut i32_0: i32 = 51i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = -46i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut u32_0: u32 = 70u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 51u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = -83i8;
    let mut i8_1: i8 = 65i8;
    let mut i8_2: i8 = 27i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_0: i64 = 6i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_2: i32 = -51i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i64_1: i64 = 28i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 78u32;
    let mut u8_3: u8 = 45u8;
    let mut u8_4: u8 = 47u8;
    let mut u8_5: u8 = 1u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = -63i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_4);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut i32_4: i32 = 13i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_5, date_5);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut i32_5: i32 = -31i32;
    let mut i64_2: i64 = -60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut i32_6: i32 = 139i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_2);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_7, time: time_4};
    let mut u16_0: u16 = 1u16;
    let mut i32_7: i32 = 0i32;
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut i64_3: i64 = -124i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u32_2: u32 = 25u32;
    let mut u8_6: u8 = 53u8;
    let mut u8_7: u8 = 86u8;
    let mut u8_8: u8 = 60u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_1: u16 = 55u16;
    let mut i32_8: i32 = -99i32;
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_1);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_9, time_5);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_5, duration_3);
    let mut i64_4: i64 = 120i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_3);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    panic!("From RustyUnit with love");
}
}