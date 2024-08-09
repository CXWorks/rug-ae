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
fn rusty_test_5988() {
    rusty_monitor::set_test_id(5988);
    let mut i64_0: i64 = 84i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 19u8;
    let mut u8_2: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 50i8;
    let mut i8_1: i8 = -58i8;
    let mut i8_2: i8 = 28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i32_0: i32 = 18i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 70u16;
    let mut i32_1: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i32_2: i32 = -94i32;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 13u8;
    let mut u8_5: u8 = 18u8;
    let mut u16_1: u16 = 1u16;
    let mut i32_3: i32 = 0i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut u32_1: u32 = 65u32;
    let mut u8_6: u8 = 39u8;
    let mut u8_7: u8 = 37u8;
    let mut u8_8: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i32_4: i32 = 72i32;
    let mut i32_5: i32 = 20i32;
    let mut i64_2: i64 = -41i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut i8_3: i8 = -53i8;
    let mut i8_4: i8 = 41i8;
    let mut i8_5: i8 = -102i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut i64_3: i64 = 12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = -27i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i64_5: i64 = 229i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i32_6: i32 = 0i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_6};
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut u32_2: u32 = 6u32;
    let mut u8_9: u8 = 85u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 48u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_2);
    let mut i32_7: i32 = 2i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut u8_12: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_4, i32_4);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_5, u8_4, u8_3);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut month_0: month::Month = crate::month::Month::June;
    panic!("From RustyUnit with love");
}
}