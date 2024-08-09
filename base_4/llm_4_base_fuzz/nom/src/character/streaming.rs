//! Character specific parsers and combinators, streaming version
//!
//! Functions recognizing specific characters

use crate::branch::alt;
use crate::combinator::opt;
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, IResult, Needed};
use crate::traits::{AsChar, FindToken, Input};
use crate::traits::{Compare, CompareResult};

/// Recognizes one character.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{ErrorKind, Error}, Needed, IResult};
/// # use nom::character::streaming::char;
/// fn parser(i: &str) -> IResult<&str, char> {
///     char('a')(i)
/// }
/// assert_eq!(parser("abc"), Ok(("bc", 'a')));
/// assert_eq!(parser("bc"), Err(Err::Error(Error::new("bc", ErrorKind::Char))));
/// assert_eq!(parser(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn char<I, Error: ParseError<I>>(c: char) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
{
  move |i: I| match (i).iter_elements().next().map(|t| {
    let b = t.as_char() == c;
    (&c, b)
  }) {
    None => Err(Err::Incomplete(Needed::new(c.len() - i.input_len()))),
    Some((_, false)) => Err(Err::Error(Error::from_char(i, c))),
    Some((c, true)) => Ok((i.take_from(c.len()), c.as_char())),
  }
}

/// Recognizes one character and checks that it satisfies a predicate
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{ErrorKind, Error}, Needed, IResult};
/// # use nom::character::streaming::satisfy;
/// fn parser(i: &str) -> IResult<&str, char> {
///     satisfy(|c| c == 'a' || c == 'b')(i)
/// }
/// assert_eq!(parser("abc"), Ok(("bc", 'a')));
/// assert_eq!(parser("cd"), Err(Err::Error(Error::new("cd", ErrorKind::Satisfy))));
/// assert_eq!(parser(""), Err(Err::Incomplete(Needed::Unknown)));
/// ```
pub fn satisfy<F, I, Error: ParseError<I>>(cond: F) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
  F: Fn(char) -> bool,
{
  move |i: I| match (i).iter_elements().next().map(|t| {
    let c = t.as_char();
    let b = cond(c);
    (c, b)
  }) {
    None => Err(Err::Incomplete(Needed::Unknown)),
    Some((_, false)) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Satisfy))),
    Some((c, true)) => Ok((i.take_from(c.len()), c)),
  }
}

/// Recognizes one of the provided characters.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::character::streaming::one_of;
/// assert_eq!(one_of::<_, _, (_, ErrorKind)>("abc")("b"), Ok(("", 'b')));
/// assert_eq!(one_of::<_, _, (_, ErrorKind)>("a")("bc"), Err(Err::Error(("bc", ErrorKind::OneOf))));
/// assert_eq!(one_of::<_, _, (_, ErrorKind)>("a")(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn one_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
  T: FindToken<<I as Input>::Item>,
{
  move |i: I| match (i).iter_elements().next().map(|c| (c, list.find_token(c))) {
    None => Err(Err::Incomplete(Needed::new(1))),
    Some((_, false)) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::OneOf))),
    Some((c, true)) => Ok((i.take_from(c.len()), c.as_char())),
  }
}

/// Recognizes a character that is not in the provided characters.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::character::streaming::none_of;
/// assert_eq!(none_of::<_, _, (_, ErrorKind)>("abc")("z"), Ok(("", 'z')));
/// assert_eq!(none_of::<_, _, (_, ErrorKind)>("ab")("a"), Err(Err::Error(("a", ErrorKind::NoneOf))));
/// assert_eq!(none_of::<_, _, (_, ErrorKind)>("a")(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn none_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
  T: FindToken<<I as Input>::Item>,
{
  move |i: I| match (i).iter_elements().next().map(|c| (c, !list.find_token(c))) {
    None => Err(Err::Incomplete(Needed::new(1))),
    Some((_, false)) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::NoneOf))),
    Some((c, true)) => Ok((i.take_from(c.len()), c.as_char())),
  }
}

/// Recognizes the string "\r\n".
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::crlf;
/// assert_eq!(crlf::<_, (_, ErrorKind)>("\r\nc"), Ok(("c", "\r\n")));
/// assert_eq!(crlf::<_, (_, ErrorKind)>("ab\r\nc"), Err(Err::Error(("ab\r\nc", ErrorKind::CrLf))));
/// assert_eq!(crlf::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(2))));
/// ```
pub fn crlf<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  T: Compare<&'static str>,
{
  match input.compare("\r\n") {
    //FIXME: is this the right index?
    CompareResult::Ok => Ok(input.take_split(2)),
    CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(2))),
    CompareResult::Error => {
      let e: ErrorKind = ErrorKind::CrLf;
      Err(Err::Error(E::from_error_kind(input, e)))
    }
  }
}

/// Recognizes a string of any char except '\r\n' or '\n'.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::streaming::not_line_ending;
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("ab\r\nc"), Ok(("\r\nc", "ab")));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("abc"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("a\rb\nc"), Err(Err::Error(("a\rb\nc", ErrorKind::Tag ))));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("a\rbc"), Err(Err::Error(("a\rbc", ErrorKind::Tag ))));
/// ```
pub fn not_line_ending<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  T: Compare<&'static str>,
  <T as Input>::Item: AsChar,
{
  match input.position(|item| {
    let c = item.as_char();
    c == '\r' || c == '\n'
  }) {
    None => Err(Err::Incomplete(Needed::Unknown)),
    Some(index) => {
      let mut it = input.take_from(index).iter_elements();
      let nth = it.next().unwrap().as_char();
      if nth == '\r' {
        let sliced = input.take_from(index);
        let comp = sliced.compare("\r\n");
        match comp {
          //FIXME: calculate the right index
          CompareResult::Incomplete => Err(Err::Incomplete(Needed::Unknown)),
          CompareResult::Error => {
            let e: ErrorKind = ErrorKind::Tag;
            Err(Err::Error(E::from_error_kind(input, e)))
          }
          CompareResult::Ok => Ok(input.take_split(index)),
        }
      } else {
        Ok(input.take_split(index))
      }
    }
  }
}

/// Recognizes an end of line (both '\n' and '\r\n').
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::line_ending;
/// assert_eq!(line_ending::<_, (_, ErrorKind)>("\r\nc"), Ok(("c", "\r\n")));
/// assert_eq!(line_ending::<_, (_, ErrorKind)>("ab\r\nc"), Err(Err::Error(("ab\r\nc", ErrorKind::CrLf))));
/// assert_eq!(line_ending::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn line_ending<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  T: Compare<&'static str>,
{
  match input.compare("\n") {
    CompareResult::Ok => Ok(input.take_split(1)),
    CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(1))),
    CompareResult::Error => {
      match input.compare("\r\n") {
        //FIXME: is this the right index?
        CompareResult::Ok => Ok(input.take_split(2)),
        CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(2))),
        CompareResult::Error => Err(Err::Error(E::from_error_kind(input, ErrorKind::CrLf))),
      }
    }
  }
}

/// Matches a newline character '\\n'.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::newline;
/// assert_eq!(newline::<_, (_, ErrorKind)>("\nc"), Ok(("c", '\n')));
/// assert_eq!(newline::<_, (_, ErrorKind)>("\r\nc"), Err(Err::Error(("\r\nc", ErrorKind::Char))));
/// assert_eq!(newline::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn newline<I, Error: ParseError<I>>(input: I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
{
  char('\n')(input)
}

