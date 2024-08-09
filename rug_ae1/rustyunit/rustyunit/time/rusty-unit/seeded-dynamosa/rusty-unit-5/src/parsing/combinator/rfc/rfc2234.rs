//! Rules defined in [RFC 2234].
//!
//! [RFC 2234]: https://datatracker.ietf.org/doc/html/rfc2234

use crate::parsing::ParsedItem;

/// Consume exactly one space or tab.
pub(crate) const fn wsp(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    match input {
        [b' ' | b'\t', rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6743() {
//    rusty_monitor::set_test_id(6743);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 5i8;
    let mut i8_3: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 331i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = 25i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 2i8;
    let mut i8_6: i8 = -20i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut u16_0: u16 = 366u16;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 47u8;
    let mut i32_1: i32 = 2147483647i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = 127i8;
    let mut i8_9: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i8_10: i8 = 1i8;
    let mut i8_11: i8 = 0i8;
    let mut i8_12: i8 = 4i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_11, i8_10);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_3);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i128_1: i128 = 0i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_2: i32 = 392i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_2};
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_2, u8_2, u8_1, u8_0, u16_0);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
//    panic!("From RustyUnit with love");
}
}