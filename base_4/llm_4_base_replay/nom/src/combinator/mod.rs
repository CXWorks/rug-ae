//! General purpose combinators
#![allow(unused_imports)]
#[cfg(feature = "alloc")]
use crate::lib::std::boxed::Box;
use crate::error::{ErrorKind, FromExternalError, ParseError};
use crate::internal::*;
use crate::lib::std::borrow::Borrow;
use crate::lib::std::convert::Into;
#[cfg(feature = "std")]
use crate::lib::std::fmt::Debug;
use crate::lib::std::mem::transmute;
use crate::lib::std::ops::{Range, RangeFrom, RangeTo};
use crate::traits::{AsChar, Input, InputLength, ParseTo};
use crate::traits::{Compare, CompareResult, Offset};
#[cfg(test)]
mod tests;
/// Return the remaining input.
///
/// ```rust
/// # use nom::error::ErrorKind;
/// use nom::combinator::rest;
/// assert_eq!(rest::<_,(_, ErrorKind)>("abc"), Ok(("", "abc")));
/// assert_eq!(rest::<_,(_, ErrorKind)>(""), Ok(("", "")));
/// ```
#[inline]
pub fn rest<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: Input,
{
    Ok(input.take_split(input.input_len()))
}
/// Return the length of the remaining input.
///
/// ```rust
/// # use nom::error::ErrorKind;
/// use nom::combinator::rest_len;
/// assert_eq!(rest_len::<_,(_, ErrorKind)>("abc"), Ok(("abc", 3)));
/// assert_eq!(rest_len::<_,(_, ErrorKind)>(""), Ok(("", 0)));
/// ```
#[inline]
pub fn rest_len<T, E: ParseError<T>>(input: T) -> IResult<T, usize, E>
where
    T: InputLength,
{
    let len = input.input_len();
    Ok((input, len))
}
/// Maps a function on the result of a parser.
///
/// ```rust
/// use nom::{Err,error::ErrorKind, IResult,Parser};
/// use nom::character::complete::digit1;
/// use nom::combinator::map;
/// # fn main() {
///
/// let mut parser = map(digit1, |s: &str| s.len());
///
/// // the parser will count how many characters were returned by digit1
/// assert_eq!(parser.parse("123456"), Ok(("", 6)));
///
/// // this will fail if digit1 fails
/// assert_eq!(parser.parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
/// # }
/// ```
pub fn map<I, O, E, F, G>(mut parser: F, mut f: G) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, Error = E>,
    G: FnMut(<F as Parser<I>>::Output) -> O,
{
    move |input: I| {
        let (input, o1) = parser.parse(input)?;
        Ok((input, f(o1)))
    }
}
/// Applies a function returning a `Result` over the result of a parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::character::complete::digit1;
/// use nom::combinator::map_res;
/// # fn main() {
///
/// let mut parse = map_res(digit1, |s: &str| s.parse::<u8>());
///
/// // the parser will convert the result of digit1 to a number
/// assert_eq!(parse("123"), Ok(("", 123)));
///
/// // this will fail if digit1 fails
/// assert_eq!(parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
///
/// // this will fail if the mapped function fails (a `u8` is too small to hold `123456`)
/// assert_eq!(parse("123456"), Err(Err::Error(("123456", ErrorKind::MapRes))));
/// # }
/// ```
pub fn map_res<I: Clone, O, E: FromExternalError<I, E2>, E2, F, G>(
    mut parser: F,
    mut f: G,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, Error = E>,
    G: FnMut(<F as Parser<I>>::Output) -> Result<O, E2>,
{
    move |input: I| {
        let i = input.clone();
        let (input, o1) = parser.parse(input)?;
        match f(o1) {
            Ok(o2) => Ok((input, o2)),
            Err(e) => Err(Err::Error(E::from_external_error(i, ErrorKind::MapRes, e))),
        }
    }
}
/// Applies a function returning an `Option` over the result of a parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::character::complete::digit1;
/// use nom::combinator::map_opt;
/// # fn main() {
///
/// let mut parse = map_opt(digit1, |s: &str| s.parse::<u8>().ok());
///
/// // the parser will convert the result of digit1 to a number
/// assert_eq!(parse("123"), Ok(("", 123)));
///
/// // this will fail if digit1 fails
/// assert_eq!(parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
///
/// // this will fail if the mapped function fails (a `u8` is too small to hold `123456`)
/// assert_eq!(parse("123456"), Err(Err::Error(("123456", ErrorKind::MapOpt))));
/// # }
/// ```
pub fn map_opt<I: Clone, O, E: ParseError<I>, F, G>(
    mut parser: F,
    mut f: G,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, Error = E>,
    G: FnMut(<F as Parser<I>>::Output) -> Option<O>,
{
    move |input: I| {
        let i = input.clone();
        let (input, o1) = parser.parse(input)?;
        match f(o1) {
            Some(o2) => Ok((input, o2)),
            None => Err(Err::Error(E::from_error_kind(i, ErrorKind::MapOpt))),
        }
    }
}
/// Applies a parser over the result of another one.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::character::complete::digit1;
/// use nom::bytes::complete::take;
/// use nom::combinator::map_parser;
/// # fn main() {
///
/// let mut parse = map_parser(take(5u8), digit1);
///
/// assert_eq!(parse("12345"), Ok(("", "12345")));
/// assert_eq!(parse("123ab"), Ok(("", "123")));
/// assert_eq!(parse("123"), Err(Err::Error(("123", ErrorKind::Eof))));
/// # }
/// ```
pub fn map_parser<I, O, E: ParseError<I>, F, G>(
    mut parser: F,
    mut applied_parser: G,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, Error = E>,
    G: Parser<<F as Parser<I>>::Output, Output = O, Error = E>,
{
    move |input: I| {
        let (input, o1) = parser.parse(input)?;
        let (_, o2) = applied_parser.parse(o1)?;
        Ok((input, o2))
    }
}
/// Creates a new parser from the output of the first parser, then apply that parser over the rest of the input.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::bytes::complete::take;
/// use nom::number::complete::u8;
/// use nom::combinator::flat_map;
/// # fn main() {
///
/// let mut parse = flat_map(u8, take);
///
/// assert_eq!(parse(&[2, 0, 1, 2][..]), Ok((&[2][..], &[0, 1][..])));
/// assert_eq!(parse(&[4, 0, 1, 2][..]), Err(Err::Error((&[0, 1, 2][..], ErrorKind::Eof))));
/// # }
/// ```
pub fn flat_map<I, O, E: ParseError<I>, F, G, H>(
    mut parser: F,
    mut applied_parser: G,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, Error = E>,
    G: FnMut(<F as Parser<I>>::Output) -> H,
    H: Parser<I, Output = O, Error = E>,
{
    move |input: I| {
        let (input, o1) = parser.parse(input)?;
        applied_parser(o1).parse(input)
    }
}
/// Optional parser, will return `None` on [`Err::Error`].
///
/// To chain an error up, see [`cut`].
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::opt;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// fn parser(i: &str) -> IResult<&str, Option<&str>> {
///   opt(alpha1)(i)
/// }
///
/// assert_eq!(parser("abcd;"), Ok((";", Some("abcd"))));
/// assert_eq!(parser("123;"), Ok(("123;", None)));
/// # }
/// ```
pub fn opt<I: Clone, E: ParseError<I>, F>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Option<<F as Parser<I>>::Output>, E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| {
        let i = input.clone();
        match f.parse(input) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(Err::Error(_)) => Ok((i, None)),
            Err(e) => Err(e),
        }
    }
}
/// Calls the parser if the condition is met.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, IResult};
/// use nom::combinator::cond;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {
///   cond(b, alpha1)(i)
/// }
///
/// assert_eq!(parser(true, "abcd;"), Ok((";", Some("abcd"))));
/// assert_eq!(parser(false, "abcd;"), Ok(("abcd;", None)));
/// assert_eq!(parser(true, "123;"), Err(Err::Error(Error::new("123;", ErrorKind::Alpha))));
/// assert_eq!(parser(false, "123;"), Ok(("123;", None)));
/// # }
/// ```
pub fn cond<I, E: ParseError<I>, F>(
    b: bool,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Option<<F as Parser<I>>::Output>, E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| {
        if b {
            match f.parse(input) {
                Ok((i, o)) => Ok((i, Some(o))),
                Err(e) => Err(e),
            }
        } else {
            Ok((input, None))
        }
    }
}
/// Tries to apply its parser without consuming the input.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::peek;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = peek(alpha1);
///
/// assert_eq!(parser("abcd;"), Ok(("abcd;", "abcd")));
/// assert_eq!(parser("123;"), Err(Err::Error(("123;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn peek<I: Clone, E: ParseError<I>, F>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, <F as Parser<I>>::Output, E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| {
        let i = input.clone();
        match f.parse(input) {
            Ok((_, o)) => Ok((i, o)),
            Err(e) => Err(e),
        }
    }
}
/// returns its input if it is at the end of input data
///
/// When we're at the end of the data, this combinator
/// will succeed
///
/// ```
/// # use std::str;
/// # use nom::{Err, error::ErrorKind, IResult};
/// # use nom::combinator::eof;
///
/// # fn main() {
/// let parser = eof;
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Eof))));
/// assert_eq!(parser(""), Ok(("", "")));
/// # }
/// ```
pub fn eof<I: InputLength + Clone, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
    if input.input_len() == 0 {
        let clone = input.clone();
        Ok((input, clone))
    } else {
        Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
    }
}
/// Transforms Incomplete into `Error`.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::bytes::streaming::take;
/// use nom::combinator::complete;
/// # fn main() {
///
/// let mut parser = complete(take(5u8));
///
/// assert_eq!(parser("abcdefg"), Ok(("fg", "abcde")));
/// assert_eq!(parser("abcd"), Err(Err::Error(("abcd", ErrorKind::Complete))));
/// # }
/// ```
pub fn complete<I: Clone, O, E: ParseError<I>, F>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, Output = O, Error = E>,
{
    move |input: I| {
        let i = input.clone();
        match f.parse(input) {
            Err(Err::Incomplete(_)) => {
                Err(Err::Error(E::from_error_kind(i, ErrorKind::Complete)))
            }
            rest => rest,
        }
    }
}
/// Succeeds if all the input has been consumed by its child parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::all_consuming;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = all_consuming(alpha1);
///
/// assert_eq!(parser("abcd"), Ok(("", "abcd")));
/// assert_eq!(parser("abcd;"),Err(Err::Error((";", ErrorKind::Eof))));
/// assert_eq!(parser("123abcd;"),Err(Err::Error(("123abcd;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn all_consuming<I, E: ParseError<I>, F>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, <F as Parser<I>>::Output, E>
where
    I: InputLength,
    F: Parser<I, Error = E>,
{
    move |input: I| {
        let (input, res) = f.parse(input)?;
        if input.input_len() == 0 {
            Ok((input, res))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
        }
    }
}
/// Returns the result of the child parser if it satisfies a verification function.
///
/// The verification function takes as argument a reference to the output of the
/// parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::verify;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = verify(alpha1, |s: &str| s.len() == 4);
///
/// assert_eq!(parser("abcd"), Ok(("", "abcd")));
/// assert_eq!(parser("abcde"), Err(Err::Error(("abcde", ErrorKind::Verify))));
/// assert_eq!(parser("123abcd;"),Err(Err::Error(("123abcd;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn verify<I: Clone, O2, E: ParseError<I>, F, G>(
    mut first: F,
    second: G,
) -> impl FnMut(I) -> IResult<I, <F as Parser<I>>::Output, E>
where
    F: Parser<I, Error = E>,
    G: Fn(&O2) -> bool,
    <F as Parser<I>>::Output: Borrow<O2>,
    O2: ?Sized,
{
    move |input: I| {
        let i = input.clone();
        let (input, o) = first.parse(input)?;
        if second(o.borrow()) {
            Ok((input, o))
        } else {
            Err(Err::Error(E::from_error_kind(i, ErrorKind::Verify)))
        }
    }
}
/// Returns the provided value if the child parser succeeds.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::value;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = value(1234, alpha1);
///
/// assert_eq!(parser("abcd"), Ok(("", 1234)));
/// assert_eq!(parser("123abcd;"), Err(Err::Error(("123abcd;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn value<I, O1: Clone, E: ParseError<I>, F>(
    val: O1,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O1, E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| parser.parse(input).map(|(i, _)| (i, val.clone()))
}
/// Succeeds if the child parser returns an error.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::not;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = not(alpha1);
///
/// assert_eq!(parser("123"), Ok(("123", ())));
/// assert_eq!(parser("abcd"), Err(Err::Error(("abcd", ErrorKind::Not))));
/// # }
/// ```
pub fn not<I: Clone, E: ParseError<I>, F>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, (), E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| {
        let i = input.clone();
        match parser.parse(input) {
            Ok(_) => Err(Err::Error(E::from_error_kind(i, ErrorKind::Not))),
            Err(Err::Error(_)) => Ok((i, ())),
            Err(e) => Err(e),
        }
    }
}
/// If the child parser was successful, return the consumed input as produced value.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::recognize;
/// use nom::character::complete::{char, alpha1};
/// use nom::sequence::separated_pair;
/// # fn main() {
///
/// let mut parser = recognize(separated_pair(alpha1, char(','), alpha1));
///
/// assert_eq!(parser("abcd,efgh"), Ok(("", "abcd,efgh")));
/// assert_eq!(parser("abcd;"),Err(Err::Error((";", ErrorKind::Char))));
/// # }
/// ```
pub fn recognize<I: Clone + Offset + Input, E: ParseError<I>, F>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, I, E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| {
        let i = input.clone();
        match parser.parse(i) {
            Ok((i, _)) => {
                let index = input.offset(&i);
                Ok((i, input.take(index)))
            }
            Err(e) => Err(e),
        }
    }
}
/// if the child parser was successful, return the consumed input with the output
/// as a tuple. Functions similarly to [recognize](fn.recognize.html) except it
/// returns the parser output as well.
///
/// This can be useful especially in cases where the output is not the same type
/// as the input, or the input is a user defined type.
///
/// Returned tuple is of the format `(consumed input, produced output)`.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::{consumed, value, recognize, map};
/// use nom::character::complete::{char, alpha1};
/// use nom::bytes::complete::tag;
/// use nom::sequence::separated_pair;
///
/// fn inner_parser(input: &str) -> IResult<&str, bool> {
///     value(true, tag("1234"))(input)
/// }
///
/// # fn main() {
///
/// let mut consumed_parser = consumed(value(true, separated_pair(alpha1, char(','), alpha1)));
///
/// assert_eq!(consumed_parser("abcd,efgh1"), Ok(("1", ("abcd,efgh", true))));
/// assert_eq!(consumed_parser("abcd;"),Err(Err::Error((";", ErrorKind::Char))));
///
///
/// // the first output (representing the consumed input)
/// // should be the same as that of the `recognize` parser.
/// let mut recognize_parser = recognize(inner_parser);
/// let mut consumed_parser = map(consumed(inner_parser), |(consumed, output)| consumed);
///
/// assert_eq!(recognize_parser("1234"), consumed_parser("1234"));
/// assert_eq!(recognize_parser("abcd"), consumed_parser("abcd"));
/// # }
/// ```
pub fn consumed<I, F, E>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, (I, <F as Parser<I>>::Output), E>
where
    I: Clone + Offset + Input,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    move |input: I| {
        let i = input.clone();
        match parser.parse(i) {
            Ok((remaining, result)) => {
                let index = input.offset(&remaining);
                let consumed = input.take(index);
                Ok((remaining, (consumed, result)))
            }
            Err(e) => Err(e),
        }
    }
}
/// Transforms an [`Err::Error`] (recoverable) to [`Err::Failure`] (unrecoverable)
///
/// This commits the parse result, preventing alternative branch paths like with
/// [`nom::branch::alt`][crate::branch::alt].
///
/// # Example
///
/// Without `cut`:
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// # use nom::character::complete::{one_of, digit1};
/// # use nom::combinator::rest;
/// # use nom::branch::alt;
/// # use nom::sequence::preceded;
/// # fn main() {
///
/// fn parser(input: &str) -> IResult<&str, &str> {
///   alt((
///     preceded(one_of("+-"), digit1),
///     rest
///   ))(input)
/// }
///
/// assert_eq!(parser("+10 ab"), Ok((" ab", "10")));
/// assert_eq!(parser("ab"), Ok(("", "ab")));
/// assert_eq!(parser("+"), Ok(("", "+")));
/// # }
/// ```
///
/// With `cut`:
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, error::Error};
/// # use nom::character::complete::{one_of, digit1};
/// # use nom::combinator::rest;
/// # use nom::branch::alt;
/// # use nom::sequence::preceded;
/// use nom::combinator::cut;
/// # fn main() {
///
/// fn parser(input: &str) -> IResult<&str, &str> {
///   alt((
///     preceded(one_of("+-"), cut(digit1)),
///     rest
///   ))(input)
/// }
///
/// assert_eq!(parser("+10 ab"), Ok((" ab", "10")));
/// assert_eq!(parser("ab"), Ok(("", "ab")));
/// assert_eq!(parser("+"), Err(Err::Failure(Error { input: "", code: ErrorKind::Digit })));
/// # }
/// ```
pub fn cut<I, E: ParseError<I>, F>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, <F as Parser<I>>::Output, E>
where
    F: Parser<I, Error = E>,
{
    move |input: I| match parser.parse(input) {
        Err(Err::Error(e)) => Err(Err::Failure(e)),
        rest => rest,
    }
}
/// automatically converts the child parser's result to another type
///
/// it will be able to convert the output value and the error value
/// as long as the `Into` implementations are available
///
/// ```rust
/// # use nom::IResult;
/// use nom::combinator::into;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
///  fn parser1(i: &str) -> IResult<&str, &str> {
///    alpha1(i)
///  }
///
///  let mut parser2 = into(parser1);
///
/// // the parser converts the &str output of the child parser into a Vec<u8>
/// let bytes: IResult<&str, Vec<u8>> = parser2("abcd");
/// assert_eq!(bytes, Ok(("", vec![97, 98, 99, 100])));
/// # }
/// ```
pub fn into<I, O1, O2, E1, E2, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, O2, E2>
where
    O1: Into<O2>,
    E1: Into<E2>,
    E1: ParseError<I>,
    E2: ParseError<I>,
    F: Parser<I, Output = O1, Error = E1>,
{
    move |input: I| match parser.parse(input) {
        Ok((i, o)) => Ok((i, o.into())),
        Err(Err::Error(e)) => Err(Err::Error(e.into())),
        Err(Err::Failure(e)) => Err(Err::Failure(e.into())),
        Err(Err::Incomplete(e)) => Err(Err::Incomplete(e)),
    }
}
/// Creates an iterator from input data and a parser.
///
/// Call the iterator's [ParserIterator::finish] method to get the remaining input if successful,
/// or the error value if we encountered an error.
///
/// On [`Err::Error`], iteration will stop. To instead chain an error up, see [`cut`].
///
/// ```rust
/// use nom::{combinator::iterator, IResult, bytes::complete::tag, character::complete::alpha1, sequence::terminated};
/// use std::collections::HashMap;
///
/// let data = "abc|defg|hijkl|mnopqr|123";
/// let mut it = iterator(data, terminated(alpha1, tag("|")));
///
/// let parsed = it.map(|v| (v, v.len())).collect::<HashMap<_,_>>();
/// let res: IResult<_,_> = it.finish();
///
/// assert_eq!(parsed, [("abc", 3usize), ("defg", 4), ("hijkl", 5), ("mnopqr", 6)].iter().cloned().collect());
/// assert_eq!(res, Ok(("123", ())));
/// ```
pub fn iterator<Input, Error, F>(input: Input, f: F) -> ParserIterator<Input, Error, F>
where
    F: Parser<Input>,
    Error: ParseError<Input>,
{
    ParserIterator {
        iterator: f,
        input,
        state: Some(State::Running),
    }
}
/// Main structure associated to the [iterator] function.
pub struct ParserIterator<I, E, F> {
    iterator: F,
    input: I,
    state: Option<State<E>>,
}
impl<I: Clone, E, F> ParserIterator<I, E, F> {
    /// Returns the remaining input if parsing was successful, or the error if we encountered an error.
    pub fn finish(mut self) -> IResult<I, (), E> {
        match self.state.take().unwrap() {
            State::Running | State::Done => Ok((self.input, ())),
            State::Failure(e) => Err(Err::Failure(e)),
            State::Incomplete(i) => Err(Err::Incomplete(i)),
        }
    }
}
impl<'a, Input, Output, Error, F> core::iter::Iterator
for &'a mut ParserIterator<Input, Error, F>
where
    F: FnMut(Input) -> IResult<Input, Output, Error>,
    Input: Clone,
{
    type Item = Output;
    fn next(&mut self) -> Option<Self::Item> {
        if let State::Running = self.state.take().unwrap() {
            let input = self.input.clone();
            match (self.iterator)(input) {
                Ok((i, o)) => {
                    self.input = i;
                    self.state = Some(State::Running);
                    Some(o)
                }
                Err(Err::Error(_)) => {
                    self.state = Some(State::Done);
                    None
                }
                Err(Err::Failure(e)) => {
                    self.state = Some(State::Failure(e));
                    None
                }
                Err(Err::Incomplete(i)) => {
                    self.state = Some(State::Incomplete(i));
                    None
                }
            }
        } else {
            None
        }
    }
}
enum State<E> {
    Running,
    Done,
    Failure(E),
    Incomplete(Needed),
}
/// a parser which always succeeds with given value without consuming any input.
///
/// It can be used for example as the last alternative in `alt` to
/// specify the default case.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::branch::alt;
/// use nom::combinator::{success, value};
/// use nom::character::complete::char;
/// # fn main() {
///
/// let mut parser = success::<_,_,(_,ErrorKind)>(10);
/// assert_eq!(parser("xyz"), Ok(("xyz", 10)));
///
/// let mut sign = alt((value(-1, char('-')), value(1, char('+')), success::<_,_,(_,ErrorKind)>(1)));
/// assert_eq!(sign("+10"), Ok(("10", 1)));
/// assert_eq!(sign("-10"), Ok(("10", -1)));
/// assert_eq!(sign("10"), Ok(("10", 1)));
/// # }
/// ```
pub fn success<I, O: Clone, E: ParseError<I>>(val: O) -> impl Fn(I) -> IResult<I, O, E> {
    move |input: I| Ok((input, val.clone()))
}
/// A parser which always fails.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, IResult};
/// use nom::combinator::fail;
///
/// let s = "string";
/// assert_eq!(fail::<_, &str, _>(s), Err(Err::Error((s, ErrorKind::Fail))));
/// ```
pub fn fail<I, O, E: ParseError<I>>(i: I) -> IResult<I, O, E> {
    Err(Err::Error(E::from_error_kind(i, ErrorKind::Fail)))
}
#[cfg(test)]
mod tests_llm_16_389 {
    use super::*;
    use crate::*;
    use crate::{
        error::{Error, ErrorKind, ParseError},
        Err, IResult,
    };
    use crate::bytes::streaming::take;
    use crate::combinator::complete;
    fn take_5(input: &str) -> IResult<&str, &str, Error<&str>> {
        take(5u8)(input)
    }
    #[test]
    fn test_complete_success() {
        let mut parser = complete(take_5);
        let result = parser("abcdefg");
        assert_eq!(result, Ok(("fg", "abcde")));
    }
    #[test]
    fn test_complete_error() {
        let mut parser = complete(take_5);
        let result = parser("abcd");
        assert_eq!(result, Err(Err::Error(Error::new("abcd", ErrorKind::Complete))));
    }
}
#[cfg(test)]
mod tests_llm_16_390 {
    use crate::{
        character::complete::alpha1, combinator::cond, error::{Error, ErrorKind},
        Err, IResult,
    };
    #[test]
    fn test_cond_true() {
        fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {
            cond(b, alpha1)(i)
        }
        assert_eq!(parser(true, "abcd;"), Ok((";", Some("abcd"))));
    }
    #[test]
    fn test_cond_false() {
        fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {
            cond(b, alpha1)(i)
        }
        assert_eq!(parser(false, "abcd;"), Ok(("abcd;", None)));
    }
    #[test]
    fn test_cond_true_err() {
        fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {
            cond(b, alpha1)(i)
        }
        assert_eq!(
            parser(true, "123;"), Err(Err::Error(Error::new("123;", ErrorKind::Alpha)))
        );
    }
    #[test]
    fn test_cond_false_noop() {
        fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {
            cond(b, alpha1)(i)
        }
        assert_eq!(parser(false, "123;"), Ok(("123;", None)));
    }
}
#[cfg(test)]
mod tests_llm_16_391 {
    use crate::{
        combinator::consumed, error::ErrorKind, error::ParseError, IResult, Parser,
        sequence::tuple,
    };
    /// Parser that consumes an "a" character and then digits, returning the digits as an integer.
    fn a_followed_by_digits(
        input: &str,
    ) -> IResult<&str, u32, crate::error::Error<&str>> {
        let (input, (a, digits)) = tuple((
            crate::character::complete::char('a'),
            crate::character::complete::digit1,
        ))(input)?;
        let number = digits.parse::<u32>().unwrap();
        Ok((input, number))
    }
    #[test]
    fn consumed_successful() {
        let mut parser = consumed(a_followed_by_digits);
        assert_eq!(parser("a123test"), Ok(("test", ("a123", 123))));
    }
    #[test]
    fn consumed_incomplete() {
        let mut parser = consumed(a_followed_by_digits);
        assert_eq!(
            parser("a"), Err(crate ::Err::Error(crate ::error::Error::new("a",
            ErrorKind::Digit)))
        );
    }
    #[test]
    fn consumed_error() {
        let mut parser = consumed(a_followed_by_digits);
        assert_eq!(
            parser("test"), Err(crate ::Err::Error(crate ::error::Error::new("test",
            ErrorKind::Char)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_392_llm_16_392 {
    use crate::combinator::cut;
    use crate::character::complete::digit1;
    use crate::branch::alt;
    use crate::sequence::preceded;
    use crate::bytes::complete::take_while1;
    use crate::error::{Error, ErrorKind};
    use crate::{Err, IResult};
    /// Dummy parser that succeeds only if the input starts with 'a'
    fn test_parser(input: &str) -> IResult<&str, &str, Error<&str>> {
        if input.chars().next() == Some('a') {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(Err::Error(Error::new(input, ErrorKind::Char)))
        }
    }
    #[test]
    fn test_cut_success() {
        let mut parser = preceded(test_parser, cut(digit1));
        let input = "a123";
        let result = parser(input);
        assert_eq!(result, Ok(("", "123")));
    }
    #[test]
    fn test_cut_failure_before_cut() {
        let mut parser = preceded(test_parser, cut(digit1));
        let input = "b123";
        let result = parser(input);
        assert!(matches!(result, Err(Err::Error(_))));
    }
    #[test]
    fn test_cut_failure_after_cut() {
        let mut parser = preceded(test_parser, cut(digit1));
        let input = "aabc";
        let result = parser(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }
    #[test]
    fn test_cut_with_alt() {
        let mut parser = alt((
            preceded(test_parser, cut(digit1)),
            take_while1(|c: char| c.is_alphabetic()),
        ));
        let input = "a123";
        let alt_success = parser(input);
        assert_eq!(alt_success, Ok(("", "123")));
        let input = "abc";
        let alt_failure = parser(input);
        assert_eq!(alt_failure, Ok(("bc", "a")));
        let input = "a";
        let alt_cut_failure = parser(input);
        assert!(matches!(alt_cut_failure, Err(Err::Failure(_))));
    }
}
#[cfg(test)]
mod tests_llm_16_393 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::combinator::eof;
    #[test]
    fn test_eof() {
        fn test_parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            eof(input)
        }
        assert_eq!(
            test_parser("abc"), Err(Err::Error(Error::new("abc", ErrorKind::Eof)))
        );
        assert_eq!(test_parser(""), Ok(("", "")));
    }
}
#[cfg(test)]
mod tests_llm_16_394 {
    use crate::{
        error::{ErrorKind, ParseError},
        Err, IResult,
    };
    use crate::combinator::fail;
    #[derive(Debug, PartialEq)]
    struct CustomError<'a>(&'a str, ErrorKind);
    impl<'a> ParseError<&'a str> for CustomError<'a> {
        fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
            CustomError(input, kind)
        }
        fn append(input: &'a str, kind: ErrorKind, _other: Self) -> Self {
            CustomError(input, kind)
        }
    }
    #[test]
    fn test_fail_always_fails() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let res: IResult<&str, &str, CustomError> = fail(input);
        debug_assert_eq!(res, Err(Err::Error(CustomError(input, ErrorKind::Fail))));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_396 {
    use crate::{
        IResult, combinator::into, character::complete::alpha1, error::{Error, ErrorKind},
    };
    #[test]
    fn test_into_success() {
        fn parser1(i: &str) -> IResult<&str, &str> {
            alpha1(i)
        }
        let mut parser2 = into(parser1);
        let result: IResult<&str, Vec<u8>> = parser2("abcd");
        assert_eq!(result, Ok(("", vec![97, 98, 99, 100])));
    }
    #[test]
    fn test_into_failure() {
        fn parser1(i: &str) -> IResult<&str, &str, Error<&str>> {
            alpha1(i)
        }
        let mut parser2 = into(parser1);
        let result: IResult<&str, Vec<u8>, Error<&str>> = parser2("1234");
        assert!(result.is_err());
        if let Err(crate::Err::Error(Error { input, code })) = result {
            assert_eq!(input, "1234");
            assert_eq!(code, ErrorKind::Alpha);
        } else {
            panic!("Error expected");
        }
    }
}
#[cfg(test)]
mod tests_llm_16_398 {
    use crate::{
        combinator::map, error::{ErrorKind, ParseError},
        Err, IResult, Parser,
    };
    use crate::character::complete::digit1;
    #[test]
    fn test_map() {
        fn parse_digits_to_length(s: &str) -> IResult<&str, usize> {
            map(digit1, |s: &str| s.len()).parse(s)
        }
        assert_eq!(parse_digits_to_length("123456"), Ok(("", 6)));
        assert_eq!(
            parse_digits_to_length("abc"),
            Err(Err::Error(ParseError::from_error_kind("abc", ErrorKind::Digit)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_400_llm_16_400 {
    use crate::{
        bytes::complete::take, character::complete::digit1, combinator::map_parser,
        error::{Error, ErrorKind, ParseError},
        Err, IResult, Parser,
    };
    #[test]
    fn test_map_parser_success_complete() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parse = map_parser(take::<_, _, Error<&str>>(rug_fuzz_0), digit1);
        debug_assert_eq!(parse(rug_fuzz_1), Ok(("", "12345")));
             }
}
}
}    }
    #[test]
    fn test_map_parser_success_partial() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parse = map_parser(take::<_, _, Error<&str>>(rug_fuzz_0), digit1);
        debug_assert_eq!(parse(rug_fuzz_1), Ok(("ab", "123")));
             }
}
}
}    }
    #[test]
    fn test_map_parser_incomplete() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parse = map_parser(take::<_, _, Error<&str>>(rug_fuzz_0), digit1);
        let result: IResult<&str, &str, Error<&str>> = parse(rug_fuzz_1);
        debug_assert_eq!(result, Err(Err::Error(Error::new("123", ErrorKind::Eof))));
             }
}
}
}    }
    #[test]
    fn test_map_parser_no_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parse = map_parser(take::<_, _, Error<&str>>(rug_fuzz_0), digit1);
        let result: IResult<&str, &str, Error<&str>> = parse(rug_fuzz_1);
        debug_assert!(matches!(result, Err(Err::Error(_))));
             }
}
}
}    }
    #[test]
    fn test_map_parser_empty_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parse = map_parser(take::<_, _, Error<&str>>(rug_fuzz_0), digit1);
        let result: IResult<&str, &str, Error<&str>> = parse(rug_fuzz_1);
        debug_assert_eq!(result, Err(Err::Error(Error::new("", ErrorKind::Eof))));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_401 {
    use crate::{
        combinator::map_res, character::complete::digit1,
        error::{Error, ErrorKind, ParseError, FromExternalError},
        Err, IResult,
    };
    fn parse_u8(input: &str) -> IResult<&str, u8, Error<&str>> {
        map_res(digit1, |s: &str| s.parse::<u8>())(input)
    }
    #[test]
    fn test_map_res_success() {
        assert_eq!(parse_u8("123"), Ok(("", 123)));
    }
    #[test]
    fn test_map_res_incomplete() {
        assert_eq!(parse_u8("123abc"), Ok(("abc", 123)));
    }
    #[test]
    fn test_map_res_error_digit() {
        assert_eq!(
            parse_u8("abc"), Err(Err::Error(Error::new("abc", ErrorKind::Digit)))
        );
    }
    #[test]
    fn test_map_res_error_map_res() {
        assert_eq!(
            parse_u8("123456"), Err(Err::Error(Error::new("123456", ErrorKind::MapRes)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_402_llm_16_402 {
    use crate::{
        combinator::not, character::complete::alpha1,
        error::{Error, ErrorKind, ParseError},
        Err, IResult,
    };
    #[test]
    fn not_parser_succeeds_when_child_fails() {
        let mut parser = not::<_, Error<&str>, _>(alpha1);
        assert_eq!(parser("123"), Ok(("123", ())));
    }
    #[test]
    fn not_parser_fails_when_child_succeeds() {
        let mut parser = not::<_, Error<&str>, _>(alpha1);
        assert_eq!(parser("abcd"), Err(Err::Error(Error::new("abcd", ErrorKind::Not))));
    }
    #[test]
    fn not_parser_propagates_fatal_errors() {
        fn fatal_error_parser(input: &str) -> IResult<&str, &str, Error<&str>> {
            Err(Err::Failure(Error::new(input, ErrorKind::Alpha)))
        }
        let mut parser = not::<_, Error<&str>, _>(fatal_error_parser);
        assert_eq!(
            parser("abcd"), Err(Err::Failure(Error::new("abcd", ErrorKind::Alpha)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_403 {
    use crate::{
        IResult, combinator::opt, character::complete::alpha1, error::Error,
        error::ErrorKind,
    };
    fn parser(i: &str) -> IResult<&str, Option<&str>, Error<&str>> {
        opt(alpha1)(i)
    }
    #[test]
    fn test_opt_parser_matches_alpha() {
        assert_eq!(parser("abcd;"), Ok((";", Some("abcd"))));
    }
    #[test]
    fn test_opt_parser_none_on_non_alpha() {
        assert_eq!(parser("123;"), Ok(("123;", None)));
    }
    #[test]
    fn test_opt_parser_incomplete() {
        assert_eq!(parser(""), Err(crate ::Err::Error(Error::new("", ErrorKind::Eof))));
    }
    #[test]
    fn test_opt_parser_error_propagation() {
        assert!(
            matches!(parser("🚀;"), Err(crate ::Err::Error(Error { input, .. })) if
            input == "🚀;")
        );
    }
}
#[cfg(test)]
mod tests_llm_16_404 {
    use crate::{
        error::{Error, ErrorKind},
        character::complete::alpha1, combinator::peek, Err, IResult,
    };
    #[test]
    fn peek_success() {
        fn peek_alpha(input: &str) -> IResult<&str, &str, Error<&str>> {
            peek(alpha1)(input)
        }
        assert_eq!(peek_alpha("abcd;"), Ok(("abcd;", "abcd")));
    }
    #[test]
    fn peek_failure() {
        fn peek_alpha(input: &str) -> IResult<&str, &str, Error<&str>> {
            peek(alpha1)(input)
        }
        assert_eq!(
            peek_alpha("123;"), Err(Err::Error(Error::new("123;", ErrorKind::Alpha)))
        );
    }
    #[test]
    fn peek_incomplete() {
        fn peek_alpha(input: &str) -> IResult<&str, &str, Error<&str>> {
            peek(alpha1)(input)
        }
        assert!(peek_alpha("").is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_407_llm_16_407 {
    use crate::{
        combinator::rest_len, error::{Error, ParseError},
        traits::InputLength, IResult,
    };
    #[test]
    fn test_rest_len_non_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let len = input.len();
        debug_assert_eq!(rest_len:: < _, Error < & str > > (input), Ok((input, len)));
             }
}
}
}    }
    #[test]
    fn test_rest_len_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let len = input.len();
        debug_assert_eq!(rest_len:: < _, Error < & str > > (input), Ok((input, len)));
             }
}
}
}    }
    #[test]
    fn test_rest_len_unicode() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let len = input.len();
        debug_assert_eq!(rest_len:: < _, Error < & str > > (input), Ok((input, len)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_408 {
    use crate::{
        error::{ErrorKind, ParseError},
        IResult, combinator::success,
    };
    #[derive(Debug, Clone, PartialEq)]
    struct CustomError<I> {
        input: I,
        code: ErrorKind,
    }
    impl<I> ParseError<I> for CustomError<I> {
        fn from_error_kind(input: I, kind: ErrorKind) -> Self {
            CustomError { input, code: kind }
        }
        fn append(input: I, kind: ErrorKind, _other: Self) -> Self {
            CustomError { input, code: kind }
        }
    }
    #[test]
    fn success_parser_always_succeeds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let parser = success::<&str, _, CustomError<&str>>(rug_fuzz_0);
        let result: IResult<&str, _, CustomError<&str>> = parser(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("input", 42)));
             }
}
}
}    }
    #[test]
    fn success_parser_works_with_empty_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let parser = success::<&str, _, CustomError<&str>>(rug_fuzz_0);
        let result: IResult<&str, _, CustomError<&str>> = parser(rug_fuzz_1);
        debug_assert_eq!(result, Ok(("", "success")));
             }
}
}
}    }
    #[test]
    fn success_parser_works_with_non_str_input() {
        let _rug_st_tests_llm_16_408_rrrruuuugggg_success_parser_works_with_non_str_input = 0;
        let rug_fuzz_0 = b's';
        let rug_fuzz_1 = b"input";
        let parser = success::<&[u8], _, CustomError<&[u8]>>(rug_fuzz_0);
        let result: IResult<&[u8], _, CustomError<&[u8]>> = parser(rug_fuzz_1);
        debug_assert_eq!(result, Ok((b"input" as & [u8], b's')));
        let _rug_ed_tests_llm_16_408_rrrruuuugggg_success_parser_works_with_non_str_input = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_409 {
    use super::*;
    use crate::*;
    use crate::{
        error::{ErrorKind, ParseError},
        IResult,
    };
    fn dummy_parser(input: &str) -> IResult<&str, &str> {
        if input.starts_with("abc") {
            Ok((&input[3..], &input[..3]))
        } else {
            Err(crate::Err::Error(crate::error::Error::new(input, ErrorKind::Alpha)))
        }
    }
    #[test]
    fn test_value_success() {
        let mut parser = value("Result", dummy_parser);
        let input = "abcde";
        let expected = Ok(("de", "Result"));
        assert_eq!(parser(input), expected);
    }
    #[test]
    fn test_value_failure() {
        let mut parser = value("Result", dummy_parser);
        let input = "123abc";
        assert!(parser(input).is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_410 {
    use crate::{
        combinator::verify, error::ErrorKind, error::ParseError,
        character::complete::alpha1, IResult,
    };
    #[test]
    fn verify_length() {
        fn parser_length(input: &str) -> IResult<&str, &str> {
            verify(alpha1, |s: &str| s.len() == 4)(input)
        }
        assert_eq!(parser_length("abcd"), Ok(("", "abcd")));
        assert!(parser_length("abcde").is_err());
        assert!(parser_length("123").is_err());
    }
    #[test]
    fn verify_content() {
        fn parser_content(input: &str) -> IResult<&str, &str> {
            verify(alpha1, |s: &str| s.contains('x'))(input)
        }
        assert_eq!(parser_content("xabc"), Ok(("", "xabc")));
        assert!(parser_content("abc").is_err());
        assert!(parser_content("123").is_err());
    }
    #[test]
    fn verify_error_kind() {
        fn parser(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
            verify(alpha1, |s: &str| s.len() == 4)(input)
        }
        match parser("abcde") {
            Err(crate::Err::Error(crate::error::Error { input, code })) => {
                assert_eq!(input, "abcde");
                assert_eq!(code, ErrorKind::Verify);
            }
            _ => panic!("Error kind test failed"),
        }
    }
    #[test]
    fn verify_error_on_non_alpha() {
        fn parser(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
            verify(alpha1, |s: &str| s.len() == 4)(input)
        }
        match parser("123abcd;") {
            Err(crate::Err::Error(crate::error::Error { input, code })) => {
                assert_eq!(input, "123abcd;");
                assert_eq!(code, ErrorKind::Alpha);
            }
            _ => panic!("Non-alpha error test failed"),
        }
    }
}