/// Matches a tab character '\t'.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::tab;
/// assert_eq!(tab::<_, (_, ErrorKind)>("\tc"), Ok(("c", '\t')));
/// assert_eq!(tab::<_, (_, ErrorKind)>("\r\nc"), Err(Err::Error(("\r\nc", ErrorKind::Char))));
/// assert_eq!(tab::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn tab<I, Error: ParseError<I>>(input: I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
{
  char('\t')(input)
}

/// Matches one byte as a character. Note that the input type will
/// accept a `str`, but not a `&[u8]`, unlike many other nom parsers.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{character::streaming::anychar, Err, error::ErrorKind, IResult, Needed};
/// assert_eq!(anychar::<_, (_, ErrorKind)>("abc"), Ok(("bc",'a')));
/// assert_eq!(anychar::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn anychar<T, E: ParseError<T>>(input: T) -> IResult<T, char, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  let mut it = input.iter_elements();
  match it.next() {
    None => Err(Err::Incomplete(Needed::new(1))),
    Some(c) => Ok((input.take_from(c.len()), c.as_char())),
  }
}

/// Recognizes zero or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphabetic character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::alpha0;
/// assert_eq!(alpha0::<_, (_, ErrorKind)>("ab1c"), Ok(("1c", "ab")));
/// assert_eq!(alpha0::<_, (_, ErrorKind)>("1c"), Ok(("1c", "")));
/// assert_eq!(alpha0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn alpha0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| !item.is_alpha())
}

/// Recognizes one or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphabetic character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::alpha1;
/// assert_eq!(alpha1::<_, (_, ErrorKind)>("aB1c"), Ok(("1c", "aB")));
/// assert_eq!(alpha1::<_, (_, ErrorKind)>("1c"), Err(Err::Error(("1c", ErrorKind::Alpha))));
/// assert_eq!(alpha1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn alpha1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(|item| !item.is_alpha(), ErrorKind::Alpha)
}

/// Recognizes zero or more ASCII numerical characters: 0-9
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::digit0;
/// assert_eq!(digit0::<_, (_, ErrorKind)>("21c"), Ok(("c", "21")));
/// assert_eq!(digit0::<_, (_, ErrorKind)>("a21c"), Ok(("a21c", "")));
/// assert_eq!(digit0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn digit0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| !item.is_dec_digit())
}

/// Recognizes one or more ASCII numerical characters: 0-9
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::digit1;
/// assert_eq!(digit1::<_, (_, ErrorKind)>("21c"), Ok(("c", "21")));
/// assert_eq!(digit1::<_, (_, ErrorKind)>("c1"), Err(Err::Error(("c1", ErrorKind::Digit))));
/// assert_eq!(digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn digit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(|item| !item.is_dec_digit(), ErrorKind::Digit)
}

/// Recognizes zero or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non hexadecimal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::hex_digit0;
/// assert_eq!(hex_digit0::<_, (_, ErrorKind)>("21cZ"), Ok(("Z", "21c")));
/// assert_eq!(hex_digit0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(hex_digit0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn hex_digit0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| !item.is_hex_digit())
}

/// Recognizes one or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non hexadecimal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::hex_digit1;
/// assert_eq!(hex_digit1::<_, (_, ErrorKind)>("21cZ"), Ok(("Z", "21c")));
/// assert_eq!(hex_digit1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::HexDigit))));
/// assert_eq!(hex_digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn hex_digit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(|item| !item.is_hex_digit(), ErrorKind::HexDigit)
}

/// Recognizes zero or more octal characters: 0-7
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non octal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::oct_digit0;
/// assert_eq!(oct_digit0::<_, (_, ErrorKind)>("21cZ"), Ok(("cZ", "21")));
/// assert_eq!(oct_digit0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(oct_digit0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn oct_digit0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| !item.is_oct_digit())
}

/// Recognizes one or more octal characters: 0-7
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non octal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::oct_digit1;
/// assert_eq!(oct_digit1::<_, (_, ErrorKind)>("21cZ"), Ok(("cZ", "21")));
/// assert_eq!(oct_digit1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::OctDigit))));
/// assert_eq!(oct_digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn oct_digit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(|item| !item.is_oct_digit(), ErrorKind::OctDigit)
}

/// Recognizes zero or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphanumerical character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::alphanumeric0;
/// assert_eq!(alphanumeric0::<_, (_, ErrorKind)>("21cZ%1"), Ok(("%1", "21cZ")));
/// assert_eq!(alphanumeric0::<_, (_, ErrorKind)>("&Z21c"), Ok(("&Z21c", "")));
/// assert_eq!(alphanumeric0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn alphanumeric0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| !item.is_alphanum())
}

/// Recognizes one or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphanumerical character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::alphanumeric1;
/// assert_eq!(alphanumeric1::<_, (_, ErrorKind)>("21cZ%1"), Ok(("%1", "21cZ")));
/// assert_eq!(alphanumeric1::<_, (_, ErrorKind)>("&H2"), Err(Err::Error(("&H2", ErrorKind::AlphaNumeric))));
/// assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn alphanumeric1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(|item| !item.is_alphanum(), ErrorKind::AlphaNumeric)
}

/// Recognizes zero or more spaces and tabs.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::space0;
/// assert_eq!(space0::<_, (_, ErrorKind)>(" \t21c"), Ok(("21c", " \t")));
/// assert_eq!(space0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(space0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn space0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| {
    let c = item.as_char();
    !(c == ' ' || c == '\t')
  })
}
/// Recognizes one or more spaces and tabs.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::space1;
/// assert_eq!(space1::<_, (_, ErrorKind)>(" \t21c"), Ok(("21c", " \t")));
/// assert_eq!(space1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::Space))));
/// assert_eq!(space1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn space1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(
    |item| {
      let c = item.as_char();
      !(c == ' ' || c == '\t')
    },
    ErrorKind::Space,
  )
}

/// Recognizes zero or more spaces, tabs, carriage returns and line feeds.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::multispace0;
/// assert_eq!(multispace0::<_, (_, ErrorKind)>(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
/// assert_eq!(multispace0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(multispace0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn multispace0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position(|item| {
    let c = item.as_char();
    !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
  })
}

/// Recognizes one or more spaces, tabs, carriage returns and line feeds.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::streaming::multispace1;
/// assert_eq!(multispace1::<_, (_, ErrorKind)>(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
/// assert_eq!(multispace1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::MultiSpace))));
/// assert_eq!(multispace1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
pub fn multispace1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1(
    |item| {
      let c = item.as_char();
      !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
    },
    ErrorKind::MultiSpace,
  )
}

pub(crate) fn sign<T, E: ParseError<T>>(input: T) -> IResult<T, bool, E>
where
  T: Clone + Input,
  T: for<'a> Compare<&'a [u8]>,
{
  use crate::bytes::streaming::tag;
  use crate::combinator::value;

  let (i, opt_sign) = opt(alt((
    value(false, tag(&b"-"[..])),
    value(true, tag(&b"+"[..])),
  )))(input)?;
  let sign = opt_sign.unwrap_or(true);

  Ok((i, sign))
}

