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
fn rusty_test_7773() {
    rusty_monitor::set_test_id(7773);
    let mut i64_0: i64 = -37i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i128_0: i128 = 253i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 12i8;
    let mut i8_1: i8 = -7i8;
    let mut i8_2: i8 = 9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 71.225837f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -190i32;
    let mut i64_2: i64 = 1i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut i64_3: i64 = -66i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut u32_0: u32 = 99u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 68i8;
    let mut i8_4: i8 = 67i8;
    let mut i8_5: i8 = -1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 45i8;
    let mut i8_7: i8 = -41i8;
    let mut i8_8: i8 = -72i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}
}