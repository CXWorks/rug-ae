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
fn rusty_test_6548() {
    rusty_monitor::set_test_id(6548);
    let mut i8_0: i8 = 47i8;
    let mut i8_1: i8 = -34i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -14i8;
    let mut i8_4: i8 = -13i8;
    let mut i8_5: i8 = 19i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 49u16;
    let mut i32_0: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 20u16;
    let mut i32_1: i32 = 10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_6: i8 = 101i8;
    let mut i8_7: i8 = 102i8;
    let mut i8_8: i8 = -40i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_2: i32 = -188i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 35u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = 75i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_2: u16 = 92u16;
    let mut i32_3: i32 = -155i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_2);
    let mut i32_4: i32 = -98i32;
    let mut i64_0: i64 = -146i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_1);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i8_9: i8 = 42i8;
    let mut i8_10: i8 = -36i8;
    let mut i8_11: i8 = -45i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_5: i32 = -94i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_4);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_5, utcoffset_3);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_6);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i32_6: i32 = -230i32;
    let mut i64_1: i64 = 50i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_6);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_2);
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut i8_12: i8 = 120i8;
    let mut i8_13: i8 = 65i8;
    let mut i8_14: i8 = -91i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_2: i64 = -175i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_4);
    panic!("From RustyUnit with love");
}
}