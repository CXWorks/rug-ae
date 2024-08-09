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
#[timeout(30000)]fn rusty_test_8457() {
//    rusty_monitor::set_test_id(8457);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 24u16;
    let mut i32_0: i32 = 342i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_1: i32 = -118i32;
    let mut i128_0: i128 = 1000000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut u16_1: u16 = 20u16;
    let mut i32_2: i32 = 6i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_2);
    let mut i32_3: i32 = 65i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_2: i64 = 0i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_8);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_2);
//    panic!("From RustyUnit with love");
}
}