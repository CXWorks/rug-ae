use crate::error::{Error, ErrorCode, Result};
use alloc::vec::Vec;
use core::char;
use core::cmp;
use core::ops::Deref;
use core::str;
#[cfg(feature = "std")]
use crate::io;
#[cfg(feature = "std")]
use crate::iter::LineColIterator;
#[cfg(feature = "raw_value")]
use crate::raw::BorrowedRawDeserializer;
#[cfg(all(feature = "raw_value", feature = "std"))]
use crate::raw::OwnedRawDeserializer;
#[cfg(feature = "raw_value")]
use serde::de::Visitor;
/// Trait used by the deserializer for iterating over input. This is manually
/// "specialized" for iterating over &[u8]. Once feature(specialization) is
/// stable we can use actual specialization.
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json`.
pub trait Read<'de>: private::Sealed {
    #[doc(hidden)]
    fn next(&mut self) -> Result<Option<u8>>;
    #[doc(hidden)]
    fn peek(&mut self) -> Result<Option<u8>>;
    /// Only valid after a call to peek(). Discards the peeked byte.
    #[doc(hidden)]
    fn discard(&mut self);
    /// Position of the most recent call to next().
    ///
    /// The most recent call was probably next() and not peek(), but this method
    /// should try to return a sensible result if the most recent call was
    /// actually peek() because we don't always know.
    ///
    /// Only called in case of an error, so performance is not important.
    #[doc(hidden)]
    fn position(&self) -> Position;
    /// Position of the most recent call to peek().
    ///
    /// The most recent call was probably peek() and not next(), but this method
    /// should try to return a sensible result if the most recent call was
    /// actually next() because we don't always know.
    ///
    /// Only called in case of an error, so performance is not important.
    #[doc(hidden)]
    fn peek_position(&self) -> Position;
    /// Offset from the beginning of the input to the next byte that would be
    /// returned by next() or peek().
    #[doc(hidden)]
    fn byte_offset(&self) -> usize;
    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark using the given scratch space if
    /// necessary. The scratch space is initially empty.
    #[doc(hidden)]
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>>;
    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark using the given scratch space if
    /// necessary. The scratch space is initially empty.
    ///
    /// This function returns the raw bytes in the string with escape sequences
    /// expanded but without performing unicode validation.
    #[doc(hidden)]
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;
    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark but discards the data.
    #[doc(hidden)]
    fn ignore_str(&mut self) -> Result<()>;
    /// Assumes the previous byte was a hex escape sequnce ('\u') in a string.
    /// Parses next hexadecimal sequence.
    #[doc(hidden)]
    fn decode_hex_escape(&mut self) -> Result<u16>;
    /// Switch raw buffering mode on.
    ///
    /// This is used when deserializing `RawValue`.
    #[cfg(feature = "raw_value")]
    #[doc(hidden)]
    fn begin_raw_buffering(&mut self);
    /// Switch raw buffering mode off and provides the raw buffered data to the
    /// given visitor.
    #[cfg(feature = "raw_value")]
    #[doc(hidden)]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;
    /// Whether StreamDeserializer::next needs to check the failed flag. True
    /// for IoRead, false for StrRead and SliceRead which can track failure by
    /// truncating their input slice to avoid the extra check on every next
    /// call.
    #[doc(hidden)]
    const should_early_return_if_failed: bool;
    /// Mark a persistent failure of StreamDeserializer, either by setting the
    /// flag or by truncating the input data.
    #[doc(hidden)]
    fn set_failed(&mut self, failed: &mut bool);
}
pub struct Position {
    pub line: usize,
    pub column: usize,
}
pub enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'b T),
    Copied(&'c T),
}
impl<'b, 'c, T> Deref for Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(c) => c,
        }
    }
}
/// JSON input source that reads from a std::io input stream.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct IoRead<R>
where
    R: io::Read,
{
    iter: LineColIterator<io::Bytes<R>>,
    /// Temporary storage of peeked byte.
    ch: Option<u8>,
    #[cfg(feature = "raw_value")]
    raw_buffer: Option<Vec<u8>>,
}
/// JSON input source that reads from a slice of bytes.
pub struct SliceRead<'a> {
    slice: &'a [u8],
    /// Index of the *next* byte that will be returned by next() or peek().
    index: usize,
    #[cfg(feature = "raw_value")]
    raw_buffering_start_index: usize,
}
/// JSON input source that reads from a UTF-8 string.
pub struct StrRead<'a> {
    delegate: SliceRead<'a>,
    #[cfg(feature = "raw_value")]
    data: &'a str,
}
mod private {
    pub trait Sealed {}
}
#[cfg(feature = "std")]
impl<R> IoRead<R>
where
    R: io::Read,
{
    /// Create a JSON input source to read from a std::io input stream.
    pub fn new(reader: R) -> Self {
        IoRead {
            iter: LineColIterator::new(reader.bytes()),
            ch: None,
            #[cfg(feature = "raw_value")]
            raw_buffer: None,
        }
    }
}
#[cfg(feature = "std")]
impl<R> private::Sealed for IoRead<R>
where
    R: io::Read,
{}
#[cfg(feature = "std")]
impl<R> IoRead<R>
where
    R: io::Read,
{
    fn parse_str_bytes<'s, T, F>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        result: F,
    ) -> Result<T>
    where
        T: 's,
        F: FnOnce(&'s Self, &'s [u8]) -> Result<T>,
    {
        loop {
            let ch = tri!(next_or_eof(self));
            if !ESCAPE[ch as usize] {
                scratch.push(ch);
                continue;
            }
            match ch {
                b'"' => {
                    return result(self, scratch);
                }
                b'\\' => {
                    tri!(parse_escape(self, validate, scratch));
                }
                _ => {
                    if validate {
                        return error(
                            self,
                            ErrorCode::ControlCharacterWhileParsingString,
                        );
                    }
                    scratch.push(ch);
                }
            }
        }
    }
}
#[cfg(feature = "std")]
impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        match self.ch.take() {
            Some(ch) => {
                #[cfg(feature = "raw_value")]
                {
                    if let Some(buf) = &mut self.raw_buffer {
                        buf.push(ch);
                    }
                }
                Ok(Some(ch))
            }
            None => {
                match self.iter.next() {
                    Some(Err(err)) => Err(Error::io(err)),
                    Some(Ok(ch)) => {
                        #[cfg(feature = "raw_value")]
                        {
                            if let Some(buf) = &mut self.raw_buffer {
                                buf.push(ch);
                            }
                        }
                        Ok(Some(ch))
                    }
                    None => Ok(None),
                }
            }
        }
    }
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        match self.ch {
            Some(ch) => Ok(Some(ch)),
            None => {
                match self.iter.next() {
                    Some(Err(err)) => Err(Error::io(err)),
                    Some(Ok(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    None => Ok(None),
                }
            }
        }
    }
    #[cfg(not(feature = "raw_value"))]
    #[inline]
    fn discard(&mut self) {
        self.ch = None;
    }
    #[cfg(feature = "raw_value")]
    fn discard(&mut self) {
        if let Some(ch) = self.ch.take() {
            if let Some(buf) = &mut self.raw_buffer {
                buf.push(ch);
            }
        }
    }
    fn position(&self) -> Position {
        Position {
            line: self.iter.line(),
            column: self.iter.col(),
        }
    }
    fn peek_position(&self) -> Position {
        self.position()
    }
    fn byte_offset(&self) -> usize {
        match self.ch {
            Some(_) => self.iter.byte_offset() - 1,
            None => self.iter.byte_offset(),
        }
    }
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {
        self.parse_str_bytes(scratch, true, as_str).map(Reference::Copied)
    }
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes)).map(Reference::Copied)
    }
    fn ignore_str(&mut self) -> Result<()> {
        loop {
            let ch = tri!(next_or_eof(self));
            if !ESCAPE[ch as usize] {
                continue;
            }
            match ch {
                b'"' => {
                    return Ok(());
                }
                b'\\' => {
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }
    fn decode_hex_escape(&mut self) -> Result<u16> {
        let mut n = 0;
        for _ in 0..4 {
            match decode_hex_val(tri!(next_or_eof(self))) {
                None => return error(self, ErrorCode::InvalidEscape),
                Some(val) => {
                    n = (n << 4) + val;
                }
            }
        }
        Ok(n)
    }
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.raw_buffer = Some(Vec::new());
    }
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let raw = self.raw_buffer.take().unwrap();
        let raw = match String::from_utf8(raw) {
            Ok(raw) => raw,
            Err(_) => return error(self, ErrorCode::InvalidUnicodeCodePoint),
        };
        visitor
            .visit_map(OwnedRawDeserializer {
                raw_value: Some(raw),
            })
    }
    const should_early_return_if_failed: bool = true;
    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {
        *failed = true;
    }
}
impl<'a> SliceRead<'a> {
    /// Create a JSON input source to read from a slice of bytes.
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead {
            slice,
            index: 0,
            #[cfg(feature = "raw_value")]
            raw_buffering_start_index: 0,
        }
    }
    fn position_of_index(&self, i: usize) -> Position {
        let mut position = Position { line: 1, column: 0 };
        for ch in &self.slice[..i] {
            match *ch {
                b'\n' => {
                    position.line += 1;
                    position.column = 0;
                }
                _ => {
                    position.column += 1;
                }
            }
        }
        position
    }
    /// The big optimization here over IoRead is that if the string contains no
    /// backslash escape sequences, the returned &str is a slice of the raw JSON
    /// data so we avoid copying into the scratch space.
    fn parse_str_bytes<'s, T, F>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        result: F,
    ) -> Result<Reference<'a, 's, T>>
    where
        T: ?Sized + 's,
        F: for<'f> FnOnce(&'s Self, &'f [u8]) -> Result<&'f T>,
    {
        let mut start = self.index;
        loop {
            while self.index < self.slice.len()
                && !ESCAPE[self.slice[self.index] as usize]
            {
                self.index += 1;
            }
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    if scratch.is_empty() {
                        let borrowed = &self.slice[start..self.index];
                        self.index += 1;
                        return result(self, borrowed).map(Reference::Borrowed);
                    } else {
                        scratch.extend_from_slice(&self.slice[start..self.index]);
                        self.index += 1;
                        return result(self, scratch).map(Reference::Copied);
                    }
                }
                b'\\' => {
                    scratch.extend_from_slice(&self.slice[start..self.index]);
                    self.index += 1;
                    tri!(parse_escape(self, validate, scratch));
                    start = self.index;
                }
                _ => {
                    self.index += 1;
                    if validate {
                        return error(
                            self,
                            ErrorCode::ControlCharacterWhileParsingString,
                        );
                    }
                }
            }
        }
    }
}
impl<'a> private::Sealed for SliceRead<'a> {}
impl<'a> Read<'a> for SliceRead<'a> {
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        Ok(
            if self.index < self.slice.len() {
                let ch = self.slice[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            },
        )
    }
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        Ok(
            if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            },
        )
    }
    #[inline]
    fn discard(&mut self) {
        self.index += 1;
    }
    fn position(&self) -> Position {
        self.position_of_index(self.index)
    }
    fn peek_position(&self) -> Position {
        self.position_of_index(cmp::min(self.slice.len(), self.index + 1))
    }
    fn byte_offset(&self) -> usize {
        self.index
    }
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, str>> {
        self.parse_str_bytes(scratch, true, as_str)
    }
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
    }
    fn ignore_str(&mut self) -> Result<()> {
        loop {
            while self.index < self.slice.len()
                && !ESCAPE[self.slice[self.index] as usize]
            {
                self.index += 1;
            }
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    self.index += 1;
                    return Ok(());
                }
                b'\\' => {
                    self.index += 1;
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }
    fn decode_hex_escape(&mut self) -> Result<u16> {
        if self.index + 4 > self.slice.len() {
            self.index = self.slice.len();
            return error(self, ErrorCode::EofWhileParsingString);
        }
        let mut n = 0;
        for _ in 0..4 {
            let ch = decode_hex_val(self.slice[self.index]);
            self.index += 1;
            match ch {
                None => return error(self, ErrorCode::InvalidEscape),
                Some(val) => {
                    n = (n << 4) + val;
                }
            }
        }
        Ok(n)
    }
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.raw_buffering_start_index = self.index;
    }
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {
        let raw = &self.slice[self.raw_buffering_start_index..self.index];
        let raw = match str::from_utf8(raw) {
            Ok(raw) => raw,
            Err(_) => return error(self, ErrorCode::InvalidUnicodeCodePoint),
        };
        visitor
            .visit_map(BorrowedRawDeserializer {
                raw_value: Some(raw),
            })
    }
    const should_early_return_if_failed: bool = false;
    #[inline]
    #[cold]
    fn set_failed(&mut self, _failed: &mut bool) {
        self.slice = &self.slice[..self.index];
    }
}
impl<'a> StrRead<'a> {
    /// Create a JSON input source to read from a UTF-8 string.
    pub fn new(s: &'a str) -> Self {
        StrRead {
            delegate: SliceRead::new(s.as_bytes()),
            #[cfg(feature = "raw_value")]
            data: s,
        }
    }
}
impl<'a> private::Sealed for StrRead<'a> {}
impl<'a> Read<'a> for StrRead<'a> {
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        self.delegate.next()
    }
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        self.delegate.peek()
    }
    #[inline]
    fn discard(&mut self) {
        self.delegate.discard();
    }
    fn position(&self) -> Position {
        self.delegate.position()
    }
    fn peek_position(&self) -> Position {
        self.delegate.peek_position()
    }
    fn byte_offset(&self) -> usize {
        self.delegate.byte_offset()
    }
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, str>> {
        self.delegate
            .parse_str_bytes(
                scratch,
                true,
                |_, bytes| { Ok(unsafe { str::from_utf8_unchecked(bytes) }) },
            )
    }
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {
        self.delegate.parse_str_raw(scratch)
    }
    fn ignore_str(&mut self) -> Result<()> {
        self.delegate.ignore_str()
    }
    fn decode_hex_escape(&mut self) -> Result<u16> {
        self.delegate.decode_hex_escape()
    }
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.delegate.begin_raw_buffering();
    }
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {
        let raw = &self
            .data[self.delegate.raw_buffering_start_index..self.delegate.index];
        visitor
            .visit_map(BorrowedRawDeserializer {
                raw_value: Some(raw),
            })
    }
    const should_early_return_if_failed: bool = false;
    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {
        self.delegate.set_failed(failed);
    }
}
impl<'a, 'de, R> private::Sealed for &'a mut R
where
    R: Read<'de>,
{}
impl<'a, 'de, R> Read<'de> for &'a mut R
where
    R: Read<'de>,
{
    fn next(&mut self) -> Result<Option<u8>> {
        R::next(self)
    }
    fn peek(&mut self) -> Result<Option<u8>> {
        R::peek(self)
    }
    fn discard(&mut self) {
        R::discard(self);
    }
    fn position(&self) -> Position {
        R::position(self)
    }
    fn peek_position(&self) -> Position {
        R::peek_position(self)
    }
    fn byte_offset(&self) -> usize {
        R::byte_offset(self)
    }
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {
        R::parse_str(self, scratch)
    }
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        R::parse_str_raw(self, scratch)
    }
    fn ignore_str(&mut self) -> Result<()> {
        R::ignore_str(self)
    }
    fn decode_hex_escape(&mut self) -> Result<u16> {
        R::decode_hex_escape(self)
    }
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        R::begin_raw_buffering(self);
    }
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        R::end_raw_buffering(self, visitor)
    }
    const should_early_return_if_failed: bool = R::should_early_return_if_failed;
    fn set_failed(&mut self, failed: &mut bool) {
        R::set_failed(self, failed);
    }
}
/// Marker for whether StreamDeserializer can implement FusedIterator.
pub trait Fused: private::Sealed {}
impl<'a> Fused for SliceRead<'a> {}
impl<'a> Fused for StrRead<'a> {}
static ESCAPE: [bool; 256] = {
    const CT: bool = true;
    const QU: bool = true;
    const BS: bool = true;
    const __: bool = false;
    [
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        CT,
        __,
        __,
        QU,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        BS,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
    ]
};
fn next_or_eof<'de, R>(read: &mut R) -> Result<u8>
where
    R: ?Sized + Read<'de>,
{
    match tri!(read.next()) {
        Some(b) => Ok(b),
        None => error(read, ErrorCode::EofWhileParsingString),
    }
}
fn peek_or_eof<'de, R>(read: &mut R) -> Result<u8>
where
    R: ?Sized + Read<'de>,
{
    match tri!(read.peek()) {
        Some(b) => Ok(b),
        None => error(read, ErrorCode::EofWhileParsingString),
    }
}
fn error<'de, R, T>(read: &R, reason: ErrorCode) -> Result<T>
where
    R: ?Sized + Read<'de>,
{
    let position = read.position();
    Err(Error::syntax(reason, position.line, position.column))
}
fn as_str<'de, 's, R: Read<'de>>(read: &R, slice: &'s [u8]) -> Result<&'s str> {
    str::from_utf8(slice).or_else(|_| error(read, ErrorCode::InvalidUnicodeCodePoint))
}
/// Parses a JSON escape sequence and appends it into the scratch space. Assumes
/// the previous byte read was a backslash.
fn parse_escape<'de, R: Read<'de>>(
    read: &mut R,
    validate: bool,
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let ch = tri!(next_or_eof(read));
    match ch {
        b'"' => scratch.push(b'"'),
        b'\\' => scratch.push(b'\\'),
        b'/' => scratch.push(b'/'),
        b'b' => scratch.push(b'\x08'),
        b'f' => scratch.push(b'\x0c'),
        b'n' => scratch.push(b'\n'),
        b'r' => scratch.push(b'\r'),
        b't' => scratch.push(b'\t'),
        b'u' => {
            fn encode_surrogate(scratch: &mut Vec<u8>, n: u16) {
                scratch
                    .extend_from_slice(
                        &[
                            (n >> 12 & 0b0000_1111) as u8 | 0b1110_0000,
                            (n >> 6 & 0b0011_1111) as u8 | 0b1000_0000,
                            (n & 0b0011_1111) as u8 | 0b1000_0000,
                        ],
                    );
            }
            let c = match tri!(read.decode_hex_escape()) {
                n @ 0xDC00..=0xDFFF => {
                    return if validate {
                        error(read, ErrorCode::LoneLeadingSurrogateInHexEscape)
                    } else {
                        encode_surrogate(scratch, n);
                        Ok(())
                    };
                }
                n1 @ 0xD800..=0xDBFF => {
                    if tri!(peek_or_eof(read)) == b'\\' {
                        read.discard();
                    } else {
                        return if validate {
                            read.discard();
                            error(read, ErrorCode::UnexpectedEndOfHexEscape)
                        } else {
                            encode_surrogate(scratch, n1);
                            Ok(())
                        };
                    }
                    if tri!(peek_or_eof(read)) == b'u' {
                        read.discard();
                    } else {
                        return if validate {
                            read.discard();
                            error(read, ErrorCode::UnexpectedEndOfHexEscape)
                        } else {
                            encode_surrogate(scratch, n1);
                            parse_escape(read, validate, scratch)
                        };
                    }
                    let n2 = tri!(read.decode_hex_escape());
                    if n2 < 0xDC00 || n2 > 0xDFFF {
                        return error(read, ErrorCode::LoneLeadingSurrogateInHexEscape);
                    }
                    let n = (((n1 - 0xD800) as u32) << 10 | (n2 - 0xDC00) as u32)
                        + 0x1_0000;
                    match char::from_u32(n) {
                        Some(c) => c,
                        None => {
                            return error(read, ErrorCode::InvalidUnicodeCodePoint);
                        }
                    }
                }
                n => char::from_u32(n as u32).unwrap(),
            };
            scratch.extend_from_slice(c.encode_utf8(&mut [0_u8; 4]).as_bytes());
        }
        _ => {
            return error(read, ErrorCode::InvalidEscape);
        }
    }
    Ok(())
}
/// Parses a JSON escape sequence and discards the value. Assumes the previous
/// byte read was a backslash.
fn ignore_escape<'de, R>(read: &mut R) -> Result<()>
where
    R: ?Sized + Read<'de>,
{
    let ch = tri!(next_or_eof(read));
    match ch {
        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {}
        b'u' => {
            tri!(read.decode_hex_escape());
        }
        _ => {
            return error(read, ErrorCode::InvalidEscape);
        }
    }
    Ok(())
}
static HEX: [u8; 256] = {
    const __: u8 = 255;
    [
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        00,
        01,
        02,
        03,
        04,
        05,
        06,
        07,
        08,
        09,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        10,
        11,
        12,
        13,
        14,
        15,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        10,
        11,
        12,
        13,
        14,
        15,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
        __,
    ]
};
fn decode_hex_val(val: u8) -> Option<u16> {
    let n = HEX[val as usize] as u16;
    if n == 255 { None } else { Some(n) }
}
#[cfg(test)]
mod tests_llm_16_8_llm_16_8 {
    use super::*;
    use crate::*;
    use crate::de::Read;
    use crate::error::{Error, ErrorCode};
    use crate::read::StrRead;
    use std::marker::PhantomData;
    #[test]
    fn test_ignore_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut reader = StrRead::new(json_str);
        let result = reader.ignore_str();
        debug_assert!(result.is_ok());
        debug_assert_eq!(reader.peek().unwrap(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_202 {
    use super::*;
    use crate::*;
    use crate::error::Error;
    use crate::read::{IoRead, Read};
    use std::io;
    #[test]
    fn test_discard() {
        let _rug_st_tests_llm_16_202_rrrruuuugggg_test_discard = 0;
        let rug_fuzz_0 = b"some data";
        let data = rug_fuzz_0;
        let reader = io::Cursor::new(data);
        let mut io_read = IoRead::new(reader);
        debug_assert_eq!(io_read.peek().unwrap(), Some(b's'));
        debug_assert!(io_read.ch.is_some());
        io_read.discard();
        debug_assert!(io_read.ch.is_none());
        let mut rest_of_data = Vec::new();
        while let Some(byte) = io_read.next().unwrap() {
            rest_of_data.push(byte);
        }
        debug_assert_eq!(rest_of_data, b"ome data");
        let _rug_ed_tests_llm_16_202_rrrruuuugggg_test_discard = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_209 {
    use super::*;
    use crate::*;
    use std::io;
    #[test]
    fn position_at_start() {
        let _rug_st_tests_llm_16_209_rrrruuuugggg_position_at_start = 0;
        let rug_fuzz_0 = b"";
        let data = rug_fuzz_0;
        let reader = io::Cursor::new(data);
        let io_read = IoRead::new(reader);
        let position = io_read.position();
        debug_assert_eq!(position.line, 1);
        debug_assert_eq!(position.column, 0);
        let _rug_ed_tests_llm_16_209_rrrruuuugggg_position_at_start = 0;
    }
    #[test]
    fn position_after_one_line() {
        let _rug_st_tests_llm_16_209_rrrruuuugggg_position_after_one_line = 0;
        let rug_fuzz_0 = b"\n";
        let data = rug_fuzz_0;
        let reader = io::Cursor::new(data);
        let mut io_read = IoRead::new(reader);
        debug_assert!(io_read.next().is_ok());
        let position = io_read.position();
        debug_assert_eq!(position.line, 2);
        debug_assert_eq!(position.column, 0);
        let _rug_ed_tests_llm_16_209_rrrruuuugggg_position_after_one_line = 0;
    }
    #[test]
    fn position_after_some_content() {
        let _rug_st_tests_llm_16_209_rrrruuuugggg_position_after_some_content = 0;
        let rug_fuzz_0 = b"abc\ndef";
        let data = rug_fuzz_0;
        let reader = io::Cursor::new(data);
        let mut io_read = IoRead::new(reader);
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        let position = io_read.position();
        debug_assert_eq!(position.line, 1);
        debug_assert_eq!(position.column, 3);
        let _rug_ed_tests_llm_16_209_rrrruuuugggg_position_after_some_content = 0;
    }
    #[test]
    fn position_across_multiple_lines() {
        let _rug_st_tests_llm_16_209_rrrruuuugggg_position_across_multiple_lines = 0;
        let rug_fuzz_0 = b"abc\ndef\nghi";
        let data = rug_fuzz_0;
        let reader = io::Cursor::new(data);
        let mut io_read = IoRead::new(reader);
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        debug_assert!(io_read.next().is_ok());
        let position = io_read.position();
        debug_assert_eq!(position.line, 3);
        debug_assert_eq!(position.column, 0);
        let _rug_ed_tests_llm_16_209_rrrruuuugggg_position_across_multiple_lines = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_210 {
    use super::*;
    use crate::*;
    use std::io;
    #[test]
    fn test_set_failed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        struct TestReader {
            data: Vec<u8>,
        }
        impl io::Read for TestReader {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                let len = std::cmp::min(buf.len(), self.data.len());
                buf[..len].copy_from_slice(&self.data[..len]);
                self.data.drain(..len);
                Ok(len)
            }
        }
        let reader = TestReader { data: Vec::new() };
        let mut io_read = IoRead::new(reader);
        let mut failed = rug_fuzz_0;
        io_read.set_failed(&mut failed);
        debug_assert!(failed, "Failed should be true after set_failed call.");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_211 {
    use super::*;
    use crate::*;
    use std::ops::Deref;
    struct TestStruct {
        value: i32,
    }
    #[test]
    fn test_deref_borrowed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_value = TestStruct { value: rug_fuzz_0 };
        let reference = Reference::Borrowed(&test_value);
        let deref_value: &TestStruct = reference.deref();
        debug_assert_eq!(rug_fuzz_1, deref_value.value);
             }
}
}
}    }
    #[test]
    fn test_deref_copied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_value = TestStruct { value: rug_fuzz_0 };
        let reference = Reference::Copied(&test_value);
        let deref_value: &TestStruct = reference.deref();
        debug_assert_eq!(rug_fuzz_1, deref_value.value);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_212 {
    use super::*;
    use crate::*;
    #[test]
    fn test_byte_offset_at_start() {
        let _rug_st_tests_llm_16_212_rrrruuuugggg_test_byte_offset_at_start = 0;
        let rug_fuzz_0 = b"test data";
        let data = rug_fuzz_0;
        let reader = SliceRead::new(&data[..]);
        debug_assert_eq!(reader.byte_offset(), 0);
        let _rug_ed_tests_llm_16_212_rrrruuuugggg_test_byte_offset_at_start = 0;
    }
    #[test]
    fn test_byte_offset_after_read() {
        let _rug_st_tests_llm_16_212_rrrruuuugggg_test_byte_offset_after_read = 0;
        let rug_fuzz_0 = b"test data";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(&data[..]);
        while let Ok(Some(_)) = reader.next() {}
        debug_assert_eq!(reader.byte_offset(), data.len());
        let _rug_ed_tests_llm_16_212_rrrruuuugggg_test_byte_offset_after_read = 0;
    }
    #[test]
    fn test_byte_offset_after_partial_read() {
        let _rug_st_tests_llm_16_212_rrrruuuugggg_test_byte_offset_after_partial_read = 0;
        let rug_fuzz_0 = b"test data";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(&data[..]);
        reader.next().unwrap();
        reader.next().unwrap();
        debug_assert_eq!(reader.byte_offset(), 2);
        let _rug_ed_tests_llm_16_212_rrrruuuugggg_test_byte_offset_after_partial_read = 0;
    }
    #[test]
    fn test_byte_offset_after_discard() {
        let _rug_st_tests_llm_16_212_rrrruuuugggg_test_byte_offset_after_discard = 0;
        let rug_fuzz_0 = b"test data";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(&data[..]);
        reader.next().unwrap();
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 2);
        let _rug_ed_tests_llm_16_212_rrrruuuugggg_test_byte_offset_after_discard = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_214 {
    use super::*;
    use crate::*;
    #[test]
    fn discard_advances_index() {
        let _rug_st_tests_llm_16_214_rrrruuuugggg_discard_advances_index = 0;
        let rug_fuzz_0 = b"abc";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        debug_assert_eq!(reader.byte_offset(), 0);
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 1);
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 2);
        let _rug_ed_tests_llm_16_214_rrrruuuugggg_discard_advances_index = 0;
    }
    #[test]
    fn discard_at_end_of_input() {
        let _rug_st_tests_llm_16_214_rrrruuuugggg_discard_at_end_of_input = 0;
        let rug_fuzz_0 = b"";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        debug_assert_eq!(reader.byte_offset(), 0);
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 1);
        let _rug_ed_tests_llm_16_214_rrrruuuugggg_discard_at_end_of_input = 0;
    }
    #[test]
    fn discard_multiple_times() {
        let _rug_st_tests_llm_16_214_rrrruuuugggg_discard_multiple_times = 0;
        let rug_fuzz_0 = b"abcd";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        reader.discard();
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 2);
        reader.discard();
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 4);
        reader.discard();
        debug_assert_eq!(reader.byte_offset(), 5);
        let _rug_ed_tests_llm_16_214_rrrruuuugggg_discard_multiple_times = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_216 {
    use super::*;
    use crate::*;
    #[test]
    fn test_next_empty_slice() {
        let _rug_st_tests_llm_16_216_rrrruuuugggg_test_next_empty_slice = 0;
        let slice = &[];
        let mut slice_read = SliceRead::new(slice);
        debug_assert!(slice_read.next().unwrap().is_none());
        let _rug_ed_tests_llm_16_216_rrrruuuugggg_test_next_empty_slice = 0;
    }
    #[test]
    fn test_next_single_element_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let slice = &[rug_fuzz_0];
        let mut slice_read = SliceRead::new(slice);
        debug_assert_eq!(slice_read.next().unwrap(), Some(42));
        debug_assert!(slice_read.next().unwrap().is_none());
             }
}
}
}    }
    #[test]
    fn test_next_multiple_elements_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let slice = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut slice_read = SliceRead::new(slice);
        debug_assert_eq!(slice_read.next().unwrap(), Some(1));
        debug_assert_eq!(slice_read.next().unwrap(), Some(2));
        debug_assert_eq!(slice_read.next().unwrap(), Some(3));
        debug_assert_eq!(slice_read.next().unwrap(), Some(4));
        debug_assert_eq!(slice_read.next().unwrap(), Some(5));
        debug_assert!(slice_read.next().unwrap().is_none());
             }
}
}
}    }
    #[test]
    fn test_next_after_end() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let slice = &[rug_fuzz_0, rug_fuzz_1];
        let mut slice_read = SliceRead::new(slice);
        debug_assert_eq!(slice_read.next().unwrap(), Some(15));
        debug_assert_eq!(slice_read.next().unwrap(), Some(16));
        debug_assert!(slice_read.next().unwrap().is_none());
        debug_assert!(slice_read.next().unwrap().is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_218_llm_16_218 {
    use super::*;
    use crate::*;
    use crate::error::Result;
    use crate::de::Read;
    #[test]
    fn parse_str_raw_empty_string() -> Result<()> {
        let mut read = SliceRead::new(b"\"\"");
        let mut scratch = Vec::new();
        let reference = read.parse_str_raw(&mut scratch)?;
        assert_eq!(&* reference, b"");
        Ok(())
    }
    #[test]
    fn parse_str_raw_simple_string() -> Result<()> {
        let mut read = SliceRead::new(b"\"hello\"");
        let mut scratch = Vec::new();
        let reference = read.parse_str_raw(&mut scratch)?;
        assert_eq!(&* reference, b"hello");
        Ok(())
    }
    #[test]
    fn parse_str_raw_with_escaped_quotes() -> Result<()> {
        let mut read = SliceRead::new(b"\"he\\\"llo\"");
        let mut scratch = Vec::new();
        let reference = read.parse_str_raw(&mut scratch)?;
        assert_eq!(&* reference, b"he\"llo");
        Ok(())
    }
    #[test]
    fn parse_str_raw_with_unicode_escaped_characters() -> Result<()> {
        let mut read = SliceRead::new(b"\"hello\\u0020world\"");
        let mut scratch = Vec::new();
        let reference = read.parse_str_raw(&mut scratch)?;
        assert_eq!(&* reference, b"hello world");
        Ok(())
    }
    #[test]
    fn parse_str_raw_with_invalid_string() {
        let mut read = SliceRead::new(b"\"invalid");
        let mut scratch = Vec::new();
        assert!(read.parse_str_raw(& mut scratch).is_err());
    }
    #[test]
    fn parse_str_raw_with_escaped_control_characters() -> Result<()> {
        let mut read = SliceRead::new(b"\"hello\\nworld\"");
        let mut scratch = Vec::new();
        let reference = read.parse_str_raw(&mut scratch)?;
        assert_eq!(&* reference, b"hello\nworld");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_219 {
    use super::*;
    use crate::*;
    #[test]
    fn test_peek_at_beginning() {
        let _rug_st_tests_llm_16_219_rrrruuuugggg_test_peek_at_beginning = 0;
        let rug_fuzz_0 = b"hello";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        debug_assert_eq!(reader.peek().unwrap(), Some(b'h'));
        let _rug_ed_tests_llm_16_219_rrrruuuugggg_test_peek_at_beginning = 0;
    }
    #[test]
    fn test_peek_at_middle() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 5], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        reader.index = rug_fuzz_1;
        debug_assert_eq!(reader.peek().unwrap(), Some(b'l'));
             }
}
}
}    }
    #[test]
    fn test_peek_at_end() {
        let _rug_st_tests_llm_16_219_rrrruuuugggg_test_peek_at_end = 0;
        let rug_fuzz_0 = b"hello";
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        reader.index = data.len();
        debug_assert_eq!(reader.peek().unwrap(), None);
        let _rug_ed_tests_llm_16_219_rrrruuuugggg_test_peek_at_end = 0;
    }
    #[test]
    fn test_peek_past_end() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 5], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let mut reader = SliceRead::new(data);
        reader.index = data.len() + rug_fuzz_1;
        debug_assert_eq!(reader.peek().unwrap(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_221_llm_16_221 {
    use crate::read::{Position, Read, SliceRead};
    #[derive(PartialEq, Debug)]
    struct SimplePosition {
        line: usize,
        column: usize,
    }
    impl From<Position> for SimplePosition {
        fn from(p: Position) -> Self {
            SimplePosition {
                line: p.line,
                column: p.column,
            }
        }
    }
    #[test]
    fn slice_read_position_at_start() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0.as_bytes();
        let slice_read = SliceRead::new(data);
        debug_assert_eq!(
            SimplePosition::from(slice_read.position()), SimplePosition { line : 1,
            column : 0 }
        );
             }
}
}
}    }
    #[test]
    fn slice_read_position_at_line_start() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0.as_bytes();
        let mut slice_read = SliceRead::new(data);
        slice_read.index = rug_fuzz_1;
        debug_assert_eq!(
            SimplePosition::from(slice_read.position()), SimplePosition { line : 2,
            column : 0 }
        );
             }
}
}
}    }
    #[test]
    fn slice_read_position_inside_line() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0.as_bytes();
        let mut slice_read = SliceRead::new(data);
        slice_read.index = rug_fuzz_1;
        debug_assert_eq!(
            SimplePosition::from(slice_read.position()), SimplePosition { line : 2,
            column : 2 }
        );
             }
}
}
}    }
    #[test]
    fn slice_read_position_at_end() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0.as_bytes();
        let mut slice_read = SliceRead::new(data);
        slice_read.index = data.len();
        debug_assert_eq!(
            SimplePosition::from(slice_read.position()), SimplePosition { line : 3,
            column : 5 }
        );
             }
}
}
}    }
    #[test]
    fn slice_read_position_past_end() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0.as_bytes();
        let mut slice_read = SliceRead::new(data);
        slice_read.index = data.len() + rug_fuzz_1;
        debug_assert_eq!(
            SimplePosition::from(slice_read.position()), SimplePosition { line : 3,
            column : 5 }
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_222 {
    use super::*;
    use crate::*;
    #[test]
    fn set_failed_empty_slice() {
        let mut slice_read = SliceRead::new(b"");
        let mut failed = false;
        slice_read.set_failed(&mut failed);
        assert_eq!(slice_read.slice, b"");
    }
    #[test]
    fn set_failed_non_empty_slice() {
        let mut slice_read = SliceRead::new(b"test");
        slice_read.index = 2;
        let mut failed = false;
        slice_read.set_failed(&mut failed);
        assert_eq!(slice_read.slice, b"te");
    }
    #[test]
    fn set_failed_slice_already_failed() {
        let mut slice_read = SliceRead::new(b"test");
        slice_read.index = 2;
        slice_read.set_failed(&mut false);
        let original_failed_slice = slice_read.slice;
        let mut failed = true;
        slice_read.set_failed(&mut failed);
        assert_eq!(slice_read.slice, original_failed_slice);
    }
}
#[cfg(test)]
mod tests_llm_16_223 {
    use crate::read::{Read, SliceRead, StrRead};
    #[test]
    fn test_byte_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut str_read = StrRead::new(json_str);
        let _ = str_read.next();
        let offset = str_read.byte_offset();
        debug_assert_eq!(offset, 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_225 {
    use super::*;
    use crate::*;
    use crate::error::Result;
    #[test]
    fn slice_read_discard_empty_slice() {
        let _rug_st_tests_llm_16_225_rrrruuuugggg_slice_read_discard_empty_slice = 0;
        let rug_fuzz_0 = b"";
        let data = rug_fuzz_0;
        let mut slice_read = SliceRead::new(data);
        debug_assert_eq!(slice_read.byte_offset(), 0);
        slice_read.discard();
        debug_assert_eq!(slice_read.byte_offset(), 1);
        let _rug_ed_tests_llm_16_225_rrrruuuugggg_slice_read_discard_empty_slice = 0;
    }
    #[test]
    fn slice_read_discard_non_empty_slice() {
        let _rug_st_tests_llm_16_225_rrrruuuugggg_slice_read_discard_non_empty_slice = 0;
        let rug_fuzz_0 = b"abc";
        let data = rug_fuzz_0;
        let mut slice_read = SliceRead::new(data);
        debug_assert_eq!(slice_read.byte_offset(), 0);
        slice_read.discard();
        debug_assert_eq!(slice_read.byte_offset(), 1);
        slice_read.discard();
        debug_assert_eq!(slice_read.byte_offset(), 2);
        slice_read.discard();
        debug_assert_eq!(slice_read.byte_offset(), 3);
        slice_read.discard();
        debug_assert_eq!(slice_read.byte_offset(), 4);
        let _rug_ed_tests_llm_16_225_rrrruuuugggg_slice_read_discard_non_empty_slice = 0;
    }
    #[test]
    fn str_read_discard_empty_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0;
        let mut str_read = StrRead::new(data);
        debug_assert_eq!(str_read.byte_offset(), 0);
        str_read.discard();
        debug_assert_eq!(str_read.byte_offset(), 1);
             }
}
}
}    }
    #[test]
    fn str_read_discard_non_empty_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0;
        let mut str_read = StrRead::new(data);
        debug_assert_eq!(str_read.byte_offset(), 0);
        str_read.discard();
        debug_assert_eq!(str_read.byte_offset(), 1);
        str_read.discard();
        debug_assert_eq!(str_read.byte_offset(), 2);
        str_read.discard();
        debug_assert_eq!(str_read.byte_offset(), 3);
        str_read.discard();
        debug_assert_eq!(str_read.byte_offset(), 4);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_226 {
    use crate::read::{Position, Read, SliceRead, StrRead, Reference, Result};
    use std::str;
    fn ignore_str(sr: &mut SliceRead) -> Result<()> {
        sr.ignore_str()
    }
    #[test]
    fn test_ignore_str_empty_string() {
        let data = "\"\"";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_ok());
    }
    #[test]
    fn test_ignore_str_normal_string() {
        let data = "\"hello\"";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_ok());
    }
    #[test]
    fn test_ignore_str_string_with_escaped_quote() {
        let data = "\"hello \\\"world\\\"\"";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_ok());
    }
    #[test]
    fn test_ignore_str_string_with_escaped_backslash() {
        let data = "\"hello \\\\ world\"";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_ok());
    }
    #[test]
    fn test_ignore_str_incomplete_string() {
        let data = "\"hello";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_err());
    }
    #[test]
    fn test_ignore_str_incomplete_escaped_character() {
        let data = "\"hello \\";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_err());
    }
    #[test]
    fn test_ignore_str_invalid_escaped_character() {
        let data = "\"hello \\w\"";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_err());
    }
    #[test]
    fn test_ignore_str_newline_in_string() {
        let data = "\"hello\nworld\"";
        let mut sr = SliceRead::new(data.as_bytes());
        assert!(ignore_str(& mut sr).is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_227_llm_16_227 {
    use super::*;
    use crate::*;
    use crate::error::{Error, Result};
    use crate::read::StrRead;
    #[test]
    fn str_read_next_with_empty_string() -> Result<()> {
        let mut str_read = StrRead::new("");
        assert_eq!(str_read.next() ?, None);
        Ok(())
    }
    #[test]
    fn str_read_next_with_non_empty_string() -> Result<()> {
        let mut str_read = StrRead::new("abc");
        assert_eq!(str_read.next() ?, Some(b'a'));
        assert_eq!(str_read.next() ?, Some(b'b'));
        assert_eq!(str_read.next() ?, Some(b'c'));
        assert_eq!(str_read.next() ?, None);
        Ok(())
    }
    #[test]
    fn str_read_next_with_unicode_characters() -> Result<()> {
        let mut str_read = StrRead::new("ñ");
        assert_eq!(str_read.next() ?, Some(0xc3));
        assert_eq!(str_read.next() ?, Some(0xb1));
        assert_eq!(str_read.next() ?, None);
        Ok(())
    }
    #[test]
    fn str_read_next_at_end_of_string() -> Result<()> {
        let mut str_read = StrRead::new("x");
        assert_eq!(str_read.next() ?, Some(b'x'));
        assert_eq!(str_read.next() ?, None);
        assert_eq!(str_read.next() ?, None);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_230 {
    use super::*;
    use crate::*;
    use crate::error::Result;
    #[test]
    fn peek_empty_str_read() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut reader = StrRead::new(rug_fuzz_0);
        debug_assert_eq!(reader.peek().unwrap(), None);
             }
}
}
}    }
    #[test]
    fn peek_non_empty_str_read() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut reader = StrRead::new(rug_fuzz_0);
        debug_assert_eq!(reader.peek().unwrap(), Some(b't'));
        debug_assert_eq!(reader.peek().unwrap(), Some(b't'));
             }
}
}
}    }
    #[test]
    fn peek_after_read() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut reader = StrRead::new(rug_fuzz_0);
        debug_assert_eq!(reader.next().unwrap(), Some(b't'));
        debug_assert_eq!(reader.peek().unwrap(), Some(b'e'));
             }
}
}
}    }
    #[test]
    fn peek_at_end_of_str_read() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut reader = StrRead::new(rug_fuzz_0);
        debug_assert_eq!(reader.next().unwrap(), Some(b't'));
        debug_assert_eq!(reader.peek().unwrap(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_231 {
    use crate::read::{Position, Read, SliceRead, StrRead};
    #[test]
    fn test_peek_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut reader = StrRead::new(json_str);
        let initial_position = reader.peek_position();
        debug_assert_eq!(initial_position.line, 1);
        debug_assert_eq!(initial_position.column, 0);
        reader.next().unwrap();
        let position_after_h = reader.peek_position();
        debug_assert_eq!(position_after_h.line, 1);
        debug_assert_eq!(position_after_h.column, 2);
        for _ in rug_fuzz_1..rug_fuzz_2 {
            reader.next().unwrap();
        }
        let position_after_ello_newline = reader.peek_position();
        debug_assert_eq!(position_after_ello_newline.line, 2);
        debug_assert_eq!(position_after_ello_newline.column, 1);
        reader.next().unwrap();
        let position_after_w = reader.peek_position();
        debug_assert_eq!(position_after_w.line, 2);
        debug_assert_eq!(position_after_w.column, 2);
        for _ in rug_fuzz_3..rug_fuzz_4 {
            reader.next().unwrap();
        }
        let position_after_orld_newline = reader.peek_position();
        debug_assert_eq!(position_after_orld_newline.line, 3);
        debug_assert_eq!(position_after_orld_newline.column, 1);
        reader.next().unwrap();
        let position_after_exclamation = reader.peek_position();
        debug_assert_eq!(position_after_exclamation.line, 3);
        debug_assert_eq!(position_after_exclamation.column, 2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_233 {
    use super::*;
    use crate::*;
    use crate::read::Read;
    #[test]
    fn set_failed_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut str_read = StrRead::new(rug_fuzz_0);
        let mut failed = rug_fuzz_1;
        debug_assert_eq!(failed, false);
        debug_assert_eq!(str_read.byte_offset(), 0);
        str_read.set_failed(&mut failed);
        debug_assert_eq!(str_read.byte_offset(), 0);
        debug_assert_eq!(failed, true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_547 {
    use super::*;
    use crate::*;
    #[test]
    fn position_of_index_empty_slice() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 0], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let reader = SliceRead::new(data);
        let position = reader.position_of_index(rug_fuzz_1);
        debug_assert_eq!(position.line, 1);
        debug_assert_eq!(position.column, 0);
             }
}
}
}    }
    #[test]
    fn position_of_index_newline() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 17], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let reader = SliceRead::new(data);
        let position = reader.position_of_index(rug_fuzz_1);
        debug_assert_eq!(position.line, 2);
        debug_assert_eq!(position.column, 0);
             }
}
}
}    }
    #[test]
    fn position_of_index_in_the_middle_of_line() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 17], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let reader = SliceRead::new(data);
        let position = reader.position_of_index(rug_fuzz_1);
        debug_assert_eq!(position.line, 2);
        debug_assert_eq!(position.column, 4);
             }
}
}
}    }
    #[test]
    fn position_of_index_at_end() {
        let _rug_st_tests_llm_16_547_rrrruuuugggg_position_of_index_at_end = 0;
        let rug_fuzz_0 = b"line1\nline2\nline3";
        let data = rug_fuzz_0;
        let reader = SliceRead::new(data);
        let data_length = data.len();
        let position = reader.position_of_index(data_length);
        debug_assert_eq!(position.line, 3);
        debug_assert_eq!(position.column, 5);
        let _rug_ed_tests_llm_16_547_rrrruuuugggg_position_of_index_at_end = 0;
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn position_of_index_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 17], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let reader = SliceRead::new(data);
        let _position = reader.position_of_index(data.len() + rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_548_llm_16_548 {
    use crate::read::StrRead;
    use crate::read::SliceRead;
    use crate::read::{Read, Fused, private::Sealed, Result};
    #[test]
    fn str_read_new_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let reader = StrRead::new(input);
        debug_assert!(
            matches!(reader.delegate, SliceRead { slice, .. } if slice.is_empty())
        );
             }
}
}
}    }
    #[test]
    fn str_read_new_nonempty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let reader = StrRead::new(input);
        debug_assert_eq!(reader.delegate.byte_offset(), 0);
        debug_assert_eq!(reader.delegate.slice, input.as_bytes());
             }
}
}
}    }
    #[test]
    fn str_read_new_peek_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        debug_assert_eq!(reader.peek().unwrap(), None);
             }
}
}
}    }
    #[test]
    fn str_read_new_peek_nonempty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        debug_assert_eq!(reader.peek().unwrap(), Some(b't'));
             }
}
}
}    }
    #[test]
    fn str_read_new_next_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        debug_assert_eq!(reader.next().unwrap(), None);
             }
}
}
}    }
    #[test]
    fn str_read_new_next_nonempty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        debug_assert_eq!(reader.next().unwrap(), Some(b't'));
        debug_assert_eq!(reader.next().unwrap(), Some(b'e'));
             }
}
}
}    }
    #[test]
    fn str_read_new_discard() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        debug_assert_eq!(reader.next().unwrap(), Some(b't'));
        reader.discard();
        debug_assert_eq!(reader.next().unwrap(), Some(b's'));
             }
}
}
}    }
    #[test]
    fn str_read_new_positions() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        debug_assert_eq!(reader.position().line, 1);
        debug_assert_eq!(reader.position().column, 3);
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        debug_assert_eq!(reader.position().line, 2);
        debug_assert_eq!(reader.position().column, 1);
        debug_assert_eq!(reader.peek_position().line, 2);
        debug_assert_eq!(reader.peek_position().column, 2);
             }
}
}
}    }
    #[cfg(feature = "raw_value")]
    #[test]
    fn str_read_raw_value() {
        let _rug_st_tests_llm_16_548_llm_16_548_rrrruuuugggg_str_read_raw_value = 0;
        let rug_fuzz_0 = r#"{"raw":"value"}"#;
        let input = rug_fuzz_0;
        let mut reader = StrRead::new(input);
        reader.begin_raw_buffering();
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        reader.next().unwrap();
        let raw_value = reader
            .end_raw_buffering(|visitor| {
                debug_assert!(matches!(visitor.raw_value, Some("raw")));
                Ok(())
            });
        debug_assert!(raw_value.is_ok());
        let _rug_ed_tests_llm_16_548_llm_16_548_rrrruuuugggg_str_read_raw_value = 0;
    }
}
#[cfg(test)]
mod tests_rug_309 {
    use crate::de::{self, Read, StrRead};
    use crate::error::{self, ErrorCode};
    #[test]
    fn test_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        let mut p1 = ErrorCode::Message(rug_fuzz_1.into());
        let result: error::Result<()> = crate::read::error(&p0, p1);
        debug_assert!(result.is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_310 {
    use crate::de::{self, StrRead};
    use crate::error::ErrorCode;
    use crate::read::{self, as_str};
    use std::io::Read;
    use crate::error::Result as JsonResult;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_310_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "{\"some_key\": \"some_value\"}";
        let rug_fuzz_1 = b"valid UTF-8";
        let json_str = rug_fuzz_0;
        let mut p0 = de::StrRead::new(json_str);
        let p1: &[u8] = rug_fuzz_1;
        match as_str(&p0, p1) {
            Ok(valid_str) => debug_assert_eq!(valid_str, "valid UTF-8"),
            Err(e) => panic!("Test failed with error: {}", e),
        }
        let _rug_ed_tests_rug_310_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_311 {
    use super::*;
    use crate::de::{self, StrRead};
    use std::vec::Vec;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        let p1 = rug_fuzz_1;
        let mut p2 = Vec::<u8>::new();
        crate::read::parse_escape(&mut p0, p1, &mut p2).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_313 {
    use super::*;
    use crate::de::Read;
    use crate::de::SliceRead;
    use crate::error::Result;
    #[test]
    fn test_ignore_escape() {
        let _rug_st_tests_rug_313_rrrruuuugggg_test_ignore_escape = 0;
        let rug_fuzz_0 = br#"\\"#;
        let json_slice = rug_fuzz_0;
        let mut p0 = SliceRead::new(json_slice);
        debug_assert!(crate ::read::ignore_escape(& mut p0).is_ok());
        let _rug_ed_tests_rug_313_rrrruuuugggg_test_ignore_escape = 0;
    }
}
#[cfg(test)]
mod tests_rug_314 {
    use super::*;
    #[test]
    fn test_decode_hex_val() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::read::decode_hex_val(p0), Some(9));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_315 {
    use super::*;
    use crate::read::IoRead;
    use std::io::{self, Read};
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_315_rrrruuuugggg_test_new = 0;
        let stdin = io::stdin();
        let mut p0 = stdin.lock();
        IoRead::new(p0);
        let _rug_ed_tests_rug_315_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_317 {
    use super::*;
    use crate::de::{IoRead, Read};
    use std::io::Cursor;
    use crate::error::Error;
    #[test]
    fn test_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io = Cursor::new(vec![rug_fuzz_0]);
        let mut p0 = IoRead::new(io);
        debug_assert_eq!(
            < IoRead < Cursor < Vec < u8 > > > as Read > ::next(& mut p0).unwrap(),
            Some(42)
        );
        debug_assert_eq!(
            < IoRead < Cursor < Vec < u8 > > > as Read > ::next(& mut p0).unwrap(), None
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_318 {
    use super::*;
    use crate::de::{Read, IoRead};
    use crate::Error;
    use std::io::Cursor;
    #[test]
    fn test_peek() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io = Cursor::new(vec![rug_fuzz_0, 0x02, 0x03]);
        let mut p0 = IoRead::new(io);
        debug_assert_eq!(p0.peek().unwrap(), Some(0x01));
        p0.peek().unwrap();
        debug_assert_eq!(p0.peek().unwrap(), Some(0x01));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_319 {
    use super::*;
    use crate::de::{IoRead, Read};
    use crate::read::Position;
    use std::io::Cursor;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_319_rrrruuuugggg_test_rug = 0;
        let io = Cursor::new(vec![]);
        let mut p0 = IoRead::new(io);
        let _position: Position = IoRead::peek_position(&p0);
        let _rug_ed_tests_rug_319_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_320 {
    use super::*;
    use crate::de::{IoRead, Read};
    use std::io::Cursor;
    #[test]
    fn test_byte_offset() {
        let _rug_st_tests_rug_320_rrrruuuugggg_test_byte_offset = 0;
        let io = Cursor::new(vec![]);
        let mut p0 = IoRead::new(io);
        debug_assert_eq!(
            < IoRead < Cursor < Vec < u8 > > > as Read > ::byte_offset(& p0), 0
        );
        let _rug_ed_tests_rug_320_rrrruuuugggg_test_byte_offset = 0;
    }
}
#[cfg(test)]
mod tests_rug_321 {
    use super::*;
    use crate::de::{IoRead, Read};
    use std::io::Cursor;
    #[test]
    fn test_parse_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io = Cursor::new(vec![rug_fuzz_0, b't', b'e', b's', b't', b'\"']);
        let mut p0 = IoRead::new(io);
        let mut p1 = Vec::<u8>::new();
        debug_assert!(p0.parse_str(& mut p1).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_322 {
    use super::*;
    use crate::de::{IoRead, Read};
    use std::io::Cursor;
    #[test]
    fn test_parse_str_raw() {
        let _rug_st_tests_rug_322_rrrruuuugggg_test_parse_str_raw = 0;
        let io = Cursor::new(vec![]);
        let mut p0 = IoRead::new(io);
        let mut p1 = Vec::<u8>::new();
        let _ = p0.parse_str_raw(&mut p1);
        let _rug_ed_tests_rug_322_rrrruuuugggg_test_parse_str_raw = 0;
    }
}
#[cfg(test)]
mod tests_rug_323 {
    use super::*;
    use crate::de::Read;
    use crate::de::IoRead;
    use std::io::Cursor;
    #[test]
    fn test_ignore_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io = Cursor::new(vec![rug_fuzz_0, b'\\', b'n', b'"']);
        let mut p0 = IoRead::new(io);
        debug_assert!(
            < IoRead < Cursor < Vec < u8 > > > > ::ignore_str(& mut p0).is_ok()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_324 {
    use super::*;
    use crate::de::Read;
    use crate::de::IoRead;
    use std::io::Cursor;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io = Cursor::new(vec![rug_fuzz_0, b'1', b'A', b'f']);
        let mut p0 = IoRead::new(io);
        debug_assert_eq!(p0.decode_hex_escape().unwrap(), 0x01AF);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_325 {
    use crate::read::SliceRead;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_325_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"some sample json data";
        let sample_data: &[u8] = rug_fuzz_0;
        let mut p0 = sample_data;
        SliceRead::<'_>::new(p0);
        let _rug_ed_tests_rug_325_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_327 {
    use super::*;
    use crate::de::Read;
    use crate::read::SliceRead;
    #[test]
    fn test_peek_position() {
        let _rug_st_tests_rug_327_rrrruuuugggg_test_peek_position = 0;
        let rug_fuzz_0 = b"sample data";
        let data = rug_fuzz_0;
        let mut p0 = SliceRead::new(data);
        p0.peek_position();
        let _rug_ed_tests_rug_327_rrrruuuugggg_test_peek_position = 0;
    }
}
#[cfg(test)]
mod tests_rug_328 {
    use super::*;
    use crate::de::Read;
    use crate::read::{Reference, SliceRead};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_328_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"sample data";
        let data = rug_fuzz_0;
        let mut p0 = SliceRead::new(data);
        let mut p1 = Vec::<u8>::new();
        match <SliceRead as Read>::parse_str(&mut p0, &mut p1) {
            Ok(reference) => {
                match reference {
                    Reference::Borrowed(s) => println!("Borrowed: {}", s),
                    Reference::Copied(s) => println!("Copied: {}", s),
                }
            }
            Err(e) => panic!("Error parsing string: {:?}", e),
        }
        let _rug_ed_tests_rug_328_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_329 {
    use super::*;
    use crate::de::Read;
    use crate::error::Result;
    use crate::read::{ErrorCode, SliceRead, ignore_escape};
    #[test]
    fn test_ignore_str() {
        let _rug_st_tests_rug_329_rrrruuuugggg_test_ignore_str = 0;
        let rug_fuzz_0 = b"\"Sample string\"";
        let data = rug_fuzz_0;
        let mut p0 = SliceRead::new(data);
        debug_assert!(p0.ignore_str().is_ok());
        let _rug_ed_tests_rug_329_rrrruuuugggg_test_ignore_str = 0;
    }
}
#[cfg(test)]
mod tests_rug_330 {
    use super::*;
    use crate::de::Read;
    use crate::read::{SliceRead, error, ErrorCode};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_330_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"0123456789abcdef";
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = b"xyz";
        let rug_fuzz_3 = b"12";
        let data = rug_fuzz_0;
        let mut p0 = SliceRead::new(data);
        debug_assert_eq!(
            < SliceRead < '_ > > ::decode_hex_escape(& mut p0).unwrap(), 0x0123
        );
        let mut p0 = SliceRead::new(&data[rug_fuzz_1..]);
        debug_assert_eq!(
            < SliceRead < '_ > > ::decode_hex_escape(& mut p0).unwrap(), 0x4567
        );
        let invalid_data = rug_fuzz_2;
        let mut p0 = SliceRead::new(invalid_data);
        debug_assert!(
            matches!(< SliceRead < '_ > > ::decode_hex_escape(& mut p0), Err(_))
        );
        let incomplete_data = rug_fuzz_3;
        let mut p0 = SliceRead::new(incomplete_data);
        debug_assert!(
            matches!(< SliceRead < '_ > > ::decode_hex_escape(& mut p0), Err(_))
        );
        let _rug_ed_tests_rug_330_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_331 {
    use crate::read::{Position, Read, StrRead};
    #[test]
    fn test_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0;
        let mut p0 = StrRead::new(data);
        p0.position();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_332 {
    use super::*;
    use crate::de::Read;
    use crate::read;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0;
        let mut p0 = read::StrRead::new(data);
        let mut p1 = std::vec::Vec::<u8>::new();
        let _ = <read::StrRead<'_>>::parse_str(&mut p0, &mut p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_333 {
    use super::*;
    use crate::de::Read;
    use crate::read::StrRead;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0;
        let mut p0 = StrRead::new(data);
        let mut p1 = std::vec::Vec::<u8>::new();
        let _ = p0.parse_str_raw(&mut p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_334 {
    use super::*;
    use crate::de::Read;
    use crate::read::StrRead;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = rug_fuzz_0;
        let mut p0 = StrRead::new(data);
        debug_assert_eq!(
            < StrRead < '_ > > ::decode_hex_escape(& mut p0).unwrap(), 0x12AB
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_335 {
    use super::*;
    use crate::de::{Read, SliceRead};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_335_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = br#"{"some":"data"}"#;
        let json_slice = rug_fuzz_0;
        let mut p0: SliceRead = SliceRead::new(json_slice);
        p0.next();
        let _rug_ed_tests_rug_335_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_336 {
    use super::*;
    use crate::de::{self, Read, StrRead};
    #[test]
    fn test_peek() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        debug_assert!(p0.peek().is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_337 {
    use super::*;
    use crate::de::{self, Read, StrRead};
    #[test]
    fn test_discard() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        p0.discard();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_338 {
    use super::*;
    use crate::de::{self, Read, StrRead};
    #[test]
    fn test_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        p0.position();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_339 {
    use crate::de::{Read, SliceRead};
    use crate::read::Position;
    #[test]
    fn test_peek_position() {
        let _rug_st_tests_rug_339_rrrruuuugggg_test_peek_position = 0;
        let rug_fuzz_0 = br#"{"some":"data"}"#;
        let json_slice = rug_fuzz_0;
        let mut p0 = SliceRead::new(json_slice);
        p0.peek_position();
        let _rug_ed_tests_rug_339_rrrruuuugggg_test_peek_position = 0;
    }
}
#[cfg(test)]
mod tests_rug_340 {
    use super::*;
    use crate::de::{Read, SliceRead};
    #[test]
    fn test_byte_offset() {
        let _rug_st_tests_rug_340_rrrruuuugggg_test_byte_offset = 0;
        let rug_fuzz_0 = br#"{"some":"data"}"#;
        let json_slice = rug_fuzz_0;
        let mut p0 = SliceRead::new(json_slice);
        p0.byte_offset();
        let _rug_ed_tests_rug_340_rrrruuuugggg_test_byte_offset = 0;
    }
}
#[cfg(test)]
mod tests_rug_341 {
    use super::*;
    use crate::de::Read;
    use crate::de::{self, StrRead};
    use std::vec::Vec;
    #[test]
    fn test_parse_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        let mut p1: Vec<u8> = Vec::new();
        p0.parse_str(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_342 {
    use super::*;
    use crate::de::{Read, SliceRead};
    use std::vec::Vec;
    #[test]
    fn test_parse_str_raw() {
        let _rug_st_tests_rug_342_rrrruuuugggg_test_parse_str_raw = 0;
        let rug_fuzz_0 = br#"{"some":"data"}"#;
        let json_slice = rug_fuzz_0;
        let mut p0 = SliceRead::new(json_slice);
        let mut p1 = Vec::<u8>::new();
        p0.parse_str_raw(&mut p1).unwrap();
        let _rug_ed_tests_rug_342_rrrruuuugggg_test_parse_str_raw = 0;
    }
}
#[cfg(test)]
mod tests_rug_343 {
    use super::*;
    use crate::de::{self, Read, StrRead};
    #[test]
    fn test_decode_hex_escape() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_str = rug_fuzz_0;
        let mut p0 = StrRead::new(json_str);
        debug_assert_eq!(
            < de::StrRead as Read > ::decode_hex_escape(& mut p0).unwrap(), 0x00ff
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_344 {
    use super::*;
    use crate::de::{Read, SliceRead};
    #[test]
    fn test_set_failed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 15], bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let json_slice = rug_fuzz_0;
        let mut p0 = SliceRead::new(json_slice);
        let mut p1: bool = rug_fuzz_1;
        p0.set_failed(&mut p1);
             }
}
}
}    }
}
