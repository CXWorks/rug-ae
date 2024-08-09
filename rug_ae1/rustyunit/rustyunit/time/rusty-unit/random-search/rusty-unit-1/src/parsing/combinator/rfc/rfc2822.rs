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
fn rusty_test_4499() {
    rusty_monitor::set_test_id(4499);
    let mut i64_0: i64 = -22i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u8_0: u8 = 76u8;
    let mut i64_1: i64 = 78i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i8_0: i8 = 29i8;
    let mut i8_1: i8 = 107i8;
    let mut i8_2: i8 = 75i8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 9u32;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 23u8;
    let mut u8_3: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut f32_0: f32 = 36.200776f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -61i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 25i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_4: i64 = 7i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i128_0: i128 = 41i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_3: i8 = -26i8;
    let mut i8_4: i8 = 45i8;
    let mut i8_5: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_5: i64 = 56i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i8_6: i8 = 79i8;
    let mut i8_7: i8 = 8i8;
    let mut i8_8: i8 = 8i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_6: i64 = -38i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i128_1: i128 = 39i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut i64_7: i64 = 205i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i128_2: i128 = 92i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_13);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut i8_9: i8 = -2i8;
    let mut i8_10: i8 = -75i8;
    let mut i8_11: i8 = -2i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i128_3: i128 = 81i128;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_3);
    let mut f32_1: f32 = -62.178814f32;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i8_12: i8 = 52i8;
    let mut i8_13: i8 = -51i8;
    let mut i8_14: i8 = -35i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 89u32;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 99u8;
    let mut u8_6: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_19: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_19);
    let mut i64_8: i64 = -29i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 2i32;
    let mut i32_1: i32 = -18i32;
    let mut i64_9: i64 = 129i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_1);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_22, i32_0);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_23);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_25: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i8_15: i8 = 23i8;
    let mut i8_16: i8 = 35i8;
    let mut i8_17: i8 = -45i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i32_2: i32 = -46i32;
    let mut i64_10: i64 = 94i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_2);
    let mut duration_27: std::time::Duration = crate::duration::Duration::abs_std(duration_26);
    let mut i8_18: i8 = -25i8;
    let mut i8_19: i8 = 97i8;
    let mut i8_20: i8 = -67i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_11: i64 = 70i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::microseconds(i64_11);
    let mut i64_12: i64 = -55i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::seconds(i64_12);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 82u32;
    let mut u8_7: u8 = 97u8;
    let mut u8_8: u8 = 34u8;
    let mut u8_9: u8 = 48u8;
    let mut i128_4: i128 = 25i128;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_4);
    let mut i64_13: i64 = 80i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_13);
    let mut i64_14: i64 = -20i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::days(i64_14);
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_32, duration_31);
    let mut i64_15: i64 = 55i64;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_15);
    let mut duration_35: std::time::Duration = crate::duration::Duration::abs_std(duration_34);
    let mut i8_21: i8 = -77i8;
    let mut i8_22: i8 = -53i8;
    let mut i8_23: i8 = 92i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f64_0: f64 = 22.151593f64;
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_37: std::time::Duration = crate::duration::Duration::abs_std(duration_36);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_38: crate::duration::Duration = crate::instant::Instant::elapsed(instant_6);
    let mut duration_39: std::time::Duration = crate::duration::Duration::abs_std(duration_38);
    let mut i8_24: i8 = -58i8;
    let mut i8_25: i8 = -80i8;
    let mut i8_26: i8 = -45i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_8);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_40: crate::duration::Duration = crate::instant::Instant::elapsed(instant_7);
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_3: u32 = 22u32;
    let mut u8_10: u8 = 72u8;
    let mut u8_11: u8 = 41u8;
    let mut u8_12: u8 = 96u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 90u32;
    let mut u8_13: u8 = 67u8;
    let mut u8_14: u8 = 9u8;
    let mut u8_15: u8 = 34u8;
    let mut u32_5: u32 = 27u32;
    let mut u8_16: u8 = 47u8;
    let mut u8_17: u8 = 66u8;
    let mut u8_18: u8 = 11u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_18, u8_17, u8_16, u32_5);
    let mut u32_6: u32 = 90u32;
    let mut u8_19: u8 = 71u8;
    let mut u8_20: u8 = 53u8;
    let mut u8_21: u8 = 36u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_21, u8_20, u8_19, u32_6);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    panic!("From RustyUnit with love");
}
}