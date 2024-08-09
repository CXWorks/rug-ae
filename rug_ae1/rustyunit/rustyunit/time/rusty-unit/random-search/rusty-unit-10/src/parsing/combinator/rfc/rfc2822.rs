//! Rules defined in [RFC 2822].
//!
//! [RFC 2822]: https://datatracker.ietf.org/doc/html/rfc2822

use crate::parsing::combinator::rfc::rfc2234::wsp;
use crate::parsing::combinator::{ascii_char, one_or_more, zero_or_more};
use crate::parsing::ParsedItem;

/// Consume the `fws` rule.
// The full rule is equivalent to /\r\n[ \t]+|[ \t]+(?:\r\n[ \t]+)*/
pub(crate) fn fws(mut input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    if let [b'\r', b'\n', rest @ ..] = input {
        one_or_more(wsp)(rest)
    } else {
        input = one_or_more(wsp)(input)?.into_inner();
        while let [b'\r', b'\n', rest @ ..] = input {
            input = one_or_more(wsp)(rest)?.into_inner();
        }
        Some(ParsedItem(input, ()))
    }
}

/// Consume the `cfws` rule.
// The full rule is equivalent to any combination of `fws` and `comment` so long as it is not empty.
pub(crate) fn cfws(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    one_or_more(|input| fws(input).or_else(|| comment(input)))(input)
}

/// Consume the `comment` rule.
fn comment(mut input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    input = ascii_char::<b'('>(input)?.into_inner();
    input = zero_or_more(fws)(input).into_inner();
    while let Some(rest) = ccontent(input) {
        input = rest.into_inner();
        input = zero_or_more(fws)(input).into_inner();
    }
    input = ascii_char::<b')'>(input)?.into_inner();

    Some(ParsedItem(input, ()))
}

/// Consume the `ccontent` rule.
fn ccontent(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    ctext(input)
        .or_else(|| quoted_pair(input))
        .or_else(|| comment(input))
}

