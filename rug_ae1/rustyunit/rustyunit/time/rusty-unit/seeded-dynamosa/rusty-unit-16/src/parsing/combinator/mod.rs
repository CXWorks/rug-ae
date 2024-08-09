//! Implementations of the low-level parser combinators.

pub(crate) mod rfc;

use crate::format_description::modifier::Padding;
use crate::parsing::shim::{Integer, IntegerParseBytes};
use crate::parsing::ParsedItem;

/// Parse a "+" or "-" sign. Returns the ASCII byte representing the sign, if present.
pub(crate) const fn sign(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    match input {
        [sign @ (b'-' | b'+'), remaining @ ..] => Some(ParsedItem(remaining, *sign)),
        _ => None,
    }
}

/// Consume the first matching item, returning its associated value.
pub(crate) fn first_match<'a, T>(
    options: impl IntoIterator<Item = (&'a [u8], T)>,
    case_sensitive: bool,
) -> impl FnMut(&'a [u8]) -> Option<ParsedItem<'a, T>> {
    let mut options = options.into_iter();
    move |input| {
        options.find_map(|(expected, t)| {
            if case_sensitive {
                Some(ParsedItem(input.strip_prefix(expected)?, t))
            } else {
                let n = expected.len();
                if n <= input.len() {
                    let (head, tail) = input.split_at(n);
                    if head.eq_ignore_ascii_case(expected) {
                        return Some(ParsedItem(tail, t));
                    }
                }
                None
            }
        })
    }
}

/// Consume zero or more instances of the provided parser. The parser must return the unit value.
pub(crate) fn zero_or_more<'a, P: Fn(&'a [u8]) -> Option<ParsedItem<'a, ()>>>(
    parser: P,
) -> impl FnMut(&'a [u8]) -> ParsedItem<'a, ()> {
    move |mut input| {
        while let Some(remaining) = parser(input) {
            input = remaining.into_inner();
        }
        ParsedItem(input, ())
    }
}

/// Consume one of or more instances of the provided parser. The parser must produce the unit value.
pub(crate) fn one_or_more<'a, P: Fn(&'a [u8]) -> Option<ParsedItem<'a, ()>>>(
    parser: P,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, ()>> {
    move |mut input| {
        input = parser(input)?.into_inner();
        while let Some(remaining) = parser(input) {
            input = remaining.into_inner();
        }
        Some(ParsedItem(input, ()))
    }
}

/// Consume between `n` and `m` instances of the provided parser.
pub(crate) fn n_to_m<
    'a,
    T,
    P: Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>,
    const N: u8,
    const M: u8,
