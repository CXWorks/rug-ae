//! Serialize a Rust data structure into JSON data.
use crate::error::{Error, ErrorCode, Result};
use crate::io;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Display};
use core::num::FpCategory;
use serde::ser::{self, Impossible, Serialize};
/// A structure for serializing Rust values into JSON.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}
impl<W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new JSON serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer::with_formatter(writer, CompactFormatter)
    }
}
impl<'a, W> Serializer<W, PrettyFormatter<'a>>
where
    W: io::Write,
{
    /// Creates a new JSON pretty print serializer.
    #[inline]
    pub fn pretty(writer: W) -> Self {
        Serializer::with_formatter(writer, PrettyFormatter::new())
    }
}
impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    /// Creates a new JSON visitor whose output will be written to the writer
    /// specified.
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer { writer, formatter }
    }
    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}
impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W, F>;
    type SerializeTuple = Compound<'a, W, F>;
    type SerializeTupleStruct = Compound<'a, W, F>;
    type SerializeTupleVariant = Compound<'a, W, F>;
    type SerializeMap = Compound<'a, W, F>;
    type SerializeStruct = Compound<'a, W, F>;
    type SerializeStructVariant = Compound<'a, W, F>;
    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.formatter.write_bool(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.formatter.write_i8(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.formatter.write_i16(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.formatter.write_i32(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.formatter.write_i64(&mut self.writer, value).map_err(Error::io)
    }
    fn serialize_i128(self, value: i128) -> Result<()> {
        self.formatter.write_i128(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        self.formatter.write_u8(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        self.formatter.write_u16(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        self.formatter.write_u32(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        self.formatter.write_u64(&mut self.writer, value).map_err(Error::io)
    }
    fn serialize_u128(self, value: u128) -> Result<()> {
        self.formatter.write_u128(&mut self.writer, value).map_err(Error::io)
    }
    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        match value.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                self.formatter.write_null(&mut self.writer).map_err(Error::io)
            }
            _ => self.formatter.write_f32(&mut self.writer, value).map_err(Error::io),
        }
    }
    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        match value.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                self.formatter.write_null(&mut self.writer).map_err(Error::io)
            }
            _ => self.formatter.write_f64(&mut self.writer, value).map_err(Error::io),
        }
    }
    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }
    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        format_escaped_str(&mut self.writer, &mut self.formatter, value)
            .map_err(Error::io)
    }
    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = tri!(self.serialize_seq(Some(value.len())));
        for byte in value {
            tri!(seq.serialize_element(byte));
        }
        seq.end()
    }
    #[inline]
    fn serialize_unit(self) -> Result<()> {
        self.formatter.write_null(&mut self.writer).map_err(Error::io)
    }
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }
    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }
    /// Serialize newtypes without an object wrapper.
    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        tri!(self.formatter.begin_object(& mut self.writer).map_err(Error::io));
        tri!(
            self.formatter.begin_object_key(& mut self.writer, true).map_err(Error::io)
        );
        tri!(self.serialize_str(variant));
        tri!(self.formatter.end_object_key(& mut self.writer).map_err(Error::io));
        tri!(self.formatter.begin_object_value(& mut self.writer).map_err(Error::io));
        tri!(value.serialize(& mut * self));
        tri!(self.formatter.end_object_value(& mut self.writer).map_err(Error::io));
        self.formatter.end_object(&mut self.writer).map_err(Error::io)
    }
    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }
    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        tri!(self.formatter.begin_array(& mut self.writer).map_err(Error::io));
        if len == Some(0) {
            tri!(self.formatter.end_array(& mut self.writer).map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }
    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }
    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }
    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        tri!(self.formatter.begin_object(& mut self.writer).map_err(Error::io));
        tri!(
            self.formatter.begin_object_key(& mut self.writer, true).map_err(Error::io)
        );
        tri!(self.serialize_str(variant));
        tri!(self.formatter.end_object_key(& mut self.writer).map_err(Error::io));
        tri!(self.formatter.begin_object_value(& mut self.writer).map_err(Error::io));
        self.serialize_seq(Some(len))
    }
    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        tri!(self.formatter.begin_object(& mut self.writer).map_err(Error::io));
        if len == Some(0) {
            tri!(self.formatter.end_object(& mut self.writer).map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }
    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(Compound::Number { ser: self }),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(Compound::RawValue { ser: self }),
            _ => self.serialize_map(Some(len)),
        }
    }
    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        tri!(self.formatter.begin_object(& mut self.writer).map_err(Error::io));
        tri!(
            self.formatter.begin_object_key(& mut self.writer, true).map_err(Error::io)
        );
        tri!(self.serialize_str(variant));
        tri!(self.formatter.end_object_key(& mut self.writer).map_err(Error::io));
        tri!(self.formatter.begin_object_value(& mut self.writer).map_err(Error::io));
        self.serialize_map(Some(len))
    }
    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Display,
    {
        use self::fmt::Write;
        struct Adapter<'ser, W: 'ser, F: 'ser> {
            writer: &'ser mut W,
            formatter: &'ser mut F,
            error: Option<io::Error>,
        }
        impl<'ser, W, F> Write for Adapter<'ser, W, F>
        where
            W: io::Write,
            F: Formatter,
        {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                debug_assert!(self.error.is_none());
                match format_escaped_str_contents(self.writer, self.formatter, s) {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        self.error = Some(err);
                        Err(fmt::Error)
                    }
                }
            }
        }
        tri!(self.formatter.begin_string(& mut self.writer).map_err(Error::io));
        {
            let mut adapter = Adapter {
                writer: &mut self.writer,
                formatter: &mut self.formatter,
                error: None,
            };
            match write!(adapter, "{}", value) {
                Ok(()) => debug_assert!(adapter.error.is_none()),
                Err(fmt::Error) => {
                    return Err(
                        Error::io(adapter.error.expect("there should be an error")),
                    );
                }
            }
        }
        self.formatter.end_string(&mut self.writer).map_err(Error::io)
    }
}
#[doc(hidden)]
#[derive(Eq, PartialEq)]
pub enum State {
    Empty,
    First,
    Rest,
}
#[doc(hidden)]
pub enum Compound<'a, W: 'a, F: 'a> {
    Map { ser: &'a mut Serializer<W, F>, state: State },
    #[cfg(feature = "arbitrary_precision")]
    Number { ser: &'a mut Serializer<W, F> },
    #[cfg(feature = "raw_value")]
    RawValue { ser: &'a mut Serializer<W, F> },
}
impl<'a, W, F> ser::SerializeSeq for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { ser, state } => {
                tri!(
                    ser.formatter.begin_array_value(& mut ser.writer, * state ==
                    State::First).map_err(Error::io)
                );
                *state = State::Rest;
                tri!(value.serialize(& mut ** ser));
                ser.formatter.end_array_value(&mut ser.writer).map_err(Error::io)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => Ok(()),
                    _ => ser.formatter.end_array(&mut ser.writer).map_err(Error::io),
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}
impl<'a, W, F> ser::SerializeTuple for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }
    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}
impl<'a, W, F> ser::SerializeTupleStruct for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }
    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}
impl<'a, W, F> ser::SerializeTupleVariant for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }
    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => {
                        tri!(
                            ser.formatter.end_array(& mut ser.writer).map_err(Error::io)
                        )
                    }
                }
                tri!(
                    ser.formatter.end_object_value(& mut ser.writer).map_err(Error::io)
                );
                ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}
impl<'a, W, F> ser::SerializeMap for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { ser, state } => {
                tri!(
                    ser.formatter.begin_object_key(& mut ser.writer, * state ==
                    State::First).map_err(Error::io)
                );
                *state = State::Rest;
                tri!(key.serialize(MapKeySerializer { ser : * ser }));
                ser.formatter.end_object_key(&mut ser.writer).map_err(Error::io)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { ser, .. } => {
                tri!(
                    ser.formatter.begin_object_value(& mut ser.writer).map_err(Error::io)
                );
                tri!(value.serialize(& mut ** ser));
                ser.formatter.end_object_value(&mut ser.writer).map_err(Error::io)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => Ok(()),
                    _ => ser.formatter.end_object(&mut ser.writer).map_err(Error::io),
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}
impl<'a, W, F> ser::SerializeStruct for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { ser, .. } => {
                if key == crate::number::TOKEN {
                    value.serialize(NumberStrEmitter(ser))
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            Compound::RawValue { ser, .. } => {
                if key == crate::raw::TOKEN {
                    value.serialize(RawValueStrEmitter(ser))
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }
    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { .. } => ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => Ok(()),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => Ok(()),
        }
    }
}
impl<'a, W, F> ser::SerializeStructVariant for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            Compound::Map { .. } => {
                ser::SerializeStruct::serialize_field(self, key, value)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => {
                        tri!(
                            ser.formatter.end_object(& mut ser.writer).map_err(Error::io)
                        )
                    }
                }
                tri!(
                    ser.formatter.end_object_value(& mut ser.writer).map_err(Error::io)
                );
                ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}
struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}
#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
    Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}
