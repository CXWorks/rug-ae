//! Choice combinators

#[cfg(test)]
mod tests;

use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, IResult, Parser};

/// Helper trait for the [alt()] combinator.
///
/// This trait is implemented for tuples of up to 21 elements
pub trait Alt<I, O, E> {
  /// Tests each parser in the tuple and returns the result of the first one that succeeds
  fn choice(&mut self, input: I) -> IResult<I, O, E>;
}

/// Tests a list of parsers one by one until one succeeds.
///
/// It takes as argument a tuple of parsers. There is a maximum of 21
/// parsers. If you need more, it is possible to nest them in other `alt` calls,
/// like this: `alt(parser_a, alt(parser_b, parser_c))`
///
/// ```rust
/// # use nom::error_position;
/// # use nom::{Err,error::ErrorKind, Needed, IResult};
/// use nom::character::complete::{alpha1, digit1};
/// use nom::branch::alt;
/// # fn main() {
/// fn parser(input: &str) -> IResult<&str, &str> {
///   alt((alpha1, digit1))(input)
/// };
///
/// // the first parser, alpha1, recognizes the input
/// assert_eq!(parser("abc"), Ok(("", "abc")));
///
/// // the first parser returns an error, so alt tries the second one
/// assert_eq!(parser("123456"), Ok(("", "123456")));
///
/// // both parsers failed, and with the default error type, alt will return the last error
/// assert_eq!(parser(" "), Err(Err::Error(error_position!(" ", ErrorKind::Digit))));
/// # }
/// ```
///
/// With a custom error type, it is possible to have alt return the error of the parser
/// that went the farthest in the input data
pub fn alt<I: Clone, O, E: ParseError<I>, List: Alt<I, O, E>>(
  mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
  move |i: I| l.choice(i)
}

/// Helper trait for the [permutation()] combinator.
///
/// This trait is implemented for tuples of up to 21 elements
pub trait Permutation<I, O, E> {
  /// Tries to apply all parsers in the tuple in various orders until all of them succeed
  fn permutation(&mut self, input: I) -> IResult<I, O, E>;
}

/// Applies a list of parsers in any order.
///
/// Permutation will succeed if all of the child parsers succeeded.
/// It takes as argument a tuple of parsers, and returns a
/// tuple of the parser results.
///
/// ```rust
/// # use nom::{Err,error::{Error, ErrorKind}, Needed, IResult};
/// use nom::character::complete::{alpha1, digit1};
/// use nom::branch::permutation;
/// # fn main() {
/// fn parser(input: &str) -> IResult<&str, (&str, &str)> {
///   permutation((alpha1, digit1))(input)
/// }
///
/// // permutation recognizes alphabetic characters then digit
/// assert_eq!(parser("abc123"), Ok(("", ("abc", "123"))));
///
/// // but also in inverse order
/// assert_eq!(parser("123abc"), Ok(("", ("abc", "123"))));
///
/// // it will fail if one of the parsers failed
/// assert_eq!(parser("abc;"), Err(Err::Error(Error::new(";", ErrorKind::Digit))));
/// # }
/// ```
///
/// The parsers are applied greedily: if there are multiple unapplied parsers
/// that could parse the next slice of input, the first one is used.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, IResult};
/// use nom::branch::permutation;
/// use nom::character::complete::{anychar, char};
///
/// fn parser(input: &str) -> IResult<&str, (char, char)> {
///   permutation((anychar, char('a')))(input)
/// }
///
/// // anychar parses 'b', then char('a') parses 'a'
/// assert_eq!(parser("ba"), Ok(("", ('b', 'a'))));
///
/// // anychar parses 'a', then char('a') fails on 'b',
/// // even though char('a') followed by anychar would succeed
/// assert_eq!(parser("ab"), Err(Err::Error(Error::new("b", ErrorKind::Char))));
/// ```
///
pub fn permutation<I: Clone, O, E: ParseError<I>, List: Permutation<I, O, E>>(
  mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
  move |i: I| l.permutation(i)
}

macro_rules! alt_trait(
  ($first:ident $second:ident $($id: ident)+) => (
    alt_trait!(__impl $first $second; $($id)+);
  );
  (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
    alt_trait_impl!($($current)*);

    alt_trait!(__impl $($current)* $head; $($id)+);
  );
  (__impl $($current:ident)*; $head:ident) => (
    alt_trait_impl!($($current)*);
    alt_trait_impl!($($current)* $head);
  );
);