>(
    parser: P,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, &'a [u8]>> {
    debug_assert!(M >= N);
    move |mut input| {
        // We need to keep this to determine the total length eventually consumed.
        let orig_input = input;

        // Mandatory
        for _ in 0..N {
            input = parser(input)?.0;
        }

        // Optional
        for _ in N..M {
            match parser(input) {
                Some(parsed) => input = parsed.0,
                None => break,
            }
        }

        Some(ParsedItem(
            input,
            &orig_input[..(orig_input.len() - input.len())],
        ))
    }
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits<T: Integer, const N: u8, const M: u8>(
    input: &[u8],
) -> Option<ParsedItem<'_, T>> {
    debug_assert!(M >= N);
    n_to_m::<_, _, N, M>(any_digit)(input)?.flat_map(|value| value.parse_bytes())
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits<T: Integer, const N: u8>(input: &[u8]) -> Option<ParsedItem<'_, T>> {
    n_to_m_digits::<_, N, N>(input)
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits_padded<'a, T: Integer, const N: u8>(
    padding: Padding,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, T>> {
    n_to_m_digits_padded::<_, N, N>(padding)
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits_padded<'a, T: Integer, const N: u8, const M: u8>(
    padding: Padding,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, T>> {
    debug_assert!(M >= N);
    move |mut input| match padding {
        Padding::None => n_to_m_digits::<_, 1, M>(input),
        Padding::Space => {
            debug_assert!(N > 0);

            let mut orig_input = input;
            for _ in 0..(N - 1) {
                match ascii_char::<b' '>(input) {
                    Some(parsed) => input = parsed.0,
                    None => break,
                }
            }
            let pad_width = (orig_input.len() - input.len()) as u8;

            orig_input = input;
            for _ in 0..(N - pad_width) {
                input = any_digit(input)?.0;
            }
            for _ in N..M {
                match any_digit(input) {
                    Some(parsed) => input = parsed.0,
                    None => break,
                }
            }

            ParsedItem(input, &orig_input[..(orig_input.len() - input.len())])
                .flat_map(|value| value.parse_bytes())
        }
        Padding::Zero => n_to_m_digits::<_, N, M>(input),
    }
}

/// Consume exactly one digit.
pub(crate) const fn any_digit(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    match input {
        [c, remaining @ ..] if c.is_ascii_digit() => Some(ParsedItem(remaining, *c)),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters.
pub(crate) fn ascii_char<const CHAR: u8>(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    debug_assert!(CHAR.is_ascii_graphic() || CHAR.is_ascii_whitespace());
    match input {
        [c, remaining @ ..] if *c == CHAR => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters, case-insensitive.
pub(crate) fn ascii_char_ignore_case<const CHAR: u8>(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    debug_assert!(CHAR.is_ascii_graphic() || CHAR.is_ascii_whitespace());
    match input {
        [c, remaining @ ..] if c.eq_ignore_ascii_case(&CHAR) => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}

/// Optionally consume an input with a given parser.
pub(crate) fn opt<'a, T>(
    parser: impl Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>,
) -> impl Fn(&'a [u8]) -> ParsedItem<'a, Option<T>> {
    move |input| match parser(input) {
        Some(value) => value.map(Some),
        None => ParsedItem(input, None),
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8959() {
//    rusty_monitor::set_test_id(8959);
    let mut i8_0: i8 = 113i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 127i8;
    let mut i8_4: i8 = -85i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 56i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 115i8;
    let mut i8_10: i8 = 1i8;
    let mut i8_11: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_2: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i8_12: i8 = 24i8;
    let mut i8_13: i8 = 2i8;
    let mut i8_14: i8 = -47i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = 60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i8_15: i8 = 60i8;
    let mut i8_16: i8 = 1i8;
    let mut i8_17: i8 = 0i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_4: i64 = 12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_5: i64 = -101i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut i8_18: i8 = 127i8;
    let mut i8_19: i8 = 1i8;
    let mut i8_20: i8 = 0i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 999999999u32;
    let mut u8_6: u8 = 52u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 11u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i128_1: i128 = 46i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_21: i8 = 1i8;
    let mut i8_22: i8 = 43i8;
    let mut i8_23: i8 = 24i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_3: u32 = 1000000000u32;
    let mut u8_9: u8 = 7u8;
    let mut u8_10: u8 = 62u8;
    let mut u8_11: u8 = 6u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_24: i8 = 4i8;
    let mut i8_25: i8 = 0i8;
    let mut i8_26: i8 = 0i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 24i8;
    let mut i8_28: i8 = 24i8;
    let mut i8_29: i8 = 19i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_6: i64 = 9223372036854775807i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i8_30: i8 = 6i8;
    let mut i8_31: i8 = 3i8;
    let mut i8_32: i8 = 3i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 1i8;
    let mut i8_34: i8 = 1i8;
    let mut i8_35: i8 = 5i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i32_0: i32 = -54i32;
    let mut i64_7: i64 = 1000000000i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_14, i32_0);
    let mut i64_8: i64 = -159i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut i8_36: i8 = 59i8;
    let mut i8_37: i8 = -22i8;
    let mut i8_38: i8 = 2i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut u32_4: u32 = 100000000u32;
    let mut u8_12: u8 = 30u8;
    let mut u8_13: u8 = 6u8;
    let mut u8_14: u8 = 0u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_39: i8 = 60i8;
    let mut i8_40: i8 = 6i8;
    let mut i8_41: i8 = 1i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u32_5: u32 = 100000000u32;
    let mut u8_15: u8 = 11u8;
    let mut u8_16: u8 = 7u8;
    let mut u8_17: u8 = 28u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_17: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i8_42: i8 = 60i8;
    let mut i8_43: i8 = 1i8;
    let mut i8_44: i8 = 23i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i32_1: i32 = 263i32;
    let mut i64_9: i64 = 12i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_1);
    let mut i8_45: i8 = 1i8;
    let mut i8_46: i8 = 59i8;
    let mut i8_47: i8 = 1i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i32_2: i32 = 305i32;
    let mut i64_10: i64 = 1000000i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_2);
    let mut i64_11: i64 = 1000000i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::hours(i64_11);
    let mut u32_6: u32 = 10000000u32;
    let mut u8_18: u8 = 98u8;
    let mut u8_19: u8 = 4u8;
    let mut u8_20: u8 = 44u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i8_48: i8 = 60i8;
    let mut i8_49: i8 = 1i8;
    let mut i8_50: i8 = 127i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i8_51: i8 = 5i8;
    let mut i8_52: i8 = 2i8;
    let mut i8_53: i8 = 1i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut i64_12: i64 = 1000i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::minutes(i64_12);
    let mut i64_13: i64 = 86400i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::days(i64_13);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut i64_14: i64 = 1000000000i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::weeks(i64_14);
    let mut duration_25: std::time::Duration = crate::duration::Duration::abs_std(duration_24);
    let mut i64_15: i64 = 24i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::seconds(i64_15);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_21: u8 = 24u8;
    let mut i32_3: i32 = 36525i32;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut u8_22: u8 = 5u8;
    let mut i32_4: i32 = -46i32;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_23: u8 = 78u8;
    let mut i32_5: i32 = 25i32;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_24: u8 = 12u8;
    let mut i32_6: i32 = 342i32;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_25: u8 = 12u8;
    let mut i32_7: i32 = 314i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_7, u8_25, weekday_5);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_6, u8_24, weekday_4);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_5, u8_23, weekday_3);
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_22, weekday_2);
    let mut result_4: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_21, weekday_0);
//    panic!("From RustyUnit with love");
}
}