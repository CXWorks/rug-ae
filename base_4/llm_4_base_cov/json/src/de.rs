//! Deserialize JSON data to a Rust data structure.
use crate::error::{Error, ErrorCode, Result};
#[cfg(feature = "float_roundtrip")]
use crate::lexical;
use crate::number::Number;
use crate::read::{self, Fused, Reference};
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "float_roundtrip")]
use core::iter;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::result;
use core::str::FromStr;
use serde::de::{self, Expected, Unexpected};
use serde::forward_to_deserialize_any;
#[cfg(feature = "arbitrary_precision")]
use crate::number::NumberDeserializer;
pub use crate::read::{Read, SliceRead, StrRead};
#[cfg(feature = "std")]
pub use crate::read::IoRead;
/// A structure that deserializes JSON into Rust values.
pub struct Deserializer<R> {
    read: R,
    scratch: Vec<u8>,
    remaining_depth: u8,
    #[cfg(feature = "float_roundtrip")]
    single_precision: bool,
    #[cfg(feature = "unbounded_depth")]
    disable_recursion_limit: bool,
}
impl<'de, R> Deserializer<R>
where
    R: read::Read<'de>,
{
    /// Create a JSON deserializer from one of the possible serde_json input
    /// sources.
    ///
    /// Typically it is more convenient to use one of these methods instead:
    ///
    ///   - Deserializer::from_str
    ///   - Deserializer::from_slice
    ///   - Deserializer::from_reader
    pub fn new(read: R) -> Self {
        Deserializer {
            read,
            scratch: Vec::new(),
            remaining_depth: 128,
            #[cfg(feature = "float_roundtrip")]
            single_precision: false,
            #[cfg(feature = "unbounded_depth")]
            disable_recursion_limit: false,
        }
    }
}
#[cfg(feature = "std")]
impl<R> Deserializer<read::IoRead<R>>
where
    R: crate::io::Read,
{
    /// Creates a JSON deserializer from an `io::Read`.
    ///
    /// Reader-based deserializers do not support deserializing borrowed types
    /// like `&str`, since the `std::io::Read` trait has no non-copying methods
    /// -- everything it does involves copying bytes out of the data source.
    pub fn from_reader(reader: R) -> Self {
        Deserializer::new(read::IoRead::new(reader))
    }
}
impl<'a> Deserializer<read::SliceRead<'a>> {
    /// Creates a JSON deserializer from a `&[u8]`.
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        Deserializer::new(read::SliceRead::new(bytes))
    }
}
impl<'a> Deserializer<read::StrRead<'a>> {
    /// Creates a JSON deserializer from a `&str`.
    pub fn from_str(s: &'a str) -> Self {
        Deserializer::new(read::StrRead::new(s))
    }
}
macro_rules! overflow {
    ($a:ident * 10 + $b:ident, $c:expr) => {
        match $c { c => $a >= c / 10 && ($a > c / 10 || $b > c % 10), }
    };
}
pub(crate) enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
    #[cfg(feature = "arbitrary_precision")]
    String(String),
}
impl ParserNumber {
    fn visit<'de, V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            ParserNumber::F64(x) => visitor.visit_f64(x),
            ParserNumber::U64(x) => visitor.visit_u64(x),
            ParserNumber::I64(x) => visitor.visit_i64(x),
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(x) => {
                visitor
                    .visit_map(NumberDeserializer {
                        number: x.into(),
                    })
            }
        }
    }
    fn invalid_type(self, exp: &dyn Expected) -> Error {
        match self {
            ParserNumber::F64(x) => de::Error::invalid_type(Unexpected::Float(x), exp),
            ParserNumber::U64(x) => de::Error::invalid_type(Unexpected::Unsigned(x), exp),
            ParserNumber::I64(x) => de::Error::invalid_type(Unexpected::Signed(x), exp),
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(_) => {
                de::Error::invalid_type(Unexpected::Other("number"), exp)
            }
        }
    }
}
impl<'de, R: Read<'de>> Deserializer<R> {
    /// The `Deserializer::end` method should be called after a value has been fully deserialized.
    /// This allows the `Deserializer` to validate that the input stream is at the end or that it
    /// only has trailing whitespace.
    pub fn end(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            None => Ok(()),
        }
    }
    /// Turn a JSON deserializer into an iterator over values of type T.
    pub fn into_iter<T>(self) -> StreamDeserializer<'de, R, T>
    where
        T: de::Deserialize<'de>,
    {
        let offset = self.read.byte_offset();
        StreamDeserializer {
            de: self,
            offset,
            failed: false,
            output: PhantomData,
            lifetime: PhantomData,
        }
    }
    /// Parse arbitrarily deep JSON structures without any consideration for
    /// overflowing the stack.
    ///
    /// You will want to provide some other way to protect against stack
    /// overflows, such as by wrapping your Deserializer in the dynamically
    /// growing stack adapter provided by the serde_stacker crate. Additionally
    /// you will need to be careful around other recursive operations on the
    /// parsed result which may overflow the stack after deserialization has
    /// completed, including, but not limited to, Display and Debug and Drop
    /// impls.
    ///
    /// *This method is only available if serde_json is built with the
    /// `"unbounded_depth"` feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::Deserialize;
    /// use serde_json::Value;
    ///
    /// fn main() {
    ///     let mut json = String::new();
    ///     for _ in 0..10000 {
    ///         json = format!("[{}]", json);
    ///     }
    ///
    ///     let mut deserializer = serde_json::Deserializer::from_str(&json);
    ///     deserializer.disable_recursion_limit();
    ///     let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    ///     let value = Value::deserialize(deserializer).unwrap();
    ///
    ///     carefully_drop_nested_arrays(value);
    /// }
    ///
    /// fn carefully_drop_nested_arrays(value: Value) {
    ///     let mut stack = vec![value];
    ///     while let Some(value) = stack.pop() {
    ///         if let Value::Array(array) = value {
    ///             stack.extend(array);
    ///         }
    ///     }
    /// }
    /// ```
    #[cfg(feature = "unbounded_depth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unbounded_depth")))]
    pub fn disable_recursion_limit(&mut self) {
        self.disable_recursion_limit = true;
    }
    fn peek(&mut self) -> Result<Option<u8>> {
        self.read.peek()
    }
    fn peek_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.peek()).unwrap_or(b'\x00'))
    }
    fn eat_char(&mut self) {
        self.read.discard();
    }
    fn next_char(&mut self) -> Result<Option<u8>> {
        self.read.next()
    }
    fn next_char_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.next_char()).unwrap_or(b'\x00'))
    }
    /// Error caused by a byte from next_char().
    #[cold]
    fn error(&self, reason: ErrorCode) -> Error {
        let position = self.read.position();
        Error::syntax(reason, position.line, position.column)
    }
    /// Error caused by a byte from peek().
    #[cold]
    fn peek_error(&self, reason: ErrorCode) -> Error {
        let position = self.read.peek_position();
        Error::syntax(reason, position.line, position.column)
    }
    /// Returns the first non-whitespace byte without consuming it, or `None` if
    /// EOF is encountered.
    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        loop {
            match tri!(self.peek()) {
                Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r') => {
                    self.eat_char();
                }
                other => {
                    return Ok(other);
                }
            }
        }
    }
    #[cold]
    fn peek_invalid_type(&mut self, exp: &dyn Expected) -> Error {
        let err = match self.peek_or_null().unwrap_or(b'\x00') {
            b'n' => {
                self.eat_char();
                if let Err(err) = self.parse_ident(b"ull") {
                    return err;
                }
                de::Error::invalid_type(Unexpected::Unit, exp)
            }
            b't' => {
                self.eat_char();
                if let Err(err) = self.parse_ident(b"rue") {
                    return err;
                }
                de::Error::invalid_type(Unexpected::Bool(true), exp)
            }
            b'f' => {
                self.eat_char();
                if let Err(err) = self.parse_ident(b"alse") {
                    return err;
                }
                de::Error::invalid_type(Unexpected::Bool(false), exp)
            }
            b'-' => {
                self.eat_char();
                match self.parse_any_number(false) {
                    Ok(n) => n.invalid_type(exp),
                    Err(err) => return err,
                }
            }
            b'0'..=b'9' => {
                match self.parse_any_number(true) {
                    Ok(n) => n.invalid_type(exp),
                    Err(err) => return err,
                }
            }
            b'"' => {
                self.eat_char();
                self.scratch.clear();
                match self.read.parse_str(&mut self.scratch) {
                    Ok(s) => de::Error::invalid_type(Unexpected::Str(&s), exp),
                    Err(err) => return err,
                }
            }
            b'[' => de::Error::invalid_type(Unexpected::Seq, exp),
            b'{' => de::Error::invalid_type(Unexpected::Map, exp),
            _ => self.peek_error(ErrorCode::ExpectedSomeValue),
        };
        self.fix_position(err)
    }
    fn deserialize_number<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'-' => {
                self.eat_char();
                tri!(self.parse_integer(false)).visit(visitor)
            }
            b'0'..=b'9' => tri!(self.parse_integer(true)).visit(visitor),
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
        match tri!(self.next_char_or_null()) {
            b'0' => {
                buf.push('0');
                match tri!(self.peek_or_null()) {
                    b'0'..=b'9' => Err(self.peek_error(ErrorCode::InvalidNumber)),
                    _ => Ok(()),
                }
            }
            c @ b'1'..=b'9' => {
                buf.push(c as char);
                while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
                    self.eat_char();
                    buf.push(c as char);
                }
                Ok(())
            }
            _ => Err(self.error(ErrorCode::InvalidNumber)),
        }
    }
    #[cold]
    fn fix_position(&self, err: Error) -> Error {
        err.fix_position(move |code| self.error(code))
    }
    fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
        for expected in ident {
            match tri!(self.next_char()) {
                None => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
                Some(next) => {
                    if next != *expected {
                        return Err(self.error(ErrorCode::ExpectedSomeIdent));
                    }
                }
            }
        }
        Ok(())
    }
    fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber> {
        let next = match tri!(self.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };
        match next {
            b'0' => {
                match tri!(self.peek_or_null()) {
                    b'0'..=b'9' => Err(self.peek_error(ErrorCode::InvalidNumber)),
                    _ => self.parse_number(positive, 0),
                }
            }
            c @ b'1'..=b'9' => {
                let mut significand = (c - b'0') as u64;
                loop {
                    match tri!(self.peek_or_null()) {
                        c @ b'0'..=b'9' => {
                            let digit = (c - b'0') as u64;
                            if overflow!(significand * 10 + digit, u64::max_value()) {
                                return Ok(
                                    ParserNumber::F64(
                                        tri!(self.parse_long_integer(positive, significand),),
                                    ),
                                );
                            }
                            self.eat_char();
                            significand = significand * 10 + digit;
                        }
                        _ => {
                            return self.parse_number(positive, significand);
                        }
                    }
                }
            }
            _ => Err(self.error(ErrorCode::InvalidNumber)),
        }
    }
    fn parse_number(
        &mut self,
        positive: bool,
        significand: u64,
    ) -> Result<ParserNumber> {
        Ok(
            match tri!(self.peek_or_null()) {
                b'.' => {
                    ParserNumber::F64(tri!(self.parse_decimal(positive, significand, 0)))
                }
                b'e' | b'E' => {
                    ParserNumber::F64(
                        tri!(self.parse_exponent(positive, significand, 0)),
                    )
                }
                _ => {
                    if positive {
                        ParserNumber::U64(significand)
                    } else {
                        let neg = (significand as i64).wrapping_neg();
                        if neg >= 0 {
                            ParserNumber::F64(-(significand as f64))
                        } else {
                            ParserNumber::I64(neg)
                        }
                    }
                }
            },
        )
    }
    fn parse_decimal(
        &mut self,
        positive: bool,
        mut significand: u64,
        exponent_before_decimal_point: i32,
    ) -> Result<f64> {
        self.eat_char();
        let mut exponent_after_decimal_point = 0;
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            let digit = (c - b'0') as u64;
            if overflow!(significand * 10 + digit, u64::max_value()) {
                let exponent = exponent_before_decimal_point
                    + exponent_after_decimal_point;
                return self.parse_decimal_overflow(positive, significand, exponent);
            }
            self.eat_char();
            significand = significand * 10 + digit;
            exponent_after_decimal_point -= 1;
        }
        if exponent_after_decimal_point == 0 {
            match tri!(self.peek()) {
                Some(_) => return Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            }
        }
        let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.parse_exponent(positive, significand, exponent),
            _ => self.f64_from_parts(positive, significand, exponent),
        }
    }
    fn parse_exponent(
        &mut self,
        positive: bool,
        significand: u64,
        starting_exp: i32,
    ) -> Result<f64> {
        self.eat_char();
        let positive_exp = match tri!(self.peek_or_null()) {
            b'+' => {
                self.eat_char();
                true
            }
            b'-' => {
                self.eat_char();
                false
            }
            _ => true,
        };
        let next = match tri!(self.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };
        let mut exp = match next {
            c @ b'0'..=b'9' => (c - b'0') as i32,
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        };
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            let digit = (c - b'0') as i32;
            if overflow!(exp * 10 + digit, i32::max_value()) {
                let zero_significand = significand == 0;
                return self
                    .parse_exponent_overflow(positive, zero_significand, positive_exp);
            }
            exp = exp * 10 + digit;
        }
        let final_exp = if positive_exp {
            starting_exp.saturating_add(exp)
        } else {
            starting_exp.saturating_sub(exp)
        };
        self.f64_from_parts(positive, significand, final_exp)
    }
    #[cfg(feature = "float_roundtrip")]
    fn f64_from_parts(
        &mut self,
        positive: bool,
        significand: u64,
        exponent: i32,
    ) -> Result<f64> {
        let f = if self.single_precision {
            lexical::parse_concise_float::<f32>(significand, exponent) as f64
        } else {
            lexical::parse_concise_float::<f64>(significand, exponent)
        };
        if f.is_infinite() {
            Err(self.error(ErrorCode::NumberOutOfRange))
        } else {
            Ok(if positive { f } else { -f })
        }
    }
    #[cfg(not(feature = "float_roundtrip"))]
    fn f64_from_parts(
        &mut self,
        positive: bool,
        significand: u64,
        mut exponent: i32,
    ) -> Result<f64> {
        let mut f = significand as f64;
        loop {
            match POW10.get(exponent.wrapping_abs() as usize) {
                Some(&pow) => {
                    if exponent >= 0 {
                        f *= pow;
                        if f.is_infinite() {
                            return Err(self.error(ErrorCode::NumberOutOfRange));
                        }
                    } else {
                        f /= pow;
                    }
                    break;
                }
                None => {
                    if f == 0.0 {
                        break;
                    }
                    if exponent >= 0 {
                        return Err(self.error(ErrorCode::NumberOutOfRange));
                    }
                    f /= 1e308;
                    exponent += 308;
                }
            }
        }
        Ok(if positive { f } else { -f })
    }
    #[cfg(feature = "float_roundtrip")]
    #[cold]
    #[inline(never)]
    fn parse_long_integer(
        &mut self,
        positive: bool,
        partial_significand: u64,
    ) -> Result<f64> {
        self.scratch.clear();
        self.scratch
            .extend_from_slice(
                itoa::Buffer::new().format(partial_significand).as_bytes(),
            );
        loop {
            match tri!(self.peek_or_null()) {
                c @ b'0'..=b'9' => {
                    self.scratch.push(c);
                    self.eat_char();
                }
                b'.' => {
                    self.eat_char();
                    return self.parse_long_decimal(positive, self.scratch.len());
                }
                b'e' | b'E' => {
                    return self.parse_long_exponent(positive, self.scratch.len());
                }
                _ => {
                    return self.f64_long_from_parts(positive, self.scratch.len(), 0);
                }
            }
        }
    }
    #[cfg(not(feature = "float_roundtrip"))]
    #[cold]
    #[inline(never)]
    fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64> {
        let mut exponent = 0;
        loop {
            match tri!(self.peek_or_null()) {
                b'0'..=b'9' => {
                    self.eat_char();
                    exponent += 1;
                }
                b'.' => {
                    return self.parse_decimal(positive, significand, exponent);
                }
                b'e' | b'E' => {
                    return self.parse_exponent(positive, significand, exponent);
                }
                _ => {
                    return self.f64_from_parts(positive, significand, exponent);
                }
            }
        }
    }
    #[cfg(feature = "float_roundtrip")]
    #[cold]
    fn parse_long_decimal(&mut self, positive: bool, integer_end: usize) -> Result<f64> {
        let mut at_least_one_digit = integer_end < self.scratch.len();
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.scratch.push(c);
            self.eat_char();
            at_least_one_digit = true;
        }
        if !at_least_one_digit {
            match tri!(self.peek()) {
                Some(_) => return Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            }
        }
        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.parse_long_exponent(positive, integer_end),
            _ => self.f64_long_from_parts(positive, integer_end, 0),
        }
    }
    #[cfg(feature = "float_roundtrip")]
    fn parse_long_exponent(
        &mut self,
        positive: bool,
        integer_end: usize,
    ) -> Result<f64> {
        self.eat_char();
        let positive_exp = match tri!(self.peek_or_null()) {
            b'+' => {
                self.eat_char();
                true
            }
            b'-' => {
                self.eat_char();
                false
            }
            _ => true,
        };
        let next = match tri!(self.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };
        let mut exp = match next {
            c @ b'0'..=b'9' => (c - b'0') as i32,
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        };
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            let digit = (c - b'0') as i32;
            if overflow!(exp * 10 + digit, i32::max_value()) {
                let zero_significand = self.scratch.iter().all(|&digit| digit == b'0');
                return self
                    .parse_exponent_overflow(positive, zero_significand, positive_exp);
            }
            exp = exp * 10 + digit;
        }
        let final_exp = if positive_exp { exp } else { -exp };
        self.f64_long_from_parts(positive, integer_end, final_exp)
    }
    #[cfg(feature = "float_roundtrip")]
    #[cold]
    #[inline(never)]
    fn parse_decimal_overflow(
        &mut self,
        positive: bool,
        significand: u64,
        exponent: i32,
    ) -> Result<f64> {
        let mut buffer = itoa::Buffer::new();
        let significand = buffer.format(significand);
        let fraction_digits = -exponent as usize;
        self.scratch.clear();
        if let Some(zeros) = fraction_digits.checked_sub(significand.len() + 1) {
            self.scratch.extend(iter::repeat(b'0').take(zeros + 1));
        }
        self.scratch.extend_from_slice(significand.as_bytes());
        let integer_end = self.scratch.len() - fraction_digits;
        self.parse_long_decimal(positive, integer_end)
    }
    #[cfg(not(feature = "float_roundtrip"))]
    #[cold]
    #[inline(never)]
    fn parse_decimal_overflow(
        &mut self,
        positive: bool,
        significand: u64,
        exponent: i32,
    ) -> Result<f64> {
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
        }
        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.parse_exponent(positive, significand, exponent),
            _ => self.f64_from_parts(positive, significand, exponent),
        }
    }
    #[cold]
    #[inline(never)]
    fn parse_exponent_overflow(
        &mut self,
        positive: bool,
        zero_significand: bool,
        positive_exp: bool,
    ) -> Result<f64> {
        if !zero_significand && positive_exp {
            return Err(self.error(ErrorCode::NumberOutOfRange));
        }
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
        }
        Ok(if positive { 0.0 } else { -0.0 })
    }
    #[cfg(feature = "float_roundtrip")]
    fn f64_long_from_parts(
        &mut self,
        positive: bool,
        integer_end: usize,
        exponent: i32,
    ) -> Result<f64> {
        let integer = &self.scratch[..integer_end];
        let fraction = &self.scratch[integer_end..];
        let f = if self.single_precision {
            lexical::parse_truncated_float::<f32>(integer, fraction, exponent) as f64
        } else {
            lexical::parse_truncated_float::<f64>(integer, fraction, exponent)
        };
        if f.is_infinite() {
            Err(self.error(ErrorCode::NumberOutOfRange))
        } else {
            Ok(if positive { f } else { -f })
        }
    }
    fn parse_any_signed_number(&mut self) -> Result<ParserNumber> {
        let peek = match tri!(self.peek()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'-' => {
                self.eat_char();
                self.parse_any_number(false)
            }
            b'0'..=b'9' => self.parse_any_number(true),
            _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
        };
        let value = match tri!(self.peek()) {
            Some(_) => Err(self.peek_error(ErrorCode::InvalidNumber)),
            None => value,
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    #[cfg(not(feature = "arbitrary_precision"))]
    fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber> {
        self.parse_integer(positive)
    }
    #[cfg(feature = "arbitrary_precision")]
    fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber> {
        let mut buf = String::with_capacity(16);
        if !positive {
            buf.push('-');
        }
        self.scan_integer(&mut buf)?;
        if positive {
            if let Ok(unsigned) = buf.parse() {
                return Ok(ParserNumber::U64(unsigned));
            }
        } else {
            if let Ok(signed) = buf.parse() {
                return Ok(ParserNumber::I64(signed));
            }
        }
        Ok(ParserNumber::String(buf))
    }
    #[cfg(feature = "arbitrary_precision")]
    fn scan_or_eof(&mut self, buf: &mut String) -> Result<u8> {
        match tri!(self.next_char()) {
            Some(b) => {
                buf.push(b as char);
                Ok(b)
            }
            None => Err(self.error(ErrorCode::EofWhileParsingValue)),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    fn scan_integer(&mut self, buf: &mut String) -> Result<()> {
        match tri!(self.scan_or_eof(buf)) {
            b'0' => {
                match tri!(self.peek_or_null()) {
                    b'0'..=b'9' => Err(self.peek_error(ErrorCode::InvalidNumber)),
                    _ => self.scan_number(buf),
                }
            }
            b'1'..=b'9' => {
                loop {
                    match tri!(self.peek_or_null()) {
                        c @ b'0'..=b'9' => {
                            self.eat_char();
                            buf.push(c as char);
                        }
                        _ => {
                            return self.scan_number(buf);
                        }
                    }
                }
            }
            _ => Err(self.error(ErrorCode::InvalidNumber)),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    fn scan_number(&mut self, buf: &mut String) -> Result<()> {
        match tri!(self.peek_or_null()) {
            b'.' => self.scan_decimal(buf),
            e @ b'e' | e @ b'E' => self.scan_exponent(e as char, buf),
            _ => Ok(()),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    fn scan_decimal(&mut self, buf: &mut String) -> Result<()> {
        self.eat_char();
        buf.push('.');
        let mut at_least_one_digit = false;
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            buf.push(c as char);
            at_least_one_digit = true;
        }
        if !at_least_one_digit {
            match tri!(self.peek()) {
                Some(_) => return Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            }
        }
        match tri!(self.peek_or_null()) {
            e @ b'e' | e @ b'E' => self.scan_exponent(e as char, buf),
            _ => Ok(()),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    fn scan_exponent(&mut self, e: char, buf: &mut String) -> Result<()> {
        self.eat_char();
        buf.push(e);
        match tri!(self.peek_or_null()) {
            b'+' => {
                self.eat_char();
                buf.push('+');
            }
            b'-' => {
                self.eat_char();
                buf.push('-');
            }
            _ => {}
        }
        match tri!(self.scan_or_eof(buf)) {
            b'0'..=b'9' => {}
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        }
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            buf.push(c as char);
        }
        Ok(())
    }
    fn parse_object_colon(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(b':') => {
                self.eat_char();
                Ok(())
            }
            Some(_) => Err(self.peek_error(ErrorCode::ExpectedColon)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
        }
    }
    fn end_seq(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(b']') => {
                self.eat_char();
                Ok(())
            }
            Some(b',') => {
                self.eat_char();
                match self.parse_whitespace() {
                    Ok(Some(b']')) => Err(self.peek_error(ErrorCode::TrailingComma)),
                    _ => Err(self.peek_error(ErrorCode::TrailingCharacters)),
                }
            }
            Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingList)),
        }
    }
    fn end_map(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(b'}') => {
                self.eat_char();
                Ok(())
            }
            Some(b',') => Err(self.peek_error(ErrorCode::TrailingComma)),
            Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
        }
    }
    fn ignore_value(&mut self) -> Result<()> {
        self.scratch.clear();
        let mut enclosing = None;
        loop {
            let peek = match tri!(self.parse_whitespace()) {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };
            let frame = match peek {
                b'n' => {
                    self.eat_char();
                    tri!(self.parse_ident(b"ull"));
                    None
                }
                b't' => {
                    self.eat_char();
                    tri!(self.parse_ident(b"rue"));
                    None
                }
                b'f' => {
                    self.eat_char();
                    tri!(self.parse_ident(b"alse"));
                    None
                }
                b'-' => {
                    self.eat_char();
                    tri!(self.ignore_integer());
                    None
                }
                b'0'..=b'9' => {
                    tri!(self.ignore_integer());
                    None
                }
                b'"' => {
                    self.eat_char();
                    tri!(self.read.ignore_str());
                    None
                }
                frame @ b'[' | frame @ b'{' => {
                    self.scratch.extend(enclosing.take());
                    self.eat_char();
                    Some(frame)
                }
                _ => return Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
            };
            let (mut accept_comma, mut frame) = match frame {
                Some(frame) => (false, frame),
                None => {
                    match enclosing.take() {
                        Some(frame) => (true, frame),
                        None => {
                            match self.scratch.pop() {
                                Some(frame) => (true, frame),
                                None => return Ok(()),
                            }
                        }
                    }
                }
            };
            loop {
                match tri!(self.parse_whitespace()) {
                    Some(b',') if accept_comma => {
                        self.eat_char();
                        break;
                    }
                    Some(b']') if frame == b'[' => {}
                    Some(b'}') if frame == b'{' => {}
                    Some(_) => {
                        if accept_comma {
                            return Err(
                                self
                                    .peek_error(
                                        match frame {
                                            b'[' => ErrorCode::ExpectedListCommaOrEnd,
                                            b'{' => ErrorCode::ExpectedObjectCommaOrEnd,
                                            _ => unreachable!(),
                                        },
                                    ),
                            );
                        } else {
                            break;
                        }
                    }
                    None => {
                        return Err(
                            self
                                .peek_error(
                                    match frame {
                                        b'[' => ErrorCode::EofWhileParsingList,
                                        b'{' => ErrorCode::EofWhileParsingObject,
                                        _ => unreachable!(),
                                    },
                                ),
                        );
                    }
                }
                self.eat_char();
                frame = match self.scratch.pop() {
                    Some(frame) => frame,
                    None => return Ok(()),
                };
                accept_comma = true;
            }
            if frame == b'{' {
                match tri!(self.parse_whitespace()) {
                    Some(b'"') => self.eat_char(),
                    Some(_) => return Err(self.peek_error(ErrorCode::KeyMustBeAString)),
                    None => return Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
                }
                tri!(self.read.ignore_str());
                match tri!(self.parse_whitespace()) {
                    Some(b':') => self.eat_char(),
                    Some(_) => return Err(self.peek_error(ErrorCode::ExpectedColon)),
                    None => return Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
                }
            }
            enclosing = Some(frame);
        }
    }
    fn ignore_integer(&mut self) -> Result<()> {
        match tri!(self.next_char_or_null()) {
            b'0' => {
                if let b'0'..=b'9' = tri!(self.peek_or_null()) {
                    return Err(self.peek_error(ErrorCode::InvalidNumber));
                }
            }
            b'1'..=b'9' => {
                while let b'0'..=b'9' = tri!(self.peek_or_null()) {
                    self.eat_char();
                }
            }
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        }
        match tri!(self.peek_or_null()) {
            b'.' => self.ignore_decimal(),
            b'e' | b'E' => self.ignore_exponent(),
            _ => Ok(()),
        }
    }
    fn ignore_decimal(&mut self) -> Result<()> {
        self.eat_char();
        let mut at_least_one_digit = false;
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            at_least_one_digit = true;
        }
        if !at_least_one_digit {
            return Err(self.peek_error(ErrorCode::InvalidNumber));
        }
        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.ignore_exponent(),
            _ => Ok(()),
        }
    }
    fn ignore_exponent(&mut self) -> Result<()> {
        self.eat_char();
        match tri!(self.peek_or_null()) {
            b'+' | b'-' => self.eat_char(),
            _ => {}
        }
        match tri!(self.next_char_or_null()) {
            b'0'..=b'9' => {}
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        }
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
        }
        Ok(())
    }
    #[cfg(feature = "raw_value")]
    fn deserialize_raw_value<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.parse_whitespace()?;
        self.read.begin_raw_buffering();
        self.ignore_value()?;
        self.read.end_raw_buffering(visitor)
    }
}
impl FromStr for Number {
    type Err = Error;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Deserializer::from_str(s).parse_any_signed_number().map(Into::into)
    }
}
#[cfg(not(feature = "float_roundtrip"))]
static POW10: [f64; 309] = [
    1e000,
    1e001,
    1e002,
    1e003,
    1e004,
    1e005,
    1e006,
    1e007,
    1e008,
    1e009,
    1e010,
    1e011,
    1e012,
    1e013,
    1e014,
    1e015,
    1e016,
    1e017,
    1e018,
    1e019,
    1e020,
    1e021,
    1e022,
    1e023,
    1e024,
    1e025,
    1e026,
    1e027,
    1e028,
    1e029,
    1e030,
    1e031,
    1e032,
    1e033,
    1e034,
    1e035,
    1e036,
    1e037,
    1e038,
    1e039,
    1e040,
    1e041,
    1e042,
    1e043,
    1e044,
    1e045,
    1e046,
    1e047,
    1e048,
    1e049,
    1e050,
    1e051,
    1e052,
    1e053,
    1e054,
    1e055,
    1e056,
    1e057,
    1e058,
    1e059,
    1e060,
    1e061,
    1e062,
    1e063,
    1e064,
    1e065,
    1e066,
    1e067,
    1e068,
    1e069,
    1e070,
    1e071,
    1e072,
    1e073,
    1e074,
    1e075,
    1e076,
    1e077,
    1e078,
    1e079,
    1e080,
    1e081,
    1e082,
    1e083,
    1e084,
    1e085,
    1e086,
    1e087,
    1e088,
    1e089,
    1e090,
    1e091,
    1e092,
    1e093,
    1e094,
    1e095,
    1e096,
    1e097,
    1e098,
    1e099,
    1e100,
    1e101,
    1e102,
    1e103,
    1e104,
    1e105,
    1e106,
    1e107,
    1e108,
    1e109,
    1e110,
    1e111,
    1e112,
    1e113,
    1e114,
    1e115,
    1e116,
    1e117,
    1e118,
    1e119,
    1e120,
    1e121,
    1e122,
    1e123,
    1e124,
    1e125,
    1e126,
    1e127,
    1e128,
    1e129,
    1e130,
    1e131,
    1e132,
    1e133,
    1e134,
    1e135,
    1e136,
    1e137,
    1e138,
    1e139,
    1e140,
    1e141,
    1e142,
    1e143,
    1e144,
    1e145,
    1e146,
    1e147,
    1e148,
    1e149,
    1e150,
    1e151,
    1e152,
    1e153,
    1e154,
    1e155,
    1e156,
    1e157,
    1e158,
    1e159,
    1e160,
    1e161,
    1e162,
    1e163,
    1e164,
    1e165,
    1e166,
    1e167,
    1e168,
    1e169,
    1e170,
    1e171,
    1e172,
    1e173,
    1e174,
    1e175,
    1e176,
    1e177,
    1e178,
    1e179,
    1e180,
    1e181,
    1e182,
    1e183,
    1e184,
    1e185,
    1e186,
    1e187,
    1e188,
    1e189,
    1e190,
    1e191,
    1e192,
    1e193,
    1e194,
    1e195,
    1e196,
    1e197,
    1e198,
    1e199,
    1e200,
    1e201,
    1e202,
    1e203,
    1e204,
    1e205,
    1e206,
    1e207,
    1e208,
    1e209,
    1e210,
    1e211,
    1e212,
    1e213,
    1e214,
    1e215,
    1e216,
    1e217,
    1e218,
    1e219,
    1e220,
    1e221,
    1e222,
    1e223,
    1e224,
    1e225,
    1e226,
    1e227,
    1e228,
    1e229,
    1e230,
    1e231,
    1e232,
    1e233,
    1e234,
    1e235,
    1e236,
    1e237,
    1e238,
    1e239,
    1e240,
    1e241,
    1e242,
    1e243,
    1e244,
    1e245,
    1e246,
    1e247,
    1e248,
    1e249,
    1e250,
    1e251,
    1e252,
    1e253,
    1e254,
    1e255,
    1e256,
    1e257,
    1e258,
    1e259,
    1e260,
    1e261,
    1e262,
    1e263,
    1e264,
    1e265,
    1e266,
    1e267,
    1e268,
    1e269,
    1e270,
    1e271,
    1e272,
    1e273,
    1e274,
    1e275,
    1e276,
    1e277,
    1e278,
    1e279,
    1e280,
    1e281,
    1e282,
    1e283,
    1e284,
    1e285,
    1e286,
    1e287,
    1e288,
    1e289,
    1e290,
    1e291,
    1e292,
    1e293,
    1e294,
    1e295,
    1e296,
    1e297,
    1e298,
    1e299,
    1e300,
    1e301,
    1e302,
    1e303,
    1e304,
    1e305,
    1e306,
    1e307,
    1e308,
];
macro_rules! deserialize_number {
    ($method:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value > where V : de::Visitor
        <'de >, { self.deserialize_number(visitor) }
    };
}
#[cfg(not(feature = "unbounded_depth"))]
macro_rules! if_checking_recursion_limit {
    ($($body:tt)*) => {
        $($body)*
    };
}
#[cfg(feature = "unbounded_depth")]
macro_rules! if_checking_recursion_limit {
    ($this:ident $($body:tt)*) => {
        if !$this .disable_recursion_limit { $this $($body)* }
    };
}
macro_rules! check_recursion {
    ($this:ident $($body:tt)*) => {
        if_checking_recursion_limit! { $this .remaining_depth -= 1; if $this
        .remaining_depth == 0 { return Err($this
        .peek_error(ErrorCode::RecursionLimitExceeded)); } } $this $($body)*
        if_checking_recursion_limit! { $this .remaining_depth += 1; }
    };
}
impl<'de, 'a, R: Read<'de>> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'n' => {
                self.eat_char();
                tri!(self.parse_ident(b"ull"));
                visitor.visit_unit()
            }
            b't' => {
                self.eat_char();
                tri!(self.parse_ident(b"rue"));
                visitor.visit_bool(true)
            }
            b'f' => {
                self.eat_char();
                tri!(self.parse_ident(b"alse"));
                visitor.visit_bool(false)
            }
            b'-' => {
                self.eat_char();
                tri!(self.parse_any_number(false)).visit(visitor)
            }
            b'0'..=b'9' => tri!(self.parse_any_number(true)).visit(visitor),
            b'"' => {
                self.eat_char();
                self.scratch.clear();
                match tri!(self.read.parse_str(& mut self.scratch)) {
                    Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
                    Reference::Copied(s) => visitor.visit_str(s),
                }
            }
            b'[' => {
                check_recursion! {
                    self.eat_char(); let ret = visitor.visit_seq(SeqAccess::new(self));
                }
                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            b'{' => {
                check_recursion! {
                    self.eat_char(); let ret = visitor.visit_map(MapAccess::new(self));
                }
                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            _ => Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b't' => {
                self.eat_char();
                tri!(self.parse_ident(b"rue"));
                visitor.visit_bool(true)
            }
            b'f' => {
                self.eat_char();
                tri!(self.parse_ident(b"alse"));
                visitor.visit_bool(false)
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    deserialize_number!(deserialize_i8);
    deserialize_number!(deserialize_i16);
    deserialize_number!(deserialize_i32);
    deserialize_number!(deserialize_i64);
    deserialize_number!(deserialize_u8);
    deserialize_number!(deserialize_u16);
    deserialize_number!(deserialize_u32);
    deserialize_number!(deserialize_u64);
    #[cfg(not(feature = "float_roundtrip"))]
    deserialize_number!(deserialize_f32);
    deserialize_number!(deserialize_f64);
    #[cfg(feature = "float_roundtrip")]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.single_precision = true;
        let val = self.deserialize_number(visitor);
        self.single_precision = false;
        val
    }
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = String::new();
        match tri!(self.parse_whitespace()) {
            Some(b'-') => {
                self.eat_char();
                buf.push('-');
            }
            Some(_) => {}
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        tri!(self.scan_integer128(& mut buf));
        let value = match buf.parse() {
            Ok(int) => visitor.visit_i128(int),
            Err(_) => {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match tri!(self.parse_whitespace()) {
            Some(b'-') => {
                return Err(self.peek_error(ErrorCode::NumberOutOfRange));
            }
            Some(_) => {}
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        }
        let mut buf = String::new();
        tri!(self.scan_integer128(& mut buf));
        let value = match buf.parse() {
            Ok(int) => visitor.visit_u128(int),
            Err(_) => {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'"' => {
                self.eat_char();
                self.scratch.clear();
                match tri!(self.read.parse_str(& mut self.scratch)) {
                    Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
                    Reference::Copied(s) => visitor.visit_str(s),
                }
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    /// Parses a JSON string as bytes. Note that this function does not check
    /// whether the bytes represent a valid UTF-8 string.
    ///
    /// The relevant part of the JSON specification is Section 8.2 of [RFC
    /// 7159]:
    ///
    /// > When all the strings represented in a JSON text are composed entirely
    /// > of Unicode characters (however escaped), then that JSON text is
    /// > interoperable in the sense that all software implementations that
    /// > parse it will agree on the contents of names and of string values in
    /// > objects and arrays.
    /// >
    /// > However, the ABNF in this specification allows member names and string
    /// > values to contain bit sequences that cannot encode Unicode characters;
    /// > for example, "\uDEAD" (a single unpaired UTF-16 surrogate). Instances
    /// > of this have been observed, for example, when a library truncates a
    /// > UTF-16 string without checking whether the truncation split a
    /// > surrogate pair.  The behavior of software that receives JSON texts
    /// > containing such values is unpredictable; for example, implementations
    /// > might return different values for the length of a string value or even
    /// > suffer fatal runtime exceptions.
    ///
    /// [RFC 7159]: https://tools.ietf.org/html/rfc7159
    ///
    /// The behavior of serde_json is specified to fail on non-UTF-8 strings
    /// when deserializing into Rust UTF-8 string types such as String, and
    /// succeed with non-UTF-8 bytes when deserializing using this method.
    ///
    /// Escape sequences are processed as usual, and for `\uXXXX` escapes it is
    /// still checked if the hex number represents a valid Unicode code point.
    ///
    /// # Examples
    ///
    /// You can use this to parse JSON strings containing invalid UTF-8 bytes,
    /// or unpaired surrogates.
    ///
    /// ```
    /// use serde_bytes::ByteBuf;
    ///
    /// fn look_at_bytes() -> Result<(), serde_json::Error> {
    ///     let json_data = b"\"some bytes: \xe5\x00\xe5\"";
    ///     let bytes: ByteBuf = serde_json::from_slice(json_data)?;
    ///
    ///     assert_eq!(b'\xe5', bytes[12]);
    ///     assert_eq!(b'\0', bytes[13]);
    ///     assert_eq!(b'\xe5', bytes[14]);
    ///
    ///     Ok(())
    /// }
    /// #
    /// # look_at_bytes().unwrap();
    /// ```
    ///
    /// Backslash escape sequences like `\n` are still interpreted and required
    /// to be valid. `\u` escape sequences are required to represent a valid
    /// Unicode code point or lone surrogate.
    ///
    /// ```
    /// use serde_bytes::ByteBuf;
    ///
    /// fn look_at_bytes() -> Result<(), serde_json::Error> {
    ///     let json_data = b"\"lone surrogate: \\uD801\"";
    ///     let bytes: ByteBuf = serde_json::from_slice(json_data)?;
    ///     let expected = b"lone surrogate: \xED\xA0\x81";
    ///     assert_eq!(expected, bytes.as_slice());
    ///     Ok(())
    /// }
    /// #
    /// # look_at_bytes();
    /// ```
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'"' => {
                self.eat_char();
                self.scratch.clear();
                match tri!(self.read.parse_str_raw(& mut self.scratch)) {
                    Reference::Borrowed(b) => visitor.visit_borrowed_bytes(b),
                    Reference::Copied(b) => visitor.visit_bytes(b),
                }
            }
            b'[' => self.deserialize_seq(visitor),
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }
    /// Parses a `null` as a None, and any other values as a `Some(...)`.
    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match tri!(self.parse_whitespace()) {
            Some(b'n') => {
                self.eat_char();
                tri!(self.parse_ident(b"ull"));
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'n' => {
                self.eat_char();
                tri!(self.parse_ident(b"ull"));
                visitor.visit_unit()
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }
    /// Parses a newtype struct as the underlying value.
    #[inline]
    fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        #[cfg(feature = "raw_value")]
        {
            if name == crate::raw::TOKEN {
                return self.deserialize_raw_value(visitor);
            }
        }
        let _ = name;
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'[' => {
                check_recursion! {
                    self.eat_char(); let ret = visitor.visit_seq(SeqAccess::new(self));
                }
                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'{' => {
                check_recursion! {
                    self.eat_char(); let ret = visitor.visit_map(MapAccess::new(self));
                }
                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };
        let value = match peek {
            b'[' => {
                check_recursion! {
                    self.eat_char(); let ret = visitor.visit_seq(SeqAccess::new(self));
                }
                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            b'{' => {
                check_recursion! {
                    self.eat_char(); let ret = visitor.visit_map(MapAccess::new(self));
                }
                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };
        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
    /// Parses an enum as an object like `{"$KEY":$VALUE}`, where $VALUE is either a straight
    /// value, a `[..]`, or a `{..}`.
    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match tri!(self.parse_whitespace()) {
            Some(b'{') => {
                check_recursion! {
                    self.eat_char(); let value = tri!(visitor
                    .visit_enum(VariantAccess::new(self)));
                }
                match tri!(self.parse_whitespace()) {
                    Some(b'}') => {
                        self.eat_char();
                        Ok(value)
                    }
                    Some(_) => Err(self.error(ErrorCode::ExpectedSomeValue)),
                    None => Err(self.error(ErrorCode::EofWhileParsingObject)),
                }
            }
            Some(b'"') => visitor.visit_enum(UnitVariantAccess::new(self)),
            Some(_) => Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
        }
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        tri!(self.ignore_value());
        visitor.visit_unit()
    }
}
struct SeqAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
impl<'a, R: 'a> SeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        SeqAccess { de, first: true }
    }
}
impl<'de, 'a, R: Read<'de> + 'a> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        let peek = match tri!(self.de.parse_whitespace()) {
            Some(b']') => {
                return Ok(None);
            }
            Some(b',') if !self.first => {
                self.de.eat_char();
                tri!(self.de.parse_whitespace())
            }
            Some(b) => {
                if self.first {
                    self.first = false;
                    Some(b)
                } else {
                    return Err(self.de.peek_error(ErrorCode::ExpectedListCommaOrEnd));
                }
            }
            None => {
                return Err(self.de.peek_error(ErrorCode::EofWhileParsingList));
            }
        };
        match peek {
            Some(b']') => Err(self.de.peek_error(ErrorCode::TrailingComma)),
            Some(_) => Ok(Some(tri!(seed.deserialize(& mut * self.de)))),
            None => Err(self.de.peek_error(ErrorCode::EofWhileParsingValue)),
        }
    }
}
struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
impl<'a, R: 'a> MapAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        MapAccess { de, first: true }
    }
}
impl<'de, 'a, R: Read<'de> + 'a> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        let peek = match tri!(self.de.parse_whitespace()) {
            Some(b'}') => {
                return Ok(None);
            }
            Some(b',') if !self.first => {
                self.de.eat_char();
                tri!(self.de.parse_whitespace())
            }
            Some(b) => {
                if self.first {
                    self.first = false;
                    Some(b)
                } else {
                    return Err(self.de.peek_error(ErrorCode::ExpectedObjectCommaOrEnd));
                }
            }
            None => {
                return Err(self.de.peek_error(ErrorCode::EofWhileParsingObject));
            }
        };
        match peek {
            Some(b'"') => seed.deserialize(MapKey { de: &mut *self.de }).map(Some),
            Some(b'}') => Err(self.de.peek_error(ErrorCode::TrailingComma)),
            Some(_) => Err(self.de.peek_error(ErrorCode::KeyMustBeAString)),
            None => Err(self.de.peek_error(ErrorCode::EofWhileParsingValue)),
        }
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        tri!(self.de.parse_object_colon());
        seed.deserialize(&mut *self.de)
    }
}
struct VariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
impl<'a, R: 'a> VariantAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        VariantAccess { de }
    }
}
impl<'de, 'a, R: Read<'de> + 'a> de::EnumAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = tri!(seed.deserialize(& mut * self.de));
        tri!(self.de.parse_object_colon());
        Ok((val, self))
    }
}
impl<'de, 'a, R: Read<'de> + 'a> de::VariantAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        de::Deserialize::deserialize(self.de)
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}
struct UnitVariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
impl<'a, R: 'a> UnitVariantAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        UnitVariantAccess { de }
    }
}
impl<'de, 'a, R: Read<'de> + 'a> de::EnumAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = tri!(seed.deserialize(& mut * self.de));
        Ok((variant, self))
    }
}
impl<'de, 'a, R: Read<'de> + 'a> de::VariantAccess<'de> for UnitVariantAccess<'a, R> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        Ok(())
    }
    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
    }
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
    }
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant"))
    }
}
/// Only deserialize from this after peeking a '"' byte! Otherwise it may
/// deserialize invalid JSON successfully.
struct MapKey<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
macro_rules! deserialize_integer_key {
    ($method:ident => $visit:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value > where V : de::Visitor
        <'de >, { self.de.eat_char(); self.de.scratch.clear(); let string = tri!(self.de
        .read.parse_str(& mut self.de.scratch)); match (string.parse(), string) {
        (Ok(integer), _) => visitor.$visit (integer), (Err(_), Reference::Borrowed(s)) =>
        visitor.visit_borrowed_str(s), (Err(_), Reference::Copied(s)) => visitor
        .visit_str(s), } }
    };
}
impl<'de, 'a, R> de::Deserializer<'de> for MapKey<'a, R>
where
    R: Read<'de>,
{
    type Error = Error;
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.eat_char();
        self.de.scratch.clear();
        match tri!(self.de.read.parse_str(& mut self.de.scratch)) {
            Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
            Reference::Copied(s) => visitor.visit_str(s),
        }
    }
    deserialize_integer_key!(deserialize_i8 => visit_i8);
    deserialize_integer_key!(deserialize_i16 => visit_i16);
    deserialize_integer_key!(deserialize_i32 => visit_i32);
    deserialize_integer_key!(deserialize_i64 => visit_i64);
    deserialize_integer_key!(deserialize_i128 => visit_i128);
    deserialize_integer_key!(deserialize_u8 => visit_u8);
    deserialize_integer_key!(deserialize_u16 => visit_u16);
    deserialize_integer_key!(deserialize_u32 => visit_u32);
    deserialize_integer_key!(deserialize_u64 => visit_u64);
    deserialize_integer_key!(deserialize_u128 => visit_u128);
    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }
    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        #[cfg(feature = "raw_value")]
        {
            if name == crate::raw::TOKEN {
                return self.de.deserialize_raw_value(visitor);
            }
        }
        let _ = name;
        visitor.visit_newtype_struct(self)
    }
    #[inline]
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_enum(name, variants, visitor)
    }
    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_bytes(visitor)
    }
    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_bytes(visitor)
    }
    forward_to_deserialize_any! {
        bool f32 f64 char str string unit unit_struct seq tuple tuple_struct map struct
        identifier ignored_any
    }
}
/// Iterator that deserializes a stream into multiple JSON values.
///
/// A stream deserializer can be created from any JSON deserializer using the
/// `Deserializer::into_iter` method.
///
/// The data can consist of any JSON value. Values need to be a self-delineating value e.g.
/// arrays, objects, or strings, or be followed by whitespace or a self-delineating value.
///
/// ```
/// use serde_json::{Deserializer, Value};
///
/// fn main() {
///     let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";
///
///     let stream = Deserializer::from_str(data).into_iter::<Value>();
///
///     for value in stream {
///         println!("{}", value.unwrap());
///     }
/// }
/// ```
pub struct StreamDeserializer<'de, R, T> {
    de: Deserializer<R>,
    offset: usize,
    failed: bool,
    output: PhantomData<T>,
    lifetime: PhantomData<&'de ()>,
}
impl<'de, R, T> StreamDeserializer<'de, R, T>
where
    R: read::Read<'de>,
    T: de::Deserialize<'de>,
{
    /// Create a JSON stream deserializer from one of the possible serde_json
    /// input sources.
    ///
    /// Typically it is more convenient to use one of these methods instead:
    ///
    ///   - Deserializer::from_str(...).into_iter()
    ///   - Deserializer::from_slice(...).into_iter()
    ///   - Deserializer::from_reader(...).into_iter()
    pub fn new(read: R) -> Self {
        let offset = read.byte_offset();
        StreamDeserializer {
            de: Deserializer::new(read),
            offset,
            failed: false,
            output: PhantomData,
            lifetime: PhantomData,
        }
    }
    /// Returns the number of bytes so far deserialized into a successful `T`.
    ///
    /// If a stream deserializer returns an EOF error, new data can be joined to
    /// `old_data[stream.byte_offset()..]` to try again.
    ///
    /// ```
    /// let data = b"[0] [1] [";
    ///
    /// let de = serde_json::Deserializer::from_slice(data);
    /// let mut stream = de.into_iter::<Vec<i32>>();
    /// assert_eq!(0, stream.byte_offset());
    ///
    /// println!("{:?}", stream.next()); // [0]
    /// assert_eq!(3, stream.byte_offset());
    ///
    /// println!("{:?}", stream.next()); // [1]
    /// assert_eq!(7, stream.byte_offset());
    ///
    /// println!("{:?}", stream.next()); // error
    /// assert_eq!(8, stream.byte_offset());
    ///
    /// // If err.is_eof(), can join the remaining data to new data and continue.
    /// let remaining = &data[stream.byte_offset()..];
    /// ```
    ///
    /// *Note:* In the future this method may be changed to return the number of
    /// bytes so far deserialized into a successful T *or* syntactically valid
    /// JSON skipped over due to a type error. See [serde-rs/json#70] for an
    /// example illustrating this.
    ///
    /// [serde-rs/json#70]: https://github.com/serde-rs/json/issues/70
    pub fn byte_offset(&self) -> usize {
        self.offset
    }
    fn peek_end_of_value(&mut self) -> Result<()> {
        match tri!(self.de.peek()) {
            Some(b' ')
            | Some(b'\n')
            | Some(b'\t')
            | Some(b'\r')
            | Some(b'"')
            | Some(b'[')
            | Some(b']')
            | Some(b'{')
            | Some(b'}')
            | Some(b',')
            | Some(b':')
            | None => Ok(()),
            Some(_) => {
                let position = self.de.read.peek_position();
                Err(
                    Error::syntax(
                        ErrorCode::TrailingCharacters,
                        position.line,
                        position.column,
                    ),
                )
            }
        }
    }
}
impl<'de, R, T> Iterator for StreamDeserializer<'de, R, T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    type Item = Result<T>;
    fn next(&mut self) -> Option<Result<T>> {
        if R::should_early_return_if_failed && self.failed {
            return None;
        }
        match self.de.parse_whitespace() {
            Ok(None) => {
                self.offset = self.de.read.byte_offset();
                None
            }
            Ok(Some(b)) => {
                let self_delineated_value = match b {
                    b'[' | b'"' | b'{' => true,
                    _ => false,
                };
                self.offset = self.de.read.byte_offset();
                let result = de::Deserialize::deserialize(&mut self.de);
                Some(
                    match result {
                        Ok(value) => {
                            self.offset = self.de.read.byte_offset();
                            if self_delineated_value {
                                Ok(value)
                            } else {
                                self.peek_end_of_value().map(|_| value)
                            }
                        }
                        Err(e) => {
                            self.de.read.set_failed(&mut self.failed);
                            Err(e)
                        }
                    },
                )
            }
            Err(e) => {
                self.de.read.set_failed(&mut self.failed);
                Some(Err(e))
            }
        }
    }
}
impl<'de, R, T> FusedIterator for StreamDeserializer<'de, R, T>
where
    R: Read<'de> + Fused,
    T: de::Deserialize<'de>,
{}
fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = tri!(de::Deserialize::deserialize(& mut de));
    tri!(de.end());
    Ok(value)
}
/// Deserialize an instance of type `T` from an IO stream of JSON.
///
/// The content of the IO stream is deserialized directly from the stream
/// without being buffered in memory by serde_json.
///
/// When reading from a source against which short reads are not efficient, such
/// as a [`File`], you will want to apply your own buffering because serde_json
/// will not buffer the input. See [`std::io::BufReader`].
///
/// It is expected that the input stream ends after the deserialized object.
/// If the stream does not end, such as in the case of a persistent socket connection,
/// this function will not return. It is possible instead to deserialize from a prefix of an input
/// stream without looking for EOF by managing your own [`Deserializer`].
///
/// Note that counter to intuition, this function is usually slower than
/// reading a file completely into memory and then applying [`from_str`]
/// or [`from_slice`] on it. See [issue #160].
///
/// [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
/// [`std::io::BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
/// [`from_str`]: ./fn.from_str.html
/// [`from_slice`]: ./fn.from_slice.html
/// [issue #160]: https://github.com/serde-rs/json/issues/160
///
/// # Example
///
/// Reading the contents of a file.
///
/// ```
/// use serde::Deserialize;
///
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::BufReader;
/// use std::path::Path;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
///     // Open the file in read-only mode with buffer.
///     let file = File::open(path)?;
///     let reader = BufReader::new(file);
///
///     // Read the JSON contents of the file as an instance of `User`.
///     let u = serde_json::from_reader(reader)?;
///
///     // Return the `User`.
///     Ok(u)
/// }
///
/// fn main() {
/// # }
/// # fn fake_main() {
///     let u = read_user_from_file("test.json").unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// Reading from a persistent socket connection.
///
/// ```
/// use serde::Deserialize;
///
/// use std::error::Error;
/// use std::net::{TcpListener, TcpStream};
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn read_user_from_stream(tcp_stream: TcpStream) -> Result<User, Box<dyn Error>> {
///     let mut de = serde_json::Deserializer::from_reader(tcp_stream);
///     let u = User::deserialize(&mut de)?;
///
///     Ok(u)
/// }
///
/// fn main() {
/// # }
/// # fn fake_main() {
///     let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
///
///     for stream in listener.incoming() {
///         println!("{:#?}", read_user_from_stream(stream.unwrap()));
///     }
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned,
{
    from_trait(read::IoRead::new(rdr))
}
/// Deserialize an instance of type `T` from bytes of JSON text.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `&[u8]`
///     let j = b"
///         {
///             \"fingerprint\": \"0xF9BA143B95FF6D82\",
///             \"location\": \"Menlo Park, CA\"
///         }";
///
///     let u: User = serde_json::from_slice(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    from_trait(read::SliceRead::new(v))
}
/// Deserialize an instance of type `T` from a string of JSON text.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `&str`
///     let j = "
///         {
///             \"fingerprint\": \"0xF9BA143B95FF6D82\",
///             \"location\": \"Menlo Park, CA\"
///         }";
///
///     let u: User = serde_json::from_str(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    from_trait(read::StrRead::new(s))
}
#[cfg(test)]
mod tests_llm_16_16_llm_16_16 {
    use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, Visitor};
    use crate::{
        Deserializer as JsonDeserializer, Error as JsonError, Number, Value, Map,
    };
    use std::fmt;
    use std::str::FromStr;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid JSON value")
        }
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Bool(v))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Number(v.into()))
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Number(v.into()))
        }
        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Number(Number::from_f64(v).unwrap()))
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::String(v.to_owned()))
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Null)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Deserialize::deserialize(deserializer)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Null)
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = seq.next_element()? {
                vec.push(elem);
            }
            Ok(Value::Array(vec))
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut m = Map::new();
            while let Some((key, value)) = map.next_entry()? {
                m.insert(key, value);
            }
            Ok(Value::Object(m))
        }
    }
    impl<'de> DeserializeSeed<'de> for TestVisitor {
        type Value = Value;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
    }
    fn deserialize_json<'de, D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TestVisitor)
    }
    #[test]
    fn test_deserialize_any_bool() {
        let s = "true";
        let mut deserializer = JsonDeserializer::from_str(s);
        let value: Value = deserialize_json(&mut deserializer).unwrap();
        assert_eq!(value, Value::Bool(true));
    }
    #[test]
    fn test_deserialize_any_null() {
        let s = "null";
        let mut deserializer = JsonDeserializer::from_str(s);
        let value: Value = deserialize_json(&mut deserializer).unwrap();
        assert_eq!(value, Value::Null);
    }
    #[test]
    fn test_deserialize_any_number() {
        let s = "123";
        let mut deserializer = JsonDeserializer::from_str(s);
        let value: Value = deserialize_json(&mut deserializer).unwrap();
        assert_eq!(value, Value::Number(123.into()));
    }
    #[test]
    fn test_deserialize_any_string() {
        let s = "\"hello\"";
        let mut deserializer = JsonDeserializer::from_str(s);
        let value: Value = deserialize_json(&mut deserializer).unwrap();
        assert_eq!(value, Value::String("hello".to_owned()));
    }
    #[test]
    fn test_deserialize_any_array() {
        let s = "[1, true, null]";
        let mut deserializer = JsonDeserializer::from_str(s);
        let value: Value = deserialize_json(&mut deserializer).unwrap();
        assert_eq!(
            value, Value::Array(vec![Value::Number(1.into()), Value::Bool(true),
            Value::Null])
        );
    }
    #[test]
    fn test_deserialize_any_object() {
        let s = "{\"a\":1, \"b\":null}";
        let mut deserializer = JsonDeserializer::from_str(s);
        let value: Value = deserialize_json(&mut deserializer).unwrap();
        let mut m = Map::new();
        m.insert("a".to_owned(), Value::Number(1.into()));
        m.insert("b".to_owned(), Value::Null);
        assert_eq!(value, Value::Object(m));
    }
    #[test]
    fn test_deserialize_any_error() {
        let s = "invalid_json";
        let mut deserializer = JsonDeserializer::from_str(s);
        let result: Result<Value, JsonError> = deserialize_json(&mut deserializer);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_19 {
    use crate::de::Deserializer;
    use crate::error::Error;
    use serde::de::Deserialize;
    use serde_bytes::ByteBuf;
    #[test]
    fn test_deserialize_bytes() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_bytes = 0;
        let rug_fuzz_0 = b"\"some bytes: \xe5\x00\xe5\"";
        let rug_fuzz_1 = b"some bytes: \xe5\x00\xe5";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let bytes: ByteBuf = Deserialize::deserialize(&mut deserializer).unwrap();
        debug_assert_eq!(rug_fuzz_1, bytes.as_ref());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_bytes = 0;
    }
    #[test]
    fn test_deserialize_lone_surrogate() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_lone_surrogate = 0;
        let rug_fuzz_0 = b"\"lone surrogate: \\uD801\"";
        let rug_fuzz_1 = b"lone surrogate: \xED\xA0\x81";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let bytes: ByteBuf = Deserialize::deserialize(&mut deserializer).unwrap();
        let expected = rug_fuzz_1;
        debug_assert_eq!(expected, bytes.as_ref());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_lone_surrogate = 0;
    }
    #[test]
    fn test_deserialize_invalid_utf8() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_invalid_utf8 = 0;
        let rug_fuzz_0 = b"\"invalid: \xFF\xFE\xFD\"";
        let rug_fuzz_1 = b"invalid: \xFF\xFE\xFD";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let bytes: ByteBuf = Deserialize::deserialize(&mut deserializer).unwrap();
        debug_assert_eq!(rug_fuzz_1, bytes.as_ref());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_invalid_utf8 = 0;
    }
    #[test]
    fn test_deserialize_empty() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_empty = 0;
        let rug_fuzz_0 = b"\"\"";
        let rug_fuzz_1 = b"";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let bytes: ByteBuf = Deserialize::deserialize(&mut deserializer).unwrap();
        debug_assert_eq!(rug_fuzz_1, bytes.as_ref());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_empty = 0;
    }
    #[test]
    fn test_deserialize_invalid_escape() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_invalid_escape = 0;
        let rug_fuzz_0 = b"\"invalid: \\x\"";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let result: Result<ByteBuf, Error> = Deserialize::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_invalid_escape = 0;
    }
    #[test]
    fn test_deserialize_incomplete_escape() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_incomplete_escape = 0;
        let rug_fuzz_0 = b"\"incomplete: \\u1\"";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let result: Result<ByteBuf, Error> = Deserialize::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_incomplete_escape = 0;
    }
    #[test]
    fn test_deserialize_invalid_json() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_invalid_json = 0;
        let rug_fuzz_0 = b"invalid_json";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let result: Result<ByteBuf, Error> = Deserialize::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_invalid_json = 0;
    }
    #[test]
    fn test_deserialize_incomplete_json() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_deserialize_incomplete_json = 0;
        let rug_fuzz_0 = b"\"incomplete";
        let json_data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_slice(json_data);
        let result: Result<ByteBuf, Error> = Deserialize::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_deserialize_incomplete_json = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::{Deserializer as JsonDeserializer, Error};
    use std::fmt;
    use std::str::FromStr;
    struct MockVisitor;
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = f32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float")
        }
        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    fn test_deserialize_f32_helper(input: &str, expected: f32) -> Result<f32, Error> {
        let mut de = JsonDeserializer::from_str(input);
        let visitor = MockVisitor;
        de.deserialize_f32(visitor)
    }
    #[test]
    fn test_deserialize_f32() {
        let tests = vec![
            ("0.0", 0.0f32), ("3.14", 3.14f32), ("-1.23", - 1.23f32), ("5e2", 500.0f32),
            ("1e-3", 0.001f32),
        ];
        for (input, expected) in tests {
            assert_eq!(test_deserialize_f32_helper(input, expected).unwrap(), expected);
        }
    }
    #[test]
    #[should_panic(
        expected = "Error(\"invalid type: string \\\"not a float\\\", expected a float\", line: 1, column: 13)"
    )]
    fn test_deserialize_f32_invalid() {
        let input = "\"not a float\"";
        test_deserialize_f32_helper(input, 0.0f32).unwrap();
    }
    #[test]
    #[should_panic(expected = "expected value")]
    fn test_deserialize_f32_empty_input() {
        let input = "";
        test_deserialize_f32_helper(input, 0.0f32).unwrap();
    }
}
#[cfg(test)]
mod tests_llm_16_26_llm_16_26 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::Deserializer as JsonDeserializer;
    use crate::Error;
    use crate::de::Read;
    use std::fmt;
    use std::str::FromStr;
    struct I32Visitor;
    impl<'de> de::Visitor<'de> for I32Visitor {
        type Value = i32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an i32")
        }
        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    fn deserialize_i32<'de, R>(
        deserializer: &mut JsonDeserializer<R>,
    ) -> Result<i32, Error>
    where
        R: Read<'de>,
    {
        deserializer.deserialize_number(I32Visitor)
    }
    #[test]
    fn test_deserialize_i32() {
        let test_cases = [
            ("123", 123),
            ("-123", -123),
            ("0", 0),
            ("2147483647", 2147483647),
            ("-2147483648", -2147483648),
        ];
        for &(json, expected) in &test_cases {
            let mut de = JsonDeserializer::from_str(json);
            let result = deserialize_i32(&mut de).expect("failed to parse i32");
            assert_eq!(result, expected);
        }
        let error_cases = ["", "abc", "2147483648", "-2147483649", "null", "[]", "{}"];
        for &json in &error_cases {
            let mut de = JsonDeserializer::from_str(json);
            assert!(deserialize_i32(& mut de).is_err());
        }
    }
}
#[cfg(test)]
mod tests_llm_16_30_llm_16_30 {
    use serde::de::{
        self, DeserializeOwned, Deserialize, Deserializer as SerdeDeserializer, Visitor,
    };
    use crate::{Deserializer, Map, Value};
    use crate::error::Error;
    use crate::de::read::SliceRead;
    use std::fmt;
    use std::string::String;
    struct IgnoredAny;
    impl<'de> Visitor<'de> for IgnoredAny {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any value")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }
    fn deserialize_ignored_any<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: SerdeDeserializer<'de>,
    {
        deserializer.deserialize_ignored_any(IgnoredAny)
    }
    fn test_deserialize_ignored_any_helper<T>(json: &'static str) -> Result<(), Error>
    where
        T: DeserializeOwned,
    {
        let mut deserializer = Deserializer::from_slice(json.as_bytes());
        let _: T = Deserialize::deserialize(&mut deserializer)?;
        deserialize_ignored_any(&mut deserializer)?;
        deserializer.end()
    }
    #[test]
    fn test_deserialize_ignored_any() {
        let json = r#"[1, 2, 3, 4, 5]"#;
        test_deserialize_ignored_any_helper::<Vec<u64>>(json).unwrap();
    }
    #[test]
    fn test_deserialize_ignored_any_object() {
        let json = r#"{"a": 1, "b": 2, "c": 3}"#;
        test_deserialize_ignored_any_helper::<Map<String, Value>>(json).unwrap();
    }
    #[test]
    fn test_deserialize_ignored_any_empty_object() {
        let json = r#"{}"#;
        test_deserialize_ignored_any_helper::<Map<String, Value>>(json).unwrap();
    }
    #[test]
    fn test_deserialize_ignored_any_string() {
        let json = r#""a string""#;
        test_deserialize_ignored_any_helper::<String>(json).unwrap();
    }
}
#[cfg(test)]
mod tests_llm_16_33 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::{Map, Value, Error};
    use std::fmt;
    use std::str::FromStr;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Map<String, Value>>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an option")
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Deserialize::deserialize(deserializer).map(Some)
        }
    }
    #[test]
    fn test_deserialize_option_none() {
        let _rug_st_tests_llm_16_33_rrrruuuugggg_test_deserialize_option_none = 0;
        let rug_fuzz_0 = "null";
        let json_str = rug_fuzz_0;
        let mut deserializer = crate::Deserializer::from_str(json_str);
        let visitor = TestVisitor;
        let result: Result<Option<Map<String, Value>>, Error> = deserializer
            .deserialize_option(visitor);
        debug_assert_eq!(result.unwrap(), None);
        let _rug_ed_tests_llm_16_33_rrrruuuugggg_test_deserialize_option_none = 0;
    }
    #[test]
    fn test_deserialize_option_some() {
        let _rug_st_tests_llm_16_33_rrrruuuugggg_test_deserialize_option_some = 0;
        let rug_fuzz_0 = r#"{"key": "value"}"#;
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "value";
        let json_str = rug_fuzz_0;
        let mut deserializer = crate::Deserializer::from_str(json_str);
        let visitor = TestVisitor;
        let result: Result<Option<Map<String, Value>>, Error> = deserializer
            .deserialize_option(visitor);
        let mut expected_map = Map::new();
        expected_map
            .insert(rug_fuzz_1.to_string(), Value::String(rug_fuzz_2.to_string()));
        debug_assert_eq!(result.unwrap(), Some(expected_map));
        let _rug_ed_tests_llm_16_33_rrrruuuugggg_test_deserialize_option_some = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_36_llm_16_36 {
    use serde::de::{self, Deserializer, Visitor, MapAccess};
    use crate::error::Error;
    use crate::map::Map;
    use crate::value::Value;
    use crate::de::Deserializer as JsonDeserializer;
    use std::fmt;
    use std::str::FromStr;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut map = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                map.insert(key, value);
            }
            Ok(map)
        }
    }
    fn test_deserialize_string(input: &str, expected: Map<String, Value>) {
        let _rug_st_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string = 0;
        let mut de = JsonDeserializer::from_str(input);
        let visitor = TestVisitor;
        let result: Result<Map<String, Value>, Error> = de.deserialize_string(visitor);
        debug_assert_eq!(result.unwrap(), expected);
        let _rug_ed_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string = 0;
    }
    #[test]
    fn test_deserialize_string_valid() {
        let _rug_st_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string_valid = 0;
        let rug_fuzz_0 = r#"{"key1": "value1", "key2": "value2"}"#;
        let rug_fuzz_1 = "key1";
        let rug_fuzz_2 = "value1";
        let rug_fuzz_3 = "key2";
        let rug_fuzz_4 = "value2";
        let json = rug_fuzz_0;
        let mut expected = Map::new();
        expected.insert(rug_fuzz_1.to_owned(), Value::from_str(rug_fuzz_2).unwrap());
        expected.insert(rug_fuzz_3.to_owned(), Value::from_str(rug_fuzz_4).unwrap());
        test_deserialize_string(json, expected);
        let _rug_ed_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string_valid = 0;
    }
    #[test]
    fn test_deserialize_string_empty() {
        let _rug_st_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string_empty = 0;
        let rug_fuzz_0 = r#"{}"#;
        let json = rug_fuzz_0;
        let expected = Map::new();
        test_deserialize_string(json, expected);
        let _rug_ed_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string_empty = 0;
    }
    #[test]
    #[should_panic(expected = "Error(\"expected value\", line: 1, column: 1)")]
    fn test_deserialize_string_invalid() {
        let _rug_st_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string_invalid = 0;
        let rug_fuzz_0 = r#"not a json string"#;
        let json = rug_fuzz_0;
        let expected = Map::new();
        test_deserialize_string(json, expected);
        let _rug_ed_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_deserialize_string_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_43_llm_16_43 {
    use serde::de::{self, Deserialize};
    use crate::de::{Deserializer, Error};
    use crate::value::{self, Value};
    #[test]
    fn test_deserialize_u64_from_u64() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_u64 = 0;
        let rug_fuzz_0 = "42";
        let mut deserializer = Deserializer::from_str(rug_fuzz_0);
        let value = u64::deserialize(&mut deserializer).unwrap();
        debug_assert_eq!(value, 42);
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_u64 = 0;
    }
    #[test]
    fn test_deserialize_u64_from_string() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_string = 0;
        let rug_fuzz_0 = "\"42\"";
        let mut deserializer = Deserializer::from_str(rug_fuzz_0);
        let result = u64::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_string = 0;
    }
    #[test]
    fn test_deserialize_u64_from_negative_number() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_negative_number = 0;
        let rug_fuzz_0 = "-42";
        let mut deserializer = Deserializer::from_str(rug_fuzz_0);
        let result = u64::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_negative_number = 0;
    }
    #[test]
    fn test_deserialize_u64_from_floating_point() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_floating_point = 0;
        let rug_fuzz_0 = "42.0";
        let mut deserializer = Deserializer::from_str(rug_fuzz_0);
        let result = u64::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_floating_point = 0;
    }
    #[test]
    fn test_deserialize_u64_from_invalid_json() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_invalid_json = 0;
        let rug_fuzz_0 = "invalid";
        let mut deserializer = Deserializer::from_str(rug_fuzz_0);
        let result = u64::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_invalid_json = 0;
    }
    #[test]
    fn test_deserialize_u64_from_out_of_range_number() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_out_of_range_number = 0;
        let rug_fuzz_0 = "18446744073709551616";
        let mut deserializer = Deserializer::from_str(rug_fuzz_0);
        let result = u64::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_deserialize_u64_from_out_of_range_number = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44_llm_16_44 {
    use serde::Deserialize;
    use crate::{Deserializer, Error};
    fn deserialize_u8_from_str(input: &str) -> Result<u8, Error> {
        let mut deserializer = Deserializer::from_str(input);
        let u = u8::deserialize(&mut deserializer)?;
        Ok(u)
    }
    #[test]
    fn test_deserialize_u8_valid() {
        let json = "42";
        let u = deserialize_u8_from_str(json).unwrap();
        assert_eq!(u, 42);
    }
    #[test]
    fn test_deserialize_u8_too_large() {
        let json = "256";
        let result = deserialize_u8_from_str(json);
        assert!(result.is_err());
    }
    #[test]
    fn test_deserialize_u8_negative() {
        let json = "-1";
        let result = deserialize_u8_from_str(json);
        assert!(result.is_err());
    }
    #[test]
    fn test_deserialize_u8_non_number() {
        let json = "\"not a number\"";
        let result = deserialize_u8_from_str(json);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_107 {
    use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, Visitor};
    use crate::de::{Deserializer as JsonDeserializer, MapKey};
    use crate::error::Error;
    use std::fmt;
    use std::marker::PhantomData;
    struct EnumVisitor;
    impl<'de> Visitor<'de> for EnumVisitor {
        type Value = String;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an enum")
        }
        fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where
            A: de::EnumAccess<'de>,
        {
            let (variant, _) = data.variant::<String>()?;
            Ok(variant)
        }
    }
    struct EnumSeed;
    impl<'de> DeserializeSeed<'de> for EnumSeed {
        type Value = String;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_enum("TestEnum", &["A", "B", "C"], EnumVisitor)
        }
    }
    #[test]
    fn test_deserialize_enum() {
        let json_str = r#""A""#;
        let mut deserializer = JsonDeserializer::from_str(json_str);
        let map_key_deserializer = MapKey { de: &mut deserializer };
        let result: Result<String, Error> = map_key_deserializer
            .deserialize_enum("TestEnum", &["A", "B", "C"], EnumVisitor);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "A");
    }
}
#[cfg(test)]
mod tests_llm_16_108 {
    use serde::de::Visitor;
    use serde::de::Deserializer;
    use serde::Deserializer as SerdeDeserializer;
    use crate::Deserializer as JsonDeserializer;
    use crate::de::MapKey;
    use crate::error::Error;
    use crate::value::{Map, Value};
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i128;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i128")
        }
        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse::<i128>().map_err(serde::de::Error::custom)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse::<i128>().map_err(serde::de::Error::custom)
        }
    }
    #[test]
    fn deserialize_i128_from_string_key() {
        let _rug_st_tests_llm_16_108_rrrruuuugggg_deserialize_i128_from_string_key = 0;
        let rug_fuzz_0 = r#""-9223372036854775808""#;
        let json_str = rug_fuzz_0;
        let mut de = JsonDeserializer::from_str(json_str);
        let map_key = MapKey { de: &mut de };
        let result: Result<i128, Error> = map_key.deserialize_i128(TestVisitor);
        debug_assert_eq!(result.unwrap(), - 9223372036854775808i128);
        let _rug_ed_tests_llm_16_108_rrrruuuugggg_deserialize_i128_from_string_key = 0;
    }
    #[test]
    fn deserialize_i128_from_numeric_key() {
        let _rug_st_tests_llm_16_108_rrrruuuugggg_deserialize_i128_from_numeric_key = 0;
        let rug_fuzz_0 = r#""42""#;
        let json_str = rug_fuzz_0;
        let mut de = JsonDeserializer::from_str(json_str);
        let map_key = MapKey { de: &mut de };
        let result: Result<i128, Error> = map_key.deserialize_i128(TestVisitor);
        debug_assert_eq!(result.unwrap(), 42i128);
        let _rug_ed_tests_llm_16_108_rrrruuuugggg_deserialize_i128_from_numeric_key = 0;
    }
    #[test]
    fn deserialize_i128_from_invalid_key() {
        let _rug_st_tests_llm_16_108_rrrruuuugggg_deserialize_i128_from_invalid_key = 0;
        let rug_fuzz_0 = r#"not-a-number"#;
        let json_str = rug_fuzz_0;
        let mut de = JsonDeserializer::from_str(json_str);
        let map_key = MapKey { de: &mut de };
        let result: Result<i128, Error> = map_key.deserialize_i128(TestVisitor);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_108_rrrruuuugggg_deserialize_i128_from_invalid_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_110 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::Deserializer as JsonDeserializer;
    use crate::{Map, Value};
    use std::fmt;
    use std::str::FromStr;
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an i32 integer")
        }
        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
    fn deserialize_i32_from_str<'de, T: de::Deserializer<'de>>(
        deserializer: T,
    ) -> Result<i32, T::Error> {
        let visitor = TestVisitor;
        deserializer.deserialize_i32(visitor)
    }
    #[test]
    fn test_deserialize_i32_for_key_as_str() {
        let s = "\"32\"";
        let mut deserializer = JsonDeserializer::from_str(s);
        let map_key_deserializer = super::MapKey {
            de: &mut deserializer,
        };
        let i = deserialize_i32_from_str(map_key_deserializer).unwrap();
        assert_eq!(i, 32);
    }
    #[test]
    fn test_deserialize_i32_for_key_as_str_with_leading_zeros() {
        let s = "\"0032\"";
        let mut deserializer = JsonDeserializer::from_str(s);
        let map_key_deserializer = super::MapKey {
            de: &mut deserializer,
        };
        let i = deserialize_i32_from_str(map_key_deserializer).unwrap();
        assert_eq!(i, 32);
    }
    #[test]
    fn test_deserialize_i32_for_key_as_negative_str() {
        let s = "\"-32\"";
        let mut deserializer = JsonDeserializer::from_str(s);
        let map_key_deserializer = super::MapKey {
            de: &mut deserializer,
        };
        let i = deserialize_i32_from_str(map_key_deserializer).unwrap();
        assert_eq!(i, - 32);
    }
    #[test]
    fn test_deserialize_i32_for_key_as_invalid_str() {
        let s = "\"abc\"";
        let mut deserializer = JsonDeserializer::from_str(s);
        let map_key_deserializer = super::MapKey {
            de: &mut deserializer,
        };
        let result = deserialize_i32_from_str(map_key_deserializer);
        assert!(result.is_err());
    }
    #[test]
    fn test_deserialize_i32_for_key_as_empty_str() {
        let s = "\"\"";
        let mut deserializer = JsonDeserializer::from_str(s);
        let map_key_deserializer = super::MapKey {
            de: &mut deserializer,
        };
        let result = deserialize_i32_from_str(map_key_deserializer);
        assert!(result.is_err());
    }
    #[test]
    fn test_deserialize_i32_for_key_as_invalid_json() {
        let s = "not_a_json";
        let mut deserializer = JsonDeserializer::from_str(s);
        let map_key_deserializer = super::MapKey {
            de: &mut deserializer,
        };
        let result = deserialize_i32_from_str(map_key_deserializer);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_112 {
    use serde::Deserializer;
    use serde::de::{self, Visitor};
    use crate::de::{Deserializer as JsonDeserializer, MapKey};
    use crate::Error as JsonError;
    use std::fmt;
    use std::str::FromStr;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an i8")
        }
        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
    #[test]
    fn test_deserialize_i8() {
        let _rug_st_tests_llm_16_112_rrrruuuugggg_test_deserialize_i8 = 0;
        let rug_fuzz_0 = "\"42\"";
        let rug_fuzz_1 = "\"-42\"";
        let rug_fuzz_2 = "\"not an i8\"";
        let mut de = JsonDeserializer::from_str(rug_fuzz_0);
        let key_de = MapKey { de: &mut de };
        let v = key_de.deserialize_i8(TestVisitor).unwrap();
        debug_assert_eq!(v, 42_i8);
        let mut de = JsonDeserializer::from_str(rug_fuzz_1);
        let key_de = MapKey { de: &mut de };
        let v = key_de.deserialize_i8(TestVisitor).unwrap();
        debug_assert_eq!(v, - 42_i8);
        let mut de = JsonDeserializer::from_str(rug_fuzz_2);
        let key_de = MapKey { de: &mut de };
        let res = key_de.deserialize_i8(TestVisitor);
        debug_assert!(res.is_err());
        debug_assert_eq!(
            res.unwrap_err().to_string(),
            "invalid type: string \"not an i8\", expected an i8"
        );
        let _rug_ed_tests_llm_16_112_rrrruuuugggg_test_deserialize_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_114 {
    use serde::{Deserialize, Deserializer};
    use crate::de::{Deserializer as JsonDeserializer, MapKey};
    use crate::{Error, Value};
    use std::fmt;
    struct TestVisitor;
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a JSON value")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::Null)
        }
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::Bool(v))
        }
        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::Number(v.into()))
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Deserialize::deserialize(deserializer)
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::Null)
        }
    }
    #[test]
    fn test_deserialize_option() -> Result<(), Error> {
        let json_str = r#""test_key""#;
        let mut json_deserializer = JsonDeserializer::from_str(json_str);
        let map_key_deserializer = MapKey {
            de: &mut json_deserializer,
        };
        let visitor = TestVisitor;
        let value = map_key_deserializer.deserialize_option(visitor)?;
        if let Value::String(s) = value {
            assert_eq!(s, "test_key");
        } else {
            panic!("Expected a Value::String variant");
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_121_llm_16_121 {
    use crate::{Deserializer, StreamDeserializer, Result, de::{Read, IoRead}};
    use std::{iter::FusedIterator, io::Cursor};
    use serde::de::DeserializeOwned;
    #[test]
    fn test_next_all_valid_json_elements() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_next_all_valid_json_elements = 0;
        let rug_fuzz_0 = r#"{"key": "value"} 123 "string" [1, 2, 3]"#;
        let data = rug_fuzz_0;
        let read = IoRead::new(Cursor::new(data.as_bytes()));
        let mut stream_deserializer = StreamDeserializer::<_, crate::Value>::new(read);
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(stream_deserializer.next().is_none());
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_next_all_valid_json_elements = 0;
    }
    #[test]
    fn test_next_invalid_json() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_next_invalid_json = 0;
        let rug_fuzz_0 = r#"{"key": "value", } 123"#;
        let data = rug_fuzz_0;
        let read = IoRead::new(Cursor::new(data.as_bytes()));
        let mut stream_deserializer = StreamDeserializer::<_, crate::Value>::new(read);
        debug_assert!(matches!(stream_deserializer.next(), Some(Err(_))));
        debug_assert!(stream_deserializer.next().is_none());
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_next_invalid_json = 0;
    }
    #[test]
    fn test_next_valid_followed_by_invalid_json() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_next_valid_followed_by_invalid_json = 0;
        let rug_fuzz_0 = r#"{"key": "value"} 123 "string" [1, 2,]"#;
        let data = rug_fuzz_0;
        let read = IoRead::new(Cursor::new(data.as_bytes()));
        let mut stream_deserializer = StreamDeserializer::<_, crate::Value>::new(read);
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(stream_deserializer.next().is_some());
        debug_assert!(matches!(stream_deserializer.next(), Some(Err(_))));
        debug_assert!(stream_deserializer.next().is_none());
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_next_valid_followed_by_invalid_json = 0;
    }
    #[test]
    fn test_byte_offset_tracking() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_byte_offset_tracking = 0;
        let rug_fuzz_0 = r#"{"key": "value"} 123 "string" [1, 2, 3]"#;
        let data = rug_fuzz_0;
        let read = IoRead::new(Cursor::new(data.as_bytes()));
        let mut stream_deserializer = StreamDeserializer::<_, crate::Value>::new(read);
        stream_deserializer.next().unwrap().unwrap();
        debug_assert_eq!(stream_deserializer.byte_offset(), 17);
        stream_deserializer.next().unwrap().unwrap();
        debug_assert_eq!(stream_deserializer.byte_offset(), 21);
        stream_deserializer.next().unwrap().unwrap();
        debug_assert_eq!(stream_deserializer.byte_offset(), 30);
        stream_deserializer.next().unwrap().unwrap();
        debug_assert_eq!(stream_deserializer.byte_offset(), 41);
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_byte_offset_tracking = 0;
    }
    #[test]
    fn test_fused_iterator() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_fused_iterator = 0;
        let rug_fuzz_0 = r#"{"key": "value"} 123 "string" [1, 2, 3]"#;
        let data = rug_fuzz_0;
        let read = IoRead::new(Cursor::new(data.as_bytes()));
        let mut stream_deserializer = StreamDeserializer::<_, crate::Value>::new(read);
        let mut fused = stream_deserializer.by_ref().fuse();
        while let Some(Ok(_)) = fused.next() {}
        debug_assert!(fused.next().is_none());
        debug_assert!(fused.next().is_none());
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_fused_iterator = 0;
    }
    #[test]
    fn test_early_return_if_failed_behavior() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_early_return_if_failed_behavior = 0;
        let rug_fuzz_0 = r#"{"key": "value", } 123"#;
        let data = rug_fuzz_0;
        let read = IoRead::new(Cursor::new(data.as_bytes()));
        let mut stream_deserializer = StreamDeserializer::<_, crate::Value>::new(read);
        let result1 = stream_deserializer.next();
        debug_assert!(result1.is_some());
        debug_assert!(result1.unwrap().is_err());
        let result2 = stream_deserializer.next();
        debug_assert!(result2.is_none());
        let result3 = stream_deserializer.next();
        debug_assert!(result3.is_none());
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_early_return_if_failed_behavior = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_125_llm_16_125 {
    use serde::de::{self, DeserializeSeed, Visitor, EnumAccess, VariantAccess};
    use crate::de::{Deserializer, UnitVariantAccess};
    use crate::error::{Error, Category};
    use crate::value::{self, Value};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple variant")
        }
        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            Ok(Value::Array(Vec::new()))
        }
    }
    #[test]
    fn test_tuple_variant() {
        let _rug_st_tests_llm_16_125_llm_16_125_rrrruuuugggg_test_tuple_variant = 0;
        let rug_fuzz_0 = r#"{"SomeKey": "SomeValue"}"#;
        let rug_fuzz_1 = 0;
        let data = rug_fuzz_0;
        let mut deserializer = Deserializer::from_str(data);
        let unit_variant_access = UnitVariantAccess::new(&mut deserializer);
        let res: Result<Value, Error> = unit_variant_access
            .tuple_variant(rug_fuzz_1, TestVisitor);
        debug_assert!(res.is_err());
        if let Err(e) = res {
            match e.classify() {
                Category::Data => {
                    debug_assert_eq!(
                        e.to_string(),
                        "invalid type: unit variant, expected tuple variant"
                    );
                }
                _ => panic!("expected data error for tuple_variant"),
            }
        }
        let _rug_ed_tests_llm_16_125_llm_16_125_rrrruuuugggg_test_tuple_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_433 {
    use crate::number::Number;
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid_int() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_valid_int = 0;
        let rug_fuzz_0 = "123";
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        debug_assert_eq!(number, Number::from(123i64));
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_valid_int = 0;
    }
    #[test]
    fn test_from_str_valid_float() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_valid_float = 0;
        let rug_fuzz_0 = "123.456";
        let rug_fuzz_1 = 123.456;
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        let expected = crate::value::Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert_eq!(number, expected);
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_valid_float = 0;
    }
    #[test]
    fn test_from_str_invalid_number() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_invalid_number = 0;
        let rug_fuzz_0 = "abc";
        let s = rug_fuzz_0;
        debug_assert!(Number::from_str(s).is_err());
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_invalid_number = 0;
    }
    #[test]
    fn test_from_str_empty_string() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_empty_string = 0;
        let rug_fuzz_0 = "";
        let s = rug_fuzz_0;
        debug_assert!(Number::from_str(s).is_err());
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_empty_string = 0;
    }
    #[test]
    fn test_from_str_space() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_space = 0;
        let rug_fuzz_0 = " ";
        let s = rug_fuzz_0;
        debug_assert!(Number::from_str(s).is_err());
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_space = 0;
    }
    #[test]
    fn test_from_str_positive_sign() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_positive_sign = 0;
        let rug_fuzz_0 = "+123";
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        debug_assert_eq!(number, Number::from(123i64));
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_positive_sign = 0;
    }
    #[test]
    fn test_from_str_negative_int() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_negative_int = 0;
        let rug_fuzz_0 = "-123";
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        debug_assert_eq!(number, Number::from(- 123i64));
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_negative_int = 0;
    }
    #[test]
    fn test_from_str_zero() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_zero = 0;
        let rug_fuzz_0 = "0";
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        debug_assert_eq!(number, Number::from(0i64));
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_zero = 0;
    }
    #[test]
    fn test_from_str_float_trailing_zeros() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_float_trailing_zeros = 0;
        let rug_fuzz_0 = "123.4500";
        let rug_fuzz_1 = 123.45;
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        let expected = crate::value::Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert_eq!(number, expected);
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_float_trailing_zeros = 0;
    }
    #[test]
    fn test_from_str_float_with_exponent() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_float_with_exponent = 0;
        let rug_fuzz_0 = "123e2";
        let rug_fuzz_1 = 123e2;
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        let expected = crate::value::Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert_eq!(number, expected);
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_float_with_exponent = 0;
    }
    #[test]
    fn test_from_str_negative_float_with_exponent() {
        let _rug_st_tests_llm_16_433_rrrruuuugggg_test_from_str_negative_float_with_exponent = 0;
        let rug_fuzz_0 = "-123.45e-2";
        let rug_fuzz_1 = 123.45e-2;
        let s = rug_fuzz_0;
        let number = Number::from_str(s).unwrap();
        let expected = crate::value::Number::from_f64(-rug_fuzz_1).unwrap();
        debug_assert_eq!(number, expected);
        let _rug_ed_tests_llm_16_433_rrrruuuugggg_test_from_str_negative_float_with_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_439_llm_16_439 {
    use super::*;
    use crate::*;
    use crate::de::{Deserializer, read};
    use crate::error::{Error, ErrorCode};
    #[test]
    fn test_error_syntax() {
        let _rug_st_tests_llm_16_439_llm_16_439_rrrruuuugggg_test_error_syntax = 0;
        let rug_fuzz_0 = r#"{"some":"json"}"#;
        let json_str = rug_fuzz_0;
        let read = read::SliceRead::new(json_str.as_bytes());
        let de = Deserializer::new(read);
        let reason = ErrorCode::ExpectedSomeValue;
        let err = de.error(reason);
        debug_assert_eq!(err.line(), 1);
        debug_assert_eq!(err.column(), 1);
        debug_assert!(matches!(err.classify(), crate ::error::Category::Syntax));
        debug_assert_eq!(format!("{}", err), "expected value at line 1 column 1");
        let _rug_ed_tests_llm_16_439_llm_16_439_rrrruuuugggg_test_error_syntax = 0;
    }
    #[test]
    fn test_error_io() {
        let _rug_st_tests_llm_16_439_llm_16_439_rrrruuuugggg_test_error_io = 0;
        let rug_fuzz_0 = "io error";
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, rug_fuzz_0);
        let err = Error::io(io_error);
        debug_assert_eq!(err.line(), 0);
        debug_assert_eq!(err.column(), 0);
        debug_assert!(matches!(err.classify(), crate ::error::Category::Io));
        debug_assert_eq!(format!("{}", err), "io error at line 0 column 0");
        let _rug_ed_tests_llm_16_439_llm_16_439_rrrruuuugggg_test_error_io = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_441 {
    use super::*;
    use crate::*;
    use crate::de::{Deserializer, Error};
    use crate::error::ErrorCode;
    #[test]
    fn test_fix_position_with_line_info() {
        let _rug_st_tests_llm_16_441_rrrruuuugggg_test_fix_position_with_line_info = 0;
        let rug_fuzz_0 = br#"{}"#;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let json = rug_fuzz_0;
        let mut de = Deserializer::from_slice(json);
        let error_with_info = de.peek_error(ErrorCode::ExpectedSomeValue);
        let error = Error::syntax(ErrorCode::ExpectedSomeValue, rug_fuzz_1, rug_fuzz_2);
        let fixed_error = de.fix_position(error);
        debug_assert_eq!(fixed_error.line(), error_with_info.line());
        debug_assert_eq!(fixed_error.column(), error_with_info.column());
        debug_assert_eq!(fixed_error.is_syntax(), error_with_info.is_syntax());
        let _rug_ed_tests_llm_16_441_rrrruuuugggg_test_fix_position_with_line_info = 0;
    }
    #[test]
    fn test_fix_position_without_line_info() {
        let _rug_st_tests_llm_16_441_rrrruuuugggg_test_fix_position_without_line_info = 0;
        let rug_fuzz_0 = br#"{}"#;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let json = rug_fuzz_0;
        let mut de = Deserializer::from_slice(json);
        let error_without_info = Error::syntax(
            ErrorCode::ExpectedSomeValue,
            rug_fuzz_1,
            rug_fuzz_2,
        );
        let fixed_error = de.fix_position(error_without_info);
        debug_assert_eq!(fixed_error.line(), 1);
        debug_assert_eq!(fixed_error.column(), 1);
        debug_assert!(fixed_error.is_syntax());
        let _rug_ed_tests_llm_16_441_rrrruuuugggg_test_fix_position_without_line_info = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_443 {
    use super::*;
    use crate::*;
    #[test]
    fn test_ignore_exponent_valid_exponent_positive() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_valid_exponent_positive = 0;
        let rug_fuzz_0 = b"2.998e+8";
        let rug_fuzz_1 = "Failed to parse integer part";
        let rug_fuzz_2 = "Failed to parse decimal part";
        let rug_fuzz_3 = "Failed to ignore valid positive exponent";
        let data = rug_fuzz_0;
        let mut de = Deserializer::from_slice(data);
        de.ignore_integer().expect(rug_fuzz_1);
        de.ignore_decimal().expect(rug_fuzz_2);
        de.ignore_exponent().expect(rug_fuzz_3);
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_valid_exponent_positive = 0;
    }
    #[test]
    fn test_ignore_exponent_valid_exponent_negative() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_valid_exponent_negative = 0;
        let rug_fuzz_0 = b"-1.602e-19";
        let rug_fuzz_1 = "Failed to parse integer part";
        let rug_fuzz_2 = "Failed to parse decimal part";
        let rug_fuzz_3 = "Failed to ignore valid negative exponent";
        let data = rug_fuzz_0;
        let mut de = Deserializer::from_slice(data);
        de.ignore_integer().expect(rug_fuzz_1);
        de.ignore_decimal().expect(rug_fuzz_2);
        de.ignore_exponent().expect(rug_fuzz_3);
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_valid_exponent_negative = 0;
    }
    #[test]
    fn test_ignore_exponent_no_exponent_digits() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_no_exponent_digits = 0;
        let rug_fuzz_0 = b"6.022e";
        let rug_fuzz_1 = "Failed to parse integer part";
        let rug_fuzz_2 = "Failed to parse decimal part";
        let data = rug_fuzz_0;
        let mut de = Deserializer::from_slice(data);
        de.ignore_integer().expect(rug_fuzz_1);
        de.ignore_decimal().expect(rug_fuzz_2);
        let result = de.ignore_exponent();
        debug_assert!(result.is_err());
        debug_assert_eq!(
            result.unwrap_err().classify(), crate ::error::Category::Syntax
        );
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_no_exponent_digits = 0;
    }
    #[test]
    fn test_ignore_exponent_invalid_exponent() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_invalid_exponent = 0;
        let rug_fuzz_0 = b"1.23e+-4";
        let rug_fuzz_1 = "Failed to parse integer part";
        let rug_fuzz_2 = "Failed to parse decimal part";
        let data = rug_fuzz_0;
        let mut de = Deserializer::from_slice(data);
        de.ignore_integer().expect(rug_fuzz_1);
        de.ignore_decimal().expect(rug_fuzz_2);
        let result = de.ignore_exponent();
        debug_assert!(result.is_err());
        debug_assert_eq!(
            result.unwrap_err().classify(), crate ::error::Category::Syntax
        );
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_invalid_exponent = 0;
    }
    #[test]
    fn test_ignore_exponent_missing_exponent() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_missing_exponent = 0;
        let rug_fuzz_0 = b"9.109";
        let rug_fuzz_1 = "Failed to parse integer part";
        let rug_fuzz_2 = "Failed to parse decimal part";
        let data = rug_fuzz_0;
        let mut de = Deserializer::from_slice(data);
        de.ignore_integer().expect(rug_fuzz_1);
        de.ignore_decimal().expect(rug_fuzz_2);
        let result = de.ignore_exponent();
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_ignore_exponent_missing_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_447 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_deserializer_from_str() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_new_deserializer_from_str = 0;
        let rug_fuzz_0 = "{\"test\": 1}";
        let json_str = rug_fuzz_0;
        let deserializer = Deserializer::from_str(json_str);
        debug_assert_eq!(deserializer.remaining_depth, 128);
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_new_deserializer_from_str = 0;
    }
    #[test]
    fn test_new_deserializer_from_slice() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_new_deserializer_from_slice = 0;
        let rug_fuzz_0 = b"{\"test\": 1}";
        let json_slice = rug_fuzz_0;
        let deserializer = Deserializer::from_slice(json_slice);
        debug_assert_eq!(deserializer.remaining_depth, 128);
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_new_deserializer_from_slice = 0;
    }
    #[test]
    fn test_new_deserializer_from_reader() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_new_deserializer_from_reader = 0;
        let rug_fuzz_0 = "{\"test\": 1}";
        use std::io::Cursor;
        let json_reader = Cursor::new(rug_fuzz_0);
        let deserializer = Deserializer::from_reader(json_reader);
        debug_assert_eq!(deserializer.remaining_depth, 128);
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_new_deserializer_from_reader = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_449 {
    use super::*;
    use crate::*;
    use crate::error::Result;
    #[test]
    fn test_next_char_or_null_empty() {
        let _rug_st_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_empty = 0;
        let rug_fuzz_0 = b"";
        let mut de = Deserializer::from_slice(rug_fuzz_0);
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_empty = 0;
    }
    #[test]
    fn test_next_char_or_null_non_empty() {
        let _rug_st_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_non_empty = 0;
        let rug_fuzz_0 = b"abc";
        let mut de = Deserializer::from_slice(rug_fuzz_0);
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'a');
        let _rug_ed_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_non_empty = 0;
    }
    #[test]
    fn test_next_char_or_null_null() {
        let _rug_st_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_null = 0;
        let rug_fuzz_0 = b"\x00";
        let mut de = Deserializer::from_slice(rug_fuzz_0);
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_null = 0;
    }
    #[test]
    fn test_next_char_or_null_multiple_chars() {
        let _rug_st_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_multiple_chars = 0;
        let rug_fuzz_0 = b"abc";
        let mut de = Deserializer::from_slice(rug_fuzz_0);
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'a');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'b');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'c');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_multiple_chars = 0;
    }
    #[test]
    fn test_next_char_or_null_whitespace() {
        let _rug_st_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_whitespace = 0;
        let rug_fuzz_0 = b" \n\r\t";
        let mut de = Deserializer::from_slice(rug_fuzz_0);
        debug_assert_eq!(de.next_char_or_null().unwrap(), b' ');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\n');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\r');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\t');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_whitespace = 0;
    }
    #[test]
    fn test_next_char_or_null_unicode() {
        let _rug_st_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_unicode = 0;
        let rug_fuzz_0 = "ñ";
        let mut de = Deserializer::from_slice(rug_fuzz_0.as_bytes());
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\xc3');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\xb1');
        debug_assert_eq!(de.next_char_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_449_rrrruuuugggg_test_next_char_or_null_unicode = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_452 {
    use super::*;
    use crate::*;
    use crate::de::{Deserializer, ErrorCode, ParserNumber};
    use crate::{Error, Result};
    #[test]
    fn test_parse_decimal() {
        fn parse_string(s: &'static str) -> Result<f64> {
            let mut de = Deserializer::from_slice(s.as_bytes());
            let mut positive = true;
            let mut significand = 0;
            match de.parse_any_signed_number()? {
                ParserNumber::F64(f) => Ok(f),
                ParserNumber::I64(i) => {
                    if i < 0 {
                        positive = false;
                        significand = i.wrapping_abs() as u64;
                    } else {
                        significand = i as u64;
                    }
                    de.parse_decimal(positive, significand, 0)
                        .map_err(|_| de.peek_error(ErrorCode::InvalidNumber))
                }
                ParserNumber::U64(u) => {
                    positive = true;
                    significand = u as u64;
                    de.parse_decimal(positive, significand, 0)
                        .map_err(|_| de.peek_error(ErrorCode::InvalidNumber))
                }
                #[cfg(feature = "arbitrary_precision")]
                ParserNumber::String(_) => unreachable!(),
            }
        }
        let test_cases = vec![
            ("0.1", 0.1f64), ("-0.1", - 0.1f64), ("10.5", 10.5f64), ("-10.5", - 10.5f64),
            ("0.0000000001", 0.0000000001f64), ("123.456", 123.456f64), ("-123.456", -
            123.456f64), ("1234567890.123456789", 1234567890.1234567f64),
        ];
        for (input, expected) in test_cases {
            let result = parse_string(input);
            assert!(result.is_ok(), "Parsing failed for input: {}", input);
            let result_f64 = result.unwrap();
            let delta = (result_f64 - expected).abs();
            assert!(
                delta < std::f64::EPSILON, "Delta for {} is too large: {}", input, delta
            );
        }
    }
}
#[cfg(test)]
mod tests_llm_16_455_llm_16_455 {
    use super::*;
    use crate::*;
    use crate::de::{read, Deserializer, Error, ErrorCode, Result};
    fn create_deserializer(input: &'static str) -> Deserializer<read::StrRead<'static>> {
        Deserializer::from_str(input)
    }
    fn create_error(code: ErrorCode) -> Error {
        Error::syntax(code, 0, 0)
    }
    #[test]
    fn test_parse_exponent_overflow_positive_zero_significand_positive_exp() {
        let mut de = create_deserializer("0e309");
        let result = de.parse_exponent_overflow(true, true, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }
    #[test]
    fn test_parse_exponent_overflow_positive_zero_significand_negative_exp() {
        let mut de = create_deserializer("0e-309");
        let result = de.parse_exponent_overflow(true, true, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }
    #[test]
    fn test_parse_exponent_overflow_negative_zero_significand_positive_exp() {
        let mut de = create_deserializer("-0e309");
        let result = de.parse_exponent_overflow(false, true, true);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, - 0.0);
        assert!(value.is_sign_negative());
    }
    #[test]
    fn test_parse_exponent_overflow_negative_zero_significand_negative_exp() {
        let mut de = create_deserializer("-0e-309");
        let result = de.parse_exponent_overflow(false, true, false);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, - 0.0);
        assert!(value.is_sign_negative());
    }
    #[test]
    fn test_parse_exponent_overflow_nonzero_significand_positive_exp() {
        let mut de = create_deserializer("1e309");
        let result = de.parse_exponent_overflow(true, false, true);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.classify(), create_error(ErrorCode::NumberOutOfRange).classify());
    }
    #[test]
    fn test_parse_exponent_overflow_nonzero_significand_negative_exp() {
        let mut de = create_deserializer("1e-309");
        let result = de.parse_exponent_overflow(true, false, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }
}
#[cfg(test)]
mod tests_llm_16_461 {
    use super::*;
    use crate::*;
    use crate::error::Error;
    use crate::de::{Deserializer, SliceRead};
    fn parse_whitespace_helper(input: &[u8], expected: Option<u8>) -> Result<()> {
        let mut de = Deserializer::from_slice(input);
        let parse_result = de.parse_whitespace()?;
        assert_eq!(parse_result, expected);
        Ok(())
    }
    #[test]
    fn parse_whitespace_empty_slice() -> Result<()> {
        parse_whitespace_helper(b"", None)
    }
    #[test]
    fn parse_whitespace_space() -> Result<()> {
        parse_whitespace_helper(b" ", Some(b' '))
    }
    #[test]
    fn parse_whitespace_newline() -> Result<()> {
        parse_whitespace_helper(b"\nfoo", Some(b'f'))
    }
    #[test]
    fn parse_whitespace_mixed() -> Result<()> {
        parse_whitespace_helper(b" \t\n \rbar", Some(b'b'))
    }
    #[test]
    fn parse_whitespace_eof() -> Result<()> {
        parse_whitespace_helper(b"\t\n \r", None)
    }
    #[test]
    fn parse_whitespace_non_whitespace() -> Result<()> {
        parse_whitespace_helper(b"foo", Some(b'f'))
    }
}
#[cfg(test)]
mod tests_llm_16_463_llm_16_463 {
    use super::*;
    use crate::*;
    use crate::de::{Deserializer, Error, ErrorCode};
    use crate::error::Category;
    #[test]
    fn peek_error_eof_while_parsing_value() {
        let raw_input = br#"{"some_key": "some_value""#;
        let mut de = Deserializer::from_slice(raw_input);
        let error = de.peek_error(ErrorCode::EofWhileParsingValue);
        assert_eq!(error.line(), 1);
        assert_eq!(error.column(), raw_input.len() + 1);
        assert!(matches!(error.classify(), Category::Eof));
    }
    #[test]
    fn peek_error_expected_some_value() {
        let raw_input = br#"{"some_key":  "some_value", "another_key": }"#;
        let mut de = Deserializer::from_slice(raw_input);
        let error_offset = 34;
        let error = de.peek_error(ErrorCode::ExpectedSomeValue);
        assert_eq!(error.line(), 1);
        assert_eq!(error.column(), error_offset + 1);
        assert!(matches!(error.classify(), Category::Syntax));
    }
    #[test]
    fn peek_error_invalid_number() {
        let raw_input = br#"{"some_key": 123A}"#;
        let mut de = Deserializer::from_slice(raw_input);
        let error_offset = 14;
        let error = de.peek_error(ErrorCode::InvalidNumber);
        assert_eq!(error.line(), 1);
        assert_eq!(error.column(), error_offset + 1);
        assert!(matches!(error.classify(), Category::Syntax));
    }
    #[test]
    fn peek_error_trailing_characters() {
        let raw_input = br#"{"some_key": "some_value"} trailing"#;
        let mut de = Deserializer::from_slice(raw_input);
        let error_offset = 27;
        let error = de.peek_error(ErrorCode::TrailingCharacters);
        assert_eq!(error.line(), 1);
        assert_eq!(error.column(), error_offset + 1);
        assert!(matches!(error.classify(), Category::Syntax));
    }
    fn create_error_state_de() -> Deserializer<crate::de::read::SliceRead<'static>> {
        let raw_input = br#"{"some_key": "some""#;
        let mut de = Deserializer::from_slice(raw_input);
        de.peek_error(ErrorCode::EofWhileParsingValue);
        de
    }
    #[test]
    fn error_fix_position() {
        let error = create_error_state_de().peek_error(ErrorCode::EofWhileParsingValue);
        assert_eq!(error.line(), 1);
        assert!(error.column() > 0);
        assert!(error.is_eof());
    }
    #[test]
    fn error_fix_position_non_eof() {
        let error = create_error_state_de().peek_error(ErrorCode::InvalidNumber);
        assert_eq!(error.line(), 1);
        assert!(error.column() > 0);
        assert!(error.is_syntax());
    }
}
#[cfg(test)]
mod tests_llm_16_465 {
    use super::*;
    use crate::*;
    use crate::error::Result;
    use crate::de::{Deserializer, read};
    #[test]
    fn test_peek_or_null_empty_slice() {
        let _rug_st_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_empty_slice = 0;
        let rug_fuzz_0 = b"";
        let empty_slice = &rug_fuzz_0[..];
        let mut de = Deserializer::from_slice(empty_slice);
        debug_assert_eq!(de.peek_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_empty_slice = 0;
    }
    #[test]
    fn test_peek_or_null_non_empty_slice() {
        let _rug_st_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_non_empty_slice = 0;
        let rug_fuzz_0 = b"some data";
        let non_empty_slice = &rug_fuzz_0[..];
        let mut de = Deserializer::from_slice(non_empty_slice);
        debug_assert_eq!(de.peek_or_null().unwrap(), b's');
        let _rug_ed_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_non_empty_slice = 0;
    }
    #[test]
    fn test_peek_or_null_after_exhausting_data() {
        let _rug_st_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_after_exhausting_data = 0;
        let rug_fuzz_0 = b"some data";
        let data_slice = &rug_fuzz_0[..];
        let mut de = Deserializer::from_slice(data_slice);
        let _ = de.end().unwrap();
        debug_assert_eq!(de.peek_or_null().unwrap(), b'\x00');
        let _rug_ed_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_after_exhausting_data = 0;
    }
    #[test]
    fn test_peek_or_null_after_partial_consume() {
        let _rug_st_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_after_partial_consume = 0;
        let rug_fuzz_0 = b"some data";
        let data_slice = &rug_fuzz_0[..];
        let mut de = Deserializer::from_slice(data_slice);
        de.peek().unwrap();
        de.eat_char();
        debug_assert_eq!(de.peek_or_null().unwrap(), b'o');
        let _rug_ed_tests_llm_16_465_rrrruuuugggg_test_peek_or_null_after_partial_consume = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_466 {
    use crate::de::{Deserializer, ErrorCode, Error};
    use crate::error::Result;
    fn scan_integer128_test(input: &str, expected: Result<String>) {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_scan_integer128_test = 0;
        let mut de = Deserializer::from_str(input);
        let mut actual = String::new();
        let result = de.scan_integer128(&mut actual);
        match expected {
            Ok(ref expected_str) => {
                debug_assert!(result.is_ok());
                debug_assert_eq!(actual, * expected_str);
            }
            Err(ref expected_err) => {
                debug_assert!(result.is_err());
                let actual_err = result.err().unwrap();
                debug_assert_eq!(actual_err.classify(), expected_err.classify());
            }
        }
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_scan_integer128_test = 0;
    }
    #[test]
    fn test_scan_integer128_single_zero() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_single_zero = 0;
        let rug_fuzz_0 = "0";
        let rug_fuzz_1 = "0";
        scan_integer128_test(rug_fuzz_0, Ok(rug_fuzz_1.to_string()));
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_single_zero = 0;
    }
    #[test]
    fn test_scan_integer128_leading_zero() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_leading_zero = 0;
        let rug_fuzz_0 = "0123";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        scan_integer128_test(
            rug_fuzz_0,
            Err(Error::syntax(ErrorCode::InvalidNumber, rug_fuzz_1, rug_fuzz_2)),
        );
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_leading_zero = 0;
    }
    #[test]
    fn test_scan_integer128_valid() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_valid = 0;
        let rug_fuzz_0 = "12345";
        let rug_fuzz_1 = "12345";
        scan_integer128_test(rug_fuzz_0, Ok(rug_fuzz_1.to_string()));
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_valid = 0;
    }
    #[test]
    fn test_scan_integer128_valid_with_following() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_valid_with_following = 0;
        let rug_fuzz_0 = "12345abc";
        let rug_fuzz_1 = "12345";
        scan_integer128_test(rug_fuzz_0, Ok(rug_fuzz_1.to_string()));
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_valid_with_following = 0;
    }
    #[test]
    fn test_scan_integer128_invalid() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_invalid = 0;
        let rug_fuzz_0 = "abc";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        scan_integer128_test(
            rug_fuzz_0,
            Err(Error::syntax(ErrorCode::InvalidNumber, rug_fuzz_1, rug_fuzz_2)),
        );
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_scan_integer128_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_467 {
    use crate::de;
    use serde::Deserialize;
    use std::io::Cursor;
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        key: String,
        value: i32,
    }
    #[test]
    fn test_from_reader() {
        let _rug_st_tests_llm_16_467_rrrruuuugggg_test_from_reader = 0;
        let rug_fuzz_0 = r#"{ "key": "test_key", "value": 42 }"#;
        let json_data = rug_fuzz_0;
        let cursor = Cursor::new(json_data.as_bytes());
        let mut deserializer = de::Deserializer::<
            de::read::IoRead<Cursor<&[u8]>>,
        >::from_reader(cursor);
        let test_struct: TestStruct = Deserialize::deserialize(&mut deserializer)
            .unwrap();
        debug_assert_eq!(
            test_struct, TestStruct { key : "test_key".to_string(), value : 42 }
        );
        let _rug_ed_tests_llm_16_467_rrrruuuugggg_test_from_reader = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_468 {
    use super::*;
    use crate::*;
    use serde::Deserialize;
    use crate::error::Result;
    use std::fmt::Debug;
    fn assert_de_tokens<'de, T>(value: &T, tokens: &'de [(&'de [u8], T)])
    where
        T: Debug + PartialEq + Deserialize<'de>,
    {
        let _rug_st_tests_llm_16_468_rrrruuuugggg_assert_de_tokens = 0;
        for &(token, ref expected) in tokens {
            let mut de = Deserializer::from_slice(token);
            let actual = T::deserialize(&mut de).expect(&format!("token: {:?}", token));
            debug_assert_eq!(actual, * expected);
        }
        let _rug_ed_tests_llm_16_468_rrrruuuugggg_assert_de_tokens = 0;
    }
    #[test]
    fn test_from_slice() {
        let _rug_st_tests_llm_16_468_rrrruuuugggg_test_from_slice = 0;
        let rug_fuzz_0 = b"null";
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = b"true";
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = false;
        let rug_fuzz_5 = b"false";
        let rug_fuzz_6 = false;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = b"10";
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = b"-10";
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 1.5f64;
        let rug_fuzz_14 = b"1.5";
        let rug_fuzz_15 = 1.5;
        let rug_fuzz_16 = "abc";
        let rug_fuzz_17 = b"\"abc\"";
        let rug_fuzz_18 = "abc";
        let rug_fuzz_19 = "a\"b";
        let rug_fuzz_20 = b"\"a\\\"b\"";
        let rug_fuzz_21 = "a\"b";
        let rug_fuzz_22 = true;
        let rug_fuzz_23 = b"[true,false]";
        let rug_fuzz_24 = true;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = b"[1,2,3]";
        let rug_fuzz_27 = 1;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 2;
        let rug_fuzz_30 = b"{\"x\":1,\"y\":2}";
        let rug_fuzz_31 = 1;
        let rug_fuzz_32 = 2;
        assert_de_tokens(&(), &[(rug_fuzz_0, ())]);
        assert_de_tokens(&rug_fuzz_1, &[(rug_fuzz_2, rug_fuzz_3)]);
        assert_de_tokens(&rug_fuzz_4, &[(rug_fuzz_5, rug_fuzz_6)]);
        assert_de_tokens(&rug_fuzz_7, &[(rug_fuzz_8, rug_fuzz_9)]);
        assert_de_tokens(&-rug_fuzz_10, &[(rug_fuzz_11, -rug_fuzz_12)]);
        assert_de_tokens(&rug_fuzz_13, &[(rug_fuzz_14, rug_fuzz_15)]);
        assert_de_tokens(&rug_fuzz_16, &[(rug_fuzz_17, rug_fuzz_18)]);
        assert_de_tokens(&rug_fuzz_19, &[(rug_fuzz_20, rug_fuzz_21)]);
        assert_de_tokens(
            &vec![rug_fuzz_22, false],
            &[(rug_fuzz_23, vec![rug_fuzz_24, false])],
        );
        assert_de_tokens(
            &vec![rug_fuzz_25, 2, 3],
            &[(rug_fuzz_26, vec![rug_fuzz_27, 2, 3])],
        );
        #[derive(Deserialize, PartialEq, Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        assert_de_tokens(
            &Point {
                x: rug_fuzz_28,
                y: rug_fuzz_29,
            },
            &[
                (
                    rug_fuzz_30,
                    Point {
                        x: rug_fuzz_31,
                        y: rug_fuzz_32,
                    },
                ),
            ],
        );
        let _rug_ed_tests_llm_16_468_rrrruuuugggg_test_from_slice = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_469_llm_16_469 {
    use super::*;
    use crate::*;
    use serde::Deserialize;
    #[test]
    fn test_from_str_valid_json() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_from_str_valid_json = 0;
        let rug_fuzz_0 = r#"{"name":"John","age":30}"#;
        let rug_fuzz_1 = "name";
        let rug_fuzz_2 = "age";
        let s = rug_fuzz_0;
        let mut deserializer = Deserializer::<read::StrRead>::from_str(s);
        let value: crate::Value = Deserialize::deserialize(&mut deserializer).unwrap();
        debug_assert_eq!(value[rug_fuzz_1], "John");
        debug_assert_eq!(value[rug_fuzz_2], 30);
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_from_str_valid_json = 0;
    }
    #[test]
    fn test_from_str_empty() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_from_str_empty = 0;
        let rug_fuzz_0 = "";
        let s = rug_fuzz_0;
        let mut deserializer = Deserializer::<read::StrRead>::from_str(s);
        let result: crate::error::Result<crate::Value> = Deserialize::deserialize(
            &mut deserializer,
        );
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_from_str_empty = 0;
    }
    #[test]
    fn test_from_str_invalid_json() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_from_str_invalid_json = 0;
        let rug_fuzz_0 = "{name:John,age:30}";
        let s = rug_fuzz_0;
        let mut deserializer = Deserializer::<read::StrRead>::from_str(s);
        let result: crate::error::Result<crate::Value> = Deserialize::deserialize(
            &mut deserializer,
        );
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_from_str_invalid_json = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_470 {
    use super::*;
    use crate::*;
    use serde::Deserialize;
    use crate::de::{Deserializer, MapAccess};
    #[derive(Deserialize)]
    struct TestStruct {
        key: String,
    }
    #[test]
    fn test_map_access_new() {
        let _rug_st_tests_llm_16_470_rrrruuuugggg_test_map_access_new = 0;
        let rug_fuzz_0 = r#"{"key": "value"}"#;
        let json_str = rug_fuzz_0;
        let mut deserializer = Deserializer::from_str(json_str);
        let map_access = MapAccess::new(&mut deserializer);
        debug_assert_eq!(map_access.first, true);
        let result: TestStruct = Deserialize::deserialize(&mut deserializer).unwrap();
        debug_assert_eq!(result.key, "value");
        let _rug_ed_tests_llm_16_470_rrrruuuugggg_test_map_access_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_471 {
    use super::*;
    use crate::*;
    use serde::de::{self, Expected, Unexpected};
    struct MyExpected;
    impl Expected for MyExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "unit test dummy expected")
        }
    }
    #[test]
    fn test_invalid_type_f64() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_invalid_type_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let num = ParserNumber::F64(rug_fuzz_0);
        let exp = MyExpected;
        let err = num.invalid_type(&exp);
        debug_assert!(err.is_data());
        debug_assert_eq!(
            err.to_string(), "invalid type: 42.0, expected unit test dummy expected"
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_invalid_type_f64 = 0;
    }
    #[test]
    fn test_invalid_type_u64() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_invalid_type_u64 = 0;
        let rug_fuzz_0 = 42;
        let num = ParserNumber::U64(rug_fuzz_0);
        let exp = MyExpected;
        let err = num.invalid_type(&exp);
        debug_assert!(err.is_data());
        debug_assert_eq!(
            err.to_string(), "invalid type: 42, expected unit test dummy expected"
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_invalid_type_u64 = 0;
    }
    #[test]
    fn test_invalid_type_i64() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_invalid_type_i64 = 0;
        let rug_fuzz_0 = 42;
        let num = ParserNumber::I64(-rug_fuzz_0);
        let exp = MyExpected;
        let err = num.invalid_type(&exp);
        debug_assert!(err.is_data());
        debug_assert_eq!(
            err.to_string(), "invalid type: -42, expected unit test dummy expected"
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_invalid_type_i64 = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_invalid_type_string() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_invalid_type_string = 0;
        let rug_fuzz_0 = "42";
        let num = ParserNumber::String(rug_fuzz_0.to_owned());
        let exp = MyExpected;
        let err = num.invalid_type(&exp);
        debug_assert!(err.is_data());
        debug_assert_eq!(
            err.to_string(), "invalid type: number, expected unit test dummy expected"
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_invalid_type_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_472_llm_16_472 {
    use super::*;
    use crate::*;
    use crate::de::ParserNumber;
    use serde::de::{self, Visitor};
    use std::fmt;
    #[derive(Debug)]
    struct Map<T, V>(std::marker::PhantomData<(T, V)>);
    impl<T, V> Map<T, V> {
        fn new() -> Self {
            Map(std::marker::PhantomData)
        }
    }
    #[derive(Debug)]
    struct Value;
    #[derive(Debug)]
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_i64<E>(self, _value: i64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        fn visit_u64<E>(self, _value: u64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        fn visit_f64<E>(self, _value: f64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(feature = "arbitrary_precision")]
        fn visit_map<V>(self, _: V) -> std::result::Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(Map::new())
        }
    }
    #[test]
    fn visit_f64_test() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_f64_test = 0;
        let rug_fuzz_0 = 3.14159;
        let pn = ParserNumber::F64(rug_fuzz_0);
        let visitor = TestVisitor;
        let result = pn.visit(visitor).unwrap();
        debug_assert!(matches!(result, Map(_)));
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_f64_test = 0;
    }
    #[test]
    fn visit_u64_test() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_u64_test = 0;
        let rug_fuzz_0 = 42;
        let pn = ParserNumber::U64(rug_fuzz_0);
        let visitor = TestVisitor;
        let result = pn.visit(visitor).unwrap();
        debug_assert!(matches!(result, Map(_)));
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_u64_test = 0;
    }
    #[test]
    fn visit_i64_test() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_i64_test = 0;
        let rug_fuzz_0 = 42;
        let pn = ParserNumber::I64(-rug_fuzz_0);
        let visitor = TestVisitor;
        let result = pn.visit(visitor).unwrap();
        debug_assert!(matches!(result, Map(_)));
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_i64_test = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn visit_string_test() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_string_test = 0;
        let rug_fuzz_0 = "42";
        let pn = ParserNumber::String(rug_fuzz_0.to_owned());
        let visitor = TestVisitor;
        let result = pn.visit(visitor).unwrap();
        debug_assert!(matches!(result, Map(_)));
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_visit_string_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_474_llm_16_474 {
    use crate::{Deserializer, StreamDeserializer, de, error::Category};
    use serde::Deserialize;
    #[derive(Debug, Deserialize, PartialEq)]
    struct SimpleIntStruct {
        value: i32,
    }
    #[test]
    fn test_byte_offset() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_byte_offset = 0;
        let rug_fuzz_0 = b"{ \"value\": 1 } { \"value\": 2 } nonsense { \"value\": 4 }";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 32;
        let rug_fuzz_6 = 33;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 33;
        let rug_fuzz_9 = " nonsense ";
        let data = rug_fuzz_0;
        let mut stream = Deserializer::from_slice(data).into_iter::<SimpleIntStruct>();
        debug_assert_eq!(rug_fuzz_1, stream.byte_offset());
        let first_value: SimpleIntStruct = stream.next().unwrap().unwrap();
        debug_assert_eq!(SimpleIntStruct { value : rug_fuzz_2 }, first_value);
        debug_assert_eq!(rug_fuzz_3, stream.byte_offset());
        let second_value: SimpleIntStruct = stream.next().unwrap().unwrap();
        debug_assert_eq!(SimpleIntStruct { value : rug_fuzz_4 }, second_value);
        debug_assert_eq!(rug_fuzz_5, stream.byte_offset());
        let error = stream.next().unwrap().unwrap_err();
        debug_assert_eq!(Category::Data, error.classify());
        debug_assert_eq!(rug_fuzz_6, stream.byte_offset());
        let third_value: SimpleIntStruct = stream.next().unwrap().unwrap();
        debug_assert_eq!(SimpleIntStruct { value : rug_fuzz_7 }, third_value);
        let after_nonsense_offset = rug_fuzz_8 + rug_fuzz_9.len();
        debug_assert!(stream.byte_offset() > after_nonsense_offset);
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_byte_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_476_llm_16_476 {
    use super::*;
    use crate::*;
    use serde::de::{Deserialize, IntoDeserializer};
    use crate::de::{Deserializer, StreamDeserializer};
    use crate::error::{Error, ErrorCode};
    use crate::de::read::StrRead;
    use std::marker::PhantomData;
    use std::result;
    type Result<T> = result::Result<T, Error>;
    fn create_stream_deserializer<'de, T>(
        input: &'de str,
    ) -> StreamDeserializer<'de, StrRead<'de>, T>
    where
        T: Deserialize<'de>,
    {
        let de = Deserializer::from_str(input);
        de.into_iter::<T>()
    }
    #[test]
    fn test_peek_end_of_value_ok() {
        let mut stream_deserializer = create_stream_deserializer::<String>(r#""test""#);
        let peek_result = stream_deserializer.peek_end_of_value();
        assert!(peek_result.is_ok());
    }
    #[test]
    fn test_peek_end_of_value_trailing_comma() {
        let mut stream_deserializer = create_stream_deserializer::<
            String,
        >(r#""test", {"key":"value"}"#);
        stream_deserializer.next().unwrap().unwrap();
        let peek_result = stream_deserializer.peek_end_of_value();
        assert!(peek_result.is_ok());
    }
    #[test]
    fn test_peek_end_of_value_trailing_bracket() {
        let mut stream_deserializer = create_stream_deserializer::<String>(r#""test"]"#);
        let peek_result = stream_deserializer.peek_end_of_value();
        assert!(peek_result.is_ok());
    }
    #[test]
    fn test_peek_end_of_value_trailing_white_space() {
        let mut stream_deserializer = create_stream_deserializer::<
            String,
        >(r#""test"    "#);
        let peek_result = stream_deserializer.peek_end_of_value();
        assert!(peek_result.is_ok());
    }
    #[test]
    fn test_peek_end_of_value_trailing_invalid() {
        let mut stream_deserializer = create_stream_deserializer::<
            String,
        >(r#""test"invalid"#);
        let peek_result = stream_deserializer.peek_end_of_value();
        assert!(peek_result.is_err());
        assert_eq!(
            peek_result.unwrap_err().to_string(),
            Error::syntax(ErrorCode::TrailingCharacters, 1, 7).to_string()
        );
    }
    #[test]
    fn test_peek_end_of_value_end_of_input() {
        let mut stream_deserializer = create_stream_deserializer::<String>(r#""test""#);
        stream_deserializer.next().unwrap().unwrap();
        let peek_result = stream_deserializer.peek_end_of_value();
        assert!(peek_result.is_ok());
    }
}
#[cfg(test)]
mod tests_llm_16_480 {
    use serde::Deserialize;
    use crate::{de::from_slice, Value, Error};
    #[derive(Deserialize, PartialEq, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }
    #[test]
    fn test_from_slice_valid_json() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_valid_json = 0;
        let rug_fuzz_0 = br#"
        {
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": "Menlo Park, CA"
        }"#;
        let rug_fuzz_1 = "0xF9BA143B95FF6D82";
        let rug_fuzz_2 = "Menlo Park, CA";
        let json_data = rug_fuzz_0;
        let expected_user = User {
            fingerprint: rug_fuzz_1.to_string(),
            location: rug_fuzz_2.to_string(),
        };
        let result: Result<User, Error> = from_slice(json_data);
        debug_assert_eq!(result.unwrap(), expected_user);
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_valid_json = 0;
    }
    #[test]
    fn test_from_slice_invalid_json_syntax() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_invalid_json_syntax = 0;
        let rug_fuzz_0 = br#" { "fingerprint": "0xF9BA143B95FF6D82", "location": "Menlo Park, CA", } "#;
        let json_data = rug_fuzz_0;
        let result: Result<User, Error> = from_slice(json_data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_invalid_json_syntax = 0;
    }
    #[test]
    fn test_from_slice_invalid_json_data_type() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_invalid_json_data_type = 0;
        let rug_fuzz_0 = br#"
        {
            "fingerprint": 12345,
            "location": "Menlo Park, CA"
        }"#;
        let json_data = rug_fuzz_0;
        let result: Result<User, Error> = from_slice(json_data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_invalid_json_data_type = 0;
    }
    #[test]
    fn test_from_slice_missing_json_key() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_missing_json_key = 0;
        let rug_fuzz_0 = br#"
        {
            "location": "Menlo Park, CA"
        }"#;
        let json_data = rug_fuzz_0;
        let result: Result<User, Error> = from_slice(json_data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_missing_json_key = 0;
    }
    #[test]
    fn test_from_slice_empty_json_object() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_empty_json_object = 0;
        let rug_fuzz_0 = br#"{}"#;
        let json_data = rug_fuzz_0;
        let result: Result<User, Error> = from_slice(json_data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_empty_json_object = 0;
    }
    #[test]
    fn test_from_slice_extra_json_key() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_extra_json_key = 0;
        let rug_fuzz_0 = br#"
        {
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": "Menlo Park, CA",
            "extra": "data"
        }"#;
        let json_data = rug_fuzz_0;
        let result: Result<User, Error> = from_slice(json_data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_extra_json_key = 0;
    }
    #[test]
    fn test_from_slice_json_array() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_json_array = 0;
        let rug_fuzz_0 = br#"
        [
            {
                "fingerprint": "0xF9BA143B95FF6D82",
                "location": "Menlo Park, CA"
            }
        ]
        "#;
        let json_data = rug_fuzz_0;
        let result: Result<Vec<User>, Error> = from_slice(json_data);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_json_array = 0;
    }
    #[test]
    fn test_from_slice_json_value() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_from_slice_json_value = 0;
        let rug_fuzz_0 = br#"
        {
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": "Menlo Park, CA"
        }
        "#;
        let json_data = rug_fuzz_0;
        let result: Result<Value, Error> = from_slice(json_data);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_from_slice_json_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_481_llm_16_481 {
    use serde::{Deserialize, Serialize};
    use crate::{from_str, json, Error, Value};
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum TestEnum {
        Number(i32),
        Text(String),
    }
    #[test]
    fn test_from_str_valid_user() {
        let _rug_st_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_valid_user = 0;
        let rug_fuzz_0 = r#"{
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": "Menlo Park, CA"
        }"#;
        let data = rug_fuzz_0;
        let u: User = from_str(data).unwrap();
        debug_assert_eq!(
            u, User { fingerprint : "0xF9BA143B95FF6D82".to_string(), location :
            "Menlo Park, CA".to_string(), }
        );
        let _rug_ed_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_valid_user = 0;
    }
    #[test]
    fn test_from_str_invalid_user() {
        let _rug_st_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_invalid_user = 0;
        let rug_fuzz_0 = r#"{
            "fingerprint": "0xF9BA143B95FF6D82",
            "age": 30
        }"#;
        let data = rug_fuzz_0;
        let result: Result<User, Error> = from_str(data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_invalid_user = 0;
    }
    #[test]
    fn test_from_str_valid_enum_number() {
        let _rug_st_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_valid_enum_number = 0;
        let rug_fuzz_0 = r#"5"#;
        let data = rug_fuzz_0;
        let value: TestEnum = from_str(data).unwrap();
        debug_assert_eq!(value, TestEnum::Number(5));
        let _rug_ed_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_valid_enum_number = 0;
    }
    #[test]
    fn test_from_str_valid_enum_text() {
        let _rug_st_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_valid_enum_text = 0;
        let rug_fuzz_0 = r#""hello""#;
        let data = rug_fuzz_0;
        let value: TestEnum = from_str(data).unwrap();
        debug_assert_eq!(value, TestEnum::Text("hello".to_string()));
        let _rug_ed_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_valid_enum_text = 0;
    }
    #[test]
    fn test_from_str_json_value() {
        let _rug_st_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_json_value = 0;
        let rug_fuzz_0 = r#"{"key": "value"}"#;
        let data = rug_fuzz_0;
        let value: Value = from_str(data).unwrap();
        debug_assert_eq!(value, json!({ "key" : "value" }));
        let _rug_ed_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_json_value = 0;
    }
    #[test]
    fn test_from_str_invalid_json() {
        let _rug_st_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_invalid_json = 0;
        let rug_fuzz_0 = r#"{"key": "value"#;
        let data = rug_fuzz_0;
        let result: Result<Value, Error> = from_str(data);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_481_llm_16_481_rrrruuuugggg_test_from_str_invalid_json = 0;
    }
}