macro_rules! alt_trait_impl(
  ($($id:ident)+) => (
    impl<
      Input: Clone, Output, Error: ParseError<Input>,
      $($id: Parser<Input, Output = Output, Error = Error>),+
    > Alt<Input, Output, Error> for ( $($id),+ ) {

      fn choice(&mut self, input: Input) -> IResult<Input, Output, Error> {
        match self.0.parse(input.clone()) {
          Err(Err::Error(e)) => alt_trait_inner!(1, self, input, e, $($id)+),
          res => res,
        }
      }
    }
  );
);

macro_rules! alt_trait_inner(
  ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident $($id:ident)+) => (
    match $self.$it.parse($input.clone()) {
      Err(Err::Error(e)) => {
        let err = $err.or(e);
        succ!($it, alt_trait_inner!($self, $input, err, $($id)+))
      }
      res => res,
    }
  );
  ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident) => (
    Err(Err::Error(Error::append($input, ErrorKind::Alt, $err)))
  );
);

alt_trait!(A B C D E F G H I J K L M N O P Q R S T U);

// Manually implement Alt for (A,), the 1-tuple type
impl<Input, Output, Error: ParseError<Input>, A: Parser<Input, Output = Output, Error = Error>>
  Alt<Input, Output, Error> for (A,)
{
  fn choice(&mut self, input: Input) -> IResult<Input, Output, Error> {
    self.0.parse(input)
  }
}

macro_rules! permutation_trait(
  (
    $name1:ident $ty1:ident $item1:ident
    $name2:ident $ty2:ident $item2:ident
    $($name3:ident $ty3:ident $item3:ident)*
  ) => (
    permutation_trait!(__impl $name1 $ty1 $item1, $name2 $ty2 $item2; $($name3 $ty3 $item3)*);
  );
  (
    __impl $($name:ident $ty:ident $item:ident),+;
    $name1:ident $ty1:ident $item1:ident $($name2:ident $ty2:ident $item2:ident)*
  ) => (
    permutation_trait_impl!($($name $ty $item),+);
    permutation_trait!(__impl $($name $ty $item),+ , $name1 $ty1 $item1; $($name2 $ty2 $item2)*);
  );
  (__impl $($name:ident $ty:ident $item:ident),+;) => (
    permutation_trait_impl!($($name $ty $item),+);
  );
);

macro_rules! permutation_trait_impl(
  ($($name:ident $ty:ident $item:ident),+) => (
    impl<
      Input: Clone, $($ty),+ , Error: ParseError<Input>,
      $($name: Parser<Input, Output = $ty, Error = Error>),+
    > Permutation<Input, ( $($ty),+ ), Error> for ( $($name),+ ) {

      fn permutation(&mut self, mut input: Input) -> IResult<Input, ( $($ty),+ ), Error> {
        let mut res = ($(Option::<$ty>::None),+);

        loop {
          let mut err: Option<Error> = None;
          permutation_trait_inner!(0, self, input, res, err, $($name)+);

          // If we reach here, every iterator has either been applied before,
          // or errored on the remaining input
          if let Some(err) = err {
            // There are remaining parsers, and all errored on the remaining input
            return Err(Err::Error(Error::append(input, ErrorKind::Permutation, err)));
          }

          // All parsers were applied
          match res {
            ($(Some($item)),+) => return Ok((input, ($($item),+))),
            _ => unreachable!(),
          }
        }
      }
    }
  );
);

macro_rules! permutation_trait_inner(
  ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr, $head:ident $($id:ident)*) => (
    if $res.$it.is_none() {
      match $self.$it.parse($input.clone()) {
        Ok((i, o)) => {
          $input = i;
          $res.$it = Some(o);
          continue;
        }
        Err(Err::Error(e)) => {
          $err = Some(match $err {
            Some(err) => err.or(e),
            None => e,
          });
        }
        Err(e) => return Err(e),
      };
    }
    succ!($it, permutation_trait_inner!($self, $input, $res, $err, $($id)*));
  );
  ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr,) => ();
);

permutation_trait!(
  FnA A a
  FnB B b
  FnC C c
  FnD D d
  FnE E e
  FnF F f
  FnG G g
  FnH H h
  FnI I i
  FnJ J j
  FnK K k
  FnL L l
  FnM M m
  FnN N n
  FnO O o
  FnP P p
  FnQ Q q
  FnR R r
  FnS S s
  FnT T t
  FnU U u
);
#[cfg(test)]
mod tests_llm_16_84_llm_16_84 {
    use crate::error::ParseError;
    use crate::IResult;
    use crate::branch::alt;

