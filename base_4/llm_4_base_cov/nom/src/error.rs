//! Error management
//!
//! Parsers are generic over their error type, requiring that it implements
//! the `error::ParseError<Input>` trait.

use crate::internal::Parser;
use crate::lib::std::fmt;

#[cfg(feature = "alloc")]
use crate::alloc::borrow::ToOwned;

/// This trait must be implemented by the error type of a nom parser.
///
/// There are already implementations of it for `(Input, ErrorKind)`
/// and `VerboseError<Input>`.
///
/// It provides methods to create an error from some combinators,
/// and combine existing errors in combinators like `alt`.
pub trait ParseError<I>: Sized {
  /// Creates an error from the input position and an [ErrorKind]
  fn from_error_kind(input: I, kind: ErrorKind) -> Self;

  /// Combines an existing error with a new one created from the input
  /// position and an [ErrorKind]. This is useful when backtracking
  /// through a parse tree, accumulating error context on the way
  fn append(input: I, kind: ErrorKind, other: Self) -> Self;

  /// Creates an error from an input position and an expected character
  fn from_char(input: I, _: char) -> Self {
    Self::from_error_kind(input, ErrorKind::Char)
  }

  /// Combines two existing errors. This function is used to compare errors
  /// generated in various branches of `alt`.
  fn or(self, other: Self) -> Self {
    other
  }
}

/// This trait is required by the `context` combinator to add a static string
/// to an existing error
pub trait ContextError<I>: Sized {
  /// Creates a new error from an input position, a static string and an existing error.
  /// This is used mainly in the [context] combinator, to add user friendly information
  /// to errors when backtracking through a parse tree
  fn add_context(_input: I, _ctx: &'static str, other: Self) -> Self {
    other
  }
}

/// This trait is required by the `map_res` combinator to integrate
/// error types from external functions, like [std::str::FromStr]
pub trait FromExternalError<I, E> {
  /// Creates a new error from an input position, an [ErrorKind] indicating the
  /// wrapping parser, and an external error
  fn from_external_error(input: I, kind: ErrorKind, e: E) -> Self;
}

/// default error type, only contains the error' location and code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error<I> {
  /// position of the error in the input data
  pub input: I,
  /// nom error code
  pub code: ErrorKind,
}

impl<I> Error<I> {
  /// creates a new basic error
  pub fn new(input: I, code: ErrorKind) -> Error<I> {
    Error { input, code }
  }
}

impl<I> ParseError<I> for Error<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    Error { input, code: kind }
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

impl<I> ContextError<I> for Error<I> {}

impl<I, E> FromExternalError<I, E> for Error<I> {
  /// Create a new error from an input position and an external error
  fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
    Error { input, code: kind }
  }
}

/// The Display implementation allows the std::error::Error implementation
impl<I: fmt::Display> fmt::Display for Error<I> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "error {:?} at: {}", self.code, self.input)
  }
}

#[cfg(feature = "std")]
impl<I: fmt::Debug + fmt::Display> std::error::Error for Error<I> {}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl From<Error<&[u8]>> for Error<crate::lib::std::vec::Vec<u8>> {
  fn from(value: Error<&[u8]>) -> Self {
    Error {
      input: value.input.to_owned(),
      code: value.code,
    }
  }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl From<Error<&str>> for Error<crate::lib::std::string::String> {
  fn from(value: Error<&str>) -> Self {
    Error {
      input: value.input.to_owned(),
      code: value.code,
    }
  }
}

// for backward compatibility, keep those trait implementations
// for the previously used error type
impl<I> ParseError<I> for (I, ErrorKind) {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    (input, kind)
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

impl<I> ContextError<I> for (I, ErrorKind) {}

impl<I, E> FromExternalError<I, E> for (I, ErrorKind) {
  fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
    (input, kind)
  }
}

impl<I> ParseError<I> for () {
  fn from_error_kind(_: I, _: ErrorKind) -> Self {}

  fn append(_: I, _: ErrorKind, _: Self) -> Self {}
}

impl<I> ContextError<I> for () {}

impl<I, E> FromExternalError<I, E> for () {
  fn from_external_error(_input: I, _kind: ErrorKind, _e: E) -> Self {}
}

/// Creates an error from the input position and an [ErrorKind]
pub fn make_error<I, E: ParseError<I>>(input: I, kind: ErrorKind) -> E {
  E::from_error_kind(input, kind)
}

/// Combines an existing error with a new one created from the input
/// position and an [ErrorKind]. This is useful when backtracking
/// through a parse tree, accumulating error context on the way
pub fn append_error<I, E: ParseError<I>>(input: I, kind: ErrorKind, other: E) -> E {
  E::append(input, kind, other)
}

/// This error type accumulates errors and their position when backtracking
/// through a parse tree. With some post processing (cf `examples/json.rs`),
/// it can be used to display user friendly error messages
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerboseError<I> {
  /// List of errors accumulated by `VerboseError`, containing the affected
  /// part of input data, and some context
  pub errors: crate::lib::std::vec::Vec<(I, VerboseErrorKind)>,
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Error context for `VerboseError`
pub enum VerboseErrorKind {
  /// Static string added by the `context` function
  Context(&'static str),
  /// Indicates which character was expected by the `char` function
  Char(char),
  /// Error kind given by various nom parsers
  Nom(ErrorKind),
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl<I> ParseError<I> for VerboseError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    VerboseError {
      errors: vec![(input, VerboseErrorKind::Nom(kind))],
    }
  }

  fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
    other.errors.push((input, VerboseErrorKind::Nom(kind)));
    other
  }

  fn from_char(input: I, c: char) -> Self {
    VerboseError {
      errors: vec![(input, VerboseErrorKind::Char(c))],
    }
  }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl<I> ContextError<I> for VerboseError<I> {
  fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
    other.errors.push((input, VerboseErrorKind::Context(ctx)));
    other
  }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl<I, E> FromExternalError<I, E> for VerboseError<I> {
  /// Create a new error from an input position and an external error
  fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
    Self::from_error_kind(input, kind)
  }
}

#[cfg(feature = "alloc")]
impl<I: fmt::Display> fmt::Display for VerboseError<I> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "Parse error:")?;
    for (input, error) in &self.errors {
      match error {
        VerboseErrorKind::Nom(e) => writeln!(f, "{:?} at: {}", e, input)?,
        VerboseErrorKind::Char(c) => writeln!(f, "expected '{}' at: {}", c, input)?,
        VerboseErrorKind::Context(s) => writeln!(f, "in section '{}', at: {}", s, input)?,
      }
    }

    Ok(())
  }
}

