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
#[timeout(30000)]fn rusty_test_7313() {
//    rusty_monitor::set_test_id(7313);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 8u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 1000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i32_1: i32 = 212i32;
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 59u8;
    let mut i32_2: i32 = 376i32;
    let mut i64_1: i64 = 2i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 604800i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 10u32;
    let mut u8_6: u8 = 6u8;
    let mut u8_7: u8 = 2u8;
    let mut u8_8: u8 = 13u8;
    let mut i32_3: i32 = 6i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i32_4: i32 = -9i32;
    let mut i64_4: i64 = 2440588i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut u32_3: u32 = 10000u32;
    let mut u8_9: u8 = 24u8;
    let mut u8_10: u8 = 11u8;
    let mut u8_11: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_5: i64 = 604800i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut u16_0: u16 = 366u16;
    let mut i32_5: i32 = 54i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_1, i32_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_5, u8_4, u8_3, u32_1);
    let mut u8_12: u8 = crate::util::days_in_year_month(i32_1, month_1);
    let mut u8_13: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}
}