#[doc(hidden)]
macro_rules! ints {
    ($($t:tt)+) => {
        $(
        /// will parse a number in text form to a number
        ///
        /// *Complete version*: can parse until the end of input.
        pub fn $t<T, E: ParseError<T>>(input: T) -> IResult<T, $t, E>
            where
            T: Input +  Clone,
            <T as Input>::Item: AsChar,
            T: for <'a> Compare<&'a[u8]>,
            {
              let (i, sign) = sign(input.clone())?;

                if i.input_len() == 0 {
                    return Err(Err::Incomplete(Needed::new(1)));
                }

                let mut value: $t = 0;
                if sign {
                    let mut pos = 0;
                    for c in i.iter_elements() {
                        match c.as_char().to_digit(10) {
                            None => {
                                if pos == 0 {
                                    return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
                                } else {
                                    return Ok((i.take_from(pos), value));
                                }
                            },
                            Some(d) => match value.checked_mul(10).and_then(|v| v.checked_add(d as $t)) {
                                None => return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit))),
                                Some(v) => {
                                  pos += c.len();
                                  value = v;
                                },
                            }
                        }
                    }
                } else {
                    let mut pos = 0;
                    for c in i.iter_elements() {
                        match c.as_char().to_digit(10) {
                            None => {
                                if pos == 0 {
                                    return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
                                } else {
                                    return Ok((i.take_from(pos), value));
                                }
                            },
                            Some(d) => match value.checked_mul(10).and_then(|v| v.checked_sub(d as $t)) {
                                None => return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit))),
                                Some(v) => {
                                  pos += c.len();
                                  value = v;
                                },
                            }
                        }
                    }
                }

                Err(Err::Incomplete(Needed::new(1)))
            }
        )+
    }
}

ints! { i8 i16 i32 i64 i128 }

#[doc(hidden)]
macro_rules! uints {
    ($($t:tt)+) => {
        $(
        /// will parse a number in text form to a number
        ///
        /// *Complete version*: can parse until the end of input.
        pub fn $t<T, E: ParseError<T>>(input: T) -> IResult<T, $t, E>
            where
            T: Input ,
            <T as Input>::Item: AsChar,
            {
                let i = input;

                if i.input_len() == 0 {
                    return Err(Err::Incomplete(Needed::new(1)));
                }

                let mut value: $t = 0;
                let mut pos = 0;
                for c in i.iter_elements() {
                    match c.as_char().to_digit(10) {
                        None => {
                            if pos == 0 {
                                return Err(Err::Error(E::from_error_kind(i, ErrorKind::Digit)));
                            } else {
                                return Ok((i.take_from(pos), value));
                            }
                        },
                        Some(d) => match value.checked_mul(10).and_then(|v| v.checked_add(d as $t)) {
                            None => return Err(Err::Error(E::from_error_kind(i, ErrorKind::Digit))),
                            Some(v) => {
                              pos += c.len();
                              value = v;
                            },
                        }
                    }
                }

                Err(Err::Incomplete(Needed::new(1)))
            }
        )+
    }
}

