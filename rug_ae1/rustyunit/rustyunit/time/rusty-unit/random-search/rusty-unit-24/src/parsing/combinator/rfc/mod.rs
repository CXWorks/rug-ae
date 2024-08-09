//! Combinators for rules as defined in an RFC.
//!
//! These rules have been converted strictly following the ABNF syntax as specified in [RFC 2234].
//!
//! [RFC 2234]: https://datatracker.ietf.org/doc/html/rfc2234

pub(crate) mod rfc2234;
pub(crate) mod rfc2822;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4560() {
    rusty_monitor::set_test_id(4560);
    let mut i32_0: i32 = -159i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 66u8;
    let mut u8_2: u8 = 95u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 61u16;
    let mut i32_1: i32 = -90i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut i8_0: i8 = -73i8;
    let mut i8_1: i8 = -47i8;
    let mut i8_2: i8 = 12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 84u32;
    let mut u8_3: u8 = 65u8;
    let mut u8_4: u8 = 26u8;
    let mut u8_5: u8 = 95u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -87i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut i32_3: i32 = -13i32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_3);
    let mut f64_0: f64 = -143.168534f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut u32_2: u32 = 10u32;
    let mut u8_6: u8 = 59u8;
    let mut u8_7: u8 = 92u8;
    let mut u8_8: u8 = 44u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_1: i64 = -14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_4: i32 = 140i32;
    let mut i64_2: i64 = 13i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut u16_1: u16 = 96u16;
    let mut i32_5: i32 = 21i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_3);
    let mut i64_3: i64 = 135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut u32_3: u32 = 83u32;
    let mut u8_9: u8 = 12u8;
    let mut u8_10: u8 = 59u8;
    let mut u8_11: u8 = 0u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i128_0: i128 = -25i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i8_3: i8 = 93i8;
    let mut i8_4: i8 = -124i8;
    let mut i8_5: i8 = 118i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_6: i32 = 106i32;
    let mut i64_4: i64 = -82i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_8);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_12: u8 = crate::date::Date::sunday_based_week(date_4);
    panic!("From RustyUnit with love");
}
}