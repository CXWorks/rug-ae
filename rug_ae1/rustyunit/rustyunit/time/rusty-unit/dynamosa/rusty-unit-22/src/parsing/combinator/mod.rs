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
fn rusty_test_1745() {
    rusty_monitor::set_test_id(1745);
    let mut i32_0: i32 = 0i32;
    let mut i64_0: i64 = -60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -120i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -64i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u16_0: u16 = 50u16;
    let mut i32_1: i32 = -88i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u16_1: u16 = 54u16;
    let mut i32_2: i32 = 33i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    panic!("From RustyUnit with love");
}
}