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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8230() {
    rusty_monitor::set_test_id(8230);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 1u8;
    let mut i64_0: i64 = 49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u32_1: u32 = 27u32;
    let mut u8_3: u8 = 42u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 83i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = 28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 65i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = -56i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -22i8;
    let mut i8_7: i8 = -27i8;
    let mut i8_8: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 65i8;
    let mut i8_10: i8 = -124i8;
    let mut i8_11: i8 = 121i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_2: u32 = 66u32;
    let mut u8_6: u8 = 46u8;
    let mut u8_7: u8 = 73u8;
    let mut u8_8: u8 = 86u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_12: i8 = -103i8;
    let mut i8_13: i8 = 15i8;
    let mut i8_14: i8 = 70i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 111i8;
    let mut i8_16: i8 = 28i8;
    let mut i8_17: i8 = 114i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut f32_0: f32 = -68.319723f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -9.289847f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = 183.654628f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_1: i64 = -234i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_3: u32 = 80u32;
    let mut u8_9: u8 = 77u8;
    let mut u8_10: u8 = 18u8;
    let mut u8_11: u8 = 77u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i32_0: i32 = 71i32;
    let mut i64_2: i64 = -129i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_1: i32 = -165i32;
    let mut i64_3: i64 = -76i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut u8_12: u8 = 72u8;
    let mut i32_2: i32 = -37i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut i32_3: i32 = -135i32;
    let mut i32_4: i32 = 187i32;
    let mut i64_4: i64 = 50i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i32_5: i32 = -18i32;
    let mut i64_5: i64 = 138i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_5);
    let mut i8_18: i8 = -5i8;
    let mut i8_19: i8 = 13i8;
    let mut i8_20: i8 = 90i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_6);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u32_4: u32 = 1u32;
    let mut u8_13: u8 = 86u8;
    let mut u8_14: u8 = 68u8;
    let mut u8_15: u8 = 65u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_15, u8_14, u8_13, u32_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_3};
    let mut u8_16: u8 = 83u8;
    let mut u8_17: u8 = 18u8;
    let mut u8_18: u8 = 21u8;
    let mut u16_0: u16 = 88u16;
    let mut i32_6: i32 = -102i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut i32_7: i32 = -72i32;
    let mut i64_6: i64 = -17i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i32_8: i32 = 48i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut i32_9: i32 = 66i32;
    let mut f64_1: f64 = 75.060164f64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_13, i32_9);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_7: i64 = -41i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i8_21: i8 = 65i8;
    let mut i8_22: i8 = -89i8;
    let mut i8_23: i8 = -7i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_7, utcoffset_7);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_10: i32 = crate::date::Date::year(date_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_12, i32_7);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_3, u8_18, u8_17, u8_16);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_12);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}
}