    fn parser1(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
        if input.starts_with("a") {
            Ok((&input[1..], "a"))
        } else {
            Err(crate::Err::Error(crate::error::Error::new(input, crate::error::ErrorKind::Char)))
        }
    }

    fn parser2(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
        if input.starts_with("b") {
            Ok((&input[1..], "b"))
        } else {
            Err(crate::Err::Error(crate::error::Error::new(input, crate::error::ErrorKind::Char)))
        }
    }

    fn parser3(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
        if input.starts_with("c") {
            Ok((&input[1..], "c"))
        } else {
            Err(crate::Err::Error(crate::error::Error::new(input, crate::error::ErrorKind::Char)))
        }
    }

    #[test]
    fn test_choice_success_first() {
        let mut parser = alt((parser1, parser2));
        let result = parser("abc");
        assert_eq!(result, Ok(("bc", "a")));
    }

    #[test]
    fn test_choice_success_second() {
        let mut parser = alt((parser1, parser2));
        let result = parser("bac");
        assert_eq!(result, Ok(("ac", "b")));
    }

    #[test]
    fn test_choice_failure() {
        let mut parser = alt((parser1, parser2));
        let result = parser("xyz");
        assert!(result.is_err());
        if let Err(crate::Err::Error(crate::error::Error { input, code })) = result {
            assert_eq!(input, "xyz");
            assert_eq!(code, crate::error::ErrorKind::Char);
        } else {
            panic!("Expected Err::Error, got {:?}", result);
        }
    }

    #[test]
    fn test_choice_with_more_alternatives() {
        let mut parser = alt((parser1, parser2, parser3));
        let result = parser("cde");
        assert_eq!(result, Ok(("de", "c")));
    }
}#[cfg(test)]
mod tests_llm_16_87_llm_16_87 {
    use crate::{
        branch::alt,
        error::{ErrorKind, ParseError},
        IResult, Parser,
    };

    // Assuming input and output types for simplicity and demonstration
    type Input = &'static str;
    type Output = &'static str;
    type Error = (&'static str, ErrorKind);

    // Mock parsers
    fn parser_a(input: Input) -> IResult<Input, Output, Error> {
        if input == "a" {
            Ok((input, "A"))
        } else {
            Err(crate::Err::Error((input, ErrorKind::Char)))
        }
    }

    fn parser_b(input: Input) -> IResult<Input, Output, Error> {
        if input == "b" {
            Ok((input, "B"))
        } else {
            Err(crate::Err::Error((input, ErrorKind::Char)))
        }
    }

    fn parser_c(input: Input) -> IResult<Input, Output, Error> {
        if input == "c" {
            Ok((input, "C"))
        } else {
            Err(crate::Err::Error((input, ErrorKind::Char)))
        }
    }

    fn parser_d(input: Input) -> IResult<Input, Output, Error> {
        if input == "d" {
            Ok((input, "D"))
        } else {
            Err(crate::Err::Error((input, ErrorKind::Char)))
        }
    }

    fn parser_e(input: Input) -> IResult<Input, Output, Error> {
        if input == "e" {
            Ok((input, "E"))
        } else {
            Err(crate::Err::Error((input, ErrorKind::Char)))
        }
    }

    #[test]
    fn test_choice() {
        let mut parsers = alt((parser_a, parser_b, parser_c, parser_d, parser_e));

        let res_a = parsers.parse("a");
        assert_eq!(res_a, Ok(("a", "A")));

        let res_b = parsers.parse("b");
        assert_eq!(res_b, Ok(("b", "B")));

        let res_c = parsers.parse("c");
        assert_eq!(res_c, Ok(("c", "C")));

        let res_d = parsers.parse("d");
        assert_eq!(res_d, Ok(("d", "D")));

        let res_e = parsers.parse("e");
        assert_eq!(res_e, Ok(("e", "E")));

        let res_f = parsers.parse("f");
        assert!(res_f.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_88_llm_16_88 {
    use crate::{
        branch::alt,
        combinator::map,
        error::{Error, ErrorKind},
        Err as NomErr, IResult, Parser,
    };

    fn parser1(input: &str) -> IResult<&str, &str, Error<&str>> {
        if input.starts_with('a') {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(NomErr::Error(Error::new(input, ErrorKind::Char)))
        }
    }

    fn parser2(input: &str) -> IResult<&str, &str, Error<&str>> {
        if input.starts_with('b') {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(NomErr::Error(Error::new(input, ErrorKind::Char)))
        }
    }

    fn parser3(input: &str) -> IResult<&str, &str, Error<&str>> {
        if input.starts_with('c') {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(NomErr::Error(Error::new(input, ErrorKind::Char)))
        }
    }

    #[test]
    fn test_choice() {
        let mut parser = alt((parser1, parser2, parser3));
        let input = "a123";

        assert_eq!(parser.parse(input), Ok(("123", "a")));

        let input = "b123";
        assert_eq!(parser.parse(input), Ok(("123", "b")));

        let input = "c123";
        assert_eq!(parser.parse(input), Ok(("123", "c")));

        let input = "d123";
        assert!(parser.parse(input).is_err());
    }
}#[cfg(test)]
mod tests_llm_16_106_llm_16_106 {
    use crate::{
        branch::permutation,
        error::{make_error, ErrorKind, ParseError},
        Err, IResult, Parser,
    };

    #[derive(Debug, PartialEq)]
    struct CustomError<'a>(&'a str);
    type CustomResult<'a, O> = IResult<&'a str, O, CustomError<'a>>;

    impl<'a> ParseError<&'a str> for CustomError<'a> {
        fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
            CustomError(input)
        }

        fn append(input: &'a str, kind: ErrorKind, other: Self) -> Self {
            CustomError(input)
        }

        fn from_char(input: &'a str, _: char) -> Self {
            CustomError(input)
        }

        fn or(self, other: Self) -> Self {
            other
        }
    }

    fn parse_a(input: &str) -> CustomResult<'_, char> {
        if let Some(first) = input.chars().next() {
            if first == 'a' {
                return Ok((&input[1..], 'a'));
            }
        }
        Err(Err::Error(make_error(input, ErrorKind::Char)))
    }

