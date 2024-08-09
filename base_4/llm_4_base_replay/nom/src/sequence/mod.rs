//! Combinators applying parsers in sequence

#[cfg(test)]
mod tests;

use crate::error::ParseError;
use crate::internal::{IResult, Parser};

/// Gets an object from the first parser,
/// then gets another object from the second parser.
///
/// # Arguments
/// * `first` The first parser to apply.
/// * `second` The second parser to apply.
///
/// # Example
/// ```rust
/// use nom::sequence::pair;
/// use nom::bytes::complete::tag;
/// use nom::{error::ErrorKind, Err};
///
/// let mut parser = pair(tag("abc"), tag("efg"));
///
/// assert_eq!(parser("abcefg"), Ok(("", ("abc", "efg"))));
/// assert_eq!(parser("abcefghij"), Ok(("hij", ("abc", "efg"))));
/// assert_eq!(parser(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn pair<I, O1, O2, E: ParseError<I>, F, G>(
  mut first: F,
  mut second: G,
) -> impl FnMut(I) -> IResult<I, (O1, O2), E>
where
  F: Parser<I, Output = O1, Error = E>,
  G: Parser<I, Output = O2, Error = E>,
{
  move |input: I| {
    let (input, o1) = first.parse(input)?;
    second.parse(input).map(|(i, o2)| (i, (o1, o2)))
  }
}

/// Matches an object from the first parser and discards it,
/// then gets an object from the second parser.
///
/// # Arguments
/// * `first` The opening parser.
/// * `second` The second parser to get object.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::sequence::preceded;
/// use nom::bytes::complete::tag;
///
/// let mut parser = preceded(tag("abc"), tag("efg"));
///
/// assert_eq!(parser("abcefg"), Ok(("", "efg")));
/// assert_eq!(parser("abcefghij"), Ok(("hij", "efg")));
/// assert_eq!(parser(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn preceded<I, O, E: ParseError<I>, F, G>(
  mut first: F,
  mut second: G,
) -> impl FnMut(I) -> IResult<I, O, E>
where
  F: Parser<I, Error = E>,
  G: Parser<I, Output = O, Error = E>,
{
  move |input: I| {
    let (input, _) = first.parse(input)?;
    second.parse(input)
  }
}

/// Gets an object from the first parser,
/// then matches an object from the second parser and discards it.
///
/// # Arguments
/// * `first` The first parser to apply.
/// * `second` The second parser to match an object.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::sequence::terminated;
/// use nom::bytes::complete::tag;
///
/// let mut parser = terminated(tag("abc"), tag("efg"));
///
/// assert_eq!(parser("abcefg"), Ok(("", "abc")));
/// assert_eq!(parser("abcefghij"), Ok(("hij", "abc")));
/// assert_eq!(parser(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn terminated<I, O, E: ParseError<I>, F, G>(
  mut first: F,
  mut second: G,
) -> impl FnMut(I) -> IResult<I, O, E>
where
  F: Parser<I, Output = O, Error = E>,
  G: Parser<I, Error = E>,
{
  move |input: I| {
    let (input, o1) = first.parse(input)?;
    second.parse(input).map(|(i, _)| (i, o1))
  }
}

/// Gets an object from the first parser,
/// then matches an object from the sep_parser and discards it,
/// then gets another object from the second parser.
///
/// # Arguments
/// * `first` The first parser to apply.
/// * `sep` The separator parser to apply.
/// * `second` The second parser to apply.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::sequence::separated_pair;
/// use nom::bytes::complete::tag;
///
/// let mut parser = separated_pair(tag("abc"), tag("|"), tag("efg"));
///
/// assert_eq!(parser("abc|efg"), Ok(("", ("abc", "efg"))));
/// assert_eq!(parser("abc|efghij"), Ok(("hij", ("abc", "efg"))));
/// assert_eq!(parser(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn separated_pair<I, O1, O2, E: ParseError<I>, F, G, H>(
  mut first: F,
  mut sep: G,
  mut second: H,
) -> impl FnMut(I) -> IResult<I, (O1, O2), E>
where
  F: Parser<I, Output = O1, Error = E>,
  G: Parser<I, Error = E>,
  H: Parser<I, Output = O2, Error = E>,
{
  move |input: I| {
    let (input, o1) = first.parse(input)?;
    let (input, _) = sep.parse(input)?;
    second.parse(input).map(|(i, o2)| (i, (o1, o2)))
  }
}