uints! { u8 u16 u32 u64 u128 }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::error::ErrorKind;
  use crate::internal::{Err, Needed};
  use crate::sequence::pair;
  use crate::traits::ParseTo;
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, (_, ErrorKind)> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn anychar_str() {
    use super::anychar;
    assert_eq!(anychar::<_, (&str, ErrorKind)>("Ә"), Ok(("", 'Ә')));
  }

  #[test]
  fn character() {
    let a: &[u8] = b"abcd";
    let b: &[u8] = b"1234";
    let c: &[u8] = b"a123";
    let d: &[u8] = "azé12".as_bytes();
    let e: &[u8] = b" ";
    let f: &[u8] = b" ;";
    //assert_eq!(alpha1::<_, (_, ErrorKind)>(a), Err(Err::Incomplete(Needed::new(1))));
    assert_parse!(alpha1(a), Err(Err::Incomplete(Needed::new(1))));
    assert_eq!(alpha1(b), Err(Err::Error((b, ErrorKind::Alpha))));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(c), Ok((&c[1..], &b"a"[..])));
    assert_eq!(
      alpha1::<_, (_, ErrorKind)>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(digit1(a), Err(Err::Error((a, ErrorKind::Digit))));
    assert_eq!(
      digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(digit1(c), Err(Err::Error((c, ErrorKind::Digit))));
    assert_eq!(digit1(d), Err(Err::Error((d, ErrorKind::Digit))));
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(d),
      Ok(("zé12".as_bytes(), &b"a"[..]))
    );
    assert_eq!(hex_digit1(e), Err(Err::Error((e, ErrorKind::HexDigit))));
    assert_eq!(oct_digit1(a), Err(Err::Error((a, ErrorKind::OctDigit))));
    assert_eq!(
      oct_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(oct_digit1(c), Err(Err::Error((c, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1(d), Err(Err::Error((d, ErrorKind::OctDigit))));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    //assert_eq!(fix_error!(b,(), alphanumeric1), Ok((empty, b)));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(
      space1::<_, (_, ErrorKind)>(e),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(space1::<_, (_, ErrorKind)>(f), Ok((&b";"[..], &b" "[..])));
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn character_s() {
    let a = "abcd";
    let b = "1234";
    let c = "a123";
    let d = "azé12";
    let e = " ";
    assert_eq!(
      alpha1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(alpha1(b), Err(Err::Error((b, ErrorKind::Alpha))));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(c), Ok((&c[1..], "a")));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(d), Ok(("é12", "az")));
    assert_eq!(digit1(a), Err(Err::Error((a, ErrorKind::Digit))));
    assert_eq!(
      digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(digit1(c), Err(Err::Error((c, ErrorKind::Digit))));
    assert_eq!(digit1(d), Err(Err::Error((d, ErrorKind::Digit))));
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(d), Ok(("zé12", "a")));
    assert_eq!(hex_digit1(e), Err(Err::Error((e, ErrorKind::HexDigit))));
    assert_eq!(oct_digit1(a), Err(Err::Error((a, ErrorKind::OctDigit))));
    assert_eq!(
      oct_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(oct_digit1(c), Err(Err::Error((c, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1(d), Err(Err::Error((d, ErrorKind::OctDigit))));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    //assert_eq!(fix_error!(b,(), alphanumeric1), Ok((empty, b)));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(d), Ok(("é12", "az")));
    assert_eq!(
      space1::<_, (_, ErrorKind)>(e),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  use crate::traits::Offset;
  #[test]
  fn offset() {
    let a = &b"abcd;"[..];
    let b = &b"1234;"[..];
    let c = &b"a123;"[..];
    let d = &b" \t;"[..];
    let e = &b" \t\r\n;"[..];
    let f = &b"123abcDEF;"[..];

    match alpha1::<_, (_, ErrorKind)>(a) {
      Ok((i, _)) => {
        assert_eq!(a.offset(i) + i.len(), a.len());
      }
      _ => panic!("wrong return type in offset test for alpha"),
    }
    match digit1::<_, (_, ErrorKind)>(b) {
      Ok((i, _)) => {
        assert_eq!(b.offset(i) + i.len(), b.len());
      }
      _ => panic!("wrong return type in offset test for digit"),
    }
    match alphanumeric1::<_, (_, ErrorKind)>(c) {
      Ok((i, _)) => {
        assert_eq!(c.offset(i) + i.len(), c.len());
      }
      _ => panic!("wrong return type in offset test for alphanumeric"),
    }
    match space1::<_, (_, ErrorKind)>(d) {
      Ok((i, _)) => {
        assert_eq!(d.offset(i) + i.len(), d.len());
      }
      _ => panic!("wrong return type in offset test for space"),
    }
    match multispace1::<_, (_, ErrorKind)>(e) {
      Ok((i, _)) => {
        assert_eq!(e.offset(i) + i.len(), e.len());
      }
      _ => panic!("wrong return type in offset test for multispace"),
    }
    match hex_digit1::<_, (_, ErrorKind)>(f) {
      Ok((i, _)) => {
        assert_eq!(f.offset(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for hex_digit"),
    }
    match oct_digit1::<_, (_, ErrorKind)>(f) {
      Ok((i, _)) => {
        assert_eq!(f.offset(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for oct_digit"),
    }
  }

  #[test]
  fn is_not_line_ending_bytes() {
    let a: &[u8] = b"ab12cd\nefgh";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(a),
      Ok((&b"\nefgh"[..], &b"ab12cd"[..]))
    );

    let b: &[u8] = b"ab12cd\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(b),
      Ok((&b"\nefgh\nijkl"[..], &b"ab12cd"[..]))
    );

    let c: &[u8] = b"ab12cd\r\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(c),
      Ok((&b"\r\nefgh\nijkl"[..], &b"ab12cd"[..]))
    );

    let d: &[u8] = b"ab12cd";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(d),
      Err(Err::Incomplete(Needed::Unknown))
    );
  }

  #[test]
  fn is_not_line_ending_str() {
    /*
    let a: &str = "ab12cd\nefgh";
    assert_eq!(not_line_ending(a), Ok((&"\nefgh"[..], &"ab12cd"[..])));

    let b: &str = "ab12cd\nefgh\nijkl";
    assert_eq!(not_line_ending(b), Ok((&"\nefgh\nijkl"[..], &"ab12cd"[..])));

    let c: &str = "ab12cd\r\nefgh\nijkl";
    assert_eq!(not_line_ending(c), Ok((&"\r\nefgh\nijkl"[..], &"ab12cd"[..])));

    let d = "βèƒôřè\nÂßÇáƒƭèř";
    assert_eq!(not_line_ending(d), Ok((&"\nÂßÇáƒƭèř"[..], &"βèƒôřè"[..])));

    let e = "βèƒôřè\r\nÂßÇáƒƭèř";
    assert_eq!(not_line_ending(e), Ok((&"\r\nÂßÇáƒƭèř"[..], &"βèƒôřè"[..])));
    */

    let f = "βèƒôřè\rÂßÇáƒƭèř";
    assert_eq!(not_line_ending(f), Err(Err::Error((f, ErrorKind::Tag))));

    let g2: &str = "ab12cd";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(g2),
      Err(Err::Incomplete(Needed::Unknown))
    );
  }

  #[test]
  fn hex_digit_test() {
    let i = &b"0123456789abcdefABCDEF;"[..];
    assert_parse!(hex_digit1(i), Ok((&b";"[..], &i[..i.len() - 1])));

    let i = &b"g"[..];
    assert_parse!(
      hex_digit1(i),
      Err(Err::Error(error_position!(i, ErrorKind::HexDigit)))
    );

    let i = &b"G"[..];
    assert_parse!(
      hex_digit1(i),
      Err(Err::Error(error_position!(i, ErrorKind::HexDigit)))
    );

    assert!(crate::character::is_hex_digit(b'0'));
    assert!(crate::character::is_hex_digit(b'9'));
    assert!(crate::character::is_hex_digit(b'a'));
    assert!(crate::character::is_hex_digit(b'f'));
    assert!(crate::character::is_hex_digit(b'A'));
    assert!(crate::character::is_hex_digit(b'F'));
    assert!(!crate::character::is_hex_digit(b'g'));
    assert!(!crate::character::is_hex_digit(b'G'));
    assert!(!crate::character::is_hex_digit(b'/'));
    assert!(!crate::character::is_hex_digit(b':'));
    assert!(!crate::character::is_hex_digit(b'@'));
    assert!(!crate::character::is_hex_digit(b'\x60'));
  }

  #[test]
  fn oct_digit_test() {
    let i = &b"01234567;"[..];
    assert_parse!(oct_digit1(i), Ok((&b";"[..], &i[..i.len() - 1])));

    let i = &b"8"[..];
    assert_parse!(
      oct_digit1(i),
      Err(Err::Error(error_position!(i, ErrorKind::OctDigit)))
    );

    assert!(crate::character::is_oct_digit(b'0'));
    assert!(crate::character::is_oct_digit(b'7'));
    assert!(!crate::character::is_oct_digit(b'8'));
    assert!(!crate::character::is_oct_digit(b'9'));
    assert!(!crate::character::is_oct_digit(b'a'));
    assert!(!crate::character::is_oct_digit(b'A'));
    assert!(!crate::character::is_oct_digit(b'/'));
    assert!(!crate::character::is_oct_digit(b':'));
    assert!(!crate::character::is_oct_digit(b'@'));
    assert!(!crate::character::is_oct_digit(b'\x60'));
  }

  #[test]
  fn full_line_windows() {
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\r\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\r\n"[..]))));
  }

  #[test]
  fn full_line_unix() {
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\n"[..]))));
  }

  #[test]
  fn check_windows_lineending() {
    let input = b"\r\n";
    let output = line_ending(&input[..]);
    assert_parse!(output, Ok((&b""[..], &b"\r\n"[..])));
  }

  #[test]
  fn check_unix_lineending() {
    let input = b"\n";
    let output = line_ending(&input[..]);
    assert_parse!(output, Ok((&b""[..], &b"\n"[..])));
  }

  #[test]
  fn cr_lf() {
    assert_parse!(crlf(&b"\r\na"[..]), Ok((&b"a"[..], &b"\r\n"[..])));
    assert_parse!(crlf(&b"\r"[..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      crlf(&b"\ra"[..]),
      Err(Err::Error(error_position!(&b"\ra"[..], ErrorKind::CrLf)))
    );

    assert_parse!(crlf("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(crlf("\r"), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      crlf("\ra"),
      Err(Err::Error(error_position!("\ra", ErrorKind::CrLf)))
    );
  }

  #[test]
  fn end_of_line() {
    assert_parse!(line_ending(&b"\na"[..]), Ok((&b"a"[..], &b"\n"[..])));
    assert_parse!(line_ending(&b"\r\na"[..]), Ok((&b"a"[..], &b"\r\n"[..])));
    assert_parse!(
      line_ending(&b"\r"[..]),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      line_ending(&b"\ra"[..]),
      Err(Err::Error(error_position!(&b"\ra"[..], ErrorKind::CrLf)))
    );

    assert_parse!(line_ending("\na"), Ok(("a", "\n")));
    assert_parse!(line_ending("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(line_ending("\r"), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      line_ending("\ra"),
      Err(Err::Error(error_position!("\ra", ErrorKind::CrLf)))
    );
  }

  fn digit_to_i16(input: &str) -> IResult<&str, i16> {
    let i = input;
    let (i, opt_sign) = opt(alt((char('+'), char('-'))))(i)?;
    let sign = match opt_sign {
      Some('+') => true,
      Some('-') => false,
      _ => true,
    };

    let (i, s) = match digit1::<_, crate::error::Error<_>>(i) {
      Ok((i, s)) => (i, s),
      Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
      Err(_) => {
        return Err(Err::Error(crate::error::Error::from_error_kind(
          input,
          ErrorKind::Digit,
        )))
      }
    };
    match s.parse_to() {
      Some(n) => {
        if sign {
          Ok((i, n))
        } else {
          Ok((i, -n))
        }
      }
      None => Err(Err::Error(crate::error::Error::from_error_kind(
        i,
        ErrorKind::Digit,
      ))),
    }
  }

  fn digit_to_u32(i: &str) -> IResult<&str, u32> {
    let (i, s) = digit1(i)?;
    match s.parse_to() {
      Some(n) => Ok((i, n)),
      None => Err(Err::Error(crate::error::Error::from_error_kind(
        i,
        ErrorKind::Digit,
      ))),
    }
  }

  proptest! {
    #[test]
    fn ints(s in "\\PC*") {
        let res1 = digit_to_i16(&s);
        let res2 = i16(s.as_str());
        assert_eq!(res1, res2);
    }

    #[test]
    fn uints(s in "\\PC*") {
        let res1 = digit_to_u32(&s);
        let res2 = u32(s.as_str());
        assert_eq!(res1, res2);
    }
  }
}
#[cfg(test)]
mod tests_llm_16_352 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::character::streaming::alpha0;

    #[test]
    fn alpha0_empty_input() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn alpha0_all_alpha_input() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("abc"), Ok(("", "abc")));
    }

    #[test]
    fn alpha0_all_capital_alpha_input() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("ABC"), Ok(("", "ABC")));
    }

    #[test]
    fn alpha0_mixed_alpha_input() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("aBcDef"), Ok(("", "aBcDef")));
    }

    #[test]
    fn alpha0_non_alpha_prefix() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("1abc"), Ok(("1abc", "")));
    }

    #[test]
    fn alpha0_alpha_followed_by_non_alpha() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("abc1def"), Ok(("1def", "abc")));
    }

    #[test]
    fn alpha0_non_alpha_input() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("123"), Ok(("123", "")));
    }

    #[test]
    fn alpha0_mixed_input() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("abc123def"), Ok(("123def", "abc")));
    }

    #[test]
    fn alpha0_mixed_case_followed_by_symbols() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("aBcDeF$$"), Ok(("$$", "aBcDeF")));
    }

    #[test]
    fn alpha0_only_symbols() {
        assert_eq!(alpha0::<_, (_, ErrorKind)>("!@#$"), Ok(("!@#$", "")));
    }
}#[cfg(test)]
mod tests_llm_16_353_llm_16_353 {
  use crate::{Err, Needed, IResult};
  use crate::error::{Error, ErrorKind};
  use crate::character::streaming::alpha1;