#[cfg(feature = "raw_value")]
fn invalid_raw_value() -> Error {
    Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0)
}
fn key_must_be_a_string() -> Error {
    Error::syntax(ErrorCode::KeyMustBeAString, 0, 0)
}
impl<'a, W, F> ser::Serializer for MapKeySerializer<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.ser.serialize_str(value)
    }
    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.ser.serialize_str(variant)
    }
    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;
    fn serialize_bool(self, _value: bool) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_i8(self, value: i8) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_i8(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_i16(self, value: i16) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_i16(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_i32(self, value: i32) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_i32(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_i64(self, value: i64) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_i64(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_i128(self, value: i128) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_i128(& mut self.ser.writer, value)
            .map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_u8(self, value: u8) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_u8(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_u16(self, value: u16) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_u16(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_u32(self, value: u32) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_u32(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_u64(self, value: u64) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_u64(& mut self.ser.writer, value).map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_u128(self, value: u128) -> Result<()> {
        tri!(self.ser.formatter.begin_string(& mut self.ser.writer).map_err(Error::io));
        tri!(
            self.ser.formatter.write_u128(& mut self.ser.writer, value)
            .map_err(Error::io)
        );
        self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
    }
    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_char(self, value: char) -> Result<()> {
        self.ser.serialize_str(&value.to_string())
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_unit(self) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }
    fn serialize_none(self) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }
    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Display,
    {
        self.ser.collect_str(value)
    }
}
#[cfg(feature = "arbitrary_precision")]
struct NumberStrEmitter<'a, W: 'a + io::Write, F: 'a + Formatter>(
    &'a mut Serializer<W, F>,
);
#[cfg(feature = "arbitrary_precision")]
impl<'a, W: io::Write, F: Formatter> ser::Serializer for NumberStrEmitter<'a, W, F> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_i128(self, _v: i128) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_u128(self, _v: u128) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_char(self, _v: char) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_str(self, value: &str) -> Result<()> {
        let NumberStrEmitter(serializer) = self;
        serializer
            .formatter
            .write_number_str(&mut serializer.writer, value)
            .map_err(Error::io)
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_none(self) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }
    fn serialize_unit(self) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(invalid_number())
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(invalid_number())
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(invalid_number())
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(invalid_number())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(invalid_number())
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(invalid_number())
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(invalid_number())
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(invalid_number())
    }
}
#[cfg(feature = "raw_value")]
struct RawValueStrEmitter<'a, W: 'a + io::Write, F: 'a + Formatter>(
    &'a mut Serializer<W, F>,
);
#[cfg(feature = "raw_value")]
impl<'a, W: io::Write, F: Formatter> ser::Serializer for RawValueStrEmitter<'a, W, F> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_i128(self, _v: i128) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_u128(self, _v: u128) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_char(self, _v: char) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_str(self, value: &str) -> Result<()> {
        let RawValueStrEmitter(serializer) = self;
        serializer
            .formatter
            .write_raw_fragment(&mut serializer.writer, value)
            .map_err(Error::io)
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_none(self) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_unit(self) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(ser::Error::custom("expected RawValue"))
    }
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        self.serialize_str(&value.to_string())
    }
}
/// Represents a character escape code in a type-safe manner.
pub enum CharEscape {
    /// An escaped quote `"`
    Quote,
    /// An escaped reverse solidus `\`
    ReverseSolidus,
    /// An escaped solidus `/`
    Solidus,
    /// An escaped backspace character (usually escaped as `\b`)
    Backspace,
    /// An escaped form feed character (usually escaped as `\f`)
    FormFeed,
    /// An escaped line feed character (usually escaped as `\n`)
    LineFeed,
    /// An escaped carriage return character (usually escaped as `\r`)
    CarriageReturn,
    /// An escaped tab character (usually escaped as `\t`)
    Tab,
    /// An escaped ASCII plane control character (usually escaped as
    /// `\u00XX` where `XX` are two hex characters)
    AsciiControl(u8),
}
impl CharEscape {
    #[inline]
    fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
        match escape {
            self::BB => CharEscape::Backspace,
            self::TT => CharEscape::Tab,
            self::NN => CharEscape::LineFeed,
            self::FF => CharEscape::FormFeed,
            self::RR => CharEscape::CarriageReturn,
            self::QU => CharEscape::Quote,
            self::BS => CharEscape::ReverseSolidus,
            self::UU => CharEscape::AsciiControl(byte),
            _ => unreachable!(),
        }
    }
}
/// This trait abstracts away serializing the JSON control characters, which allows the user to
/// optionally pretty print the JSON output.
pub trait Formatter {
    /// Writes a `null` value to the specified writer.
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"null")
    }
    /// Writes a `true` or `false` value to the specified writer.
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let s = if value { b"true" as &[u8] } else { b"false" as &[u8] };
        writer.write_all(s)
    }
    /// Writes an integer value like `-123` to the specified writer.
    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `-123` to the specified writer.
    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `-123` to the specified writer.
    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `-123` to the specified writer.
    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `-123` to the specified writer.
    #[inline]
    fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `123` to the specified writer.
    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `123` to the specified writer.
    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `123` to the specified writer.
    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `123` to the specified writer.
    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes an integer value like `123` to the specified writer.
    #[inline]
    fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes a floating point value like `-31.26e+12` to the specified writer.
    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes a floating point value like `-31.26e+12` to the specified writer.
    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }
    /// Writes a number that has already been rendered to a string.
    #[inline]
    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(value.as_bytes())
    }
    /// Called before each series of `write_string_fragment` and
    /// `write_char_escape`.  Writes a `"` to the specified writer.
    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }
    /// Called after each series of `write_string_fragment` and
    /// `write_char_escape`.  Writes a `"` to the specified writer.
    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }
    /// Writes a string fragment that doesn't need any escaping to the
    /// specified writer.
    #[inline]
    fn write_string_fragment<W>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }
    /// Writes a character escape code to the specified writer.
    #[inline]
    fn write_char_escape<W>(
        &mut self,
        writer: &mut W,
        char_escape: CharEscape,
    ) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        use self::CharEscape::*;
        let s = match char_escape {
            Quote => b"\\\"",
            ReverseSolidus => b"\\\\",
            Solidus => b"\\/",
            Backspace => b"\\b",
            FormFeed => b"\\f",
            LineFeed => b"\\n",
            CarriageReturn => b"\\r",
            Tab => b"\\t",
            AsciiControl(byte) => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                let bytes = &[
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX_DIGITS[(byte >> 4) as usize],
                    HEX_DIGITS[(byte & 0xF) as usize],
                ];
                return writer.write_all(bytes);
            }
        };
        writer.write_all(s)
    }
    /// Called before every array.  Writes a `[` to the specified
    /// writer.
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"[")
    }
    /// Called after every array.  Writes a `]` to the specified
    /// writer.
    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"]")
    }
    /// Called before every array value.  Writes a `,` if needed to
    /// the specified writer.
    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first { Ok(()) } else { writer.write_all(b",") }
    }
    /// Called after every array value.
    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    /// Called before every object.  Writes a `{` to the specified
    /// writer.
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }
    /// Called after every object.  Writes a `}` to the specified
    /// writer.
    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }
    /// Called before every object key.
    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first { Ok(()) } else { writer.write_all(b",") }
    }
    /// Called after every object key.  A `:` should be written to the
    /// specified writer by either this method or
    /// `begin_object_value`.
    #[inline]
    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    /// Called before every object value.  A `:` should be written to
    /// the specified writer by either this method or
    /// `end_object_key`.
    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b":")
    }
    /// Called after every object value.
    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    /// Writes a raw JSON fragment that doesn't need any escaping to the
    /// specified writer.
    #[inline]
    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }
}
/// This structure compacts a JSON value with no extra whitespace.
#[derive(Clone, Debug)]
pub struct CompactFormatter;
impl Formatter for CompactFormatter {}
/// This structure pretty prints a JSON value to make it human readable.
#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}
impl<'a> PrettyFormatter<'a> {
    /// Construct a pretty printer formatter that defaults to using two spaces for indentation.
    pub fn new() -> Self {
        PrettyFormatter::with_indent(b"  ")
    }
    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }
}
impl<'a> Default for PrettyFormatter<'a> {
    fn default() -> Self {
        PrettyFormatter::new()
    }
}
impl<'a> Formatter for PrettyFormatter<'a> {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }
    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;
        if self.has_value {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }
        writer.write_all(b"]")
    }
    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(writer.write_all(if first { b"\n" } else { b",\n" }));
        indent(writer, self.current_indent, self.indent)
    }
    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }
    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;
        if self.has_value {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }
        writer.write_all(b"}")
    }
    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(writer.write_all(if first { b"\n" } else { b",\n" }));
        indent(writer, self.current_indent, self.indent)
    }
    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }
    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}
fn format_escaped_str<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    tri!(formatter.begin_string(writer));
    tri!(format_escaped_str_contents(writer, formatter, value));
    formatter.end_string(writer)
}
fn format_escaped_str_contents<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    let bytes = value.as_bytes();
    let mut start = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }
        if start < i {
            tri!(formatter.write_string_fragment(writer, & value[start..i]));
        }
        let char_escape = CharEscape::from_escape_table(escape, byte);
        tri!(formatter.write_char_escape(writer, char_escape));
        start = i + 1;
    }
    if start == bytes.len() {
        return Ok(());
    }
    formatter.write_string_fragment(writer, &value[start..])
}
const BB: u8 = b'b';
const TT: u8 = b't';
const NN: u8 = b'n';
const FF: u8 = b'f';
const RR: u8 = b'r';
const QU: u8 = b'"';
const BS: u8 = b'\\';
const UU: u8 = b'u';
const __: u8 = 0;
static ESCAPE: [u8; 256] = [
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    BB,
    TT,
    NN,
    UU,
    FF,
    RR,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
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
];
/// Serialize the given data structure as JSON into the IO stream.
///
/// Serialization guarantees it only feeds valid UTF-8 sequences to the writer.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}
/// Serialize the given data structure as pretty-printed JSON into the IO
/// stream.
///
/// Serialization guarantees it only feeds valid UTF-8 sequences to the writer.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::pretty(writer);
    value.serialize(&mut ser)
}
/// Serialize the given data structure as a JSON byte vector.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    tri!(to_writer(& mut writer, value));
    Ok(writer)
}
/// Serialize the given data structure as a pretty-printed JSON byte vector.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    tri!(to_writer_pretty(& mut writer, value));
    Ok(writer)
}
/// Serialize the given data structure as a String of JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = tri!(to_vec(value));
    let string = unsafe { String::from_utf8_unchecked(vec) };
    Ok(string)
}
/// Serialize the given data structure as a pretty-printed String of JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = tri!(to_vec_pretty(value));
    let string = unsafe { String::from_utf8_unchecked(vec) };
    Ok(string)
}
fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        tri!(wr.write_all(s));
    }
    Ok(())
}
#[cfg(test)]
mod tests_llm_16_48_llm_16_48 {
    use crate::error::Error;
    use crate::ser::{CompactFormatter, Formatter, Serializer};
    use crate::ser;
    use serde::Serializer as _;
    use std::fmt::{self, Display};
    use std::io::{self, Write};
    struct TestWriter {
        buffer: Vec<u8>,
        fail: bool,
    }
    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                buffer: Vec::new(),
                fail: false,
            }
        }
        fn with_fail() -> Self {
            TestWriter {
                buffer: Vec::new(),
                fail: true,
            }
        }
    }
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.fail {
                Err(
                    io::Error::new(
                        io::ErrorKind::Other,
                        "Write operation intentionally failed",
                    ),
                )
            } else {
                self.buffer.extend_from_slice(buf);
                Ok(buf.len())
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            if self.fail {
                Err(
                    io::Error::new(
                        io::ErrorKind::Other,
                        "Flush operation intentionally failed",
                    ),
                )
            } else {
                Ok(())
            }
        }
    }
    struct TestDisplay;
    impl Display for TestDisplay {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestDisplay")
        }
    }
    #[test]
    fn test_collect_str_success() {
        let _rug_st_tests_llm_16_48_llm_16_48_rrrruuuugggg_test_collect_str_success = 0;
        let test_writer = TestWriter::new();
        let mut serializer = Serializer::new(test_writer);
        let test_display = TestDisplay;
        let result = serializer.collect_str(&test_display);
        debug_assert!(result.is_ok());
        let output = String::from_utf8(serializer.into_inner().buffer).unwrap();
        debug_assert_eq!(output, "\"TestDisplay\"");
        let _rug_ed_tests_llm_16_48_llm_16_48_rrrruuuugggg_test_collect_str_success = 0;
    }
    #[test]
    fn test_collect_str_fail() {
        let _rug_st_tests_llm_16_48_llm_16_48_rrrruuuugggg_test_collect_str_fail = 0;
        let test_writer = TestWriter::with_fail();
        let mut serializer = Serializer::new(test_writer);
        let test_display = TestDisplay;
        let result = serializer.collect_str(&test_display);
        debug_assert!(result.is_err());
        if let Err(e) = result {
            debug_assert!(e.is_io());
        }
        let _rug_ed_tests_llm_16_48_llm_16_48_rrrruuuugggg_test_collect_str_fail = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_51 {
    use crate::ser::{CompactFormatter, PrettyFormatter, Serializer};
    use serde::Serializer as SerdeSerializer;
    use std::io;
    #[test]
    fn serialize_char_test_compact() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::<_, CompactFormatter>::new(&mut output);
        serializer.serialize_char(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, vec!['a' as u8]);
             }
});    }
    #[test]
    fn serialize_char_test_pretty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::<_, PrettyFormatter>::pretty(&mut output);
        serializer.serialize_char(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, vec!['a' as u8]);
             }
});    }
    #[test]
    fn serialize_char_test_non_ascii() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::<_, CompactFormatter>::new(&mut output);
        serializer.serialize_char(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, String::from("ñ").into_bytes());
             }
});    }
    #[test]
    fn serialize_char_test_buffer_size() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::<_, CompactFormatter>::new(&mut output);
        serializer.serialize_char(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, String::from("💖").into_bytes());
             }
});    }
    #[test]
    fn serialize_char_test_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = FailingWriter;
        let mut serializer = Serializer::<_, CompactFormatter>::new(&mut output);
        debug_assert!(serializer.serialize_char(rug_fuzz_0).is_err());
             }
});    }
    struct FailingWriter;
    impl io::Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "deliberate failure"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