    fn parse_b(input: &str) -> CustomResult<'_, char> {
        if let Some(first) = input.chars().next() {
            if first == 'b' {
                return Ok((&input[1..], 'b'));
            }
        }
        Err(Err::Error(make_error(input, ErrorKind::Char)))
    }

    #[test]
    fn test_permutation_success() {
        let input = "ab";
        let res = permutation((parse_a, parse_b))(input);
        assert_eq!(res, Ok(("", ('a', 'b'))));

        let input = "ba";
        let res = permutation((parse_a, parse_b))(input);
        assert_eq!(res, Ok(("", ('b', 'a'))));
    }

    #[test]
    fn test_permutation_incomplete() {
        let input = "a";
        let res = permutation((parse_a, parse_b))(input);
        assert!(res.is_err());
    }

    #[test]
    fn test_permutation_error() {
        let input = "cd";
        let res = permutation((parse_a, parse_b))(input);
        assert!(res.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_108_llm_16_108 {
    use super::*;

use crate::*;
    use crate::{
        branch::permutation,
        error::{ErrorKind, ParseError},
        Err, IResult, Parser,
    };

    #[derive(Debug, PartialEq)]
    struct CustomError<'a>(&'a str);

    impl<'a> ParseError<&'a str> for CustomError<'a> {
        fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
            CustomError("custom error")
        }

        fn append(_input: &'a str, _kind: ErrorKind, other: Self) -> Self {
            other
        }
    }

    type TestResult<'a, O> = IResult<&'a str, O, CustomError<'a>>;

    fn parser_a(input: &str) -> TestResult<&str> {
        if input.starts_with("a") {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(Err::Error(CustomError("Expected 'a'")))
        }
    }

    fn parser_b(input: &str) -> TestResult<&str> {
        if input.starts_with("b") {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(Err::Error(CustomError("Expected 'b'")))
        }
    }

    fn parser_c(input: &str) -> TestResult<&str> {
        if input.starts_with("c") {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(Err::Error(CustomError("Expected 'c'")))
        }
    }

    #[test]
    fn test_permutation() {
        let mut parser = permutation((parser_a, parser_b, parser_c));

        assert_eq!(
            parser("abc"),
            Ok(("", ("a", "b", "c")))
        );

        assert_eq!(
            parser("cba"),
            Ok(("", ("c", "b", "a")))
        );

        assert_eq!(
            parser("bac"),
            Ok(("", ("b", "a", "c")))
        );

        assert_eq!(
            parser("acb"),
            Ok(("", ("a", "c", "b")))
        );

        assert_eq!(
            parser("a"),
            Err(Err::Error(CustomError("custom error")))
        );

        assert_eq!(
            parser(""),
            Err(Err::Error(CustomError("custom error")))
        );

        assert_eq!(
            parser("aa"),
            Err(Err::Error(CustomError("Expected 'b'")))
        );
    }
}#[cfg(test)]
mod tests_llm_16_110 {
    use crate::{
        branch::permutation,
        character::complete::{char, digit1},
        IResult,
    };