  #[test]
  fn test_alpha1() {
    fn test_error(res: IResult<&str, &str, Error<&str>>) -> bool {
      matches!(res, Err(Err::Error(Error { .. })))
    }

    fn test_incomplete(res: IResult<&str, &str, Error<&str>>) -> bool {
      matches!(res, Err(Err::Incomplete(Needed::Size(_))))
    }

    // Test cases that should succeed
    assert_eq!(alpha1::<_, Error<&str>>("abcDEF"), Ok(("", "abcDEF")));
    assert_eq!(alpha1::<_, Error<&str>>("XyZ123"), Ok(("123", "XyZ")));
    assert_eq!(alpha1::<_, Error<&str>>("testAlpha1 "), Ok((" ", "testAlpha1")));

    // Test cases that should result in Error
    assert!(test_error(alpha1::<_, Error<&str>>("123")));
    assert!(test_error(alpha1::<_, Error<&str>>("!?@")));

    // Test cases that should result in Incomplete
    assert!(test_incomplete(alpha1::<_, Error<&str>>("")));
  }
}#[cfg(test)]
mod tests_llm_16_354 {
    use crate::{error::ErrorKind, Err, IResult, Needed};
    use crate::character::streaming::alphanumeric0;

    #[test]
    fn test_alphanumeric0() {
        fn test_fn(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
            alphanumeric0(input)
        }

        // Test with alphanumeric input.
        assert_eq!(test_fn("21cZ%1"), Ok(("%1", "21cZ")));

        // Test with input that doesn't start with alphanumeric characters.
        assert_eq!(test_fn("&Z21c"), Ok(("&Z21c", "")));

        // Test with an empty input.
        assert_eq!(test_fn(""), Err(Err::Incomplete(Needed::new(1))));

        // Test with input that has only alphanumeric characters.
        assert_eq!(test_fn("9zZ"), Ok(("", "9zZ")));

        // Test with input that has only non-alphanumeric characters.
        assert_eq!(test_fn("?!"), Ok(("?!", "")));

        // Test with numeric input only.
        assert_eq!(test_fn("123"), Ok(("", "123")));

        // Test with alphabetic input only.
        assert_eq!(test_fn("abcXYZ"), Ok(("", "abcXYZ")));

        // Test with input that ends with non-alphanumeric characters.
        assert_eq!(test_fn("123ABCDE!@#"), Ok(("!@#", "123ABCDE")));

        // Test with input that has a sequence of alphanumeric followed by non-alphanumeric characters.
        assert_eq!(test_fn("abc123!XYZ"), Ok(("!XYZ", "abc123")));

        // Test with input that has no alphanumeric characters at all.
        assert_eq!(test_fn("!@#"), Ok(("!@#", "")));
    }
}#[cfg(test)]
mod tests_llm_16_355 {
    use crate::{Err, IResult, Needed};
    use crate::error::ErrorKind;
    use crate::character::streaming::alphanumeric1;

    #[test]
    fn test_alphanumeric1() {
        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("abc123");
        assert_eq!(res, Ok(("", "abc123")));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("123abc!");
        assert_eq!(res, Ok(("!", "123abc")));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("12345");
        assert_eq!(res, Ok(("", "12345")));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("abcXYZ");
        assert_eq!(res, Ok(("", "abcXYZ")));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("!@#");
        assert_eq!(res, Err(Err::Error(crate::error::Error::new("!@#", ErrorKind::AlphaNumeric))));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("");
        assert_eq!(res, Err(Err::Incomplete(Needed::new(1))));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("123abc!@#");
        assert_eq!(res, Ok(("!@#", "123abc")));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("abc!123");
        assert_eq!(res, Ok(("!123", "abc")));

        let res: IResult<&str, &str, crate::error::Error<&str>> = alphanumeric1("abc");
        assert_eq!(res, Ok(("", "abc")));
    }
}#[cfg(test)]
mod tests_llm_16_357 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::character::streaming::char;
    use crate::error::ParseError;
    use crate::traits::{AsChar, Input};

    fn parser(i: &str) -> IResult<&str, char> {
        char('a')(i)
    }

    #[test]
    fn char_success() {
        assert_eq!(parser("abc"), Ok(("bc", 'a')));
    }

    #[test]
    fn char_failure() {
        assert_eq!(
            parser("bc"),
            Err(Err::Error(Error::new("bc", ErrorKind::Char)))
        );
    }

    #[test]
    fn char_incomplete() {
        assert_eq!(parser(""), Err(Err::Incomplete(Needed::new(1))));
    }
}#[cfg(test)]
mod tests_llm_16_358 {
    use crate::{Err, IResult, Needed, error::{ErrorKind, ParseError}};
    use crate::character::streaming::crlf;

    #[test]
    fn test_crlf_success() {
        fn crlf_tester(input: &str) -> IResult<&str, &str> {
            crlf(input)
        }

        // Successful match of the crlf ending
        assert_eq!(crlf_tester("\r\nabc"), Ok(("abc", "\r\n")));
    }

    #[test]
    fn test_crlf_incomplete() {
        fn crlf_tester(input: &str) -> IResult<&str, &str> {
            crlf(input)
        }

        // Incomplete match where the crlf ending is only partially present
        assert_eq!(crlf_tester("\r"), Err(Err::Incomplete(Needed::new(2))));
    }

    #[test]
    fn test_crlf_error() {
        fn crlf_tester(input: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
            crlf(input)
        }

        // Error match where there is no crlf ending
        assert_eq!(crlf_tester("abc"), Err(Err::Error(("abc", ErrorKind::CrLf))));
    }
}#[cfg(test)]
mod tests_llm_16_359_llm_16_359 {
    use crate::character::streaming::digit0;
    use crate::error::ErrorKind;
    use crate::error::ParseError;
    use crate::traits::{AsChar, Input};
    use crate::{Err, IResult, Needed};

