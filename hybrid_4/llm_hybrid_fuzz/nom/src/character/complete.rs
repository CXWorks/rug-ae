//! Character specific parsers and combinators, complete input version.
//!
//! Functions recognizing specific characters.

use crate::branch::alt;
use crate::combinator::opt;
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, IResult};
use crate::traits::{AsChar, FindToken, Input, InputLength};
use crate::traits::{Compare, CompareResult};

/// Recognizes one character.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{ErrorKind, Error}, IResult};
/// # use nom::character::complete::char;
/// fn parser(i: &str) -> IResult<&str, char> {
///     char('a')(i)
/// }
/// assert_eq!(parser("abc"), Ok(("bc", 'a')));
/// assert_eq!(parser(" abc"), Err(Err::Error(Error::new(" abc", ErrorKind::Char))));
/// assert_eq!(parser("bc"), Err(Err::Error(Error::new("bc", ErrorKind::Char))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Char))));
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
    Some((c, true)) => Ok((i.take_from(c.len()), c.as_char())),
    _ => Err(Err::Error(Error::from_char(i, c))),
  }
}

/// Recognizes one character and checks that it satisfies a predicate
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{ErrorKind, Error}, Needed, IResult};
/// # use nom::character::complete::satisfy;
/// fn parser(i: &str) -> IResult<&str, char> {
///     satisfy(|c| c == 'a' || c == 'b')(i)
/// }
/// assert_eq!(parser("abc"), Ok(("bc", 'a')));
/// assert_eq!(parser("cd"), Err(Err::Error(Error::new("cd", ErrorKind::Satisfy))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Satisfy))));
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
    Some((c, true)) => Ok((i.take_from(c.len()), c)),
    _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Satisfy))),
  }
}

/// Recognizes one of the provided characters.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind};
/// # use nom::character::complete::one_of;
/// assert_eq!(one_of::<_, _, (&str, ErrorKind)>("abc")("b"), Ok(("", 'b')));
/// assert_eq!(one_of::<_, _, (&str, ErrorKind)>("a")("bc"), Err(Err::Error(("bc", ErrorKind::OneOf))));
/// assert_eq!(one_of::<_, _, (&str, ErrorKind)>("a")(""), Err(Err::Error(("", ErrorKind::OneOf))));
/// ```
pub fn one_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
  T: FindToken<<I as Input>::Item>,
{
  move |i: I| match (i).iter_elements().next().map(|c| (c, list.find_token(c))) {
    Some((c, true)) => Ok((i.take_from(c.len()), c.as_char())),
    _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::OneOf))),
  }
}

/// Recognizes a character that is not in the provided characters.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind};
/// # use nom::character::complete::none_of;
/// assert_eq!(none_of::<_, _, (&str, ErrorKind)>("abc")("z"), Ok(("", 'z')));
/// assert_eq!(none_of::<_, _, (&str, ErrorKind)>("ab")("a"), Err(Err::Error(("a", ErrorKind::NoneOf))));
/// assert_eq!(none_of::<_, _, (&str, ErrorKind)>("a")(""), Err(Err::Error(("", ErrorKind::NoneOf))));
/// ```
pub fn none_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Input,
  <I as Input>::Item: AsChar,
  T: FindToken<<I as Input>::Item>,
{
  move |i: I| match (i).iter_elements().next().map(|c| (c, !list.find_token(c))) {
    Some((c, true)) => Ok((i.take_from(c.len()), c.as_char())),
    _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::NoneOf))),
  }
}

/// Recognizes the string "\r\n".
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult};
/// # use nom::character::complete::crlf;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     crlf(input)
/// }
///
/// assert_eq!(parser("\r\nc"), Ok(("c", "\r\n")));
/// assert_eq!(parser("ab\r\nc"), Err(Err::Error(Error::new("ab\r\nc", ErrorKind::CrLf))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::CrLf))));
/// ```
pub fn crlf<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  T: Compare<&'static str>,
{
  match input.compare("\r\n") {
    CompareResult::Ok => Ok(input.take_split(2)),
    _ => {
      let e: ErrorKind = ErrorKind::CrLf;
      Err(Err::Error(E::from_error_kind(input, e)))
    }
  }
}

//FIXME: there's still an incomplete
/// Recognizes a string of any char except '\r\n' or '\n'.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::not_line_ending;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     not_line_ending(input)
/// }
///
/// assert_eq!(parser("ab\r\nc"), Ok(("\r\nc", "ab")));
/// assert_eq!(parser("ab\nc"), Ok(("\nc", "ab")));
/// assert_eq!(parser("abc"), Ok(("", "abc")));
/// assert_eq!(parser(""), Ok(("", "")));
/// assert_eq!(parser("a\rb\nc"), Err(Err::Error(Error { input: "a\rb\nc", code: ErrorKind::Tag })));
/// assert_eq!(parser("a\rbc"), Err(Err::Error(Error { input: "a\rbc", code: ErrorKind::Tag })));
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
    None => Ok(input.take_split(input.input_len())),
    Some(index) => {
      let mut it = input.take_from(index).iter_elements();
      let nth = it.next().unwrap().as_char();
      if nth == '\r' {
        let sliced = input.take_from(index);
        let comp = sliced.compare("\r\n");
        match comp {
          //FIXME: calculate the right index
          CompareResult::Ok => Ok(input.take_split(index)),
          _ => {
            let e: ErrorKind = ErrorKind::Tag;
            Err(Err::Error(E::from_error_kind(input, e)))
          }
        }
      } else {
        Ok(input.take_split(index))
      }
    }
  }
}

/// Recognizes an end of line (both '\n' and '\r\n').
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::line_ending;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     line_ending(input)
/// }
///
/// assert_eq!(parser("\r\nc"), Ok(("c", "\r\n")));
/// assert_eq!(parser("ab\r\nc"), Err(Err::Error(Error::new("ab\r\nc", ErrorKind::CrLf))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::CrLf))));
/// ```
pub fn line_ending<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input + InputLength,
  T: Compare<&'static str>,
{
  match input.compare("\n") {
    CompareResult::Ok => Ok(input.take_split(1)),
    CompareResult::Incomplete => Err(Err::Error(E::from_error_kind(input, ErrorKind::CrLf))),
    CompareResult::Error => match input.compare("\r\n") {
      CompareResult::Ok => Ok(input.take_split(2)),
      _ => Err(Err::Error(E::from_error_kind(input, ErrorKind::CrLf))),
    },
  }
}