    fn parse_permutation(input: &str) -> IResult<&str, (char, &str, char)> {
        permutation((char('A'), digit1, char('B')))(input)
    }

    #[test]
    fn test_permutation_success() {
        let res = parse_permutation("1A2B");
        assert_eq!(res, Ok(("2", ('A', "1", 'B'))));

        let res = parse_permutation("A1B2");
        assert_eq!(res, Ok(("2", ('A', "1", 'B'))));
    }

    #[test]
    fn test_permutation_partial() {
        let res = parse_permutation("A1");
        assert!(res.is_err());
    }

    #[test]
    fn test_permutation_failure() {
        let res = parse_permutation("1C2");
        assert!(res.is_err());
    }

    #[test]
    fn test_permutation_incomplete() {
        let res = parse_permutation("A");
        assert!(res.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_116_llm_16_116 {
    use super::*; // to use permutation and other items not imported explicitly

use crate::*;
    use crate::{
        combinator::map,
        error::ParseError,
        multi::many1,
        IResult,
    };
    use std::ops::RangeInclusive;

    // Helper parser functions
    fn parser_a(input: &str) -> IResult<&str, char> {
        match input.chars().next() {
            Some('a') => Ok((&input[1..], 'a')),
            _ => Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Char))),
        }
    }
    fn parser_b(input: &str) -> IResult<&str, char> {
        match input.chars().next() {
            Some('b') => Ok((&input[1..], 'b')),
            _ => Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Char))),
        }
    }
    fn parser_c(input: &str) -> IResult<&str, char> {
        match input.chars().next() {
            Some('c') => Ok((&input[1..], 'c')),
            _ => Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Char))),
        }
    }

    #[test]
    fn test_permutation() {
        // This test assumes a 3-element permutation for demonstration
        let mut permutation_parser = permutation((parser_a, parser_b, parser_c));

        // Test successful permutation
        let input = "abc";
        let expected = Ok(("c", ('a', 'b', 'a')));
        assert_eq!(permutation_parser(input), expected);

        // Test a failed permutation
        let input = "1bc"; // assuming the parser_a requires 'a'
        assert!(matches!(permutation_parser(input), Err(crate::Err::Error(_))));

        // Test incomplete permutation
        let input = "ab"; // not enough input for the third parser
        assert!(matches!(permutation_parser(input), Err(crate::Err::Error(_))));
    }
}#[cfg(test)]
mod tests_llm_16_122 {
    use super::*;

use crate::*;
    use crate::{
        branch::permutation,
        bytes::complete::tag,
        error::ErrorKind,
        sequence::preceded,
        IResult,
    };

    fn abc_tag(input: &str) -> IResult<&str, &str> {
        tag("abc")(input)
    }

    fn def_tag(input: &str) -> IResult<&str, &str> {
        tag("def")(input)
    }

    fn xyz_tag(input: &str) -> IResult<&str, &str> {
        tag("xyz")(input)
    }

