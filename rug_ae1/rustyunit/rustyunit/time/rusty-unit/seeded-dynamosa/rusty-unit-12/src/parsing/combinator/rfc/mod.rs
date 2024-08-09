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
#[timeout(30000)]fn rusty_test_9135() {
//    rusty_monitor::set_test_id(9135);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 367u16;
    let mut i32_0: i32 = 70i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i64_1: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = 1000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut f64_1: f64 = 4741671816366391296.000000f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut i32_1: i32 = -100i32;
    let mut i64_3: i64 = 2147483647i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_9);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_4: i64 = 253402300799i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 57u8;
    let mut u8_5: u8 = 24u8;
    let mut u16_1: u16 = 10u16;
    let mut i32_2: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_5, u8_4, u8_3, u32_1);
//    panic!("From RustyUnit with love");
}
}