/// Matches a newline character '\n'.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::newline;
/// fn parser(input: &str) -> IResult<&str, char> {
///     newline(input)
/// }
///
/// assert_eq!(parser("\nc"), Ok(("c", '\n')));
/// assert_eq!(parser("\r\nc"), Err(Err::Error(Error::new("\r\nc", ErrorKind::Char))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Char))));
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
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::tab;
/// fn parser(input: &str) -> IResult<&str, char> {
///     tab(input)
/// }
///
/// assert_eq!(parser("\tc"), Ok(("c", '\t')));
/// assert_eq!(parser("\r\nc"), Err(Err::Error(Error::new("\r\nc", ErrorKind::Char))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Char))));
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
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```
/// # use nom::{character::complete::anychar, Err, error::{Error, ErrorKind}, IResult};
/// fn parser(input: &str) -> IResult<&str, char> {
///     anychar(input)
/// }
///
/// assert_eq!(parser("abc"), Ok(("bc",'a')));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Eof))));
/// ```
pub fn anychar<T, E: ParseError<T>>(input: T) -> IResult<T, char, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  let mut it = input.iter_elements();
  match it.next() {
    None => Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof))),
    Some(c) => Ok((input.take_from(c.len()), c.as_char())),
  }
}

/// Recognizes zero or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z
///
/// *Complete version*: Will return the whole input if no terminating token is found (a non
/// alphabetic character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::alpha0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     alpha0(input)
/// }
///
/// assert_eq!(parser("ab1c"), Ok(("1c", "ab")));
/// assert_eq!(parser("1c"), Ok(("1c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn alpha0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position_complete(|item| !item.is_alpha())
}

/// Recognizes one or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found  (a non alphabetic character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::alpha1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     alpha1(input)
/// }
///
/// assert_eq!(parser("aB1c"), Ok(("1c", "aB")));
/// assert_eq!(parser("1c"), Err(Err::Error(Error::new("1c", ErrorKind::Alpha))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Alpha))));
/// ```
pub fn alpha1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(|item| !item.is_alpha(), ErrorKind::Alpha)
}

/// Recognizes zero or more ASCII numerical characters: 0-9
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::digit0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     digit0(input)
/// }
///
/// assert_eq!(parser("21c"), Ok(("c", "21")));
/// assert_eq!(parser("21"), Ok(("", "21")));
/// assert_eq!(parser("a21c"), Ok(("a21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn digit0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position_complete(|item| !item.is_dec_digit())
}

/// Recognizes one or more ASCII numerical characters: 0-9
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::digit1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     digit1(input)
/// }
///
/// assert_eq!(parser("21c"), Ok(("c", "21")));
/// assert_eq!(parser("c1"), Err(Err::Error(Error::new("c1", ErrorKind::Digit))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Digit))));
/// ```
///
/// ## Parsing an integer
/// You can use `digit1` in combination with [`map_res`] to parse an integer:
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::combinator::map_res;
/// # use nom::character::complete::digit1;
/// fn parser(input: &str) -> IResult<&str, u32> {
///   map_res(digit1, str::parse)(input)
/// }
///
/// assert_eq!(parser("416"), Ok(("", 416)));
/// assert_eq!(parser("12b"), Ok(("b", 12)));
/// assert!(parser("b").is_err());
/// ```
///
/// [`map_res`]: crate::combinator::map_res
pub fn digit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(|item| !item.is_dec_digit(), ErrorKind::Digit)
}

/// Recognizes zero or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f
///
/// *Complete version*: Will return the whole input if no terminating token is found (a non hexadecimal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::hex_digit0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     hex_digit0(input)
/// }
///
/// assert_eq!(parser("21cZ"), Ok(("Z", "21c")));
/// assert_eq!(parser("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn hex_digit0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position_complete(|item| !item.is_hex_digit())
}
/// Recognizes one or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non hexadecimal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::hex_digit1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     hex_digit1(input)
/// }
///
/// assert_eq!(parser("21cZ"), Ok(("Z", "21c")));
/// assert_eq!(parser("H2"), Err(Err::Error(Error::new("H2", ErrorKind::HexDigit))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::HexDigit))));
/// ```
pub fn hex_digit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(|item| !item.is_hex_digit(), ErrorKind::HexDigit)
}

/// Recognizes zero or more octal characters: 0-7
///
/// *Complete version*: Will return the whole input if no terminating token is found (a non octal
/// digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::oct_digit0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     oct_digit0(input)
/// }
///
/// assert_eq!(parser("21cZ"), Ok(("cZ", "21")));
/// assert_eq!(parser("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn oct_digit0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position_complete(|item| !item.is_oct_digit())
}

/// Recognizes one or more octal characters: 0-7
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non octal digit character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::oct_digit1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     oct_digit1(input)
/// }
///
/// assert_eq!(parser("21cZ"), Ok(("cZ", "21")));
/// assert_eq!(parser("H2"), Err(Err::Error(Error::new("H2", ErrorKind::OctDigit))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::OctDigit))));
/// ```
pub fn oct_digit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(|item| !item.is_oct_digit(), ErrorKind::OctDigit)
}

/// Recognizes zero or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z
///
/// *Complete version*: Will return the whole input if no terminating token is found (a non
/// alphanumerical character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::alphanumeric0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     alphanumeric0(input)
/// }
///
/// assert_eq!(parser("21cZ%1"), Ok(("%1", "21cZ")));
/// assert_eq!(parser("&Z21c"), Ok(("&Z21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn alphanumeric0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position_complete(|item| !item.is_alphanum())
}

/// Recognizes one or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non alphanumerical character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::alphanumeric1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     alphanumeric1(input)
/// }
///
/// assert_eq!(parser("21cZ%1"), Ok(("%1", "21cZ")));
/// assert_eq!(parser("&H2"), Err(Err::Error(Error::new("&H2", ErrorKind::AlphaNumeric))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::AlphaNumeric))));
/// ```
pub fn alphanumeric1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(|item| !item.is_alphanum(), ErrorKind::AlphaNumeric)
}

/// Recognizes zero or more spaces and tabs.
///
/// *Complete version*: Will return the whole input if no terminating token is found (a non space
/// character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::space0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     space0(input)
/// }
///
/// assert_eq!(parser(" \t21c"), Ok(("21c", " \t")));
/// assert_eq!(parser("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn space0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar + Clone,
{
  input.split_at_position_complete(|item| {
    let c = item.as_char();
    !(c == ' ' || c == '\t')
  })
}

/// Recognizes one or more spaces and tabs.
///
/// *Complete version*: Will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::space1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     space1(input)
/// }
///
/// assert_eq!(parser(" \t21c"), Ok(("21c", " \t")));
/// assert_eq!(parser("H2"), Err(Err::Error(Error::new("H2", ErrorKind::Space))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Space))));
/// ```
pub fn space1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(
    |item| {
      let c = item.as_char();
      !(c == ' ' || c == '\t')
    },
    ErrorKind::Space,
  )
}