    #[test]
    fn test_permutation() {
        // Successful permutation parsing
        let expected = Ok(("", ("xyz", "def", "abc")));
        let res = permutation((xyz_tag, def_tag, abc_tag))("xyzdefabc");
        assert_eq!(res, expected);

        // Successful permutation parsing with mixed order
        let expected = Ok(("", ("abc", "xyz", "def")));
        let res = permutation((abc_tag, xyz_tag, def_tag))("xyzdefabc");
        assert_eq!(res, expected);

        // Incomplete permutation
        let res = permutation((abc_tag, def_tag, xyz_tag))("defabc");
        assert!(res.is_err());
        if let Err(crate::Err::Error(e)) = res {
            assert_eq!(e.code, ErrorKind::Permutation);
        } else {
            panic!("Error expected");
        }

        // Permutation with extra input
        let expected = Ok(("ghi", ("xyz", "def", "abc")));
        let res = permutation((xyz_tag, def_tag, abc_tag))("xyzdefabcghi");
        assert_eq!(res, expected);

        // Permutation with a missing element
        let res = permutation((abc_tag, xyz_tag))("xyzdefabc");
        assert!(res.is_err());
        if let Err(crate::Err::Error(e)) = res {
            assert_eq!(e.code, ErrorKind::Permutation);
        } else {
            panic!("Error expected");
        }
    }
}#[cfg(test)]
mod tests_llm_16_124 {
    use super::*;

use crate::*;
    use crate::{
        branch::permutation,
        error::{ErrorKind, ParseError},
        IResult, Parser,
    };