/// Matches an object from the first parser and discards it,
/// then gets an object from the second parser,
/// and finally matches an object from the third parser and discards it.
///
/// # Arguments
/// * `first` The first parser to apply and discard.
/// * `second` The second parser to apply.
/// * `third` The third parser to apply and discard.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::sequence::delimited;
/// use nom::bytes::complete::tag;
///
/// let mut parser = delimited(tag("("), tag("abc"), tag(")"));
///
/// assert_eq!(parser("(abc)"), Ok(("", "abc")));
/// assert_eq!(parser("(abc)def"), Ok(("def", "abc")));
/// assert_eq!(parser(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn delimited<I, O, E: ParseError<I>, F, G, H>(
  mut first: F,
  mut second: G,
  mut third: H,
) -> impl FnMut(I) -> IResult<I, O, E>
where
  F: Parser<I, Error = E>,
  G: Parser<I, Output = O, Error = E>,
  H: Parser<I, Error = E>,
{
  move |input: I| {
    let (input, _) = first.parse(input)?;
    let (input, o2) = second.parse(input)?;
    third.parse(input).map(|(i, _)| (i, o2))
  }
}

/// Helper trait for the tuple combinator.
///
/// This trait is implemented for tuples of parsers of up to 21 elements.
#[deprecated(since = "8.0.0", note = "`Parser` is directly implemented for tuples")]
#[allow(deprecated)]
pub trait Tuple<I, O, E> {
  /// Parses the input and returns a tuple of results of each parser.
  fn parse(&mut self, input: I) -> IResult<I, O, E>;
}

#[allow(deprecated)]
impl<Input, Output, Error: ParseError<Input>, F: Parser<Input, Output = Output, Error = Error>>
  Tuple<Input, (Output,), Error> for (F,)
{
  fn parse(&mut self, input: Input) -> IResult<Input, (Output,), Error> {
    self.0.parse(input).map(|(i, o)| (i, (o,)))
  }
}

macro_rules! tuple_trait(
  ($name1:ident $ty1:ident, $name2: ident $ty2:ident, $($name:ident $ty:ident),*) => (
    tuple_trait!(__impl $name1 $ty1, $name2 $ty2; $($name $ty),*);
  );
  (__impl $($name:ident $ty: ident),+; $name1:ident $ty1:ident, $($name2:ident $ty2:ident),*) => (
    tuple_trait_impl!($($name $ty),+);
    tuple_trait!(__impl $($name $ty),+ , $name1 $ty1; $($name2 $ty2),*);
  );
  (__impl $($name:ident $ty: ident),+; $name1:ident $ty1:ident) => (
    tuple_trait_impl!($($name $ty),+);
    tuple_trait_impl!($($name $ty),+, $name1 $ty1);
  );
);

macro_rules! tuple_trait_impl(
  ($($name:ident $ty: ident),+) => (
    #[allow(deprecated)]
    impl<
      Input: Clone, $($ty),+ , Error: ParseError<Input>,
      $($name: Parser<Input, Output = $ty, Error = Error>),+
    > Tuple<Input, ( $($ty),+ ), Error> for ( $($name),+ ) {
      fn parse(&mut self, input: Input) -> IResult<Input, ( $($ty),+ ), Error> {
        tuple_trait_inner!(0, self, input, (), $($name)+)

      }
    }
  );
);

macro_rules! tuple_trait_inner(
  ($it:tt, $self:expr, $input:expr, (), $head:ident $($id:ident)+) => ({
    let (i, o) = $self.$it.parse($input.clone())?;

    succ!($it, tuple_trait_inner!($self, i, ( o ), $($id)+))
  });
  ($it:tt, $self:expr, $input:expr, ($($parsed:tt)*), $head:ident $($id:ident)+) => ({
    let (i, o) = $self.$it.parse($input.clone())?;

    succ!($it, tuple_trait_inner!($self, i, ($($parsed)* , o), $($id)+))
  });
  ($it:tt, $self:expr, $input:expr, ($($parsed:tt)*), $head:ident) => ({
    let (i, o) = $self.$it.parse($input.clone())?;

    Ok((i, ($($parsed)* , o)))
  });
);