/// Recognizes zero or more spaces, tabs, carriage returns and line feeds.
///
/// *Complete version*: will return the whole input if no terminating token is found (a non space
/// character).
/// # Example
///
/// ```
/// # use nom::{Err, error::ErrorKind, IResult, Needed};
/// # use nom::character::complete::multispace0;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     multispace0(input)
/// }
///
/// assert_eq!(parser(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
/// assert_eq!(parser("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn multispace0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position_complete(|item| {
    let c = item.as_char();
    !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
  })
}

/// Recognizes one or more spaces, tabs, carriage returns and line feeds.
///
/// *Complete version*: will return an error if there's not enough input data,
/// or the whole input if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom::character::complete::multispace1;
/// fn parser(input: &str) -> IResult<&str, &str> {
///     multispace1(input)
/// }
///
/// assert_eq!(parser(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
/// assert_eq!(parser("H2"), Err(Err::Error(Error::new("H2", ErrorKind::MultiSpace))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::MultiSpace))));
/// ```
pub fn multispace1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
  <T as Input>::Item: AsChar,
{
  input.split_at_position1_complete(
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
  use crate::bytes::complete::tag;
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
            T: Input  + Clone,
            <T as Input>::Item: AsChar,
            T: for <'a> Compare<&'a[u8]>,
            {
                let (i, sign) = sign(input.clone())?;

                if i.input_len() == 0 {
                    return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
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

                Ok((i.take_from(i.input_len()), value))
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
                    return Err(Err::Error(E::from_error_kind(i, ErrorKind::Digit)));
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

                Ok((i.take_from(i.input_len()), value))
            }
        )+
    }
}

uints! { u8 u16 u32 u64 u128 }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::internal::Err;
  use crate::traits::ParseTo;
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, (_, ErrorKind)> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn character() {
    let empty: &[u8] = b"";
    let a: &[u8] = b"abcd";
    let b: &[u8] = b"1234";
    let c: &[u8] = b"a123";
    let d: &[u8] = "azé12".as_bytes();
    let e: &[u8] = b" ";
    let f: &[u8] = b" ;";
    //assert_eq!(alpha1::<_, (_, ErrorKind)>(a), Err(Err::Incomplete(Needed::Size(1))));
    assert_parse!(alpha1(a), Ok((empty, a)));
    assert_eq!(alpha1(b), Err(Err::Error((b, ErrorKind::Alpha))));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(c), Ok((&c[1..], &b"a"[..])));
    assert_eq!(
      alpha1::<_, (_, ErrorKind)>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(digit1(a), Err(Err::Error((a, ErrorKind::Digit))));
    assert_eq!(digit1::<_, (_, ErrorKind)>(b), Ok((empty, b)));
    assert_eq!(digit1(c), Err(Err::Error((c, ErrorKind::Digit))));
    assert_eq!(digit1(d), Err(Err::Error((d, ErrorKind::Digit))));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(a), Ok((empty, a)));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(b), Ok((empty, b)));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(c), Ok((empty, c)));
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(d),
      Ok(("zé12".as_bytes(), &b"a"[..]))
    );
    assert_eq!(hex_digit1(e), Err(Err::Error((e, ErrorKind::HexDigit))));
    assert_eq!(oct_digit1(a), Err(Err::Error((a, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1::<_, (_, ErrorKind)>(b), Ok((empty, b)));
    assert_eq!(oct_digit1(c), Err(Err::Error((c, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1(d), Err(Err::Error((d, ErrorKind::OctDigit))));
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(a), Ok((empty, a)));
    //assert_eq!(fix_error!(b,(), alphanumeric), Ok((empty, b)));
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(c), Ok((empty, c)));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(space1::<_, (_, ErrorKind)>(e), Ok((empty, e)));
    assert_eq!(space1::<_, (_, ErrorKind)>(f), Ok((&b";"[..], &b" "[..])));
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn character_s() {
    let empty = "";
    let a = "abcd";
    let b = "1234";
    let c = "a123";
    let d = "azé12";
    let e = " ";
    assert_eq!(alpha1::<_, (_, ErrorKind)>(a), Ok((empty, a)));
    assert_eq!(alpha1(b), Err(Err::Error((b, ErrorKind::Alpha))));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(c), Ok((&c[1..], "a")));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(d), Ok(("é12", "az")));
    assert_eq!(digit1(a), Err(Err::Error((a, ErrorKind::Digit))));
    assert_eq!(digit1::<_, (_, ErrorKind)>(b), Ok((empty, b)));
    assert_eq!(digit1(c), Err(Err::Error((c, ErrorKind::Digit))));
    assert_eq!(digit1(d), Err(Err::Error((d, ErrorKind::Digit))));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(a), Ok((empty, a)));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(b), Ok((empty, b)));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(c), Ok((empty, c)));
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(d), Ok(("zé12", "a")));
    assert_eq!(hex_digit1(e), Err(Err::Error((e, ErrorKind::HexDigit))));
    assert_eq!(oct_digit1(a), Err(Err::Error((a, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1::<_, (_, ErrorKind)>(b), Ok((empty, b)));
    assert_eq!(oct_digit1(c), Err(Err::Error((c, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1(d), Err(Err::Error((d, ErrorKind::OctDigit))));
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(a), Ok((empty, a)));
    //assert_eq!(fix_error!(b,(), alphanumeric), Ok((empty, b)));
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(c), Ok((empty, c)));
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(d), Ok(("é12", "az")));
    assert_eq!(space1::<_, (_, ErrorKind)>(e), Ok((empty, e)));
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
    assert_eq!(not_line_ending::<_, (_, ErrorKind)>(d), Ok((&[][..], d)));
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
    assert_eq!(not_line_ending::<_, (_, ErrorKind)>(g2), Ok(("", g2)));
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
    use crate::sequence::pair;
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\r\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\r\n"[..]))));
  }

  #[test]
  fn full_line_unix() {
    use crate::sequence::pair;
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
    assert_parse!(
      crlf(&b"\r"[..]),
      Err(Err::Error(error_position!(&b"\r"[..], ErrorKind::CrLf)))
    );
    assert_parse!(
      crlf(&b"\ra"[..]),
      Err(Err::Error(error_position!(&b"\ra"[..], ErrorKind::CrLf)))
    );

    assert_parse!(crlf("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(
      crlf("\r"),
      Err(Err::Error(error_position!("\r", ErrorKind::CrLf)))
    );
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
      Err(Err::Error(error_position!(&b"\r"[..], ErrorKind::CrLf)))
    );
    assert_parse!(
      line_ending(&b"\ra"[..]),
      Err(Err::Error(error_position!(&b"\ra"[..], ErrorKind::CrLf)))
    );

    assert_parse!(line_ending("\na"), Ok(("a", "\n")));
    assert_parse!(line_ending("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(
      line_ending("\r"),
      Err(Err::Error(error_position!("\r", ErrorKind::CrLf)))
    );
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
mod tests_llm_16_310 {
    use crate::{error::ErrorKind, Err, IResult};
    use crate::character::complete::alpha0;

    #[test]
    fn test_alpha0() {
        fn parser(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
            alpha0(input)
        }

        let empty: &str = "";
        let alpha: &str = "abcDEF";
        let numeric: &str = "123";
        let alphanumeric: &str = "abc123";
        let non_alpha: &str = "!?><";
        let mixed: &str = "abcXYZ123";
        let full_mixed: &str = "XYZ123abcXYZ";

        assert_eq!(parser(empty), Ok(("", "")));
        assert_eq!(parser(alpha), Ok(("", alpha)));
        assert_eq!(parser(numeric), Ok((numeric, "")));
        assert_eq!(parser(alphanumeric), Ok(("123", "abc")));
        assert_eq!(parser(non_alpha), Ok((non_alpha, "")));
        assert_eq!(parser(mixed), Ok(("123", "abcXYZ")));
        assert_eq!(parser(full_mixed), Ok(("123abcXYZ", "XYZ")));
    }
}#[cfg(test)]
mod tests_llm_16_311 {
    use crate::{
        error::{Error, ErrorKind}, 
        Err, 
        IResult
    };
    use crate::character::complete::alpha1;

    fn parser(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    #[test]
    fn alpha1_at_least_one_alpha() {
        assert_eq!(parser("aB1c"), Ok(("1c", "aB")));
    }

    #[test]
    fn alpha1_no_alpha_at_start() {
        assert_eq!(parser("1c"), Err(Err::Error(Error::new("1c", ErrorKind::Alpha))));
    }

    #[test]
    fn alpha1_empty_input() {
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Alpha))));
    }

    #[test]
    fn alpha1_complete_alpha() {
        assert_eq!(parser("abcXYZ"), Ok(("", "abcXYZ")));
    }

    #[test]
    fn alpha1_start_with_alpha() {
        assert_eq!(parser("Ab1"), Ok(("1", "Ab")));
    }

    #[test]
    fn alpha1_alpha_with_trailing_space() {
        assert_eq!(parser("AbCd "), Ok((" ", "AbCd")));
    }

    #[test]
    fn alpha1_only_non_alpha() {
        assert_eq!(parser("123"), Err(Err::Error(Error::new("123", ErrorKind::Alpha))));
    }
}#[cfg(test)]
mod tests_llm_16_312 {
    use crate::{
        error::{Error, ErrorKind},
        IResult,
    };
    use crate::character::complete::alphanumeric0;

    #[test]
    fn test_alphanumeric0() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            alphanumeric0(input)
        }

        let empty = "";

        assert_eq!(parser("21cZ%1"), Ok(("%1", "21cZ")));
        assert_eq!(parser("&Z21c"), Ok(("&Z21c", empty)));
        assert_eq!(parser(empty), Ok((empty, empty)));
        assert_eq!(parser("abcXYZ09"), Ok((empty, "abcXYZ09")));
        assert_eq!(parser("123!@#$"), Ok(("!@#$", "123")));
        assert_eq!(parser("no-special_chars123"), Ok((empty, "no-special_chars123")));
        assert_eq!(parser("!!"), Ok(("!!", empty)));
        assert_eq!(parser("αβγ"), Ok(("αβγ", empty))); // Unicode Greek letters
        assert_eq!(parser("你好"), Ok(("你好", empty))); // Unicode Chinese characters
    }
}#[cfg(test)]
mod tests_llm_16_313 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::character::complete::alphanumeric1;

    fn parser(input: &str) -> IResult<&str, &str> {
        alphanumeric1(input)
    }

    #[test]
    fn parse_alphanumeric() {
        assert_eq!(parser("123abcXYZ"), Ok(("", "123abcXYZ")));
        assert_eq!(parser("21cZ%1"), Ok(("%1", "21cZ")));
        assert_eq!(parser("endswithspace "), Ok((" ", "endswithspace")));
    }

    #[test]
    fn parse_non_alphanumeric_start() {
        assert_eq!(
            parser("&H2"),
            Err(Err::Error(Error::new("&H2", ErrorKind::AlphaNumeric))),
        );
        assert_eq!(
            parser("%starts%with%symbols"),
            Err(Err::Error(Error::new("%starts%with%symbols", ErrorKind::AlphaNumeric))),
        );
    }

    #[test]
    fn parse_empty() {
        assert_eq!(
            parser(""),
            Err(Err::Error(Error::new("", ErrorKind::AlphaNumeric))),
        );
    }
}#[cfg(test)]
mod tests_llm_16_314 {
    use crate::{
        character::complete::anychar,
        error::{Error, ErrorKind},
        Err, IResult,
    };