#[cfg(test)]
mod tests_llm_16_52 {
    use crate::error::Error;
    use crate::ser::{CompactFormatter, Serializer};
    use serde::{Serialize, Serializer as SerdeSerializer};
    use std::io;
    use std::f32;
    struct TestWriter {
        output: Vec<u8>,
    }
    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }
        fn get_output(&self) -> String {
            String::from_utf8(self.output.clone()).expect("Output should be valid UTF-8")
        }
    }
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_f32_normal() -> Result<(), Error> {
        let mut output = TestWriter::new();
        let mut serializer = Serializer::new(&mut output);
        serializer.serialize_f32(3.14)?;
        assert_eq!(output.get_output(), "3.14");
        Ok(())
    }
    #[test]
    fn test_serialize_f32_nan() -> Result<(), Error> {
        let mut output = TestWriter::new();
        let mut serializer = Serializer::new(&mut output);
        serializer.serialize_f32(f32::NAN)?;
        assert_eq!(output.get_output(), "null");
        Ok(())
    }
    #[test]
    fn test_serialize_f32_infinity() -> Result<(), Error> {
        let mut output = TestWriter::new();
        let mut serializer = Serializer::new(&mut output);
        serializer.serialize_f32(f32::INFINITY)?;
        assert_eq!(output.get_output(), "null");
        Ok(())
    }
    #[test]
    fn test_serialize_f32_neg_infinity() -> Result<(), Error> {
        let mut output = TestWriter::new();
        let mut serializer = Serializer::new(&mut output);
        serializer.serialize_f32(f32::NEG_INFINITY)?;
        assert_eq!(output.get_output(), "null");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_53 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::error::Error;
    use crate::ser::{CompactFormatter, Serializer as JsonSerializer};
    use std::f64;
    use std::io::Write;
    struct MockWriter(Vec<u8>);
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_f64_finite() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = MockWriter(Vec::new());
        let mut ser = JsonSerializer::new(&mut writer);
        let value = rug_fuzz_0;
        let result = ser.serialize_f64(value);
        debug_assert!(result.is_ok());
        debug_assert_eq!(writer.0, b"10.5");
             }
});    }
    #[test]
    fn test_serialize_f64_nan() {
        let _rug_st_tests_llm_16_53_rrrruuuugggg_test_serialize_f64_nan = 0;
        let mut writer = MockWriter(Vec::new());
        let mut ser = JsonSerializer::new(&mut writer);
        let value = f64::NAN;
        let result = ser.serialize_f64(value);
        debug_assert!(result.is_ok());
        debug_assert_eq!(writer.0, b"null");
        let _rug_ed_tests_llm_16_53_rrrruuuugggg_test_serialize_f64_nan = 0;
    }
    #[test]
    fn test_serialize_f64_infinity() {
        let _rug_st_tests_llm_16_53_rrrruuuugggg_test_serialize_f64_infinity = 0;
        let mut writer = MockWriter(Vec::new());
        let mut ser = JsonSerializer::new(&mut writer);
        let value = f64::INFINITY;
        let result = ser.serialize_f64(value);
        debug_assert!(result.is_ok());
        debug_assert_eq!(writer.0, b"null");
        let _rug_ed_tests_llm_16_53_rrrruuuugggg_test_serialize_f64_infinity = 0;
    }
    #[test]
    fn test_serialize_f64_neg_infinity() {
        let _rug_st_tests_llm_16_53_rrrruuuugggg_test_serialize_f64_neg_infinity = 0;
        let mut writer = MockWriter(Vec::new());
        let mut ser = JsonSerializer::new(&mut writer);
        let value = f64::NEG_INFINITY;
        let result = ser.serialize_f64(value);
        debug_assert!(result.is_ok());
        debug_assert_eq!(writer.0, b"null");
        let _rug_ed_tests_llm_16_53_rrrruuuugggg_test_serialize_f64_neg_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_54_llm_16_54 {
    use crate::ser::{Serializer, Formatter};
    use crate::error::Error;
    use serde::Serializer as SerdeSerializer;
    use std::io::{self, Write};
    #[test]
    fn test_serialize_i128() {
        struct TestFormatter;
        impl Formatter for TestFormatter {
            fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>
            where
                W: io::Write + ?Sized,
            {
                write!(writer, "{}", value)
            }
        }
        let mut output = Vec::new();
        let mut serializer = Serializer::with_formatter(&mut output, TestFormatter);
        let result = SerdeSerializer::serialize_i128(
            &mut serializer,
            -170141183460469231731687303715884105728i128,
        );
        assert!(result.is_ok());
        assert_eq!(output, b"-170141183460469231731687303715884105728");
    }
}
#[cfg(test)]
mod tests_llm_16_57_llm_16_57 {
    use crate::ser::{Serializer, Formatter};
    use serde::Serializer as SerdeSerializer;
    use std::fmt::Write as FmtWrite;
    use std::io::{self, Write as IoWrite};
    use crate::error::Error;
    use crate::ser::CompactFormatter;
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            write!(writer, "{}", value)
        }
    }
    #[test]
    fn test_serialize_i64() -> Result<(), Error> {
        let mut output = Vec::new();
        {
            let mut serializer = Serializer::with_formatter(&mut output, TestFormatter);
            SerdeSerializer::serialize_i64(&mut serializer, 42)?;
        }
        assert_eq!(output, b"42");
        Ok(())
    }
    #[test]
    fn test_serialize_i64_negative() -> Result<(), Error> {
        let mut output = Vec::new();
        {
            let mut serializer = Serializer::with_formatter(&mut output, TestFormatter);
            SerdeSerializer::serialize_i64(&mut serializer, -42)?;
        }
        assert_eq!(output, b"-42");
        Ok(())
    }
    #[test]
    fn test_serialize_i64_zero() -> Result<(), Error> {
        let mut output = Vec::new();
        {
            let mut serializer = Serializer::with_formatter(&mut output, TestFormatter);
            SerdeSerializer::serialize_i64(&mut serializer, 0)?;
        }
        assert_eq!(output, b"0");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_58 {
    use crate::ser::{Serializer, CompactFormatter};
    use serde::Serializer as SerdeSerializer;
    use std::io;
    use crate::Error;
    use std::fmt::Formatter;
    #[test]
    fn test_serialize_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);
        let value = rug_fuzz_0;
        let result = SerdeSerializer::serialize_i8(&mut serializer, value);
        debug_assert!(result.is_ok(), "Expected serialization to be ok");
        debug_assert_eq!(output, value.to_string().as_bytes());
             }
});    }
    #[test]
    fn test_serialize_i8_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);
        let value = -rug_fuzz_0;
        let result = SerdeSerializer::serialize_i8(&mut serializer, value);
        debug_assert!(result.is_ok(), "Expected serialization to be ok");
        debug_assert_eq!(output, value.to_string().as_bytes());
             }
});    }
    #[test]
    fn test_serialize_i8_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);
        let value = rug_fuzz_0;
        let result = SerdeSerializer::serialize_i8(&mut serializer, value);
        debug_assert!(result.is_ok(), "Expected serialization to be ok");
        debug_assert_eq!(output, value.to_string().as_bytes());
             }
});    }
    #[test]
    fn test_serialize_i8_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = FailingWriter;
        let mut serializer = Serializer::new(&mut output);
        let value = rug_fuzz_0;
        let result = SerdeSerializer::serialize_i8(&mut serializer, value);
        debug_assert!(result.is_err(), "Expected serialization to fail");
             }
});    }
    struct FailingWriter;
    impl io::Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "intentional failure"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "intentional failure"))
        }
    }
}
#[cfg(test)]
mod tests_llm_16_59 {
    use super::*;
    use crate::*;
    use serde::ser::{SerializeMap, Serializer as _};
    use std::io::Write;
    struct TestWriter(Vec<u8>);
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_map_empty() -> crate::Result<()> {
        let writer = TestWriter(Vec::new());
        let mut ser = crate::Serializer::new(writer);
        let mut map = ser.serialize_map(Some(0))?;
        map.end()?;
        let output = ser.into_inner().0;
        assert_eq!(output, b"{}");
        Ok(())
    }
    #[test]
    fn test_serialize_map_non_empty() -> crate::Result<()> {
        let writer = TestWriter(Vec::new());
        let mut ser = crate::Serializer::new(writer);
        let map = ser.serialize_map(Some(2))?;
        assert!(matches!(map, crate ::ser::Compound::Map { .. }));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_60_llm_16_60 {
    use crate::{
        ser::{Serializer, Formatter, CompactFormatter},
        error::Error, value::Value, map::Map,
    };
    use serde::Serializer as _;
    #[test]
    fn test_serialize_newtype_struct() -> Result<(), Error> {
        let mut buffer = Vec::new();
        {
            let mut serializer = Serializer::new(&mut buffer);
            let name = "NewtypeStruct";
            let newtype_struct_value = Value::String("Newtype value".to_owned());
            serializer.serialize_newtype_struct(name, &newtype_struct_value)?;
        }
        let serialized_str = String::from_utf8(buffer).expect("Not UTF-8");
        assert_eq!(serialized_str, "\"Newtype value\"");
        Ok(())
    }
    #[test]
    fn test_serialize_newtype_struct_map() -> Result<(), Error> {
        let mut buffer = Vec::new();
        {
            let mut serializer = Serializer::new(&mut buffer);
            let name = "NewtypeStructMap";
            let mut map = Map::new();
            map.insert("key".to_owned(), Value::String("value".to_owned()));
            serializer.serialize_newtype_struct(name, &map)?;
        }
        let serialized_str = String::from_utf8(buffer).expect("Not UTF-8");
        assert_eq!(serialized_str, "{\"key\":\"value\"}");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_62 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer as SerdeSerializer;
    use crate::ser::{Serializer, CompactFormatter};
    use crate::Result;
    use std::io::Write;
    struct TestWriter;
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_none() -> Result<()> {
        let writer = TestWriter;
        let mut serializer = Serializer::new(writer);
        serializer.serialize_none()
    }
}
#[cfg(test)]
mod tests_llm_16_64_llm_16_64 {
    use serde::{Serialize, Serializer};
    use crate::{
        ser::{Serializer as JsonSerializer, CompactFormatter},
        value::Value, map::Map, error::Error,
    };
    use std::io;
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_some_for_value_string() -> Result<(), Error> {
        let mut ser = JsonSerializer::new(DummyWriter);
        let value = Value::String("Hello, World!".into());
        ser.serialize_some(&value)
    }
    #[test]
    fn test_serialize_some_for_value_number() -> Result<(), Error> {
        let mut ser = JsonSerializer::new(DummyWriter);
        let value = Value::Number(123.into());
        ser.serialize_some(&value)
    }
    #[test]
    fn test_serialize_some_for_value_null() -> Result<(), Error> {
        let mut ser = JsonSerializer::new(DummyWriter);
        let value = Value::Null;
        ser.serialize_some(&value)
    }
    #[test]
    fn test_serialize_some_for_map() -> Result<(), Error> {
        let mut ser = JsonSerializer::new(DummyWriter);
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        ser.serialize_some(&map)
    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use serde::Serializer;
    use crate::ser::{self, CompactFormatter, Error};
    use crate::Result;
    use std::io::Write;
    struct TestWriter {
        buffer: Vec<u8>,
    }
    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: Vec::new() }
        }
        fn into_string(self) -> String {
            String::from_utf8(self.buffer).unwrap()
        }
    }
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_str = rug_fuzz_0;
        let expected_output = rug_fuzz_1;
        let writer = TestWriter::new();
        let mut serializer = ser::Serializer::<
            TestWriter,
            CompactFormatter,
        >::new(writer);
        match serializer.serialize_str(test_str) {
            Ok(()) => {
                debug_assert_eq!(serializer.into_inner().into_string(), expected_output)
            }
            Err(e) => panic!("Serialization failed with error: {}", e),
        }
             }
});    }
    #[test]
    fn test_serialize_str_with_special_characters() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_str = rug_fuzz_0;
        let expected_output = rug_fuzz_1;
        let writer = TestWriter::new();
        let mut serializer = ser::Serializer::<
            TestWriter,
            CompactFormatter,
        >::new(writer);
        match serializer.serialize_str(test_str) {
            Ok(()) => {
                debug_assert_eq!(serializer.into_inner().into_string(), expected_output)
            }
            Err(e) => panic!("Serialization failed with error: {}", e),
        }
             }
});    }
    #[test]
    fn test_serialize_str_with_unicode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_str = rug_fuzz_0;
        let expected_output = rug_fuzz_1;
        let writer = TestWriter::new();
        let mut serializer = ser::Serializer::<
            TestWriter,
            CompactFormatter,
        >::new(writer);
        match serializer.serialize_str(test_str) {
            Ok(()) => {
                debug_assert_eq!(serializer.into_inner().into_string(), expected_output)
            }
            Err(e) => panic!("Serialization failed with error: {}", e),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_66 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer as SerdeSerializer;
    use crate::ser::{PrettyFormatter, Serializer, CompactFormatter};
    use std::io::Write;
    struct FakeWriter;
    impl Write for FakeWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
        fn write_all(&mut self, _: &[u8]) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_struct_normal() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_serialize_struct_normal = 0;
        let rug_fuzz_0 = "NormalStruct";
        let rug_fuzz_1 = 0;
        let writer = FakeWriter;
        let mut serializer = Serializer::new(writer);
        let result = serializer.serialize_struct(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_serialize_struct_normal = 0;
    }
    #[test]
    #[cfg(feature = "arbitrary_precision")]
    fn test_serialize_struct_number_token() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_serialize_struct_number_token = 0;
        let rug_fuzz_0 = 0;
        let writer = FakeWriter;
        let mut serializer = Serializer::new(writer);
        let result = serializer.serialize_struct(crate::number::TOKEN, rug_fuzz_0);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_serialize_struct_number_token = 0;
    }
    #[test]
    #[cfg(feature = "raw_value")]
    fn test_serialize_struct_raw_value_token() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_serialize_struct_raw_value_token = 0;
        let rug_fuzz_0 = 0;
        let writer = FakeWriter;
        let mut serializer = Serializer::new(writer);
        let result = serializer.serialize_struct(crate::raw::TOKEN, rug_fuzz_0);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_serialize_struct_raw_value_token = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use serde::ser::{SerializeStructVariant, Serializer as SerdeSerializer};
    use crate::ser::{CompactFormatter, Error, Formatter, Serializer};
    use crate::Serializer as JsonSerializer;
    use std::io;
    struct MockWriter {
        pub output: Vec<u8>,
    }
    impl MockWriter {
        pub fn new() -> MockWriter {
            MockWriter { output: Vec::new() }
        }
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn create_serializer() -> Serializer<MockWriter, CompactFormatter> {
        let mock_writer = MockWriter::new();
        JsonSerializer::new(mock_writer)
    }
    #[test]
    fn test_serialize_struct_variant() -> Result<(), Error> {
        let mut serializer = create_serializer();
        let variant_name = "Variant";
        let struct_variant = serializer
            .serialize_struct_variant("Struct", 0u32, variant_name, 1)?;
        struct_variant.end()?;
        let result = String::from_utf8(serializer.into_inner().output).unwrap();
        assert_eq!(result, format!(r#"{{"Variant":{{"#));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_71_llm_16_71 {
    use crate::ser::{CompactFormatter, Serializer};
    use crate::error::Error;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::{self, Write};
    struct TestWriter {
        written: Vec<u8>,
    }
    impl TestWriter {
        fn new() -> TestWriter {
            TestWriter { written: Vec::new() }
        }
    }
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_u128() -> Result<(), Error> {
        let test_value: u128 = 123456789012345678901234567890123456789u128;
        let mut serializer = Serializer::<
            TestWriter,
            CompactFormatter,
        >::new(TestWriter::new());
        SerdeSerializer::serialize_u128(&mut serializer, test_value)?;
        let result = serializer.into_inner();
        let expected = test_value.to_string().into_bytes();
        assert_eq!(result.written, expected);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_74_llm_16_74 {
    use super::*;
    use crate::*;
    use crate::error::Error;
    use crate::ser::{Formatter, Serializer as JsonSerializer};
    use serde::ser::{self, Serializer};
    use std::io::{self, Write};
    struct MockWriter {
        buf: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> MockWriter {
            MockWriter { buf: Vec::new() }
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            write!(writer, "{}", value)
        }
    }
    #[test]
    fn test_serialize_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u64 = rug_fuzz_0;
        let writer = MockWriter::new();
        let formatter = TestFormatter;
        let mut serializer = JsonSerializer::with_formatter(writer, formatter);
        serializer.serialize_u64(value).unwrap();
        let result = String::from_utf8(serializer.into_inner().buf).unwrap();
        debug_assert_eq!(result, "1234");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_75_llm_16_75 {
    use crate::error::Error;
    use crate::ser::{CompactFormatter, Serializer};
    use serde::Serializer as _;
    use std::vec::Vec;
    #[test]
    fn test_serialize_u8() -> Result<(), Error> {
        let mut output = Vec::new();
        {
            let mut serializer = Serializer::<_, CompactFormatter>::new(&mut output);
            serde::Serializer::serialize_u8(&mut serializer, 123_u8)?;
        }
        assert_eq!(output, b"123");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_76_llm_16_76 {
    use crate::ser::{CompactFormatter, Serializer};
    use crate::error::Error;
    use serde::Serializer as _;
    use std::io::{self, Write};
    struct MockWriter {
        written: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { written: Vec::new() }
        }
        fn contents(&self) -> &[u8] {
            &self.written
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl CompactFormatter {
        pub fn new() -> Self {
            CompactFormatter
        }
    }
    #[test]
    fn test_serialize_unit() -> crate::Result<()> {
        let mut mock_writer = MockWriter::new();
        let mut serializer = Serializer::new(&mut mock_writer);
        serializer.serialize_unit()?;
        assert_eq!(mock_writer.contents(), b"null");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_77_llm_16_77 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer as _Serializer;
    use crate::ser::{CompactFormatter, Serializer};
    use std::io::{self, Write};
    struct MockWriter {
        written: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { written: Vec::new() }
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_unit_struct() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_serialize_unit_struct = 0;
        let rug_fuzz_0 = "MyUnitStruct";
        let mock_writer = MockWriter::new();
        let mut serializer = Serializer::new(mock_writer);
        let result = _Serializer::serialize_unit_struct(&mut serializer, rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(serializer.into_inner().written, b"null");
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_serialize_unit_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_234 {
    use crate::error::Error;
    use crate::ser::{Compound, Formatter, PrettyFormatter, Serializer, State};
    use serde::{ser::SerializeMap, Serialize};
    use std::io::{self, Write};
    #[derive(Default)]
    struct MockWriter {
        buffer: Vec<u8>,
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_end_with_empty_state() -> Result<(), Error> {
        let writer = MockWriter::default();
        let formatter = PrettyFormatter::new();
        let mut ser = Serializer::with_formatter(writer, formatter);
        let compound = Compound::Map {
            ser: &mut ser,
            state: State::Empty,
        };
        assert!(SerializeMap::end(compound).is_ok());
        Ok(())
    }
    #[test]
    fn test_end_with_non_empty_state() -> Result<(), Error> {
        let mut writer = MockWriter::default();
        let formatter = PrettyFormatter::new();
        let mut ser = Serializer::with_formatter(&mut writer, formatter);
        let compound = Compound::Map {
            ser: &mut ser,
            state: State::First,
        };
        assert!(SerializeMap::end(compound).is_err());
        let mut ser = Serializer::with_formatter(
            MockWriter::default(),
            PrettyFormatter::new(),
        );
        let compound = Compound::Map {
            ser: &mut ser,
            state: State::Rest,
        };
        assert!(SerializeMap::end(compound).is_err());
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_239_llm_16_239 {
    use serde::ser::{SerializeStruct, Serializer as _};
    use serde::Serialize;
    use crate::error::Result;
    use crate::ser::{Compound, Serializer, State};
    use std::io::{self, Write};
    struct MockWriter;
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct NoOpFormatter;
    impl crate::ser::Formatter for NoOpFormatter {
        fn begin_array<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn end_array<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn begin_array_value<W>(
            &mut self,
            _writer: &mut W,
            _first: bool,
        ) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn begin_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn end_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn begin_object_key<W>(
            &mut self,
            _writer: &mut W,
            _first: bool,
        ) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn begin_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
        fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: Write + ?Sized,
        {
            Ok(())
        }
    }
    #[test]
    fn test_end_map_empty_state() -> Result<()> {
        let writer = MockWriter;
        let formatter = NoOpFormatter;
        let mut ser = Serializer::with_formatter(writer, formatter);
        let compound_map = Compound::Map {
            ser: &mut ser,
            state: State::Empty,
        };
        assert_eq!(compound_map.end() ?, ());
        Ok(())
    }
    #[test]
    fn test_end_map_non_empty_state() -> Result<()> {
        let writer = MockWriter;
        let formatter = NoOpFormatter;
        let mut ser = Serializer::with_formatter(writer, formatter);
        let compound_map = Compound::Map {
            ser: &mut ser,
            state: State::First,
        };
        assert!(compound_map.end().is_ok());
        Ok(())
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_end_number() -> Result<()> {
        let writer = MockWriter;
        let formatter = NoOpFormatter;
        let mut ser = Serializer::with_formatter(writer, formatter);
        let compound_number = Compound::Number { ser: &mut ser };
        assert_eq!(compound_number.end() ?, ());
        Ok(())
    }
    #[cfg(feature = "raw_value")]
    #[test]
    fn test_end_raw_value() -> Result<()> {
        let writer = MockWriter;
        let formatter = NoOpFormatter;
        let mut ser = Serializer::with_formatter(writer, formatter);
        let compound_raw_value = Compound::RawValue {
            ser: &mut ser,
        };
        assert_eq!(compound_raw_value.end() ?, ());
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_242_llm_16_242 {
    use serde::{Serialize, Serializer, ser::SerializeStructVariant};
    use crate::ser::{Compound, State};
    use crate::error::Error;
    use crate::value::Value;
    use std::io;
    #[derive(Serialize)]
    struct TestStruct {
        a: u32,
        b: String,
    }
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn serialize_field_map() {
        let _rug_st_tests_llm_16_242_llm_16_242_rrrruuuugggg_serialize_field_map = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "example";
        let rug_fuzz_2 = "a";
        let rug_fuzz_3 = "b";
        let mut writer = DummyWriter;
        let mut ser = crate::Serializer::new(writer);
        let mut compound = Compound::Map {
            ser: &mut ser,
            state: State::Empty,
        };
        let test_struct = TestStruct {
            a: rug_fuzz_0,
            b: rug_fuzz_1.to_string(),
        };
        let result = compound.serialize_field(rug_fuzz_2, &test_struct.a);
        debug_assert!(result.is_ok());
        let result = compound.serialize_field(rug_fuzz_3, &test_struct.b);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_242_llm_16_242_rrrruuuugggg_serialize_field_map = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    #[should_panic(expected = "unreachable")]
    fn serialize_field_number() {
        let _rug_st_tests_llm_16_242_llm_16_242_rrrruuuugggg_serialize_field_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "example";
        let rug_fuzz_2 = "a";
        let mut writer = DummyWriter;
        let mut ser = crate::Serializer::new(writer);
        let mut compound = Compound::Number { ser: &mut ser };
        let test_struct = TestStruct {
            a: rug_fuzz_0,
            b: rug_fuzz_1.to_string(),
        };
        compound.serialize_field(rug_fuzz_2, &test_struct.a).unwrap();
        let _rug_ed_tests_llm_16_242_llm_16_242_rrrruuuugggg_serialize_field_number = 0;
    }
    #[cfg(feature = "raw_value")]
    #[test]
    #[should_panic(expected = "unreachable")]
    fn serialize_field_raw_value() {
        let _rug_st_tests_llm_16_242_llm_16_242_rrrruuuugggg_serialize_field_raw_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "example";
        let rug_fuzz_2 = "b";
        let mut writer = DummyWriter;
        let mut ser = crate::Serializer::new(writer);
        let mut compound = Compound::RawValue {
            ser: &mut ser,
        };
        let test_struct = TestStruct {
            a: rug_fuzz_0,
            b: rug_fuzz_1.to_string(),
        };
        compound.serialize_field(rug_fuzz_2, &test_struct.b).unwrap();
        let _rug_ed_tests_llm_16_242_llm_16_242_rrrruuuugggg_serialize_field_raw_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_243_llm_16_243 {
    use super::*;
    use crate::*;
    use crate::error::Error;
    use crate::ser::{Compound, Formatter, Serializer, State};
    use serde::ser::{Serialize, SerializeSeq};
    use std::io::{self, Write};
    struct MockWriter;
    impl Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_end_empty_map() {
        let _rug_st_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_end_empty_map = 0;
        let mock_writer = MockWriter;
        let formatter = CompactFormatter;
        let mut serializer = Serializer::with_formatter(mock_writer, formatter);
        let compound = Compound::Map {
            ser: &mut serializer,
            state: State::Empty,
        };
        let result: Result<()> = compound.end();
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_end_empty_map = 0;
    }
    #[test]
    fn test_end_non_empty_map() {
        let _rug_st_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_end_non_empty_map = 0;
        let mock_writer = MockWriter;
        let formatter = CompactFormatter;
        let mut serializer = Serializer::with_formatter(mock_writer, formatter);
        let compound = Compound::Map {
            ser: &mut serializer,
            state: State::Rest,
        };
        let result: Result<()> = compound.end();
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_end_non_empty_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_246 {
    use crate::ser::{Compound, Formatter, PrettyFormatter, Serializer};
    use crate::value::Value;
    use serde::{ser::Serialize, ser::SerializeTupleStruct};
    use std::io;
    struct TestStruct {
        a: i32,
        b: String,
    }
    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut tuple = serializer.serialize_tuple_struct("TestStruct", 2)?;
            tuple.serialize_field(&self.a)?;
            tuple.serialize_field(&self.b)?;
            tuple.end()
        }
    }
    #[test]
    fn test_serialize_field() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_struct = TestStruct {
            a: rug_fuzz_0,
            b: rug_fuzz_1.to_owned(),
        };
        let mut buffer = Vec::new();
        let formatter = PrettyFormatter::new();
        let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
        let mut compound = Compound::Map {
            ser: &mut serializer,
            state: crate::ser::State::First,
        };
        let _ = compound.serialize_field(&test_struct.a).unwrap();
        let _ = compound.serialize_field(&test_struct.b).unwrap();
        debug_assert_eq!(
            std::str::from_utf8(& buffer).unwrap(), "42\"Answer to everything\""
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_249 {
    use serde::ser::{Serializer, Serialize};
    use std::fmt::{self, Display};
    use crate::ser::{Serializer as JsonSerializer, MapKeySerializer};
    use crate::error::Error as JsonError;
    use std::io;
    struct Displayable;
    impl Display for Displayable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "displayable")
        }
    }
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl Serialize for Displayable {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_str(&self)
        }
    }
    #[test]
    fn test_collect_str_for_displayable() {
        let _rug_st_tests_llm_16_249_rrrruuuugggg_test_collect_str_for_displayable = 0;
        let mut json_serializer = JsonSerializer::new(TestWriter);
        let mut map_key_serializer = MapKeySerializer {
            ser: &mut json_serializer,
        };
        let displayable = Displayable;
        let result = map_key_serializer.collect_str(&displayable);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_249_rrrruuuugggg_test_collect_str_for_displayable = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_255_llm_16_255 {
    use serde::Serializer;
    use crate::error::Error;
    use crate::ser::{MapKeySerializer, Serializer as JsonSerializer};
    use std::io::{self, Write};
    struct TestWriter;
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn serialize_i128_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut ser = JsonSerializer::new(TestWriter);
        let mut key_serializer = MapKeySerializer { ser: &mut ser };
        let result = key_serializer.serialize_i128(rug_fuzz_0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_257_llm_16_257 {
    use serde::Serializer;
    use crate::ser::{Formatter, MapKeySerializer, Serializer as JsonSerializer};
    use crate::error::Error;
    use std::io::{self, Write};
    struct FakeFormatter;
    impl Formatter for FakeFormatter {
        fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            writer.write_all(b"\"").map(|_| ())
        }
        fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            writer.write_all(b"\"").map(|_| ())
        }
        fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            write!(writer, "{}", value)
        }
    }
    #[test]
    fn test_serialize_i32() -> Result<(), Error> {
        let mut vec = Vec::new();
        let mut serializer = JsonSerializer::with_formatter(&mut vec, FakeFormatter);
        let map_key_serializer = MapKeySerializer {
            ser: &mut serializer,
        };
        map_key_serializer.serialize_i32(42)?;
        assert_eq!(String::from_utf8(vec).unwrap(), "\"42\"");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_259_llm_16_259 {
    use serde::Serializer;
    use crate::error::Error;
    use crate::ser::{CompactFormatter, MapKeySerializer, Serializer as JsonSerializer};
    use std::io::Write;
    struct MockWriter(Vec<u8>);
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.write(buf)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.0.flush()
        }
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter(Vec::new())
        }
    }
    #[test]
    fn test_serialize_i8() -> Result<(), Error> {
        let mut mock_writer = MockWriter::new();
        let formatter = CompactFormatter;
        let mut serializer = JsonSerializer::with_formatter(&mut mock_writer, formatter);
        let map_key_serializer = MapKeySerializer {
            ser: &mut serializer,
        };
        let result = map_key_serializer.serialize_i8(42);
        assert!(result.is_ok());
        assert_eq!(mock_writer.0, b"\"42\"".to_vec());
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_266 {
    use crate::ser::{self, Formatter, Serializer, MapKeySerializer};
    use serde::Serializer as _;
    use std::io::{self, Write};
    struct TestFormatter;
    impl Formatter for TestFormatter {}
    struct TestWriter;
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_str() -> crate::Result<()> {
        let mut writer = TestWriter;
        let formatter = TestFormatter;
        let mut serializer = Serializer::with_formatter(writer, formatter);
        let map_key_serializer = MapKeySerializer {
            ser: &mut serializer,
        };
        let result = map_key_serializer.serialize_str("test_key");
        assert!(result.is_ok());
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_270_llm_16_270 {
    use super::*;
    use crate::*;
    use serde::{
        ser::{Impossible, Serializer},
        Serialize,
    };
    use crate::error::Error;
    use crate::ser::{Formatter, MapKeySerializer, Serializer as JsonSerializer};
    struct FakeWriter;
    impl std::io::Write for FakeWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    struct FakeFormatter;
    impl Formatter for FakeFormatter {
        fn write_null<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: std::io::Write + ?Sized,
        {
            write!(writer, "null")
        }
    }
    impl FakeFormatter {
        pub fn new() -> Self {
            FakeFormatter
        }
    }
    #[test]
    fn test_serialize_tuple_struct() {
        let _rug_st_tests_llm_16_270_llm_16_270_rrrruuuugggg_test_serialize_tuple_struct = 0;
        let rug_fuzz_0 = "MyTupleStruct";
        let rug_fuzz_1 = 3;
        let mut writer = FakeWriter;
        let formatter = FakeFormatter::new();
        let mut serializer = JsonSerializer::with_formatter(writer, formatter);
        let map_key_serializer = MapKeySerializer {
            ser: &mut serializer,
        };
        let result = map_key_serializer.serialize_tuple_struct(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(
            result.is_err(), "Expected serialize_tuple_struct to return error."
        );
        let _rug_ed_tests_llm_16_270_llm_16_270_rrrruuuugggg_test_serialize_tuple_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_271 {
    use serde::ser::{Impossible, SerializeTupleVariant};
    use serde::Serializer;
    use crate::Error;
    use crate::ser::{Formatter, Serializer as JsonSerializer};
    use std::io;
    struct MockFormatter;
    impl Formatter for MockFormatter {}
    fn new_map_key_serializer() -> crate::ser::MapKeySerializer<
        'static,
        Vec<u8>,
        MockFormatter,
    > {
        let writer = Vec::new();
        let formatter = MockFormatter;
        let serializer = JsonSerializer::with_formatter(writer, formatter);
        crate::ser::MapKeySerializer {
            ser: Box::leak(Box::new(serializer)),
        }
    }
    #[test]
    fn test_serialize_tuple_variant_should_fail() {
        let map_key_serializer = new_map_key_serializer();
        let result: Result<Impossible<(), Error>, _> = map_key_serializer
            .serialize_tuple_variant("name", 0, "variant", 0);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_274 {
    use crate::ser::{MapKeySerializer, Serializer, Formatter, CompactFormatter};
    use crate::error::Error;
    use serde::Serializer as SerdeSerializer;
    use std::io;
    struct MyFormatter;
    impl Formatter for MyFormatter {
        fn write_null<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
        {
            unimplemented!()
        }
        fn write_bool<W: ?Sized>(
            &mut self,
            _writer: &mut W,
            _value: bool,
        ) -> io::Result<()>
        where
            W: io::Write,
        {
            unimplemented!()
        }
        fn write_u8<W: ?Sized>(&mut self, _writer: &mut W, _value: u8) -> io::Result<()>
        where
            W: io::Write,
        {
            unimplemented!()
        }
        fn write_u16<W: ?Sized>(
            &mut self,
            _writer: &mut W,
            _value: u16,
        ) -> io::Result<()>
        where
            W: io::Write,
        {
            unimplemented!()
        }
        fn write_u32<W: ?Sized>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
        where
            W: io::Write,
        {
            write!(writer, "{}", value)
        }
    }
    #[test]
    fn test_serialize_u32() -> Result<(), Error> {
        let mut output = Vec::new();
        {
            let mut serializer = Serializer::with_formatter(&mut output, MyFormatter);
            let map_serializer = MapKeySerializer {
                ser: &mut serializer,
            };
            map_serializer.serialize_u32(123)?;
        }
        assert_eq!(output, b"123");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_276_llm_16_276 {
    use serde::Serializer;
    use crate::error::Error;
    use crate::ser::{Formatter, Serializer as JsonSerializer, CompactFormatter};
    use std::{fmt, io};
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_string<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            Ok(())
        }
        fn end_string<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            Ok(())
        }
        fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            write!(writer, "{}", value)
        }
    }
    struct TestWriter(Vec<u8>);
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let writer = TestWriter(Vec::new());
        let formatter = TestFormatter;
        let mut serializer = JsonSerializer::with_formatter(writer, formatter);
        let value: u8 = rug_fuzz_0;
        let map_key_serializer = crate::ser::MapKeySerializer {
            ser: &mut serializer,
        };
        let result = map_key_serializer.serialize_u8(value);
        debug_assert!(result.is_ok(), "Serialization of u8 as map key should be OK.");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_280_llm_16_280 {
    use super::*;
    use crate::*;
    use crate::ser::PrettyFormatter;
    use crate::value::Value;
    use std::io::{self, Write};
    use std::fmt;
    #[track_caller]
    fn io_error(one: fmt::Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, one)
    }
    struct MockWriter<'a> {
        output: &'a mut Vec<u8>,
    }
    impl<'a> Write for MockWriter<'a> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct WriterFormatter<'a, 'b: 'a> {
        inner: &'a mut fmt::Formatter<'b>,
    }
    impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = unsafe { std::str::from_utf8_unchecked(buf) };
            self.inner.write_str(s).map_err(io_error).map(|_| buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_begin_array() {
        let mut output = Vec::new();
        let mut formatter = PrettyFormatter::with_indent(b"    ");
        let mut writer = MockWriter { output: &mut output };
        formatter.begin_array(&mut writer).unwrap();
        assert_eq!(formatter.current_indent, 1);
        assert_eq!(formatter.has_value, false);
        assert_eq!(& output, b"[");
    }
}
#[cfg(test)]
mod tests_llm_16_281 {
    use super::*;
    use crate::*;
    use crate::ser::{Formatter, PrettyFormatter};
    use std::fmt;
    use std::io::{self, Write};
    struct MockWriter {
        written: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { written: Vec::new() }
        }
        fn content(&self) -> &[u8] {
            &self.written
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn indent(writer: &mut impl Write, n: usize, s: &[u8]) -> io::Result<()> {
        for _ in 0..n {
            writer.write_all(s)?;
        }
        Ok(())
    }
    #[test]
    fn test_begin_array_value() -> io::Result<()> {
        let indent_str = b"  ";
        let mut formatter = PrettyFormatter::with_indent(indent_str);
        let mut writer = MockWriter::new();
        formatter.begin_array_value(&mut writer, true)?;
        assert_eq!(writer.content(), b"\n");
        indent(&mut writer, formatter.current_indent, indent_str)?;
        let expected_first_value = b"\n  ";
        assert_eq!(writer.content(), expected_first_value);
        formatter.begin_array_value(&mut writer, false)?;
        assert_eq!(writer.content(), b"\n  ,\n");
        indent(&mut writer, formatter.current_indent, indent_str)?;
        let expected_second_value = b"\n  ,\n  ";
        assert_eq!(writer.content(), expected_second_value);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_282 {
    use super::*;
    use crate::*;
    use crate::ser::{Formatter, PrettyFormatter};
    use std::fmt;
    use std::io::{self, Write};
    struct MockWriter {
        buffer: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
        fn buffer_as_string(&self) -> String {
            String::from_utf8(self.buffer.clone()).unwrap()
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_begin_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let indent = rug_fuzz_0;
        let mut formatter = PrettyFormatter::with_indent(indent.as_bytes());
        let mut writer = MockWriter::new();
        formatter.begin_object(&mut writer).unwrap();
        let expected = rug_fuzz_1;
        debug_assert_eq!(writer.buffer_as_string(), expected);
        debug_assert_eq!(formatter.current_indent, 1);
        debug_assert_eq!(formatter.has_value, false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_283 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use crate::ser::PrettyFormatter;
    use std::fmt;
    use std::io;
    struct MockWriter {
        output: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: Vec::new() }
        }
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn indent(writer: &mut fmt::Formatter<'_>, n: usize, s: &[u8]) -> io::Result<()> {
        for _ in 0..n {
            tri!(
                writer.write_str(unsafe { std::str::from_utf8_unchecked(s) })
                .map_err(io_error)
            );
        }
        Ok(())
    }
    fn io_error(error: fmt::Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    }
    #[test]
    fn test_begin_object_key() {
        let indent = b"  ";
        let mut formatter = PrettyFormatter::with_indent(indent);
        let mut output = MockWriter::new();
        formatter.begin_object_key(&mut output, true).unwrap();
        assert_eq!(output.output, b"\n");
        output.output.clear();
        formatter.begin_object_key(&mut output, false).unwrap();
        assert_eq!(output.output, b",\n");
        formatter.current_indent = 1;
        output.output.clear();
        formatter.begin_object_key(&mut output, false).unwrap();
        let expected = b",\n  ";
        assert_eq!(output.output, expected);
        formatter.current_indent = 2;
        output.output.clear();
        formatter.begin_object_key(&mut output, false).unwrap();
        let expected = b",\n    ";
        assert_eq!(output.output, expected);
    }
}
#[cfg(test)]
mod tests_llm_16_284_llm_16_284 {
    use super::*;
    use crate::*;
    use crate::ser::PrettyFormatter;
    use std::io::{self, Write};
    #[test]
    fn test_begin_object_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Vec::new();
        let mut formatter = PrettyFormatter::new();
        formatter.begin_object_value(&mut buffer).expect(rug_fuzz_0);
        debug_assert_eq!(buffer, b": ");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_285_llm_16_285 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use crate::ser::PrettyFormatter;
    use std::fmt::{self, Write as FmtWrite};
    use std::io::{self, Write as IoWrite};
    struct MockWriterFormatter {
        output: Vec<u8>,
    }
    impl MockWriterFormatter {
        fn new() -> Self {
            MockWriterFormatter {
                output: Vec::new(),
            }
        }
    }
    impl IoWrite for MockWriterFormatter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn indent(writer: &mut MockWriterFormatter, n: usize, s: &[u8]) -> io::Result<()> {
        for _ in 0..n {
            writer.write_all(s)?;
        }
        Ok(())
    }
    #[test]
    fn test_end_array_no_values() {
        let indent = b"  ";
        let mut writer_formatter = MockWriterFormatter::new();
        let mut pretty_formatter = PrettyFormatter::with_indent(indent);
        assert!(pretty_formatter.begin_array(& mut writer_formatter).is_ok());
        assert!(pretty_formatter.end_array(& mut writer_formatter).is_ok());
        assert_eq!(writer_formatter.output, b"[]");
    }
    #[test]
    fn test_end_array_with_values() {
        let indent = b"  ";
        let mut writer_formatter = MockWriterFormatter::new();
        let mut pretty_formatter = PrettyFormatter::with_indent(indent);
        assert!(pretty_formatter.begin_array(& mut writer_formatter).is_ok());
        assert!(
            pretty_formatter.begin_array_value(& mut writer_formatter, true).is_ok()
        );
        pretty_formatter.end_array_value(&mut writer_formatter).unwrap();
        assert!(pretty_formatter.end_array(& mut writer_formatter).is_ok());
        let expected = format!("[\n{}]", "  ".repeat(pretty_formatter.current_indent));
        assert_eq!(writer_formatter.output, expected.as_bytes());
    }
}
#[cfg(test)]
mod tests_llm_16_286 {
    use super::*;
    use crate::*;
    use crate::ser::PrettyFormatter;
    use std::io::{self, Write};
    struct MockWriterFormatter(Vec<u8>);
    impl Write for MockWriterFormatter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_pretty_formatter_end_array_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 4], bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let indent = rug_fuzz_0;
        let mut formatter = PrettyFormatter::with_indent(indent);
        formatter.has_value = rug_fuzz_1;
        let mut writer = MockWriterFormatter(Vec::new());
        formatter.end_array_value(&mut writer).unwrap();
        debug_assert!(
            formatter.has_value, "has_value should be true after calling end_array_value"
        );
        debug_assert!(
            writer.0.is_empty(), "Writer should not have any content written to it"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_288 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use crate::ser::PrettyFormatter;
    use std::fmt::{self, Write};
    struct TestWriter(String);
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.push_str(&String::from_utf8_lossy(buf));
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_end_object_value() -> io::Result<()> {
        let mut formatter = PrettyFormatter::with_indent(b"    ");
        let mut test_writer = TestWriter(String::new());
        formatter.end_object_value(&mut test_writer)?;
        assert_eq!(formatter.has_value, true);
        assert_eq!(test_writer.0, "");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_289_llm_16_289 {
    use crate::ser::PrettyFormatter;
    use std::default::Default;
    use std::fmt::Debug;
    #[test]
    fn pretty_formatter_default() {
        let _rug_st_tests_llm_16_289_llm_16_289_rrrruuuugggg_pretty_formatter_default = 0;
        let formatter = PrettyFormatter::default();
        debug_assert_eq!(formatter.current_indent, 0);
        debug_assert_eq!(formatter.has_value, false);
        debug_assert_eq!(formatter.indent, b"  ");
        let _rug_ed_tests_llm_16_289_llm_16_289_rrrruuuugggg_pretty_formatter_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_558 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::io;
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            writer.write_all(b"[")
        }
    }
    struct MockWriter {
        contents: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { contents: Vec::new() }
        }
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.contents.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_begin_array() {
        let _rug_st_tests_llm_16_558_rrrruuuugggg_test_begin_array = 0;
        let mut formatter = TestFormatter;
        let mut writer = MockWriter::new();
        formatter.begin_array(&mut writer).unwrap();
        debug_assert_eq!(writer.contents, b"[");
        let _rug_ed_tests_llm_16_558_rrrruuuugggg_test_begin_array = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_559_llm_16_559 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::io::{self, Write};
    use std::str;
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            if first { Ok(()) } else { writer.write_all(b",") }
        }
    }
    struct MockWriter {
        output: Vec<u8>,
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            Write::write(self, s.as_bytes()).map(|_| ()).map_err(|_| fmt::Error)
        }
    }
    #[test]
    fn test_begin_array_value_first() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = MockWriter { output: vec![] };
        let mut formatter = TestFormatter;
        let first = rug_fuzz_0;
        formatter.begin_array_value(&mut output, first).expect(rug_fuzz_1);
        debug_assert_eq!(output.output, b"");
             }
});    }
    #[test]
    fn test_begin_array_value_not_first() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = MockWriter { output: vec![] };
        let mut formatter = TestFormatter;
        let first = rug_fuzz_0;
        formatter.begin_array_value(&mut output, first).expect(rug_fuzz_1);
        debug_assert_eq!(output.output, b",");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_560_llm_16_560 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::fmt::Write;
    struct MockWriterFormatter<'a> {
        inner: &'a mut String,
    }
    impl<'a> std::io::Write for MockWriterFormatter<'a> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf)
                .map_err(|_| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid UTF8",
                ))?;
            self.inner
                .write_str(s)
                .map_err(|_| std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Couldn't write",
                ))?;
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            writer.write_all(b"{")
        }
    }
    #[test]
    fn test_begin_object() -> std::io::Result<()> {
        let mut output = String::new();
        let mut writer = MockWriterFormatter {
            inner: &mut output,
        };
        let mut formatter = TestFormatter;
        formatter.begin_object(&mut writer)?;
        assert_eq!(output, "{");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_561 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::io;
    use std::str;
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            if first { Ok(()) } else { writer.write_all(b",") }
        }
    }
    struct WriterFormatter<'a, 'b: 'a> {
        inner: &'a mut fmt::Formatter<'b>,
    }
    impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = unsafe { str::from_utf8_unchecked(buf) };
            self.inner.write_str(s).map_err(io_error)?;
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn io_error(err: fmt::Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
    #[test]
    fn test_begin_object_key_first() {
        let mut formatter = TestFormatter;
        let mut output = Vec::new();
        let result = formatter.begin_object_key(&mut output, true);
        assert!(result.is_ok(), "Should be Ok for first key");
        assert!(output.is_empty(), "Should not write anything for first key");
    }
    #[test]
    fn test_begin_object_key_not_first() {
        let mut formatter = TestFormatter;
        let mut output = Vec::new();
        let result = formatter.begin_object_key(&mut output, false);
        assert!(result.is_ok(), "Should be Ok for not first key");
        assert_eq!(output, b",", "Should write a comma for not first key");
    }
}
#[cfg(test)]
mod tests_llm_16_563_llm_16_563 {
    use crate::ser::{CharEscape, Formatter};
    use std::io;
    use std::fmt;
    struct MockWriter {
        buf: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> MockWriter {
            MockWriter { buf: Vec::new() }
        }
        fn as_str(&self) -> &str {
            std::str::from_utf8(&self.buf).unwrap()
        }
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_string_fragment<W>(&mut self, _: &mut W, _: &str) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            unimplemented!()
        }
        fn write_char_escape<W>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            unimplemented!()
        }
    }
    #[test]
    fn test_begin_string() -> io::Result<()> {
        let mut formatter = TestFormatter;
        let mut writer = MockWriter::new();
        formatter.begin_string(&mut writer)?;
        assert_eq!("\"", writer.as_str());
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_566 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::io;
    use std::str;
    struct TestFormatter;
    impl Formatter for TestFormatter {}
    struct MockWriter {
        content: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { content: Vec::new() }
        }
        fn content_as_str(&self) -> &str {
            str::from_utf8(&self.content).unwrap()
        }
    }
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.content.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_end_object() -> io::Result<()> {
        let mut formatter = TestFormatter;
        let mut writer = MockWriter::new();
        formatter.end_object(&mut writer)?;
        assert_eq!(writer.content_as_str(), "}");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_567_llm_16_567 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::io::{self, Write};
    struct DummyFormatter;
    impl Formatter for DummyFormatter {}
    struct WriterFormatter<'a> {
        inner: &'a mut String,
    }
    impl<'a> Write for WriterFormatter<'a> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = match std::str::from_utf8(buf) {
                Ok(v) => v,
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            };
            self.inner.push_str(s);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_end_object_key() -> io::Result<()> {
        let mut dummy_formatter = DummyFormatter;
        let mut output = String::new();
        let mut writer = WriterFormatter {
            inner: &mut output,
        };
        dummy_formatter.end_object_key(&mut writer)?;
        assert_eq!(output, "");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_568_llm_16_568 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::io;
    use std::io::Write;
    use std::str;
    struct TestFormatter;
    impl Formatter for TestFormatter {}
    struct TestWriter;
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_end_object_value() -> io::Result<()> {
        let mut test_writer = TestWriter;
        let mut test_formatter = TestFormatter;
        test_formatter.end_object_value(&mut test_writer)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_572 {
    use crate::ser::{Formatter, CompactFormatter};
    use std::io;
    struct TestFormatter {
        compact: CompactFormatter,
    }
    impl TestFormatter {
        fn new() -> TestFormatter {
            TestFormatter {
                compact: CompactFormatter,
            }
        }
    }
    impl Formatter for TestFormatter {
        fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.compact.write_f32(writer, value)
        }
    }
    #[test]
    fn test_write_f32() -> io::Result<()> {
        let mut output = Vec::new();
        let mut formatter = TestFormatter::new();
        formatter.write_f32(&mut output, 123.456f32)?;
        let output_str = std::str::from_utf8(&output).expect("Not UTF-8");
        assert_eq!(output_str, "123.456");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_573_llm_16_573 {
    use super::*;
    use crate::*;
    use std::io::Write;
    use std::fmt;
    use crate::ser::Formatter;
    use std::io;
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            let mut buffer = ryu::Buffer::new();
            let s = buffer.format_finite(value);
            writer.write_all(s.as_bytes())
        }
    }
    #[test]
    fn test_write_f64() -> io::Result<()> {
        let value = -31.26e+12;
        let not_nan_inf = "-31.26e12";
        let mut buffer = Vec::new();
        let mut writer = io::Cursor::new(&mut buffer);
        let mut formatter = TestFormatter;
        formatter.write_f64(&mut writer, value)?;
        let result = std::str::from_utf8(&buffer).unwrap();
        assert_eq!(result, not_nan_inf);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_576_llm_16_576 {
    use crate::ser::Formatter;
    use std::io;
    use std::fmt::{self, Write};
    struct WriterFormatter<'a> {
        inner: &'a mut String,
    }
    impl<'a> io::Write for WriterFormatter<'a> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = std::str::from_utf8(buf)
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
            self.inner
                .write_str(s)
                .map_err(|_| io::Error::from(io::ErrorKind::WriteZero))?;
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct MockFormatter;
    impl<'a> Formatter for MockFormatter {
        fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            let mut buffer = itoa::Buffer::new();
            let s = buffer.format(value);
            writer.write_all(s.as_bytes())
        }
    }
    #[test]
    fn test_write_i32() -> io::Result<()> {
        let val: i32 = -123;
        let mut buffer = String::new();
        let mut formatter = MockFormatter;
        let mut writer_formatter = WriterFormatter {
            inner: &mut buffer,
        };
        formatter.write_i32(&mut writer_formatter, val)?;
        assert_eq!(buffer, "-123");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_578_llm_16_578 {
    use crate::ser::Formatter;
    use std::fmt::Write as FmtWrite;
    use std::io::Write as IoWrite;
    use std::str;
    use std::io;
    struct MockWriterFormatter<'a> {
        output: &'a mut String,
    }
    impl<'a> IoWrite for MockWriterFormatter<'a> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = str::from_utf8(buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            self.output
                .write_str(s)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
        where
            W: ?Sized + IoWrite,
        {
            let mut buffer = itoa::Buffer::new();
            let s = buffer.format(value);
            writer.write_all(s.as_bytes())
        }
    }
    #[test]
    fn test_write_i8_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let mut writer = MockWriterFormatter {
            output: &mut output,
        };
        let mut formatter = TestFormatter;
        formatter.write_i8(&mut writer, rug_fuzz_0).unwrap();
        debug_assert_eq!(& output, "123");
             }
});    }
    #[test]
    fn test_write_i8_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let mut writer = MockWriterFormatter {
            output: &mut output,
        };
        let mut formatter = TestFormatter;
        formatter.write_i8(&mut writer, -rug_fuzz_0).unwrap();
        debug_assert_eq!(& output, "-123");
             }
});    }
    #[test]
    fn test_write_i8_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let mut writer = MockWriterFormatter {
            output: &mut output,
        };
        let mut formatter = TestFormatter;
        formatter.write_i8(&mut writer, rug_fuzz_0).unwrap();
        debug_assert_eq!(& output, "0");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_579 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::io::{self, Write};
    struct MockWrite(Vec<u8>);
    impl Write for MockWrite {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            writer.write_all(b"null")
        }
    }
    #[test]
    fn test_write_null() {
        let _rug_st_tests_llm_16_579_rrrruuuugggg_test_write_null = 0;
        let mut writer = MockWrite(Vec::new());
        let mut formatter = MockFormatter;
        formatter.write_null(&mut writer).unwrap();
        debug_assert_eq!(writer.0, b"null");
        let _rug_ed_tests_llm_16_579_rrrruuuugggg_test_write_null = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_583_llm_16_583 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::io;
    use std::fmt::Write as FmtWrite;
    struct MockWriterFormatter {
        buffer: String,
    }
    impl io::Write for MockWriterFormatter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = std::str::from_utf8(buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            self.buffer
                .write_str(s)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            let mut buffer = itoa::Buffer::new();
            let s = buffer.format(value);
            writer.write_all(s.as_bytes())
        }
    }
    #[test]
    fn test_write_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = MockWriterFormatter {
            buffer: String::new(),
        };
        {
            let mut formatter = TestFormatter;
            formatter.write_u128(&mut writer, rug_fuzz_0).unwrap();
        }
        debug_assert_eq!(writer.buffer, "123456789123456789123456789");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_584 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    use std::fmt;
    use std::io::{self, Write};
    struct TestWriter {
        output: Vec<u8>,
    }
    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }
        fn into_string(self) -> String {
            String::from_utf8(self.output).unwrap()
        }
    }
    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            let mut buffer = itoa::Buffer::new();
            let s = buffer.format(value);
            writer.write_all(s.as_bytes())
        }
    }
    #[test]
    fn test_write_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut test_writer = TestWriter::new();
        let mut formatter = TestFormatter;
        formatter.write_u16(&mut test_writer, rug_fuzz_0).unwrap();
        debug_assert_eq!(test_writer.into_string(), "12345");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_588 {
    use super::*;
    use crate::*;
    use crate::ser::Formatter;
    #[test]
    fn pretty_formatter_new() {
        let _rug_st_tests_llm_16_588_rrrruuuugggg_pretty_formatter_new = 0;
        let formatter = PrettyFormatter::new();
        debug_assert_eq!(formatter.current_indent, 0);
        debug_assert_eq!(formatter.has_value, false);
        debug_assert_eq!(formatter.indent, b"  ");
        let _clone = formatter.clone();
        let _default = PrettyFormatter::default();
        let _debug = format!("{:?}", formatter);
        let _rug_ed_tests_llm_16_588_rrrruuuugggg_pretty_formatter_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_589_llm_16_589 {
    use super::*;
    use crate::*;
    use std::io::Write;
    #[test]
    fn test_with_indent() {
        let _rug_st_tests_llm_16_589_llm_16_589_rrrruuuugggg_test_with_indent = 0;
        let rug_fuzz_0 = b"    ";
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = false;
        let rug_fuzz_3 = b"[\n    ,\n    \n    ]";
        let rug_fuzz_4 = true;
        let rug_fuzz_5 = b"{\n    : \n    }";
        let indent_value = rug_fuzz_0;
        let mut formatter = PrettyFormatter::with_indent(indent_value);
        debug_assert_eq!(formatter.indent, indent_value);
        debug_assert_eq!(formatter.current_indent, 0);
        debug_assert_eq!(formatter.has_value, false);
        let mut buffer = Vec::new();
        formatter.begin_array(&mut buffer).unwrap();
        debug_assert_eq!(formatter.current_indent, 1);
        debug_assert_eq!(buffer, b"[");
        formatter.begin_array_value(&mut buffer, rug_fuzz_1).unwrap();
        debug_assert_eq!(buffer, b"[\n    ");
        debug_assert_eq!(formatter.has_value, false);
        formatter.end_array_value(&mut buffer).unwrap();
        debug_assert_eq!(formatter.has_value, true);
        formatter.begin_array_value(&mut buffer, rug_fuzz_2).unwrap();
        debug_assert_eq!(buffer, b"[\n    ,\n    ");
        debug_assert_eq!(formatter.has_value, false);
        formatter.end_array(&mut buffer).unwrap();
        let expected_end = rug_fuzz_3;
        debug_assert_eq!(buffer, expected_end);
        buffer.clear();
        formatter.begin_object(&mut buffer).unwrap();
        debug_assert_eq!(formatter.current_indent, 1);
        debug_assert_eq!(buffer, b"{");
        formatter.begin_object_key(&mut buffer, rug_fuzz_4).unwrap();
        debug_assert_eq!(buffer, b"{\n    ");
        debug_assert_eq!(formatter.has_value, false);
        formatter.begin_object_value(&mut buffer).unwrap();
        debug_assert_eq!(buffer, b"{\n    : ");
        debug_assert_eq!(formatter.has_value, false);
        formatter.end_object_value(&mut buffer).unwrap();
        debug_assert_eq!(formatter.has_value, true);
        formatter.end_object(&mut buffer).unwrap();
        let expected_object_end = rug_fuzz_5;
        debug_assert_eq!(buffer, expected_object_end);
        let _rug_ed_tests_llm_16_589_llm_16_589_rrrruuuugggg_test_with_indent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_590 {
    use super::*;
    use crate::*;
    use crate::ser::{Serializer, CompactFormatter};
    use std::io::{self, Write};
    struct MockWriter {
        data: Vec<u8>,
    }
    impl MockWriter {
        fn new() -> Self {
            MockWriter { data: Vec::new() }
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_into_inner() {
        let _rug_st_tests_llm_16_590_rrrruuuugggg_test_into_inner = 0;
        let writer = MockWriter::new();
        let serializer = Serializer::new(writer);
        let writer_unwrapped = serializer.into_inner();
        debug_assert!(writer_unwrapped.data.is_empty());
        let _rug_ed_tests_llm_16_590_rrrruuuugggg_test_into_inner = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_591_llm_16_591 {
    use super::*;
    use crate::*;
    use crate::ser::{CompactFormatter, PrettyFormatter, Serializer};
    use std::io::Write;
    #[test]
    fn test_with_formatter_compact() {
        let _rug_st_tests_llm_16_591_llm_16_591_rrrruuuugggg_test_with_formatter_compact = 0;
        let output = Vec::new();
        let serializer = Serializer::with_formatter(output, CompactFormatter);
        let output_after = serializer.into_inner();
        debug_assert_eq!(output_after, Vec:: < u8 > ::new());
        let _rug_ed_tests_llm_16_591_llm_16_591_rrrruuuugggg_test_with_formatter_compact = 0;
    }
    #[test]
    fn test_with_formatter_pretty() {
        let _rug_st_tests_llm_16_591_llm_16_591_rrrruuuugggg_test_with_formatter_pretty = 0;
        let output = Vec::new();
        let serializer = Serializer::with_formatter(output, PrettyFormatter::new());
        let output_after = serializer.into_inner();
        debug_assert_eq!(output_after, Vec:: < u8 > ::new());
        let _rug_ed_tests_llm_16_591_llm_16_591_rrrruuuugggg_test_with_formatter_pretty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_593 {
    use super::*;
    use crate::*;
    use crate::ser::{Serializer, CompactFormatter};
    use std::io;
    #[test]
    fn test_new_serializer() {
        let _rug_st_tests_llm_16_593_rrrruuuugggg_test_new_serializer = 0;
        let output = Vec::new();
        let serializer = Serializer::new(output);
        let output_after = serializer.into_inner();
        debug_assert!(
            output_after.is_empty(), "Serializer output should initially be empty."
        );
        let _rug_ed_tests_llm_16_593_rrrruuuugggg_test_new_serializer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_596 {
    use super::*;
    use crate::*;
    use crate::ser;
    use std::fmt::{self, Write as FmtWrite};
    use std::io::{self, Write as IOWrite};
    use std::str;
    struct MockWriter<'a> {
        output: &'a mut String,
    }
    impl<'a> FmtWrite for MockWriter<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    impl<'a> IOWrite for MockWriter<'a> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            match str::from_utf8(buf) {
                Ok(s) => {
                    self.output.push_str(s);
                    Ok(buf.len())
                }
                Err(_) => {
                    Err(
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid UTF-8 sequence",
                        ),
                    )
                }
            }
        }
        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            match str::from_utf8(buf) {
                Ok(s) => {
                    self.output.push_str(s);
                    Ok(())
                }
                Err(_) => {
                    Err(
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid UTF-8 sequence",
                        ),
                    )
                }
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_indent_zero_n() -> io::Result<()> {
        let mut output = String::new();
        let mut mock_writer = MockWriter { output: &mut output };
        ser::indent(&mut mock_writer, 0, b" ")?;
        assert_eq!(output, "");
        Ok(())
    }
    #[test]
    fn test_indent_non_zero_n() -> io::Result<()> {
        let mut output = String::new();
        let mut mock_writer = MockWriter { output: &mut output };
        ser::indent(&mut mock_writer, 3, b" ")?;
        assert_eq!(output, "   ");
        Ok(())
    }
    #[test]
    fn test_indent_with_newline() -> io::Result<()> {
        let mut output = String::new();
        let mut mock_writer = MockWriter { output: &mut output };
        ser::indent(&mut mock_writer, 2, b"\n")?;
        assert_eq!(output, "\n\n");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_597_llm_16_597 {
    use super::*;
    use crate::*;
    use crate::error::{Error, ErrorCode};
    use std::fmt::Display;
    #[test]
    fn test_key_must_be_a_string() {
        let error = key_must_be_a_string();
        assert!(error.is_syntax());
        assert!(! error.is_io());
        assert!(! error.is_data());
        assert!(! error.is_eof());
        assert_eq!(error.line(), 0);
        assert_eq!(error.column(), 0);
        match error.classify() {
            crate::error::Category::Syntax => {}
            _ => panic!("error.classify() did not return Category::Syntax"),
        }
        assert!(
            format!("{}", error) .contains("key must be a string at line 0 column 0")
        );
        assert!(
            format!("{:?}", error)
            .contains("Error(\"key must be a string\", line: 0, column: 0)")
        );
    }
    fn make_error(msg: String) -> Error {
        Error::syntax(ErrorCode::Message(msg.into_boxed_str()), 0, 0)
    }
}
#[cfg(test)]
mod tests_llm_16_598 {
    use crate::ser::to_string;
    use crate::{Map, Number, Value};
    #[test]
    fn test_to_string_number() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = Number::from(rug_fuzz_0);
        let serialized = to_string(&num).unwrap();
        debug_assert_eq!(serialized, "42");
             }
});    }
    #[test]
    fn test_to_string_map() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let serialized = to_string(&map).unwrap();
        debug_assert_eq!(serialized, r#"{"key":"value"}"#);
             }
});    }
    #[test]
    fn test_to_string_value_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_string());
        let serialized = to_string(&value).unwrap();
        debug_assert_eq!(serialized, r#""A string""#);
             }
});    }
    #[test]
    fn test_to_string_value_boolean() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        let serialized = to_string(&value).unwrap();
        debug_assert_eq!(serialized, "true");
             }
});    }
    #[test]
    fn test_to_string_value_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(
            vec![
                Value::Number(Number::from(rug_fuzz_0)), Value::String("Hello"
                .to_string())
            ],
        );
        let serialized = to_string(&value).unwrap();
        debug_assert_eq!(serialized, r#"[42,"Hello"]"#);
             }
});    }
    #[test]
    fn test_to_string_value_null() {
        let _rug_st_tests_llm_16_598_rrrruuuugggg_test_to_string_value_null = 0;
        let value = Value::Null;
        let serialized = to_string(&value).unwrap();
        debug_assert_eq!(serialized, "null");
        let _rug_ed_tests_llm_16_598_rrrruuuugggg_test_to_string_value_null = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_599 {
    use serde::Serialize;
    use crate::ser::to_string_pretty;
    use crate::value::Value;
    use crate::map::Map;
    #[derive(Serialize)]
    struct TestStruct {
        integer: i32,
        float: f64,
        boolean: bool,
        text: String,
    }
    #[test]
    fn test_to_string_pretty_struct() {
        let _rug_st_tests_llm_16_599_rrrruuuugggg_test_to_string_pretty_struct = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 3.14;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = "example";
        let rug_fuzz_4 = r#"{
  "integer": 42,
  "float": 3.14,
  "boolean": true,
  "text": "example"
}"#;
        let test_value = TestStruct {
            integer: rug_fuzz_0,
            float: rug_fuzz_1,
            boolean: rug_fuzz_2,
            text: String::from(rug_fuzz_3),
        };
        let pretty_json = to_string_pretty(&test_value).unwrap();
        let expected = rug_fuzz_4;
        debug_assert_eq!(pretty_json, expected);
        let _rug_ed_tests_llm_16_599_rrrruuuugggg_test_to_string_pretty_struct = 0;
    }
    #[test]
    fn test_to_string_pretty_map() {
        let _rug_st_tests_llm_16_599_rrrruuuugggg_test_to_string_pretty_map = 0;
        let rug_fuzz_0 = "integer";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "float";
        let rug_fuzz_3 = 3.14;
        let rug_fuzz_4 = "boolean";
        let rug_fuzz_5 = true;
        let rug_fuzz_6 = "text";
        let rug_fuzz_7 = "example";
        let rug_fuzz_8 = r#"{
  "boolean": true,
  "float": 3.14,
  "integer": 42,
  "text": "example"
}"#;
        let mut test_map = Map::new();
        test_map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        test_map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        test_map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        test_map.insert(rug_fuzz_6.to_string(), Value::from(rug_fuzz_7));
        let pretty_json = to_string_pretty(&test_map).unwrap();
        let expected = rug_fuzz_8;
        debug_assert_eq!(pretty_json, expected);
        let _rug_ed_tests_llm_16_599_rrrruuuugggg_test_to_string_pretty_map = 0;
    }
    #[test]
    fn test_to_string_pretty_value() {
        let _rug_st_tests_llm_16_599_rrrruuuugggg_test_to_string_pretty_value = 0;
        let rug_fuzz_0 = "integer";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "float";
        let rug_fuzz_3 = 3.14;
        let rug_fuzz_4 = "boolean";
        let rug_fuzz_5 = true;
        let rug_fuzz_6 = "text";
        let rug_fuzz_7 = "example";
        let rug_fuzz_8 = r#"{
  "boolean": true,
  "float": 3.14,
  "integer": 42,
  "text": "example"
}"#;
        let test_value = Value::Object({
            let mut m = Map::new();
            m.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
            m.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
            m.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
            m.insert(rug_fuzz_6.to_string(), Value::from(rug_fuzz_7));
            m
        });
        let pretty_json = to_string_pretty(&test_value).unwrap();
        let expected = rug_fuzz_8;
        debug_assert_eq!(pretty_json, expected);
        let _rug_ed_tests_llm_16_599_rrrruuuugggg_test_to_string_pretty_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_600 {
    use crate::ser::to_vec;
    use crate::{Number, Value, Map};
    use serde::Serialize;
    #[derive(Serialize)]
    struct SimpleStruct {
        x: i32,
        y: i32,
        z: i32,
    }
    #[test]
    fn test_to_vec_simple_struct() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let simple_struct = SimpleStruct {
            x: rug_fuzz_0,
            y: rug_fuzz_1,
            z: rug_fuzz_2,
        };
        let vec = to_vec(&simple_struct).unwrap();
        let json_str = String::from_utf8(vec).unwrap();
        debug_assert_eq!(json_str, r#"{"x":1,"y":2,"z":3}"#);
             }
});    }
    #[test]
    fn test_to_vec_number() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = Number::from(rug_fuzz_0);
        let vec = to_vec(&number).unwrap();
        let json_str = String::from_utf8(vec).unwrap();
        debug_assert_eq!(json_str, "42");
             }
});    }
    #[test]
    fn test_to_vec_map() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, i32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::Number(Number::from(rug_fuzz_3)));
        let vec = to_vec(&map).unwrap();
        let json_str = String::from_utf8(vec).unwrap();
        debug_assert!(json_str == rug_fuzz_4 || json_str == rug_fuzz_5);
             }
});    }
    #[test]
    fn test_to_vec_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        let vec = to_vec(&value).unwrap();
        let json_str = String::from_utf8(vec).unwrap();
        debug_assert_eq!(json_str, r#""example""#);
             }
});    }
    #[test]
    fn test_to_vec_empty_map() {
        let _rug_st_tests_llm_16_600_rrrruuuugggg_test_to_vec_empty_map = 0;
        let map: Map<String, Value> = Map::new();
        let vec = to_vec(&map).unwrap();
        let json_str = String::from_utf8(vec).unwrap();
        debug_assert_eq!(json_str, r#"{}"#);
        let _rug_ed_tests_llm_16_600_rrrruuuugggg_test_to_vec_empty_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_601 {
    use crate::ser::to_vec_pretty;
    use crate::{Map, Value};
    use serde::{Serialize, Deserialize};
    use std::collections::BTreeMap;
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        name: String,
        value: i32,
        flag: bool,
    }
    #[test]
    fn test_to_vec_pretty_struct() -> Result<(), crate::Error> {
        let test_data = TestStruct {
            name: "Test".to_string(),
            value: 42,
            flag: true,
        };
        let json_vec = to_vec_pretty(&test_data)?;
        let json_string = String::from_utf8(json_vec).unwrap();
        assert_eq!(
            json_string.trim(), r#"{
  "name": "Test",
  "value": 42,
  "flag": true
}"#
        );
        Ok(())
    }
    #[test]
    fn test_to_vec_pretty_map() -> Result<(), crate::Error> {
        let mut test_map = Map::new();
        test_map.insert("key".to_string(), Value::String("value".to_string()));
        test_map.insert("number".to_string(), Value::Number(42.into()));
        let json_vec = to_vec_pretty(&test_map)?;
        let json_string = String::from_utf8(json_vec).unwrap();
        let expected = if test_map.get("key").is_some() {
            r#"{
  "key": "value",
  "number": 42
}"#
        } else {
            r#"{
  "number": 42,
  "key": "value"
}"#
        };
        assert_eq!(json_string.trim(), expected);
        Ok(())
    }
    #[test]
    fn test_to_vec_pretty_btreemap() -> Result<(), crate::Error> {
        let mut test_btreemap = BTreeMap::new();
        test_btreemap.insert("apple".to_string(), 1);
        test_btreemap.insert("banana".to_string(), 2);
        let json_vec = to_vec_pretty(&test_btreemap)?;
        let json_string = String::from_utf8(json_vec).unwrap();
        assert_eq!(json_string.trim(), r#"{
  "apple": 1,
  "banana": 2
}"#);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_602_llm_16_602 {
    use crate::{ser::to_writer, Map, Value, Error};
    use serde::Serialize;
    use std::io::Write;
    struct MockWriter {
        pub content: Vec<u8>,
        pub should_fail: bool,
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            if self.should_fail {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "forced write failure",
                    ),
                )
            } else {
                self.content.extend_from_slice(buf);
                Ok(buf.len())
            }
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_to_writer_valid_json() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut mock_writer = MockWriter {
            content: vec![],
            should_fail: rug_fuzz_0,
        };
        let data = Value::String(rug_fuzz_1.to_owned());
        let result = to_writer(&mut mock_writer, &data);
        debug_assert!(result.is_ok());
        debug_assert_eq!(mock_writer.content, br#""Hello, World!""#);
             }
});    }
    #[test]
    fn test_to_writer_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut mock_writer = MockWriter {
            content: vec![],
            should_fail: rug_fuzz_0,
        };
        let data = Value::String(rug_fuzz_1.to_owned());
        let result = to_writer(&mut mock_writer, &data);
        debug_assert!(result.is_err());
        let error = result.unwrap_err();
        debug_assert!(matches!(error.classify(), crate ::error::Category::Io));
             }
});    }
    #[test]
    fn test_to_writer_complex_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(bool, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut mock_writer = MockWriter {
            content: vec![],
            should_fail: rug_fuzz_0,
        };
        let mut map = Map::new();
        map.insert(rug_fuzz_1.to_owned(), Value::String(rug_fuzz_2.to_owned()));
        let result = to_writer(&mut mock_writer, &map);
        debug_assert!(result.is_ok());
        debug_assert_eq!(mock_writer.content, br#"{"key":"value"}"#);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_603 {
    use crate::{ser::to_writer_pretty, value::Value, Map};
    use std::fmt;
    use std::io::{self, Write};
    struct MockWriter {
        buf: Vec<u8>,
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.write(buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.buf.flush()
        }
    }
    impl fmt::Display for MockWriter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", String::from_utf8_lossy(& self.buf))
        }
    }
    #[test]
    fn test_to_writer_pretty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::Number(rug_fuzz_3.into()));
        let value = Value::Object(map);
        let mut writer = MockWriter { buf: Vec::new() };
        let result = to_writer_pretty(&mut writer, &value);
        debug_assert!(result.is_ok());
        debug_assert_eq!(
            writer.to_string(), "{\n  \"age\": 30,\n  \"name\": \"John Doe\"\n}"
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_99 {
    use super::*;
    use serde::ser::Serializer;
    use crate::ser::{Serializer as JsonSerializer, CompactFormatter};
    use std::io;
    #[test]
    fn test_serialize_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer: Vec<u8> = Vec::new();
        let formatter = CompactFormatter;
        let mut serializer = JsonSerializer::with_formatter(writer, formatter);
        let value: bool = rug_fuzz_0;
        serializer.serialize_bool(value).unwrap();
        debug_assert_eq!(serializer.into_inner(), b"true");
             }
});    }
}
#[cfg(test)]
mod tests_rug_100 {
    use super::*;
    use crate::error::Error;
    use std::fmt::Formatter;
    use crate::ser::{Serializer, CompactFormatter};
    #[test]
    fn test_serialize_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        struct TestWriter;
        impl std::io::Write for TestWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut writer = TestWriter;
        let formatter = CompactFormatter {};
        let mut serializer = Serializer::<TestWriter, CompactFormatter>::new(writer);
        let p0: &mut Serializer<TestWriter, CompactFormatter> = &mut serializer;
        let p1: i16 = rug_fuzz_0;
        let result = <&mut Serializer<
            TestWriter,
            CompactFormatter,
        > as serde::Serializer>::serialize_i16(p0, p1);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_102 {
    use super::*;
    use serde::ser::{Serializer as SerdeSerializer, Serialize};
    use crate::error::Error;
    use crate::ser::{Serializer, CompactFormatter};
    use std::io::Write;
    #[test]
    fn test_serialize_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        struct TestWriter;
        impl Write for TestWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let writer = TestWriter {};
        let formatter = CompactFormatter {};
        let mut serializer = Serializer { writer, formatter };
        let p0: &mut Serializer<TestWriter, CompactFormatter> = &mut serializer;
        let p1: u16 = rug_fuzz_0;
        debug_assert!(
            < & mut Serializer < TestWriter, CompactFormatter > > ::serialize_u16(p0, p1)
            .is_ok()
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::error::Error;
    use crate::ser::{Formatter, Serializer, CompactFormatter};
    use std::io;
    struct VecWriter;
    impl io::Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = VecWriter;
        let formatter = CompactFormatter;
        let mut serializer = Serializer::with_formatter(writer, formatter);
        let p0: &mut Serializer<VecWriter, CompactFormatter> = &mut serializer;
        let p1: u32 = rug_fuzz_0;
        <&mut Serializer<
            VecWriter,
            CompactFormatter,
        > as serde::Serializer>::serialize_u32(p0, p1)
            .unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    use std::io;
    use crate::ser::{CompactFormatter, Serializer};
    use serde::ser::{Serialize, Serializer as SerdeSerializer};
    #[test]
    fn test_serialize_seq() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = Vec::new();
        let formatter = CompactFormatter;
        let mut p0 = Serializer::with_formatter(&mut writer, formatter);
        let mut p1 = Some(rug_fuzz_0);
        p0.serialize_seq(p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_108 {
    use super::*;
    use crate::ser::{Serializer, Formatter};
    use serde::ser::{Serialize, Serializer as SerdeSerializer};
    use std::io::Write;
    struct VecWriter;
    impl Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    struct SimpleFormatter;
    impl Formatter for SimpleFormatter {}
    #[test]
    fn test_serialize_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = VecWriter;
        let formatter = SimpleFormatter;
        let mut serializer = Serializer::with_formatter(&mut writer, formatter);
        let tuple_len: usize = rug_fuzz_0;
        serializer.serialize_tuple(tuple_len).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    use crate::error::Error;
    use crate::ser::{Formatter, PrettyFormatter, Serializer};
    use crate::ser;
    use crate::value::Number;
    use std::io;
    use serde::ser::{Serialize, SerializeSeq};
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_element() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let formatter = PrettyFormatter::new();
        let writer = TestWriter;
        let mut serializer = Serializer::with_formatter(writer, formatter);
        let mut p0 = ser::Compound::Map {
            ser: &mut serializer,
            state: ser::State::First,
        };
        let mut p1 = Number::from(rug_fuzz_0);
        p0.serialize_element(&p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_125 {
    use serde::Serializer;
    use crate::ser::{MapKeySerializer, Formatter, Serializer as JsonSerializer};
    use crate::error::Error;
    struct FakeFormatter;
    impl Formatter for FakeFormatter {}
    #[test]
    fn test_serialize_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let buf = Vec::new();
        let formatter = FakeFormatter;
        let mut json_serializer = JsonSerializer::with_formatter(buf, formatter);
        let mut p0 = MapKeySerializer {
            ser: &mut json_serializer,
        };
        let p1: i16 = rug_fuzz_0;
        <MapKeySerializer<
            '_,
            Vec<u8>,
            FakeFormatter,
        > as Serializer>::serialize_i16(p0, p1)
            .unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_144 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        let mut p1: u8 = rug_fuzz_1;
        crate::ser::CharEscape::from_escape_table(p0, p1);
             }
});    }
}