tuple_trait!(FnA A, FnB B, FnC C, FnD D, FnE E, FnF F, FnG G, FnH H, FnI I, FnJ J, FnK K, FnL L,
  FnM M, FnN N, FnO O, FnP P, FnQ Q, FnR R, FnS S, FnT T, FnU U);

// Special case: implement `Tuple` for `()`, the unit type.
// This can come up in macros which accept a variable number of arguments.
// Literally, `()` is an empty tuple, so it should simply parse nothing.
#[allow(deprecated)]
impl<I, E: ParseError<I>> Tuple<I, (), E> for () {
  fn parse(&mut self, input: I) -> IResult<I, (), E> {
    Ok((input, ()))
  }
}

///Applies a tuple of parsers one by one and returns their results as a tuple.
///There is a maximum of 21 parsers
/// ```rust
/// # use nom::{Err, error::ErrorKind};
/// use nom::sequence::tuple;
/// use nom::character::complete::{alpha1, digit1};
/// let mut parser = tuple((alpha1, digit1, alpha1));
///
/// assert_eq!(parser("abc123def"), Ok(("", ("abc", "123", "def"))));
/// assert_eq!(parser("123def"), Err(Err::Error(("123def", ErrorKind::Alpha))));
/// ```
#[deprecated(since = "8.0.0", note = "`Parser` is directly implemented for tuples")]
#[allow(deprecated)]
pub fn tuple<I, O, E: ParseError<I>, List: Tuple<I, O, E>>(
  mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
  move |i: I| l.parse(i)
}
#[cfg(test)]
mod tests_llm_16_105_llm_16_105 {
    use crate::{
        error::{ErrorKind, ParseError},
        sequence::tuple,
        IResult, Parser,
    };

    // Mock parser function to use with `parse`
    fn mock_parser(input: &str) -> IResult<&str, &str> {
        if input.starts_with("hello") {
            Ok((&input[5..], &input[..5]))
        } else {
            Err(crate::Err::Error(ParseError::from_error_kind(input, ErrorKind::Tag)))
        }
    }

    #[test]
    fn test_parse_success() {
        let mut parser = tuple((mock_parser,));
        let input = "hello world";
        let expected = Ok((" world", ("hello",)));
        assert_eq!(parser.parse(input), expected);
    }

    #[test]
    fn test_parse_failure() {
        let mut parser = tuple((mock_parser,));
        let input = "goodbye world";
        assert!(parser.parse(input).is_err());
    }
}#[cfg(test)]
mod tests_llm_16_107 {
    use crate::{
        IResult, 
        sequence::tuple,
        bytes::complete::tag,
    };

