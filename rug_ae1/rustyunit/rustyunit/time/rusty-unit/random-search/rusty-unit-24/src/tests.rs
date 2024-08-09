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
fn rusty_test_4887() {
    rusty_monitor::set_test_id(4887);
    let mut f64_0: f64 = -183.610436f64;
    let mut i64_0: i64 = 178i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = -69i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
    let mut i8_0: i8 = -24i8;
    let mut i8_1: i8 = 100i8;
    let mut i8_2: i8 = -47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -29i8;
    let mut i8_4: i8 = -7i8;
    let mut i8_5: i8 = -52i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = -20i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i8_6: i8 = 105i8;
    let mut i8_7: i8 = -107i8;
    let mut i8_8: i8 = 26i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = 141i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 77u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 45u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 1i8;
    let mut i8_10: i8 = -8i8;
    let mut i8_11: i8 = 61i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 40i8;
    let mut i8_13: i8 = 41i8;
    let mut i8_14: i8 = 28i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 27u32;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 43u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_4: i64 = 102i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut i8_15: i8 = -100i8;
    let mut i8_16: i8 = 56i8;
    let mut i8_17: i8 = -118i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 65i8;
    let mut i8_19: i8 = 50i8;
    let mut i8_20: i8 = -6i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 2u32;
    let mut u8_6: u8 = 68u8;
    let mut u8_7: u8 = 44u8;
    let mut u8_8: u8 = 30u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut f64_1: f64 = 76.944752f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i8_21: i8 = 116i8;
    let mut i8_22: i8 = 0i8;
    let mut i8_23: i8 = -88i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i64_5: i64 = 32i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = 23i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i128_0: i128 = -60i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -258i32;
    let mut i64_7: i64 = -163i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_0);
    let mut i64_8: i64 = -165i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut i32_1: i32 = 49i32;
    let mut i64_9: i64 = -82i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_15, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_16);
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i128_1: i128 = 75i128;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::abs(duration_17);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_18);
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i64_10: i64 = 144i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds(i64_10);
    let mut i32_2: i32 = -61i32;
    let mut i128_2: i128 = -52i128;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_20, i32_2);
    let mut i64_11: i64 = -45i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::microseconds(i64_11);
    let mut i32_3: i32 = -144i32;
    let mut i64_12: i64 = -30i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_12, i32_3);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_23);
    let mut i64_13: i64 = -114i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_13);
    let mut i8_24: i8 = 86i8;
    let mut i8_25: i8 = -9i8;
    let mut i8_26: i8 = -37i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_6, utcoffset_10);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut i64_14: i64 = -113i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut duration_27: std::time::Duration = crate::duration::Duration::abs_std(duration_26);
    panic!("From RustyUnit with love");
}
}