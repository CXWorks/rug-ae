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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5157() {
//    rusty_monitor::set_test_id(5157);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_0: u8 = 24u8;
    let mut i32_0: i32 = 54i32;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut u8_1: u8 = 7u8;
    let mut i32_1: i32 = 144i32;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::next(weekday_3);
    let mut u8_2: u8 = 59u8;
    let mut u32_0: u32 = 1000000u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut u16_0: u16 = 30u16;
    let mut i32_2: i32 = 5119853i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i64_0: i64 = 64i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_3: i32 = 7i32;
    let mut i32_4: i32 = 133i32;
    let mut i64_1: i64 = -4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_6: u8 = 5u8;
    let mut i32_5: i32 = 150i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_6, weekday_5);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_5, u8_2, weekday_4);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_1, weekday_2);
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
//    panic!("From RustyUnit with love");
}
}