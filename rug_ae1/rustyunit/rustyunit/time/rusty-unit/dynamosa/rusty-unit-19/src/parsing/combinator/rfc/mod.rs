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
fn rusty_test_1500() {
    rusty_monitor::set_test_id(1500);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -10i64;
    let mut i64_1: i64 = 4i64;
    let mut i64_2: i64 = 20i64;
    let mut str_0: &str = "XVAvQMZvG";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 73i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_0: i32 = -104i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i8_0: i8 = 55i8;
    let mut i8_1: i8 = -61i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 25i8;
    let mut i8_4: i8 = -33i8;
    let mut i8_5: i8 = -46i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 95u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 95u32;
    let mut u8_6: u8 = 13u8;
    let mut u8_7: u8 = 28u8;
    let mut u8_8: u8 = 36u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i128_0: i128 = 71i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 28u16;
    let mut i32_1: i32 = -114i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_2: i32 = -50i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_3: i32 = 108i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut u32_3: u32 = 99u32;
    let mut u8_9: u8 = 15u8;
    let mut u8_10: u8 = 99u8;
    let mut u8_11: u8 = 9u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_6: i8 = 89i8;
    let mut i8_7: i8 = 53i8;
    let mut i8_8: i8 = -63i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_4: i64 = 74i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i64_5: i64 = -12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut u32_4: u32 = 40u32;
    let mut u8_12: u8 = 81u8;
    let mut u8_13: u8 = 74u8;
    let mut u8_14: u8 = 34u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_4);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_6: i64 = -9i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_4);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_1, duration_7);
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    panic!("From RustyUnit with love");
}
}