/// Consume the `ctext` rule.
fn ctext(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    no_ws_ctl(input).or_else(|| match input {
        [33..=39 | 42..=91 | 93..=126, rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    })
}

/// Consume the `quoted_pair` rule.
fn quoted_pair(mut input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    input = ascii_char::<b'\\'>(input)?.into_inner();

    let old_input_len = input.len();

    input = text(input).into_inner();

    // If nothing is parsed, this means we hit the `obs-text` rule and nothing matched. This is
    // technically a success, but we should still check the `obs-qp` rule to ensure we consume
    // everything possible.
    if input.len() == old_input_len {
        match input {
            [0..=127, rest @ ..] => Some(ParsedItem(rest, ())),
            _ => Some(ParsedItem(input, ())),
        }
    } else {
        Some(ParsedItem(input, ()))
    }
}

/// Consume the `no_ws_ctl` rule.
const fn no_ws_ctl(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    match input {
        [1..=8 | 11..=12 | 14..=31 | 127, rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    }
}

/// Consume the `text` rule.
fn text<'a>(input: &'a [u8]) -> ParsedItem<'a, ()> {
    let new_text = |input: &'a [u8]| match input {
        [1..=9 | 11..=12 | 14..=127, rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    };

    let obs_char = |input: &'a [u8]| match input {
        // This is technically allowed, but consuming this would mean the rest of the string is
        // eagerly consumed without consideration for where the comment actually ends.
        [b')', ..] => None,
        [0..=9 | 11..=12 | 14..=127, rest @ ..] => Some(rest),
        _ => None,
    };

    let obs_text = |mut input| {
        input = zero_or_more(ascii_char::<b'\n'>)(input).into_inner();
        input = zero_or_more(ascii_char::<b'\r'>)(input).into_inner();
        while let Some(rest) = obs_char(input) {
            input = rest;
            input = zero_or_more(ascii_char::<b'\n'>)(input).into_inner();
            input = zero_or_more(ascii_char::<b'\r'>)(input).into_inner();
        }

        ParsedItem(input, ())
    };

    new_text(input).unwrap_or_else(|| obs_text(input))
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3404() {
    rusty_monitor::set_test_id(3404);
    let mut i32_0: i32 = -65i32;
    let mut i64_0: i64 = 9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = -4i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 21u8;
    let mut u8_1: u8 = 39u8;
    let mut u8_2: u8 = 45u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 31u16;
    let mut i32_2: i32 = 15i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i64_1: i64 = 18i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 37u8;
    let mut u8_4: u8 = 41u8;
    let mut u8_5: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i32_3: i32 = -168i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut i8_0: i8 = 54i8;
    let mut i64_3: i64 = -131i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut f32_0: f32 = -125.536427f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u16_1: u16 = 50u16;
    let mut i32_4: i32 = -19i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_5);
    let mut i64_4: i64 = crate::offset_date_time::OffsetDateTime::unix_timestamp(offsetdatetime_3);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3420() {
    rusty_monitor::set_test_id(3420);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 10u8;
    let mut i64_0: i64 = 85i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -14i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = -88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -51i32;
    let mut i64_1: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i64_2: i64 = 69i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut f32_0: f32 = -140.261924f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_3: i8 = 83i8;
    let mut i8_4: i8 = 58i8;
    let mut i8_5: i8 = -13i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -41i8;
    let mut i8_7: i8 = -92i8;
    let mut i8_8: i8 = 56i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 65u32;
    let mut u8_3: u8 = 25u8;
    let mut u8_4: u8 = 54u8;
    let mut u8_5: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = 76i8;
    let mut i8_10: i8 = -17i8;
    let mut i8_11: i8 = -28i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_1: i32 = 26i32;
    let mut i64_3: i64 = -35i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut i8_12: i8 = -22i8;
    let mut i8_13: i8 = -117i8;
    let mut i8_14: i8 = -44i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_2: u32 = 47u32;
    let mut u8_6: u8 = 77u8;
    let mut u8_7: u8 = 18u8;
    let mut u8_8: u8 = 40u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_4: i64 = -102i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i8_15: i8 = 40i8;
    let mut i8_16: i8 = 13i8;
    let mut i8_17: i8 = -27i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_3: u32 = 90u32;
    let mut u8_9: u8 = 21u8;
    let mut u8_10: u8 = 76u8;
    let mut u8_11: u8 = 56u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_5: i64 = -10i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i8_18: i8 = -9i8;
    let mut i8_19: i8 = -98i8;
    let mut i8_20: i8 = -6i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_6: i64 = 174i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut i8_21: i8 = 14i8;
    let mut i8_22: i8 = 30i8;
    let mut i8_23: i8 = 30i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_4: u32 = 48u32;
    let mut u8_12: u8 = 14u8;
    let mut u8_13: u8 = 90u8;
    let mut u8_14: u8 = 60u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_24: i8 = -124i8;
    let mut i8_25: i8 = 92i8;
    let mut i8_26: i8 = 8i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = -62i8;
    let mut i8_28: i8 = -22i8;
    let mut i8_29: i8 = 32i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i32_2: i32 = 95i32;
    let mut i64_7: i64 = 107i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_2);
    let mut i32_3: i32 = 98i32;
    let mut i64_8: i64 = -22i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_3);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::abs(duration_10);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_30: i8 = -34i8;
    let mut i8_31: i8 = 37i8;
    let mut i8_32: i8 = -85i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut u32_5: u32 = 78u32;
    let mut u8_15: u8 = 42u8;
    let mut u8_16: u8 = 70u8;
    let mut u8_17: u8 = 44u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i8_33: i8 = -119i8;
    let mut i8_34: i8 = -102i8;
    let mut i8_35: i8 = 51i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = -112i8;
    let mut i8_37: i8 = -4i8;
    let mut i8_38: i8 = 64i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i64_9: i64 = -81i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_9);
    let mut i64_10: i64 = 56i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_10);
    let mut i64_11: i64 = -46i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_11);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_14, duration_13);
    let mut i32_4: i32 = 90i32;
    let mut i32_5: i32 = 93i32;
    let mut i64_12: i64 = 107i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new(i64_12, i32_5);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_16, i32_4);
    let mut u32_6: u32 = 42u32;
    let mut u8_18: u8 = 52u8;
    let mut u8_19: u8 = 52u8;
    let mut u8_20: u8 = 75u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i32_6: i32 = 16i32;
    let mut i64_13: i64 = 12i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new(i64_13, i32_6);
    let mut i8_39: i8 = 32i8;
    let mut i8_40: i8 = -7i8;
    let mut i8_41: i8 = -93i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i8_42: i8 = -117i8;
    let mut i8_43: i8 = -101i8;
    let mut i8_44: i8 = 83i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i8_45: i8 = 55i8;
    let mut i8_46: i8 = 116i8;
    let mut i8_47: i8 = -64i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i8_48: i8 = 48i8;
    let mut i8_49: i8 = 87i8;
    let mut i8_50: i8 = -11i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i128_0: i128 = -128i128;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_7: i32 = -18i32;
    let mut i64_14: i64 = -7i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_14, i32_7);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_20, duration_19);
    let mut u32_7: u32 = 39u32;
    let mut u8_21: u8 = 23u8;
    let mut u8_22: u8 = 46u8;
    let mut u8_23: u8 = 6u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut i32_8: i32 = -227i32;
    let mut i64_15: i64 = 93i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_15, i32_8);
    let mut i8_51: i8 = -15i8;
    let mut i8_52: i8 = -93i8;
    let mut i8_53: i8 = 42i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut i8_54: i8 = 10i8;
    let mut i8_55: i8 = 26i8;
    let mut i8_56: i8 = 2i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut u32_8: u32 = 92u32;
    let mut u8_24: u8 = 41u8;
    let mut u8_25: u8 = 73u8;
    let mut u8_26: u8 = 43u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_26, u8_25, u8_24, u32_8);
    let mut i64_16: i64 = -91i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::hours(i64_16);
    let mut i8_57: i8 = -107i8;
    let mut i8_58: i8 = 21i8;
    let mut i8_59: i8 = -9i8;
    let mut utcoffset_19: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_59, i8_58, i8_57);
    let mut i8_60: i8 = 116i8;
    let mut i8_61: i8 = -14i8;
    let mut i8_62: i8 = 60i8;
    let mut utcoffset_20: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_62, i8_61, i8_60);
    let mut i64_17: i64 = -60i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::weeks(i64_17);
    let mut f64_0: f64 = -3.369950f64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_25, duration_24);
    let mut i8_63: i8 = -98i8;
    let mut i8_64: i8 = 27i8;
    let mut i8_65: i8 = 119i8;
    let mut utcoffset_21: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_65, i8_64, i8_63);
    let mut i64_18: i64 = 56i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::days(i64_18);
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::abs(duration_27);
    let mut i8_66: i8 = 12i8;
    let mut i8_67: i8 = -68i8;
    let mut i8_68: i8 = -10i8;
    let mut utcoffset_22: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_68, i8_67, i8_66);
    let mut i64_19: i64 = 199i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::hours(i64_19);
    let mut u32_9: u32 = 3u32;
    let mut u8_27: u8 = 82u8;
    let mut u8_28: u8 = 11u8;
    let mut u8_29: u8 = 71u8;
    let mut time_8: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_29, u8_28, u8_27, u32_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_30: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_20: i64 = -22i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::hours(i64_20);
    let mut duration_32: std::time::Duration = crate::duration::Duration::abs_std(duration_31);
    let mut u32_10: u32 = 52u32;
    let mut u8_30: u8 = 54u8;
    let mut u8_31: u8 = 4u8;
    let mut u8_32: u8 = 21u8;
    let mut time_9: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_32, u8_31, u8_30, u32_10);
    let mut i32_9: i32 = -89i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_9);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_9);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}
}