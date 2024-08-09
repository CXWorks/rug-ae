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
#[timeout(30000)]fn rusty_test_8070() {
//    rusty_monitor::set_test_id(8070);
    let mut i64_0: i64 = -40i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut f32_0: f32 = 52.411992f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_0: i32 = 268i32;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 14u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = -101i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u16_0: u16 = 34u16;
    let mut i32_1: i32 = 6i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_0);
    let mut primitivedatetime_0_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_4: i64 = 2440588i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i32_2: i32 = 8i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_8);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 7u8;
    let mut u8_4: u8 = 8u8;
    let mut u8_5: u8 = 24u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 367u16;
    let mut i32_3: i32 = 381i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_6, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_5);
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_5: i64 = -19i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i128_0: i128 = 0i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_4: i32 = 86399i32;
    let mut i64_6: i64 = 2147483647i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_4);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_14, duration_13);
    let mut i64_7: i64 = 2440588i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_16);
    let mut date_7: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_add(date_7, duration_15);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_3, duration_7);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
//    panic!("From RustyUnit with love");
}
}