    #[test]
    fn test_parse_tuple() {
        fn parser(input: &str) -> IResult<&str, (&str, &str)> {
            tuple((tag("hello"), tag("world")))(input)
        }

        let result1 = parser("helloworld!");
        assert_eq!(result1, Ok(("!", ("hello", "world"))));

        let result2 = parser("hello!");
        assert!(result2.is_err());

        let result3 = parser("helloworldworld!");
        assert_eq!(result3, Ok(("world!", ("hello", "world"))));

        let result4 = parser("worldhello!");
        assert!(result4.is_err());

        let result5 = parser("goodbyeworld!");
        assert!(result5.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_109_llm_16_109 {
    use crate::{
        IResult,
        bytes::complete::tag,
        sequence::tuple,
        error::ErrorKind,
        Err
    };

    fn parse_a(input: &str) -> IResult<&str, &str> {
        tag("a")(input)
    }

    fn parse_b(input: &str) -> IResult<&str, &str> {
        tag("b")(input)
    }

    fn parse_c(input: &str) -> IResult<&str, &str> {
        tag("c")(input)
    }

    #[test]
    fn test_parse() {
        let mut parser = tuple((parse_a, parse_b, parse_c));

        assert_eq!(parser("abc"), Ok(("", ("a", "b", "c"))));
        assert_eq!(parser("ab"), Err(Err::Error(crate::error::Error::new("ab", ErrorKind::Tag))));
        assert_eq!(parser("a"), Err(Err::Error(crate::error::Error::new("a", ErrorKind::Tag))));
        assert_eq!(parser("bc"), Err(Err::Error(crate::error::Error::new("bc", ErrorKind::Tag))));
        assert_eq!(parser("c"), Err(Err::Error(crate::error::Error::new("c", ErrorKind::Tag))));
        assert_eq!(parser("abcd"), Ok(("d", ("a", "b", "c"))));
    }
}#[cfg(test)]
mod tests_llm_16_111 {
    use crate::{
        error::ParseError,
        sequence::tuple,
        IResult,
        bytes::complete::tag
    };
    
    fn parse_a(input: &str) -> IResult<&str, &str> {
        tag("A")(input)
    }

    fn parse_b(input: &str) -> IResult<&str, &str> {
        tag("B")(input)
    }

    fn parse_c(input: &str) -> IResult<&str, &str> {
        tag("C")(input)
    }

    fn parse_d(input: &str) -> IResult<&str, &str> {
        tag("D")(input)
    }

    #[test]
    fn test_parse_tuple() {
        let input = "ABCD";
        let expected = Ok(("", ("A", "B", "C", "D")));
        let parsed = tuple((parse_a, parse_b, parse_c, parse_d))(input);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_tuple_incomplete() {
        let input = "ABC";
        let parsed = tuple((parse_a, parse_b, parse_c, parse_d))(input);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_tuple_extra_input() {
        let input = "ABCDE";
        let expected = Ok(("E", ("A", "B", "C", "D")));
        let parsed = tuple((parse_a, parse_b, parse_c, parse_d))(input);
        assert_eq!(parsed, expected);
    }
}#[cfg(test)]
mod tests_llm_16_115_llm_16_115 {
    use crate::{
        error::{Error, ErrorKind},
        IResult, sequence::tuple, Parser,
    };

    fn parser_a(input: &str) -> IResult<&str, char> {
        if let Some(first) = input.chars().next() {
            Ok((&input[first.len_utf8()..], first))
        } else {
            Err(crate::Err::Error(Error::new(input, ErrorKind::Eof)))
        }
    }

    fn parser_b(input: &str) -> IResult<&str, char> {
        if let Some(first) = input.chars().next() {
            Ok((&input[first.len_utf8()..], first))
        } else {
            Err(crate::Err::Error(Error::new(input, ErrorKind::Eof)))
        }
    }

    fn parser_c(input: &str) -> IResult<&str, char> {
        if let Some(first) = input.chars().next() {
            Ok((&input[first.len_utf8()..], first))
        } else {
            Err(crate::Err::Error(Error::new(input, ErrorKind::Eof)))
        }
    }

    #[test]
    fn test_parse() {
        let input = "abc";
        let mut parsers = tuple((parser_a, parser_b, parser_c));
        let result: IResult<_, _> = parsers.parse(input);
        assert_eq!(result, Ok(("", ('a', 'b', 'c'))));
    }
}#[cfg(test)]
mod tests_llm_16_117_llm_16_117 {
    use crate::{
        error::ParseError,
        sequence::tuple,
        IResult,
        combinator::map,
        character::complete::char
    };

    // Dummy parser functions wrapped with `map` to always succeed with the given character
    fn parse_a(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('a'), |c| c)(input)
    }

    fn parse_b(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('b'), |c| c)(input)
    }

    fn parse_c(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('c'), |c| c)(input)
    }

    fn parse_d(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('d'), |c| c)(input)
    }

    fn parse_e(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('e'), |c| c)(input)
    }

    fn parse_f(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('f'), |c| c)(input)
    }

    fn parse_g(input: &str) -> IResult<&str, char, (&str, crate::error::ErrorKind)> {
        map(char('g'), |c| c)(input)
    }

    #[test]
    fn test_parse() {
        let mut parse_tuple = tuple((parse_a, parse_b, parse_c, parse_d, parse_e, parse_f, parse_g));

        let input = "abcdefg";
        let expected = Ok(("", ('a', 'b', 'c', 'd', 'e', 'f', 'g')));

        assert_eq!(parse_tuple(input), expected);
    }
}#[cfg(test)]
mod tests_llm_16_121 {
    use super::*; // Adjust the path to match the actual module structure

use crate::*;
    use crate::{
        bytes::complete::tag,
        character::complete::digit1,
        sequence::{tuple, Tuple},
        IResult,
    };

