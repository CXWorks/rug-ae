//! Parsers recognizing bytes streams, streaming version
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, IResult, Needed, Parser};
use crate::lib::std::result::Result::*;
use crate::traits::{
    Compare, CompareResult, FindSubstring, FindToken, InputLength, ToUsize,
};
use crate::Input;
/// Recognizes a pattern.
///
/// The input data will be compared to the tag combinator's argument and will return the part of
/// the input that matches the argument.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::tag;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///   tag("Hello")(s)
/// }
///
/// assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
/// assert_eq!(parser("Something"), Err(Err::Error(Error::new("Something", ErrorKind::Tag))));
/// assert_eq!(parser("S"), Err(Err::Error(Error::new("S", ErrorKind::Tag))));
/// assert_eq!(parser("H"), Err(Err::Incomplete(Needed::new(4))));
/// ```
pub fn tag<T, I, Error: ParseError<I>>(tag: T) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input + Compare<T>,
    T: InputLength + Clone,
{
    move |i: I| {
        let tag_len = tag.input_len();
        let t = tag.clone();
        let res: IResult<_, _, Error> = match i.compare(t) {
            CompareResult::Ok => Ok(i.take_split(tag_len)),
            CompareResult::Incomplete => {
                Err(Err::Incomplete(Needed::new(tag_len - i.input_len())))
            }
            CompareResult::Error => {
                let e: ErrorKind = ErrorKind::Tag;
                Err(Err::Error(Error::from_error_kind(i, e)))
            }
        };
        res
    }
}
/// Recognizes a case insensitive pattern.
///
/// The input data will be compared to the tag combinator's argument and will return the part of
/// the input that matches the argument with no regard to case.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::tag_no_case;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///   tag_no_case("hello")(s)
/// }
///
/// assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
/// assert_eq!(parser("hello, World!"), Ok((", World!", "hello")));
/// assert_eq!(parser("HeLlO, World!"), Ok((", World!", "HeLlO")));
/// assert_eq!(parser("Something"), Err(Err::Error(Error::new("Something", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Incomplete(Needed::new(5))));
/// ```
pub fn tag_no_case<T, I, Error: ParseError<I>>(
    tag: T,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input + Compare<T>,
    T: InputLength + Clone,
{
    move |i: I| {
        let tag_len = tag.input_len();
        let t = tag.clone();
        let res: IResult<_, _, Error> = match (i).compare_no_case(t) {
            CompareResult::Ok => Ok(i.take_split(tag_len)),
            CompareResult::Incomplete => {
                Err(Err::Incomplete(Needed::new(tag_len - i.input_len())))
            }
            CompareResult::Error => {
                let e: ErrorKind = ErrorKind::Tag;
                Err(Err::Error(Error::from_error_kind(i, e)))
            }
        };
        res
    }
}
/// Parse till certain characters are met.
///
/// The parser will return the longest slice till one of the characters of the combinator's argument are met.
///
/// It doesn't consume the matched character.
///
/// It will return a `Err::Incomplete(Needed::new(1))` if the pattern wasn't met.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::is_not;
///
/// fn not_space(s: &str) -> IResult<&str, &str> {
///   is_not(" \t\r\n")(s)
/// }
///
/// assert_eq!(not_space("Hello, World!"), Ok((" World!", "Hello,")));
/// assert_eq!(not_space("Sometimes\t"), Ok(("\t", "Sometimes")));
/// assert_eq!(not_space("Nospace"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(not_space(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn is_not<T, I, Error: ParseError<I>>(arr: T) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    T: FindToken<<I as Input>::Item>,
{
    move |i: I| {
        let e: ErrorKind = ErrorKind::IsNot;
        i.split_at_position1(|c| arr.find_token(c), e)
    }
}
/// Returns the longest slice of the matches the pattern.
///
/// The parser will return the longest slice consisting of the characters in provided in the
/// combinator's argument.
///
/// # Streaming specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the pattern wasn't met
/// or if the pattern reaches the end of the input.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::is_a;
///
/// fn hex(s: &str) -> IResult<&str, &str> {
///   is_a("1234567890ABCDEF")(s)
/// }
///
/// assert_eq!(hex("123 and voila"), Ok((" and voila", "123")));
/// assert_eq!(hex("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
/// assert_eq!(hex("BADBABEsomething"), Ok(("something", "BADBABE")));
/// assert_eq!(hex("D15EA5E"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(hex(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn is_a<T, I, Error: ParseError<I>>(arr: T) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    T: FindToken<<I as Input>::Item>,
{
    move |i: I| {
        let e: ErrorKind = ErrorKind::IsA;
        i.split_at_position1(|c| !arr.find_token(c), e)
    }
}
/// Returns the longest input slice (if any) that matches the predicate.
///
/// The parser will return the longest slice that matches the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the pattern reaches the end of the input.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::take_while;
/// use nom::character::is_alphabetic;
///
/// fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   take_while(is_alphabetic)(s)
/// }
///
/// assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
/// assert_eq!(alpha(b"12345"), Ok((&b"12345"[..], &b""[..])));
/// assert_eq!(alpha(b"latin"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(alpha(b""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn take_while<F, I, Error: ParseError<I>>(
    cond: F,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    F: Fn(<I as Input>::Item) -> bool,
{
    move |i: I| i.split_at_position(|c| !cond(c))
}
/// Returns the longest (at least 1) input slice that matches the predicate.
///
/// The parser will return the longest slice that matches the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// It will return an `Err(Err::Error((_, ErrorKind::TakeWhile1)))` if the pattern wasn't met.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` or if the pattern reaches the end of the input.
///
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_while1;
/// use nom::character::is_alphabetic;
///
/// fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   take_while1(is_alphabetic)(s)
/// }
///
/// assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
/// assert_eq!(alpha(b"latin"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(alpha(b"12345"), Err(Err::Error(Error::new(&b"12345"[..], ErrorKind::TakeWhile1))));
/// ```
pub fn take_while1<F, I, Error: ParseError<I>>(
    cond: F,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    F: Fn(<I as Input>::Item) -> bool,
{
    move |i: I| {
        let e: ErrorKind = ErrorKind::TakeWhile1;
        i.split_at_position1(|c| !cond(c), e)
    }
}
/// Returns the longest (m <= len <= n) input slice  that matches the predicate.
///
/// The parser will return the longest slice that matches the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// It will return an `Err::Error((_, ErrorKind::TakeWhileMN))` if the pattern wasn't met.
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))`  if the pattern reaches the end of the input or is too short.
///
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_while_m_n;
/// use nom::character::is_alphabetic;
///
/// fn short_alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   take_while_m_n(3, 6, is_alphabetic)(s)
/// }
///
/// assert_eq!(short_alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
/// assert_eq!(short_alpha(b"lengthy"), Ok((&b"y"[..], &b"length"[..])));
/// assert_eq!(short_alpha(b"latin"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(short_alpha(b"ed"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(short_alpha(b"12345"), Err(Err::Error(Error::new(&b"12345"[..], ErrorKind::TakeWhileMN))));
/// ```
pub fn take_while_m_n<F, I, Error: ParseError<I>>(
    m: usize,
    n: usize,
    cond: F,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    F: Fn(<I as Input>::Item) -> bool,
{
    move |i: I| {
        let input = i;
        for (i, (index, item)) in input.iter_indices().enumerate() {
            if i == n {
                return Ok(input.take_split(index));
            }
            if !cond(item) {
                if i >= m {
                    return Ok(input.take_split(index));
                } else {
                    return Err(
                        Err::Error(Error::from_error_kind(input, ErrorKind::TakeWhileMN)),
                    );
                }
            }
        }
        let input_len = input.input_len();
        let needed = if m > input_len { m - input_len } else { 1 };
        Err(Err::Incomplete(Needed::new(needed)))
    }
}
/// Returns the longest input slice (if any) till a predicate is met.
///
/// The parser will return the longest slice till the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the match reaches the
/// end of input or if there was not match.
///
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::take_till;
///
/// fn till_colon(s: &str) -> IResult<&str, &str> {
///   take_till(|c| c == ':')(s)
/// }
///
/// assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
/// assert_eq!(till_colon(":empty matched"), Ok((":empty matched", ""))); //allowed
/// assert_eq!(till_colon("12345"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(till_colon(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[allow(clippy::redundant_closure)]
pub fn take_till<F, I, Error: ParseError<I>>(
    cond: F,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    F: Fn(<I as Input>::Item) -> bool,
{
    move |i: I| i.split_at_position(|c| cond(c))
}
/// Returns the longest (at least 1) input slice till a predicate is met.
///
/// The parser will return the longest slice till the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the match reaches the
/// end of input or if there was not match.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_till1;
///
/// fn till_colon(s: &str) -> IResult<&str, &str> {
///   take_till1(|c| c == ':')(s)
/// }
///
/// assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
/// assert_eq!(till_colon(":empty matched"), Err(Err::Error(Error::new(":empty matched", ErrorKind::TakeTill1))));
/// assert_eq!(till_colon("12345"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(till_colon(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[allow(clippy::redundant_closure)]
pub fn take_till1<F, I, Error: ParseError<I>>(
    cond: F,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input,
    F: Fn(<I as Input>::Item) -> bool,
{
    move |i: I| {
        let e: ErrorKind = ErrorKind::TakeTill1;
        i.split_at_position1(|c| cond(c), e)
    }
}
/// Returns an input slice containing the first N input elements (Input[..N]).
///
/// # Streaming Specific
/// *Streaming version* if the input has less than N elements, `take` will
/// return a `Err::Incomplete(Needed::new(M))` where M is the number of
/// additional bytes the parser would need to succeed.
/// It is well defined for `&[u8]` as the number of elements is the byte size,
/// but for types like `&str`, we cannot know how many bytes correspond for
/// the next few chars, so the result will be `Err::Incomplete(Needed::Unknown)`
///
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::take;
///
/// fn take6(s: &str) -> IResult<&str, &str> {
///   take(6usize)(s)
/// }
///
/// assert_eq!(take6("1234567"), Ok(("7", "123456")));
/// assert_eq!(take6("things"), Ok(("", "things")));
/// assert_eq!(take6("short"), Err(Err::Incomplete(Needed::Unknown)));
/// ```
pub fn take<C, I, Error: ParseError<I>>(count: C) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input + InputLength,
    C: ToUsize,
{
    let c = count.to_usize();
    move |i: I| match i.slice_index(c) {
        Err(i) => Err(Err::Incomplete(i)),
        Ok(index) => Ok(i.take_split(index)),
    }
}
/// Returns the input slice up to the first occurrence of the pattern.
///
/// It doesn't consume the pattern.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(N))` if the input doesn't
/// contain the pattern or if the input is smaller than the pattern.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::take_until;
///
/// fn until_eof(s: &str) -> IResult<&str, &str> {
///   take_until("eof")(s)
/// }
///
/// assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
/// assert_eq!(until_eof("hello, world"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("hello, worldeo"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
/// ```
pub fn take_until<T, I, Error: ParseError<I>>(
    tag: T,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input + FindSubstring<T>,
    T: Clone,
{
    move |i: I| {
        let t = tag.clone();
        let res: IResult<_, _, Error> = match i.find_substring(t) {
            None => Err(Err::Incomplete(Needed::Unknown)),
            Some(index) => Ok(i.take_split(index)),
        };
        res
    }
}
/// Returns the non empty input slice up to the first occurrence of the pattern.
///
/// It doesn't consume the pattern.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(N))` if the input doesn't
/// contain the pattern or if the input is smaller than the pattern.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_until1;
///
/// fn until_eof(s: &str) -> IResult<&str, &str> {
///   take_until1("eof")(s)
/// }
///
/// assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
/// assert_eq!(until_eof("hello, world"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("hello, worldeo"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
/// assert_eq!(until_eof("eof"),  Err(Err::Error(Error::new("eof", ErrorKind::TakeUntil))));
/// ```
pub fn take_until1<T, I, Error: ParseError<I>>(
    tag: T,
) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Input + FindSubstring<T>,
    T: Clone,
{
    move |i: I| {
        let t = tag.clone();
        let res: IResult<_, _, Error> = match i.find_substring(t) {
            None => Err(Err::Incomplete(Needed::Unknown)),
            Some(0) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil))),
            Some(index) => Ok(i.take_split(index)),
        };
        res
    }
}
/// Matches a byte string with escaped characters.
///
/// * The first argument matches the normal characters (it must not accept the control character)
/// * The second argument is the control character (like `\` in most languages)
/// * The third argument matches the escaped characters
/// # Example
/// ```
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// # use nom::character::complete::digit1;
/// use nom::bytes::streaming::escaped;
/// use nom::character::streaming::one_of;
///
/// fn esc(s: &str) -> IResult<&str, &str> {
///   escaped(digit1, '\\', one_of("\"n\\"))(s)
/// }
///
/// assert_eq!(esc("123;"), Ok((";", "123")));
/// assert_eq!(esc("12\\\"34;"), Ok((";", "12\\\"34")));
/// ```
///
pub fn escaped<I, Error, F, G>(
    mut normal: F,
    control_char: char,
    mut escapable: G,
) -> impl FnMut(I) -> IResult<I, I, Error>
where
    I: Input + Clone + crate::traits::Offset,
    <I as Input>::Item: crate::traits::AsChar,
    F: Parser<I, Error = Error>,
    G: Parser<I, Error = Error>,
    Error: ParseError<I>,
{
    use crate::traits::AsChar;
    move |input: I| {
        let mut i = input.clone();
        while i.input_len() > 0 {
            let current_len = i.input_len();
            match normal.parse(i.clone()) {
                Ok((i2, _)) => {
                    if i2.input_len() == 0 {
                        return Err(Err::Incomplete(Needed::Unknown));
                    } else if i2.input_len() == current_len {
                        let index = input.offset(&i2);
                        return Ok(input.take_split(index));
                    } else {
                        i = i2;
                    }
                }
                Err(Err::Error(_)) => {
                    if i.iter_elements().next().unwrap().as_char() == control_char {
                        let next = control_char.len_utf8();
                        if next >= i.input_len() {
                            return Err(Err::Incomplete(Needed::new(1)));
                        } else {
                            match escapable.parse(i.take_from(next)) {
                                Ok((i2, _)) => {
                                    if i2.input_len() == 0 {
                                        return Err(Err::Incomplete(Needed::Unknown));
                                    } else {
                                        i = i2;
                                    }
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    } else {
                        let index = input.offset(&i);
                        return Ok(input.take_split(index));
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(Err::Incomplete(Needed::Unknown))
    }
}
/// Matches a byte string with escaped characters.
///
/// * The first argument matches the normal characters (it must not match the control character)
/// * The second argument is the control character (like `\` in most languages)
/// * The third argument matches the escaped characters and transforms them
///
/// As an example, the chain `abc\tdef` could be `abc    def` (it also consumes the control character)
///
/// ```
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// # use std::str::from_utf8;
/// use nom::bytes::streaming::{escaped_transform, tag};
/// use nom::character::streaming::alpha1;
/// use nom::branch::alt;
/// use nom::combinator::value;
///
/// fn parser(input: &str) -> IResult<&str, String> {
///   escaped_transform(
///     alpha1,
///     '\\',
///     alt((
///       value("\\", tag("\\")),
///       value("\"", tag("\"")),
///       value("\n", tag("n")),
///     ))
///   )(input)
/// }
///
/// assert_eq!(parser("ab\\\"cd\""), Ok(("\"", String::from("ab\"cd"))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn escaped_transform<I, Error, F, G, O1, O2, ExtendItem, Output>(
    mut normal: F,
    control_char: char,
    mut transform: G,
) -> impl FnMut(I) -> IResult<I, Output, Error>
where
    I: Clone + crate::traits::Offset + Input,
    I: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
    O1: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
    O2: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
    <I as Input>::Item: crate::traits::AsChar,
    F: Parser<I, Output = O1, Error = Error>,
    G: Parser<I, Output = O2, Error = Error>,
    Error: ParseError<I>,
{
    use crate::traits::AsChar;
    move |input: I| {
        let mut index = 0;
        let mut res = input.new_builder();
        let i = input.clone();
        while index < i.input_len() {
            let current_len = i.input_len();
            let remainder = i.take_from(index);
            match normal.parse(remainder.clone()) {
                Ok((i2, o)) => {
                    o.extend_into(&mut res);
                    if i2.input_len() == 0 {
                        return Err(Err::Incomplete(Needed::Unknown));
                    } else if i2.input_len() == current_len {
                        return Ok((remainder, res));
                    } else {
                        index = input.offset(&i2);
                    }
                }
                Err(Err::Error(_)) => {
                    if remainder.iter_elements().next().unwrap().as_char()
                        == control_char
                    {
                        let next = index + control_char.len_utf8();
                        let input_len = input.input_len();
                        if next >= input_len {
                            return Err(Err::Incomplete(Needed::Unknown));
                        } else {
                            match transform.parse(i.take_from(next)) {
                                Ok((i2, o)) => {
                                    o.extend_into(&mut res);
                                    if i2.input_len() == 0 {
                                        return Err(Err::Incomplete(Needed::Unknown));
                                    } else {
                                        index = input.offset(&i2);
                                    }
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    } else {
                        return Ok((remainder, res));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Err(Err::Incomplete(Needed::Unknown))
    }
}
#[cfg(test)]
mod tests_llm_16_296_llm_16_296 {
    use crate::bytes::streaming::escaped;
    use crate::character::streaming::digit1;
    use crate::character::streaming::one_of;
    use crate::error::{Error, ErrorKind, ParseError};
    use crate::internal::{IResult, Needed, Err};
    #[test]
    fn escaped_test() {
        fn esc(s: &str) -> IResult<&str, &str, Error<&str>> {
            escaped(digit1, '\\', one_of("\"n\\"))(s)
        }
        assert_eq!(esc("123;"), Ok((";", "123")));
        assert_eq!(esc("12\\\"34;"), Ok((";", "12\\\"34")));
        assert_eq!(
            esc("123\\"), Err(Err::Error(Error::from_error_kind("123\\",
            ErrorKind::Escaped)))
        );
        assert_eq!(esc("12\\n34;"), Ok((";", "12\\n34")));
        assert_eq!(
            esc("12\\n\\"), Err(Err::Error(Error::from_error_kind("12\\n\\",
            ErrorKind::Escaped)))
        );
        assert_eq!(
            esc("12\\x34"), Err(Err::Error(Error::from_error_kind("x34",
            ErrorKind::OneOf)))
        );
        assert_eq!(
            esc("12n34"), Err(Err::Error(Error::from_error_kind("n34",
            ErrorKind::Digit)))
        );
        assert_eq!(
            esc(""), Err(Err::Error(Error::from_error_kind("", ErrorKind::Escaped)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_297 {
    use super::*;
    use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::bytes::streaming::{escaped_transform, tag};
    use crate::character::streaming::alpha1;
    use crate::branch::alt;
    use crate::combinator::value;
    use crate::traits::{Input, ExtendInto};
    #[test]
    fn test_escaped_transform() {
        fn parser(input: &str) -> IResult<&str, String> {
            escaped_transform(
                alpha1,
                '\\',
                alt((
                    value("\\", tag("\\")),
                    value("\"", tag("\"")),
                    value("\n", tag("n")),
                )),
            )(input)
        }
        assert_eq!(parser("ab\\\"cd\""), Ok(("\"", String::from("ab\"cd"))));
        assert_eq!(parser("no\\nescaped"), Ok(("", String::from("no\nescaped"))));
        assert_eq!(parser("normal\\ttext"), Ok(("ttext", String::from("normal"))));
        assert_eq!(parser("\\\\slashes\\\\"), Ok(("", String::from("\\slashes\\"))));
        assert_eq!(parser("unfinished\\"), Err(Err::Incomplete(Needed::Unknown)));
        assert_eq!(
            parser("escape\\at_the_end\\"), Err(Err::Incomplete(Needed::Unknown))
        );
        assert_eq!(
            parser("ab\\1cd"), Err(Err::Error(Error::new("1cd", ErrorKind::Tag)))
        );
        assert_eq!(
            parser("invalid\\escape"), Err(Err::Error(Error::new("escape",
            ErrorKind::Tag)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_298 {
    use super::*;
    use crate::*;
    use crate::{
        Err, error::{Error, ErrorKind},
        IResult, Needed,
    };
    #[test]
    fn test_is_a_success() {
        fn test_parser(s: &str) -> IResult<&str, &str> {
            is_a("1234567890ABCDEF")(s)
        }
        assert_eq!(test_parser("123ABC"), Ok(("", "123ABC")));
        assert_eq!(test_parser("123 and voila"), Ok((" and voila", "123")));
        assert_eq!(test_parser("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
        assert_eq!(test_parser("BADBABEsomething"), Ok(("something", "BADBABE")));
    }
    #[test]
    fn test_is_a_incomplete() {
        fn test_parser(s: &str) -> IResult<&str, &str> {
            is_a("1234567890ABCDEF")(s)
        }
        assert_eq!(
            test_parser("GHIJKL"), Err(Err::Error(Error::new("GHIJKL", ErrorKind::IsA)))
        );
        assert_eq!(test_parser("D15EA5E"), Err(Err::Incomplete(Needed::new(1))));
        assert_eq!(test_parser(""), Err(Err::Incomplete(Needed::new(1))));
    }
}
#[cfg(test)]
mod tests_llm_16_299 {
    use super::*;
    use crate::*;
    use crate::{
        bytes::streaming::is_not, error::{Error, ErrorKind, ParseError},
        Err, IResult, Needed,
    };
    #[test]
    fn is_not_space() {
        fn not_space(s: &str) -> IResult<&str, &str, Error<&str>> {
            is_not(" \t\r\n")(s)
        }
        assert_eq!(not_space("Hello, World!"), Ok((" World!", "Hello,")));
        assert_eq!(not_space("Sometimes\t"), Ok(("\t", "Sometimes")));
        assert_eq!(not_space("Nospace"), Err(Err::Incomplete(Needed::new(1))));
        assert_eq!(not_space(""), Err(Err::Incomplete(Needed::new(1))));
    }
    #[test]
    fn is_not_empty_input() {
        let result: IResult<&str, &str, Error<&str>> = is_not("abc")("");
        assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
    }
    #[test]
    fn is_not_no_match() {
        let result: IResult<&str, &str, Error<&str>> = is_not("abc")("defghijkl");
        assert_eq!(result, Ok(("defghijkl", "")));
    }
    #[test]
    fn is_not_partial_match() {
        let result: IResult<&str, &str, Error<&str>> = is_not("abc")("defabcghi");
        assert_eq!(result, Ok(("abcghi", "def")));
    }
    #[test]
    fn is_not_error() {
        let result: IResult<&str, &str, Error<&str>> = is_not("abc")("abc");
        assert_eq!(result, Err(Err::Error(Error::new("abc", ErrorKind::IsNot))));
    }
}
#[cfg(test)]
mod tests_llm_16_300 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::bytes::streaming::tag;
    #[test]
    fn test_tag_success() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }
        assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
    }
    #[test]
    fn test_tag_incomplete() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }
        assert_eq!(parser("Hell"), Err(Err::Incomplete(crate ::Needed::new(1))));
    }
    #[test]
    fn test_tag_error() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }
        assert_eq!(
            parser("Goodbye, World!"), Err(Err::Error(Error::new("Goodbye, World!",
            ErrorKind::Tag)))
        );
    }
    #[test]
    fn test_tag_empty_input() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }
        assert_eq!(parser(""), Err(Err::Incomplete(crate ::Needed::new(5))));
    }
    #[test]
    fn test_tag_partial_match() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }
        assert_eq!(
            parser("Hellish"), Err(Err::Error(Error::new("Hellish", ErrorKind::Tag)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_301 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::bytes::streaming::tag_no_case;
    #[test]
    fn test_tag_no_case() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag_no_case("hello")(s)
        }
        assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
        assert_eq!(parser("hello, World!"), Ok((", World!", "hello")));
        assert_eq!(parser("HeLlO, World!"), Ok((", World!", "HeLlO")));
        assert_eq!(parser("hello, World!"), Ok((", World!", "hello")));
        assert_eq!(parser("HELLO, World!"), Ok((", World!", "HELLO")));
        assert_eq!(parser("hElLo, World!"), Ok((", World!", "hElLo")));
        assert_eq!(
            parser("Hi, World!"), Err(Err::Error(Error::new("Hi, World!",
            ErrorKind::Tag)))
        );
        assert_eq!(
            parser("Something"), Err(Err::Error(Error::new("Something", ErrorKind::Tag)))
        );
        assert_eq!(parser("he"), Err(Err::Incomplete(crate ::Needed::new(3))));
        assert_eq!(parser(""), Err(Err::Incomplete(crate ::Needed::new(5))));
    }
}
#[cfg(test)]
mod tests_llm_16_302 {
    use super::*;
    use crate::*;
    use crate::{Err, Needed, IResult};
    use crate::error::{ErrorKind, ParseError};
    #[test]
    fn take_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str> = take(rug_fuzz_0)(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("5", "1234")));
             }
}
}
}    }
    #[test]
    fn take_incomplete() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str, crate::error::Error<&str>> = take(
            rug_fuzz_0,
        )(rug_fuzz_1);
        debug_assert_eq!(result, Err(Err::Incomplete(Needed::Unknown)));
             }
}
}
}    }
    #[test]
    fn take_exact() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str> = take(rug_fuzz_0)(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("", "12345")));
             }
}
}
}    }
    #[test]
    fn take_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str> = take(rug_fuzz_0)(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("12345", "")));
             }
}
}
}    }
    #[test]
    fn take_full() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str> = take(rug_fuzz_0)(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("", "12345")));
             }
}
}
}    }
    #[test]
    fn take_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str, crate::error::Error<&str>> = take(
            rug_fuzz_0,
        )(rug_fuzz_1);
        debug_assert_eq!(result, Err(Err::Incomplete(Needed::Unknown)));
             }
}
}
}    }
    #[test]
    fn take_none() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str> = take(rug_fuzz_0)(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("", "")));
             }
}
}
}    }
    #[test]
    fn take_incomplete_empty_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result: IResult<&str, &str, crate::error::Error<&str>> = take(
            rug_fuzz_0,
        )(rug_fuzz_1);
        debug_assert_eq!(result, Err(Err::Incomplete(Needed::Unknown)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_303 {
    use crate::{
        bytes::streaming::take_till, error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    #[test]
    fn take_till_test() {
        fn till_colon(s: &str) -> IResult<&str, &str, Error<&str>> {
            take_till(|c| c == ':')(s)
        }
        assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
        assert_eq!(till_colon(":empty matched"), Ok((":empty matched", "")));
        assert_eq!(till_colon("12345"), Err(Err::Incomplete(Needed::new(1))));
        assert_eq!(till_colon(""), Err(Err::Incomplete(Needed::new(1))));
        assert_eq!(till_colon("latin words:12345"), Ok((":12345", "latin words")));
        assert_eq!(till_colon("::12345"), Ok((":12345", "")));
        assert_eq!(till_colon("no_colons"), Err(Err::Incomplete(Needed::new(1))));
    }
}
#[cfg(test)]
mod tests_llm_16_304 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::bytes::streaming::take_till1;
    #[test]
    fn test_take_till1_non_empty_match() {
        fn till_colon(s: &str) -> IResult<&str, &str, Error<&str>> {
            take_till1(|c| c == ':')(s)
        }
        assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
    }
    #[test]
    fn test_take_till1_empty_match() {
        fn till_colon(s: &str) -> IResult<&str, &str, Error<&str>> {
            take_till1(|c| c == ':')(s)
        }
        assert_eq!(
            till_colon(":empty matched"), Err(Err::Error(Error::new(":empty matched",
            ErrorKind::TakeTill1)))
        );
    }
    #[test]
    fn test_take_till1_incomplete() {
        fn till_colon(s: &str) -> IResult<&str, &str, Error<&str>> {
            take_till1(|c| c == ':')(s)
        }
        assert_eq!(till_colon("12345"), Err(Err::Incomplete(Needed::new(1))));
    }
    #[test]
    fn test_take_till1_empty_input() {
        fn till_colon(s: &str) -> IResult<&str, &str, Error<&str>> {
            take_till1(|c| c == ':')(s)
        }
        assert_eq!(till_colon(""), Err(Err::Incomplete(Needed::new(1))));
    }
}
#[cfg(test)]
mod tests_llm_16_305 {
    use crate::{
        bytes::streaming::take_until, error::{Error, ErrorKind, ParseError},
        Err, IResult, Needed,
    };
    #[test]
    fn take_until_test() {
        fn until_hello(input: &str) -> IResult<&str, &str, Error<&str>> {
            take_until("hello")(input)
        }
        assert_eq!(
            until_hello("say hello to the world"), Ok(("hello to the world", "say "))
        );
        assert_eq!(until_hello("no hello here"), Err(Err::Incomplete(Needed::Unknown)));
        assert_eq!(until_hello("hello"), Ok(("", "hello")));
        assert_eq!(until_hello("hell"), Err(Err::Incomplete(Needed::Unknown)));
        assert_eq!(until_hello("say hi"), Err(Err::Incomplete(Needed::Unknown)));
        assert_eq!(until_hello("he"), Err(Err::Incomplete(Needed::Unknown)));
        assert_eq!(until_hello("hellohello"), Ok(("hellohello", "")));
    }
    #[test]
    fn take_until_test_with_error() {
        fn until_world(input: &str) -> IResult<&str, &str, Error<&str>> {
            take_until("world")(input)
        }
        match until_world("say hello") {
            Err(Err::Incomplete(Needed::Unknown)) => {}
            _ => panic!("Error: Expected Err::Incomplete(Needed::Unknown)"),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_306 {
    use crate::{
        Err, IResult, Needed, error::{Error, ErrorKind},
        bytes::streaming::take_until1,
    };
    #[test]
    fn take_until1_non_empty_up_to_pattern() {
        fn until_eof(s: &str) -> IResult<&str, &str> {
            take_until1("eof")(s)
        }
        assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
    }
    #[test]
    fn take_until1_incomplete_when_no_pattern() {
        fn until_eof(s: &str) -> IResult<&str, &str> {
            take_until1("eof")(s)
        }
        assert_eq!(until_eof("hello, world"), Err(Err::Incomplete(Needed::Unknown)));
    }
    #[test]
    fn take_until1_incomplete_when_input_shorter_than_pattern() {
        fn until_eof(s: &str) -> IResult<&str, &str> {
            take_until1("eof")(s)
        }
        assert_eq!(until_eof("hello, worldeo"), Err(Err::Incomplete(Needed::Unknown)));
    }
    #[test]
    fn take_until1_until_first_occurrence_pattern() {
        fn until_eof(s: &str) -> IResult<&str, &str> {
            take_until1("eof")(s)
        }
        assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
    }
    #[test]
    fn take_until1_error_when_only_pattern() {
        fn until_eof(s: &str) -> IResult<&str, &str> {
            take_until1("eof")(s)
        }
        assert_eq!(
            until_eof("eof"), Err(Err::Error(Error::new("eof", ErrorKind::TakeUntil)))
        );
    }
}
