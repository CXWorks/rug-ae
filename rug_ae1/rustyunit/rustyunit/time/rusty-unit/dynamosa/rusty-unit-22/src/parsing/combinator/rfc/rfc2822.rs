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
fn rusty_test_7583() {
    rusty_monitor::set_test_id(7583);
    let mut i64_0: i64 = -67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 45i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i64_2: i64 = -72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = -54i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u32_1: u32 = 79u32;
    let mut u8_3: u8 = 17u8;
    let mut u8_4: u8 = 85u8;
    let mut u8_5: u8 = 85u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_4: i64 = 154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_0: i32 = -209i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_2: u32 = 92u32;
    let mut u8_6: u8 = 97u8;
    let mut u8_7: u8 = 19u8;
    let mut u8_8: u8 = 94u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_1: i32 = 116i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_5: i64 = 100i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut u16_0: u16 = 29u16;
    let mut i32_2: i32 = 137i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u16_1: u16 = 30u16;
    let mut i32_3: i32 = 158i32;
    let mut i32_4: i32 = -236i32;
    let mut i64_6: i64 = -67i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_5: i32 = 186i32;
    let mut i64_7: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_3, u16_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut i32_6: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_1);
    panic!("From RustyUnit with love");
}
}