    #[test]
    fn test_parse() {
        fn parse_a(input: &str) -> IResult<&str, &str> {
            tag("a")(input)
        }

        fn parse_b(input: &str) -> IResult<&str, &str> {
            digit1(input)
        }

        fn parse_c(input: &str) -> IResult<&str, &str> {
            tag("c")(input)
        }

        let mut parser = tuple((parse_a, parse_b, parse_c));

        // Test case #1: Success
        let input = "a123c";
        match parser.parse(input) {
            Ok((remaining, output)) => {
                assert_eq!(remaining, "");
                assert_eq!(output, ("a", "123", "c"));
            }
            Err(_) => panic!("Test case #1: Expected successful parse"),
        }

        // Test case #2: Partial parse
        let input = "a123";
        match parser.parse(input) {
            Ok((remaining, output)) => {
                assert_eq!(remaining, "");
                assert_eq!(output, ("a", "123", ""));
            }
            Err(_) => panic!("Test case #2: Expected partial successful parse"),
        }

        // Test case #3: Error
        let input = "a12";
        match parser.parse(input) {
            Ok(_) => panic!("Test case #3: Expected error"),
            Err(_) => (), // Expected error
        }

        // Add more test cases as needed
    }
}#[cfg(test)]
mod tests_llm_16_125_llm_16_125 {
    use super::*;

use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, sequence::tuple
    };

    #[test]
    fn test_parse_tuple() {
        // Define dummy parsers for the tuple elements
        fn parse_a(input: &str) -> IResult<&str, char, Error<&str>> {
            if input.starts_with('a') {
                Ok((&input[1..], 'a'))
            } else {
                Err(Err::Error(Error::new(input, ErrorKind::Char)))
            }
        }

        fn parse_b(input: &str) -> IResult<&str, char, Error<&str>> {
            if input.starts_with('b') {
                Ok((&input[1..], 'b'))
            } else {
                Err(Err::Error(Error::new(input, ErrorKind::Char)))
            }
        }

        // ... Create parsers for all other elements, from `parse_c` to `parse_k`

        // Combine the dummy parsers into a tuple
        let mut parser = tuple((parse_a, parse_b /*, parse_c, ..., parse_k*/));
        
        // Test successful parsing
        let test_input = "abcdefghijk";
        if let IResult::Ok((remaining, (a, b /*, c, ..., k*/))) = parser(test_input) {
            assert_eq!(remaining, "cdefghijk");
            assert_eq!(a, 'a');
            assert_eq!(b, 'b');
            // ... Assert all other elements, from `c` to `k`
        } else {
            panic!("Parser error: Expected successful parsing.");
        }

        // Test incomplete input
        let test_input_incomplete = "ab";
        assert!(matches!(parser(test_input_incomplete), Err(Err::Error(_)) | Err(Err::Failure(_))));

        // Test input that does not match the first element
        let test_input_fail = "zbcdefghijk";
        assert!(matches!(parser(test_input_fail), Err(Err::Error(_)) | Err(Err::Failure(_))));
    }
}#[cfg(test)]
mod tests_llm_16_129_llm_16_129 {
    use crate::*;
    use crate::bytes::complete::tag;
    use crate::sequence::tuple;
    use crate::IResult;

    // Define a test function to use with `parse`
    fn dummy_parser(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
        tag("test")(input)
    }

    // Another test function with different output
    fn another_dummy_parser(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
        tag("another")(input)
    }

    // Now define the test for the `parse` function
    #[test]
    fn test_parse_success() {
        let mut parsers = tuple((dummy_parser, another_dummy_parser));
        let result = parsers.parse("testanother");
        assert_eq!(result, Ok(("", ("test", "another"))));
    }

    #[test]
    fn test_parse_partial() {
        let mut parsers = tuple((dummy_parser, another_dummy_parser));
        let result = parsers.parse("testanothermore");
        assert_eq!(result, Ok(("more", ("test", "another"))));
    }

    #[test]
    fn test_parse_failure() {
        let mut parsers = tuple((dummy_parser, another_dummy_parser));
        let result = parsers.parse("failtest");
        assert!(result.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_133_llm_16_133 {
    use crate::{
        error::{ErrorKind, ParseError},
        sequence::tuple,
        IResult, Err,
    };

    #[derive(Debug, PartialEq)]
    struct Error<'a> {
        input: &'a str,
        kind: ErrorKind,
    }

    impl<'a> ParseError<&'a str> for Error<'a> {
        fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
            Error { input, kind }
        }

        fn append(_: &'a str, _: ErrorKind, other: Self) -> Self {
            other
        }
    }

    fn parse_a(input: &str) -> IResult<&str, char, Error> {
        Ok((input, 'a'))
    }