    fn parser_a(input: &str) -> IResult<&str, char> {
        match input.chars().next() {
            Some('a') => Ok((&input['a'.len_utf8()..], 'a')),
            _ => Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Char))),
        }
    }

    fn parser_b(input: &str) -> IResult<&str, char> {
        match input.chars().next() {
            Some('b') => Ok((&input['b'.len_utf8()..], 'b')),
            _ => Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Char))),
        }
    }

    fn parser_c(input: &str) -> IResult<&str, char> {
        match input.chars().next() {
            Some('c') => Ok((&input['c'.len_utf8()..], 'c')),
            _ => Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Char))),
        }
    }

    #[test]
    fn test_permutation() {
        let mut parser = permutation((parser_a, parser_b, parser_c));
        let input = "cab";
        let result = parser.parse(input);
        assert_eq!(result, Ok(("", ('c', 'a', 'b'))));
    }

    #[test]
    fn test_permutation_incomplete() {
        let mut parser = permutation((parser_a, parser_b, parser_c));
        let input = "ac";
        let result = parser.parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_permutation_unexpected() {
        let mut parser = permutation((parser_a, parser_b, parser_c));
        let input = "xyz";
        let result = parser.parse(input);
        assert!(result.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_126 {
    use crate::{
        branch::permutation,
        bytes::complete::tag,
        error::{Error, ErrorKind},
        Err, IResult,
    };

    #[test]
    fn test_permutation() {
        fn parser_a(input: &str) -> IResult<&str, &str, Error<&str>> {
            tag("a")(input)
        }
        fn parser_b(input: &str) -> IResult<&str, &str, Error<&str>> {
            tag("b")(input)
        }
        fn parser_c(input: &str) -> IResult<&str, &str, Error<&str>> {
            tag("c")(input)
        }

        let result = permutation((parser_a, parser_b, parser_c))("cab");
        assert_eq!(result, Ok(("", ("c", "a", "b"))));

        let result = permutation((parser_a, parser_b, parser_c))("acb");
        assert_eq!(result, Ok(("", ("a", "c", "b"))));

        let result = permutation((parser_a, parser_b, parser_c))("abc");
        assert_eq!(result, Ok(("", ("a", "b", "c"))));

        let result = permutation((parser_a, parser_b, parser_c))("bac");
        assert_eq!(result, Ok(("", ("b", "a", "c"))));

        let result = permutation((parser_a, parser_b, parser_c))("bca");
        assert_eq!(result, Ok(("", ("b", "c", "a"))));

        let result = permutation((parser_a, parser_b, parser_c))("cba");
        assert_eq!(result, Ok(("", ("c", "b", "a"))));

        let err_result = permutation((parser_a, parser_b, parser_c))("dab");
        assert!(matches!(
            err_result,
            Err(Err::Error(Error {
                input,
                code: ErrorKind::Permutation,
                ..
            })) if input == "dab"
        ));
    }
}#[cfg(test)]
mod tests_llm_16_128_llm_16_128 {
    use crate::{
        branch::Permutation,
        bytes::complete::tag,
        error::{ErrorKind, ParseError},
        IResult,
    };

    #[test]
    fn test_permutation() {
        fn parser_a(input: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
            tag("a")(input)
        }
        fn parser_b(input: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
            tag("b")(input)
        }
        fn parser_c(input: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
            tag("c")(input)
        }

        let mut permutation_parser = (parser_a, parser_b, parser_c);

        let input = "bac";
        let result = permutation_parser.permutation(input);
        assert_eq!(result, Ok(("", ("b", "a", "c"))));
    }
}#[cfg(test)]
mod tests_llm_16_130 {
    use crate::{
        branch::Permutation,
        error::ErrorKind,
        IResult,
    };

    fn parser_a(input: &str) -> IResult<&str, &str> {
        crate::bytes::complete::tag("a")(input)
    }

    fn parser_b(input: &str) -> IResult<&str, &str> {
        crate::bytes::complete::tag("b")(input)
    }

    fn parser_c(input: &str) -> IResult<&str, &str> {
        crate::bytes::complete::tag("c")(input)
    }

    #[test]
    fn test_permutation() {
        let mut parsers = (parser_a, parser_b, parser_c);

        let input = "bac";
        let output = parsers.permutation(input);
        assert_eq!(output, Ok(("", ("b", "a", "c"))));

        let input = "abc";
        let output = parsers.permutation(input);
        assert_eq!(output, Ok(("", ("a", "b", "c"))));

        let input = "acb";
        let output = parsers.permutation(input);
        assert_eq!(output, Ok(("", ("a", "c", "b"))));

        let input = "cba";
        let output = parsers.permutation(input);
        assert_eq!(output, Ok(("", ("c", "b", "a"))));

        // Test partial input
        let input = "ba";
        let output = parsers.permutation(input);
        assert!(output.is_err());
        if let Err(crate::Err::Error(e)) = output {
            assert_eq!(e.code, ErrorKind::Permutation);
        } else {
            panic!("Expected Permutation error, but got {:?}", output);
        }

        // Test wrong input
        let input = "def";
        let output = parsers.permutation(input);
        assert!(output.is_err());
        if let Err(crate::Err::Error(e)) = output {
            assert_eq!(e.code, ErrorKind::Permutation);
        } else {
            panic!("Expected Permutation error, but got {:?}", output);
        }
    }
}#[cfg(test)]
mod tests_llm_16_132_llm_16_132 {
    use crate::{
        branch::permutation,
        error::{Error, ErrorKind},
        Err, IResult,
    };

    // Define dummy parsers to use with the permutation function
    fn parse_a(input: &[u8]) -> IResult<&[u8], u8, Error<&[u8]>> {
        if input.is_empty() {
            Err(Err::Error(Error::new(input, ErrorKind::Eof)))
        } else {
            Ok((&input[1..], input[0]))
        }
    }
    fn parse_b(input: &[u8]) -> IResult<&[u8], u8, Error<&[u8]>> {
        if input.len() < 2 {
            Err(Err::Error(Error::new(input, ErrorKind::Eof)))
        } else {
            Ok((&input[2..], input[1]))
        }
    }
    fn parse_c(input: &[u8]) -> IResult<&[u8], u8, Error<&[u8]>> {
        if input.len() < 3 {
            Err(Err::Error(Error::new(input, ErrorKind::Eof)))
        } else {
            Ok((&input[3..], input[2]))
        }
    }

    #[test]
    fn test_permutation() {
        let mut parser = permutation((parse_a, parse_b, parse_c));
        let input = &[1, 2, 3, 4, 5][..];

        let expected = Ok((&[4, 5][..], (1, 2, 3)));
        let result = parser(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_permutation_incomplete() {
        let mut parser = permutation((parse_a, parse_b, parse_c));
        let input = &[1, 2][..];

        if let Err(Err::Error(err)) = parser(input) {
            assert_eq!(err.input, &[2][..]);
            assert_eq!(err.code, ErrorKind::Eof);
        } else {
            panic!("Expected error, but got success");
        }
    }

    #[test]
    fn test_permutation_error() {
        let mut parser = permutation((parse_a, parse_b, parse_c));
        let input = &[][..];

        assert!(parser(input).is_err());
    }
}#[cfg(test)]
mod tests_llm_16_134 {
    use crate::{
        branch::permutation,
        bytes::complete::tag,
        error::{ErrorKind, ParseError},
        IResult,
    };

    #[test]
    fn test_permutation() {
        fn parser1(input: &str) -> IResult<&str, char> {
            tag("A")(input).map(|(next_input, res)| (next_input, res.chars().next().unwrap()))
        }

        fn parser2(input: &str) -> IResult<&str, char> {
            tag("B")(input).map(|(next_input, res)| (next_input, res.chars().next().unwrap()))
        }

        fn parser3(input: &str) -> IResult<&str, char> {
            tag("C")(input).map(|(next_input, res)| (next_input, res.chars().next().unwrap()))
        }

        let mut parser = permutation((parser1, parser2, parser3));

        let input = "CBA";
        let expected = Ok(("", ('A', 'B', 'C')));
        let res = parser(input);
        assert_eq!(res, expected);

        let input = "ACB";
        let expected = Ok(("", ('A', 'B', 'C')));
        let res = parser(input);
        assert_eq!(res, expected);

        let input = "BAC";
        let expected = Ok(("", ('A', 'B', 'C')));
        let res = parser(input);
        assert_eq!(res, expected);

        let input = "BCA";
        let expected = Ok(("", ('A', 'B', 'C')));
        let res = parser(input);
        assert_eq!(res, expected);

        let input = "CAB";
        let expected = Ok(("", ('A', 'B', 'C')));
        let res = parser(input);
        assert_eq!(res, expected);

        let input = "ABC";
        let expected = Ok(("", ('A', 'B', 'C')));
        let res = parser(input);
        assert_eq!(res, expected);

        let input = "AB";
        let err = Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Permutation)));
        let res = parser(input);
        assert_eq!(res, err);

        let input = "A";
        let err = Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Permutation)));
        let res = parser(input);
        assert_eq!(res, err);

        let input = "";
        let err = Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Permutation)));
        let res = parser(input);
        assert_eq!(res, err);
    }
}#[cfg(test)]
mod tests_llm_16_138 {
    use crate::{
        branch::permutation,
        bytes::complete::tag,
        error::{ErrorKind, ParseError},
        Err, IResult,
    };