    #[test]
    fn anychar_success() {
        fn parse_anychar(input: &str) -> IResult<&str, char> {
            anychar(input)
        }

        assert_eq!(parse_anychar("abc"), Ok(("bc", 'a')));
        assert_eq!(parse_anychar("123"), Ok(("23", '1')));
        assert_eq!(parse_anychar("-?"), Ok(("?", '-')));
    }

    #[test]
    fn anychar_incomplete() {
        fn parse_anychar(input: &str) -> IResult<&str, char> {
            anychar(input)
        }

        assert_eq!(parse_anychar(""), Err(Err::Error(Error::new("", ErrorKind::Eof))));
    }

    #[test]
    fn anychar_empty_followed_by_input() {
        fn parse_anychar(input: &str) -> IResult<&str, char> {
            anychar(input)
        }

        let input = "\0abc";
        assert_eq!(parse_anychar(input), Ok(("abc", '\0')));
    }
}#[cfg(test)]
mod tests_llm_16_315 {
    use crate::{Err, error::ErrorKind, error::Error, IResult};
    use crate::character::complete::char;

    #[test]
    fn test_char_success() {
        fn parser(i: &str) -> IResult<&str, char> {
            char('a')(i)
        }

        assert_eq!(parser("abc"), Ok(("bc", 'a')));
    }

    #[test]
    fn test_char_failure_at_beginning() {
        fn parser(i: &str) -> IResult<&str, char> {
            char('a')(i)
        }

        assert_eq!(parser(" bc"), Err(Err::Error(Error::new(" bc", ErrorKind::Char))));
    }

    #[test]
    fn test_char_failure_at_middle() {
        fn parser(i: &str) -> IResult<&str, char> {
            char('a')(i)
        }

        assert_eq!(parser("bc"), Err(Err::Error(Error::new("bc", ErrorKind::Char))));
    }

    #[test]
    fn test_char_failure_empty_input() {
        fn parser(i: &str) -> IResult<&str, char> {
            char('a')(i)
        }

        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Char))));
    }
}#[cfg(test)]
mod tests_llm_16_316 {
  use super::*; // assuming `crlf` and other related types and traits are in the same module

use crate::*;
  use crate::{
    error::{Error, ErrorKind},
    Err, IResult,
  };