    fn parse_b(input: &str) -> IResult<&str, char, Error> {
        Ok((input, 'b'))
    }

    fn parse_c(input: &str) -> IResult<&str, char, Error> {
        Ok((input, 'c'))
    }

    #[test]
    fn test_parse() {
        let mut parsers = tuple((parse_a, parse_b, parse_c));
        let input = "input";
        let expected = Ok(("input", ('a', 'b', 'c')));
        let result = parsers(input);
        assert_eq!(result, expected);
    }
}#[cfg(test)]
mod tests_llm_16_135 {
    use crate::{error::Error, error::ErrorKind, error::ParseError, sequence::tuple, IResult, Err};

    #[test]
    fn test_parse() {
        fn parse_a(input: &str) -> IResult<&str, char, Error<&str>> {
            if let Some(first) = input.chars().next() {
                Ok((&input[first.len_utf8()..], first))
            } else {
                Err(Err::Error(Error::from_error_kind(input, ErrorKind::Eof)))
            }
        }

        fn parse_b(input: &str) -> IResult<&str, char, Error<&str>> {
            let mut chars = input.chars();
            let _ = chars.next();
            if let Some(second) = chars.next() {
                Ok((&input[second.len_utf8()..], second))
            } else {
                Err(Err::Error(Error::from_error_kind(input, ErrorKind::Eof)))
            }
        }

        let mut parser = tuple((parse_a, parse_b));
        assert_eq!(parser("abc"), Ok(("c", ('a', 'b'))));
        assert_eq!(parser("a"), Err(Err::Error(Error::from_error_kind("a", ErrorKind::Eof))));
    }
}#[cfg(test)]
mod tests_llm_16_139_llm_16_139 {
    use crate::IResult;
    use crate::sequence::tuple;

    fn parser_a(input: &str) -> IResult<&str, &str> {
        Ok((input, "result_a"))
    }

    fn parser_b(input: &str) -> IResult<&str, &str> {
        Ok((input, "result_b"))
    }

    #[test]
    fn test_parse() {
        let mut combined_parser = tuple((parser_a, parser_b));
        let input = "Some input string";
        let expected = Ok((input, ("result_a", "result_b")));

        assert_eq!(combined_parser(input), expected);
    }
}#[cfg(test)]
mod tests_llm_16_141_llm_16_141 {
    use crate::{
        error::ParseError,
        sequence::tuple,
        internal::Parser,
        IResult,
        bytes::complete::tag,
    };

    #[test]
    fn test_parse_tuple() {
        fn parse_a(input: &str) -> IResult<&str, &str> {
            tag("a")(input)
        }

        fn parse_b(input: &str) -> IResult<&str, &str> {
            tag("b")(input)
        }

        fn parse_c(input: &str) -> IResult<&str, &str> {
            tag("c")(input)
        }

        let mut parser = tuple((parse_a, parse_b, parse_c));
        let input = "abc";
        let expected = Ok(("", ("a", "b", "c")));

        let result = parser.parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_tuple_incomplete() {
        fn parse_a(input: &str) -> IResult<&str, &str> {
            tag("a")(input)
        }

        fn parse_b(input: &str) -> IResult<&str, &str> {
            tag("b")(input)
        }

        fn parse_c(input: &str) -> IResult<&str, &str> {
            tag("c")(input)
        }

        let mut parser = tuple((parse_a, parse_b, parse_c));
        let input = "ab";
        assert!(parser.parse(input).is_err());
    }
}#[cfg(test)]
mod tests_llm_16_560_llm_16_560 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        sequence::delimited,
        bytes::complete::tag,
        IResult,
    };

    #[test]
    fn delimited_success_cases() {
        let mut parser = delimited(tag::<&str, _, Error<_>>("("), tag::<&str, _, Error<_>>("abc"), tag::<&str, _, Error<_>>(")"));

        assert_eq!(parser("(abc)"), Ok(("", "abc")));
        assert_eq!(parser("(abc)def"), Ok(("def", "abc")));
    }

    #[test]
    fn delimited_incomplete_case() {
        let mut parser = delimited(tag::<&str, _, Error<_>>("("), tag::<&str, _, Error<_>>("abc"), tag::<&str, _, Error<_>>(")"));

        assert_eq!(parser("("), Err(crate::Err::Error(Error::new("(", ErrorKind::Tag))));
    }

    #[test]
    fn delimited_error_cases() {
        let mut parser = delimited(tag::<&str, _, Error<_>>("("), tag::<&str, _, Error<_>>("abc"), tag::<&str, _, Error<_>>(")"));

        assert_eq!(parser("abc"), Err(crate::Err::Error(Error::new("abc", ErrorKind::Tag))));
        assert_eq!(parser(")abc("), Err(crate::Err::Error(Error::new(")abc(", ErrorKind::Tag))));
        assert_eq!(parser("def(abc)"), Err(crate::Err::Error(Error::new("def(abc)", ErrorKind::Tag))));
    }
}#[cfg(test)]
mod tests_llm_16_562 {
  use crate::{
    error::{Error, ErrorKind},
    sequence::preceded,
    bytes::complete::tag,
    IResult,
  };