    #[test]
    fn digit0_empty_input() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>(""),
            Err(Err::Incomplete(Needed::new(1)))
        );
    }

    #[test]
    fn digit0_no_digit() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("abc"),
            Ok(("abc", ""))
        );
    }

    #[test]
    fn digit0_with_leading_digits() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("123abc"),
            Ok(("abc", "123"))
        );
    }

    #[test]
    fn digit0_only_digits() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("123"),
            Ok(("", "123"))
        );
    }

    #[test]
    fn digit0_with_trailing_digits() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("abc123"),
            Ok(("abc123", ""))
        );
    }

    #[test]
    fn digit0_with_leading_and_trailing_digits() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("123abc123"),
            Ok(("abc123", "123"))
        );
    }

    #[test]
    fn digit0_with_special_char() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("123!#@"),
            Ok(("!#@", "123"))
        );
    }

    #[test]
    fn digit0_with_newline() {
        assert_eq!(
            digit0::<&str, (&str, ErrorKind)>("123\n"),
            Ok(("\n", "123"))
        );
    }
}#[cfg(test)]
mod tests_llm_16_360 {
  use crate::{Err, error::ErrorKind, IResult, Needed};
  use crate::character::streaming::digit1;

  #[test]
  fn digit1_success() {
    assert_eq!(digit1::<_, (_, ErrorKind)>("123abc"), Ok(("abc", "123")));
    assert_eq!(digit1::<_, (_, ErrorKind)>("9"), Ok(("", "9")));
    assert_eq!(digit1::<_, (_, ErrorKind)>("0 "), Ok((" ", "0")));
    assert_eq!(digit1::<_, (_, ErrorKind)>("9876543210xyz"), Ok(("xyz", "9876543210")));
  }

  #[test]
  fn digit1_incomplete() {
    assert_eq!(digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn digit1_error() {
    assert_eq!(digit1::<_, (_, ErrorKind)>("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
    assert_eq!(digit1::<_, (_, ErrorKind)>("-123"), Err(Err::Error(("-123", ErrorKind::Digit))));
    assert_eq!(digit1::<_, (_, ErrorKind)>(" abc"), Err(Err::Error((" abc", ErrorKind::Digit))));
    assert_eq!(digit1::<_, (_, ErrorKind)>("abc123"), Err(Err::Error(("abc123", ErrorKind::Digit))));
    assert_eq!(digit1::<_, (_, ErrorKind)>("!"), Err(Err::Error(("!", ErrorKind::Digit))));
  }
}#[cfg(test)]
mod tests_llm_16_361 {
    use crate::{Err, error::ErrorKind, error::Error, IResult, Needed};
    use crate::character::streaming::hex_digit0;

    #[test]
    fn hex_digit0_empty() {
        assert_eq!(hex_digit0::<&str, Error<&str>>(""), Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn hex_digit0_hex() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("1Ae"), Ok(("e", "1A")));
    }

    #[test]
    fn hex_digit0_hex_upper_lower() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("1AeFbB"), Ok(("", "1AeFbB")));
    }

    #[test]
    fn hex_digit0_non_hex_prefix() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("G1A"), Ok(("G1A", "")));
    }

    #[test]
    fn hex_digit0_non_hex_suffix() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("1AG"), Ok(("G", "1A")));
    }

    #[test]
    fn hex_digit0_hex_with_termination() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("1AeZ"), Ok(("Z", "1Ae")));
    }

    #[test]
    fn hex_digit0_numbers_only() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("123"), Ok(("", "123")));
    }

    #[test]
    fn hex_digit0_letters_only() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("aBcD"), Ok(("", "aBcD")));
    }

    #[test]
    fn hex_digit0_mixed_with_special_chars() {
        assert_eq!(hex_digit0::<&str, Error<&str>>("123aBcD-+=!"), Ok(("-+=!", "123aBcD")));
    }
}#[cfg(test)]
mod tests_llm_16_362_llm_16_362 {
    use crate::character::streaming::hex_digit1; // Correcting import path
    use crate::{Err, error::ErrorKind, error::Error, IResult, Needed};

    #[test]
    fn test_hex_digit1_valid_hex() {
        let test_cases = vec![
            ("21cZ", Ok(("Z", "21c"))),
            ("0", Ok(("", "0"))),
            ("1a2B3c", Ok(("", "1a2B3c"))),
            ("Ff", Ok(("", "Ff"))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(hex_digit1::<_, (_, ErrorKind)>(input), expected);
        }
    }

    #[test]
    fn test_hex_digit1_invalid_hex() {
        let test_cases = vec![
            ("", Err(Err::Incomplete(Needed::new(1)))),
            ("g", Err(Err::Error(Error::new("g", ErrorKind::HexDigit)))),
            ("1g", Err(Err::Error(Error::new("g", ErrorKind::HexDigit)))),
            ("1G", Err(Err::Error(Error::new("G", ErrorKind::HexDigit)))),
            ("!1a", Err(Err::Error(Error::new("!1a", ErrorKind::HexDigit)))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(hex_digit1::<_, Error<&str>>(input), expected);
        }
    }

    #[test]
    fn test_hex_digit1_incomplete() {
        let test_cases = vec![
            ("", Err(Err::Incomplete(Needed::new(1)))),
            (" ", Err(Err::Incomplete(Needed::new(1)))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(hex_digit1::<_, Error<&str>>(input), expected);
        }
    }
}#[cfg(test)]
mod tests_llm_16_363 {
  use crate::character::streaming::i128;
  use crate::error::ErrorKind;
  use crate::error::ParseError;
  use crate::error::Error;
  use crate::{Err, IResult, Needed};

  #[test]
  fn parse_positive_i128() {
    let input = "123456789012345678901234567890";
    let result: IResult<&str, i128> = i128(input);
    assert_eq!(result, Ok(("", 123456789012345678901234567890i128)));
  }

  #[test]
  fn parse_negative_i128() {
    let input = "-123456789012345678901234567890";
    let result: IResult<&str, i128> = i128(input);
    assert_eq!(result, Ok(("", -123456789012345678901234567890i128)));
  }

  #[test]
  fn parse_i128_incomplete() {
    let input = "";
    let result: IResult<&str, i128> = i128(input);
    assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn parse_i128_error() {
    let input = "abc";
    let result: IResult<&str, i128, Error<&str>> = i128(input);
    assert_eq!(
        result,
        Err(Err::Error(Error::new(input, ErrorKind::Digit)))
    );
  }

  #[test]
  fn parse_i128_overflow() {
    let input = "1234567890123456789012345678901234567890";
    let result: IResult<&str, i128> = i128(input);
    assert!(matches!(result, Err(Err::Error(_))));
  }
}#[cfg(test)]
mod tests_llm_16_366_llm_16_366 {
    use crate::{
        character::streaming::i64 as parse_i64,
        error::{Error, ErrorKind, ParseError},
        Err, IResult, Needed,
    };

    // Helper to convert from &str to nom's error type, needed for comparison purposes.
    fn from_error_kind(input: &str, kind: ErrorKind) -> crate::Err<Error<&str>> {
        Err::Error(Error::from_error_kind(input, kind))
    }

    #[test]
    fn parse_positive_i64() {
        let input = "12345";
        let expected = Ok(("", 12345i64));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_negative_i64() {
        let input = "-12345";
        let expected = Ok(("", -12345i64));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_i64_incomplete() {
        let input = "";
        let expected = Err(Err::Incomplete(Needed::new(1)));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_i64_with_leading_space() {
        let input = " 12345";
        let expected = Err(from_error_kind(input, ErrorKind::Digit));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_i64_with_trailing_chars() {
        let input = "12345abc";
        let expected = Ok(("abc", 12345i64));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_i64_with_invalid_chars() {
        let input = "abc";
        let expected = Err(from_error_kind(input, ErrorKind::Digit));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_i64_with_overflow() {
        let input = "9223372036854775808"; // i64::MAX + 1
        let expected = Err(from_error_kind(input, ErrorKind::Digit));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_i64_with_underflow() {
        let input = "-9223372036854775809"; // i64::MIN - 1
        let expected = Err(from_error_kind(input, ErrorKind::Digit));
        let result = parse_i64::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }
}#[cfg(test)]
mod test {
  use super::*;

use crate::*;
  use crate::{
    error::{Error, ErrorKind},
    Err, IResult, Needed,
  };

  #[test]
  fn detect_line_ending_newline() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>("some text\nmore text"),
      Ok(("more text", "some text\n"))
    );
  }

  #[test]
  fn detect_line_ending_crlf() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>("some text\r\nmore text"),
      Ok(("more text", "some text\r\n"))
    );
  }

  #[test]
  fn no_line_ending() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>("some text without line ending"),
      Err(Err::Error(("some text without line ending", ErrorKind::CrLf)))
    );
  }

  #[test]
  fn incomplete_newline() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>("some text with incomplete line ending\n"),
      Ok(("", "some text with incomplete line ending\n"))
    );
  }

  #[test]
  fn incomplete_crlf() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>("some text with incomplete crlf\r\n"),
      Ok(("", "some text with incomplete crlf\r\n"))
    );
  }

  #[test]
  fn incomplete_input() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>(""),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test]
  fn input_ending_with_cr() {
    assert_eq!(
      line_ending::<_, (_, ErrorKind)>("ending with cr\r"),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test]
  fn test_error_conversion() {
    let partial_input = "some text\r";
    assert_eq!(
      line_ending::<_, Error<&str>>(partial_input),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }
}#[cfg(test)]
mod tests_llm_16_369 {
    use crate::{
        character::streaming::multispace0,
        error::{ErrorKind, ParseError},
        Err, IResult, Needed,
    };

    // Helper macro to assert error equality based on IResult
    macro_rules! assert_err {
        ($left:expr, $right:expr) => {
            match ($left, $right) {
                (Err(Err::Error(e1)), Err(Err::Error(e2))) | (Err(Err::Failure(e1)), Err(Err::Failure(e2))) => {
                    assert_eq!(e1.code, e2.code)
                }
                _ => assert!($left.is_err() && $right.is_err()),
            }
        };
    }

    // Assert `multispace0` with a string having leading spaces
    #[test]
    fn multispace0_leading_spaces() {
        assert_eq!(
            multispace0::<_, (_, ErrorKind)>(" \t\n\rtrail"),
            Ok(("trail", " \t\n\r"))
        );
    }

    // Assert `multispace0` with a string having no leading spaces
    #[test]
    fn multispace0_no_leading_spaces() {
        assert_eq!(multispace0::<_, (_, ErrorKind)>("trail"), Ok(("trail", "")));
    }

    // Assert `multispace0` with an empty string
    #[test]
    fn multispace0_empty() {
        assert_eq!(multispace0::<_, (_, ErrorKind)>(""), Ok(("", "")));
    }

    // Assert `multispace0` Incomplete handling with a partial input
    #[test]
    fn multispace0_incomplete() {
        // As multispace0 will match and consume all spaces, it will never return
        // Err::Incomplete, so here we can only check that it returns a result
        // test this with a complete input that should not return Incomplete
        assert!(
            matches!(
                multispace0::<_, (_, ErrorKind)>(" \t\n\r"),
                Ok(("", _))
            )
        );
    }

    // Assert `multispace0` Error handling with not space characters
    #[test]
    fn multispace0_not_space_chars() {
        // multispace0 should consume all spaces and return what follows
        // test this with a complete input that should return what's after spaces
        assert_eq!(
            multispace0::<_, (_, ErrorKind)>(" \t\n\rtrail space"),
            Ok(("trail space", " \t\n\r"))
        );
    }
}#[cfg(test)]
mod tests_llm_16_370_llm_16_370 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        character::streaming::multispace1,
        IResult, Err, Needed,
    };

