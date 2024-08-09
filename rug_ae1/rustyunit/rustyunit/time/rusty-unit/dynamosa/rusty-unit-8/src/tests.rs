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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5761() {
    rusty_monitor::set_test_id(5761);
    let mut i32_0: i32 = -55i32;
    let mut i64_0: i64 = -104i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = 72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_1: i32 = -123i32;
    let mut i64_3: i64 = -152i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 76i8;
    let mut i8_2: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -38.884742f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_4: i64 = 75i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_2: i32 = 11i32;
    let mut i64_5: i64 = -15i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_2);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_3: i32 = -41i32;
    let mut i64_6: i64 = 130i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_3);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i8_3: i8 = -76i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = -38i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_4: i32 = -16i32;
    let mut i32_5: i32 = -5i32;
    let mut i64_7: i64 = 74i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_4);
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 36u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 64i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_6: i8 = -24i8;
    let mut i8_7: i8 = -109i8;
    let mut i8_8: i8 = -51i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_8: i64 = 1i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i64_9: i64 = -56i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut i64_10: i64 = 18i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_10);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_17, duration_16);
    let mut duration_19: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut i64_11: i64 = -10i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut i32_6: i32 = 33i32;
    let mut i64_12: i64 = 90i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds(i64_12);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_21, i32_6);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut u16_0: u16 = 1u16;
    let mut i32_7: i32 = 47i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut i8_9: i8 = 33i8;
    let mut i8_10: i8 = 3i8;
    let mut i8_11: i8 = -44i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_8: i32 = 57i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut u32_1: u32 = 37u32;
    let mut u8_3: u8 = 50u8;
    let mut u8_4: u8 = 74u8;
    let mut u8_5: u8 = 66u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_2);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i32_9: i32 = -158i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_9};
    let mut i8_12: i8 = 23i8;
    let mut i8_13: i8 = -43i8;
    let mut i8_14: i8 = -3i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -41i8;
    let mut i8_16: i8 = 26i8;
    let mut i8_17: i8 = -74i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 28u32;
    let mut u8_6: u8 = 91u8;
    let mut u8_7: u8 = 71u8;
    let mut u8_8: u8 = 29u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_10: i32 = -40i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_10};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_6);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_5);
    let mut f32_1: f32 = -13.974023f32;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_25: std::time::Duration = crate::duration::Duration::abs_std(duration_24);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_26: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_11: i32 = -19i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_11};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_4, duration_26);
    let mut i64_13: i64 = 156i64;
    let mut u16_1: u16 = 69u16;
    let mut i32_12: i32 = -171i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_12, u16_1);
    let mut i32_13: i32 = 122i32;
    let mut i64_14: i64 = 25i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::weeks(i64_14);
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_27, i32_13);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_28);
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut i8_18: i8 = 78i8;
    let mut i8_19: i8 = -91i8;
    let mut i8_20: i8 = -71i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_8);
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_6, utcoffset_7);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_9: u8 = 58u8;
    let mut i32_14: i32 = 75i32;
    let mut f64_0: f64 = -32.307070f64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_3: u32 = 19u32;
    let mut u8_10: u8 = 37u8;
    let mut u8_11: u8 = 79u8;
    let mut u8_12: u8 = 98u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut i64_15: i64 = -56i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::seconds(i64_15);
    let mut u16_2: u16 = 49u16;
    let mut i32_15: i32 = 21i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_15, u16_2);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_add(date_7, duration_30);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_4);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_8, duration_29);
    let mut i64_16: i64 = 123i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_16);
    let mut i32_16: i32 = -194i32;
    let mut date_9: crate::date::Date = crate::date::Date {value: i32_16};
    let mut i64_17: i64 = 2i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::weeks(i64_17);
    let mut i64_18: i64 = 26i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::weeks(i64_18);
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_33, duration_32);
    let mut i32_17: i32 = -179i32;
    let mut date_10: crate::date::Date = crate::date::Date {value: i32_17};
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_14, u8_9, weekday_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_7);
    let mut u8_13: u8 = crate::date::Date::sunday_based_week(date_5);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_13);
    let mut i128_1: i128 = crate::duration::Duration::whole_nanoseconds(duration_34);
    panic!("From RustyUnit with love");
}
}