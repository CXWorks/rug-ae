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
#[timeout(30000)]fn rusty_test_8376() {
//    rusty_monitor::set_test_id(8376);
    let mut i32_0: i32 = 5119853i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -64i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = 119i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_3: i64 = 604800i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_1: i32 = 218i32;
    let mut i64_4: i64 = 18i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i32_2: i32 = 6i32;
    let mut i64_5: i64 = 2440588i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut f64_0: f64 = -107.049868f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut f64_1: f64 = 4607182418800017408.000000f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u8_3: u8 = 37u8;
    let mut i32_3: i32 = 224i32;
    let mut f64_2: f64 = 4828193600913801216.000000f64;
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = -33i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 999999u32;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 4u8;
    let mut u8_6: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i64_6: i64 = 12i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i32_4: i32 = 376i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_13);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_2);
    let mut offsetdatetime_4_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_4;
    let mut i8_6: i8 = 1i8;
    let mut i8_7: i8 = 4i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut u16_0: u16 = 81u16;
    let mut i32_5: i32 = 308i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_14);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i8_9: i8 = 3i8;
    let mut i8_10: i8 = 24i8;
    let mut i8_11: i8 = 63i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_2: u32 = 1000u32;
    let mut u8_7: u8 = 60u8;
    let mut u8_8: u8 = 14u8;
    let mut u8_9: u8 = 79u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i32_6: i32 = 285i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_3, weekday_0);
//    panic!("From RustyUnit with love");
}
}