  #[test]
  fn test_crlf_success() {
    let input = "\r\nc";
    let expected_output: IResult<&str, &str> = Ok(("c", "\r\n"));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }

  #[test]
  fn test_crlf_incomplete() {
    let input = "ab\r\nc";
    let expected_output: IResult<&str, &str> = Err(Err::Error(Error::new(input, ErrorKind::CrLf)));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }

  #[test]
  fn test_crlf_empty_input() {
    let input = "";
    let expected_output: IResult<&str, &str> = Err(Err::Error(Error::new(input, ErrorKind::CrLf)));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }

  #[test]
  fn test_crlf_no_crlf() {
    let input = "abc";
    let expected_output: IResult<&str, &str> = Err(Err::Error(Error::new(input, ErrorKind::CrLf)));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }

  #[test]
  fn test_crlf_only_cr() {
    let input = "\rc";
    let expected_output: IResult<&str, &str> = Err(Err::Error(Error::new(input, ErrorKind::CrLf)));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }

  #[test]
  fn test_crlf_only_lf() {
    let input = "\nc";
    let expected_output: IResult<&str, &str> = Err(Err::Error(Error::new(input, ErrorKind::CrLf)));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }

  #[test]
  fn test_crlf_multiple_crlf() {
    let input = "\r\n\r\nc";
    let expected_output: IResult<&str, &str> = Ok(("\r\nc", "\r\n"));
    let result = crlf(input);
    assert_eq!(result, expected_output);
  }
}#[cfg(test)]
mod tests_llm_16_317 {
    use super::*;

use crate::*;
    use crate::{IResult, error::{Error, ErrorKind}};

