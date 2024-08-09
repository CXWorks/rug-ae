//! Date and time parsing routines.
#![allow(deprecated)]
use core::borrow::Borrow;
use core::str;
use core::usize;
use super::scan;
use super::{Fixed, InternalFixed, InternalInternal, Item, Numeric, Pad, Parsed};
use super::{ParseError, ParseErrorKind, ParseResult};
use super::{BAD_FORMAT, INVALID, NOT_ENOUGH, OUT_OF_RANGE, TOO_LONG, TOO_SHORT};
use crate::{DateTime, FixedOffset, Weekday};
fn set_weekday_with_num_days_from_sunday(p: &mut Parsed, v: i64) -> ParseResult<()> {
    p.set_weekday(
        match v {
            0 => Weekday::Sun,
            1 => Weekday::Mon,
            2 => Weekday::Tue,
            3 => Weekday::Wed,
            4 => Weekday::Thu,
            5 => Weekday::Fri,
            6 => Weekday::Sat,
            _ => return Err(OUT_OF_RANGE),
        },
    )
}
fn set_weekday_with_number_from_monday(p: &mut Parsed, v: i64) -> ParseResult<()> {
    p.set_weekday(
        match v {
            1 => Weekday::Mon,
            2 => Weekday::Tue,
            3 => Weekday::Wed,
            4 => Weekday::Thu,
            5 => Weekday::Fri,
            6 => Weekday::Sat,
            7 => Weekday::Sun,
            _ => return Err(OUT_OF_RANGE),
        },
    )
}
/// Parse an RFC 2822 format datetime
/// e.g. `Fri, 21 Nov 1997 09:55:06 -0600`
///
/// This function allows arbitrary intermixed whitespace per RFC 2822 appendix A.5
fn parse_rfc2822<'a>(parsed: &mut Parsed, mut s: &'a str) -> ParseResult<(&'a str, ())> {
    macro_rules! try_consume {
        ($e:expr) => {
            { let (s_, v) = $e ?; s = s_; v }
        };
    }
    s = s.trim_left();
    if let Ok((s_, weekday)) = scan::short_weekday(s) {
        if !s_.starts_with(',') {
            return Err(INVALID);
        }
        s = &s_[1..];
        parsed.set_weekday(weekday)?;
    }
    s = s.trim_left();
    parsed.set_day(try_consume!(scan::number(s, 1, 2)))?;
    s = scan::space(s)?;
    parsed.set_month(1 + i64::from(try_consume!(scan::short_month0(s))))?;
    s = scan::space(s)?;
    let prevlen = s.len();
    let mut year = try_consume!(scan::number(s, 2, usize::MAX));
    let yearlen = prevlen - s.len();
    match (yearlen, year) {
        (2, 0..=49) => {
            year += 2000;
        }
        (2, 50..=99) => {
            year += 1900;
        }
        (3, _) => {
            year += 1900;
        }
        (_, _) => {}
    }
    parsed.set_year(year)?;
    s = scan::space(s)?;
    parsed.set_hour(try_consume!(scan::number(s, 2, 2)))?;
    s = scan::char(s.trim_left(), b':')?.trim_left();
    parsed.set_minute(try_consume!(scan::number(s, 2, 2)))?;
    if let Ok(s_) = scan::char(s.trim_left(), b':') {
        parsed.set_second(try_consume!(scan::number(s_, 2, 2)))?;
    }
    s = scan::space(s)?;
    if let Some(offset) = try_consume!(scan::timezone_offset_2822(s)) {
        parsed.set_offset(i64::from(offset))?;
    }
    while let Ok((s_out, ())) = scan::comment_2822(s) {
        s = s_out;
    }
    Ok((s, ()))
}
fn parse_rfc3339<'a>(parsed: &mut Parsed, mut s: &'a str) -> ParseResult<(&'a str, ())> {
    macro_rules! try_consume {
        ($e:expr) => {
            { let (s_, v) = $e ?; s = s_; v }
        };
    }
    parsed.set_year(try_consume!(scan::number(s, 4, 4)))?;
    s = scan::char(s, b'-')?;
    parsed.set_month(try_consume!(scan::number(s, 2, 2)))?;
    s = scan::char(s, b'-')?;
    parsed.set_day(try_consume!(scan::number(s, 2, 2)))?;
    s = match s.as_bytes().first() {
        Some(&b't') | Some(&b'T') => &s[1..],
        Some(_) => return Err(INVALID),
        None => return Err(TOO_SHORT),
    };
    parsed.set_hour(try_consume!(scan::number(s, 2, 2)))?;
    s = scan::char(s, b':')?;
    parsed.set_minute(try_consume!(scan::number(s, 2, 2)))?;
    s = scan::char(s, b':')?;
    parsed.set_second(try_consume!(scan::number(s, 2, 2)))?;
    if s.starts_with('.') {
        let nanosecond = try_consume!(scan::nanosecond(& s[1..]));
        parsed.set_nanosecond(nanosecond)?;
    }
    let offset = try_consume!(scan::timezone_offset_zulu(s, | s | scan::char(s, b':')));
    if offset <= -86_400 || offset >= 86_400 {
        return Err(OUT_OF_RANGE);
    }
    parsed.set_offset(i64::from(offset))?;
    Ok((s, ()))
}
/// Tries to parse given string into `parsed` with given formatting items.
/// Returns `Ok` when the entire string has been parsed (otherwise `parsed` should not be used).
/// There should be no trailing string after parsing;
/// use a stray [`Item::Space`](./enum.Item.html#variant.Space) to trim whitespaces.
///
/// This particular date and time parser is:
///
/// - Greedy. It will consume the longest possible prefix.
///   For example, `April` is always consumed entirely when the long month name is requested;
///   it equally accepts `Apr`, but prefers the longer prefix in this case.
///
/// - Padding-agnostic (for numeric items).
///   The [`Pad`](./enum.Pad.html) field is completely ignored,
///   so one can prepend any number of zeroes before numbers.
///
/// - (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.
pub fn parse<'a, I, B>(parsed: &mut Parsed, s: &str, items: I) -> ParseResult<()>
where
    I: Iterator<Item = B>,
    B: Borrow<Item<'a>>,
{
    parse_internal(parsed, s, items).map(|_| ()).map_err(|(_s, e)| e)
}
fn parse_internal<'a, 'b, I, B>(
    parsed: &mut Parsed,
    mut s: &'b str,
    items: I,
) -> Result<&'b str, (&'b str, ParseError)>
where
    I: Iterator<Item = B>,
    B: Borrow<Item<'a>>,
{
    macro_rules! try_consume {
        ($e:expr) => {
            { match $e { Ok((s_, v)) => { s = s_; v } Err(e) => return Err((s, e)), } }
        };
    }
    for item in items {
        match *item.borrow() {
            Item::Literal(prefix) => {
                if s.len() < prefix.len() {
                    return Err((s, TOO_SHORT));
                }
                if !s.starts_with(prefix) {
                    return Err((s, INVALID));
                }
                s = &s[prefix.len()..];
            }
            #[cfg(any(feature = "alloc", feature = "std", test))]
            Item::OwnedLiteral(ref prefix) => {
                if s.len() < prefix.len() {
                    return Err((s, TOO_SHORT));
                }
                if !s.starts_with(&prefix[..]) {
                    return Err((s, INVALID));
                }
                s = &s[prefix.len()..];
            }
            Item::Space(item_space) => {
                for expect in item_space.chars() {
                    let actual = match s.chars().next() {
                        Some(c) => c,
                        None => {
                            return Err((s, TOO_SHORT));
                        }
                    };
                    if expect != actual {
                        return Err((s, INVALID));
                    }
                    s = scan::s_next(s);
                }
            }
            #[cfg(any(feature = "alloc", feature = "std", test))]
            Item::OwnedSpace(ref item_space) => {
                for expect in item_space.chars() {
                    let actual = match s.chars().next() {
                        Some(c) => c,
                        None => {
                            return Err((s, TOO_SHORT));
                        }
                    };
                    if expect != actual {
                        return Err((s, INVALID));
                    }
                    s = scan::s_next(s);
                }
            }
            Item::Numeric(ref spec, ref _pad) => {
                use super::Numeric::*;
                type Setter = fn(&mut Parsed, i64) -> ParseResult<()>;
                let (width, signed, set): (usize, bool, Setter) = match *spec {
                    Year => (4, true, Parsed::set_year),
                    YearDiv100 => (2, false, Parsed::set_year_div_100),
                    YearMod100 => (2, false, Parsed::set_year_mod_100),
                    IsoYear => (4, true, Parsed::set_isoyear),
                    IsoYearDiv100 => (2, false, Parsed::set_isoyear_div_100),
                    IsoYearMod100 => (2, false, Parsed::set_isoyear_mod_100),
                    Month => (2, false, Parsed::set_month),
                    Day => (2, false, Parsed::set_day),
                    WeekFromSun => (2, false, Parsed::set_week_from_sun),
                    WeekFromMon => (2, false, Parsed::set_week_from_mon),
                    IsoWeek => (2, false, Parsed::set_isoweek),
                    NumDaysFromSun => (1, false, set_weekday_with_num_days_from_sunday),
                    WeekdayFromMon => (1, false, set_weekday_with_number_from_monday),
                    Ordinal => (3, false, Parsed::set_ordinal),
                    Hour => (2, false, Parsed::set_hour),
                    Hour12 => (2, false, Parsed::set_hour12),
                    Minute => (2, false, Parsed::set_minute),
                    Second => (2, false, Parsed::set_second),
                    Nanosecond => (9, false, Parsed::set_nanosecond),
                    Timestamp => (usize::MAX, false, Parsed::set_timestamp),
                    Internal(ref int) => match int._dummy {}
                };
                let v = if signed {
                    if s.starts_with('-') {
                        let v = try_consume!(scan::number(& s[1..], 1, usize::MAX));
                        0i64.checked_sub(v).ok_or((s, OUT_OF_RANGE))?
                    } else if s.starts_with('+') {
                        try_consume!(scan::number(& s[1..], 1, usize::MAX))
                    } else {
                        try_consume!(scan::number(s, 1, width))
                    }
                } else {
                    try_consume!(scan::number(s, 1, width))
                };
                set(parsed, v).map_err(|e| (s, e))?;
            }
            Item::Fixed(ref spec) => {
                use super::Fixed::*;
                match spec {
                    &ShortMonthName => {
                        let month0 = try_consume!(scan::short_month0(s));
                        parsed.set_month(i64::from(month0) + 1).map_err(|e| (s, e))?;
                    }
                    &LongMonthName => {
                        let month0 = try_consume!(scan::short_or_long_month0(s));
                        parsed.set_month(i64::from(month0) + 1).map_err(|e| (s, e))?;
                    }
                    &ShortWeekdayName => {
                        let weekday = try_consume!(scan::short_weekday(s));
                        parsed.set_weekday(weekday).map_err(|e| (s, e))?;
                    }
                    &LongWeekdayName => {
                        let weekday = try_consume!(scan::short_or_long_weekday(s));
                        parsed.set_weekday(weekday).map_err(|e| (s, e))?;
                    }
                    &LowerAmPm | &UpperAmPm => {
                        if s.len() < 2 {
                            return Err((s, TOO_SHORT));
                        }
                        let ampm = match (s.as_bytes()[0] | 32, s.as_bytes()[1] | 32) {
                            (b'a', b'm') => false,
                            (b'p', b'm') => true,
                            _ => return Err((s, INVALID)),
                        };
                        parsed.set_ampm(ampm).map_err(|e| (s, e))?;
                        s = &s[2..];
                    }
                    &Nanosecond | &Nanosecond3 | &Nanosecond6 | &Nanosecond9 => {
                        if s.starts_with('.') {
                            let nano = try_consume!(scan::nanosecond(& s[1..]));
                            parsed.set_nanosecond(nano).map_err(|e| (s, e))?;
                        }
                    }
                    &Internal(
                        InternalFixed { val: InternalInternal::Nanosecond3NoDot },
                    ) => {
                        if s.len() < 3 {
                            return Err((s, TOO_SHORT));
                        }
                        let nano = try_consume!(scan::nanosecond_fixed(s, 3));
                        parsed.set_nanosecond(nano).map_err(|e| (s, e))?;
                    }
                    &Internal(
                        InternalFixed { val: InternalInternal::Nanosecond6NoDot },
                    ) => {
                        if s.len() < 6 {
                            return Err((s, TOO_SHORT));
                        }
                        let nano = try_consume!(scan::nanosecond_fixed(s, 6));
                        parsed.set_nanosecond(nano).map_err(|e| (s, e))?;
                    }
                    &Internal(
                        InternalFixed { val: InternalInternal::Nanosecond9NoDot },
                    ) => {
                        if s.len() < 9 {
                            return Err((s, TOO_SHORT));
                        }
                        let nano = try_consume!(scan::nanosecond_fixed(s, 9));
                        parsed.set_nanosecond(nano).map_err(|e| (s, e))?;
                    }
                    &TimezoneName => {
                        try_consume!(scan::timezone_name_skip(s));
                    }
                    &TimezoneOffsetColon
                    | &TimezoneOffsetDoubleColon
                    | &TimezoneOffsetTripleColon
                    | &TimezoneOffset => {
                        s = scan::trim1(s);
                        let offset = try_consume!(
                            scan::timezone_offset(s, scan::consume_colon_maybe)
                        );
                        parsed.set_offset(i64::from(offset)).map_err(|e| (s, e))?;
                    }
                    &TimezoneOffsetColonZ | &TimezoneOffsetZ => {
                        s = scan::trim1(s);
                        let offset = try_consume!(
                            scan::timezone_offset_zulu(s, scan::consume_colon_maybe)
                        );
                        parsed.set_offset(i64::from(offset)).map_err(|e| (s, e))?;
                    }
                    &Internal(
                        InternalFixed { val: InternalInternal::TimezoneOffsetPermissive },
                    ) => {
                        s = scan::trim1(s);
                        let offset = try_consume!(
                            scan::timezone_offset_permissive(s,
                            scan::consume_colon_maybe)
                        );
                        parsed.set_offset(i64::from(offset)).map_err(|e| (s, e))?;
                    }
                    &RFC2822 => try_consume!(parse_rfc2822(parsed, s)),
                    &RFC3339 => try_consume!(parse_rfc3339(parsed, s)),
                }
            }
            Item::Error => {
                return Err((s, BAD_FORMAT));
            }
        }
    }
    if !s.is_empty() { Err((s, TOO_LONG)) } else { Ok(s) }
}
/// Accepts a relaxed form of RFC3339.
/// A space or a 'T' are accepted as the separator between the date and time
/// parts.
///
/// ```
/// # use chrono::{DateTime, offset::FixedOffset};
/// "2000-01-02T03:04:05Z".parse::<DateTime<FixedOffset>>();
/// "2000-01-02 03:04:05Z".parse::<DateTime<FixedOffset>>();
/// ```
impl str::FromStr for DateTime<FixedOffset> {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<DateTime<FixedOffset>> {
        const DATE_ITEMS: &[Item<'static>] = &[
            Item::Numeric(Numeric::Year, Pad::Zero),
            Item::Literal("-"),
            Item::Numeric(Numeric::Month, Pad::Zero),
            Item::Literal("-"),
            Item::Numeric(Numeric::Day, Pad::Zero),
        ];
        const TIME_ITEMS: &[Item<'static>] = &[
            Item::Numeric(Numeric::Hour, Pad::Zero),
            Item::Literal(":"),
            Item::Numeric(Numeric::Minute, Pad::Zero),
            Item::Literal(":"),
            Item::Numeric(Numeric::Second, Pad::Zero),
            Item::Fixed(Fixed::Nanosecond),
            Item::Fixed(Fixed::TimezoneOffsetZ),
        ];
        let mut parsed = Parsed::new();
        match parse_internal(&mut parsed, s, DATE_ITEMS.iter()) {
            Err((remainder, e)) if e.0 == ParseErrorKind::TooLong => {
                if remainder.starts_with('T') || remainder.starts_with(' ') {
                    parse(&mut parsed, &remainder[1..], TIME_ITEMS.iter())?;
                } else {
                    return Err(INVALID);
                }
            }
            Err((_s, e)) => return Err(e),
            Ok(_) => return Err(NOT_ENOUGH),
        };
        parsed.to_datetime()
    }
}
#[cfg(test)]
#[test]
fn test_parse() {
    use super::*;
    fn parse_all(s: &str, items: &[Item]) -> ParseResult<Parsed> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, items.iter())?;
        Ok(parsed)
    }
    macro_rules! check {
        ($fmt:expr, $items:expr; $err:tt) => {
            eprintln!("test_parse: format {:?}", $fmt); assert_eq!(parse_all($fmt,
            &$items), Err($err))
        };
        ($fmt:expr, $items:expr; $($k:ident : $v:expr),*) => {
            { eprintln!("test_parse: format {:?}", $fmt); let expected = Parsed { $($k :
            Some($v),)* ..Default::default() }; assert_eq!(parse_all($fmt, &$items),
            Ok(expected)) }
        };
    }
    check!("", [];);
    check!(" ", []; TOO_LONG);
    check!("a", []; TOO_LONG);
    check!("abc", []; TOO_LONG);
    check!("🤠", []; TOO_LONG);
    check!("", [sp!("")];);
    check!(" ", [sp!(" ")];);
    check!("  ", [sp!("  ")];);
    check!("   ", [sp!("   ")];);
    check!(" ", [sp!("")]; TOO_LONG);
    check!("  ", [sp!(" ")]; TOO_LONG);
    check!("   ", [sp!("  ")]; TOO_LONG);
    check!("    ", [sp!("  ")]; TOO_LONG);
    check!("", [sp!(" ")]; TOO_SHORT);
    check!(" ", [sp!("  ")]; TOO_SHORT);
    check!("  ", [sp!("   ")]; TOO_SHORT);
    check!("  ", [sp!("  "), sp!("  ")]; TOO_SHORT);
    check!("   ", [sp!("  "), sp!("  ")]; TOO_SHORT);
    check!("  ", [sp!(" "), sp!(" ")];);
    check!("   ", [sp!("  "), sp!(" ")];);
    check!("   ", [sp!(" "), sp!("  ")];);
    check!("   ", [sp!(" "), sp!(" "), sp!(" ")];);
    check!("\t", [sp!("")]; TOO_LONG);
    check!(" \n\r  \n", [sp!("")]; TOO_LONG);
    check!("\t", [sp!("\t")];);
    check!("\t", [sp!(" ")]; INVALID);
    check!(" ", [sp!("\t")]; INVALID);
    check!("\t\r", [sp!("\t\r")];);
    check!("\t\r ", [sp!("\t\r ")];);
    check!("\t \r", [sp!("\t \r")];);
    check!(" \t\r", [sp!(" \t\r")];);
    check!(" \n\r  \n", [sp!(" \n\r  \n")];);
    check!(" \t\n", [sp!(" \t")]; TOO_LONG);
    check!(" \n\t", [sp!(" \t\n")]; INVALID);
    check!("\u{2002}", [sp!("\u{2002}")];);
    check!(
        "\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}",
        [sp!("\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}")];
    );
    check!(
        "\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}",
        [sp!("\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}"),
        sp!("\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}")];
    );
    check!("a", [sp!("")]; TOO_LONG);
    check!("a", [sp!(" ")]; INVALID);
    check!("a", [sp!("a")];);
    check!("abc", [sp!("")]; TOO_LONG);
    check!("abc", [sp!(" ")]; INVALID);
    check!(" abc", [sp!("")]; TOO_LONG);
    check!(" abc", [sp!(" ")]; TOO_LONG);
    check!("", [lit!("")];);
    check!("", [lit!("a")]; TOO_SHORT);
    check!(" ", [lit!("a")]; INVALID);
    check!("a", [lit!("a")];);
    check!(" ", [lit!(" ")];);
    check!("aa", [lit!("a")]; TOO_LONG);
    check!("🤠", [lit!("a")]; INVALID);
    check!("A", [lit!("a")]; INVALID);
    check!("a", [lit!("z")]; INVALID);
    check!("a", [lit!("🤠")]; TOO_SHORT);
    check!("a", [lit!("\u{0363}a")]; TOO_SHORT);
    check!("\u{0363}a", [lit!("a")]; INVALID);
    check!("\u{0363}a", [lit!("\u{0363}a")];);
    check!("a", [lit!("ab")]; TOO_SHORT);
    check!("xy", [lit!("xy")];);
    check!("xy", [lit!("x"), lit!("y")];);
    check!("1", [lit!("1")];);
    check!("1234", [lit!("1234")];);
    check!("+1234", [lit!("+1234")];);
    check!("PST", [lit!("PST")];);
    check!("🤠", [lit!("🤠")];);
    check!("🤠a", [lit!("🤠"), lit!("a")];);
    check!("🤠a🤠", [lit!("🤠"), lit!("a🤠")];);
    check!("a🤠b", [lit!("a"), lit!("🤠"), lit!("b")];);
    check!("xy", [lit!("xy")];);
    check!("xyz", [lit!("xyz")];);
    check!("xy", [lit!("x"), lit!("y")];);
    check!("xyz", [lit!("x"), lit!("yz")];);
    check!("xyz", [lit!("xy"), lit!("z")];);
    check!("xyz", [lit!("x"), lit!("y"), lit!("z")];);
    check!("x y", [lit!("x"), lit!("y")]; INVALID);
    check!("xy", [lit!("x"), sp!(""), lit!("y")];);
    check!("x y", [lit!("x"), sp!(""), lit!("y")]; INVALID);
    check!("x y", [lit!("x"), sp!(" "), lit!("y")];);
    check!("a\n", [lit!("a"), sp!("\n")];);
    check!("\tab\n", [sp!("\t"), lit!("ab"), sp!("\n")];);
    check!("ab\tcd\ne", [lit!("ab"), sp!("\t"), lit!("cd"), sp!("\n"), lit!("e")];);
    check!(
        "+1ab\tcd\r\n+,.", [lit!("+1ab"), sp!("\t"), lit!("cd"), sp!("\r\n"),
        lit!("+,.")];
    );
    check!("a\tb", [lit!("a\tb")];);
    check!("a\tb", [lit!("a"), sp!("\t"), lit!("b")];);
    check!("1987", [num!(Year)]; year : 1987);
    check!("1987 ", [num!(Year)]; TOO_LONG);
    check!("0x12", [num!(Year)]; TOO_LONG);
    check!("x123", [num!(Year)]; INVALID);
    check!("o123", [num!(Year)]; INVALID);
    check!("2015", [num!(Year)]; year : 2015);
    check!("0000", [num!(Year)]; year : 0);
    check!("9999", [num!(Year)]; year : 9999);
    check!(" \t987", [num!(Year)]; INVALID);
    check!(" \t987", [sp!(" \t"), num!(Year)]; year : 987);
    check!(" \t987🤠", [sp!(" \t"), num!(Year), lit!("🤠")]; year : 987);
    check!("987🤠", [num!(Year), lit!("🤠")]; year : 987);
    check!("5", [num!(Year)]; year : 5);
    check!("5\0", [num!(Year)]; TOO_LONG);
    check!("\x005", [num!(Year)]; INVALID);
    check!("", [num!(Year)]; TOO_SHORT);
    check!("12345", [num!(Year), lit!("5")]; year : 1234);
    check!("12345", [nums!(Year), lit!("5")]; year : 1234);
    check!("12345", [num0!(Year), lit!("5")]; year : 1234);
    check!("12341234", [num!(Year), num!(Year)]; year : 1234);
    check!("1234 1234", [num!(Year), num!(Year)]; INVALID);
    check!("1234 1234", [num!(Year), sp!(" "), num!(Year)]; year : 1234);
    check!("1234 1235", [num!(Year), num!(Year)]; INVALID);
    check!("1234 1234", [num!(Year), lit!("x"), num!(Year)]; INVALID);
    check!("1234x1234", [num!(Year), lit!("x"), num!(Year)]; year : 1234);
    check!("1234 x 1234", [num!(Year), lit!("x"), num!(Year)]; INVALID);
    check!("1234xx1234", [num!(Year), lit!("x"), num!(Year)]; INVALID);
    check!("1234xx1234", [num!(Year), lit!("xx"), num!(Year)]; year : 1234);
    check!(
        "1234 x 1234", [num!(Year), sp!(" "), lit!("x"), sp!(" "), num!(Year)]; year :
        1234
    );
    check!(
        "1234 x 1235", [num!(Year), sp!(" "), lit!("x"), sp!(" "), lit!("1235")]; year :
        1234
    );
    check!("-42", [num!(Year)]; year : - 42);
    check!("+42", [num!(Year)]; year : 42);
    check!("-0042", [num!(Year)]; year : - 42);
    check!("+0042", [num!(Year)]; year : 42);
    check!("-42195", [num!(Year)]; year : - 42195);
    check!("+42195", [num!(Year)]; year : 42195);
    check!(" -42195", [num!(Year)]; INVALID);
    check!(" +42195", [num!(Year)]; INVALID);
    check!("  -42195", [num!(Year)]; INVALID);
    check!("  +42195", [num!(Year)]; INVALID);
    check!("-42195 ", [num!(Year)]; TOO_LONG);
    check!("+42195 ", [num!(Year)]; TOO_LONG);
    check!("  -   42", [num!(Year)]; INVALID);
    check!("  +   42", [num!(Year)]; INVALID);
    check!("  -42195", [sp!("  "), num!(Year)]; year : - 42195);
    check!("  +42195", [sp!("  "), num!(Year)]; year : 42195);
    check!("  -   42", [sp!("  "), num!(Year)]; INVALID);
    check!("  +   42", [sp!("  "), num!(Year)]; INVALID);
    check!("-", [num!(Year)]; TOO_SHORT);
    check!("+", [num!(Year)]; TOO_SHORT);
    check!("345", [num!(Ordinal)]; ordinal : 345);
    check!("+345", [num!(Ordinal)]; INVALID);
    check!("-345", [num!(Ordinal)]; INVALID);
    check!(" 345", [num!(Ordinal)]; INVALID);
    check!("345 ", [num!(Ordinal)]; TOO_LONG);
    check!(" 345", [sp!(" "), num!(Ordinal)]; ordinal : 345);
    check!("345 ", [num!(Ordinal), sp!(" ")]; ordinal : 345);
    check!("345🤠 ", [num!(Ordinal), lit!("🤠"), sp!(" ")]; ordinal : 345);
    check!("345🤠", [num!(Ordinal)]; TOO_LONG);
    check!("\u{0363}345", [num!(Ordinal)]; INVALID);
    check!(" +345", [num!(Ordinal)]; INVALID);
    check!(" -345", [num!(Ordinal)]; INVALID);
    check!("\t345", [sp!("\t"), num!(Ordinal)]; ordinal : 345);
    check!(" +345", [sp!(" "), num!(Ordinal)]; INVALID);
    check!(" -345", [sp!(" "), num!(Ordinal)]; INVALID);
    check!("1234 5678", [num!(Year), num!(IsoYear)]; INVALID);
    check!(
        "1234 5678", [num!(Year), sp!(" "), num!(IsoYear)]; year : 1234, isoyear : 5678
    );
    check!(
        "12 34 56 78", [num!(YearDiv100), num!(YearMod100), num!(IsoYearDiv100),
        num!(IsoYearMod100)]; INVALID
    );
    check!(
        "12 34🤠56 78", [num!(YearDiv100), sp!(" "), num!(YearMod100), lit!("🤠"),
        num!(IsoYearDiv100), sp!(" "), num!(IsoYearMod100)]; year_div_100 : 12,
        year_mod_100 : 34, isoyear_div_100 : 56, isoyear_mod_100 : 78
    );
    check!(
        "1 2 3 4 5 6", [num!(Month), sp!(" "), num!(Day), sp!(" "), num!(WeekFromSun),
        sp!(" "), num!(WeekFromMon), sp!(" "), num!(IsoWeek), sp!(" "),
        num!(NumDaysFromSun)]; month : 1, day : 2, week_from_sun : 3, week_from_mon : 4,
        isoweek : 5, weekday : Weekday::Sat
    );
    check!(
        "7 89 01", [num!(WeekdayFromMon), sp!(" "), num!(Ordinal), sp!(" "),
        num!(Hour12)]; weekday : Weekday::Sun, ordinal : 89, hour_mod_12 : 1
    );
    check!(
        "23 45 6 78901234 567890123", [num!(Hour), sp!(" "), num!(Minute), sp!(" "),
        num!(Second), sp!(" "), num!(Nanosecond), sp!(" "), num!(Timestamp)]; hour_div_12
        : 1, hour_mod_12 : 11, minute : 45, second : 6, nanosecond : 78_901_234,
        timestamp : 567_890_123
    );
    check!("apr", [fix!(ShortMonthName)]; month : 4);
    check!("Apr", [fix!(ShortMonthName)]; month : 4);
    check!("APR", [fix!(ShortMonthName)]; month : 4);
    check!("ApR", [fix!(ShortMonthName)]; month : 4);
    check!("\u{0363}APR", [fix!(ShortMonthName)]; INVALID);
    check!("April", [fix!(ShortMonthName)]; TOO_LONG);
    check!("A", [fix!(ShortMonthName)]; TOO_SHORT);
    check!("Sol", [fix!(ShortMonthName)]; INVALID);
    check!("Apr", [fix!(LongMonthName)]; month : 4);
    check!("Apri", [fix!(LongMonthName)]; TOO_LONG);
    check!("April", [fix!(LongMonthName)]; month : 4);
    check!("Aprill", [fix!(LongMonthName)]; TOO_LONG);
    check!("Aprill", [fix!(LongMonthName), lit!("l")]; month : 4);
    check!("Aprl", [fix!(LongMonthName), lit!("l")]; month : 4);
    check!("April", [fix!(LongMonthName), lit!("il")]; TOO_SHORT);
    check!("thu", [fix!(ShortWeekdayName)]; weekday : Weekday::Thu);
    check!("Thu", [fix!(ShortWeekdayName)]; weekday : Weekday::Thu);
    check!("THU", [fix!(ShortWeekdayName)]; weekday : Weekday::Thu);
    check!("tHu", [fix!(ShortWeekdayName)]; weekday : Weekday::Thu);
    check!("Thursday", [fix!(ShortWeekdayName)]; TOO_LONG);
    check!("T", [fix!(ShortWeekdayName)]; TOO_SHORT);
    check!("The", [fix!(ShortWeekdayName)]; INVALID);
    check!("Nop", [fix!(ShortWeekdayName)]; INVALID);
    check!("Thu", [fix!(LongWeekdayName)]; weekday : Weekday::Thu);
    check!("Thur", [fix!(LongWeekdayName)]; TOO_LONG);
    check!("Thurs", [fix!(LongWeekdayName)]; TOO_LONG);
    check!("Thursday", [fix!(LongWeekdayName)]; weekday : Weekday::Thu);
    check!("Thursdays", [fix!(LongWeekdayName)]; TOO_LONG);
    check!("Thursdays", [fix!(LongWeekdayName), lit!("s")]; weekday : Weekday::Thu);
    check!("Thus", [fix!(LongWeekdayName), lit!("s")]; weekday : Weekday::Thu);
    check!("Thursday", [fix!(LongWeekdayName), lit!("rsday")]; TOO_SHORT);
    check!("am", [fix!(LowerAmPm)]; hour_div_12 : 0);
    check!("pm", [fix!(LowerAmPm)]; hour_div_12 : 1);
    check!("AM", [fix!(LowerAmPm)]; hour_div_12 : 0);
    check!("PM", [fix!(LowerAmPm)]; hour_div_12 : 1);
    check!("am", [fix!(UpperAmPm)]; hour_div_12 : 0);
    check!("pm", [fix!(UpperAmPm)]; hour_div_12 : 1);
    check!("AM", [fix!(UpperAmPm)]; hour_div_12 : 0);
    check!("PM", [fix!(UpperAmPm)]; hour_div_12 : 1);
    check!("Am", [fix!(LowerAmPm)]; hour_div_12 : 0);
    check!(" Am", [sp!(" "), fix!(LowerAmPm)]; hour_div_12 : 0);
    check!("Am🤠", [fix!(LowerAmPm), lit!("🤠")]; hour_div_12 : 0);
    check!("🤠Am", [lit!("🤠"), fix!(LowerAmPm)]; hour_div_12 : 0);
    check!("\u{0363}am", [fix!(LowerAmPm)]; INVALID);
    check!("\u{0360}am", [fix!(LowerAmPm)]; INVALID);
    check!(" Am", [fix!(LowerAmPm)]; INVALID);
    check!("Am ", [fix!(LowerAmPm)]; TOO_LONG);
    check!("a.m.", [fix!(LowerAmPm)]; INVALID);
    check!("A.M.", [fix!(LowerAmPm)]; INVALID);
    check!("ame", [fix!(LowerAmPm)]; TOO_LONG);
    check!("a", [fix!(LowerAmPm)]; TOO_SHORT);
    check!("p", [fix!(LowerAmPm)]; TOO_SHORT);
    check!("x", [fix!(LowerAmPm)]; TOO_SHORT);
    check!("xx", [fix!(LowerAmPm)]; INVALID);
    check!("", [fix!(LowerAmPm)]; TOO_SHORT);
    check!("", [fix!(Nanosecond)];);
    check!("4", [fix!(Nanosecond)]; TOO_LONG);
    check!("4", [fix!(Nanosecond), num!(Second)]; second : 4);
    check!(".0", [fix!(Nanosecond)]; nanosecond : 0);
    check!(".4", [fix!(Nanosecond)]; nanosecond : 400_000_000);
    check!(".42", [fix!(Nanosecond)]; nanosecond : 420_000_000);
    check!(".421", [fix!(Nanosecond)]; nanosecond : 421_000_000);
    check!(".42195", [fix!(Nanosecond)]; nanosecond : 421_950_000);
    check!(".421951", [fix!(Nanosecond)]; nanosecond : 421_951_000);
    check!(".4219512", [fix!(Nanosecond)]; nanosecond : 421_951_200);
    check!(".42195123", [fix!(Nanosecond)]; nanosecond : 421_951_230);
    check!(".421950803", [fix!(Nanosecond)]; nanosecond : 421_950_803);
    check!(".4219508035", [fix!(Nanosecond)]; nanosecond : 421_950_803);
    check!(".42195080354", [fix!(Nanosecond)]; nanosecond : 421_950_803);
    check!(".421950803547", [fix!(Nanosecond)]; nanosecond : 421_950_803);
    check!(".000000003", [fix!(Nanosecond)]; nanosecond : 3);
    check!(".0000000031", [fix!(Nanosecond)]; nanosecond : 3);
    check!(".0000000035", [fix!(Nanosecond)]; nanosecond : 3);
    check!(".000000003547", [fix!(Nanosecond)]; nanosecond : 3);
    check!(".0000000009", [fix!(Nanosecond)]; nanosecond : 0);
    check!(".000000000547", [fix!(Nanosecond)]; nanosecond : 0);
    check!(".0000000009999999999999999999999999", [fix!(Nanosecond)]; nanosecond : 0);
    check!(".4🤠", [fix!(Nanosecond), lit!("🤠")]; nanosecond : 400_000_000);
    check!(".", [fix!(Nanosecond)]; TOO_SHORT);
    check!(".4x", [fix!(Nanosecond)]; TOO_LONG);
    check!(".  4", [fix!(Nanosecond)]; INVALID);
    check!("  .4", [fix!(Nanosecond)]; TOO_LONG);
    check!("", [internal_fix!(Nanosecond3NoDot)]; TOO_SHORT);
    check!("0", [internal_fix!(Nanosecond3NoDot)]; TOO_SHORT);
    check!("4", [internal_fix!(Nanosecond3NoDot)]; TOO_SHORT);
    check!("42", [internal_fix!(Nanosecond3NoDot)]; TOO_SHORT);
    check!("421", [internal_fix!(Nanosecond3NoDot)]; nanosecond : 421_000_000);
    check!("4210", [internal_fix!(Nanosecond3NoDot)]; TOO_LONG);
    check!(
        "42143", [internal_fix!(Nanosecond3NoDot), num!(Second)]; nanosecond :
        421_000_000, second : 43
    );
    check!(
        "421🤠", [internal_fix!(Nanosecond3NoDot), lit!("🤠")]; nanosecond :
        421_000_000
    );
    check!(
        "🤠421", [lit!("🤠"), internal_fix!(Nanosecond3NoDot)]; nanosecond :
        421_000_000
    );
    check!("42195", [internal_fix!(Nanosecond3NoDot)]; TOO_LONG);
    check!("123456789", [internal_fix!(Nanosecond3NoDot)]; TOO_LONG);
    check!("4x", [internal_fix!(Nanosecond3NoDot)]; TOO_SHORT);
    check!("  4", [internal_fix!(Nanosecond3NoDot)]; INVALID);
    check!(".421", [internal_fix!(Nanosecond3NoDot)]; INVALID);
    check!("", [internal_fix!(Nanosecond6NoDot)]; TOO_SHORT);
    check!("0", [internal_fix!(Nanosecond6NoDot)]; TOO_SHORT);
    check!("1234", [internal_fix!(Nanosecond6NoDot)]; TOO_SHORT);
    check!("12345", [internal_fix!(Nanosecond6NoDot)]; TOO_SHORT);
    check!("421950", [internal_fix!(Nanosecond6NoDot)]; nanosecond : 421_950_000);
    check!("000003", [internal_fix!(Nanosecond6NoDot)]; nanosecond : 3000);
    check!("000000", [internal_fix!(Nanosecond6NoDot)]; nanosecond : 0);
    check!("1234567", [internal_fix!(Nanosecond6NoDot)]; TOO_LONG);
    check!("123456789", [internal_fix!(Nanosecond6NoDot)]; TOO_LONG);
    check!("4x", [internal_fix!(Nanosecond6NoDot)]; TOO_SHORT);
    check!("     4", [internal_fix!(Nanosecond6NoDot)]; INVALID);
    check!(".42100", [internal_fix!(Nanosecond6NoDot)]; INVALID);
    check!("", [internal_fix!(Nanosecond9NoDot)]; TOO_SHORT);
    check!("42195", [internal_fix!(Nanosecond9NoDot)]; TOO_SHORT);
    check!("12345678", [internal_fix!(Nanosecond9NoDot)]; TOO_SHORT);
    check!("421950803", [internal_fix!(Nanosecond9NoDot)]; nanosecond : 421_950_803);
    check!("000000003", [internal_fix!(Nanosecond9NoDot)]; nanosecond : 3);
    check!(
        "42195080354", [internal_fix!(Nanosecond9NoDot), num!(Second)]; nanosecond :
        421_950_803, second : 54
    );
    check!("1234567890", [internal_fix!(Nanosecond9NoDot)]; TOO_LONG);
    check!("000000000", [internal_fix!(Nanosecond9NoDot)]; nanosecond : 0);
    check!("00000000x", [internal_fix!(Nanosecond9NoDot)]; INVALID);
    check!("        4", [internal_fix!(Nanosecond9NoDot)]; INVALID);
    check!(".42100000", [internal_fix!(Nanosecond9NoDot)]; INVALID);
    check!("1", [fix!(TimezoneOffset)]; INVALID);
    check!("12", [fix!(TimezoneOffset)]; INVALID);
    check!("123", [fix!(TimezoneOffset)]; INVALID);
    check!("1234", [fix!(TimezoneOffset)]; INVALID);
    check!("12345", [fix!(TimezoneOffset)]; INVALID);
    check!("123456", [fix!(TimezoneOffset)]; INVALID);
    check!("1234567", [fix!(TimezoneOffset)]; INVALID);
    check!("+1", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+12", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+123", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+1234", [fix!(TimezoneOffset)]; offset : 45_240);
    check!("+12345", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+123456", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+1234567", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12345678", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12:", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+12:3", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+12:34", [fix!(TimezoneOffset)]; offset : 45_240);
    check!("-12:34", [fix!(TimezoneOffset)]; offset : - 45_240);
    check!("+12:34:", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12:34:5", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12:34:56", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12:34:56:", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12 34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12  34", [fix!(TimezoneOffset)]; INVALID);
    check!("12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("12:34:56", [fix!(TimezoneOffset)]; INVALID);
    check!("+12::34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12: :34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12:::34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12::::34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12::34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12:34:56", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12:3456", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+1234:56", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+1234:567", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+00:00", [fix!(TimezoneOffset)]; offset : 0);
    check!("-00:00", [fix!(TimezoneOffset)]; offset : 0);
    check!("+00:01", [fix!(TimezoneOffset)]; offset : 60);
    check!("-00:01", [fix!(TimezoneOffset)]; offset : - 60);
    check!("+00:30", [fix!(TimezoneOffset)]; offset : 1_800);
    check!("-00:30", [fix!(TimezoneOffset)]; offset : - 1_800);
    check!("+24:00", [fix!(TimezoneOffset)]; offset : 86_400);
    check!("-24:00", [fix!(TimezoneOffset)]; offset : - 86_400);
    check!("+99:59", [fix!(TimezoneOffset)]; offset : 359_940);
    check!("-99:59", [fix!(TimezoneOffset)]; offset : - 359_940);
    check!("+00:60", [fix!(TimezoneOffset)]; OUT_OF_RANGE);
    check!("+00:99", [fix!(TimezoneOffset)]; OUT_OF_RANGE);
    check!("#12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12:34 ", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12 34 ", [fix!(TimezoneOffset)]; INVALID);
    check!(" +12:34", [fix!(TimezoneOffset)]; offset : 45_240);
    check!(" -12:34", [fix!(TimezoneOffset)]; offset : - 45_240);
    check!("  +12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("  -12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("\t -12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12: 34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12 :34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12 : 34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12 :  34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12  : 34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12:  34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12  :34", [fix!(TimezoneOffset)]; INVALID);
    check!("-12  :  34", [fix!(TimezoneOffset)]; INVALID);
    check!("12:34 ", [fix!(TimezoneOffset)]; INVALID);
    check!(" 12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+12345", [fix!(TimezoneOffset), num!(Day)]; offset : 45_240, day : 5);
    check!("+12:345", [fix!(TimezoneOffset), num!(Day)]; offset : 45_240, day : 5);
    check!("+12:34:", [fix!(TimezoneOffset), lit!(":")]; offset : 45_240);
    check!("Z12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("X12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("Z+12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("X+12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("🤠+12:34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12:34🤠", [fix!(TimezoneOffset)]; TOO_LONG);
    check!("+12:🤠34", [fix!(TimezoneOffset)]; INVALID);
    check!("+12:34🤠", [fix!(TimezoneOffset), lit!("🤠")]; offset : 45_240);
    check!("🤠+12:34", [lit!("🤠"), fix!(TimezoneOffset)]; offset : 45_240);
    check!("Z", [fix!(TimezoneOffset)]; INVALID);
    check!("A", [fix!(TimezoneOffset)]; INVALID);
    check!("PST", [fix!(TimezoneOffset)]; INVALID);
    check!("#Z", [fix!(TimezoneOffset)]; INVALID);
    check!(":Z", [fix!(TimezoneOffset)]; INVALID);
    check!("+Z", [fix!(TimezoneOffset)]; TOO_SHORT);
    check!("+:Z", [fix!(TimezoneOffset)]; INVALID);
    check!("+Z:", [fix!(TimezoneOffset)]; INVALID);
    check!("z", [fix!(TimezoneOffset)]; INVALID);
    check!(" :Z", [fix!(TimezoneOffset)]; INVALID);
    check!(" Z", [fix!(TimezoneOffset)]; INVALID);
    check!(" z", [fix!(TimezoneOffset)]; INVALID);
    check!("1", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("123", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("1234", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12345", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("123456", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("1234567", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12345678", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+1", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+12", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+123", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+1234", [fix!(TimezoneOffsetColon)]; offset : 45_240);
    check!("-1234", [fix!(TimezoneOffsetColon)]; offset : - 45_240);
    check!("+12345", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+123456", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+1234567", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12345678", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("1:", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:3", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:34:", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:34:5", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:34:56", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+1:", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12:", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+12:3", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+12:34", [fix!(TimezoneOffsetColon)]; offset : 45_240);
    check!("-12:34", [fix!(TimezoneOffsetColon)]; offset : - 45_240);
    check!("+12:34:", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12:34:5", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12:34:56", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12:34:56:", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12:34:56:7", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12:34:56:78", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12:3456", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+1234:56", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!("+12 34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12: 34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12 :34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12 : 34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12  : 34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12 :  34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12  :  34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12::34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12: :34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12:::34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12::::34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12::34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("#1234", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("#12:34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12:34 ", [fix!(TimezoneOffsetColon)]; TOO_LONG);
    check!(" +12:34", [fix!(TimezoneOffsetColon)]; offset : 45_240);
    check!("\t+12:34", [fix!(TimezoneOffsetColon)]; offset : 45_240);
    check!("\t\t+12:34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("12:34 ", [fix!(TimezoneOffsetColon)]; INVALID);
    check!(" 12:34", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!(":", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+12345", [fix!(TimezoneOffsetColon), num!(Day)]; offset : 45_240, day : 5);
    check!("+12:345", [fix!(TimezoneOffsetColon), num!(Day)]; offset : 45_240, day : 5);
    check!("+12:34:", [fix!(TimezoneOffsetColon), lit!(":")]; offset : 45_240);
    check!("Z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("A", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("PST", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("#Z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!(":Z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+Z", [fix!(TimezoneOffsetColon)]; TOO_SHORT);
    check!("+:Z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("+Z:", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!(" :Z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!(" Z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!(" z", [fix!(TimezoneOffsetColon)]; INVALID);
    check!("1", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("123", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("1234", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12345", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("123456", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("1234567", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12345678", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+1", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+12", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+123", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+1234", [fix!(TimezoneOffsetZ)]; offset : 45_240);
    check!("-1234", [fix!(TimezoneOffsetZ)]; offset : - 45_240);
    check!("+12345", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+123456", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+1234567", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12345678", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("1:", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:3", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:34:", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:34:5", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:34:56", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+1:", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12:", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+12:3", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+12:34", [fix!(TimezoneOffsetZ)]; offset : 45_240);
    check!("-12:34", [fix!(TimezoneOffsetZ)]; offset : - 45_240);
    check!("+12:34:", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12:34:5", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12:34:56", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12:34:56:", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12:34:56:7", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12:34:56:78", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12::34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12:3456", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+1234:56", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12 34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12  34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12: 34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12 :34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12 : 34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12  : 34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12 :  34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12  :  34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("12:34 ", [fix!(TimezoneOffsetZ)]; INVALID);
    check!(" 12:34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+12:34 ", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("+12 34 ", [fix!(TimezoneOffsetZ)]; INVALID);
    check!(" +12:34", [fix!(TimezoneOffsetZ)]; offset : 45_240);
    check!("+12345", [fix!(TimezoneOffsetZ), num!(Day)]; offset : 45_240, day : 5);
    check!("+12:345", [fix!(TimezoneOffsetZ), num!(Day)]; offset : 45_240, day : 5);
    check!("+12:34:", [fix!(TimezoneOffsetZ), lit!(":")]; offset : 45_240);
    check!("Z12:34", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("X12:34", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("Z", [fix!(TimezoneOffsetZ)]; offset : 0);
    check!("z", [fix!(TimezoneOffsetZ)]; offset : 0);
    check!(" Z", [fix!(TimezoneOffsetZ)]; offset : 0);
    check!(" z", [fix!(TimezoneOffsetZ)]; offset : 0);
    check!("\u{0363}Z", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("Z ", [fix!(TimezoneOffsetZ)]; TOO_LONG);
    check!("A", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("PST", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("#Z", [fix!(TimezoneOffsetZ)]; INVALID);
    check!(":Z", [fix!(TimezoneOffsetZ)]; INVALID);
    check!(":z", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+Z", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("-Z", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+A", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+🙃", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("+Z:", [fix!(TimezoneOffsetZ)]; INVALID);
    check!(" :Z", [fix!(TimezoneOffsetZ)]; INVALID);
    check!(" +Z", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!(" -Z", [fix!(TimezoneOffsetZ)]; TOO_SHORT);
    check!("+:Z", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("Y", [fix!(TimezoneOffsetZ)]; INVALID);
    check!("Zulu", [fix!(TimezoneOffsetZ), lit!("ulu")]; offset : 0);
    check!("zulu", [fix!(TimezoneOffsetZ), lit!("ulu")]; offset : 0);
    check!("+1234ulu", [fix!(TimezoneOffsetZ), lit!("ulu")]; offset : 45_240);
    check!("+12:34ulu", [fix!(TimezoneOffsetZ), lit!("ulu")]; offset : 45_240);
    check!("1", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("123", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("1234", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12345", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("123456", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("1234567", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12345678", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+1", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("+12", [internal_fix!(TimezoneOffsetPermissive)]; offset : 43_200);
    check!("+123", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("+1234", [internal_fix!(TimezoneOffsetPermissive)]; offset : 45_240);
    check!("-1234", [internal_fix!(TimezoneOffsetPermissive)]; offset : - 45_240);
    check!("+12345", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+123456", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+1234567", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12345678", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("1:", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:3", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:34:", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:34:5", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:34:56", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+1:", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:", [internal_fix!(TimezoneOffsetPermissive)]; offset : 43_200);
    check!("+12:3", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("+12:34", [internal_fix!(TimezoneOffsetPermissive)]; offset : 45_240);
    check!("-12:34", [internal_fix!(TimezoneOffsetPermissive)]; offset : - 45_240);
    check!("+12:34:", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12:34:5", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12:34:56", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12:34:56:", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12:34:56:7", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12:34:56:78", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12 34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12  34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12 :34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12: 34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12 : 34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12  :34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:  34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12  :  34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12::34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12 ::34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12: :34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:: 34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12  ::34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:  :34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12::  34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:::34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12::::34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("12:34 ", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!(" 12:34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:34 ", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!(" +12:34", [internal_fix!(TimezoneOffsetPermissive)]; offset : 45_240);
    check!(
        "+12345", [internal_fix!(TimezoneOffsetPermissive), num!(Day)]; offset : 45_240,
        day : 5
    );
    check!(
        "+12:345", [internal_fix!(TimezoneOffsetPermissive), num!(Day)]; offset : 45_240,
        day : 5
    );
    check!(
        "+12:34:", [internal_fix!(TimezoneOffsetPermissive), lit!(":")]; offset : 45_240
    );
    check!("🤠+12:34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+12:34🤠", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("+12:🤠34", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!(
        "+12:34🤠", [internal_fix!(TimezoneOffsetPermissive), lit!("🤠")]; offset :
        45_240
    );
    check!(
        "🤠+12:34", [lit!("🤠"), internal_fix!(TimezoneOffsetPermissive)]; offset :
        45_240
    );
    check!("Z", [internal_fix!(TimezoneOffsetPermissive)]; offset : 0);
    check!("A", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("PST", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("z", [internal_fix!(TimezoneOffsetPermissive)]; offset : 0);
    check!(" Z", [internal_fix!(TimezoneOffsetPermissive)]; offset : 0);
    check!(" z", [internal_fix!(TimezoneOffsetPermissive)]; offset : 0);
    check!("Z ", [internal_fix!(TimezoneOffsetPermissive)]; TOO_LONG);
    check!("#Z", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!(":Z", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!(":z", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+Z", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("-Z", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("+A", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("+PST", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+🙃", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("+Z:", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!(" :Z", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!(" +Z", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!(" -Z", [internal_fix!(TimezoneOffsetPermissive)]; TOO_SHORT);
    check!("+:Z", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("Y", [internal_fix!(TimezoneOffsetPermissive)]; INVALID);
    check!("CEST", [fix!(TimezoneName)];);
    check!("cest", [fix!(TimezoneName)];);
    check!("XXXXXXXX", [fix!(TimezoneName)];);
    check!("!!!!", [fix!(TimezoneName)];);
    check!("CEST 5", [fix!(TimezoneName), lit!(" "), num!(Day)]; day : 5);
    check!("CEST ", [fix!(TimezoneName)]; TOO_LONG);
    check!(" CEST", [fix!(TimezoneName)]; TOO_LONG);
    check!("CE ST", [fix!(TimezoneName)]; TOO_LONG);
    check!(
        "2015-02-04T14:37:05+09:00", [num!(Year), lit!("-"), num!(Month), lit!("-"),
        num!(Day), lit!("T"), num!(Hour), lit!(":"), num!(Minute), lit!(":"),
        num!(Second), fix!(TimezoneOffset)]; year : 2015, month : 2, day : 4, hour_div_12
        : 1, hour_mod_12 : 2, minute : 37, second : 5, offset : 32400
    );
    check!(
        "20150204143705567", [num!(Year), num!(Month), num!(Day), num!(Hour),
        num!(Minute), num!(Second), internal_fix!(Nanosecond3NoDot)]; year : 2015, month
        : 2, day : 4, hour_div_12 : 1, hour_mod_12 : 2, minute : 37, second : 5,
        nanosecond : 567000000
    );
    check!(
        "20150204143705.567", [num!(Year), num!(Month), num!(Day), num!(Hour),
        num!(Minute), num!(Second), fix!(Nanosecond)]; year : 2015, month : 2, day : 4,
        hour_div_12 : 1, hour_mod_12 : 2, minute : 37, second : 5, nanosecond : 567000000
    );
    check!(
        "20150204143705.567891", [num!(Year), num!(Month), num!(Day), num!(Hour),
        num!(Minute), num!(Second), fix!(Nanosecond)]; year : 2015, month : 2, day : 4,
        hour_div_12 : 1, hour_mod_12 : 2, minute : 37, second : 5, nanosecond : 567891000
    );
    check!(
        "20150204143705.567891023", [num!(Year), num!(Month), num!(Day), num!(Hour),
        num!(Minute), num!(Second), fix!(Nanosecond)]; year : 2015, month : 2, day : 4,
        hour_div_12 : 1, hour_mod_12 : 2, minute : 37, second : 5, nanosecond : 567891023
    );
    check!(
        "Mon, 10 Jun 2013 09:32:37  GMT", [fix!(ShortWeekdayName), lit!(","), sp!(" "),
        num!(Day), sp!(" "), fix!(ShortMonthName), sp!(" "), num!(Year), sp!(" "),
        num!(Hour), lit!(":"), num!(Minute), lit!(":"), num!(Second), sp!("  "),
        lit!("GMT")]; year : 2013, month : 6, day : 10, weekday : Weekday::Mon,
        hour_div_12 : 0, hour_mod_12 : 9, minute : 32, second : 37
    );
    check!(
        "🤠Mon, 10 Jun🤠2013 09:32:37  GMT🤠", [lit!("🤠"),
        fix!(ShortWeekdayName), lit!(","), sp!(" "), num!(Day), sp!(" "),
        fix!(ShortMonthName), lit!("🤠"), num!(Year), sp!(" "), num!(Hour), lit!(":"),
        num!(Minute), lit!(":"), num!(Second), sp!("  "), lit!("GMT"), lit!("🤠")];
        year : 2013, month : 6, day : 10, weekday : Weekday::Mon, hour_div_12 : 0,
        hour_mod_12 : 9, minute : 32, second : 37
    );
    check!(
        "Sun Aug 02 13:39:15 CEST 2020", [fix!(ShortWeekdayName), sp!(" "),
        fix!(ShortMonthName), sp!(" "), num!(Day), sp!(" "), num!(Hour), lit!(":"),
        num!(Minute), lit!(":"), num!(Second), sp!(" "), fix!(TimezoneName), sp!(" "),
        num!(Year)]; year : 2020, month : 8, day : 2, weekday : Weekday::Sun, hour_div_12
        : 1, hour_mod_12 : 1, minute : 39, second : 15
    );
    check!(
        "20060102150405", [num!(Year), num!(Month), num!(Day), num!(Hour), num!(Minute),
        num!(Second)]; year : 2006, month : 1, day : 2, hour_div_12 : 1, hour_mod_12 : 3,
        minute : 4, second : 5
    );
    check!(
        "3:14PM", [num!(Hour12), lit!(":"), num!(Minute), fix!(LowerAmPm)]; hour_div_12 :
        1, hour_mod_12 : 3, minute : 14
    );
    check!(
        "12345678901234.56789", [num!(Timestamp), lit!("."), num!(Nanosecond)];
        nanosecond : 56_789, timestamp : 12_345_678_901_234
    );
    check!(
        "12345678901234.56789", [num!(Timestamp), fix!(Nanosecond)]; nanosecond :
        567_890_000, timestamp : 12_345_678_901_234
    );
    check!(
        "2000-01-02T03:04:05Z", [num!(Year), lit!("-"), num!(Month), lit!("-"),
        num!(Day), lit!("T"), num!(Hour), lit!(":"), num!(Minute), lit!(":"),
        num!(Second), internal_fix!(TimezoneOffsetPermissive)]; year : 2000, month : 1,
        day : 2, hour_div_12 : 0, hour_mod_12 : 3, minute : 4, second : 5, offset : 0
    );
    check!(
        "2000-01-02 03:04:05Z", [num!(Year), lit!("-"), num!(Month), lit!("-"),
        num!(Day), sp!(" "), num!(Hour), lit!(":"), num!(Minute), lit!(":"),
        num!(Second), internal_fix!(TimezoneOffsetPermissive)]; year : 2000, month : 1,
        day : 2, hour_div_12 : 0, hour_mod_12 : 3, minute : 4, second : 5, offset : 0
    );
}
#[cfg(test)]
#[test]
fn test_rfc2822() {
    use super::NOT_ENOUGH;
    use super::*;
    use crate::offset::FixedOffset;
    use crate::DateTime;
    let testdates = [
        ("Tue, 20 Jan 2015 17:35:20 -0800", Ok("Tue, 20 Jan 2015 17:35:20 -0800")),
        ("Fri,  2 Jan 2015 17:35:20 -0800", Ok("Fri, 02 Jan 2015 17:35:20 -0800")),
        ("Fri, 02 Jan 2015 17:35:20 -0800", Ok("Fri, 02 Jan 2015 17:35:20 -0800")),
        ("Tue, 20 Jan 2015 17:35:20 -0800 (UTC)", Ok("Tue, 20 Jan 2015 17:35:20 -0800")),
        (
            "Tue,  20 Jan 2015 17:35:20 -0800 (UTC)",
            Ok("Tue, 20 Jan 2015 17:35:20 -0800"),
        ),
        (
            "Tue, 20     Jan   2015\t17:35:20\t-0800\t\t(UTC)",
            Ok("Tue, 20 Jan 2015 17:35:20 -0800"),
        ),
        (
            r"Tue, 20 Jan 2015 17:35:20 -0800 ( (UTC ) (\( (a)\(( \t ) ) \\( \) ))",
            Ok("Tue, 20 Jan 2015 17:35:20 -0800"),
        ),
        (r"Tue, 20 Jan 2015 17:35:20 -0800 (UTC\)", Err(TOO_LONG)),
        (
            "Tue, 20 Jan 2015 17:35:20 -0800 (UTC)\t \r\n(Anothercomment)",
            Ok("Tue, 20 Jan 2015 17:35:20 -0800"),
        ),
        ("Tue, 20 Jan 2015 17:35:20 -0800 (UTC) ", Err(TOO_LONG)),
        ("20 Jan 2015 17:35:20 -0800", Ok("Tue, 20 Jan 2015 17:35:20 -0800")),
        ("20 JAN 2015 17:35:20 -0800", Ok("Tue, 20 Jan 2015 17:35:20 -0800")),
        ("Tue, 20 Jan 2015 17:35 -0800", Ok("Tue, 20 Jan 2015 17:35:00 -0800")),
        ("11 Sep 2001 09:45:00 EST", Ok("Tue, 11 Sep 2001 09:45:00 -0500")),
        ("30 Feb 2015 17:35:20 -0800", Err(OUT_OF_RANGE)),
        ("Tue, 20 Jan 2015", Err(TOO_SHORT)),
        ("Tue, 20 Avr 2015 17:35:20 -0800", Err(INVALID)),
        ("Tue, 20 Jan 2015 25:35:20 -0800", Err(OUT_OF_RANGE)),
        ("Tue, 20 Jan 2015 7:35:20 -0800", Err(INVALID)),
        ("Tue, 20 Jan 2015 17:65:20 -0800", Err(OUT_OF_RANGE)),
        ("Tue, 20 Jan 2015 17:35:90 -0800", Err(OUT_OF_RANGE)),
        ("Tue, 20 Jan 2015 17:35:20 -0890", Err(OUT_OF_RANGE)),
        ("6 Jun 1944 04:00:00Z", Err(INVALID)),
        ("Tue, 20 Jan 2015 17:35:20 HAS", Err(NOT_ENOUGH)),
        ("Tue, 20 Jan 2015 17:35:20 GMT", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 UT", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 ut", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 EDT", Ok("Tue, 20 Jan 2015 17:35:20 -0400")),
        ("Tue, 20 Jan 2015 17:35:20 EST", Ok("Tue, 20 Jan 2015 17:35:20 -0500")),
        ("Tue, 20 Jan 2015 17:35:20 CDT", Ok("Tue, 20 Jan 2015 17:35:20 -0500")),
        ("Tue, 20 Jan 2015 17:35:20 CST", Ok("Tue, 20 Jan 2015 17:35:20 -0600")),
        ("Tue, 20 Jan 2015 17:35:20 MDT", Ok("Tue, 20 Jan 2015 17:35:20 -0600")),
        ("Tue, 20 Jan 2015 17:35:20 MST", Ok("Tue, 20 Jan 2015 17:35:20 -0700")),
        ("Tue, 20 Jan 2015 17:35:20 PDT", Ok("Tue, 20 Jan 2015 17:35:20 -0700")),
        ("Tue, 20 Jan 2015 17:35:20 PST", Ok("Tue, 20 Jan 2015 17:35:20 -0800")),
        ("Tue, 20 Jan 2015 17:35:20 pst", Ok("Tue, 20 Jan 2015 17:35:20 -0800")),
        ("Tue, 20 Jan 2015 17:35:20 Z", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 A", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 a", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 K", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 k", Ok("Tue, 20 Jan 2015 17:35:20 +0000")),
        ("Tue, 20 Jan 2015 17:35:20 J", Err(NOT_ENOUGH)),
        ("Tue, 20 Jan 2015😈17:35:20 -0800", Err(INVALID)),
    ];
    fn rfc2822_to_datetime(date: &str) -> ParseResult<DateTime<FixedOffset>> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, date, [Item::Fixed(Fixed::RFC2822)].iter())?;
        parsed.to_datetime()
    }
    fn fmt_rfc2822_datetime(dt: DateTime<FixedOffset>) -> String {
        dt.format_with_items([Item::Fixed(Fixed::RFC2822)].iter()).to_string()
    }
    for &(date, checkdate) in testdates.iter() {
        let d = rfc2822_to_datetime(date);
        let dt = match d {
            Ok(dt) => Ok(fmt_rfc2822_datetime(dt)),
            Err(e) => Err(e),
        };
        if dt != checkdate.map(|s| s.to_string()) {
            panic!(
                "Date conversion failed for {}\nReceived: {:?}\nExpected: {:?}", date,
                dt, checkdate
            );
        }
    }
}
#[cfg(test)]
#[test]
fn parse_rfc850() {
    use crate::{TimeZone, Utc};
    static RFC850_FMT: &str = "%A, %d-%b-%y %T GMT";
    let dt_str = "Sunday, 06-Nov-94 08:49:37 GMT";
    let dt = Utc.with_ymd_and_hms(1994, 11, 6, 8, 49, 37).unwrap();
    assert_eq!(dt.format(RFC850_FMT).to_string(), dt_str);
    assert_eq!(
        Ok(dt), Utc.datetime_from_str("Sunday, 06-Nov-94 08:49:37 GMT", RFC850_FMT)
    );
    let testdates = [
        (
            Utc.with_ymd_and_hms(1994, 11, 7, 8, 49, 37).unwrap(),
            "Monday, 07-Nov-94 08:49:37 GMT",
        ),
        (
            Utc.with_ymd_and_hms(1994, 11, 8, 8, 49, 37).unwrap(),
            "Tuesday, 08-Nov-94 08:49:37 GMT",
        ),
        (
            Utc.with_ymd_and_hms(1994, 11, 9, 8, 49, 37).unwrap(),
            "Wednesday, 09-Nov-94 08:49:37 GMT",
        ),
        (
            Utc.with_ymd_and_hms(1994, 11, 10, 8, 49, 37).unwrap(),
            "Thursday, 10-Nov-94 08:49:37 GMT",
        ),
        (
            Utc.with_ymd_and_hms(1994, 11, 11, 8, 49, 37).unwrap(),
            "Friday, 11-Nov-94 08:49:37 GMT",
        ),
        (
            Utc.with_ymd_and_hms(1994, 11, 12, 8, 49, 37).unwrap(),
            "Saturday, 12-Nov-94 08:49:37 GMT",
        ),
    ];
    for val in &testdates {
        assert_eq!(Ok(val.0), Utc.datetime_from_str(val.1, RFC850_FMT));
    }
}
#[cfg(test)]
#[test]
fn test_rfc3339() {
    use super::*;
    use crate::offset::FixedOffset;
    use crate::DateTime;
    let testdates = [
        ("2015-01-20T17:35:20-08:00", Ok("2015-01-20T17:35:20-08:00")),
        ("1944-06-06T04:04:00Z", Ok("1944-06-06T04:04:00+00:00")),
        ("2001-09-11T09:45:00-08:00", Ok("2001-09-11T09:45:00-08:00")),
        ("2015-01-20T17:35:20.001-08:00", Ok("2015-01-20T17:35:20.001-08:00")),
        ("2015-01-20T17:35:20.000031-08:00", Ok("2015-01-20T17:35:20.000031-08:00")),
        (
            "2015-01-20T17:35:20.000000004-08:00",
            Ok("2015-01-20T17:35:20.000000004-08:00"),
        ),
        ("2015-01-20T17:35:20.000000000452-08:00", Ok("2015-01-20T17:35:20-08:00")),
        ("2015-01-20 17:35:20.001-08:00", Err(INVALID)),
        ("2015/01/20T17:35:20.001-08:00", Err(INVALID)),
        ("2015-01-20T17-35-20.001-08:00", Err(INVALID)),
        ("99999-01-20T17:35:20-08:00", Err(INVALID)),
        ("-2000-01-20T17:35:20-08:00", Err(INVALID)),
        ("2015-02-30T17:35:20-08:00", Err(OUT_OF_RANGE)),
        ("2015-01-20T25:35:20-08:00", Err(OUT_OF_RANGE)),
        ("2015-01-20T17:65:20-08:00", Err(OUT_OF_RANGE)),
        ("2015-01-20T17:35:90-08:00", Err(OUT_OF_RANGE)),
        ("2015-01-20T17:35:20-24:00", Err(OUT_OF_RANGE)),
        ("15-01-20T17:35:20-08:00", Err(INVALID)),
        ("15-01-20T17:35:20-08:00:00", Err(INVALID)),
        ("2015-01-20T17:35:20-0800", Err(INVALID)),
        ("2015-01-20T17:35:20.001-08 : 00", Err(INVALID)),
        ("2015-01-20T17:35:20-08:00:00", Err(TOO_LONG)),
        ("2015-01-20T17:35:20-08:", Err(TOO_SHORT)),
        ("2015-01-20T17:35:20-08", Err(TOO_SHORT)),
        ("2015-01-20T", Err(TOO_SHORT)),
        ("2015-01-20T00:00:1", Err(TOO_SHORT)),
        ("2015-01-20T00:00:1-08:00", Err(INVALID)),
    ];
    fn rfc3339_to_datetime(date: &str) -> ParseResult<DateTime<FixedOffset>> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, date, [Item::Fixed(Fixed::RFC3339)].iter())?;
        parsed.to_datetime()
    }
    fn fmt_rfc3339_datetime(dt: DateTime<FixedOffset>) -> String {
        dt.format_with_items([Item::Fixed(Fixed::RFC3339)].iter()).to_string()
    }
    for &(date, checkdate) in testdates.iter() {
        eprintln!("test_rfc3339: date {:?}, expect {:?}", date, checkdate);
        let d = rfc3339_to_datetime(date);
        let dt = match d {
            Ok(dt) => Ok(fmt_rfc3339_datetime(dt)),
            Err(e) => Err(e),
        };
        if dt != checkdate.map(|s| s.to_string()) {
            panic!(
                "Date conversion failed for {}\nReceived: {:?}\nExpected: {:?}", date,
                dt, checkdate
            );
        }
    }
}
#[cfg(test)]
#[test]
fn test_issue_1010() {
    let dt = crate::NaiveDateTime::parse_from_str(
        "\u{c}SUN\u{e}\u{3000}\0m@J\u{3000}\0\u{3000}\0m\u{c}!\u{c}\u{b}\u{c}\u{c}\u{c}\u{c}%A\u{c}\u{b}\0SU\u{c}\u{c}",
        "\u{c}\u{c}%A\u{c}\u{b}\0SUN\u{c}\u{c}\u{c}SUNN\u{c}\u{c}\u{c}SUN\u{c}\u{c}!\u{c}\u{b}\u{c}\u{c}\u{c}\u{c}%A\u{c}\u{b}%a",
    );
    assert_eq!(dt, Err(ParseError(ParseErrorKind::Invalid)));
}
#[cfg(test)]
mod tests_llm_16_285_llm_16_285 {
    use crate::format::parse::parse;
    use crate::format::strftime::StrftimeItems;
    use crate::format::parsed::Parsed;
    use crate::format::Item;
    use crate::format::Numeric::*;
    use crate::format::Fixed::*;
    use crate::format::Pad::Zero;
    use crate::NaiveTime;
    use crate::Weekday;
    #[test]
    fn test_parse_function() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = StrftimeItems::new(rug_fuzz_0);
        let result = parse(&mut parsed, rug_fuzz_1, items);
        debug_assert!(result.is_ok());
        debug_assert_eq!(parsed.year, Some(2023));
        debug_assert_eq!(parsed.month, Some(4));
        debug_assert_eq!(parsed.day, Some(11));
        let naive_time = parsed.to_naive_time().unwrap();
        debug_assert_eq!(naive_time, NaiveTime::from_hms(16, 20, 0));
        let items = StrftimeItems::new(rug_fuzz_2);
        let result = parse(&mut parsed, rug_fuzz_3, items);
        debug_assert!(result.is_ok());
        debug_assert_eq!(parsed.month, Some(4));
        debug_assert_eq!(parsed.day, Some(11));
        let items = StrftimeItems::new(rug_fuzz_4);
        let result = parse(&mut parsed, rug_fuzz_5, items);
        debug_assert!(result.is_ok());
        debug_assert_eq!(parsed.year, Some(2023));
        let custom_items = vec![
            Item::Numeric(Year, Zero), Item::Literal("-"), Item::Numeric(Month, Zero),
            Item::Literal("-"), Item::Numeric(Day, Zero)
        ];
        let result = parse(&mut parsed, rug_fuzz_6, custom_items.into_iter());
        debug_assert!(result.is_ok());
        debug_assert_eq!(parsed.year, Some(2023));
        debug_assert_eq!(parsed.month, Some(4));
        debug_assert_eq!(parsed.day, Some(11));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_286_llm_16_286 {
    use super::*;
    use crate::*;
    use crate::format::parse::Parsed;
    use crate::format::Item;
    use crate::format::Pad::Zero;
    use crate::Weekday;
    use crate::format::{Fixed, Numeric};
    #[test]
    fn test_parse_internal_literal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Literal(rug_fuzz_0)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_1, items.into_iter()).unwrap(),
            "T12:00:00Z"
        );
             }
}
}
}    }
    #[test]
    fn test_parse_internal_short_month_name() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Fixed(Fixed::ShortMonthName)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.month, Some(4));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_short_weekday_name() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Fixed(Fixed::ShortWeekdayName)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.weekday, Some(Weekday::Mon));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Numeric(Numeric::Year, Zero)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.year, Some(2023));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_month() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Numeric(Numeric::Month, Zero)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.month, Some(4));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Numeric(Numeric::Day, Zero)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.day, Some(1));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_hour() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Numeric(Numeric::Hour, Zero)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.hour_div_12, Some(1));
        debug_assert_eq!(parsed.hour_mod_12, Some(0));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_minute() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Numeric(Numeric::Minute, Zero)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.minute, Some(0));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Numeric(Numeric::Second, Zero)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.second, Some(0));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_numeric_nanosecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Fixed(Fixed::Nanosecond)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.nanosecond, Some(123456789));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_fixed_upper_ampm() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Fixed(Fixed::UpperAmPm)];
        debug_assert_eq!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).unwrap(), ""
        );
        debug_assert_eq!(parsed.hour_div_12, Some(1));
             }
}
}
}    }
    #[test]
    fn test_parse_internal_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let items = vec![Item::Fixed(Fixed::ShortMonthName)];
        debug_assert!(
            parse_internal(& mut parsed, rug_fuzz_0, items.into_iter()).is_err()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_287 {
    use super::*;
    use crate::*;
    use crate::format::parse::Parsed;
    #[test]
    fn test_parse_rfc2822_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let inputs = vec![
            rug_fuzz_0, "Tue, 15 Nov 1994 12:45:26 +0200",
            "Sun, 06 Nov 1994 08:49:37 GMT", "Thu, 10 Feb 2000 13:00:00 +0000"
        ];
        for &input in &inputs {
            debug_assert!(parse_rfc2822(& mut parsed, input).is_ok());
            parsed = Parsed::new();
        }
             }
}
}
}    }
    #[test]
    fn test_parse_rfc2822_invalid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let inputs = vec![
            rug_fuzz_0, "32 Nov 1997 09:55:06 -0600", "Thu, 10 Feb 2000 13:00:00 +2500",
            "Tue, 15 Nov 1994 12:45:26 +02000", "Sun, 06 Nov 1994 08 08:49:37 GMT",
            "Sun, 06 Nov 1994 08:61:37 GMT", "Sun, 06 Nov 1994 08:49:61 GMT"
        ];
        for &input in &inputs {
            debug_assert!(parse_rfc2822(& mut parsed, input).is_err());
            parsed = Parsed::new();
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_288 {
    use crate::format::parse::{parse_rfc3339, Parsed, ParseResult};
    use crate::format::scan;
    use crate::format::ParseError;
    #[test]
    fn test_parse_rfc3339_valid_datetime() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        debug_assert_eq!(parse_rfc3339(& mut parsed, input), Ok(("", ())));
        debug_assert_eq!(parsed.to_naive_date().unwrap().to_string(), "2023-03-18");
        debug_assert_eq!(parsed.to_naive_time().unwrap().to_string(), "12:34:56");
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_valid_datetime_with_fraction() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        debug_assert_eq!(parse_rfc3339(& mut parsed, input), Ok(("", ())));
        debug_assert_eq!(parsed.to_naive_date().unwrap().to_string(), "2023-03-18");
        debug_assert_eq!(parsed.to_naive_time().unwrap().to_string(), "12:34:56.789");
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_valid_datetime_with_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        debug_assert_eq!(parse_rfc3339(& mut parsed, input), Ok(("", ())));
        debug_assert_eq!(parsed.to_naive_date().unwrap().to_string(), "2023-03-18");
        debug_assert_eq!(parsed.to_naive_time().unwrap().to_string(), "12:34:56");
        debug_assert_eq!(parsed.offset, Some(5400));
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_invalid_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        match parse_rfc3339(&mut parsed, input) {
            Err(ParseError(_)) => {}
            _ => panic!("Should have failed to parse an invalid date"),
        }
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_invalid_time() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        match parse_rfc3339(&mut parsed, input) {
            Err(ParseError(_)) => {}
            _ => panic!("Should have failed to parse an invalid time"),
        }
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_invalid_format() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        match parse_rfc3339(&mut parsed, input) {
            Err(ParseError(_)) => {}
            _ => panic!("Should have failed to parse an unexpected format"),
        }
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_invalid_leading_trailing_chars() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let input = rug_fuzz_0;
        match parse_rfc3339(&mut parsed, input) {
            Err(ParseError(_)) => {}
            _ => panic!("Should have failed to parse with leading invalid characters"),
        }
        let input = rug_fuzz_1;
        match parse_rfc3339(&mut parsed, input) {
            Err(ParseError(_)) => {}
            _ => panic!("Should have failed to parse with trailing invalid characters"),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_69 {
    use crate::format::parse::{set_weekday_with_num_days_from_sunday, Parsed};
    use crate::format::ParseResult;
    use crate::Weekday;
    #[test]
    fn test_set_weekday_with_num_days_from_sunday() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Parsed::new();
        let p1: i64 = rug_fuzz_0;
        debug_assert_eq!(
            set_weekday_with_num_days_from_sunday(& mut p0, p1), ParseResult::Ok(())
        );
        debug_assert_eq!(p0.weekday, Some(Weekday::Wed));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_70 {
    use super::*;
    use crate::format::Parsed;
    use crate::Weekday;
    use crate::format::parse::{
        set_weekday_with_number_from_monday, ParseResult, OUT_OF_RANGE,
    };
    #[test]
    fn test_set_weekday_with_number_from_monday() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Parsed::new();
        let p1: i64 = rug_fuzz_0;
        debug_assert_eq!(set_weekday_with_number_from_monday(& mut p0, p1), Ok(()));
        debug_assert_eq!(p0.weekday, Some(Weekday::Mon));
        let mut p0 = Parsed::new();
        let p1: i64 = rug_fuzz_1;
        debug_assert_eq!(set_weekday_with_number_from_monday(& mut p0, p1), Ok(()));
        debug_assert_eq!(p0.weekday, Some(Weekday::Sun));
        let mut p0 = Parsed::new();
        let p1: i64 = rug_fuzz_2;
        debug_assert_eq!(
            set_weekday_with_number_from_monday(& mut p0, p1), Err(OUT_OF_RANGE)
        );
             }
}
}
}    }
}