    #[test]
    fn test_multispace1() {
        fn test_func(input: &str) -> IResult<&str, &str, Error<&str>> {
            multispace1(input)
        }

        assert_eq!(test_func(" \t\r\nab"), Ok(("ab", " \t\r\n")));
        assert_eq!(test_func("abc"), Err(Err::Error(Error::new("abc", ErrorKind::MultiSpace))));
        assert_eq!(test_func("1\r\n \t"), Ok(("1", "\r\n \t")));
        assert_eq!(test_func(" \r\n \t"), Ok((" ", "\r\n \t")));
        assert_eq!(test_func(""), Err(Err::Incomplete(Needed::new(1))));
        assert_eq!(test_func("a b"), Err(Err::Error(Error::new("a b", ErrorKind::MultiSpace))));
        assert_eq!(test_func("\n \r\n"), Ok(("\n", " \r\n")));
    }
}#[cfg(test)]
mod tests_llm_16_371 {
  use crate::{Err, error::ErrorKind, IResult, Needed};
  use crate::character::streaming::newline;

  #[test]
  fn success_newline() {
    assert_eq!(newline::<_, (_, ErrorKind)>("\nc"), Ok(("c", '\n')));
  }

  #[test]
  fn error_newline_with_invalid_input() {
    assert_eq!(newline::<_, (_, ErrorKind)>("\r\nc"), Err(Err::Error(("\r\nc", ErrorKind::Char))));
  }

  #[test]
  fn error_newline_with_incomplete_input() {
    assert_eq!(newline::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
  }
}#[cfg(test)]
mod tests_llm_16_374_llm_16_374 {
  use super::*;

use crate::*;
  use crate::{
    error::{Error, ErrorKind},
    Err,
    IResult,
    Needed,
  };

  #[test]
  fn oct_digit0_empty() {
    let input = ""; // Empty input
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn oct_digit0_only_octal_digits() {
    let input = "12345670"; // Only octal digits
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("", "12345670")));
  }

  #[test]
  fn oct_digit0_octal_digits_followed_by_non_octal() {
    let input = "12345670abc"; // Octal digits followed by non-octal
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("abc", "12345670")));
  }

  #[test]
  fn oct_digit0_non_octal() {
    let input = "abc"; // Non-octal digits
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("abc", "")));
  }

  #[test]
  fn oct_digit0_octal_digits_followed_by_zero() {
    let input = "123456700"; // Octal digits followed by 0
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("", "123456700")));
  }

  #[test]
  fn oct_digit0_leading_zeros() {
    let input = "00012345670"; // Leading zeros
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("", "00012345670")));
  }

  // The test below has been removed due to invalid UTF-8 sequence which cannot be included in the code.
  // #[test]
  // fn oct_digit0_with_invalid_utf8() {
  //   let input = "1234\xF05670"; // Invalid UTF-8 sequence
  //   let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
  //   assert!(res.is_err());
  // }

  #[test]
  fn oct_digit0_with_special_chars() {
    let input = "1234\n5670"; // Octal digits with special char in between
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("\n5670", "1234")));
  }

  #[test]
  fn oct_digit0_with_some_utf8_chars() {
    let input = "1234ö5670"; // Octal digits with UTF-8 char in between
    let res: IResult<&str, &str, Error<&str>> = oct_digit0(input);
    assert_eq!(res, Ok(("ö5670", "1234")));
  }
}#[cfg(test)]
mod tests_llm_16_375 {
    use crate::{Err, error::{ErrorKind, ParseError}, Needed};
    use crate::character::streaming::oct_digit1;
    use crate::IResult;