  fn setup<'a>(input: &'a str) -> IResult<&'a str, &'a str, Error<&'a str>> {
    preceded(tag("abc"), tag("def"))(input)
  }

  #[test]
  fn test_preceded_success() {
    assert_eq!(setup("abcdef"), Ok(("", "def")));
    assert_eq!(setup("abcdefg"), Ok(("g", "def")));
  }

  #[test]
  fn test_preceded_incomplete() {
    assert_eq!(setup("abc"), Err(crate::Err::Error(Error::new("abc", ErrorKind::Tag))));
    assert_eq!(setup("abcde"), Err(crate::Err::Error(Error::new("de", ErrorKind::Tag))));
  }

  #[test]
  fn test_preceded_failure() {
    assert_eq!(setup("abxdef"), Err(crate::Err::Error(Error::new("abxdef", ErrorKind::Tag))));
    assert_eq!(setup("a"), Err(crate::Err::Error(Error::new("a", ErrorKind::Tag))));
    assert_eq!(setup(""), Err(crate::Err::Error(Error::new("", ErrorKind::Tag))));
  }
}#[cfg(test)]
mod tests_llm_16_563 {
    use crate::{
        error::{Error, ErrorKind},
        sequence::separated_pair,
        IResult,
        bytes::complete::tag,
    };

    #[test]
    fn test_separated_pair() {
        let mut parser = separated_pair(tag("abc"), tag("|"), tag("efg"));

        assert_eq!(parser("abc|efg"), Ok(("", ("abc", "efg"))));
        assert_eq!(parser("abc|efghij"), Ok(("hij", ("abc", "efg"))));
        assert_eq!(parser("abc|"), Err(crate::Err::Error(Error { input: "", code: ErrorKind::Tag })));
        assert_eq!(parser("|efg"), Err(crate::Err::Error(Error { input: "|efg", code: ErrorKind::Tag })));
        assert_eq!(parser("abc|abc"), Err(crate::Err::Error(Error { input: "abc", code: ErrorKind::Tag })));
        assert_eq!(parser(""), Err(crate::Err::Error(Error { input: "", code: ErrorKind::Tag })));
        assert_eq!(parser("123"), Err(crate::Err::Error(Error { input: "123", code: ErrorKind::Tag })));
    }
}#[cfg(test)]
mod tests_llm_16_564 {
    use crate::{
        error::{Error, ErrorKind},
        sequence::terminated,
        bytes::complete::tag,
        IResult,
    };

    #[test]
    fn test_terminated_success() {
        fn parse(input: &str) -> IResult<&str, &str, Error<&str>> {
            let mut parser = terminated(tag("abc"), tag("efg"));
            parser(input)
        }

        assert_eq!(parse("abcefg"), Ok(("", "abc")));
        assert_eq!(parse("abcefghij"), Ok(("hij", "abc")));
    }

    #[test]
    fn test_terminated_incomplete() {
        fn parse(input: &str) -> IResult<&str, &str, Error<&str>> {
            let mut parser = terminated(tag("abc"), tag("efg"));
            parser(input)
        }

        assert!(parse("abc").is_err());
    }

    #[test]
    fn test_terminated_error() {
        fn parse(input: &str) -> IResult<&str, &str, Error<&str>> {
            let mut parser = terminated(tag("abc"), tag("efg"));
            parser(input)
        }

        assert_eq!(parse(""), Err(crate::Err::Error(Error::new("", ErrorKind::Tag))));
        assert_eq!(parse("123"), Err(crate::Err::Error(Error::new("123", ErrorKind::Tag))));
    }
}