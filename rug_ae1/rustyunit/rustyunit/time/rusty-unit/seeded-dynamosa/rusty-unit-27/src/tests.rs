#![allow(
    clippy::let_underscore_drop,
    clippy::clone_on_copy,
    clippy::cognitive_complexity
)]

//! Tests for internal details.
//!
//! This module should only be used when it is not possible to test the implementation in a
//! reasonable manner externally.

use std::num::NonZeroU8;

use crate::format_description::modifier::Modifiers;
use crate::format_description::FormatItem;
use crate::formatting::DigitCount;
use crate::parsing::shim::Integer;
use crate::{duration, parsing};

#[test]
fn digit_count() {
    assert_eq!(1_u8.num_digits(), 1);
    assert_eq!(9_u8.num_digits(), 1);
    assert_eq!(10_u8.num_digits(), 2);
    assert_eq!(99_u8.num_digits(), 2);
    assert_eq!(100_u8.num_digits(), 3);

    assert_eq!(1_u16.num_digits(), 1);
    assert_eq!(9_u16.num_digits(), 1);
    assert_eq!(10_u16.num_digits(), 2);
    assert_eq!(99_u16.num_digits(), 2);
    assert_eq!(100_u16.num_digits(), 3);
    assert_eq!(999_u16.num_digits(), 3);
    assert_eq!(1_000_u16.num_digits(), 4);
    assert_eq!(9_999_u16.num_digits(), 4);
    assert_eq!(10_000_u16.num_digits(), 5);

    assert_eq!(1_u32.num_digits(), 1);
    assert_eq!(9_u32.num_digits(), 1);
    assert_eq!(10_u32.num_digits(), 2);
    assert_eq!(99_u32.num_digits(), 2);
    assert_eq!(100_u32.num_digits(), 3);
    assert_eq!(999_u32.num_digits(), 3);
    assert_eq!(1_000_u32.num_digits(), 4);
    assert_eq!(9_999_u32.num_digits(), 4);
    assert_eq!(10_000_u32.num_digits(), 5);
    assert_eq!(99_999_u32.num_digits(), 5);
    assert_eq!(100_000_u32.num_digits(), 6);
    assert_eq!(999_999_u32.num_digits(), 6);
    assert_eq!(1_000_000_u32.num_digits(), 7);
    assert_eq!(9_999_999_u32.num_digits(), 7);
    assert_eq!(10_000_000_u32.num_digits(), 8);
    assert_eq!(99_999_999_u32.num_digits(), 8);
    assert_eq!(100_000_000_u32.num_digits(), 9);
    assert_eq!(999_999_999_u32.num_digits(), 9);
    assert_eq!(1_000_000_000_u32.num_digits(), 10);
}

#[test]
fn default() {
    assert_eq!(
        duration::Padding::Optimize.clone(),
        duration::Padding::default()
    );
}

#[test]
fn debug() {
    let _ = format!("{:?}", duration::Padding::Optimize);
    let _ = format!("{:?}", Modifiers::default());
    let _ = format!(
        "{:?}",
        crate::format_description::parse::ParsedItem {
            item: FormatItem::Literal(b""),
            remaining: b""
        }
    );
    let _ = format!("{:?}", parsing::ParsedItem(b"", 0));
    let _ = format!("{:?}", parsing::component::Period::Am);
}

#[test]
fn clone() {
    assert_eq!(
        parsing::component::Period::Am.clone(),
        parsing::component::Period::Am
    );
    // does not impl Debug
    assert!(crate::time::Padding::Optimize.clone() == crate::time::Padding::Optimize);
}

#[test]
fn parsing_internals() {
    assert!(
        parsing::ParsedItem(b"", ())
            .flat_map(|_| None::<()>)
            .is_none()
    );
    assert!(<NonZeroU8 as Integer>::parse_bytes(b"256").is_none());
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8633() {
//    rusty_monitor::set_test_id(8633);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 122i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 30u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 10u8;
    let mut i16_0: i16 = 8i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_1: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 400i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_1: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_2: i64 = 86400i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_6);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i128_2: i128 = 1000000i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut i32_3: i32 = 134i32;
    let mut i64_3: i64 = 132i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_3);
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = -73i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 2i8;
    let mut i8_7: i8 = 1i8;
    let mut i8_8: i8 = 46i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_6, utcoffset_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_3);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_8);
    let mut tuple_1: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_add(time_2, duration_9);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut tuple_2: (i32, u8) = crate::date::Date::iso_year_week(date_1);
//    panic!("From RustyUnit with love");
}
}