    #[test]
    fn test_oct_digit1() {
        fn test_parser(input: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
            oct_digit1(input)
        }

        // Normal case
        assert_eq!(test_parser("12345670"), Ok(("","12345670")));
        // Incomplete case
        assert_eq!(test_parser(""), Err(Err::Incomplete(Needed::new(1))));
        // Error case: invalid octal digit
        assert_eq!(test_parser("89"), Err(Err::Error(("89", ErrorKind::OctDigit))));
        // Incomplete case in the middle of input
        assert_eq!(test_parser("1234 "), Err(Err::Error((" ", ErrorKind::OctDigit))));
        // Incomplete due to EOF
        assert!(matches!(test_parser("123"), Ok(("","123"))));
        // Error due to invalid first character
        assert_eq!(test_parser("abc"), Err(Err::Error(("abc", ErrorKind::OctDigit))));
        // Valid octal followed by other input
        assert_eq!(test_parser("123x456"), Ok(("x456","123")));
        // Leading zeroes
        assert_eq!(test_parser("00123"), Ok(("", "00123")));
    }
}#[cfg(test)]
mod tests_llm_16_376 {
    use crate::{Err, Needed};
    use crate::error::{ErrorKind, ParseError};
    use crate::character::streaming::one_of;
    use crate::error::Error;
    use crate::IResult;

    #[test]
    fn one_of_match_single_character() {
        let result: IResult<&str, char, Error<&str>> = one_of("abc")("b");
        assert_eq!(result, Ok(("", 'b')));
    }

    #[test]
    fn one_of_no_match_single_character() {
        let result: IResult<&str, char, Error<&str>> = one_of("a")("bc");
        assert_eq!(result, Err(Err::Error(Error::new("bc", ErrorKind::OneOf))));
    }

    #[test]
    fn one_of_incomplete() {
        let result: IResult<&str, char, Error<&str>> = one_of("a")("");
        assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn one_of_match_multiple_characters() {
        let result: IResult<&str, char, Error<&str>> = one_of("abc")("ade");
        assert_eq!(result, Ok(("de", 'a')));
    }

    #[test]
    fn one_of_match_end_of_input() {
        let result: IResult<&str, char, Error<&str>> = one_of("a")("a");
        assert_eq!(result, Ok(("", 'a')));
    }

    #[test]
    fn one_of_match_with_leading_space() {
        let result: IResult<&str, char, Error<&str>> = one_of("abc")(" b");
        assert_eq!(result, Ok((" b", ' ')));
    }

    #[test]
    fn one_of_no_match_empty_input() {
        let result: IResult<&str, char, Error<&str>> = one_of("abc")("");
        assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn one_of_no_match_no_common_characters() {
        let result: IResult<&str, char, Error<&str>> = one_of("abc")("def");
        assert_eq!(result, Err(Err::Error(Error::new("def", ErrorKind::OneOf))));
    }
}#[cfg(test)]
mod tests_llm_16_377 {
    use crate::{Err, error::{ErrorKind, Error}, Needed, IResult};
    use crate::character::streaming::satisfy;
    use crate::error::ParseError;
    use crate::traits::{AsChar, Input};

    #[test]
    fn satisfy_parser() {
        fn parser(i: &str) -> IResult<&str, char, Error<&str>> {
            satisfy(|c| c == 'a' || c == 'b')(i)
        }

        assert_eq!(parser("abc"), Ok(("bc", 'a')));
        assert_eq!(
            parser("cd"),
            Err(Err::Error(Error::new("cd", ErrorKind::Satisfy)))
        );
        assert_eq!(parser(""), Err(Err::Incomplete(Needed::Unknown)));
    }
}#[cfg(test)]
mod tests_llm_16_378 {
  use crate::{
    error::{ErrorKind, ParseError},
    IResult,
    character::streaming::sign,
  };

  #[test]
  fn sign_positive() {
    let input = "+";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("", true)));
  }

  #[test]
  fn sign_negative() {
    let input = "-";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("", false)));
  }

  #[test]
  fn sign_no_sign() {
    let input = "123";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("123", true)));
  }

  #[test]
  fn sign_empty() {
    let input = "";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("", true)));
  }

  #[test]
  fn sign_only_plus() {
    let input = "+123";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("123", true)));
  }

  #[test]
  fn sign_only_minus() {
    let input = "-123";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("123", false)));
  }

  #[test]
  fn sign_wrong_sign() {
    let input = "*123";
    assert_eq!(sign::<_, (&str, ErrorKind)>(input), Ok(("*123", true)));
  }
}#[cfg(test)]
mod tests_llm_16_379 {
    use crate::{Err, error::{Error, ErrorKind, ParseError}, IResult, Needed};
    use crate::character::streaming::space0;
    use crate::traits::Input;

    #[test]
    fn space0_empty() {
        let res: IResult<&str, &str, Error<&str>> = space0("");
        assert_eq!(res, Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn space0_no_space() {
        let res: IResult<&str, &str, Error<&str>> = space0("Z21c");
        assert_eq!(res, Ok(("Z21c", "")));
    }

    #[test]
    fn space0_with_spaces() {
        let res: IResult<&str, &str, Error<&str>> = space0(" \t21c");
        assert_eq!(res, Ok(("21c", " \t")));
    }

    #[test]
    fn space0_only_spaces() {
        let res: IResult<&str, &str, Error<&str>> = space0(" \t    ");
        assert_eq!(res, Ok(("", " \t    ")));
    }

    #[test]
    fn space0_newline() {
        let res: IResult<&str, &str, Error<&str>> = space0("\nZ21c");
        assert_eq!(res, Ok(("\nZ21c", "")));
    }

    #[test]
    fn space0_space_incomplete() {
        let res: IResult<&str, &str, Error<&str>> = space0(" \t");
        assert_eq!(res, Ok(("", " \t")));
    }

    #[test]
    fn space0_only_spaces_incomplete() {
        let res: IResult<&str, &str, Error<&str>> = space0("    \t");
        assert_eq!(res, Ok(("", "    \t")));
    }
}#[cfg(test)]
mod tests_llm_16_381 {
  use crate::{Err, error::ErrorKind, IResult, Needed};
  use crate::character::streaming::tab;

  #[test]
  fn tab_char() {
    assert_eq!(tab::<_, (_, ErrorKind)>("\tc"), Ok(("c", '\t')));
  }

  #[test]
  fn tab_not_char() {
    assert_eq!(tab::<_, (_, ErrorKind)>("\r\nc"), Err(Err::Error(("\r\nc", ErrorKind::Char))));
  }

  #[test]
  fn tab_incomplete() {
    assert_eq!(tab::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
  }
}#[cfg(test)]
mod tests_llm_16_385_llm_16_385 {
    use super::*;

use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };

    #[test]
    fn u64_test() {
        type TestError<'a> = Error<&'a str>;

        // Test parsing a valid u64
        let input = "12345";
        let res: IResult<&str, u64, TestError> = super::u64(input);
        assert_eq!(res, Ok(("", 12345)));

        // Test parsing an empty input, which should return an error
        let input = "";
        let res: IResult<&str, u64, TestError> = super::u64(input);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(1))));

        // Test parsing a string starting with non-digits
        let input = "abc";
        let res: IResult<&str, u64, TestError> = super::u64(input);
        assert!(res.is_err());

        // Test parsing a string that has valid digits and then an error
        let input = "123abc";
        let res: IResult<&str, u64, TestError> = super::u64(input);
        assert_eq!(res, Ok(("abc", 123)));

        // Test for possible overflow for u64
        let input = "18446744073709551615"; // u64::MAX
        let res: IResult<&str, u64, TestError> = super::u64(input);
        assert_eq!(res, Ok(("", u64::MAX)));

        // Test for overflow past u64::MAX
        let input = "18446744073709551616"; // u64::MAX + 1
        let res: IResult<&str, u64, TestError> = super::u64(input);
        assert!(res.is_err());
    }
}