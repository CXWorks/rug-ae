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
#[timeout(30000)]fn rusty_test_4899() {
//    rusty_monitor::set_test_id(4899);
    let mut i128_0: i128 = 152i128;
    let mut i64_0: i64 = -10i64;
    let mut i32_0: i32 = 5119853i32;
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 150i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i32_2: i32 = 359i32;
    let mut i64_1: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 126i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i128_1: i128 = 0i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u16_1: u16 = 367u16;
    let mut i32_3: i32 = 7i32;
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = -11i8;
    let mut i8_5: i8 = 60i8;
    let mut i64_3: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 12u8;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_4: i64 = -97i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_6);
    let mut i8_6: i8 = 2i8;
    let mut i8_7: i8 = 79i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_7, i8_3);
    let mut i8_8: i8 = 55i8;
    let mut i8_9: i8 = 5i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_4, i8_8, i8_5);
    let mut i64_5: i64 = 1000000000i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i8_10: i8 = 2i8;
    let mut i8_11: i8 = -45i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_10, i8_11);
    let mut u8_2: u8 = 4u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_7);
    let mut i32_4: i32 = 122i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
//    panic!("From RustyUnit with love");
}
}