#[cfg(feature = "std")]
impl<I: fmt::Debug + fmt::Display> std::error::Error for VerboseError<I> {}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl From<VerboseError<&[u8]>> for VerboseError<crate::lib::std::vec::Vec<u8>> {
  fn from(value: VerboseError<&[u8]>) -> Self {
    VerboseError {
      errors: value
        .errors
        .into_iter()
        .map(|(i, e)| (i.to_owned(), e))
        .collect(),
    }
  }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl From<VerboseError<&str>> for VerboseError<crate::lib::std::string::String> {
  fn from(value: VerboseError<&str>) -> Self {
    VerboseError {
      errors: value
        .errors
        .into_iter()
        .map(|(i, e)| (i.to_owned(), e))
        .collect(),
    }
  }
}

use crate::internal::{Err, IResult};

/// Create a new error from an input position, a static string and an existing error.
/// This is used mainly in the [context] combinator, to add user friendly information
/// to errors when backtracking through a parse tree
pub fn context<I: Clone, E: ContextError<I>, F, O>(
  context: &'static str,
  mut f: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
  F: Parser<I, Output = O, Error = E>,
{
  move |i: I| match f.parse(i.clone()) {
    Ok(o) => Ok(o),
    Err(Err::Incomplete(i)) => Err(Err::Incomplete(i)),
    Err(Err::Error(e)) => Err(Err::Error(E::add_context(i, context, e))),
    Err(Err::Failure(e)) => Err(Err::Failure(E::add_context(i, context, e))),
  }
}

/// Transforms a `VerboseError` into a trace with input position information
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn convert_error<I: core::ops::Deref<Target = str>>(
  input: I,
  e: VerboseError<I>,
) -> crate::lib::std::string::String {
  use crate::lib::std::fmt::Write;
  use crate::traits::Offset;

  let mut result = crate::lib::std::string::String::new();

  for (i, (substring, kind)) in e.errors.iter().enumerate() {
    let offset = input.offset(substring);

    if input.is_empty() {
      match kind {
        VerboseErrorKind::Char(c) => {
          write!(&mut result, "{}: expected '{}', got empty input\n\n", i, c)
        }
        VerboseErrorKind::Context(s) => write!(&mut result, "{}: in {}, got empty input\n\n", i, s),
        VerboseErrorKind::Nom(e) => write!(&mut result, "{}: in {:?}, got empty input\n\n", i, e),
      }
    } else {
      let prefix = &input.as_bytes()[..offset];

      // Count the number of newlines in the first `offset` bytes of input
      let line_number = prefix.iter().filter(|&&b| b == b'\n').count() + 1;

      // Find the line that includes the subslice:
      // Find the *last* newline before the substring starts
      let line_begin = prefix
        .iter()
        .rev()
        .position(|&b| b == b'\n')
        .map(|pos| offset - pos)
        .unwrap_or(0);

      // Find the full line after that newline
      let line = input[line_begin..]
        .lines()
        .next()
        .unwrap_or(&input[line_begin..])
        .trim_end();

      // The (1-indexed) column number is the offset of our substring into that line
      let column_number = line.offset(substring) + 1;

      match kind {
        VerboseErrorKind::Char(c) => {
          if let Some(actual) = substring.chars().next() {
            write!(
              &mut result,
              "{i}: at line {line_number}:\n\
               {line}\n\
               {caret:>column$}\n\
               expected '{expected}', found {actual}\n\n",
              i = i,
              line_number = line_number,
              line = line,
              caret = '^',
              column = column_number,
              expected = c,
              actual = actual,
            )
          } else {
            write!(
              &mut result,
              "{i}: at line {line_number}:\n\
               {line}\n\
               {caret:>column$}\n\
               expected '{expected}', got end of input\n\n",
              i = i,
              line_number = line_number,
              line = line,
              caret = '^',
              column = column_number,
              expected = c,
            )
          }
        }
        VerboseErrorKind::Context(s) => write!(
          &mut result,
          "{i}: at line {line_number}, in {context}:\n\
             {line}\n\
             {caret:>column$}\n\n",
          i = i,
          line_number = line_number,
          context = s,
          line = line,
          caret = '^',
          column = column_number,
        ),
        VerboseErrorKind::Nom(e) => write!(
          &mut result,
          "{i}: at line {line_number}, in {nom_err:?}:\n\
             {line}\n\
             {caret:>column$}\n\n",
          i = i,
          line_number = line_number,
          nom_err = e,
          line = line,
          caret = '^',
          column = column_number,
        ),
      }
    }
    // Because `write!` to a `String` is infallible, this `unwrap` is fine.
    .unwrap();
  }

  result
}

/// Indicates which parser returned an error
#[rustfmt::skip]
#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
#[allow(deprecated,missing_docs)]
pub enum ErrorKind {
  Tag,
  MapRes,
  MapOpt,
  Alt,
  IsNot,
  IsA,
  SeparatedList,
  SeparatedNonEmptyList,
  Many0,
  Many1,
  ManyTill,
  Count,
  TakeUntil,
  LengthValue,
  TagClosure,
  Alpha,
  Digit,
  HexDigit,
  OctDigit,
  AlphaNumeric,
  Space,
  MultiSpace,
  LengthValueFn,
  Eof,
  Switch,
  TagBits,
  OneOf,
  NoneOf,
  Char,
  CrLf,
  RegexpMatch,
  RegexpMatches,
  RegexpFind,
  RegexpCapture,
  RegexpCaptures,
  TakeWhile1,
  Complete,
  Fix,
  Escaped,
  EscapedTransform,
  NonEmpty,
  ManyMN,
  Not,
  Permutation,
  Verify,
  TakeTill1,
  TakeWhileMN,
  TooLarge,
  Many0Count,
  Many1Count,
  Float,
  Satisfy,
  Fail,
  Many,
  Fold,
}

#[rustfmt::skip]
#[allow(deprecated)]
/// Converts an ErrorKind to a number
pub fn error_to_u32(e: &ErrorKind) -> u32 {
  match *e {
    ErrorKind::Tag                       => 1,
    ErrorKind::MapRes                    => 2,
    ErrorKind::MapOpt                    => 3,
    ErrorKind::Alt                       => 4,
    ErrorKind::IsNot                     => 5,
    ErrorKind::IsA                       => 6,
    ErrorKind::SeparatedList             => 7,
    ErrorKind::SeparatedNonEmptyList     => 8,
    ErrorKind::Many1                     => 9,
    ErrorKind::Count                     => 10,
    ErrorKind::TakeUntil                 => 12,
    ErrorKind::LengthValue               => 15,
    ErrorKind::TagClosure                => 16,
    ErrorKind::Alpha                     => 17,
    ErrorKind::Digit                     => 18,
    ErrorKind::AlphaNumeric              => 19,
    ErrorKind::Space                     => 20,
    ErrorKind::MultiSpace                => 21,
    ErrorKind::LengthValueFn             => 22,
    ErrorKind::Eof                       => 23,
    ErrorKind::Switch                    => 27,
    ErrorKind::TagBits                   => 28,
    ErrorKind::OneOf                     => 29,
    ErrorKind::NoneOf                    => 30,
    ErrorKind::Char                      => 40,
    ErrorKind::CrLf                      => 41,
    ErrorKind::RegexpMatch               => 42,
    ErrorKind::RegexpMatches             => 43,
    ErrorKind::RegexpFind                => 44,
    ErrorKind::RegexpCapture             => 45,
    ErrorKind::RegexpCaptures            => 46,
    ErrorKind::TakeWhile1                => 47,
    ErrorKind::Complete                  => 48,
    ErrorKind::Fix                       => 49,
    ErrorKind::Escaped                   => 50,
    ErrorKind::EscapedTransform          => 51,
    ErrorKind::NonEmpty                  => 56,
    ErrorKind::ManyMN                    => 57,
    ErrorKind::HexDigit                  => 59,
    ErrorKind::OctDigit                  => 61,
    ErrorKind::Many0                     => 62,
    ErrorKind::Not                       => 63,
    ErrorKind::Permutation               => 64,
    ErrorKind::ManyTill                  => 65,
    ErrorKind::Verify                    => 66,
    ErrorKind::TakeTill1                 => 67,
    ErrorKind::TakeWhileMN               => 69,
    ErrorKind::TooLarge                  => 70,
    ErrorKind::Many0Count                => 71,
    ErrorKind::Many1Count                => 72,
    ErrorKind::Float                     => 73,
    ErrorKind::Satisfy                   => 74,
    ErrorKind::Fail                      => 75,
    ErrorKind::Many                      => 76,
    ErrorKind::Fold                      => 77,
  }
}

impl ErrorKind {
  #[rustfmt::skip]
  #[allow(deprecated)]
  /// Converts an ErrorKind to a text description
  pub fn description(&self) -> &str {
    match *self {
      ErrorKind::Tag                       => "Tag",
      ErrorKind::MapRes                    => "Map on Result",
      ErrorKind::MapOpt                    => "Map on Option",
      ErrorKind::Alt                       => "Alternative",
      ErrorKind::IsNot                     => "IsNot",
      ErrorKind::IsA                       => "IsA",
      ErrorKind::SeparatedList             => "Separated list",
      ErrorKind::SeparatedNonEmptyList     => "Separated non empty list",
      ErrorKind::Many0                     => "Many0",
      ErrorKind::Many1                     => "Many1",
      ErrorKind::Count                     => "Count",
      ErrorKind::TakeUntil                 => "Take until",
      ErrorKind::LengthValue               => "Length followed by value",
      ErrorKind::TagClosure                => "Tag closure",
      ErrorKind::Alpha                     => "Alphabetic",
      ErrorKind::Digit                     => "Digit",
      ErrorKind::AlphaNumeric              => "AlphaNumeric",
      ErrorKind::Space                     => "Space",
      ErrorKind::MultiSpace                => "Multiple spaces",
      ErrorKind::LengthValueFn             => "LengthValueFn",
      ErrorKind::Eof                       => "End of file",
      ErrorKind::Switch                    => "Switch",
      ErrorKind::TagBits                   => "Tag on bitstream",
      ErrorKind::OneOf                     => "OneOf",
      ErrorKind::NoneOf                    => "NoneOf",
      ErrorKind::Char                      => "Char",
      ErrorKind::CrLf                      => "CrLf",
      ErrorKind::RegexpMatch               => "RegexpMatch",
      ErrorKind::RegexpMatches             => "RegexpMatches",
      ErrorKind::RegexpFind                => "RegexpFind",
      ErrorKind::RegexpCapture             => "RegexpCapture",
      ErrorKind::RegexpCaptures            => "RegexpCaptures",
      ErrorKind::TakeWhile1                => "TakeWhile1",
      ErrorKind::Complete                  => "Complete",
      ErrorKind::Fix                       => "Fix",
      ErrorKind::Escaped                   => "Escaped",
      ErrorKind::EscapedTransform          => "EscapedTransform",
      ErrorKind::NonEmpty                  => "NonEmpty",
      ErrorKind::ManyMN                    => "Many(m, n)",
      ErrorKind::HexDigit                  => "Hexadecimal Digit",
      ErrorKind::OctDigit                  => "Octal digit",
      ErrorKind::Not                       => "Negation",
      ErrorKind::Permutation               => "Permutation",
      ErrorKind::ManyTill                  => "ManyTill",
      ErrorKind::Verify                    => "predicate verification",
      ErrorKind::TakeTill1                 => "TakeTill1",
      ErrorKind::TakeWhileMN               => "TakeWhileMN",
      ErrorKind::TooLarge                  => "Needed data size is too large",
      ErrorKind::Many0Count                => "Count occurrence of >=0 patterns",
      ErrorKind::Many1Count                => "Count occurrence of >=1 patterns",
      ErrorKind::Float                     => "Float",
      ErrorKind::Satisfy                   => "Satisfy",
      ErrorKind::Fail                      => "Fail",
      ErrorKind::Many                      => "Many",
      ErrorKind::Fold                      => "Fold",
    }
  }
}

/// Creates a parse error from a `nom::ErrorKind`
/// and the position in the input
#[allow(unused_variables)]
#[macro_export(local_inner_macros)]
macro_rules! error_position(
  ($input:expr, $code:expr) => ({
    $crate::error::make_error($input, $code)
  });
);

/// Creates a parse error from a `nom::ErrorKind`,
/// the position in the input and the next error in
/// the parsing tree
#[allow(unused_variables)]
#[macro_export(local_inner_macros)]
macro_rules! error_node_position(
  ($input:expr, $code:expr, $next:expr) => ({
    $crate::error::append_error($input, $code, $next)
  });
);

/// Prints a message and the input if the parser fails.
///
/// The message prints the `Error` or `Incomplete`
/// and the parser's calling code.
///
/// It also displays the input in hexdump format
///
/// ```rust
/// use nom::{IResult, error::dbg_dmp, bytes::complete::tag};
///
/// fn f(i: &[u8]) -> IResult<&[u8], &[u8]> {
///   dbg_dmp(tag("abcd"), "tag")(i)
/// }
///
///   let a = &b"efghijkl"[..];
///
/// // Will print the following message:
/// // Error(Position(0, [101, 102, 103, 104, 105, 106, 107, 108])) at l.5 by ' tag ! ( "abcd" ) '
/// // 00000000        65 66 67 68 69 6a 6b 6c         efghijkl
/// f(a);
/// ```
#[cfg(feature = "std")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "std")))]
pub fn dbg_dmp<'a, F, O, E: std::fmt::Debug>(
  f: F,
  context: &'static str,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], O, E>
where
  F: Fn(&'a [u8]) -> IResult<&'a [u8], O, E>,
{
  use crate::HexDisplay;
  move |i: &'a [u8]| match f(i) {
    Err(e) => {
      println!("{}: Error({:?}) at:\n{}", context, e, i.to_hex(8));
      Err(e)
    }
    a => a,
  }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
  use super::*;
  use crate::character::complete::char;

  #[test]
  fn convert_error_panic() {
    let input = "";

    let _result: IResult<_, _, VerboseError<&str>> = char('x')(input);
  }
}

/*
#[cfg(feature = "alloc")]
use lib::std::{vec::Vec, collections::HashMap};

#[cfg(feature = "std")]
use lib::std::hash::Hash;

#[cfg(feature = "std")]
pub fn add_error_pattern<'a, I: Clone + Hash + Eq, O, E: Clone + Hash + Eq>(
  h: &mut HashMap<VerboseError<I>, &'a str>,
  e: VerboseError<I>,
  message: &'a str,
) -> bool {
  h.insert(e, message);
  true
}

pub fn slice_to_offsets(input: &[u8], s: &[u8]) -> (usize, usize) {
  let start = input.as_ptr();
  let off1 = s.as_ptr() as usize - start as usize;
  let off2 = off1 + s.len();
  (off1, off2)
}

#[cfg(feature = "std")]
pub fn prepare_errors<O, E: Clone>(input: &[u8], e: VerboseError<&[u8]>) -> Option<Vec<(ErrorKind, usize, usize)>> {
  let mut v: Vec<(ErrorKind, usize, usize)> = Vec::new();

  for (p, kind) in e.errors.drain(..) {
    let (o1, o2) = slice_to_offsets(input, p);
    v.push((kind, o1, o2));
  }

  v.reverse();
  Some(v)
}

#[cfg(feature = "std")]
pub fn print_error<O, E: Clone>(input: &[u8], res: VerboseError<&[u8]>) {
  if let Some(v) = prepare_errors(input, res) {
    let colors = generate_colors(&v);
    println!("parser codes: {}", print_codes(&colors, &HashMap::new()));
    println!("{}", print_offsets(input, 0, &v));
  } else {
    println!("not an error");
  }
}

#[cfg(feature = "std")]
pub fn generate_colors<E>(v: &[(ErrorKind, usize, usize)]) -> HashMap<u32, u8> {
  let mut h: HashMap<u32, u8> = HashMap::new();
  let mut color = 0;

  for &(ref c, _, _) in v.iter() {
    h.insert(error_to_u32(c), color + 31);
    color = color + 1 % 7;
  }

  h
}

pub fn code_from_offset(v: &[(ErrorKind, usize, usize)], offset: usize) -> Option<u32> {
  let mut acc: Option<(u32, usize, usize)> = None;
  for &(ref ek, s, e) in v.iter() {
    let c = error_to_u32(ek);
    if s <= offset && offset <= e {
      if let Some((_, start, end)) = acc {
        if start <= s && e <= end {
          acc = Some((c, s, e));
        }
      } else {
        acc = Some((c, s, e));
      }
    }
  }
  if let Some((code, _, _)) = acc {
    return Some(code);
  } else {
    return None;
  }
}

#[cfg(feature = "alloc")]
pub fn reset_color(v: &mut Vec<u8>) {
  v.push(0x1B);
  v.push(b'[');
  v.push(0);
  v.push(b'm');
}

#[cfg(feature = "alloc")]
pub fn write_color(v: &mut Vec<u8>, color: u8) {
  v.push(0x1B);
  v.push(b'[');
  v.push(1);
  v.push(b';');
  let s = color.to_string();
  let bytes = s.as_bytes();
  v.extend(bytes.iter().cloned());
  v.push(b'm');
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "cargo-clippy", allow(implicit_hasher))]
pub fn print_codes(colors: &HashMap<u32, u8>, names: &HashMap<u32, &str>) -> String {
  let mut v = Vec::new();
  for (code, &color) in colors {
    if let Some(&s) = names.get(code) {
      let bytes = s.as_bytes();
      write_color(&mut v, color);
      v.extend(bytes.iter().cloned());
    } else {
      let s = code.to_string();
      let bytes = s.as_bytes();
      write_color(&mut v, color);
      v.extend(bytes.iter().cloned());
    }
    reset_color(&mut v);
    v.push(b' ');
  }
  reset_color(&mut v);

  String::from_utf8_lossy(&v[..]).into_owned()
}

#[cfg(feature = "std")]
pub fn print_offsets(input: &[u8], from: usize, offsets: &[(ErrorKind, usize, usize)]) -> String {
  let mut v = Vec::with_capacity(input.len() * 3);
  let mut i = from;
  let chunk_size = 8;
  let mut current_code: Option<u32> = None;
  let mut current_code2: Option<u32> = None;

  let colors = generate_colors(&offsets);

  for chunk in input.chunks(chunk_size) {
    let s = format!("{:08x}", i);
    for &ch in s.as_bytes().iter() {
      v.push(ch);
    }
    v.push(b'\t');

    let mut k = i;
    let mut l = i;
    for &byte in chunk {
      if let Some(code) = code_from_offset(&offsets, k) {
        if let Some(current) = current_code {
          if current != code {
            reset_color(&mut v);
            current_code = Some(code);
            if let Some(&color) = colors.get(&code) {
              write_color(&mut v, color);
            }
          }
        } else {
          current_code = Some(code);
          if let Some(&color) = colors.get(&code) {
            write_color(&mut v, color);
          }
        }
      }
      v.push(CHARS[(byte >> 4) as usize]);
      v.push(CHARS[(byte & 0xf) as usize]);
      v.push(b' ');
      k = k + 1;
    }

    reset_color(&mut v);

    if chunk_size > chunk.len() {
      for _ in 0..(chunk_size - chunk.len()) {
        v.push(b' ');
        v.push(b' ');
        v.push(b' ');
      }
    }
    v.push(b'\t');

    for &byte in chunk {
      if let Some(code) = code_from_offset(&offsets, l) {
        if let Some(current) = current_code2 {
          if current != code {
            reset_color(&mut v);
            current_code2 = Some(code);
            if let Some(&color) = colors.get(&code) {
              write_color(&mut v, color);
            }
          }
        } else {
          current_code2 = Some(code);
          if let Some(&color) = colors.get(&code) {
            write_color(&mut v, color);
          }
        }
      }
      if (byte >= 32 && byte <= 126) || byte >= 128 {
        v.push(byte);
      } else {
        v.push(b'.');
      }
      l = l + 1;
    }
    reset_color(&mut v);

    v.push(b'\n');
    i = i + chunk_size;
  }

  String::from_utf8_lossy(&v[..]).into_owned()
}
*/
#[cfg(test)]
mod tests_llm_16_79 {
    use super::*;

use crate::*;
    use crate::error::ErrorKind;
    use crate::error::FromExternalError;
    
    #[derive(Debug, PartialEq)]
    struct CustomError;

    #[test]
    fn from_external_error_tag_test() {
        let input = ();
        let error = CustomError;
        let error_kind = ErrorKind::Tag;

        let parsed_error = <() as FromExternalError<(), CustomError>>::from_external_error(input, error_kind, error);
        // You can implement checks here depending on the behavior of from_external_error
        // Example:
        // assert_eq!(parsed_error, ExpectedErrorType::new(input, error_kind));
    }

    #[test]
    fn from_external_error_eof_test() {
        let input = ();
        let error = CustomError;
        let error_kind = ErrorKind::Eof;

        let parsed_error = <() as FromExternalError<(), CustomError>>::from_external_error(input, error_kind, error);
        // You can implement checks here depending on the behavior of from_external_error
        // Example:
        // assert_eq!(parsed_error, ExpectedErrorType::new(input, error_kind));
    }

    // Add more tests for different ErrorKinds if necessary
}#[cfg(test)]
mod tests_llm_16_80_llm_16_80 {
    use crate::error::{ErrorKind, ParseError};

    #[derive(Debug, Clone, PartialEq)]
    struct DummyError;

    impl<I> ParseError<I> for DummyError {
        fn from_error_kind(_: I, _: ErrorKind) -> Self {
            DummyError
        }

        fn append(_: I, _: ErrorKind, _: Self) -> Self {
            DummyError
        }
    }

    #[test]
    fn append_error() {
        let input = ();
        let error_kind = ErrorKind::Tag;
        let initial_error = DummyError;
        let appended_error = DummyError::append(input, error_kind, initial_error);
        // Define your assertions here, for example:
        assert_eq!(appended_error, DummyError);
    }
}#[cfg(test)]
mod tests_llm_16_81 {
    use super::*; // Adjust the import to the correct path where `from_error_kind` is located

use crate::*;
    use crate::error::ErrorKind;
    use crate::error::ParseError;

    #[test]
    fn from_error_kind_test() {
        // Since `from_error_kind` is a member of the `ParseError` trait,
        // we need a type that implements `ParseError` to use it.
        // Here, we're assuming `()` implements `ParseError`, as per the provided path.
        // The actual type will likely be different, like `VerboseError<I>` or similar.
        struct DummyInput;
        let input = DummyInput; // Placeholder for the input type, you need to replace DummyInput

        // Test for a specific ErrorKind value, e.g., `ErrorKind::Tag`
        let error_kind = ErrorKind::Tag;
        let error = <() as ParseError<DummyInput>>::from_error_kind(input, error_kind); 

        // Since there is no behavior specified for the function, we cannot make assertions
        // on the side effects or return values. The function is a stub.
        // If additional logic is added to the function, you should test for that logic here.
    }
}#[cfg(test)]
mod tests_llm_16_146 {
    use super::*;

use crate::*;
    use crate::error::{ErrorKind, FromExternalError};
    
    struct MockExternalError;

    #[test]
    fn test_from_external_error() {
        let input = "test input";
        let kind = ErrorKind::Tag;
        let external_error = MockExternalError;

        let result = <(&str, ErrorKind) as FromExternalError<&str, MockExternalError>>::from_external_error(input, kind, external_error);
        assert_eq!(result, (input, kind));
    }
}#[cfg(test)]
mod tests_llm_16_147_llm_16_147 {
    use super::*;

use crate::*;

    #[derive(Debug, PartialEq)]
    struct DummyError;

    impl<I> error::ParseError<I> for DummyError {
        fn from_error_kind(_input: I, _kind: error::ErrorKind) -> Self {
            DummyError
        }

        fn append(_input: I, _kind: error::ErrorKind, _other: Self) -> Self {
            DummyError
        }
    }

    #[test]
    fn test_append_error_kind() {
        let error_kind = error::ErrorKind::Tag;
        let error = DummyError;
        let appended = DummyError::append("", error_kind, error);
        assert_eq!(appended, DummyError);
    }
}#[cfg(test)]
mod tests_llm_16_148 {
    use super::*;

use crate::*;

    #[test]
    fn from_error_kind_test() {
        let input = "test input";
        let kind = error::ErrorKind::Tag;

        let result_error = <(&str, error::ErrorKind) as error::ParseError<&str>>::from_error_kind(input, kind);

        assert_eq!(result_error, (input, error::ErrorKind::Tag));
    }
}#[cfg(test)]
mod tests_llm_16_192 {
    use super::*;

use crate::*;
    use crate::error::{Error, ErrorKind, FromExternalError};

    #[derive(Debug, PartialEq)]
    struct ExternalError;

    #[test]
    fn from_external_error_with_str_input() {
        let input = "input data";
        let kind = ErrorKind::Alt;
        let external_error = ExternalError;

        let error = Error::from_external_error(input, kind, external_error);

        assert_eq!(error.input, "input data");
        assert_eq!(error.code, ErrorKind::Alt);
    }

    #[test]
    fn from_external_error_with_bytes_input() {
        let input = b"input data";
        let kind = ErrorKind::Tag;
        let external_error = ExternalError;

        let error = Error::from_external_error(input.as_ref(), kind, external_error);

        assert_eq!(error.input, b"input data".as_ref());
        assert_eq!(error.code, ErrorKind::Tag);
    }

    #[test]
    fn from_external_error_with_vec_input() {
        let input = vec![0, 1, 2, 3];
        let kind = ErrorKind::Count;
        let external_error = ExternalError;

        let error = Error::from_external_error(input.clone(), kind, external_error);

        assert_eq!(error.input, input);
        assert_eq!(error.code, ErrorKind::Count);
    }
}#[cfg(test)]
mod tests_llm_16_193_llm_16_193 {
  use super::*;

use crate::*;

  #[test]
  fn error_append() {
    let input1 = &b"some input"[..];
    let error1 = Error::from_error_kind(input1, ErrorKind::Tag);
    let input2 = &b"some other input"[..];
    let error2 = Error::from_error_kind(input2, ErrorKind::MapRes);

    let appended_error = Error::append(input2, ErrorKind::MapRes, error1.clone());

    assert_eq!(appended_error.input, error1.input);
    assert_eq!(appended_error.code, error1.code);
  }
}#[cfg(test)]
mod tests_llm_16_194 {
    use super::*;

use crate::*;

    #[test]
    fn test_from_error_kind() {
        let input = "some input data";
        let kind = ErrorKind::Tag;

        let error = Error::from_error_kind(input, kind);

        assert_eq!(error.input, "some input data");
        assert_eq!(error.code, ErrorKind::Tag);
    }
}#[cfg(test)]
mod tests_llm_16_196_llm_16_196 {
    use crate::error::{Error, ErrorKind};
    use std::convert::From;

    #[test]
    fn test_from_str_error_to_string_error() {
        let input = "some input";
        let error_kind = ErrorKind::Tag;
        let str_error = Error {
            input,
            code: error_kind,
        };
        let string_error: Error<String> = Error::from(str_error);

        assert_eq!(string_error.input, input.to_owned());
        assert_eq!(string_error.code, error_kind);
    }
}#[cfg(test)]
mod tests_llm_16_197 {
    use super::*;

use crate::*;

    #[test]
    fn from_byte_slice_to_vec_error() {
        let byte_slice_error = Error {
            input: &[0x01, 0x02, 0x03][..],
            code: ErrorKind::Tag,
        };
        let vec_error: Error<Vec<u8>> = byte_slice_error.into();

        assert_eq!(vec_error.input, vec![0x01, 0x02, 0x03]);
        assert_eq!(vec_error.code, ErrorKind::Tag);
    }
}#[cfg(test)]
mod tests_llm_16_199 {
    use super::*;

use crate::*;
    use crate::error::{ContextError, VerboseError, VerboseErrorKind};

    #[test]
    fn test_add_context() {
        let input = "my input";
        let context = "my context";
        let mut err = VerboseError::from_error_kind(input, crate::error::ErrorKind::Tag);
        err = VerboseError::add_context(input, context, err);
        
        let expected = VerboseError {
            errors: vec![
                (input, VerboseErrorKind::Nom(crate::error::ErrorKind::Tag)),
                (input, VerboseErrorKind::Context(context)),
            ],
        };

        assert_eq!(err, expected);
    }
}#[cfg(test)]
mod tests_llm_16_200 {
    use super::*;

use crate::*;
    use crate::error::{ErrorKind, VerboseError, FromExternalError};

    #[test]
    fn test_from_external_error() {
        let input = "test input";
        let kind = ErrorKind::Tag;
        let external_error = "External Error";
        let verbose_error: VerboseError<&str> = VerboseError::from_external_error(input, kind, external_error);

        assert_eq!(verbose_error.errors.len(), 1);
        match verbose_error.errors.first() {
            Some((i, VerboseErrorKind::Nom(k))) => {
                assert_eq!(i, &input);
                assert_eq!(*k, kind);
            },
            _ => panic!("ErrorKind::Nom expected"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_201 {
    use super::*;

use crate::*;

    #[test]
    fn test_append() {
        // You might need to adjust these types according to how they are defined in your crate
        let input = "some input";
        let kind = ErrorKind::Tag;

        let mut original_error = VerboseError::from_error_kind(input, ErrorKind::Alt);
        let appended_error = VerboseError::append(input, kind, original_error.clone());

        assert_eq!(appended_error.errors.len(), original_error.errors.len() + 1);
        assert!(appended_error.errors.contains(&(input, VerboseErrorKind::Nom(kind))));
    }
}#[cfg(test)]
mod tests_llm_16_202 {
    use super::*; // Assuming `from_char` and `VerboseError` are in the same module.

use crate::*;
    use crate::error::{ParseError, VerboseError, VerboseErrorKind}; // Adjust crate path as necessary.

    #[test]
    fn test_from_char() {
        let input = "my input";
        let character = 'a';
        let error = VerboseError::from_char(input, character);

        assert_eq!(error.errors.len(), 1);
        assert_eq!(error.errors[0].0, input);
        assert_eq!(error.errors[0].1, VerboseErrorKind::Char(character));
    }
}#[cfg(test)]
mod tests_llm_16_203 {
    use super::*;

use crate::*;
    use crate::error::{ErrorKind, ParseError, VerboseError, VerboseErrorKind};

    #[test]
    fn test_from_error_kind() {
        let input = &b"some input data"[..];
        let error_kind = ErrorKind::Tag;
        let verbose_error: VerboseError<&[u8]> = VerboseError::from_error_kind(input, error_kind);

        assert_eq!(
            verbose_error.errors,
            vec![(input, VerboseErrorKind::Nom(ErrorKind::Tag))]
        );
    }
}#[cfg(test)]
mod tests_llm_16_205_llm_16_205 {
    use crate::error::VerboseError;
    use crate::error::VerboseErrorKind;
    use std::convert::From;
    
    #[test]
    fn test_from_str_to_string_error() {
        let str_error = VerboseError {
            errors: vec![
                ("an error occurred here", VerboseErrorKind::Context("an error")),
                ("another error occurred here", VerboseErrorKind::Char('t')),
            ],
        };

        let str_error_clone = str_error.clone();
        
        let string_error: VerboseError<String> = VerboseError::from(str_error_clone);
        
        assert_eq!(str_error.errors.len(), string_error.errors.len());
        for ((str_input, str_kind), (string_input, string_kind)) in str_error.errors.iter().zip(string_error.errors.iter()) {
            assert_eq!(str_input.to_string(), *string_input);
            assert_eq!(str_kind, string_kind);
        }
    }
}#[cfg(test)]
mod tests_llm_16_206_llm_16_206 {
    use super::*;

use crate::*;
    use crate::error::{ErrorKind, VerboseError, VerboseErrorKind};

    #[test]
    fn test_from_verbose_error_for_slice_to_vec() {
        // Given
        let input_slice: &[u8] = &[b'a', b'b', b'c']; // Specify type to &[u8] instead of &[u8; 3]
        let error_slice = VerboseError::from_error_kind(input_slice, ErrorKind::Tag);
        let expected_vec: Vec<u8> = Vec::from(input_slice);
        // When
        let error_vec: VerboseError<Vec<u8>> = VerboseError::from(error_slice);
        // Then
        assert_eq!(error_vec.errors.len(), 1);
        assert!(matches!(error_vec.errors[0].1, VerboseErrorKind::Nom(ErrorKind::Tag)), "Error kind mismatch");
        assert_eq!(&error_vec.errors[0].0[..], input_slice, "Error input mismatch");
    }
}#[cfg(test)]
mod tests_llm_16_412 {
  use super::*;

use crate::*;

  #[test]
  fn error_new_test() {
    let input = &[0xFF, 0xAA, 0xBB];
    let error_kind = ErrorKind::Tag;
    let error = Error::new(input, error_kind);
    
    assert_eq!(error.input, input);
    assert_eq!(error.code, ErrorKind::Tag);
  }
}#[cfg(test)]
mod tests_llm_16_413_llm_16_413 {
    use crate::error::ErrorKind;

    #[test]
    fn error_kind_description() {
        assert_eq!(ErrorKind::Tag.description(), "Tag");
        assert_eq!(ErrorKind::MapRes.description(), "Map on Result");
        assert_eq!(ErrorKind::MapOpt.description(), "Map on Option");
        assert_eq!(ErrorKind::Alt.description(), "Alternative");
        assert_eq!(ErrorKind::IsNot.description(), "IsNot");
        assert_eq!(ErrorKind::IsA.description(), "IsA");
        assert_eq!(ErrorKind::SeparatedList.description(), "Separated list");
        assert_eq!(ErrorKind::SeparatedNonEmptyList.description(), "Separated non empty list");
        assert_eq!(ErrorKind::Many0.description(), "Many0");
        assert_eq!(ErrorKind::Many1.description(), "Many1");
        assert_eq!(ErrorKind::Count.description(), "Count");
        assert_eq!(ErrorKind::TakeUntil.description(), "Take until");
        assert_eq!(ErrorKind::LengthValue.description(), "Length followed by value");
        assert_eq!(ErrorKind::TagClosure.description(), "Tag closure");
        assert_eq!(ErrorKind::Alpha.description(), "Alphabetic");
        assert_eq!(ErrorKind::Digit.description(), "Digit");
        assert_eq!(ErrorKind::AlphaNumeric.description(), "AlphaNumeric");
        assert_eq!(ErrorKind::Space.description(), "Space");
        assert_eq!(ErrorKind::MultiSpace.description(), "Multiple spaces");
        assert_eq!(ErrorKind::LengthValueFn.description(), "LengthValueFn");
        assert_eq!(ErrorKind::Eof.description(), "End of file");
        assert_eq!(ErrorKind::Switch.description(), "Switch");
        assert_eq!(ErrorKind::TagBits.description(), "Tag on bitstream");
        assert_eq!(ErrorKind::OneOf.description(), "OneOf");
        assert_eq!(ErrorKind::NoneOf.description(), "NoneOf");
        assert_eq!(ErrorKind::Char.description(), "Char");
        assert_eq!(ErrorKind::CrLf.description(), "CrLf");
        assert_eq!(ErrorKind::RegexpMatch.description(), "RegexpMatch");
        assert_eq!(ErrorKind::RegexpMatches.description(), "RegexpMatches");
        assert_eq!(ErrorKind::RegexpFind.description(), "RegexpFind");
        assert_eq!(ErrorKind::RegexpCapture.description(), "RegexpCapture");
        assert_eq!(ErrorKind::RegexpCaptures.description(), "RegexpCaptures");
        assert_eq!(ErrorKind::TakeWhile1.description(), "TakeWhile1");
        assert_eq!(ErrorKind::Complete.description(), "Complete");
        assert_eq!(ErrorKind::Fix.description(), "Fix");
        assert_eq!(ErrorKind::Escaped.description(), "Escaped");
        assert_eq!(ErrorKind::EscapedTransform.description(), "EscapedTransform");
        assert_eq!(ErrorKind::NonEmpty.description(), "NonEmpty");
        assert_eq!(ErrorKind::ManyMN.description(), "Many(m, n)");
        assert_eq!(ErrorKind::HexDigit.description(), "Hexadecimal Digit");
        assert_eq!(ErrorKind::OctDigit.description(), "Octal digit");
        assert_eq!(ErrorKind::Not.description(), "Negation");
        assert_eq!(ErrorKind::Permutation.description(), "Permutation");
        assert_eq!(ErrorKind::ManyTill.description(), "ManyTill");
        assert_eq!(ErrorKind::Verify.description(), "predicate verification");
        assert_eq!(ErrorKind::TakeTill1.description(), "TakeTill1");
        assert_eq!(ErrorKind::TakeWhileMN.description(), "TakeWhileMN");
        assert_eq!(ErrorKind::TooLarge.description(), "Needed data size is too large");
        assert_eq!(ErrorKind::Many0Count.description(), "Count occurrence of >=0 patterns");
        assert_eq!(ErrorKind::Many1Count.description(), "Count occurrence of >=1 patterns");
        assert_eq!(ErrorKind::Float.description(), "Float");
        assert_eq!(ErrorKind::Satisfy.description(), "Satisfy");
        assert_eq!(ErrorKind::Fail.description(), "Fail");
        assert_eq!(ErrorKind::Many.description(), "Many");
        assert_eq!(ErrorKind::Fold.description(), "Fold");
    }
}#[cfg(test)]
mod tests_llm_16_414_llm_16_414 {
    use super::*;

use crate::*;
    use crate::error::{ErrorKind, ParseError};

    #[derive(Debug, PartialEq)]
    struct TestError<'a> {
        input: &'a str,
        code: ErrorKind,
    }

    impl<'a> ParseError<&'a str> for TestError<'a> {
        fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
            TestError { input, code: kind }
        }

        fn append(_: &'a str, _: ErrorKind, other: Self) -> Self {
            other
        }
    }

    #[test]
    fn test_from_char() {
        let input = "test input";
        let expected_char = 'a';
        let error: TestError = ParseError::from_char(input, expected_char);
        assert_eq!(error, TestError {
            input,
            code: ErrorKind::Char,
        });
    }
}#[cfg(test)]
mod tests_llm_16_415 {
    use super::*;

use crate::*;

    #[derive(Debug, PartialEq)]
    struct TestError<I> {
        input: I,
        error_code: u32,
    }

    impl<I> ParseError<I> for TestError<I> {
        fn from_error_kind(input: I, kind: ErrorKind) -> Self {
            TestError {
                input,
                error_code: kind as u32,
            }
        }

        fn append(_: I, _: ErrorKind, other: Self) -> Self {
            other
        }

        fn from_char(input: I, _: char) -> Self {
            TestError {
                input,
                error_code: 0,
            }
        }

        fn or(self, other: Self) -> Self {
            other
        }
    }

    #[test]
    fn test_or() {
        let error1 = TestError {
            input: "input1",
            error_code: 1,
        };
        let error2 = TestError {
            input: "input2",
            error_code: 2,
        };

        let combined_error = error1.or(error2);

        assert_eq!(
            combined_error,
            TestError {
                input: "input2",
                error_code: 2,
            }
        );
    }
}#[cfg(test)]
mod tests_llm_16_416 {
    use super::*;

use crate::*;

    #[test]
    fn append_error_should_preserve_existing_error() {
        let initial_input = &b"The quick brown fox"[..];
        let initial_error = Error::new(initial_input, ErrorKind::Digit);
        let new_input = &b" jumps over the lazy dog"[..];
        let new_error = append_error(new_input, ErrorKind::Alpha, initial_error.clone());

        // The new error should be the initial one
        assert_eq!(new_error.input, initial_input);
        assert_eq!(new_error.code, ErrorKind::Digit);
    }

    #[test]
    fn append_error_should_ignore_new_error() {
        let initial_input = &b"The quick brown fox"[..];
        let initial_error = Error::new(initial_input, ErrorKind::Digit);
        let new_input = &b" jumps over the lazy dog"[..];
        let new_error = append_error(new_input, ErrorKind::Alpha, initial_error.clone());

        // The new error kind should be ignored
        assert_eq!(new_error.code, initial_error.code);
    }

    #[test]
    fn append_error_should_not_change_input() {
        let initial_input = &b"The quick brown fox"[..];
        let initial_error = Error::new(initial_input, ErrorKind::Digit);
        let new_input = &b" jumps over the lazy dog"[..];
        let new_error = append_error(new_input, ErrorKind::Alpha, initial_error.clone());

        // The input part of the error should be intact, ignoring the new input
        assert_eq!(new_error.input, initial_input);
    }
}#[cfg(test)]
mod tests_llm_16_417 {
    use crate::{
        error::{context, ContextError, Error, ErrorKind, ParseError},
        Err, IResult, Parser,
    };

    #[derive(Debug, PartialEq)]
    struct DummyError<I> {
        input: I,
        code: ErrorKind,
        context: &'static str,
    }

    impl<I> ContextError<I> for DummyError<I> {
        fn add_context(input: I, ctx: &'static str, other: Self) -> Self {
            DummyError {
                input,
                code: other.code,
                context: ctx,
            }
        }
    }

    impl<I> ParseError<I> for DummyError<I> {
        fn from_error_kind(input: I, kind: ErrorKind) -> Self {
            DummyError {
                input,
                code: kind,
                context: "",
            }
        }

        fn append(input: I, kind: ErrorKind, other: Self) -> Self {
            DummyError {
                input,
                code: kind,
                context: other.context,
            }
        }
    }

    fn dummy_parser<'a>(input: &'a str) -> IResult<&'a str, &'a str, DummyError<&'a str>> {
        if input.starts_with("nom") {
            Ok((&input[3..], &input[..3]))
        } else {
            Err(Err::Error(DummyError {
                input,
                code: ErrorKind::Tag,
                context: "",
            }))
        }
    }

    fn dummy_parser_with_context<'a>(input: &'a str) -> IResult<&'a str, &'a str, DummyError<&'a str>> {
        context("dummy_context", dummy_parser)(input)
    }

    #[test]
    fn test_dummy_parser_with_context_success() {
        assert_eq!(
            dummy_parser_with_context("nomnom"),
            Ok(("nom", "nom"))
        );
    }

    #[test]
    fn test_dummy_parser_with_context_failure() {
        assert_eq!(
            dummy_parser_with_context("error"),
            Err(Err::Error(DummyError {
                input: "error",
                code: ErrorKind::Tag,
                context: "dummy_context",
            }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_418 {
    use super::*;

use crate::*;
    use crate::error::{convert_error, VerboseError, VerboseErrorKind};
    use crate::traits::Offset;

    #[test]
    fn test_convert_error_empty_input() {
        let input = "";
        let errors = vec![(input, VerboseErrorKind::Char('a'))];
        let verbose_error = VerboseError { errors };
        let result = convert_error(input, verbose_error);
        assert_eq!(result, "0: expected 'a', got empty input\n\n");
    }

    #[test]
    fn test_convert_error_with_context() {
        let input = "abc";
        let errors = vec![(input, VerboseErrorKind::Context("test"))];
        let verbose_error = VerboseError { errors };
        let result = convert_error(input, verbose_error);
        assert_eq!(
            result,
            "0: at line 1, in test:\nabc\n^\n\n"
        );
    }

    #[test]
    fn test_convert_error_with_nom_error() {
        let input = "abc";
        let errors = vec![(input, VerboseErrorKind::Nom(crate::error::ErrorKind::Tag))];
        let verbose_error = VerboseError { errors };
        let result = convert_error(input, verbose_error);
        assert_eq!(
            result,
            "0: at line 1, in ErrorKind::Tag:\nabc\n^\n\n"
        );
    }

    #[test]
    fn test_convert_error_with_unexpected_char() {
        let input = "abc";
        let errors = vec![(input, VerboseErrorKind::Char('d'))];
        let verbose_error = VerboseError { errors };
        let result = convert_error(input, verbose_error);
        assert_eq!(
            result,
            "0: at line 1:\nabc\n^\nexpected 'd', found 'a'\n\n"
        );
    }
}#[cfg(test)]
mod tests_llm_16_419 {
    use super::*;

use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        Err,
    };

    #[test]
    fn test_dbg_dmp_success() {
        fn parser(i: &[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
            crate::bytes::complete::tag("abcd")(i)
        }
        let input = b"abcdef";
        let wrapped_parser = dbg_dmp(parser, "test_dbg_dmp_success");
        match wrapped_parser(input) {
            Ok((remaining, output)) => {
                assert_eq!(output, b"abcd");
                assert_eq!(remaining, b"ef");
            }
            Err(_) => assert!(false, "Parser should succeed"),
        }
    }

    #[test]
    fn test_dbg_dmp_failure() {
        fn parser(i: &[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
            crate::bytes::complete::tag("abcd")(i)
        }
        let input = b"xyz";
        let wrapped_parser = dbg_dmp(parser, "test_dbg_dmp_failure");
        match wrapped_parser(input) {
            Ok(_) => assert!(false, "Parser should fail"),
            Err(Err::Error(e)) => {
                assert_eq!(e.code, ErrorKind::Tag);
                assert_eq!(e.input, b"xyz");
            }
            Err(_) => assert!(false, "Error should be crate::Err::Error"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_420_llm_16_420 {
    use crate::error::{ErrorKind, error_to_u32};

    #[test]
    fn test_error_to_u32() {
        assert_eq!(error_to_u32(&ErrorKind::Tag), 1);
        assert_eq!(error_to_u32(&ErrorKind::MapRes), 2);
        assert_eq!(error_to_u32(&ErrorKind::MapOpt), 3);
        assert_eq!(error_to_u32(&ErrorKind::Alt), 4);
        assert_eq!(error_to_u32(&ErrorKind::IsNot), 5);
        assert_eq!(error_to_u32(&ErrorKind::IsA), 6);
        assert_eq!(error_to_u32(&ErrorKind::SeparatedList), 7);
        assert_eq!(error_to_u32(&ErrorKind::SeparatedNonEmptyList), 8);
        assert_eq!(error_to_u32(&ErrorKind::Many1), 9);
        assert_eq!(error_to_u32(&ErrorKind::Count), 10);
        assert_eq!(error_to_u32(&ErrorKind::TakeUntil), 12);
        assert_eq!(error_to_u32(&ErrorKind::LengthValue), 15);
        assert_eq!(error_to_u32(&ErrorKind::TagClosure), 16);
        assert_eq!(error_to_u32(&ErrorKind::Alpha), 17);
        assert_eq!(error_to_u32(&ErrorKind::Digit), 18);
        assert_eq!(error_to_u32(&ErrorKind::AlphaNumeric), 19);
        assert_eq!(error_to_u32(&ErrorKind::Space), 20);
        assert_eq!(error_to_u32(&ErrorKind::MultiSpace), 21);
        assert_eq!(error_to_u32(&ErrorKind::LengthValueFn), 22);
        assert_eq!(error_to_u32(&ErrorKind::Eof), 23);
        assert_eq!(error_to_u32(&ErrorKind::Switch), 27);
        assert_eq!(error_to_u32(&ErrorKind::TagBits), 28);
        assert_eq!(error_to_u32(&ErrorKind::OneOf), 29);
        assert_eq!(error_to_u32(&ErrorKind::NoneOf), 30);
        assert_eq!(error_to_u32(&ErrorKind::Char), 40);
        assert_eq!(error_to_u32(&ErrorKind::CrLf), 41);
        assert_eq!(error_to_u32(&ErrorKind::RegexpMatch), 42);
        assert_eq!(error_to_u32(&ErrorKind::RegexpMatches), 43);
        assert_eq!(error_to_u32(&ErrorKind::RegexpFind), 44);
        assert_eq!(error_to_u32(&ErrorKind::RegexpCapture), 45);
        assert_eq!(error_to_u32(&ErrorKind::RegexpCaptures), 46);
        assert_eq!(error_to_u32(&ErrorKind::TakeWhile1), 47);
        assert_eq!(error_to_u32(&ErrorKind::Complete), 48);
        assert_eq!(error_to_u32(&ErrorKind::Fix), 49);
        assert_eq!(error_to_u32(&ErrorKind::Escaped), 50);
        assert_eq!(error_to_u32(&ErrorKind::EscapedTransform), 51);
        assert_eq!(error_to_u32(&ErrorKind::NonEmpty), 56);
        assert_eq!(error_to_u32(&ErrorKind::ManyMN), 57);
        assert_eq!(error_to_u32(&ErrorKind::HexDigit), 59);
        assert_eq!(error_to_u32(&ErrorKind::OctDigit), 61);
        assert_eq!(error_to_u32(&ErrorKind::Many0), 62);
        assert_eq!(error_to_u32(&ErrorKind::Not), 63);
        assert_eq!(error_to_u32(&ErrorKind::Permutation), 64);
        assert_eq!(error_to_u32(&ErrorKind::ManyTill), 65);
        assert_eq!(error_to_u32(&ErrorKind::Verify), 66);
        assert_eq!(error_to_u32(&ErrorKind::TakeTill1), 67);
        assert_eq!(error_to_u32(&ErrorKind::TakeWhileMN), 69);
        assert_eq!(error_to_u32(&ErrorKind::TooLarge), 70);
        assert_eq!(error_to_u32(&ErrorKind::Many0Count), 71);
        assert_eq!(error_to_u32(&ErrorKind::Many1Count), 72);
        assert_eq!(error_to_u32(&ErrorKind::Float), 73);
        assert_eq!(error_to_u32(&ErrorKind::Satisfy), 74);
        assert_eq!(error_to_u32(&ErrorKind::Fail), 75);
        assert_eq!(error_to_u32(&ErrorKind::Many), 76);
        assert_eq!(error_to_u32(&ErrorKind::Fold), 77);
    }
}