    #[test]
    fn test_permutation() {
        fn parser_a(input: &str) -> IResult<&str, &str> {
            tag("a")(input)
        }
        fn parser_b(input: &str) -> IResult<&str, &str> {
            tag("b")(input)
        }
        fn parser_c(input: &str) -> IResult<&str, &str> {
            tag("c")(input)
        }

        let res = permutation((parser_a, parser_b, parser_c))("bcabcac");
        match res {
            Ok((remaining, (a, b, c))) => {
                assert_eq!(("abcac", ("a", "b", "c")), (remaining, (a, b, c)));
            }
            _ => panic!("Error while testing permutation"),
        }

        let res = permutation((parser_a, parser_b, parser_c))("def");
        match res {
            Err(Err::Error(e)) => {
                assert_eq!(e.input, "def");
                assert_eq!(e.code, ErrorKind::Permutation);
            }
            _ => panic!("Expected ErrorKind::Permutation"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_280 {
  use crate::{
    branch::alt,
    character::complete::{alpha1, digit1},
    error::{Error, ErrorKind},
    Err, IResult,
  };

  #[test]
  fn alt_success_with_alpha() {
    fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
      alt((alpha1, digit1))(input)
    }

    assert_eq!(parser("abc"), Ok(("", "abc")));
  }

  #[test]
  fn alt_success_with_digit() {
    fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
      alt((alpha1, digit1))(input)
    }

    assert_eq!(parser("123456"), Ok(("", "123456")));
  }

  #[test]
  fn alt_failure() {
    fn parser(input: &str) -> IResult<&str, &str, Error<&str>> {
      alt((alpha1, digit1))(input)
    }

    assert_eq!(
      parser(" "),
      Err(Err::Error(Error::new(" ", ErrorKind::Alpha)))
    );
  }
}#[cfg(test)]
mod tests_llm_16_281_llm_16_281 {
  use super::*; // Import from the parent module

use crate::*;
  use crate::error::{Error, ErrorKind};
  use crate::character::complete::{alpha1, digit1, char, anychar};
  use crate::branch::permutation;

  #[test]
  fn test_permutation_alpha_digit() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
      permutation((alpha1, digit1))(input)
    }

    assert_eq!(parser("abc123"), Ok(("", ("abc", "123"))));
    assert_eq!(parser("123abc"), Ok(("", ("abc", "123"))));
    assert_eq!(parser("abc;"), Err(Err::Error(Error::new(";", ErrorKind::Digit))));
  }

  #[test]
  fn test_permutation_char() {
    fn parser(input: &str) -> IResult<&str, (char, char)> {
      permutation((anychar, char('a')))(input)
    }

    assert_eq!(parser("ba"), Ok(("", ('b', 'a'))));
    assert_eq!(parser("ab"), Err(Err::Error(Error::new("b", ErrorKind::Char))));
  }
}