    #[test]
    fn digit0_empty_input() {
        let input = "";
        let expected = Ok(("", ""));
        let result = digit0::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn digit0_only_digits() {
        let input = "123456";
        let expected = Ok(("", "123456"));
        let result = digit0::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn digit0_with_leading_non_digits() {
        let input = "abc123";
        let expected = Ok(("abc123", ""));
        let result = digit0::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn digit0_with_trailing_non_digits() {
        let input = "123abc";
        let expected = Ok(("abc", "123"));
        let result = digit0::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn digit0_with_embedded_non_digits() {
        let input = "123abc456";
        let expected = Ok(("abc456", "123"));
        let result = digit0::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn digit0_with_no_digits() {
        let input = "abc";
        let expected = Ok(("abc", ""));
        let result = digit0::<_, Error<&str>>(input);
        assert_eq!(result, expected);
    }
}
#[cfg(test)]
mod tests_llm_16_318 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::character::complete::digit1;

    #[test]
    fn digit1_valid_input() {
        fn parser(input: &str) -> IResult<&str, &str> {
            digit1(input)
        }

        let result = parser("12345");
        assert_eq!(result, Ok(("", "12345")));

        let result = parser("12345abc");
        assert_eq!(result, Ok(("abc", "12345")));

        let result = parser("0");
        assert_eq!(result, Ok(("", "0")));

        let result = parser("9abc");
        assert_eq!(result, Ok(("abc", "9")));
    }

    #[test]
    fn digit1_invalid_input() {
        fn parser(input: &str) -> IResult<&str, &str> {
            digit1(input)
        }

        let result = parser("abc");
        assert_eq!(result, Err(Err::Error(Error::new("abc", ErrorKind::Digit))));

        let result = parser("");
        assert_eq!(result, Err(Err::Error(Error::new("", ErrorKind::Digit))));

        let result = parser("abc123");
        assert_eq!(result, Err(Err::Error(Error::new("abc123", ErrorKind::Digit))));

        let result = parser(" ");
        assert_eq!(result, Err(Err::Error(Error::new(" ", ErrorKind::Digit))));
    }

    #[test]
    fn digit1_incomplete_input() {
        fn parser(input: &str) -> IResult<&str, &str> {
            digit1(input)
        }

        let result = parser("123");
        assert_eq!(result, Ok(("", "123")));
    }
}#[cfg(test)]
mod tests_llm_16_319 {
    use crate::{
        error::{Error, ErrorKind},
        IResult,
    };
    use crate::character::complete::hex_digit0;

    #[test]
    fn test_hex_digit0() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            hex_digit0(input)
        }

        let empty = "";
        let hex = "0123456789abcdefABCDEF";
        let non_hex = "gG:/";
        let mix = "01234gG:/";

        assert_eq!(parser(empty), Ok((empty, empty)));
        assert_eq!(parser(hex), Ok((empty, hex)));
        assert_eq!(parser(non_hex), Ok((non_hex, empty)));
        assert_eq!(parser(mix), Ok(("gG:/", "01234")));
    }
}#[cfg(test)]
mod tests_llm_16_320 {
  use super::*;

use crate::*;
  use crate::{
    error::{Error, ErrorKind},
    Err, IResult,
  };

  #[test]
  fn test_hex_digit1_success() {
    fn parser(input: &str) -> IResult<&str, &str> {
      hex_digit1(input)
    }

    let test_cases = vec![
      ("123abc", Ok(("abc", "123"))),
      ("0", Ok(("", "0"))),
      ("1dE", Ok(("E", "1d"))),
      ("A1B2C3", Ok(("", "A1B2C3"))),
    ];

    for (input, expected) in test_cases {
      assert_eq!(parser(input), expected);
    }
  }

  #[test]
  fn test_hex_digit1_incomplete() {
    fn parser(input: &str) -> IResult<&str, &str> {
      hex_digit1(input)
    }

    let test_cases = vec![
      ("", Err(Err::Error(Error::new("", ErrorKind::HexDigit)))),
      ("g", Err(Err::Error(Error::new("g", ErrorKind::HexDigit)))),
      ("--", Err(Err::Error(Error::new("--", ErrorKind::HexDigit)))),
    ];

    for (input, expected) in test_cases {
      assert_eq!(parser(input), expected);
    }
  }
}#[cfg(test)]
mod tests_llm_16_325_llm_16_325 {
  use super::*;

use crate::*;
  use crate::{
    error::{Error, ErrorKind},
    Err,
  };

  #[test]
  fn parse_positive_i8() {
    let res: IResult<&str, i8, Error<&str>> = i8("123");
    assert_eq!(res, Ok(("", 123)));
  }

  #[test]
  fn parse_negative_i8() {
    let res: IResult<&str, i8, Error<&str>> = i8("-123");
    assert_eq!(res, Ok(("", -123)));
  }

  #[test]
  fn parse_zero_i8() {
    let res: IResult<&str, i8, Error<&str>> = i8("0");
    assert_eq!(res, Ok(("", 0)));
  }

  #[test]
  fn parse_i8_overflow() {
    let res: IResult<&str, i8, Error<&str>> = i8("128");
    assert!(matches!(res, Err(Err::Error(Error { code: ErrorKind::Digit, .. }))));
    let res: IResult<&str, i8, Error<&str>> = i8("-129");
    assert!(matches!(res, Err(Err::Error(Error { code: ErrorKind::Digit, .. }))));
  }

  #[test]
  fn parse_i8_incomplete() {
    let res: IResult<&str, i8, Error<&str>> = i8("12a");
    assert_eq!(res, Ok(("a", 12)));
  }

  #[test]
  fn parse_i8_no_digit() {
    let res: IResult<&str, i8, Error<&str>> = i8("a123");
    assert!(matches!(res, Err(Err::Error(Error { code: ErrorKind::Digit, .. }))));
  }

  #[test]
  fn parse_i8_empty() {
    let res: IResult<&str, i8, Error<&str>> = i8("");
    assert!(matches!(res, Err(Err::Error(Error { code: ErrorKind::Digit, .. }))));
  }

  #[test]
  fn parse_i8_only_sign() {
    let res: IResult<&str, i8, Error<&str>> = i8("+");
    assert!(matches!(res, Err(Err::Error(Error { code: ErrorKind::Digit, .. }))));
    let res: IResult<&str, i8, Error<&str>> = i8("-");
    assert!(matches!(res, Err(Err::Error(Error { code: ErrorKind::Digit, .. }))));
  }
}#[cfg(test)]
mod tests_llm_16_326 {
    use crate::{
        character::complete::line_ending,
        error::{Error, ErrorKind},
        Err, IResult,
    };

    #[test]
    fn test_line_ending() {
        fn test_parser(input: &str) -> IResult<&str, &str> {
            line_ending(input)
        }

        // Test for "\n"
        assert_eq!(test_parser("\nabc"), Ok(("abc", "\n")));
        // Test for "\r\n"
        assert_eq!(test_parser("\r\ndef"), Ok(("def", "\r\n")));

        // Test incomplete input
        assert_eq!(
            test_parser(""),
            Err(Err::Error(Error::new("", ErrorKind::CrLf)))
        );

        // Test input with no line ending
        assert_eq!(
            test_parser("abc"),
            Err(Err::Error(Error::new("abc", ErrorKind::CrLf)))
        );

        // Test input with only "\r" which is not a line ending
        assert_eq!(
            test_parser("\rabc"),
            Err(Err::Error(Error::new("\rabc", ErrorKind::CrLf)))
        );

        // Test input with line ending in the middle
        assert_eq!(test_parser("abc\n"), Ok(("", "abc\n")));
    }
}#[cfg(test)]
mod tests_llm_16_327_llm_16_327 {
    use crate::{
        character::complete::multispace0,
        error::{Error, ErrorKind},
        Err, IResult,
    };

    #[test]
    fn multispace0_matches_multiple_whitespace_chars() {
        let result: IResult<&str, &str, Error<&str>> = multispace0(" \t\r\nabc");
        assert_eq!(result, Ok(("abc", " \t\r\n")));
    }

    #[test]
    fn multispace0_matches_no_whitespace_chars() {
        let result: IResult<&str, &str, Error<&str>> = multispace0("abc");
        assert_eq!(result, Ok(("abc", "")));
    }

    #[test]
    fn multispace0_matches_empty_input() {
        let result: IResult<&str, &str, Error<&str>> = multispace0("");
        assert_eq!(result, Ok(("", "")));
    }

    #[test]
    fn multispace0_error() {
        let result: IResult<&str, &str, Error<&str>> = multispace0("🚀");
        assert_eq!(
            result,
            Err(Err::Error(Error {
                input: "🚀",
                code: ErrorKind::MultiSpace
            }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_328 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::character::complete::multispace1;

    #[test]
    fn test_multispace1_success() {
        fn parser(input: &str) -> IResult<&str, &str> {
            multispace1(input)
        }

        let test_cases = vec![
            (" \t\n\r21c", Ok(("21c", " \t\n\r"))),
            ("  \t", Ok(("", "  \t"))),
            ("\n\n\nabc", Ok(("abc", "\n\n\n"))),
            ("\r\r\r123", Ok(("123", "\r\r\r"))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(parser(input), expected);
        }
    }

    #[test]
    fn test_multispace1_incomplete() {
        fn parser(input: &str) -> IResult<&str, &str> {
            multispace1(input)
        }

        let test_cases = vec![
            ("", Err(Err::Error(Error::new("", ErrorKind::MultiSpace))))
        ];

        for (input, expected) in test_cases {
            assert_eq!(parser(input), expected);
        }
    }

    #[test]
    fn test_multispace1_failure() {
        fn parser(input: &str) -> IResult<&str, &str> {
            multispace1(input)
        }

        let test_cases = vec![
            ("21c", Err(Err::Error(Error::new("21c", ErrorKind::MultiSpace)))),
            ("H2", Err(Err::Error(Error::new("H2", ErrorKind::MultiSpace)))),
            ("abc", Err(Err::Error(Error::new("abc", ErrorKind::MultiSpace)))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(parser(input), expected);
        }
    }
}#[cfg(test)]
mod tests_llm_16_329 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::character::complete::newline;

    #[test]
    fn newline_should_match() {
        fn parser(input: &str) -> IResult<&str, char> {
            newline(input)
        }

        let result = parser("\nnext");
        assert_eq!(result, Ok(("next", '\n')));
    }

    #[test]
    fn newline_should_not_match_rn() {
        fn parser(input: &str) -> IResult<&str, char> {
            newline(input)
        }

        let result = parser("\r\nnext");
        assert_eq!(result, Err(Err::Error(Error::new("\r\nnext", ErrorKind::Char))));
    }

    #[test]
    fn newline_should_not_match_empty() {
        fn parser(input: &str) -> IResult<&str, char> {
            newline(input)
        }

        let result = parser("");
        assert_eq!(result, Err(Err::Error(Error::new("", ErrorKind::Char))));
    }

    #[test]
    fn newline_should_not_match_different_char() {
        fn parser(input: &str) -> IResult<&str, char> {
            newline(input)
        }

        let result = parser("anext");
        assert_eq!(result, Err(Err::Error(Error::new("anext", ErrorKind::Char))));
    }

    #[test]
    fn newline_should_not_match_eof() {
        fn parser(input: &str) -> IResult<&str, char> {
            newline(input)
        }

        let result = parser("\n");
        assert_eq!(result, Ok(("", '\n')));
    }
}#[cfg(test)]
mod tests_llm_16_330 {
    use super::*;

use crate::*;
    use crate::{
        error::{Error, ErrorKind, ParseError},
        Err,
    };

    #[test]
    fn none_of_rejects_matching_chars() {
        let none_of_ab = none_of::<_, _, Error<&str>>("ab");

        assert_eq!(
            none_of_ab("a"),
            Err(Err::Error(Error::new("a", ErrorKind::NoneOf)))
        );
        assert_eq!(
            none_of_ab("b"),
            Err(Err::Error(Error::new("b", ErrorKind::NoneOf)))
        );
    }

    #[test]
    fn none_of_accepts_non_matching_chars() {
        let none_of_ab = none_of::<_, _, Error<&str>>("ab");

        assert_eq!(none_of_ab("c"), Ok(("", 'c')));
        assert_eq!(none_of_ab("d"), Ok(("", 'd')));
        assert_eq!(none_of_ab("z"), Ok(("", 'z')));
    }

    #[test]
    fn none_of_complete_input_fail() {
        let none_of_a = none_of::<_, _, Error<&str>>("a");

        assert_eq!(
            none_of_a(""),
            Err(Err::Error(Error::new("", ErrorKind::NoneOf)))
        );
    }

    #[test]
    fn none_of_partial_input_success() {
        let none_of_ab = none_of::<_, _, Error<&str>>("ab");

        assert_eq!(none_of_ab("cdef"), Ok(("def", 'c')));
    }

    #[test]
    fn none_of_partial_input_fail() {
        let none_of_ab = none_of::<_, _, Error<&str>>("ab");

        assert_eq!(
            none_of_ab("abcdef"),
            Err(Err::Error(Error::new("abcdef", ErrorKind::NoneOf)))
        );
    }
}#[cfg(test)]
mod tests_llm_16_331 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::character::complete::not_line_ending;

    fn parser(input: &str) -> IResult<&str, &str> {
        not_line_ending(input)
    }

    #[test]
    fn test_not_line_ending_success() {
        assert_eq!(parser("ab\r\nc"), Ok(("\r\nc", "ab")));
        assert_eq!(parser("ab\nc"), Ok(("\nc", "ab")));
        assert_eq!(parser("abc"), Ok(("", "abc")));
        assert_eq!(parser(""), Ok(("", "")));
    }

    #[test]
    fn test_not_line_ending_error() {
        assert_eq!(
            parser("a\rb\nc"),
            Err(Err::Error(Error {
                input: "a\rb\nc",
                code: ErrorKind::Tag,
            }))
        );
        assert_eq!(
            parser("a\rbc"),
            Err(Err::Error(Error {
                input: "a\rbc",
                code: ErrorKind::Tag,
            }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_333 {
  use crate::{
    error::{Error, ErrorKind},
    Err, IResult,
  };
  use crate::character::complete::oct_digit1;

  #[test]
  fn oct_digit1_valid() {
    fn parser(input: &str) -> IResult<&str, &str> {
      oct_digit1(input)
    }

    let res = parser("12345");
    assert_eq!(res, Ok(("", "12345")));

    let res = parser("01234567");
    assert_eq!(res, Ok(("", "01234567")));

    let res = parser("755abc");
    assert_eq!(res, Ok(("abc", "755")));
  }

  #[test]
  fn oct_digit1_invalid() {
    fn parser(input: &str) -> IResult<&str, &str> {
      oct_digit1(input)
    }

    let res = parser("8");
    assert_eq!(res, Err(Err::Error(Error::new("8", ErrorKind::OctDigit))));

    let res = parser("1238");
    assert_eq!(res, Err(Err::Error(Error::new("8", ErrorKind::OctDigit))));

    let res = parser(";abc");
    assert_eq!(res, Err(Err::Error(Error::new(";abc", ErrorKind::OctDigit))));
  }

  #[test]
  fn oct_digit1_incomplete() {
    fn parser(input: &str) -> IResult<&str, &str> {
      oct_digit1(input)
    }

    let res = parser("");
    assert_eq!(res, Err(Err::Error(Error::new("", ErrorKind::OctDigit))));
  }
}#[cfg(test)]
mod tests_llm_16_336_llm_16_336 {
  use crate::character::complete::sign;
  use crate::error::{Error, ErrorKind};
  use crate::error::ParseError;
  use crate::internal::Err;
  use crate::IResult;
  use crate::AsBytes;

  #[test]
  fn test_sign_positive() {
    let input = "+";
    let expected = Ok(("".as_bytes(), true));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_negative() {
    let input = "-";
    let expected = Ok(("".as_bytes(), false));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_no_sign() {
    let input = "1234";
    let expected = Ok(("1234".as_bytes(), true));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_empty() {
    let input = "";
    let expected = Ok(("".as_bytes(), true));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_unexpected_char() {
    let input = "a";
    let expected: IResult<&[u8], bool, Error<&[u8]>> = Ok(("a".as_bytes(), true));
    let result = sign(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_partial() {
    let input = "+1234";
    let expected = Ok(("1234".as_bytes(), true));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_error() {
    let input = "\0";
    let expected: IResult<&[u8], bool, Error<&[u8]>> = Ok(("\0".as_bytes(), true));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_sign_multiple() {
    let input = "++";
    let expected = Err(Err::Error(Error::from_error_kind("+".as_bytes(), ErrorKind::Tag)));
    let result = sign::<&[u8], Error<&[u8]>>(input.as_bytes());
    assert_eq!(result, expected);
  }
}#[cfg(test)]
mod tests_llm_16_337 {
    use crate::{
        character::complete::space0,
        error::{Error, ErrorKind},
        Err, IResult,
    };

    #[test]
    fn space0_empty() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let empty = "";
        assert_eq!(parser(empty), Ok((empty, empty)));
    }

    #[test]
    fn space0_space() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "    rest";
        assert_eq!(parser(input), Ok(("rest", "    ")));
    }

    #[test]
    fn space0_tab() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "\t\trest";
        assert_eq!(parser(input), Ok(("rest", "\t\t")));
    }

    #[test]
    fn space0_mixed_spaces_and_tabs() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "  \t \t  rest";
        assert_eq!(parser(input), Ok(("rest", "  \t \t  ")));
    }

    #[test]
    fn space0_no_leading_space() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "rest";
        assert_eq!(parser(input), Ok((input, "")));
    }

    #[test]
    fn space0_newline() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "\nrest";
        assert_eq!(parser(input), Ok((input, "")));
    }

    #[test]
    fn space0_end_with_space() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "  ";
        assert_eq!(parser(input), Ok(("", "  ")));
    }

    #[test]
    fn space0_end_with_tab() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = "\t\t";
        assert_eq!(parser(input), Ok(("", "\t\t")));
    }

    #[test]
    fn space0_end_with_mixed_spaces_and_tabs() {
        fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            space0(input)
        }

        let input = " \t ";
        assert_eq!(parser(input), Ok(("", " \t ")));
    }
}#[cfg(test)]
mod tests_llm_16_338 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::character::complete::space1;

    fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
        space1(input)
    }

    #[test]
    fn test_space1_success() {
        assert_eq!(parser(" \t21c"), Ok(("21c", " \t")));
        assert_eq!(parser("    21c"), Ok(("21c", "    ")));
        assert_eq!(parser("\t\t21c"), Ok(("21c", "\t\t")));
    }

    #[test]
    fn test_space1_incomplete() {
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Space))));
        assert_eq!(parser("\t"), Err(Err::Error(Error::new("", ErrorKind::Space))));
        assert_eq!(parser(" "), Err(Err::Error(Error::new("", ErrorKind::Space))));
    }

    #[test]
    fn test_space1_failure() {
        assert_eq!(parser("H2"), Err(Err::Error(Error::new("H2", ErrorKind::Space))));
        assert_eq!(parser("a "), Err(Err::Error(Error::new("a ", ErrorKind::Space))));
        assert_eq!(parser("1 "), Err(Err::Error(Error::new("1 ", ErrorKind::Space))));
    }
}#[cfg(test)]
mod tests_llm_16_340_llm_16_340 {
    use super::*;

use crate::*;
    use crate::error::Error;
    use crate::error::ErrorKind;
    use crate::error::ParseError;
    use crate::character::complete::u128;
    use crate::IResult;

    #[test]
    fn parse_u128_empty_input() {
        let input = "";
        let result: IResult<&str, u128> = u128(input);
        assert!(matches!(result, Err(Err::Error(_))));
    }

    #[test]
    fn parse_u128_valid_input() {
        let input = "123456";
        let result: IResult<&str, u128> = u128(input);
        assert_eq!(result, Ok(("", 123456u128)));
    }

    #[test]
    fn parse_u128_with_extra_chars() {
        let input = "123abc";
        let result: IResult<&str, u128> = u128(input);
        assert_eq!(result, Ok(("abc", 123u128)));
    }

    #[test]
    fn parse_u128_invalid_input() {
        let input = "abc";
        let result: IResult<&str, u128> = u128(input);
        assert!(matches!(result, Err(Err::Error(_))));
    }

    #[test]
    fn parse_u128_overflow() {
        let input = "340282366920938463463374607431768211456";
        let result: IResult<&str, u128> = u128(input);
        assert!(matches!(result, Err(Err::Error(_))));
    }

    #[test]
    fn parse_u128_leading_zeros() {
        let input = "0000123";
        let result: IResult<&str, u128> = u128(input);
        assert_eq!(result, Ok(("", 123u128)));
    }

    #[test]
    fn parse_u128_zeros_only() {
        let input = "000";
        let result: IResult<&str, u128> = u128(input);
        assert_eq!(result, Ok(("", 0u128)));
    }

    #[test]
    fn parse_u128_full() {
        let input = "340282366920938463463374607431768211455"; // max value for u128 minus 1
        let result: IResult<&str, u128> = u128(input);
        assert_eq!(result, Ok(("", u128::MAX - 1)));
    }
}#[cfg(test)]
mod tests_llm_16_342_llm_16_342 {
    use crate::{
        character::complete::u32 as parse_u32,
        error::{ErrorKind, ParseError},
        Err, IResult,
    };

    // Helper to easily create the result type
    fn to_result<'a>(input: &'a str, rem: &'a str, val: u32) -> IResult<&'a str, u32, crate::error::Error<&'a str>> {
        Ok((rem, val))
    }

    #[test]
    fn parse_u32_valid() {
        assert_eq!(parse_u32::<&str, crate::error::Error<&str>>("123"), to_result("123", "", 123));
        assert_eq!(parse_u32::<&str, crate::error::Error<&str>>("0"), to_result("0", "", 0));
        assert_eq!(parse_u32::<&str, crate::error::Error<&str>>("12345abc"), to_result("12345abc", "abc", 12345));
    }

    #[test]
    fn parse_u32_incomplete() {
        assert_eq!(
            parse_u32::<&str, crate::error::Error<&str>>(""),
            Err(Err::Error(crate::error::Error::from_error_kind("", ErrorKind::Digit)))
        );
    }

    #[test]
    fn parse_u32_invalid() {
        assert_eq!(
            parse_u32::<&str, crate::error::Error<&str>>("abc"),
            Err(Err::Error(crate::error::Error::from_error_kind("abc", ErrorKind::Digit)))
        );
        assert_eq!(
            parse_u32::<&str, crate::error::Error<&str>>("-123"),
            Err(Err::Error(crate::error::Error::from_error_kind("-123", ErrorKind::Digit)))
        );
    }

    #[test]
    fn parse_u32_overflow() {
        assert_eq!(
            parse_u32::<&str, crate::error::Error<&str>>("4294967296"), // u32::MAX + 1
            Err(Err::Error(crate::error::Error::from_error_kind("4294967296", ErrorKind::Digit)))
        );
    }
}#[cfg(test)]
mod tests_llm_16_343 {
    use crate::{
        error::{ErrorKind, ParseError},
        IResult,
    };
    use crate::character::complete::u64;

    #[derive(Debug, PartialEq)]
    struct MockError;
    impl ParseError<&str> for MockError {
        fn from_error_kind(_: &str, _: ErrorKind) -> Self {
            MockError
        }
        fn append(_: &str, _: ErrorKind, _: Self) -> Self {
            MockError
        }
    }

    #[test]
    fn test_u64_success() {
        assert_eq!(u64::<_, MockError>("12345"), Ok(("", 12345u64)));
    }

    #[test]
    fn test_u64_incomplete() {
        assert_eq!(u64::<_, MockError>(""), Err(crate::Err::Error(MockError)));
    }

    #[test]
    fn test_u64_error() {
        assert_eq!(u64::<_, MockError>("abc"), Err(crate::Err::Error(MockError)));
    }

    #[test]
    fn test_u64_overflow() {
        let input = "18446744073709551616"; // u64::MAX + 1
        assert_eq!(u64::<_, MockError>(input), Err(crate::Err::Error(MockError)));
    }
}#[cfg(test)]
mod tests_rug_91 {
    use crate::{
        character::complete::tab, error::ParseError, IResult,
    };

    #[test]
    fn test_tab() {
        let mut p0: &str = "\texample";

        let _res: IResult<&str, char> = tab(p0);
    }
}#[cfg(test)]
mod tests_rug_92 {
    use super::*;
    use crate::character::complete::oct_digit0;
    use crate::{IResult, Input};

    #[test]
    fn test_rug() {
        let mut p0: &str = "12345abcdef";

        let res: IResult<&str, &str> = oct_digit0(p0);
        assert_eq!(res, Ok(("abcdef", "12345")));
    }
}