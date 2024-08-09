/*!
 * Various scanning routines for the parser.
 */
#![allow(deprecated)]
use super::{ParseResult, INVALID, OUT_OF_RANGE, TOO_SHORT};
use crate::Weekday;
/// Returns true when two slices are equal case-insensitively (in ASCII).
/// Assumes that the `pattern` is already converted to lower case.
fn equals(s: &[u8], pattern: &str) -> bool {
    let mut xs = s
        .iter()
        .map(|&c| match c {
            b'A'..=b'Z' => c + 32,
            _ => c,
        });
    let mut ys = pattern.as_bytes().iter().cloned();
    loop {
        match (xs.next(), ys.next()) {
            (None, None) => return true,
            (None, _) | (_, None) => return false,
            (Some(x), Some(y)) if x != y => return false,
            _ => {}
        }
    }
}
/// Tries to parse the non-negative number from `min` to `max` digits.
///
/// The absence of digits at all is an unconditional error.
/// More than `max` digits are consumed up to the first `max` digits.
/// Any number that does not fit in `i64` is an error.
#[inline]
pub(super) fn number(s: &str, min: usize, max: usize) -> ParseResult<(&str, i64)> {
    assert!(min <= max);
    let bytes = s.as_bytes();
    if bytes.len() < min {
        return Err(TOO_SHORT);
    }
    let mut n = 0i64;
    for (i, c) in bytes.iter().take(max).cloned().enumerate() {
        if !c.is_ascii_digit() {
            if i < min {
                return Err(INVALID);
            } else {
                return Ok((&s[i..], n));
            }
        }
        n = match n.checked_mul(10).and_then(|n| n.checked_add((c - b'0') as i64)) {
            Some(n) => n,
            None => return Err(OUT_OF_RANGE),
        };
    }
    Ok((&s[core::cmp::min(max, bytes.len())..], n))
}
/// Tries to consume at least one digits as a fractional second.
/// Returns the number of whole nanoseconds (0--999,999,999).
pub(super) fn nanosecond(s: &str) -> ParseResult<(&str, i64)> {
    let origlen = s.len();
    let (s, v) = number(s, 1, 9)?;
    let consumed = origlen - s.len();
    static SCALE: [i64; 10] = [
        0,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000,
        10_000,
        1_000,
        100,
        10,
        1,
    ];
    let v = v.checked_mul(SCALE[consumed]).ok_or(OUT_OF_RANGE)?;
    let s = s.trim_left_matches(|c: char| c.is_ascii_digit());
    Ok((s, v))
}
/// Tries to consume a fixed number of digits as a fractional second.
/// Returns the number of whole nanoseconds (0--999,999,999).
pub(super) fn nanosecond_fixed(s: &str, digits: usize) -> ParseResult<(&str, i64)> {
    let (s, v) = number(s, digits, digits)?;
    static SCALE: [i64; 10] = [
        0,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000,
        10_000,
        1_000,
        100,
        10,
        1,
    ];
    let v = v.checked_mul(SCALE[digits]).ok_or(OUT_OF_RANGE)?;
    Ok((s, v))
}
/// Tries to parse the month index (0 through 11) with the first three ASCII letters.
pub(super) fn short_month0(s: &str) -> ParseResult<(&str, u8)> {
    if s.len() < 3 {
        return Err(TOO_SHORT);
    }
    let buf = s.as_bytes();
    let month0 = match (buf[0] | 32, buf[1] | 32, buf[2] | 32) {
        (b'j', b'a', b'n') => 0,
        (b'f', b'e', b'b') => 1,
        (b'm', b'a', b'r') => 2,
        (b'a', b'p', b'r') => 3,
        (b'm', b'a', b'y') => 4,
        (b'j', b'u', b'n') => 5,
        (b'j', b'u', b'l') => 6,
        (b'a', b'u', b'g') => 7,
        (b's', b'e', b'p') => 8,
        (b'o', b'c', b't') => 9,
        (b'n', b'o', b'v') => 10,
        (b'd', b'e', b'c') => 11,
        _ => return Err(INVALID),
    };
    Ok((&s[3..], month0))
}
/// Tries to parse the weekday with the first three ASCII letters.
pub(super) fn short_weekday(s: &str) -> ParseResult<(&str, Weekday)> {
    if s.len() < 3 {
        return Err(TOO_SHORT);
    }
    let buf = s.as_bytes();
    let weekday = match (buf[0] | 32, buf[1] | 32, buf[2] | 32) {
        (b'm', b'o', b'n') => Weekday::Mon,
        (b't', b'u', b'e') => Weekday::Tue,
        (b'w', b'e', b'd') => Weekday::Wed,
        (b't', b'h', b'u') => Weekday::Thu,
        (b'f', b'r', b'i') => Weekday::Fri,
        (b's', b'a', b't') => Weekday::Sat,
        (b's', b'u', b'n') => Weekday::Sun,
        _ => return Err(INVALID),
    };
    Ok((&s[3..], weekday))
}
/// Tries to parse the month index (0 through 11) with short or long month names.
/// It prefers long month names to short month names when both are possible.
pub(super) fn short_or_long_month0(s: &str) -> ParseResult<(&str, u8)> {
    static LONG_MONTH_SUFFIXES: [&str; 12] = [
        "uary",
        "ruary",
        "ch",
        "il",
        "",
        "e",
        "y",
        "ust",
        "tember",
        "ober",
        "ember",
        "ember",
    ];
    let (mut s, month0) = short_month0(s)?;
    let suffix = LONG_MONTH_SUFFIXES[month0 as usize];
    if s.len() >= suffix.len() && equals(&s.as_bytes()[..suffix.len()], suffix) {
        s = &s[suffix.len()..];
    }
    Ok((s, month0))
}
/// Tries to parse the weekday with short or long weekday names.
/// It prefers long weekday names to short weekday names when both are possible.
pub(super) fn short_or_long_weekday(s: &str) -> ParseResult<(&str, Weekday)> {
    static LONG_WEEKDAY_SUFFIXES: [&str; 7] = [
        "day",
        "sday",
        "nesday",
        "rsday",
        "day",
        "urday",
        "day",
    ];
    let (mut s, weekday) = short_weekday(s)?;
    let suffix = LONG_WEEKDAY_SUFFIXES[weekday.num_days_from_monday() as usize];
    if s.len() >= suffix.len() && equals(&s.as_bytes()[..suffix.len()], suffix) {
        s = &s[suffix.len()..];
    }
    Ok((s, weekday))
}
/// Tries to consume exactly one given character.
pub(super) fn char(s: &str, c1: u8) -> ParseResult<&str> {
    match s.as_bytes().first() {
        Some(&c) if c == c1 => Ok(&s[1..]),
        Some(_) => Err(INVALID),
        None => Err(TOO_SHORT),
    }
}
/// Tries to consume one or more whitespace.
pub(super) fn space(s: &str) -> ParseResult<&str> {
    let s_ = s.trim_left();
    if s_.len() < s.len() {
        Ok(s_)
    } else if s.is_empty() {
        Err(TOO_SHORT)
    } else {
        Err(INVALID)
    }
}
/// Returns slice remaining after first char.
/// If <=1 chars in `s` then return an empty slice
pub(super) fn s_next(s: &str) -> &str {
    match s.char_indices().nth(1) {
        Some((offset, _)) => &s[offset..],
        None => &s[s.len()..],
    }
}
/// If the first `char` is whitespace then consume it and return `s`.
/// Else return `s`.
pub(super) fn trim1(s: &str) -> &str {
    match s.chars().next() {
        Some(c) if c.is_whitespace() => s_next(s),
        Some(_) | None => s,
    }
}
/// Consumes one colon char `:` if it is at the front of `s`.
/// Always returns `Ok(s)`.
pub(super) fn consume_colon_maybe(mut s: &str) -> ParseResult<&str> {
    if s.is_empty() {
        return Ok(s);
    }
    if s.starts_with(':') {
        s = s_next(s);
    }
    Ok(s)
}
/// Tries to parse `[-+]\d\d` continued by `\d\d`. Return an offset in seconds if possible.
///
/// The additional `colon` may be used to parse a mandatory or optional `:`
/// between hours and minutes, and should return either a new suffix or `Err` when parsing fails.
pub(super) fn timezone_offset<F>(s: &str, consume_colon: F) -> ParseResult<(&str, i32)>
where
    F: FnMut(&str) -> ParseResult<&str>,
{
    timezone_offset_internal(s, consume_colon, false)
}
fn timezone_offset_internal<F>(
    mut s: &str,
    mut consume_colon: F,
    allow_missing_minutes: bool,
) -> ParseResult<(&str, i32)>
where
    F: FnMut(&str) -> ParseResult<&str>,
{
    const fn digits(s: &str) -> ParseResult<(u8, u8)> {
        let b = s.as_bytes();
        if b.len() < 2 { Err(TOO_SHORT) } else { Ok((b[0], b[1])) }
    }
    let negative = match s.as_bytes().first() {
        Some(&b'+') => false,
        Some(&b'-') => true,
        Some(_) => return Err(INVALID),
        None => return Err(TOO_SHORT),
    };
    s = &s[1..];
    let hours = match digits(s)? {
        (h1 @ b'0'..=b'9', h2 @ b'0'..=b'9') => i32::from((h1 - b'0') * 10 + (h2 - b'0')),
        _ => return Err(INVALID),
    };
    s = &s[2..];
    s = consume_colon(s)?;
    let minutes = if let Ok(ds) = digits(s) {
        match ds {
            (m1 @ b'0'..=b'5', m2 @ b'0'..=b'9') => {
                i32::from((m1 - b'0') * 10 + (m2 - b'0'))
            }
            (b'6'..=b'9', b'0'..=b'9') => return Err(OUT_OF_RANGE),
            _ => return Err(INVALID),
        }
    } else if allow_missing_minutes {
        0
    } else {
        return Err(TOO_SHORT);
    };
    s = match s.len() {
        len if len >= 2 => &s[2..],
        len if len == 0 => s,
        _ => return Err(TOO_SHORT),
    };
    let seconds = hours * 3600 + minutes * 60;
    Ok((s, if negative { -seconds } else { seconds }))
}
/// Same as `timezone_offset` but also allows for `z`/`Z` which is the same as `+00:00`.
pub(super) fn timezone_offset_zulu<F>(s: &str, colon: F) -> ParseResult<(&str, i32)>
where
    F: FnMut(&str) -> ParseResult<&str>,
{
    let bytes = s.as_bytes();
    match bytes.first() {
        Some(&b'z') | Some(&b'Z') => Ok((&s[1..], 0)),
        Some(&b'u') | Some(&b'U') => {
            if bytes.len() >= 3 {
                let (b, c) = (bytes[1], bytes[2]);
                match (b | 32, c | 32) {
                    (b't', b'c') => Ok((&s[3..], 0)),
                    _ => Err(INVALID),
                }
            } else {
                Err(INVALID)
            }
        }
        _ => timezone_offset(s, colon),
    }
}
/// Same as `timezone_offset` but also allows for `z`/`Z` which is the same as
/// `+00:00`, and allows missing minutes entirely.
pub(super) fn timezone_offset_permissive<F>(
    s: &str,
    colon: F,
) -> ParseResult<(&str, i32)>
where
    F: FnMut(&str) -> ParseResult<&str>,
{
    match s.as_bytes().first() {
        Some(&b'z') | Some(&b'Z') => Ok((&s[1..], 0)),
        _ => timezone_offset_internal(s, colon, true),
    }
}
/// Same as `timezone_offset` but also allows for RFC 2822 legacy timezones.
/// May return `None` which indicates an insufficient offset data (i.e. `-0000`).
/// See [RFC 2822 Section 4.3].
///
/// [RFC 2822 Section 4.3]: https://tools.ietf.org/html/rfc2822#section-4.3
pub(super) fn timezone_offset_2822(s: &str) -> ParseResult<(&str, Option<i32>)> {
    let upto = s
        .as_bytes()
        .iter()
        .position(|&c| !c.is_ascii_alphabetic())
        .unwrap_or(s.len());
    if upto > 0 {
        let name = &s.as_bytes()[..upto];
        let s = &s[upto..];
        let offset_hours = |o| Ok((s, Some(o * 3600)));
        if equals(name, "gmt") || equals(name, "ut") {
            offset_hours(0)
        } else if equals(name, "edt") {
            offset_hours(-4)
        } else if equals(name, "est") || equals(name, "cdt") {
            offset_hours(-5)
        } else if equals(name, "cst") || equals(name, "mdt") {
            offset_hours(-6)
        } else if equals(name, "mst") || equals(name, "pdt") {
            offset_hours(-7)
        } else if equals(name, "pst") {
            offset_hours(-8)
        } else if name.len() == 1 {
            match name[0] {
                b'a'..=b'i' | b'k'..=b'z' | b'A'..=b'I' | b'K'..=b'Z' => offset_hours(0),
                _ => Ok((s, None)),
            }
        } else {
            Ok((s, None))
        }
    } else {
        let (s_, offset) = timezone_offset(s, |s| Ok(s))?;
        Ok((s_, Some(offset)))
    }
}
/// Tries to consume everything until next whitespace-like symbol.
/// Does not provide any offset information from the consumed data.
pub(super) fn timezone_name_skip(s: &str) -> ParseResult<(&str, ())> {
    Ok((s.trim_left_matches(|c: char| !c.is_whitespace()), ()))
}
/// Tries to consume an RFC2822 comment including preceding ` `.
///
/// Returns the remaining string after the closing parenthesis.
pub(super) fn comment_2822(s: &str) -> ParseResult<(&str, ())> {
    use CommentState::*;
    let s = s.trim_start();
    let mut state = Start;
    for (i, c) in s.bytes().enumerate() {
        state = match (state, c) {
            (Start, b'(') => Next(1),
            (Next(1), b')') => return Ok((&s[i + 1..], ())),
            (Next(depth), b'\\') => Escape(depth),
            (Next(depth), b'(') => Next(depth + 1),
            (Next(depth), b')') => Next(depth - 1),
            (Next(depth), _) | (Escape(depth), _) => Next(depth),
            _ => return Err(INVALID),
        };
    }
    Err(TOO_SHORT)
}
enum CommentState {
    Start,
    Next(usize),
    Escape(usize),
}
#[cfg(test)]
#[test]
fn test_rfc2822_comments() {
    let testdata = [
        ("", Err(TOO_SHORT)),
        (" ", Err(TOO_SHORT)),
        ("x", Err(INVALID)),
        ("(", Err(TOO_SHORT)),
        ("()", Ok("")),
        (" \r\n\t()", Ok("")),
        ("() ", Ok(" ")),
        ("()z", Ok("z")),
        ("(x)", Ok("")),
        ("(())", Ok("")),
        ("((()))", Ok("")),
        ("(x(x(x)x)x)", Ok("")),
        ("( x ( x ( x ) x ) x )", Ok("")),
        (r"(\)", Err(TOO_SHORT)),
        (r"(\()", Ok("")),
        (r"(\))", Ok("")),
        (r"(\\)", Ok("")),
        ("(()())", Ok("")),
        ("( x ( x ) x ( x ) x )", Ok("")),
    ];
    for (test_in, expected) in testdata.iter() {
        let actual = comment_2822(test_in).map(|(s, _)| s);
        assert_eq!(
            * expected, actual, "{:?} expected to produce {:?}, but produced {:?}.",
            test_in, expected, actual
        );
    }
}
#[test]
fn test_space() {
    assert_eq!(space(""), Err(TOO_SHORT));
    assert_eq!(space(" "), Ok(""));
    assert_eq!(space(" \t"), Ok(""));
    assert_eq!(space(" \ta"), Ok("a"));
    assert_eq!(space(" \ta "), Ok("a "));
    assert_eq!(space("a"), Err(INVALID));
    assert_eq!(space("a "), Err(INVALID));
}
#[test]
fn test_s_next() {
    assert_eq!(s_next(""), "");
    assert_eq!(s_next(" "), "");
    assert_eq!(s_next("a"), "");
    assert_eq!(s_next("ab"), "b");
    assert_eq!(s_next("abc"), "bc");
    assert_eq!(s_next("😾b"), "b");
    assert_eq!(s_next("a😾"), "😾");
    assert_eq!(s_next("😾bc"), "bc");
    assert_eq!(s_next("a😾c"), "😾c");
}
#[test]
fn test_trim1() {
    assert_eq!(trim1(""), "");
    assert_eq!(trim1(" "), "");
    assert_eq!(trim1("\t"), "");
    assert_eq!(trim1("\t\t"), "\t");
    assert_eq!(trim1("  "), " ");
    assert_eq!(trim1("a"), "a");
    assert_eq!(trim1("a "), "a ");
    assert_eq!(trim1("ab"), "ab");
    assert_eq!(trim1("😼"), "😼");
    assert_eq!(trim1("😼b"), "😼b");
}
#[test]
fn test_consume_colon_maybe() {
    assert_eq!(consume_colon_maybe(""), Ok(""));
    assert_eq!(consume_colon_maybe(" "), Ok(" "));
    assert_eq!(consume_colon_maybe("\n"), Ok("\n"));
    assert_eq!(consume_colon_maybe("  "), Ok("  "));
    assert_eq!(consume_colon_maybe(":"), Ok(""));
    assert_eq!(consume_colon_maybe(" :"), Ok(" :"));
    assert_eq!(consume_colon_maybe(": "), Ok(" "));
    assert_eq!(consume_colon_maybe(" : "), Ok(" : "));
    assert_eq!(consume_colon_maybe(":  "), Ok("  "));
    assert_eq!(consume_colon_maybe("  :"), Ok("  :"));
    assert_eq!(consume_colon_maybe(":: "), Ok(": "));
    assert_eq!(consume_colon_maybe("😸"), Ok("😸"));
    assert_eq!(consume_colon_maybe("😸😸"), Ok("😸😸"));
    assert_eq!(consume_colon_maybe("😸:"), Ok("😸:"));
    assert_eq!(consume_colon_maybe("😸 "), Ok("😸 "));
    assert_eq!(consume_colon_maybe(":😸"), Ok("😸"));
    assert_eq!(consume_colon_maybe(":😸 "), Ok("😸 "));
    assert_eq!(consume_colon_maybe(": 😸"), Ok(" 😸"));
    assert_eq!(consume_colon_maybe(":  😸"), Ok("  😸"));
    assert_eq!(consume_colon_maybe(": :😸"), Ok(" :😸"));
}
#[cfg(test)]
mod tests_llm_16_327 {
    use super::*;
    use crate::*;
    use crate::format::scan::{number, INVALID, OUT_OF_RANGE, TOO_SHORT};
    #[test]
    fn test_number_min_max_digits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(&str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(number(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Ok(("", 12345)));
        debug_assert_eq!(number(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Ok(("", 12345)));
        debug_assert_eq!(number(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), Ok(("", 123)));
        debug_assert_eq!(number(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), Ok(("", 123)));
        debug_assert_eq!(
            number(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14), Ok(("6", 12345))
        );
        debug_assert_eq!(
            number(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17), Err(OUT_OF_RANGE)
        );
             }
});    }
    #[test]
    fn test_number_min_digits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(&str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(number(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Ok(("", 1)));
        debug_assert_eq!(number(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Ok(("", 12)));
        debug_assert_eq!(number(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), Err(TOO_SHORT));
        debug_assert_eq!(number(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), Err(INVALID));
        debug_assert_eq!(number(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14), Ok(("a", 0)));
        debug_assert_eq!(number(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17), Err(INVALID));
        debug_assert_eq!(
            number(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20), Ok(("a456", 123))
        );
             }
});    }
    #[test]
    fn test_number_max_digits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, usize, usize, &str, usize, usize, &str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(number(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Ok(("", 12345)));
        debug_assert_eq!(
            number(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Ok(("67890", 12345))
        );
        debug_assert_eq!(number(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), Ok(("", 123)));
             }
});    }
    #[test]
    fn test_number_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(&str, usize, usize, &str, usize, usize, &str, usize, usize, &str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            number(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Ok(("", 999999999999999999))
        );
        debug_assert_eq!(number(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Err(OUT_OF_RANGE));
        debug_assert_eq!(number(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), Ok(("", 12)));
        debug_assert_eq!(number(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), Err(TOO_SHORT));
             }
});    }
    #[test]
    fn test_number_with_leading_zeros() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, usize, usize, &str, usize, usize, &str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(number(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Ok(("", 123)));
        debug_assert_eq!(number(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Ok(("", 0)));
        debug_assert_eq!(number(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), Ok(("456", 123)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_330 {
    use super::*;
    use crate::*;
    #[test]
    fn test_short_or_long_month0_with_short_names() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(short_or_long_month0(rug_fuzz_0), Ok(("", 0)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_1), Ok(("", 1)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_2), Ok(("", 2)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_3), Ok(("", 3)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_4), Ok(("", 4)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_5), Ok(("", 5)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_6), Ok(("", 6)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_7), Ok(("", 7)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_8), Ok(("", 8)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_9), Ok(("", 9)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_10), Ok(("", 10)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_11), Ok(("", 11)));
             }
});    }
    #[test]
    fn test_short_or_long_month0_with_long_names() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(short_or_long_month0(rug_fuzz_0), Ok(("", 0)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_1), Ok(("", 1)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_2), Ok(("", 2)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_3), Ok(("", 3)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_4), Ok(("", 4)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_5), Ok(("", 5)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_6), Ok(("", 6)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_7), Ok(("", 7)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_8), Ok(("", 8)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_9), Ok(("", 9)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_10), Ok(("", 10)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_11), Ok(("", 11)));
             }
});    }
    #[test]
    fn test_short_or_long_month0_with_remaining_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(short_or_long_month0(rug_fuzz_0), Ok((" snow", 0)));
        debug_assert_eq!(short_or_long_month0(rug_fuzz_1), Ok((" rain", 1)));
             }
});    }
    #[test]
    fn test_short_or_long_month0_with_incorrect_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(short_or_long_month0(rug_fuzz_0).is_err());
        debug_assert!(short_or_long_month0(rug_fuzz_1).is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_331 {
    use super::*;
    use crate::*;
    use crate::Weekday;
    fn iso_to_weekday(iso_weekday: u8) -> Option<Weekday> {
        match iso_weekday {
            1 => Some(Weekday::Mon),
            2 => Some(Weekday::Tue),
            3 => Some(Weekday::Wed),
            4 => Some(Weekday::Thu),
            5 => Some(Weekday::Fri),
            6 => Some(Weekday::Sat),
            7 => Some(Weekday::Sun),
            _ => None,
        }
    }
    #[test]
    fn test_long_weekday_names() {
        let weekdays = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        for (i, &day) in weekdays.iter().enumerate() {
            if let Some(weekday) = iso_to_weekday((i as u8) + 1) {
                if let Ok((remaining, parsed_weekday)) = short_or_long_weekday(day) {
                    assert_eq!(remaining, "");
                    assert_eq!(parsed_weekday, weekday);
                } else {
                    panic!("Failed to parse long weekday name: {}", day);
                }
            } else {
                panic!("Invalid ISO weekday number");
            }
        }
    }
    #[test]
    fn test_short_weekday_names() {
        let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        for (i, &day) in weekdays.iter().enumerate() {
            if let Some(weekday) = iso_to_weekday((i as u8) + 1) {
                if let Ok((remaining, parsed_weekday)) = short_or_long_weekday(day) {
                    assert_eq!(remaining, "");
                    assert_eq!(parsed_weekday, weekday);
                } else {
                    panic!("Failed to parse short weekday name: {}", day);
                }
            } else {
                panic!("Invalid ISO weekday number");
            }
        }
    }
    #[test]
    fn test_mixed_weekday_names() {
        let weekdays = [
            ("Mon", "day"),
            ("Tue", "sday"),
            ("Wed", "nesday"),
            ("Thu", "rsday"),
            ("Fri", "day"),
            ("Sat", "urday"),
            ("Sun", "day"),
        ];
        for (i, &(short, long_suffix)) in weekdays.iter().enumerate() {
            if let Some(weekday) = iso_to_weekday((i as u8) + 1) {
                let long_name = format!("{}{}", short, long_suffix);
                if let Ok((remaining, parsed_weekday))
                    = short_or_long_weekday(&long_name) {
                    assert_eq!(remaining, "");
                    assert_eq!(parsed_weekday, weekday);
                } else {
                    panic!("Failed to parse mixed weekday name: {}", long_name);
                }
            } else {
                panic!("Invalid ISO weekday number");
            }
        }
    }
    #[test]
    fn test_invalid_weekday_names() {
        let invalid_weekdays = [
            "Mond",
            "Tuesd",
            "Wenesday",
            "Thuday",
            "Fryday",
            "Sturday",
            "Sunay",
        ];
        for &day in invalid_weekdays.iter() {
            assert!(
                short_or_long_weekday(day).is_err(),
                "Invalid weekday name \"{}\" should not be parsed", day
            );
        }
    }
}
#[cfg(test)]
mod tests_llm_16_336 {
    use super::*;
    use crate::*;
    #[test]
    fn test_timezone_offset_2822_gmt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(0))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_edt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(- 14400))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_est() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(- 18000))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_cst() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(- 21600))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_mst() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(- 25200))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_pst() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(- 28800))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_empty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", None)));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_non_alpha() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("1234", None)));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_single_alpha() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("", Some(0))));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_single_non_alpha() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("1", None)));
             }
});    }
    #[test]
    fn test_timezone_offset_2822_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(timezone_offset_2822(rug_fuzz_0), Ok(("unknown", None)));
             }
});    }
}
#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    #[test]
    fn test_equals() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2_ext, mut rug_fuzz_3, mut rug_fuzz_4_ext, mut rug_fuzz_5, mut rug_fuzz_6_ext, mut rug_fuzz_7)) = <([u8; 7], &str, [u8; 7], &str, [u8; 7], &str, [u8; 9], &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
let rug_fuzz_2 = & rug_fuzz_2_ext;
let rug_fuzz_4 = & rug_fuzz_4_ext;
let rug_fuzz_6 = & rug_fuzz_6_ext;
        let p0: &[u8] = rug_fuzz_0;
        let p1: &str = rug_fuzz_1;
        debug_assert!(crate ::format::scan::equals(p0, & p1));
        let p0: &[u8] = rug_fuzz_2;
        let p1: &str = rug_fuzz_3;
        debug_assert!(crate ::format::scan::equals(p0, & p1));
        let p0: &[u8] = rug_fuzz_4;
        let p1: &str = rug_fuzz_5;
        debug_assert!(crate ::format::scan::equals(p0, & p1));
        let p0: &[u8] = rug_fuzz_6;
        let p1: &str = rug_fuzz_7;
        debug_assert!(! crate ::format::scan::equals(p0, & p1));
             }
});    }
}
#[cfg(test)]
mod tests_rug_73 {
    use super::*;
    #[test]
    fn test_nanosecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        debug_assert!(matches!(crate ::format::scan::nanosecond(p0), Ok(_)));
             }
});    }
}
#[cfg(test)]
mod tests_rug_74 {
    use super::*;
    #[test]
    fn test_nanosecond_fixed() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        let p1: usize = rug_fuzz_1;
        let result = crate::format::scan::nanosecond_fixed(p0, p1);
        debug_assert_eq!(result, Ok(("", 123456789)));
             }
});    }
}
#[cfg(test)]
mod tests_rug_75 {
    use super::*;
    use crate::format::scan::{self, ParseResult, TOO_SHORT, INVALID};
    #[test]
    fn test_short_month0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        debug_assert_eq!(scan::short_month0(p0), Ok(("", 5)));
        p0 = rug_fuzz_1;
        debug_assert_eq!(scan::short_month0(p0), Ok(("", 11)));
        p0 = rug_fuzz_2;
        debug_assert!(matches!(scan::short_month0(p0), Err(INVALID)));
        p0 = rug_fuzz_3;
        debug_assert!(matches!(scan::short_month0(p0), Err(TOO_SHORT)));
             }
});    }
}
#[cfg(test)]
mod tests_rug_76 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        crate::format::scan::short_weekday(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_77 {
    use super::*;
    use crate::format::scan::{char, ParseResult, INVALID, TOO_SHORT};
    #[test]
    fn test_char() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, u8, &str, u8, &str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        let p1: u8 = rug_fuzz_1;
        debug_assert_eq!(char(p0, p1), Ok("bc"));
        let p0: &str = rug_fuzz_2;
        let p1: u8 = rug_fuzz_3;
        debug_assert_eq!(char(p0, p1), Err(INVALID));
        let p0: &str = rug_fuzz_4;
        let p1: u8 = rug_fuzz_5;
        debug_assert_eq!(char(p0, p1), Err(TOO_SHORT));
             }
});    }
}
#[cfg(test)]
mod tests_rug_78 {
    use super::*;
    const TOO_SHORT: &str = "too short";
    const INVALID: &str = "invalid";
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        crate::format::scan::space(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_79 {
    use crate::format::scan::s_next;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        debug_assert_eq!(s_next(p0), ", world!");
             }
});    }
}
#[cfg(test)]
mod tests_rug_80 {
    use super::*;
    #[test]
    fn test_trim1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        debug_assert_eq!(trim1(p0), "hello world");
             }
});    }
}
#[cfg(test)]
mod tests_rug_81 {
    use super::*;
    use super::s_next;
    #[test]
    fn test_consume_colon_maybe() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        debug_assert_eq!(
            Ok(rug_fuzz_1), crate ::format::scan::consume_colon_maybe(& p0)
        );
        let mut p0: &str = rug_fuzz_2;
        debug_assert_eq!(
            Ok(rug_fuzz_3), crate ::format::scan::consume_colon_maybe(& p0)
        );
        let mut p0: &str = rug_fuzz_4;
        debug_assert_eq!(
            Ok(rug_fuzz_5), crate ::format::scan::consume_colon_maybe(& p0)
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_87 {
    use super::*;
    #[test]
    fn test_timezone_name_skip() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        debug_assert_eq!(timezone_name_skip(p0).unwrap(), ("GMT  ", ()));
             }
});    }
}
#[cfg(test)]
mod tests_rug_88 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        crate::format::scan::comment_2822(&p0);
             }
});    }
}
