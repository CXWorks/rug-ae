//! Traits input types have to implement to work with nom combinators
use core::iter::Enumerate;
use core::str::CharIndices;
use crate::error::{ErrorKind, ParseError};
use crate::internal::{Err, IResult, Needed};
use crate::lib::std::iter::Copied;
use crate::lib::std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};
use crate::lib::std::slice::Iter;
use crate::lib::std::str::from_utf8;
use crate::lib::std::str::Chars;
use crate::lib::std::str::FromStr;
#[cfg(feature = "alloc")]
use crate::lib::std::string::String;
#[cfg(feature = "alloc")]
use crate::lib::std::vec::Vec;
/// Parser input types must implement this trait
pub trait Input: Clone + Sized {
    /// The current input type is a sequence of that `Item` type.
    ///
    /// Example: `u8` for `&[u8]` or `char` for `&str`
    type Item;
    /// An iterator over the input type, producing the item
    type Iter: Iterator<Item = Self::Item>;
    /// An iterator over the input type, producing the item and its byte position
    /// If we're iterating over `&str`, the position
    /// corresponds to the byte index of the character
    type IterIndices: Iterator<Item = (usize, Self::Item)>;
    /// Calculates the input length, as indicated by its name,
    /// and the name of the trait itself
    fn input_len(&self) -> usize;
    /// Returns a slice of `index` bytes. panics if index > length
    fn take(&self, index: usize) -> Self;
    /// Returns a slice starting at `index` bytes. panics if index > length
    fn take_from(&self, index: usize) -> Self;
    /// Split the stream at the `index` byte offset. panics if index > length
    fn take_split(&self, index: usize) -> (Self, Self);
    /// Returns the byte position of the first element satisfying the predicate
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool;
    /// Returns an iterator over the elements
    fn iter_elements(&self) -> Self::Iter;
    /// Returns an iterator over the elements and their byte offsets
    fn iter_indices(&self) -> Self::IterIndices;
    /// Get the byte offset from the element's position in the stream
    fn slice_index(&self, count: usize) -> Result<usize, Needed>;
    /// Looks for the first element of the input type for which the condition returns true,
    /// and returns the input up to this position.
    ///
    /// *streaming version*: If no element is found matching the condition, this will return `Incomplete`
    fn split_at_position<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }
    /// Looks for the first element of the input type for which the condition returns true
    /// and returns the input up to this position.
    ///
    /// Fails if the produced slice is empty.
    ///
    /// *streaming version*: If no element is found matching the condition, this will return `Incomplete`
    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }
    /// Looks for the first element of the input type for which the condition returns true,
    /// and returns the input up to this position.
    ///
    /// *complete version*: If no element is found matching the condition, this will return the whole input
    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }
    /// Looks for the first element of the input type for which the condition returns true
    /// and returns the input up to this position.
    ///
    /// Fails if the produced slice is empty.
    ///
    /// *complete version*: If no element is found matching the condition, this will return the whole input
    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position1(predicate, e) {
            Err(Err::Incomplete(_)) => {
                if self.input_len() == 0 {
                    Err(Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
            res => res,
        }
    }
}
impl<'a> Input for &'a [u8] {
    type Item = u8;
    type Iter = Copied<Iter<'a, u8>>;
    type IterIndices = Enumerate<Self::Iter>;
    fn input_len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn take(&self, index: usize) -> Self {
        &self[0..index]
    }
    fn take_from(&self, index: usize) -> Self {
        &self[index..]
    }
    #[inline]
    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.split_at(index);
        (suffix, prefix)
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.iter().position(|b| predicate(*b))
    }
    #[inline]
    fn iter_elements(&self) -> Self::Iter {
        self.iter().copied()
    }
    #[inline]
    fn iter_indices(&self) -> Self::IterIndices {
        self.iter_elements().enumerate()
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.len()))
        }
    }
    fn split_at_position<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.iter().position(|c| predicate(*c)) {
            Some(i) => Ok(self.take_split(i)),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }
    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.iter().position(|c| predicate(*c)) {
            Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
            Some(i) => Ok(self.take_split(i)),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }
    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.iter().position(|c| predicate(*c)) {
            Some(i) => Ok(self.take_split(i)),
            None => Ok(self.take_split(self.len())),
        }
    }
    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.iter().position(|c| predicate(*c)) {
            Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
            Some(i) => Ok(self.take_split(i)),
            None => {
                if self.is_empty() {
                    Err(Err::Error(E::from_error_kind(self, e)))
                } else {
                    Ok(self.take_split(self.len()))
                }
            }
        }
    }
}
impl<'a> Input for &'a str {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;
    fn input_len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn take(&self, index: usize) -> Self {
        &self[..index]
    }
    #[inline]
    fn take_from(&self, index: usize) -> Self {
        &self[index..]
    }
    #[inline]
    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.split_at(index);
        (suffix, prefix)
    }
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.find(predicate)
    }
    #[inline]
    fn iter_elements(&self) -> Self::Iter {
        self.chars()
    }
    #[inline]
    fn iter_indices(&self) -> Self::IterIndices {
        self.char_indices()
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        let mut cnt = 0;
        for (index, _) in self.char_indices() {
            if cnt == count {
                return Ok(index);
            }
            cnt += 1;
        }
        if cnt == count {
            return Ok(self.len());
        }
        Err(Needed::Unknown)
    }
    fn split_at_position<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.find(predicate) {
            Some(i) => unsafe { Ok((self.get_unchecked(i..), self.get_unchecked(..i))) }
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }
    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.find(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
            Some(i) => unsafe { Ok((self.get_unchecked(i..), self.get_unchecked(..i))) }
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }
    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.find(predicate) {
            Some(i) => unsafe { Ok((self.get_unchecked(i..), self.get_unchecked(..i))) }
            None => {
                unsafe {
                    Ok((
                        self.get_unchecked(self.len()..),
                        self.get_unchecked(..self.len()),
                    ))
                }
            }
        }
    }
    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.find(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
            Some(i) => unsafe { Ok((self.get_unchecked(i..), self.get_unchecked(..i))) }
            None => {
                if self.is_empty() {
                    Err(Err::Error(E::from_error_kind(self, e)))
                } else {
                    unsafe {
                        Ok((
                            self.get_unchecked(self.len()..),
                            self.get_unchecked(..self.len()),
                        ))
                    }
                }
            }
        }
    }
}
/// Abstract method to calculate the input length
pub trait InputLength {
    /// Calculates the input length, as indicated by its name,
    /// and the name of the trait itself
    fn input_len(&self) -> usize;
}
impl<'a, T> InputLength for &'a [T] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}
impl<'a> InputLength for &'a str {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}
impl<'a> InputLength for (&'a [u8], usize) {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len() * 8 - self.1
    }
}
/// Useful functions to calculate the offset between slices and show a hexdump of a slice
pub trait Offset {
    /// Offset between the first byte of self and the first byte of the argument
    fn offset(&self, second: &Self) -> usize;
}
impl Offset for [u8] {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();
        snd as usize - fst as usize
    }
}
impl<'a> Offset for &'a [u8] {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();
        snd as usize - fst as usize
    }
}
impl Offset for str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();
        snd as usize - fst as usize
    }
}
impl<'a> Offset for &'a str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();
        snd as usize - fst as usize
    }
}
/// Helper trait for types that can be viewed as a byte slice
pub trait AsBytes {
    /// Casts the input type to a byte slice
    fn as_bytes(&self) -> &[u8];
}
impl<'a> AsBytes for &'a str {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        (*self).as_bytes()
    }
}
impl AsBytes for str {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}
impl<'a> AsBytes for &'a [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}
impl AsBytes for [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}
impl<'a, const N: usize> AsBytes for &'a [u8; N] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        *self
    }
}
impl<const N: usize> AsBytes for [u8; N] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}
/// Transforms common types to a char for basic token parsing
#[allow(clippy::len_without_is_empty)]
pub trait AsChar: Copy {
    /// makes a char from self
    fn as_char(self) -> char;
    /// Tests that self is an alphabetic character
    ///
    /// Warning: for `&str` it recognizes alphabetic
    /// characters outside of the 52 ASCII letters
    fn is_alpha(self) -> bool;
    /// Tests that self is an alphabetic character
    /// or a decimal digit
    fn is_alphanum(self) -> bool;
    /// Tests that self is a decimal digit
    fn is_dec_digit(self) -> bool;
    /// Tests that self is an hex digit
    fn is_hex_digit(self) -> bool;
    /// Tests that self is an octal digit
    fn is_oct_digit(self) -> bool;
    /// Gets the len in bytes for self
    fn len(self) -> usize;
}
impl AsChar for u8 {
    #[inline]
    fn as_char(self) -> char {
        self as char
    }
    #[inline]
    fn is_alpha(self) -> bool {
        matches!(self, 0x41..= 0x5A | 0x61..= 0x7A)
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_dec_digit()
    }
    #[inline]
    fn is_dec_digit(self) -> bool {
        matches!(self, 0x30..= 0x39)
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        matches!(self, 0x30..= 0x39 | 0x41..= 0x46 | 0x61..= 0x66)
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        matches!(self, 0x30..= 0x37)
    }
    #[inline]
    fn len(self) -> usize {
        1
    }
}
impl<'a> AsChar for &'a u8 {
    #[inline]
    fn as_char(self) -> char {
        *self as char
    }
    #[inline]
    fn is_alpha(self) -> bool {
        matches!(* self, 0x41..= 0x5A | 0x61..= 0x7A)
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_dec_digit()
    }
    #[inline]
    fn is_dec_digit(self) -> bool {
        matches!(* self, 0x30..= 0x39)
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        matches!(* self, 0x30..= 0x39 | 0x41..= 0x46 | 0x61..= 0x66)
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        matches!(* self, 0x30..= 0x37)
    }
    #[inline]
    fn len(self) -> usize {
        1
    }
}
impl AsChar for char {
    #[inline]
    fn as_char(self) -> char {
        self
    }
    #[inline]
    fn is_alpha(self) -> bool {
        self.is_ascii_alphabetic()
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_dec_digit()
    }
    #[inline]
    fn is_dec_digit(self) -> bool {
        self.is_ascii_digit()
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        self.is_ascii_hexdigit()
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        self.is_digit(8)
    }
    #[inline]
    fn len(self) -> usize {
        self.len_utf8()
    }
}
impl<'a> AsChar for &'a char {
    #[inline]
    fn as_char(self) -> char {
        *self
    }
    #[inline]
    fn is_alpha(self) -> bool {
        self.is_ascii_alphabetic()
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_dec_digit()
    }
    #[inline]
    fn is_dec_digit(self) -> bool {
        self.is_ascii_digit()
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        self.is_ascii_hexdigit()
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        self.is_digit(8)
    }
    #[inline]
    fn len(self) -> usize {
        self.len_utf8()
    }
}
/// Indicates whether a comparison was successful, an error, or
/// if more data was needed
#[derive(Debug, Eq, PartialEq)]
pub enum CompareResult {
    /// Comparison was successful
    Ok,
    /// We need more data to be sure
    Incomplete,
    /// Comparison failed
    Error,
}
/// Abstracts comparison operations
pub trait Compare<T> {
    /// Compares self to another value for equality
    fn compare(&self, t: T) -> CompareResult;
    /// Compares self to another value for equality
    /// independently of the case.
    ///
    /// Warning: for `&str`, the comparison is done
    /// by lowercasing both strings and comparing
    /// the result. This is a temporary solution until
    /// a better one appears
    fn compare_no_case(&self, t: T) -> CompareResult;
}
fn lowercase_byte(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
}
impl<'a, 'b> Compare<&'b [u8]> for &'a [u8] {
    #[inline(always)]
    fn compare(&self, t: &'b [u8]) -> CompareResult {
        let pos = self.iter().zip(t.iter()).position(|(a, b)| a != b);
        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.len() >= t.len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }
    #[inline(always)]
    fn compare_no_case(&self, t: &'b [u8]) -> CompareResult {
        if self.iter().zip(t).any(|(a, b)| lowercase_byte(*a) != lowercase_byte(*b)) {
            CompareResult::Error
        } else if self.len() < t.len() {
            CompareResult::Incomplete
        } else {
            CompareResult::Ok
        }
    }
}
impl<'a, 'b> Compare<&'b str> for &'a [u8] {
    #[inline(always)]
    fn compare(&self, t: &'b str) -> CompareResult {
        self.compare(AsBytes::as_bytes(t))
    }
    #[inline(always)]
    fn compare_no_case(&self, t: &'b str) -> CompareResult {
        self.compare_no_case(AsBytes::as_bytes(t))
    }
}
impl<'a, 'b> Compare<&'b str> for &'a str {
    #[inline(always)]
    fn compare(&self, t: &'b str) -> CompareResult {
        self.as_bytes().compare(t.as_bytes())
    }
    #[inline(always)]
    fn compare_no_case(&self, t: &'b str) -> CompareResult {
        let pos = self
            .chars()
            .zip(t.chars())
            .position(|(a, b)| a.to_lowercase().ne(b.to_lowercase()));
        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.len() >= t.len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }
}
impl<'a, 'b> Compare<&'b [u8]> for &'a str {
    #[inline(always)]
    fn compare(&self, t: &'b [u8]) -> CompareResult {
        AsBytes::as_bytes(self).compare(t)
    }
    #[inline(always)]
    fn compare_no_case(&self, t: &'b [u8]) -> CompareResult {
        AsBytes::as_bytes(self).compare_no_case(t)
    }
}
/// Look for a token in self
pub trait FindToken<T> {
    /// Returns true if self contains the token
    fn find_token(&self, token: T) -> bool;
}
impl<'a> FindToken<u8> for &'a [u8] {
    fn find_token(&self, token: u8) -> bool {
        memchr::memchr(token, self).is_some()
    }
}
impl<'a> FindToken<u8> for &'a str {
    fn find_token(&self, token: u8) -> bool {
        self.as_bytes().find_token(token)
    }
}
impl<'a, 'b> FindToken<&'a u8> for &'b [u8] {
    fn find_token(&self, token: &u8) -> bool {
        self.find_token(*token)
    }
}
impl<'a, 'b> FindToken<&'a u8> for &'b str {
    fn find_token(&self, token: &u8) -> bool {
        self.as_bytes().find_token(token)
    }
}
impl<'a> FindToken<char> for &'a [u8] {
    fn find_token(&self, token: char) -> bool {
        self.iter().any(|i| *i == token as u8)
    }
}
impl<'a> FindToken<char> for &'a str {
    fn find_token(&self, token: char) -> bool {
        self.chars().any(|i| i == token)
    }
}
impl<'a> FindToken<char> for &'a [char] {
    fn find_token(&self, token: char) -> bool {
        self.iter().any(|i| *i == token)
    }
}
impl<'a, 'b> FindToken<&'a char> for &'b [char] {
    fn find_token(&self, token: &char) -> bool {
        self.find_token(*token)
    }
}
/// Look for a substring in self
pub trait FindSubstring<T> {
    /// Returns the byte position of the substring if it is found
    fn find_substring(&self, substr: T) -> Option<usize>;
}
impl<'a, 'b> FindSubstring<&'b [u8]> for &'a [u8] {
    fn find_substring(&self, substr: &'b [u8]) -> Option<usize> {
        if substr.len() > self.len() {
            return None;
        }
        let (&substr_first, substr_rest) = match substr.split_first() {
            Some(split) => split,
            None => return Some(0),
        };
        if substr_rest.is_empty() {
            return memchr::memchr(substr_first, self);
        }
        let mut offset = 0;
        let haystack = &self[..self.len() - substr_rest.len()];
        while let Some(position) = memchr::memchr(substr_first, &haystack[offset..]) {
            offset += position;
            let next_offset = offset + 1;
            if &self[next_offset..][..substr_rest.len()] == substr_rest {
                return Some(offset);
            }
            offset = next_offset;
        }
        None
    }
}
impl<'a, 'b> FindSubstring<&'b str> for &'a [u8] {
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.find_substring(AsBytes::as_bytes(substr))
    }
}
impl<'a, 'b> FindSubstring<&'b str> for &'a str {
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.find(substr)
    }
}
/// Used to integrate `str`'s `parse()` method
pub trait ParseTo<R> {
    /// Succeeds if `parse()` succeeded. The byte slice implementation
    /// will first convert it to a `&str`, then apply the `parse()` function
    fn parse_to(&self) -> Option<R>;
}
impl<'a, R: FromStr> ParseTo<R> for &'a [u8] {
    fn parse_to(&self) -> Option<R> {
        from_utf8(self).ok().and_then(|s| s.parse().ok())
    }
}
impl<'a, R: FromStr> ParseTo<R> for &'a str {
    fn parse_to(&self) -> Option<R> {
        self.parse().ok()
    }
}
impl<const N: usize> InputLength for [u8; N] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}
impl<'a, const N: usize> InputLength for &'a [u8; N] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}
impl<'a, const N: usize> Compare<[u8; N]> for &'a [u8] {
    #[inline(always)]
    fn compare(&self, t: [u8; N]) -> CompareResult {
        self.compare(&t[..])
    }
    #[inline(always)]
    fn compare_no_case(&self, t: [u8; N]) -> CompareResult {
        self.compare_no_case(&t[..])
    }
}
impl<'a, 'b, const N: usize> Compare<&'b [u8; N]> for &'a [u8] {
    #[inline(always)]
    fn compare(&self, t: &'b [u8; N]) -> CompareResult {
        self.compare(&t[..])
    }
    #[inline(always)]
    fn compare_no_case(&self, t: &'b [u8; N]) -> CompareResult {
        self.compare_no_case(&t[..])
    }
}
impl<const N: usize> FindToken<u8> for [u8; N] {
    fn find_token(&self, token: u8) -> bool {
        memchr::memchr(token, &self[..]).is_some()
    }
}
impl<'a, const N: usize> FindToken<&'a u8> for [u8; N] {
    fn find_token(&self, token: &u8) -> bool {
        self.find_token(*token)
    }
}
/// Abstracts something which can extend an `Extend`.
/// Used to build modified input slices in `escaped_transform`
pub trait ExtendInto {
    /// The current input type is a sequence of that `Item` type.
    ///
    /// Example: `u8` for `&[u8]` or `char` for `&str`
    type Item;
    /// The type that will be produced
    type Extender;
    /// Create a new `Extend` of the correct type
    fn new_builder(&self) -> Self::Extender;
    /// Accumulate the input into an accumulator
    fn extend_into(&self, acc: &mut Self::Extender);
}
#[cfg(feature = "alloc")]
impl ExtendInto for [u8] {
    type Item = u8;
    type Extender = Vec<u8>;
    #[inline]
    fn new_builder(&self) -> Vec<u8> {
        Vec::new()
    }
    #[inline]
    fn extend_into(&self, acc: &mut Vec<u8>) {
        acc.extend(self.iter().cloned());
    }
}
#[cfg(feature = "alloc")]
impl ExtendInto for &[u8] {
    type Item = u8;
    type Extender = Vec<u8>;
    #[inline]
    fn new_builder(&self) -> Vec<u8> {
        Vec::new()
    }
    #[inline]
    fn extend_into(&self, acc: &mut Vec<u8>) {
        acc.extend_from_slice(self);
    }
}
#[cfg(feature = "alloc")]
impl ExtendInto for str {
    type Item = char;
    type Extender = String;
    #[inline]
    fn new_builder(&self) -> String {
        String::new()
    }
    #[inline]
    fn extend_into(&self, acc: &mut String) {
        acc.push_str(self);
    }
}
#[cfg(feature = "alloc")]
impl ExtendInto for &str {
    type Item = char;
    type Extender = String;
    #[inline]
    fn new_builder(&self) -> String {
        String::new()
    }
    #[inline]
    fn extend_into(&self, acc: &mut String) {
        acc.push_str(self);
    }
}
#[cfg(feature = "alloc")]
impl ExtendInto for char {
    type Item = char;
    type Extender = String;
    #[inline]
    fn new_builder(&self) -> String {
        String::new()
    }
    #[inline]
    fn extend_into(&self, acc: &mut String) {
        acc.push(*self);
    }
}
/// Helper trait to convert numbers to usize.
///
/// By default, usize implements `From<u8>` and `From<u16>` but not
/// `From<u32>` and `From<u64>` because that would be invalid on some
/// platforms. This trait implements the conversion for platforms
/// with 32 and 64 bits pointer platforms
pub trait ToUsize {
    /// converts self to usize
    fn to_usize(&self) -> usize;
}
impl ToUsize for u8 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for u16 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for usize {
    #[inline]
    fn to_usize(&self) -> usize {
        *self
    }
}
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl ToUsize for u32 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
#[cfg(target_pointer_width = "64")]
impl ToUsize for u64 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
/// Equivalent From implementation to avoid orphan rules in bits parsers
pub trait ErrorConvert<E> {
    /// Transform to another error type
    fn convert(self) -> E;
}
impl<I> ErrorConvert<(I, ErrorKind)> for ((I, usize), ErrorKind) {
    fn convert(self) -> (I, ErrorKind) {
        ((self.0).0, self.1)
    }
}
impl<I> ErrorConvert<((I, usize), ErrorKind)> for (I, ErrorKind) {
    fn convert(self) -> ((I, usize), ErrorKind) {
        ((self.0, 0), self.1)
    }
}
use crate::error;
impl<I> ErrorConvert<error::Error<I>> for error::Error<(I, usize)> {
    fn convert(self) -> error::Error<I> {
        error::Error {
            input: self.input.0,
            code: self.code,
        }
    }
}
impl<I> ErrorConvert<error::Error<(I, usize)>> for error::Error<I> {
    fn convert(self) -> error::Error<(I, usize)> {
        error::Error {
            input: (self.input, 0),
            code: self.code,
        }
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl<I> ErrorConvert<error::VerboseError<I>> for error::VerboseError<(I, usize)> {
    fn convert(self) -> error::VerboseError<I> {
        error::VerboseError {
            errors: self.errors.into_iter().map(|(i, e)| (i.0, e)).collect(),
        }
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl<I> ErrorConvert<error::VerboseError<(I, usize)>> for error::VerboseError<I> {
    fn convert(self) -> error::VerboseError<(I, usize)> {
        error::VerboseError {
            errors: self.errors.into_iter().map(|(i, e)| ((i, 0), e)).collect(),
        }
    }
}
impl ErrorConvert<()> for () {
    fn convert(self) {}
}
#[cfg(feature = "std")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "std")))]
/// Helper trait to show a byte slice as a hex dump
pub trait HexDisplay {
    /// Converts the value of `self` to a hex dump, returning the owned
    /// `String`.
    fn to_hex(&self, chunk_size: usize) -> String;
    /// Converts the value of `self` to a hex dump beginning at `from` address, returning the owned
    /// `String`.
    fn to_hex_from(&self, chunk_size: usize, from: usize) -> String;
}
#[cfg(feature = "std")]
static CHARS: &[u8] = b"0123456789abcdef";
#[cfg(feature = "std")]
impl HexDisplay for [u8] {
    #[allow(unused_variables)]
    fn to_hex(&self, chunk_size: usize) -> String {
        self.to_hex_from(chunk_size, 0)
    }
    #[allow(unused_variables)]
    fn to_hex_from(&self, chunk_size: usize, from: usize) -> String {
        let mut v = Vec::with_capacity(self.len() * 3);
        let mut i = from;
        for chunk in self.chunks(chunk_size) {
            let s = format!("{:08x}", i);
            for &ch in s.as_bytes().iter() {
                v.push(ch);
            }
            v.push(b'\t');
            i += chunk_size;
            for &byte in chunk {
                v.push(CHARS[(byte >> 4) as usize]);
                v.push(CHARS[(byte & 0xf) as usize]);
                v.push(b' ');
            }
            if chunk_size > chunk.len() {
                for j in 0..(chunk_size - chunk.len()) {
                    v.push(b' ');
                    v.push(b' ');
                    v.push(b' ');
                }
            }
            v.push(b'\t');
            for &byte in chunk {
                if matches!(byte, 32..= 126 | 128..= 255) {
                    v.push(byte);
                } else {
                    v.push(b'.');
                }
            }
            v.push(b'\n');
        }
        String::from_utf8_lossy(&v[..]).into_owned()
    }
}
#[cfg(feature = "std")]
impl HexDisplay for str {
    #[allow(unused_variables)]
    fn to_hex(&self, chunk_size: usize) -> String {
        self.to_hex_from(chunk_size, 0)
    }
    #[allow(unused_variables)]
    fn to_hex_from(&self, chunk_size: usize, from: usize) -> String {
        self.as_bytes().to_hex_from(chunk_size, from)
    }
}
/// A saturating iterator for usize.
pub struct SaturatingIterator {
    count: usize,
}
impl Iterator for SaturatingIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let old_count = self.count;
        self.count = self.count.saturating_add(1);
        Some(old_count)
    }
}
/// Abstractions for range-like types.
pub trait NomRange<Idx> {
    /// The saturating iterator type.
    type Saturating: Iterator<Item = Idx>;
    /// The bounded iterator type.
    type Bounded: Iterator<Item = Idx>;
    /// `true` if `item` is contained in the range.
    fn contains(&self, item: &Idx) -> bool;
    /// Returns the bounds of this range.
    fn bounds(&self) -> (Bound<Idx>, Bound<Idx>);
    /// `true` if the range is inverted.
    fn is_inverted(&self) -> bool;
    /// Creates a saturating iterator.
    /// A saturating iterator counts the number of iterations starting from 0 up to the upper bound of this range.
    /// If the upper bound is infinite the iterator saturates at the largest representable value of its type and
    /// returns it for all further elements.
    fn saturating_iter(&self) -> Self::Saturating;
    /// Creates a bounded iterator.
    /// A bounded iterator counts the number of iterations starting from 0 up to the upper bound of this range.
    /// If the upper bounds is infinite the iterator counts up until the amount of iterations has reached the
    /// largest representable value of its type and then returns `None` for all further elements.
    fn bounded_iter(&self) -> Self::Bounded;
}
impl NomRange<usize> for Range<usize> {
    type Saturating = Range<usize>;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Included(self.start), Bound::Excluded(self.end))
    }
    fn contains(&self, item: &usize) -> bool {
        RangeBounds::contains(self, item)
    }
    fn is_inverted(&self) -> bool {
        !(self.start < self.end)
    }
    fn saturating_iter(&self) -> Self::Saturating {
        if self.end == 0 { 1..0 } else { 0..self.end - 1 }
    }
    fn bounded_iter(&self) -> Self::Bounded {
        if self.end == 0 { 1..0 } else { 0..self.end - 1 }
    }
}
impl NomRange<usize> for RangeInclusive<usize> {
    type Saturating = Range<usize>;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Included(*self.start()), Bound::Included(*self.end()))
    }
    fn contains(&self, item: &usize) -> bool {
        RangeBounds::contains(self, item)
    }
    fn is_inverted(&self) -> bool {
        !RangeInclusive::contains(self, self.start())
    }
    fn saturating_iter(&self) -> Self::Saturating {
        0..*self.end()
    }
    fn bounded_iter(&self) -> Self::Bounded {
        0..*self.end()
    }
}
impl NomRange<usize> for RangeFrom<usize> {
    type Saturating = SaturatingIterator;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Included(self.start), Bound::Unbounded)
    }
    fn contains(&self, item: &usize) -> bool {
        RangeBounds::contains(self, item)
    }
    fn is_inverted(&self) -> bool {
        false
    }
    fn saturating_iter(&self) -> Self::Saturating {
        SaturatingIterator { count: 0 }
    }
    fn bounded_iter(&self) -> Self::Bounded {
        0..core::usize::MAX
    }
}
impl NomRange<usize> for RangeTo<usize> {
    type Saturating = Range<usize>;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Unbounded, Bound::Excluded(self.end))
    }
    fn contains(&self, item: &usize) -> bool {
        RangeBounds::contains(self, item)
    }
    fn is_inverted(&self) -> bool {
        false
    }
    fn saturating_iter(&self) -> Self::Saturating {
        if self.end == 0 { 1..0 } else { 0..self.end - 1 }
    }
    fn bounded_iter(&self) -> Self::Bounded {
        if self.end == 0 { 1..0 } else { 0..self.end - 1 }
    }
}
impl NomRange<usize> for RangeToInclusive<usize> {
    type Saturating = Range<usize>;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Unbounded, Bound::Included(self.end))
    }
    fn contains(&self, item: &usize) -> bool {
        RangeBounds::contains(self, item)
    }
    fn is_inverted(&self) -> bool {
        false
    }
    fn saturating_iter(&self) -> Self::Saturating {
        0..self.end
    }
    fn bounded_iter(&self) -> Self::Bounded {
        0..self.end
    }
}
impl NomRange<usize> for RangeFull {
    type Saturating = SaturatingIterator;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Unbounded, Bound::Unbounded)
    }
    fn contains(&self, item: &usize) -> bool {
        RangeBounds::contains(self, item)
    }
    fn is_inverted(&self) -> bool {
        false
    }
    fn saturating_iter(&self) -> Self::Saturating {
        SaturatingIterator { count: 0 }
    }
    fn bounded_iter(&self) -> Self::Bounded {
        0..core::usize::MAX
    }
}
impl NomRange<usize> for usize {
    type Saturating = Range<usize>;
    type Bounded = Range<usize>;
    fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Included(*self), Bound::Included(*self))
    }
    fn contains(&self, item: &usize) -> bool {
        self == item
    }
    fn is_inverted(&self) -> bool {
        false
    }
    fn saturating_iter(&self) -> Self::Saturating {
        0..*self
    }
    fn bounded_iter(&self) -> Self::Bounded {
        0..*self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_offset_u8() {
        let s = b"abcd123";
        let a = &s[..];
        let b = &a[2..];
        let c = &a[..4];
        let d = &a[3..5];
        assert_eq!(a.offset(b), 2);
        assert_eq!(a.offset(c), 0);
        assert_eq!(a.offset(d), 3);
    }
    #[test]
    fn test_offset_str() {
        let a = "abcřèÂßÇd123";
        let b = &a[7..];
        let c = &a[..5];
        let d = &a[5..9];
        assert_eq!(a.offset(b), 7);
        assert_eq!(a.offset(c), 0);
        assert_eq!(a.offset(d), 5);
    }
    #[test]
    fn test_slice_index() {
        let a = "abcřèÂßÇd123";
        assert_eq!(a.slice_index(0), Ok(0));
        assert_eq!(a.slice_index(2), Ok(2));
    }
    #[test]
    fn test_slice_index_utf8() {
        let a = "a¡€💢€¡a";
        for (c, len) in a.chars().zip([1, 2, 3, 4, 3, 2, 1]) {
            assert_eq!(c.len(), len);
        }
        assert_eq!(a.slice_index(0), Ok(0));
        assert_eq!(a.slice_index(1), Ok(1));
        assert_eq!(a.slice_index(2), Ok(3));
        assert_eq!(a.slice_index(3), Ok(6));
        assert_eq!(a.slice_index(4), Ok(10));
        assert_eq!(a.slice_index(5), Ok(13));
        assert_eq!(a.slice_index(6), Ok(15));
        assert_eq!(a.slice_index(7), Ok(16));
        assert!(a.slice_index(8).is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::traits::InputLength;
    #[test]
    fn test_input_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_empty: &[u8] = &[];
        let input_one: &[u8] = &[rug_fuzz_0];
        let input_many: &[u8] = &[
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
        ];
        debug_assert_eq!(input_empty.input_len(), 0);
        debug_assert_eq!(input_one.input_len(), 1);
        debug_assert_eq!(input_many.input_len(), 5);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use crate::traits::FindToken;
    #[test]
    fn find_token_in_char_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(char, char, char, char, char, char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[char] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        debug_assert!(input.find_token(rug_fuzz_4));
        debug_assert!(input.find_token(rug_fuzz_5));
        debug_assert!(input.find_token(rug_fuzz_6));
        debug_assert!(input.find_token(rug_fuzz_7));
        debug_assert!(! input.find_token(rug_fuzz_8));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use crate::traits::AsBytes;
    #[test]
    fn test_as_bytes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array: &[u8; 5] = &[
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let bytes: &[u8] = array.as_bytes();
        debug_assert_eq!(bytes, & [0, 1, 2, 3, 4]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use crate::traits::InputLength;
    #[test]
    fn input_len_for_array_ref() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array_ref: &[u8; 4] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        debug_assert_eq!(array_ref.input_len(), 4);
             }
}
}
}    }
    #[test]
    fn input_len_for_empty_array_ref() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_input_len_for_empty_array_ref = 0;
        let empty_array_ref: &[u8; 0] = &[];
        debug_assert_eq!(empty_array_ref.input_len(), 0);
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_input_len_for_empty_array_ref = 0;
    }
    #[test]
    fn input_len_for_large_array_ref() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let large_array_ref: &[u8; 1024] = &[rug_fuzz_0; 1024];
        debug_assert_eq!(large_array_ref.input_len(), 1024);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    #[test]
    fn as_bytes_identity() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let output: &[u8] = input.as_bytes();
        debug_assert_eq!(input, output);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::traits::{Compare, CompareResult};
    #[test]
    fn compare_equal_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let b: &[u8; 5] = &[rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9];
        debug_assert_eq!(a.compare(b), CompareResult::Ok);
             }
}
}
}    }
    #[test]
    fn compare_incomplete_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let b: &[u8; 5] = &[rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        debug_assert_eq!(a.compare(b), CompareResult::Incomplete);
             }
}
}
}    }
    #[test]
    fn compare_error_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let b: &[u8; 5] = &[rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9];
        debug_assert_eq!(a.compare(b), CompareResult::Error);
             }
}
}
}    }
    #[test]
    fn compare_empty_slice_with_empty_array() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_compare_empty_slice_with_empty_array = 0;
        let a: &[u8] = &[];
        let b: &[u8; 0] = &[];
        debug_assert_eq!(a.compare(b), CompareResult::Ok);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_compare_empty_slice_with_empty_array = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    #[test]
    fn compare_equal() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_compare_equal = 0;
        let rug_fuzz_0 = b"hello";
        let rug_fuzz_1 = b"hello";
        let a: &[u8] = rug_fuzz_0;
        let b: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as traits::Compare < & [u8] > > ::compare(& a, b), CompareResult::Ok
        );
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_compare_equal = 0;
    }
    #[test]
    fn compare_incomplete() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_compare_incomplete = 0;
        let rug_fuzz_0 = b"hello";
        let rug_fuzz_1 = b"hello world";
        let a: &[u8] = rug_fuzz_0;
        let b: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as traits::Compare < & [u8] > > ::compare(& a, b),
            CompareResult::Incomplete
        );
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_compare_incomplete = 0;
    }
    #[test]
    fn compare_error() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_compare_error = 0;
        let rug_fuzz_0 = b"hello";
        let rug_fuzz_1 = b"world";
        let a: &[u8] = rug_fuzz_0;
        let b: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as traits::Compare < & [u8] > > ::compare(& a, b),
            CompareResult::Error
        );
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_compare_error = 0;
    }
    #[test]
    fn compare_prefix() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_compare_prefix = 0;
        let rug_fuzz_0 = b"hello world";
        let rug_fuzz_1 = b"hello";
        let a: &[u8] = rug_fuzz_0;
        let b: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as traits::Compare < & [u8] > > ::compare(& a, b), CompareResult::Ok
        );
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_compare_prefix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    use crate::traits::Compare;
    fn lowercase_byte(input: u8) -> u8 {
        match input {
            b'A'..=b'Z' => input + 32,
            _ => input,
        }
    }
    #[test]
    fn test_compare_no_case_equal() {
        let a: &[u8] = b"abc";
        let b: &[u8] = b"abc";
        assert_eq!(
            <& [u8] as Compare <& [u8] >>::compare_no_case(& a, & b), CompareResult::Ok
        );
    }
    #[test]
    fn test_compare_no_case_equal_ignore_case() {
        let a: &[u8] = b"abc";
        let b: &[u8] = b"ABC";
        assert_eq!(
            <& [u8] as Compare <& [u8] >>::compare_no_case(& a, & b), CompareResult::Ok
        );
    }
    #[test]
    fn test_compare_no_case_incomplete() {
        let a: &[u8] = b"abcd";
        let b: &[u8] = b"abc";
        assert_eq!(
            <& [u8] as Compare <& [u8] >>::compare_no_case(& a, & b),
            CompareResult::Incomplete
        );
    }
    #[test]
    fn test_compare_no_case_error() {
        let a: &[u8] = b"abc";
        let b: &[u8] = b"xyz";
        assert_eq!(
            <& [u8] as Compare <& [u8] >>::compare_no_case(& a, & b),
            CompareResult::Error
        );
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use crate::traits::{Compare, CompareResult};
    #[test]
    fn test_compare_no_case_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 13], &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let other = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as Compare < & str > > ::compare_no_case(& input, other),
            CompareResult::Ok
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_incomplete() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 10], &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let other = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as Compare < & str > > ::compare_no_case(& input, other),
            CompareResult::Incomplete
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 13], &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let other = rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as Compare < & str > > ::compare_no_case(& input, other),
            CompareResult::Error
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    #[test]
    fn test_compare_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let pattern: [u8; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        debug_assert_eq!(
            < & [u8] as traits::Compare < [u8; 4] > > ::compare(& data, pattern),
            traits::CompareResult::Ok
        );
             }
}
}
}    }
    #[test]
    fn test_compare_incomplete() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let pattern: [u8; 4] = [rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6];
        debug_assert_eq!(
            < & [u8] as traits::Compare < [u8; 4] > > ::compare(& data, pattern),
            traits::CompareResult::Incomplete
        );
             }
}
}
}    }
    #[test]
    fn test_compare_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let pattern: [u8; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        debug_assert_eq!(
            < & [u8] as traits::Compare < [u8; 4] > > ::compare(& data, pattern),
            traits::CompareResult::Error
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;
    use crate::*;
    use crate::traits::{Compare, CompareResult};
    #[test]
    fn compare_no_case_test() {
        let _rug_st_tests_llm_16_13_rrrruuuugggg_compare_no_case_test = 0;
        let rug_fuzz_0 = b"Hello World";
        let rug_fuzz_1 = b"hello world";
        let rug_fuzz_2 = b"Hello Worl";
        let rug_fuzz_3 = b"Goodbye";
        let rug_fuzz_4 = b"hello world";
        let rug_fuzz_5 = b"hello";
        let input: &[u8] = rug_fuzz_0;
        let comparison: [u8; 11] = *rug_fuzz_1;
        debug_assert_eq!(
            < & [u8] as Compare < [u8; 11] > > ::compare_no_case(& input, comparison),
            CompareResult::Ok
        );
        let incomplete_input: &[u8] = rug_fuzz_2;
        debug_assert_eq!(
            < & [u8] as Compare < [u8; 11] > > ::compare_no_case(& incomplete_input,
            comparison), CompareResult::Incomplete
        );
        let error_input: &[u8] = rug_fuzz_3;
        let error_comparison: [u8; 11] = *rug_fuzz_4;
        debug_assert_eq!(
            < & [u8] as Compare < [u8; 11] > > ::compare_no_case(& error_input,
            error_comparison), CompareResult::Error
        );
        let different_len_comparison: [u8; 5] = *rug_fuzz_5;
        debug_assert_eq!(
            < & [u8] as Compare < [u8; 5] > > ::compare_no_case(& input,
            different_len_comparison), CompareResult::Incomplete
        );
        let _rug_ed_tests_llm_16_13_rrrruuuugggg_compare_no_case_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;
    use crate::*;
    use crate::traits::FindSubstring;
    use crate::AsBytes;
    #[test]
    fn find_substring_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <([u8; 43], &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        debug_assert_eq!(input.find_substring(rug_fuzz_1), Some(4));
        debug_assert_eq!(input.find_substring(rug_fuzz_2), Some(16));
        debug_assert_eq!(input.find_substring(rug_fuzz_3), None);
        debug_assert_eq!(input.find_substring(rug_fuzz_4), Some(0));
        debug_assert_eq!(input.find_substring(rug_fuzz_5), Some(40));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_16 {
    use crate::FindToken;
    #[test]
    fn find_token_char_in_u8_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <([u8; 11], char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input_slice: &[u8] = rug_fuzz_0;
        debug_assert!(
            < & [u8] as FindToken < char > > ::find_token(& input_slice, rug_fuzz_1)
        );
        debug_assert!(
            < & [u8] as FindToken < char > > ::find_token(& input_slice, rug_fuzz_2)
        );
        debug_assert!(
            ! < & [u8] as FindToken < char > > ::find_token(& input_slice, rug_fuzz_3)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_17_llm_16_17 {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_token_exists() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        debug_assert!(< & [u8] as FindToken < u8 > > ::find_token(& data, rug_fuzz_5));
             }
}
}
}    }
    #[test]
    fn test_find_token_not_exists() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        debug_assert!(! < & [u8] as FindToken < u8 > > ::find_token(& data, rug_fuzz_5));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_18 {
    use crate::traits::InputLength;
    #[test]
    fn input_len_empty_slice() {
        let _rug_st_tests_llm_16_18_rrrruuuugggg_input_len_empty_slice = 0;
        let input: &[u8] = &[];
        debug_assert_eq!(input.input_len(), 0);
        let _rug_ed_tests_llm_16_18_rrrruuuugggg_input_len_empty_slice = 0;
    }
    #[test]
    fn input_len_non_empty_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        debug_assert_eq!(input.input_len(), 5);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_19 {
    use super::*;
    use crate::*;
    #[test]
    fn iter_elements_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut iter = input.iter_elements();
        debug_assert_eq!(iter.next(), Some(1));
        debug_assert_eq!(iter.next(), Some(2));
        debug_assert_eq!(iter.next(), Some(3));
        debug_assert_eq!(iter.next(), Some(4));
        debug_assert_eq!(iter.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use crate::traits::Input;
    #[test]
    fn take_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let expected: &[u8] = &[rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        debug_assert_eq!(< & [u8] as Input > ::take(& input, rug_fuzz_8), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_28 {
    use super::*;
    use crate::*;
    #[test]
    fn take_from_at_start() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 13], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let result = Input::take_from(&input, rug_fuzz_1);
        debug_assert_eq!(result, b"Hello, World!");
             }
}
}
}    }
    #[test]
    fn take_from_in_the_middle() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 13], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let result = Input::take_from(&input, rug_fuzz_1);
        debug_assert_eq!(result, b"World!");
             }
}
}
}    }
    #[test]
    fn take_from_at_end() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 13], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let result = Input::take_from(&input, rug_fuzz_1);
        debug_assert_eq!(result, b"");
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn take_from_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 13], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let input: &[u8] = rug_fuzz_0;
        let _ = Input::take_from(&input, rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_30 {
    use super::*;
    use crate::*;
    use crate::traits::Offset;
    #[test]
    fn offset_non_empty_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub = &data[rug_fuzz_5..];
        let offset_value = data.offset(sub);
        debug_assert_eq!(offset_value, 1);
             }
}
}
}    }
    #[test]
    fn offset_empty_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub = &data[rug_fuzz_5..];
        let offset_value = data.offset(sub);
        debug_assert_eq!(offset_value, 5);
             }
}
}
}    }
    #[test]
    fn offset_same_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let offset_value = data.offset(data);
        debug_assert_eq!(offset_value, 0);
             }
}
}
}    }
    #[test]
    fn offset_with_offset_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub1 = &data[rug_fuzz_5..];
        let sub2 = &data[rug_fuzz_6..];
        let offset_value = sub1.offset(sub2);
        debug_assert_eq!(offset_value, 2);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn offset_incorrect_order_slices() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub1 = &data[rug_fuzz_5..];
        let sub2 = &data[rug_fuzz_6..];
        let _ = sub1.offset(sub2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    use super::*;
    use crate::*;
    use crate::traits::ParseTo;
    #[test]
    fn test_parse_to_success() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_parse_to_success = 0;
        let rug_fuzz_0 = b"123";
        let input: &[u8] = rug_fuzz_0;
        let result: Option<i32> = ParseTo::parse_to(&input);
        debug_assert_eq!(result, Some(123));
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_parse_to_success = 0;
    }
    #[test]
    fn test_parse_to_invalid_utf8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let result: Option<i32> = ParseTo::parse_to(&input);
        debug_assert_eq!(result, None);
             }
}
}
}    }
    #[test]
    fn test_parse_to_invalid_parse() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_parse_to_invalid_parse = 0;
        let rug_fuzz_0 = b"abc";
        let input: &[u8] = rug_fuzz_0;
        let result: Option<i32> = ParseTo::parse_to(&input);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_parse_to_invalid_parse = 0;
    }
    #[test]
    fn test_parse_to_valid_utf8_invalid_parse() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_parse_to_valid_utf8_invalid_parse = 0;
        let rug_fuzz_0 = b"123abc";
        let input: &[u8] = rug_fuzz_0;
        let result: Option<i32> = ParseTo::parse_to(&input);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_parse_to_valid_utf8_invalid_parse = 0;
    }
    #[test]
    fn test_parse_to_empty() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_parse_to_empty = 0;
        let rug_fuzz_0 = b"";
        let input: &[u8] = rug_fuzz_0;
        let result: Option<i32> = ParseTo::parse_to(&input);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_parse_to_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;
    use crate::*;
    #[test]
    fn as_char_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = &rug_fuzz_0;
        debug_assert_eq!(traits::AsChar::as_char(input), 'a');
        let input = &rug_fuzz_1;
        debug_assert_eq!(traits::AsChar::as_char(input), 'b');
        let input = &rug_fuzz_2;
        debug_assert_eq!(traits::AsChar::as_char(input), '1');
        let input = &rug_fuzz_3;
        debug_assert_eq!(traits::AsChar::as_char(input), '%');
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_alpha_with_ascii_alpha() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_alpha(), true);
        debug_assert_eq!(rug_fuzz_1.is_alpha(), true);
        debug_assert_eq!(rug_fuzz_2.is_alpha(), true);
             }
}
}
}    }
    #[test]
    fn test_is_alpha_with_ascii_non_alpha() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_alpha(), false);
        debug_assert_eq!(rug_fuzz_1.is_alpha(), false);
        debug_assert_eq!(rug_fuzz_2.is_alpha(), false);
             }
}
}
}    }
    #[test]
    fn test_is_alpha_with_non_ascii() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_alpha(), false);
        debug_assert_eq!(rug_fuzz_1.is_alpha(), false);
        debug_assert_eq!(rug_fuzz_2.is_alpha(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_alphanum_alpha() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!((& rug_fuzz_0).is_alphanum());
        debug_assert!((& rug_fuzz_1).is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!((& rug_fuzz_0).is_alphanum());
        debug_assert!((& rug_fuzz_1).is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_non_alphanum() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! (& rug_fuzz_0).is_alphanum());
        debug_assert!(! (& rug_fuzz_1).is_alphanum());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_dec_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(char, char, char, char, char, char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((& rug_fuzz_0).is_dec_digit(), true);
        debug_assert_eq!((& rug_fuzz_1).is_dec_digit(), true);
        debug_assert_eq!((& rug_fuzz_2).is_dec_digit(), true);
        debug_assert_eq!((& rug_fuzz_3).is_dec_digit(), true);
        debug_assert_eq!((& rug_fuzz_4).is_dec_digit(), false);
        debug_assert_eq!((& rug_fuzz_5).is_dec_digit(), false);
        debug_assert_eq!((& rug_fuzz_6).is_dec_digit(), false);
        debug_assert_eq!((& rug_fuzz_7).is_dec_digit(), false);
        debug_assert_eq!((& rug_fuzz_8).is_dec_digit(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_36_llm_16_36 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_hex_digit() {
        let _rug_st_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_is_hex_digit = 0;
        let rug_fuzz_0 = '0';
        let rug_fuzz_1 = '1';
        let rug_fuzz_2 = '2';
        let rug_fuzz_3 = '3';
        let rug_fuzz_4 = '4';
        let rug_fuzz_5 = '5';
        let rug_fuzz_6 = '6';
        let rug_fuzz_7 = '7';
        let rug_fuzz_8 = '8';
        let rug_fuzz_9 = '9';
        let rug_fuzz_10 = 'a';
        let rug_fuzz_11 = 'b';
        let rug_fuzz_12 = 'c';
        let rug_fuzz_13 = 'd';
        let rug_fuzz_14 = 'e';
        let rug_fuzz_15 = 'f';
        let rug_fuzz_16 = 'A';
        let rug_fuzz_17 = 'B';
        let rug_fuzz_18 = 'C';
        let rug_fuzz_19 = 'D';
        let rug_fuzz_20 = 'E';
        let rug_fuzz_21 = 'F';
        let rug_fuzz_22 = 'g';
        let rug_fuzz_23 = 'h';
        let rug_fuzz_24 = 'z';
        let rug_fuzz_25 = 'G';
        let rug_fuzz_26 = 'X';
        let rug_fuzz_27 = 'Z';
        let rug_fuzz_28 = '/';
        let rug_fuzz_29 = '@';
        let rug_fuzz_30 = '[';
        let rug_fuzz_31 = '`';
        let rug_fuzz_32 = '{';
        debug_assert!((& rug_fuzz_0).is_hex_digit());
        debug_assert!((& rug_fuzz_1).is_hex_digit());
        debug_assert!((& rug_fuzz_2).is_hex_digit());
        debug_assert!((& rug_fuzz_3).is_hex_digit());
        debug_assert!((& rug_fuzz_4).is_hex_digit());
        debug_assert!((& rug_fuzz_5).is_hex_digit());
        debug_assert!((& rug_fuzz_6).is_hex_digit());
        debug_assert!((& rug_fuzz_7).is_hex_digit());
        debug_assert!((& rug_fuzz_8).is_hex_digit());
        debug_assert!((& rug_fuzz_9).is_hex_digit());
        debug_assert!((& rug_fuzz_10).is_hex_digit());
        debug_assert!((& rug_fuzz_11).is_hex_digit());
        debug_assert!((& rug_fuzz_12).is_hex_digit());
        debug_assert!((& rug_fuzz_13).is_hex_digit());
        debug_assert!((& rug_fuzz_14).is_hex_digit());
        debug_assert!((& rug_fuzz_15).is_hex_digit());
        debug_assert!((& rug_fuzz_16).is_hex_digit());
        debug_assert!((& rug_fuzz_17).is_hex_digit());
        debug_assert!((& rug_fuzz_18).is_hex_digit());
        debug_assert!((& rug_fuzz_19).is_hex_digit());
        debug_assert!((& rug_fuzz_20).is_hex_digit());
        debug_assert!((& rug_fuzz_21).is_hex_digit());
        debug_assert!(! (& rug_fuzz_22).is_hex_digit());
        debug_assert!(! (& rug_fuzz_23).is_hex_digit());
        debug_assert!(! (& rug_fuzz_24).is_hex_digit());
        debug_assert!(! (& rug_fuzz_25).is_hex_digit());
        debug_assert!(! (& rug_fuzz_26).is_hex_digit());
        debug_assert!(! (& rug_fuzz_27).is_hex_digit());
        debug_assert!(! (& rug_fuzz_28).is_hex_digit());
        debug_assert!(! (& rug_fuzz_29).is_hex_digit());
        debug_assert!(! (& rug_fuzz_30).is_hex_digit());
        debug_assert!(! (& rug_fuzz_31).is_hex_digit());
        debug_assert!(! (& rug_fuzz_32).is_hex_digit());
        let _rug_ed_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_is_hex_digit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_oct_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(char, char, char, char, char, char, char, char, char, char, char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_oct_digit());
        debug_assert!(rug_fuzz_1.is_oct_digit());
        debug_assert!(rug_fuzz_2.is_oct_digit());
        debug_assert!(rug_fuzz_3.is_oct_digit());
        debug_assert!(rug_fuzz_4.is_oct_digit());
        debug_assert!(rug_fuzz_5.is_oct_digit());
        debug_assert!(rug_fuzz_6.is_oct_digit());
        debug_assert!(rug_fuzz_7.is_oct_digit());
        debug_assert!(! rug_fuzz_8.is_oct_digit());
        debug_assert!(! rug_fuzz_9.is_oct_digit());
        debug_assert!(! rug_fuzz_10.is_oct_digit());
        debug_assert!(! rug_fuzz_11.is_oct_digit());
        debug_assert!(! rug_fuzz_12.is_oct_digit());
        debug_assert!(! rug_fuzz_13.is_oct_digit());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_38_llm_16_38 {
    use crate::traits::AsChar;
    #[test]
    fn test_char_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = &rug_fuzz_0;
        let result = AsChar::len(*c);
        debug_assert_eq!(result, 'a'.len_utf8());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use crate::traits::AsBytes;
    #[test]
    fn as_bytes_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let bytes = input.as_bytes();
        debug_assert_eq!(bytes, input.as_bytes());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;
    use crate::*;
    use crate::traits::{Compare, CompareResult, AsBytes};
    #[test]
    fn compare_with_equal_bytes() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_compare_with_equal_bytes = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = b"hello";
        let input_str = rug_fuzz_0;
        let input_bytes = rug_fuzz_1;
        let result = <&str as Compare<&[u8]>>::compare(&input_str, &input_bytes[..]);
        debug_assert_eq!(result, CompareResult::Ok);
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_compare_with_equal_bytes = 0;
    }
    #[test]
    fn compare_with_non_equal_bytes() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_compare_with_non_equal_bytes = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = b"world";
        let input_str = rug_fuzz_0;
        let input_bytes = rug_fuzz_1;
        let result = <&str as Compare<&[u8]>>::compare(&input_str, &input_bytes[..]);
        debug_assert_ne!(result, CompareResult::Ok);
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_compare_with_non_equal_bytes = 0;
    }
    #[test]
    fn compare_with_partial_bytes() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_compare_with_partial_bytes = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = b"hell";
        let input_str = rug_fuzz_0;
        let input_bytes = rug_fuzz_1;
        let result = <&str as Compare<&[u8]>>::compare(&input_str, &input_bytes[..]);
        debug_assert_eq!(result, CompareResult::Incomplete);
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_compare_with_partial_bytes = 0;
    }
    #[test]
    fn compare_with_extra_bytes() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_compare_with_extra_bytes = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = b"hello world";
        let input_str = rug_fuzz_0;
        let input_bytes = rug_fuzz_1;
        let result = <&str as Compare<&[u8]>>::compare(&input_str, &input_bytes[..]);
        debug_assert_ne!(result, CompareResult::Ok);
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_compare_with_extra_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_42 {
    use super::*;
    use crate::*;
    #[test]
    fn test_compare_no_case_equal() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_compare_no_case_equal = 0;
        let rug_fuzz_0 = "abc";
        let rug_fuzz_1 = b"ABC";
        let input_str: &str = rug_fuzz_0;
        let compare_bytes: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & str as traits::Compare < & [u8] > > ::compare_no_case(& input_str,
            compare_bytes), traits::CompareResult::Ok
        );
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_compare_no_case_equal = 0;
    }
    #[test]
    fn test_compare_no_case_incomplete() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_compare_no_case_incomplete = 0;
        let rug_fuzz_0 = "ab";
        let rug_fuzz_1 = b"ABC";
        let input_str: &str = rug_fuzz_0;
        let compare_bytes: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & str as traits::Compare < & [u8] > > ::compare_no_case(& input_str,
            compare_bytes), traits::CompareResult::Incomplete
        );
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_compare_no_case_incomplete = 0;
    }
    #[test]
    fn test_compare_no_case_error() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_compare_no_case_error = 0;
        let rug_fuzz_0 = "abc";
        let rug_fuzz_1 = b"XYZ";
        let input_str: &str = rug_fuzz_0;
        let compare_bytes: &[u8] = rug_fuzz_1;
        debug_assert_eq!(
            < & str as traits::Compare < & [u8] > > ::compare_no_case(& input_str,
            compare_bytes), traits::CompareResult::Error
        );
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_compare_no_case_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_43 {
    use crate::traits::Compare;
    use crate::traits::CompareResult::*;
    #[test]
    fn test_compare_equal() {
        let _rug_st_tests_llm_16_43_rrrruuuugggg_test_compare_equal = 0;
        let rug_fuzz_0 = "Hello";
        let rug_fuzz_1 = "Hello";
        let s1: &str = rug_fuzz_0;
        let s2: &str = rug_fuzz_1;
        debug_assert_eq!(< & str as Compare < & str > > ::compare(& s1, s2), Ok);
        let _rug_ed_tests_llm_16_43_rrrruuuugggg_test_compare_equal = 0;
    }
    #[test]
    fn test_compare_incomplete() {
        let _rug_st_tests_llm_16_43_rrrruuuugggg_test_compare_incomplete = 0;
        let rug_fuzz_0 = "Hello";
        let rug_fuzz_1 = "Hello, World!";
        let s1: &str = rug_fuzz_0;
        let s2: &str = rug_fuzz_1;
        debug_assert_eq!(< & str as Compare < & str > > ::compare(& s1, s2), Incomplete);
        let _rug_ed_tests_llm_16_43_rrrruuuugggg_test_compare_incomplete = 0;
    }
    #[test]
    fn test_compare_error() {
        let _rug_st_tests_llm_16_43_rrrruuuugggg_test_compare_error = 0;
        let rug_fuzz_0 = "Hello";
        let rug_fuzz_1 = "world";
        let s1: &str = rug_fuzz_0;
        let s2: &str = rug_fuzz_1;
        debug_assert_eq!(< & str as Compare < & str > > ::compare(& s1, s2), Error);
        let _rug_ed_tests_llm_16_43_rrrruuuugggg_test_compare_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use crate::traits::{Compare, CompareResult};
    #[test]
    fn test_compare_no_case_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Ok
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_incomplete() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Incomplete
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Error
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_error_at_start() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Error
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_empty_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Incomplete
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_empty_compare_to() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Ok
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_empty_both() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Ok
        );
             }
}
}
}    }
    #[test]
    fn test_compare_no_case_special_chars() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let compare_to = rug_fuzz_1;
        debug_assert_eq!(
            < & str as Compare < & str > > ::compare_no_case(& input, compare_to),
            CompareResult::Error
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_substring() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.find_substring(rug_fuzz_1), Some(6));
        debug_assert_eq!(rug_fuzz_2.find_substring(rug_fuzz_3), Some(0));
        debug_assert_eq!(rug_fuzz_4.find_substring(rug_fuzz_5), None);
        debug_assert_eq!(rug_fuzz_6.find_substring(rug_fuzz_7), Some(0));
        debug_assert_eq!(rug_fuzz_8.find_substring(rug_fuzz_9), None);
        debug_assert_eq!(rug_fuzz_10.find_substring(rug_fuzz_11), Some(0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use crate::traits::FindToken;
    #[test]
    fn test_find_token_char_in_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert!(< & str as FindToken < char > > ::find_token(& input, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_find_token_char_not_in_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert!(
            ! < & str as FindToken < char > > ::find_token(& input, rug_fuzz_1)
        );
             }
}
}
}    }
    #[test]
    fn test_find_token_char_empty_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert!(
            ! < & str as FindToken < char > > ::find_token(& input, rug_fuzz_1)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_token() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s: &str = rug_fuzz_0;
        debug_assert_eq!(
            < & str as traits::FindToken < u8 > > ::find_token(& s, rug_fuzz_1), true
        );
        debug_assert_eq!(
            < & str as traits::FindToken < u8 > > ::find_token(& s, rug_fuzz_2), true
        );
        debug_assert_eq!(
            < & str as traits::FindToken < u8 > > ::find_token(& s, rug_fuzz_3), true
        );
        debug_assert_eq!(
            < & str as traits::FindToken < u8 > > ::find_token(& s, rug_fuzz_4), true
        );
        debug_assert_eq!(
            < & str as traits::FindToken < u8 > > ::find_token(& s, rug_fuzz_5), false
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_48 {
    use crate::traits::Input;
    #[test]
    fn test_input_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert_eq!(< & str as Input > ::input_len(& input), 5);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use super::*;
    use crate::*;
    #[test]
    fn test_iter_elements() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut iterator = <&str as traits::Input>::iter_elements(&input);
        debug_assert_eq!(iterator.next(), Some('h'));
        debug_assert_eq!(iterator.next(), Some('e'));
        debug_assert_eq!(iterator.next(), Some('l'));
        debug_assert_eq!(iterator.next(), Some('l'));
        debug_assert_eq!(iterator.next(), Some('o'));
        debug_assert_eq!(iterator.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_51 {
    use super::*;
    use crate::*;
    use crate::traits::Input;
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }
    #[test]
    fn position_finds_first_vowel() {
        let input = "bcdfghjklmnpqrstvwxyz";
        let position = input.position(is_vowel);
        assert_eq!(position, None);
    }
    #[test]
    fn position_finds_no_vowel() {
        let input = "hello";
        let position = input.position(is_vowel);
        assert_eq!(position, Some(1));
    }
    #[test]
    fn position_empty_input() {
        let input = "";
        let position = input.position(is_vowel);
        assert_eq!(position, None);
    }
    #[test]
    fn position_predicate_always_false() {
        let input = "hello";
        let position = input.position(|_| false);
        assert_eq!(position, None);
    }
    #[test]
    fn position_predicate_always_true() {
        let input = "hello";
        let position = input.position(|_| true);
        assert_eq!(position, Some(0));
    }
}
#[cfg(test)]
mod tests_llm_16_52 {
    use crate::Needed;
    use crate::traits::Input;
    #[test]
    fn test_slice_index_on_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_1), Ok(0));
        debug_assert_eq!(
            < & str as Input > ::slice_index(& input, rug_fuzz_2), Err(Needed::Unknown)
        );
             }
}
}
}    }
    #[test]
    fn test_slice_index() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_1), Ok(0));
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_2), Ok(1));
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_3), Ok(5));
        debug_assert_eq!(
            < & str as Input > ::slice_index(& input, rug_fuzz_4), Err(Needed::Unknown)
        );
             }
}
}
}    }
    #[test]
    fn test_slice_index_with_multibyte_chars() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_1), Ok(0));
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_2), Ok(3));
        debug_assert_eq!(< & str as Input > ::slice_index(& input, rug_fuzz_3), Ok(15));
        debug_assert_eq!(
            < & str as Input > ::slice_index(& input, rug_fuzz_4), Err(Needed::Unknown)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_56 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        Err, IResult,
    };
    #[test]
    fn test_split_at_position_complete() {
        fn predicate(c: char) -> bool {
            c == '|'
        }
        fn split_at_position_complete<P, E: ParseError<&'static str>>(
            input: &'static str,
            predicate: P,
        ) -> IResult<&'static str, &'static str, E>
        where
            P: Fn(char) -> bool,
        {
            match input.find(predicate) {
                Some(i) => {
                    unsafe { Ok((input.get_unchecked(i..), input.get_unchecked(..i))) }
                }
                None => {
                    unsafe {
                        Ok((
                            input.get_unchecked(input.len()..),
                            input.get_unchecked(..input.len()),
                        ))
                    }
                }
            }
        }
        let pos0: IResult<&'static str, &'static str, Error<&'static str>> = split_at_position_complete(
            "before|after",
            predicate,
        );
        assert_eq!(pos0, Ok(("|after", "before")));
        let pos1: IResult<&'static str, &'static str, Error<&'static str>> = split_at_position_complete(
            "no_delimiter",
            predicate,
        );
        assert_eq!(pos1, Ok(("", "no_delimiter")));
        let pos2: IResult<&'static str, &'static str, Error<&'static str>> = split_at_position_complete(
            "",
            predicate,
        );
        assert_eq!(pos2, Ok(("", "")));
        let error: IResult<&'static str, &'static str, Error<&'static str>> = Err(
            Err::Error(Error::from_error_kind("input", ErrorKind::Tag)),
        );
        assert!(error.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_60 {
    use crate::traits::InputLength;
    #[test]
    fn input_len_for_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let length = input.input_len();
        debug_assert_eq!(length, 13);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_61 {
    use super::*;
    use crate::*;
    use crate::traits::Offset;
    #[test]
    fn test_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let slice = &input[rug_fuzz_1..];
        let offset_value = input.offset(slice);
        debug_assert_eq!(offset_value, 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_63 {
    use crate::AsChar;
    #[test]
    fn as_char_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let byte: u8 = rug_fuzz_0;
        debug_assert_eq!(byte.as_char(), 'A');
        let byte: u8 = rug_fuzz_1;
        debug_assert_eq!(byte.as_char(), ' ');
        let byte: u8 = rug_fuzz_2;
        debug_assert_eq!(byte.as_char(), '~');
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_64 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_alpha_uppercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for c in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(
                < & u8 as AsChar > ::is_alpha(& c), "Failed for uppercase letter: {}", c
                as char
            );
        }
             }
}
}
}    }
    #[test]
    fn test_is_alpha_lowercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for c in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(
                < & u8 as AsChar > ::is_alpha(& c), "Failed for lowercase letter: {}", c
                as char
            );
        }
             }
}
}
}    }
    #[test]
    fn test_is_alpha_non_alpha() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for c in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(
                ! < & u8 as AsChar > ::is_alpha(& c), "Failed for non-alpha: {}", c as
                char
            );
        }
        for c in rug_fuzz_2..=rug_fuzz_3 {
            debug_assert!(
                ! < & u8 as AsChar > ::is_alpha(& c), "Failed for non-alpha: {}", c as
                char
            );
        }
        for c in rug_fuzz_4..=rug_fuzz_5 {
            debug_assert!(
                ! < & u8 as AsChar > ::is_alpha(& c), "Failed for non-alpha: {}", c as
                char
            );
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;
    use crate::*;
    use crate::traits::AsChar;
    #[test]
    fn is_alphanum_alpha() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!((& rug_fuzz_0 as & u8).is_alphanum());
        debug_assert!((& rug_fuzz_1 as & u8).is_alphanum());
        debug_assert!((& rug_fuzz_2 as & u8).is_alphanum());
        debug_assert!((& rug_fuzz_3 as & u8).is_alphanum());
             }
}
}
}    }
    #[test]
    fn is_alphanum_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!((& rug_fuzz_0 as & u8).is_alphanum());
        debug_assert!((& rug_fuzz_1 as & u8).is_alphanum());
        debug_assert!((& rug_fuzz_2 as & u8).is_alphanum());
             }
}
}
}    }
    #[test]
    fn is_alphanum_non_alphanum() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! (& rug_fuzz_0 as & u8).is_alphanum());
        debug_assert!(! (& rug_fuzz_1 as & u8).is_alphanum());
        debug_assert!(! (& rug_fuzz_2 as & u8).is_alphanum());
        debug_assert!(! (& rug_fuzz_3 as & u8).is_alphanum());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_66 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_dec_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!((& rug_fuzz_0 as & u8).is_dec_digit());
        debug_assert!((& rug_fuzz_1 as & u8).is_dec_digit());
        debug_assert!((& rug_fuzz_2 as & u8).is_dec_digit());
        debug_assert!(! (& rug_fuzz_3 as & u8).is_dec_digit());
        debug_assert!(! (& rug_fuzz_4 as & u8).is_dec_digit());
        debug_assert!(! (& rug_fuzz_5 as & u8).is_dec_digit());
        debug_assert!(! (& rug_fuzz_6 as & u8).is_dec_digit());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_hex_digit_with_hex_digits() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hex_digits = vec![
            rug_fuzz_0, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B',
            b'C', b'D', b'E', b'F', b'a', b'b', b'c', b'd', b'e', b'f'
        ];
        for digit in hex_digits {
            debug_assert!(
                < & u8 as traits::AsChar > ::is_hex_digit(& digit),
                "Failed for digit: {}", digit as char
            );
        }
             }
}
}
}    }
    #[test]
    fn test_is_hex_digit_with_non_hex_digits() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_hex_digits = vec![
            rug_fuzz_0, b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R',
            b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'g', b'h', b'i', b'j', b'k',
            b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
            b'y', b'z', b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'-',
            b'+', b'=', b'{', b'}', b'[', b']', b'|', b':', b';', b'\'', b'"', b',',
            b'<', b'>', b'.', b'?', b'/', b'\\', b'`', b'~', b' '
        ];
        for digit in non_hex_digits {
            debug_assert!(
                ! < & u8 as traits::AsChar > ::is_hex_digit(& digit),
                "Failed for non-digit: {}", digit as char
            );
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_68 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_oct_digit() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_is_oct_digit = 0;
        let rug_fuzz_0 = b'0';
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = b'1';
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = b'2';
        let rug_fuzz_5 = true;
        let rug_fuzz_6 = b'3';
        let rug_fuzz_7 = true;
        let rug_fuzz_8 = b'4';
        let rug_fuzz_9 = true;
        let rug_fuzz_10 = b'5';
        let rug_fuzz_11 = true;
        let rug_fuzz_12 = b'6';
        let rug_fuzz_13 = true;
        let rug_fuzz_14 = b'7';
        let rug_fuzz_15 = true;
        let rug_fuzz_16 = b'8';
        let rug_fuzz_17 = false;
        let rug_fuzz_18 = b'9';
        let rug_fuzz_19 = false;
        let rug_fuzz_20 = b'a';
        let rug_fuzz_21 = false;
        let rug_fuzz_22 = b'z';
        let rug_fuzz_23 = false;
        let rug_fuzz_24 = b'/';
        let rug_fuzz_25 = false;
        let rug_fuzz_26 = b':';
        let rug_fuzz_27 = false;
        let tests = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (rug_fuzz_4, rug_fuzz_5),
            (rug_fuzz_6, rug_fuzz_7),
            (rug_fuzz_8, rug_fuzz_9),
            (rug_fuzz_10, rug_fuzz_11),
            (rug_fuzz_12, rug_fuzz_13),
            (rug_fuzz_14, rug_fuzz_15),
            (rug_fuzz_16, rug_fuzz_17),
            (rug_fuzz_18, rug_fuzz_19),
            (rug_fuzz_20, rug_fuzz_21),
            (rug_fuzz_22, rug_fuzz_23),
            (rug_fuzz_24, rug_fuzz_25),
            (rug_fuzz_26, rug_fuzz_27),
        ];
        for (input, expected) in tests.iter() {
            debug_assert_eq!(
                traits::AsChar::is_oct_digit(input), * expected,
                "Testing if '{}' is oct digit", * input as char
            );
        }
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_is_oct_digit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;
    use crate::*;
    use crate::traits::AsChar;
    #[test]
    fn test_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &u8 = &rug_fuzz_0;
        let result = AsChar::len(*input);
        debug_assert_eq!(result, 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_71_llm_16_71 {
    use crate::traits::FindToken;
    #[test]
    fn find_token_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        debug_assert!(
            < & [u8] as FindToken < & u8 > > ::find_token(& data, & rug_fuzz_5)
        );
        debug_assert!(
            ! < & [u8] as FindToken < & u8 > > ::find_token(& data, & rug_fuzz_6)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_token() {
        let _rug_st_tests_llm_16_72_rrrruuuugggg_test_find_token = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = b'h';
        let rug_fuzz_2 = b'e';
        let rug_fuzz_3 = b'l';
        let rug_fuzz_4 = b'o';
        let rug_fuzz_5 = b'x';
        let input = rug_fuzz_0;
        debug_assert!(
            < & 'static str as traits::FindToken < & u8 > > ::find_token(& input, &
            rug_fuzz_1)
        );
        debug_assert!(
            < & 'static str as traits::FindToken < & u8 > > ::find_token(& input, &
            rug_fuzz_2)
        );
        debug_assert!(
            < & 'static str as traits::FindToken < & u8 > > ::find_token(& input, &
            rug_fuzz_3)
        );
        debug_assert!(
            < & 'static str as traits::FindToken < & u8 > > ::find_token(& input, &
            rug_fuzz_4)
        );
        debug_assert!(
            ! < & 'static str as traits::FindToken < & u8 > > ::find_token(& input, &
            rug_fuzz_5)
        );
        let _rug_ed_tests_llm_16_72_rrrruuuugggg_test_find_token = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;
    use crate::*;
    #[test]
    fn extend_into_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let mut acc: Vec<u8> = Vec::new();
        input.extend_into(&mut acc);
        debug_assert_eq!(acc, vec![1, 2, 3]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_74 {
    use crate::ExtendInto;
    #[test]
    fn new_builder_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let builder = <&[u8] as ExtendInto>::new_builder(&bytes);
        debug_assert!(builder.is_empty(), "Builder should be empty");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_75 {
    use super::*;
    use crate::*;
    #[test]
    fn test_extend_into() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut result = String::from(rug_fuzz_1);
        input.extend_into(&mut result);
        debug_assert_eq!(result, "This is a test");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_76 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_builder() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let builder = <&str as traits::ExtendInto>::new_builder(&input);
        debug_assert_eq!(builder, String::new());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_77_llm_16_77 {
    use crate::traits::InputLength;
    #[test]
    fn input_len_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = (&[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2] as &[u8], rug_fuzz_3);
        debug_assert_eq!(input.input_len(), 16);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_78 {
    use super::*;
    use crate::*;
    use crate::error::ErrorKind;
    #[test]
    fn convert_preserves_input_and_error_kind() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let error_kind = ErrorKind::Tag;
        let error = ((input, rug_fuzz_1), error_kind);
        let result = <((_, _), _) as traits::ErrorConvert<(_, _)>>::convert(error);
        debug_assert_eq!(result, (input, ErrorKind::Tag));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_83 {
    use super::*;
    use crate::*;
    #[test]
    fn test_convert() {
        <() as traits::ErrorConvert<()>>::convert(())
    }
}
#[cfg(test)]
mod tests_llm_16_149 {
    use super::*;
    use crate::*;
    use crate::error::ErrorKind;
    #[test]
    fn test_convert() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: (&str, ErrorKind) = (rug_fuzz_0, ErrorKind::Tag);
        let converted = ErrorConvert::convert(input);
        debug_assert_eq!(converted, (("input_data", 0), ErrorKind::Tag));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_172 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_bytes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let bytes = <[u8; 4] as traits::AsBytes>::as_bytes(&array);
        debug_assert_eq!(bytes, & [1, 2, 3, 4]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_173 {
    use crate::traits::FindToken;
    #[test]
    fn find_token_in_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        debug_assert!(
            < [u8; 4] as FindToken < & u8 > > ::find_token(& array, & rug_fuzz_4)
        );
        debug_assert!(
            ! < [u8; 4] as FindToken < & u8 > > ::find_token(& array, & rug_fuzz_5)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_174 {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_token() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        debug_assert!(
            < [u8; 5] as traits::FindToken < u8 > > ::find_token(& data, rug_fuzz_5)
        );
        debug_assert!(
            ! < [u8; 5] as traits::FindToken < u8 > > ::find_token(& data, rug_fuzz_6)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_175 {
    use crate::traits::InputLength;
    #[test]
    fn test_input_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        debug_assert_eq!(input.input_len(), 4);
        let empty_input: [u8; 0] = [];
        debug_assert_eq!(empty_input.input_len(), 0);
        let large_input: [u8; 1024] = [rug_fuzz_4; 1024];
        debug_assert_eq!(large_input.input_len(), 1024);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_176 {
    use super::*;
    use crate::*;
    #[test]
    fn as_bytes_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let bytes = <[u8] as traits::AsBytes>::as_bytes(data);
        debug_assert_eq!(bytes, & [1, 2, 3, 4, 5]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_177 {
    use super::*;
    use crate::*;
    #[test]
    fn test_extend_into() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut acc = Vec::new();
        <[u8] as traits::ExtendInto>::extend_into(&input, &mut acc);
        debug_assert_eq!(acc, vec![1, 2, 3, 4, 5]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_178 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_builder() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_slice: &[u8] = &[
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let builder = <[u8] as traits::ExtendInto>::new_builder(&input_slice);
        debug_assert!(builder.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_179 {
    use super::*;
    use crate::*;
    use crate::traits::HexDisplay;
    #[test]
    fn to_hex_no_chunking() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2)) = <([u8; 13], &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let bytes = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(bytes.to_hex(rug_fuzz_2), expected);
             }
}
}
}    }
    #[test]
    fn to_hex_chunking() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2)) = <([u8; 13], &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let bytes = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(bytes.to_hex(rug_fuzz_2), expected);
             }
}
}
}    }
    #[test]
    fn to_hex_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2)) = <([u8; 0], &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let bytes = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(bytes.to_hex(rug_fuzz_2), expected);
             }
}
}
}    }
    #[test]
    fn to_hex_single_byte() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2)) = <([u8; 1], &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let bytes = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(bytes.to_hex(rug_fuzz_2), expected);
             }
}
}
}    }
    #[test]
    fn to_hex_single_chunk() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2)) = <([u8; 4], &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let bytes = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(bytes.to_hex(rug_fuzz_2), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_180 {
    use crate::traits::HexDisplay;
    static CHARS: &[u8; 16] = b"0123456789abcdef";
    #[test]
    fn test_to_hex_from() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <([u8; 44], usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let hex = <[u8] as HexDisplay>::to_hex_from(data, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            hex,
            "00000000\t54 68 65 20 71 75 69 63 \tThe quic\n\
             00000008\t6b 20 62 72 6f 77 6e 20 \tk brown \n\
             00000010\t66 6f 78 20 6a 75 6d 70 \tfox jump\n\
             00000018\t73 20 6f 76 65 72 20 74 \ts over t\n\
             00000020\t68 65 20 6c 61 7a 79 20 \the lazy \n\
             00000028\t64 6f 67 2e             \tdog.\n"
        );
        let hex_offset = <[u8] as HexDisplay>::to_hex_from(data, rug_fuzz_3, rug_fuzz_4);
        debug_assert_eq!(
            hex_offset,
            "00000010\t66 6f 78 20 6a 75 6d 70 \tfox jump\n\
             00000018\t73 20 6f 76 65 72 20 74 \ts over t\n\
             00000020\t68 65 20 6c 61 7a 79 20 \the lazy \n\
             00000028\t64 6f 67 2e             \tdog.\n"
        );
        let hex_small_chunk = <[u8] as HexDisplay>::to_hex_from(
            data,
            rug_fuzz_5,
            rug_fuzz_6,
        );
        debug_assert_eq!(
            hex_small_chunk,
            "00000008\t6b 20 62 72 \tk br\n\
             0000000c\t6f 77 6e 20 \town \n\
             00000010\t66 6f 78 20 \tfox \n\
             00000014\t6a 75 6d 70 \tjump\n\
             00000018\t73 20 6f 76 \ts ov\n\
             0000001c\t65 72 20 74 \ter t\n\
             00000020\t68 65 20 6c \the l\n\
             00000024\t61 7a 79 20 \tazy \n\
             00000028\t64 6f 67 2e \tdog.\n"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_181 {
    use crate::Offset;
    #[test]
    fn test_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub: &[u8] = &data[rug_fuzz_5..];
        let sub_offset = data.offset(sub);
        debug_assert_eq!(sub_offset, 1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_offset_panic() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub: &[u8] = &[rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let _sub_offset = data.offset(sub);
             }
}
}
}    }
    #[test]
    fn test_offset_same_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let sub_offset = data.offset(data);
        debug_assert_eq!(sub_offset, 0);
             }
}
}
}    }
    #[test]
    fn test_offset_with_empty_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let empty_slice: &[u8] = &[];
        let offset_from_empty = empty_slice.offset(data);
        debug_assert_eq!(offset_from_empty, 0);
        let offset_to_empty = data.offset(empty_slice);
        debug_assert_eq!(offset_to_empty, data.len());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_182 {
    use super::*;
    use crate::*;
    #[test]
    fn as_char_for_char() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        debug_assert_eq!(c.as_char(), 'a');
             }
}
}
}    }
    #[test]
    fn as_char_for_uppercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        debug_assert_eq!(c.as_char(), 'A');
             }
}
}
}    }
    #[test]
    fn as_char_for_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        debug_assert_eq!(c.as_char(), '1');
             }
}
}
}    }
    #[test]
    fn as_char_for_special_character() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        debug_assert_eq!(c.as_char(), '@');
             }
}
}
}    }
    #[test]
    fn as_char_for_unicode() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        debug_assert_eq!(c.as_char(), 'ñ');
             }
}
}
}    }
    #[test]
    fn as_char_for_emoji() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        debug_assert_eq!(c.as_char(), '😊');
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_183 {
    use super::*;
    use crate::*;
    #[test]
    fn is_alpha_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_alpha());
        debug_assert!(rug_fuzz_1.is_alpha());
             }
}
}
}    }
    #[test]
    fn is_alpha_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_alpha());
        debug_assert!(! rug_fuzz_1.is_alpha());
        debug_assert!(! rug_fuzz_2.is_alpha());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_184_llm_16_184 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_alphanum() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(char, char, char, char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_alphanum());
        debug_assert!(rug_fuzz_1.is_alphanum());
        debug_assert!(rug_fuzz_2.is_alphanum());
        debug_assert!(rug_fuzz_3.is_alphanum());
        debug_assert!(! rug_fuzz_4.is_alphanum());
        debug_assert!(! rug_fuzz_5.is_alphanum());
        debug_assert!(! rug_fuzz_6.is_alphanum());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_185 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_dec_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(char, char, char, char, char, char, char, char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_dec_digit(), true);
        debug_assert_eq!(rug_fuzz_1.is_dec_digit(), true);
        debug_assert_eq!(rug_fuzz_2.is_dec_digit(), true);
        debug_assert_eq!(rug_fuzz_3.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_4.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_5.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_6.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_7.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_8.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_9.is_dec_digit(), false);
        debug_assert_eq!(rug_fuzz_10.is_dec_digit(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_186 {
    use crate::AsChar;
    #[test]
    fn test_is_hex_digit() {
        let _rug_st_tests_llm_16_186_rrrruuuugggg_test_is_hex_digit = 0;
        let rug_fuzz_0 = '0';
        let rug_fuzz_1 = '1';
        let rug_fuzz_2 = '2';
        let rug_fuzz_3 = '3';
        let rug_fuzz_4 = '4';
        let rug_fuzz_5 = '5';
        let rug_fuzz_6 = '6';
        let rug_fuzz_7 = '7';
        let rug_fuzz_8 = '8';
        let rug_fuzz_9 = '9';
        let rug_fuzz_10 = 'a';
        let rug_fuzz_11 = 'A';
        let rug_fuzz_12 = 'b';
        let rug_fuzz_13 = 'B';
        let rug_fuzz_14 = 'c';
        let rug_fuzz_15 = 'C';
        let rug_fuzz_16 = 'd';
        let rug_fuzz_17 = 'D';
        let rug_fuzz_18 = 'e';
        let rug_fuzz_19 = 'E';
        let rug_fuzz_20 = 'f';
        let rug_fuzz_21 = 'F';
        let rug_fuzz_22 = 'g';
        let rug_fuzz_23 = 'z';
        let rug_fuzz_24 = 'G';
        let rug_fuzz_25 = 'Z';
        let rug_fuzz_26 = '@';
        let rug_fuzz_27 = '[';
        let rug_fuzz_28 = '`';
        let rug_fuzz_29 = '{';
        let rug_fuzz_30 = ' ';
        let rug_fuzz_31 = '.';
        let rug_fuzz_32 = '/';
        let rug_fuzz_33 = ':';
        debug_assert_eq!(rug_fuzz_0.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_1.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_2.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_3.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_4.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_5.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_6.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_7.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_8.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_9.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_10.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_11.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_12.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_13.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_14.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_15.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_16.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_17.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_18.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_19.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_20.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_21.is_hex_digit(), true);
        debug_assert_eq!(rug_fuzz_22.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_23.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_24.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_25.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_26.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_27.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_28.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_29.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_30.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_31.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_32.is_hex_digit(), false);
        debug_assert_eq!(rug_fuzz_33.is_hex_digit(), false);
        let _rug_ed_tests_llm_16_186_rrrruuuugggg_test_is_hex_digit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_187 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_oct_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(char, char, char, char, char, char, char, char, char, char, char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_1.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_2.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_3.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_4.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_5.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_6.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_7.is_oct_digit(), true);
        debug_assert_eq!(rug_fuzz_8.is_oct_digit(), false);
        debug_assert_eq!(rug_fuzz_9.is_oct_digit(), false);
        debug_assert_eq!(rug_fuzz_10.is_oct_digit(), false);
        debug_assert_eq!(rug_fuzz_11.is_oct_digit(), false);
        debug_assert_eq!(rug_fuzz_12.is_oct_digit(), false);
        debug_assert_eq!(rug_fuzz_13.is_oct_digit(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_188 {
    use super::*;
    use crate::*;
    #[test]
    fn len_utf8_char() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.len(), 1);
        debug_assert_eq!(rug_fuzz_1.len(), 2);
        debug_assert_eq!(rug_fuzz_2.len(), 4);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_189 {
    use crate::ExtendInto;
    #[test]
    fn test_extend_into() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(char, char, char, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut result = String::new();
        let c = rug_fuzz_0;
        c.extend_into(&mut result);
        debug_assert_eq!(result, "a");
        result.clear();
        let c = rug_fuzz_1;
        c.extend_into(&mut result);
        debug_assert_eq!(result, "b");
        result.clear();
        let c = rug_fuzz_2;
        c.extend_into(&mut result);
        debug_assert_eq!(result, "1");
        result.clear();
        let c = rug_fuzz_3;
        c.extend_into(&mut result);
        debug_assert_eq!(result, "#");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_190 {
    use super::*;
    use crate::*;
    #[test]
    fn new_builder_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let c = rug_fuzz_0;
        let builder = <char as traits::ExtendInto>::new_builder(&c);
        debug_assert_eq!(builder, String::new());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_195_llm_16_195 {
    use super::*;
    use crate::*;
    use crate::error::{Error, ErrorKind};
    use crate::traits::ErrorConvert;
    #[test]
    fn error_convert_trait_impl_for_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let error: Error<&str> = Error {
            input: rug_fuzz_0,
            code: ErrorKind::Tag,
        };
        let converted_error: Error<(&str, usize)> = error.convert();
        debug_assert_eq!(converted_error.input.0, "input_data");
        debug_assert_eq!(converted_error.input.1, 0);
        debug_assert_eq!(converted_error.code, ErrorKind::Tag);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_198_llm_16_198 {
    use super::*;
    use crate::*;
    use crate::error::{ParseError, VerboseError, VerboseErrorKind};
    use crate::traits::ErrorConvert;
    #[test]
    fn test_convert() {
        let _rug_st_tests_llm_16_198_llm_16_198_rrrruuuugggg_test_convert = 0;
        let rug_fuzz_0 = "test input";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "test context";
        let rug_fuzz_3 = "test input";
        let rug_fuzz_4 = "test context";
        let input = ((rug_fuzz_0, rug_fuzz_1), VerboseErrorKind::Context(rug_fuzz_2));
        let tuple_error = VerboseError {
            errors: vec![input],
        };
        let converted_error: VerboseError<&str> = tuple_error.convert();
        let expected_error = VerboseError {
            errors: vec![(rug_fuzz_3, VerboseErrorKind::Context(rug_fuzz_4))],
        };
        debug_assert_eq!(converted_error, expected_error);
        let _rug_ed_tests_llm_16_198_llm_16_198_rrrruuuugggg_test_convert = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_204_llm_16_204 {
    use crate::error::{VerboseError, VerboseErrorKind, ParseError, ErrorKind};
    use crate::traits::ErrorConvert;
    #[test]
    fn convert_verbose_error() {
        let _rug_st_tests_llm_16_204_llm_16_204_rrrruuuugggg_convert_verbose_error = 0;
        let rug_fuzz_0 = "input1";
        let rug_fuzz_1 = "context1";
        let rug_fuzz_2 = "input1";
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = "context1";
        let input_error: VerboseError<&str> = VerboseError {
            errors: vec![
                (rug_fuzz_0, VerboseErrorKind::Context(rug_fuzz_1)), ("input2",
                VerboseErrorKind::Char('a')), ("input3",
                VerboseErrorKind::Nom(ErrorKind::Tag))
            ],
        };
        let converted_error = <VerboseError<
            &str,
        > as ErrorConvert<VerboseError<(&str, usize)>>>::convert(input_error);
        let expected_errors = vec![
            ((rug_fuzz_2, rug_fuzz_3), VerboseErrorKind::Context(rug_fuzz_4)),
            (("input2", 0), VerboseErrorKind::Char('a')), (("input3", 0),
            VerboseErrorKind::Nom(ErrorKind::Tag))
        ];
        debug_assert_eq!(converted_error.errors, expected_errors);
        let _rug_ed_tests_llm_16_204_llm_16_204_rrrruuuugggg_convert_verbose_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_217 {
    use crate::NomRange;
    use std::ops::Range;
    #[test]
    fn test_bounded_iter_with_end_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: Range<usize> = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let bounded = range.bounded_iter();
        debug_assert_eq!(bounded, 1..0);
        debug_assert!(bounded.is_empty());
             }
}
}
}    }
    #[test]
    fn test_bounded_iter_with_end_non_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: Range<usize> = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let bounded = range.bounded_iter();
        debug_assert_eq!(bounded, 0..9);
        debug_assert_eq!(bounded.count(), 9);
             }
}
}
}    }
    #[test]
    fn test_bounded_iter_end_exclusive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: Range<usize> = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let bounded = range.bounded_iter();
        for i in bounded {
            debug_assert!(range.contains(& i));
        }
        debug_assert!(! range.contains(& (range.end - rug_fuzz_2)));
             }
}
}
}    }
}
#[cfg(test)]
mod test {
    use std::ops::{Bound, Range};
    use crate::traits::NomRange;
    #[test]
    fn test_bounds() {
        let range = Range { start: 10, end: 20 };
        let bounds = NomRange::bounds(&range);
        assert_eq!(bounds, (Bound::Included(10), Bound::Excluded(20)));
    }
    #[test]
    fn test_bounds_empty() {
        let range = Range { start: 0, end: 0 };
        let bounds = NomRange::bounds(&range);
        assert_eq!(bounds, (Bound::Included(0), Bound::Excluded(0)));
    }
}
#[cfg(test)]
mod tests_llm_16_219 {
    use super::*;
    use crate::*;
    use std::ops::Range;
    use crate::traits::NomRange;
    use std::ops::Bound;
    #[test]
    fn test_contains() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        debug_assert!(range.contains(& rug_fuzz_2));
        debug_assert!(range.contains(& rug_fuzz_3));
        debug_assert!(range.contains(& rug_fuzz_4));
        debug_assert!(! range.contains(& rug_fuzz_5));
        debug_assert!(! range.contains(& rug_fuzz_6));
        debug_assert!(! range.contains(& rug_fuzz_7));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_220 {
    use std::ops::Range;
    use crate::traits::NomRange;
    #[test]
    fn test_is_inverted_not_inverted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: Range<usize> = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        debug_assert_eq!(range.is_inverted(), false);
             }
}
}
}    }
    #[test]
    fn test_is_inverted_inverted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: Range<usize> = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        debug_assert_eq!(range.is_inverted(), true);
             }
}
}
}    }
    #[test]
    fn test_is_inverted_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: Range<usize> = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        debug_assert_eq!(range.is_inverted(), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_221 {
    use super::*;
    use crate::*;
    use std::ops::{Bound, RangeBounds};
    #[test]
    fn test_saturating_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_empty_end_at_zero = rug_fuzz_0..rug_fuzz_1;
        let range_non_empty = rug_fuzz_2..rug_fuzz_3;
        let range_end_at_zero = rug_fuzz_4..rug_fuzz_5;
        let sat_iter_empty = range_empty_end_at_zero.saturating_iter();
        let sat_iter_non_empty = range_non_empty.saturating_iter();
        let sat_iter_end_at_zero = range_end_at_zero.saturating_iter();
        debug_assert_eq!(sat_iter_empty, 1..0);
        debug_assert_eq!(sat_iter_non_empty, 0..4);
        debug_assert_eq!(sat_iter_end_at_zero, 1..0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_222 {
    use super::*;
    use crate::*;
    use std::ops::RangeFrom;
    #[test]
    fn test_bounded_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_from = RangeFrom { start: rug_fuzz_0 };
        let bounded = <RangeFrom<
            usize,
        > as traits::NomRange<usize>>::bounded_iter(&range_from);
        debug_assert_eq!(bounded.start, 0);
        debug_assert_eq!(bounded.end, core::usize::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_223 {
    use std::ops::{Bound, RangeFrom};
    use crate::NomRange;
    #[test]
    fn range_from_bounds_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_from = RangeFrom { start: rug_fuzz_0 };
        let bounds = <RangeFrom<usize> as NomRange<usize>>::bounds(&range_from);
        debug_assert_eq!(bounds, (Bound::Included(5), Bound::Unbounded));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_224 {
    use std::ops::RangeFrom;
    use crate::traits::NomRange;
    #[test]
    fn test_contains() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range = RangeFrom { start: rug_fuzz_0 };
        debug_assert!(range.contains(& rug_fuzz_1));
        debug_assert!(range.contains(& rug_fuzz_2));
        debug_assert!(range.contains(& usize::MAX));
        debug_assert!(! range.contains(& rug_fuzz_3));
        debug_assert!(! range.contains(& rug_fuzz_4));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_225 {
    use super::*;
    use crate::*;
    use std::ops::RangeFrom;
    #[test]
    fn range_from_is_not_inverted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: RangeFrom<usize> = RangeFrom { start: rug_fuzz_0 };
        debug_assert_eq!(
            < RangeFrom < usize > as traits::NomRange < usize > > ::is_inverted(& range),
            false
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_226 {
    use super::*;
    use crate::*;
    use std::ops::RangeFrom;
    #[derive(Debug)]
    struct SaturatingIterator {
        count: usize,
    }
    trait NomRange<T> {
        type Saturating: Iterator<Item = T>;
        type Bounded: Iterator<Item = T>;
        fn bounds(&self) -> (Bound<T>, Bound<T>);
        fn contains(&self, item: &T) -> bool;
        fn is_inverted(&self) -> bool;
        fn saturating_iter(&self) -> Self::Saturating;
        fn bounded_iter(&self) -> Self::Bounded;
    }
    impl NomRange<usize> for RangeFrom<usize> {
        type Saturating = SaturatingIterator;
        type Bounded = Range<usize>;
        fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
            (Bound::Included(self.start), Bound::Unbounded)
        }
        fn contains(&self, item: &usize) -> bool {
            RangeBounds::contains(self, item)
        }
        fn is_inverted(&self) -> bool {
            false
        }
        fn saturating_iter(&self) -> Self::Saturating {
            SaturatingIterator { count: 0 }
        }
        fn bounded_iter(&self) -> Self::Bounded {
            0..usize::MAX
        }
    }
    #[test]
    fn test_saturating_iter() {
        let range_from = RangeFrom { start: 0 };
        let mut sat_iter = range_from.saturating_iter();
        impl Iterator for SaturatingIterator {
            type Item = usize;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count == usize::MAX {
                    None
                } else {
                    self.count += 1;
                    Some(self.count - 1)
                }
            }
        }
        assert_eq!(sat_iter.next(), Some(0));
        assert_eq!(sat_iter.next(), Some(1));
        let large_step = usize::MAX - 10;
        for _ in 0..large_step {
            sat_iter.next();
        }
        assert_eq!(sat_iter.next(), Some(usize::MAX - 9));
        for _ in 0..20 {
            assert_eq!(sat_iter.next(), None);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_227 {
    use super::*;
    use crate::*;
    use std::ops::{RangeFull, Bound, RangeBounds};
    use std::usize;
    #[test]
    fn test_bounded_iter() {
        let _rug_st_tests_llm_16_227_rrrruuuugggg_test_bounded_iter = 0;
        let range_full: RangeFull = ..;
        let bounded = range_full.bounded_iter();
        debug_assert_eq!(bounded.start, 0);
        debug_assert_eq!(bounded.end, usize::MAX);
        let _rug_ed_tests_llm_16_227_rrrruuuugggg_test_bounded_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_228 {
    use std::ops::{Bound, RangeFull};
    use crate::traits::NomRange;
    #[test]
    fn test_bounds() {
        let _rug_st_tests_llm_16_228_rrrruuuugggg_test_bounds = 0;
        let range_full = RangeFull;
        let (start_bound, end_bound) = range_full.bounds();
        debug_assert_eq!(start_bound, Bound::Unbounded);
        debug_assert_eq!(end_bound, Bound::Unbounded);
        let _rug_ed_tests_llm_16_228_rrrruuuugggg_test_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_229_llm_16_229 {
    use std::ops::RangeFull;
    use crate::traits::NomRange;
    #[test]
    fn range_full_contains_always_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_full = RangeFull;
        debug_assert!(NomRange::contains(& range_full, & rug_fuzz_0));
        debug_assert!(NomRange::contains(& range_full, & rug_fuzz_1));
        debug_assert!(NomRange::contains(& range_full, & usize::MAX));
        debug_assert!(NomRange::contains(& range_full, & (usize::MAX / rug_fuzz_2)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_230 {
    use std::ops::RangeFull;
    use crate::traits::NomRange;
    #[test]
    fn range_full_is_not_inverted() {
        let _rug_st_tests_llm_16_230_rrrruuuugggg_range_full_is_not_inverted = 0;
        let range_full = RangeFull;
        debug_assert_eq!(range_full.is_inverted(), false);
        let _rug_ed_tests_llm_16_230_rrrruuuugggg_range_full_is_not_inverted = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_231 {
    use super::*;
    use crate::*;
    use std::ops::Bound;
    use std::ops::RangeFull;
    pub struct SaturatingIterator {
        count: usize,
    }
    pub trait NomRange<T>: RangeBounds<T> {
        type Saturating;
        type Bounded;
        fn bounds(&self) -> (Bound<T>, Bound<T>);
        fn contains(&self, item: &T) -> bool;
        fn is_inverted(&self) -> bool;
        fn saturating_iter(&self) -> Self::Saturating;
        fn bounded_iter(&self) -> Self::Bounded;
    }
    impl NomRange<usize> for RangeFull {
        type Saturating = SaturatingIterator;
        type Bounded = Range<usize>;
        fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
            (Bound::Unbounded, Bound::Unbounded)
        }
        fn contains(&self, item: &usize) -> bool {
            RangeBounds::contains(self, item)
        }
        fn is_inverted(&self) -> bool {
            false
        }
        fn saturating_iter(&self) -> Self::Saturating {
            SaturatingIterator { count: 0 }
        }
        fn bounded_iter(&self) -> Self::Bounded {
            0..core::usize::MAX
        }
    }
    #[test]
    fn saturating_iter_test() {
        let _rug_st_tests_llm_16_231_rrrruuuugggg_saturating_iter_test = 0;
        let range_full = RangeFull;
        let saturating_iter = range_full.saturating_iter();
        debug_assert_eq!(saturating_iter.count, 0);
        let _rug_ed_tests_llm_16_231_rrrruuuugggg_saturating_iter_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_232 {
    use std::ops::RangeInclusive;
    use crate::traits::NomRange;
    #[test]
    fn test_bounded_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: RangeInclusive<usize> = rug_fuzz_0..=rug_fuzz_1;
        let bounded_iter = <RangeInclusive<
            usize,
        > as NomRange<usize>>::bounded_iter(&range);
        let collected: Vec<usize> = bounded_iter.collect();
        debug_assert_eq!(collected, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_233 {
    use std::ops::{Bound, RangeInclusive};
    use crate::traits::NomRange;
    #[test]
    fn bounds_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range = RangeInclusive::new(rug_fuzz_0, rug_fuzz_1);
        let bounds = <RangeInclusive<usize> as NomRange<usize>>::bounds(&range);
        debug_assert_eq!(bounds, (Bound::Included(10), Bound::Included(20)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_234 {
    use std::ops::RangeInclusive;
    use crate::traits::NomRange;
    #[test]
    fn test_range_inclusive_contains() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range: RangeInclusive<usize> = (rug_fuzz_0..=rug_fuzz_1);
        let out_of_range_low = rug_fuzz_2;
        let in_range = rug_fuzz_3;
        let out_of_range_high = rug_fuzz_4;
        let at_lower_bound = rug_fuzz_5;
        let at_upper_bound = rug_fuzz_6;
        debug_assert!(! range.contains(& out_of_range_low));
        debug_assert!(range.contains(& in_range));
        debug_assert!(! range.contains(& out_of_range_high));
        debug_assert!(range.contains(& at_lower_bound));
        debug_assert!(range.contains(& at_upper_bound));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_235 {
    use std::ops::{RangeInclusive, Bound};
    use crate::NomRange;
    #[test]
    fn test_is_inverted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_not_inverted: RangeInclusive<usize> = RangeInclusive::new(
            rug_fuzz_0,
            rug_fuzz_1,
        );
        debug_assert!(
            ! range_not_inverted.is_inverted(), "Range should not be inverted"
        );
        let range_inverted: RangeInclusive<usize> = RangeInclusive::new(
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert!(range_inverted.is_inverted(), "Range should be inverted");
        let range_empty: RangeInclusive<usize> = RangeInclusive::new(
            rug_fuzz_4,
            rug_fuzz_5,
        );
        debug_assert!(! range_empty.is_inverted(), "Range should not be inverted");
        let range_inverted_single_value: RangeInclusive<usize> = RangeInclusive::new(
            rug_fuzz_6,
            rug_fuzz_7,
        );
        debug_assert!(
            range_inverted_single_value.is_inverted(),
            "Range with inverted single value should be inverted"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_236 {
    use std::ops::RangeInclusive;
    use std::ops::Bound;
    use crate::NomRange;
    #[test]
    fn test_saturating_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_inclusive: RangeInclusive<usize> = RangeInclusive::new(
            rug_fuzz_0,
            rug_fuzz_1,
        );
        let mut sat_iter = range_inclusive.saturating_iter();
        debug_assert_eq!(sat_iter.next(), Some(0));
        debug_assert_eq!(sat_iter.next(), Some(1));
        debug_assert_eq!(sat_iter.next(), Some(2));
        debug_assert_eq!(sat_iter.next(), Some(3));
        debug_assert_eq!(sat_iter.next(), Some(4));
        debug_assert_eq!(sat_iter.next(), Some(5));
        debug_assert_eq!(sat_iter.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_237 {
    use super::*;
    use crate::*;
    use std::ops::RangeTo;
    #[test]
    fn test_bounded_iter_non_empty_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        let res = <RangeTo<usize> as NomRange<usize>>::bounded_iter(&range_to);
        debug_assert_eq!(res, 0..4);
             }
}
}
}    }
    #[test]
    fn test_bounded_iter_empty_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        let res = <RangeTo<usize> as NomRange<usize>>::bounded_iter(&range_to);
        debug_assert_eq!(res, 1..0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_238 {
    use std::ops::{Bound, RangeTo};
    struct NomRange<T> {
        end: T,
    }
    trait NomRangeTrait<T> {
        fn bounds(&self) -> (Bound<T>, Bound<T>);
    }
    impl NomRangeTrait<usize> for NomRange<usize> {
        fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
            (Bound::Unbounded, Bound::Excluded(self.end))
        }
    }
    #[test]
    fn bounds_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = NomRange { end: rug_fuzz_0 };
        let (start_bound, end_bound) = range_to.bounds();
        debug_assert_eq!(start_bound, Bound::Unbounded);
        debug_assert_eq!(end_bound, Bound::Excluded(5));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_239_llm_16_239 {
    use std::ops::{Bound, RangeBounds, RangeTo};
    use crate::traits::NomRange;
    #[test]
    fn range_to_usize_contains_within_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        debug_assert!(range_to.contains(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn range_to_usize_contains_at_upper_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        debug_assert!(! range_to.contains(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn range_to_usize_contains_beyond_upper_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        debug_assert!(! range_to.contains(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn range_to_usize_contains_at_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        debug_assert!(range_to.contains(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "RangeTo<usize> does not support unbounded ranges.")]
    fn range_to_usize_contains_at_unbounded() {
        let _rug_st_tests_llm_16_239_llm_16_239_rrrruuuugggg_range_to_usize_contains_at_unbounded = 0;
        let range_to = RangeTo { end: usize::MAX };
        range_to.contains(&usize::MAX);
        let _rug_ed_tests_llm_16_239_llm_16_239_rrrruuuugggg_range_to_usize_contains_at_unbounded = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_240 {
    use std::ops::RangeTo;
    use crate::NomRange;
    #[test]
    fn is_inverted_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to: RangeTo<usize> = ..rug_fuzz_0;
        debug_assert!(
            ! < RangeTo < usize > as NomRange < usize > > ::is_inverted(& range_to)
        );
        let range_to_zero: RangeTo<usize> = ..rug_fuzz_1;
        debug_assert!(
            ! < RangeTo < usize > as NomRange < usize > > ::is_inverted(& range_to_zero)
        );
        let range_to_negative: RangeTo<usize> = ..usize::MAX;
        debug_assert!(
            ! < RangeTo < usize > as NomRange < usize > > ::is_inverted(&
            range_to_negative)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_241 {
    use std::ops::RangeTo;
    use crate::traits::NomRange;
    #[test]
    fn saturating_iter_non_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        let sat_iter = range_to.saturating_iter();
        let expected: Vec<usize> = (rug_fuzz_1..rug_fuzz_2).collect();
        let result: Vec<usize> = sat_iter.collect();
        debug_assert_eq!(expected, result);
             }
}
}
}    }
    #[test]
    fn saturating_iter_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to = RangeTo { end: rug_fuzz_0 };
        let sat_iter = range_to.saturating_iter();
        let expected: Vec<usize> = (rug_fuzz_1..rug_fuzz_2).collect();
        let result: Vec<usize> = sat_iter.collect();
        debug_assert_eq!(expected, result);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_242 {
    use std::ops::RangeToInclusive;
    use crate::NomRange;
    #[test]
    fn bounded_iter_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range = RangeToInclusive {
            end: rug_fuzz_0,
        };
        let mut iter = <RangeToInclusive<
            usize,
        > as NomRange<usize>>::bounded_iter(&range);
        let mut next_val = rug_fuzz_1;
        while let Some(val) = iter.next() {
            debug_assert_eq!(val, next_val);
            next_val += rug_fuzz_2;
        }
        debug_assert_eq!(next_val, 5);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_243 {
    use std::ops::{Bound, RangeToInclusive};
    use crate::traits::NomRange;
    #[test]
    fn test_bounds_for_range_to_inclusive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to_inclusive = RangeToInclusive {
            end: rug_fuzz_0,
        };
        let (lower_bound, upper_bound) = <RangeToInclusive<
            usize,
        > as NomRange<usize>>::bounds(&range_to_inclusive);
        debug_assert_eq!(lower_bound, Bound::Unbounded);
        debug_assert_eq!(upper_bound, Bound::Included(10));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_244 {
    use super::*;
    use crate::*;
    use std::ops::RangeBounds;
    use std::ops::RangeToInclusive;
    use crate::traits::NomRange;
    #[test]
    fn contains_inclusive_range_to() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range_to_inclusive = RangeToInclusive {
            end: rug_fuzz_0,
        };
        debug_assert!(range_to_inclusive.contains(& rug_fuzz_1));
        debug_assert!(range_to_inclusive.contains(& rug_fuzz_2));
        debug_assert!(! range_to_inclusive.contains(& rug_fuzz_3));
        debug_assert!(! range_to_inclusive.contains(& rug_fuzz_4));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_245 {
    use std::ops::RangeToInclusive;
    use crate::NomRange;
    #[test]
    fn test_is_inverted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range = RangeToInclusive {
            end: rug_fuzz_0,
        };
        debug_assert_eq!(range.is_inverted(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_246 {
    use std::ops::RangeToInclusive;
    use crate::traits::NomRange;
    #[test]
    fn saturating_iter_inclusive_range_to_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let range = RangeToInclusive {
            end: rug_fuzz_0,
        };
        let saturating_iter = <RangeToInclusive<
            usize,
        > as NomRange<usize>>::saturating_iter(&range);
        let collected: Vec<usize> = saturating_iter.collect();
        debug_assert_eq!(collected, (0..10).collect:: < Vec < usize > > ());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_248 {
    use crate::traits::AsBytes;
    #[test]
    fn test_as_bytes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let expected = input.as_bytes();
        debug_assert_eq!(< str as AsBytes > ::as_bytes(input), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_249 {
    use super::*;
    use crate::*;
    #[test]
    fn test_extend_into() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut accumulator = String::from(rug_fuzz_1);
        input.extend_into(&mut accumulator);
        debug_assert_eq!(accumulator, "World!Hello, ");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_250 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_builder() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let builder = <str as traits::ExtendInto>::new_builder(&input);
        debug_assert_eq!(builder, String::new());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_251 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_hex() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let chunk_size = rug_fuzz_1;
        let expected_output = rug_fuzz_2;
        let result = input.to_hex(chunk_size);
        debug_assert_eq!(result, expected_output);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_252 {
    use super::*;
    use crate::*;
    use crate::HexDisplay;
    #[test]
    fn test_to_hex_from() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let chunk_size = rug_fuzz_1;
        let from = rug_fuzz_2;
        let result = input.to_hex_from(chunk_size, from);
        debug_assert_eq!(result, "c123".to_hex(chunk_size));
        let chunk_size_zero = rug_fuzz_3;
        let result_zero_chunk = input.to_hex_from(chunk_size_zero, from);
        debug_assert_eq!(result_zero_chunk, "c123".to_hex(chunk_size_zero));
        let from_beyond = rug_fuzz_4;
        let result_beyond = input.to_hex_from(chunk_size, from_beyond);
        debug_assert_eq!(result_beyond, "".to_hex(chunk_size));
        let chunk_size_large = rug_fuzz_5;
        let result_large_chunk = input.to_hex_from(chunk_size_large, from);
        debug_assert_eq!(result_large_chunk, "c123".to_hex(chunk_size_large));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_253 {
    use super::*;
    use crate::*;
    #[test]
    fn test_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let substr = &input[rug_fuzz_1..];
        debug_assert_eq!(input.offset(substr), 7);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_254 {
    use crate::traits::SaturatingIterator;
    #[test]
    fn test_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = SaturatingIterator {
            count: rug_fuzz_0,
        };
        debug_assert_eq!(iterator.next(), Some(0));
        debug_assert_eq!(iterator.count, 1);
        debug_assert_eq!(iterator.next(), Some(1));
        debug_assert_eq!(iterator.count, 2);
        iterator.count = usize::MAX;
        debug_assert_eq!(iterator.next(), Some(usize::MAX));
        debug_assert_eq!(iterator.count, usize::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_255 {
    use crate::traits::ToUsize;
    #[test]
    fn test_to_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u16 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, 42_usize);
             }
}
}
}    }
    #[test]
    fn test_to_usize_large_number() {
        let _rug_st_tests_llm_16_255_rrrruuuugggg_test_to_usize_large_number = 0;
        let value: u16 = u16::MAX;
        let result = value.to_usize();
        debug_assert_eq!(result, u16::MAX as usize);
        let _rug_ed_tests_llm_16_255_rrrruuuugggg_test_to_usize_large_number = 0;
    }
    #[test]
    fn test_to_usize_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u16 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, 0_usize);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_256 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u32 = rug_fuzz_0;
        let result: usize = value.to_usize();
        debug_assert_eq!(result, 42usize);
             }
}
}
}    }
    #[test]
    fn test_to_usize_max() {
        let _rug_st_tests_llm_16_256_rrrruuuugggg_test_to_usize_max = 0;
        let value: u32 = u32::MAX;
        let result: usize = value.to_usize();
        debug_assert_eq!(result, usize::try_from(u32::MAX).unwrap_or(usize::MAX));
        let _rug_ed_tests_llm_16_256_rrrruuuugggg_test_to_usize_max = 0;
    }
    #[test]
    fn test_to_usize_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u32 = rug_fuzz_0;
        let result: usize = value.to_usize();
        debug_assert_eq!(result, 0usize);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_257 {
    use super::*;
    use crate::*;
    use crate::traits::ToUsize;
    #[test]
    fn test_to_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u64 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, 42_usize);
             }
}
}
}    }
    #[test]
    fn test_to_usize_large_value() {
        let _rug_st_tests_llm_16_257_rrrruuuugggg_test_to_usize_large_value = 0;
        let value: u64 = u64::MAX;
        if let Ok(max_usize) = usize::try_from(value) {
            let result = value.to_usize();
            debug_assert_eq!(result, max_usize);
        } else {
            panic!("u64::MAX does not fit into usize on this platform");
        }
        let _rug_ed_tests_llm_16_257_rrrruuuugggg_test_to_usize_large_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_258 {
    use super::*;
    use crate::*;
    #[test]
    fn u8_as_char() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).as_char(), 'A');
        debug_assert_eq!((rug_fuzz_1).as_char(), 'a');
        debug_assert_eq!((rug_fuzz_2).as_char(), '0');
        debug_assert_eq!((rug_fuzz_3).as_char(), ' ');
        debug_assert_eq!((rug_fuzz_4).as_char(), '\0');
        debug_assert_eq!((rug_fuzz_5).as_char(), 'ÿ');
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_259 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_alpha() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0 as u8).is_alpha(), true);
        debug_assert_eq!((rug_fuzz_1 as u8).is_alpha(), true);
        debug_assert_eq!((rug_fuzz_2 as u8).is_alpha(), true);
        debug_assert_eq!((rug_fuzz_3 as u8).is_alpha(), true);
        debug_assert_eq!((rug_fuzz_4 as u8).is_alpha(), true);
        debug_assert_eq!((rug_fuzz_5 as u8).is_alpha(), true);
        debug_assert_eq!((rug_fuzz_6 as u8).is_alpha(), false);
        debug_assert_eq!((rug_fuzz_7 as u8).is_alpha(), false);
        debug_assert_eq!((rug_fuzz_8 as u8).is_alpha(), false);
        debug_assert_eq!((rug_fuzz_9 as u8).is_alpha(), false);
        debug_assert_eq!((rug_fuzz_10 as u8).is_alpha(), false);
        debug_assert_eq!((rug_fuzz_11 as u8).is_alpha(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_260 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_alphanum_alpha_lower() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_alpha_upper() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_non_alphanum() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_boundary_lower() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_alphanum());
             }
}
}
}    }
    #[test]
    fn test_is_alphanum_boundary_upper() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_alphanum());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_261 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_dec_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< u8 as crate ::traits::AsChar > ::is_dec_digit(rug_fuzz_0));
        debug_assert!(< u8 as crate ::traits::AsChar > ::is_dec_digit(rug_fuzz_1));
        debug_assert!(! < u8 as crate ::traits::AsChar > ::is_dec_digit(rug_fuzz_2));
        debug_assert!(! < u8 as crate ::traits::AsChar > ::is_dec_digit(rug_fuzz_3));
        debug_assert!(! < u8 as crate ::traits::AsChar > ::is_dec_digit(rug_fuzz_4));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_262 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_hex_digit_numbers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
             }
}
}
}    }
    #[test]
    fn test_is_hex_digit_uppercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
             }
}
}
}    }
    #[test]
    fn test_is_hex_digit_lowercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
             }
}
}
}    }
    #[test]
    fn test_is_hex_digit_non_hex_uppercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(! i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
             }
}
}
}    }
    #[test]
    fn test_is_hex_digit_non_hex_lowercase() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(! i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
             }
}
}
}    }
    #[test]
    fn test_is_hex_digit_non_hex_numbers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(! i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
        for i in rug_fuzz_2..=rug_fuzz_3 {
            debug_assert!(! i.is_hex_digit(), "Failed for i = {:#X}", i);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_263 {
    use crate::traits::AsChar;
    #[test]
    fn test_is_oct_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_oct_digit());
        debug_assert!(rug_fuzz_1.is_oct_digit());
        debug_assert!(rug_fuzz_2.is_oct_digit());
        debug_assert!(rug_fuzz_3.is_oct_digit());
        debug_assert!(rug_fuzz_4.is_oct_digit());
        debug_assert!(rug_fuzz_5.is_oct_digit());
        debug_assert!(rug_fuzz_6.is_oct_digit());
        debug_assert!(rug_fuzz_7.is_oct_digit());
        debug_assert!(! rug_fuzz_8.is_oct_digit());
        debug_assert!(! rug_fuzz_9.is_oct_digit());
        debug_assert!(! rug_fuzz_10.is_oct_digit());
        debug_assert!(! rug_fuzz_11.is_oct_digit());
        debug_assert!(! rug_fuzz_12.is_oct_digit());
        debug_assert!(! rug_fuzz_13.is_oct_digit());
        debug_assert!(! rug_fuzz_14.is_oct_digit());
        debug_assert!(! rug_fuzz_15.is_oct_digit());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_264 {
    use super::*;
    use crate::*;
    #[test]
    fn u8_len_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: u8 = rug_fuzz_0;
        let length = <u8 as traits::AsChar>::len(input);
        debug_assert_eq!(length, 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_265 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u8 = rug_fuzz_0;
        debug_assert_eq!(value.to_usize(), 100_usize);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_266 {
    use super::*;
    use crate::*;
    #[test]
    fn test_bounded_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: usize = rug_fuzz_0;
        let mut result_iter = <usize as traits::NomRange<usize>>::bounded_iter(&value);
        let mut collected = Vec::new();
        while let Some(item) = result_iter.next() {
            collected.push(item);
        }
        debug_assert_eq!(collected, vec![0, 1, 2, 3, 4]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_267 {
    use super::*;
    use crate::*;
    use crate::traits::NomRange;
    use std::ops::Bound;
    #[test]
    fn test_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: usize = rug_fuzz_0;
        let (start, end) = <usize as NomRange<usize>>::bounds(&value);
        debug_assert_eq!(start, Bound::Included(10));
        debug_assert_eq!(end, Bound::Included(10));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_268 {
    use crate::traits::*;
    #[test]
    fn test_contains() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            < usize as NomRange < usize > > ::contains(& rug_fuzz_0, & rug_fuzz_1)
        );
        debug_assert!(
            ! < usize as NomRange < usize > > ::contains(& rug_fuzz_2, & rug_fuzz_3)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_269 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_inverted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as traits::NomRange < usize > > ::is_inverted(& rug_fuzz_0), false
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_270 {
    use super::*;
    use crate::*;
    use crate::traits::NomRange;
    #[test]
    fn saturating_iter_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: usize = rug_fuzz_0;
        let iterator = <usize as NomRange<usize>>::saturating_iter(&num);
        let collected: Vec<usize> = iterator.collect();
        debug_assert_eq!(collected, vec![0, 1, 2, 3, 4]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_271 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: usize = rug_fuzz_0;
        debug_assert_eq!(value.to_usize(), 42);
             }
}
}
}    }
}
