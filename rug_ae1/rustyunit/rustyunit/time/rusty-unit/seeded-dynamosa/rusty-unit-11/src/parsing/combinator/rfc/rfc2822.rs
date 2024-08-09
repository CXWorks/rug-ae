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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8314() {
//    rusty_monitor::set_test_id(8314);
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 144i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 235i32;
    let mut i32_2: i32 = 207i32;
    let mut i64_2: i64 = 2147483647i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut i8_6: i8 = 28i8;
    let mut i8_7: i8 = -27i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 0i8;
    let mut i8_10: i8 = 4i8;
    let mut i8_11: i8 = 127i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_3: i64 = 193i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_12: i8 = -9i8;
    let mut i8_13: i8 = 49i8;
    let mut i8_14: i8 = -34i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_4: i64 = 12i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i8_15: i8 = 75i8;
    let mut i8_16: i8 = 23i8;
    let mut i8_17: i8 = 1i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 23i8;
    let mut i8_19: i8 = 2i8;
    let mut i8_20: i8 = 23i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_5: i64 = 604800i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i8_21: i8 = 59i8;
    let mut i8_22: i8 = 2i8;
    let mut i8_23: i8 = 33i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_2: u32 = 100000000u32;
    let mut u8_6: u8 = 28u8;
    let mut u8_7: u8 = 2u8;
    let mut u8_8: u8 = 12u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i128_0: i128 = 0i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_24: i8 = 0i8;
    let mut i8_25: i8 = 3i8;
    let mut i8_26: i8 = 4i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_3: i32 = 178i32;
    let mut i64_6: i64 = 1000000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_3);
    let mut i64_7: i64 = 1i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i8_27: i8 = 0i8;
    let mut i8_28: i8 = 59i8;
    let mut i8_29: i8 = 3i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 1000u32;
    let mut u8_9: u8 = 2u8;
    let mut u8_10: u8 = 1u8;
    let mut u8_11: u8 = 32u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_30: i8 = 59i8;
    let mut i8_31: i8 = 5i8;
    let mut i8_32: i8 = 5i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 5i8;
    let mut i8_34: i8 = 78i8;
    let mut i8_35: i8 = 24i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i64_8: i64 = 2440588i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut i32_4: i32 = 111i32;
    let mut i64_9: i64 = 1000i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_14, i32_4);
    let mut u32_4: u32 = 100000u32;
    let mut u8_12: u8 = 10u8;
    let mut u8_13: u8 = 23u8;
    let mut u8_14: u8 = 52u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_36: i8 = 35i8;
    let mut i8_37: i8 = 60i8;
    let mut i8_38: i8 = 5i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i8_39: i8 = 23i8;
    let mut i8_40: i8 = 24i8;
    let mut i8_41: i8 = -33i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i8_42: i8 = 60i8;
    let mut i8_43: i8 = 2i8;
    let mut i8_44: i8 = 60i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i8_45: i8 = 6i8;
    let mut i8_46: i8 = 5i8;
    let mut i8_47: i8 = 2i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i64_10: i64 = 24i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut i64_11: i64 = 60i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_17, duration_16);
    let mut u32_5: u32 = 1000000u32;
    let mut u8_15: u8 = 9u8;
    let mut u8_16: u8 = 31u8;
    let mut u8_17: u8 = 29u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i64_12: i64 = 42i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::minutes(i64_12);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i32_5: i32 = -124i32;
    let mut i128_1: i128 = 1000000i128;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_6: u32 = 999999u32;
    let mut u8_18: u8 = 23u8;
    let mut u8_19: u8 = 0u8;
    let mut u8_20: u8 = 6u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut f64_1: f64 = 4652007308841189376.000000f64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_21);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_6);
    let mut i8_48: i8 = 3i8;
    let mut i8_49: i8 = -120i8;
    let mut i8_50: i8 = 5i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut u16_0: u16 = 366u16;
    let mut i32_6: i32 = 20i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_16);
    let mut time_7: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_5, time_7);
    let mut i128_2: i128 = 9223372036854775807i128;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_9);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_14};
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_8);
    let mut u8_21: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_2);
    let mut u8_22: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_7);
//    panic!("From RustyUnit with love");
}
}