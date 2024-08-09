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
fn rusty_test_7049() {
    rusty_monitor::set_test_id(7049);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 230i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -95i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_0: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_2: i64 = 92i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_3: i64 = -2i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = -94i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i64_5: i64 = -88i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_7);
    let mut i32_1: i32 = 79i32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_1);
    let mut i64_6: i64 = -101i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_10);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut i16_0: i16 = -76i16;
    let mut i64_7: i64 = -87i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_11_ref_0: &crate::duration::Duration = &mut duration_11;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    panic!("From RustyUnit with love");
}
}