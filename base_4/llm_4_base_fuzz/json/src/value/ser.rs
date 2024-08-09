use crate::error::{Error, ErrorCode, Result};
use crate::map::Map;
use crate::value::{to_value, Value};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(not(feature = "arbitrary_precision"))]
use core::convert::TryFrom;
use core::fmt::Display;
use core::result;
use serde::ser::{Impossible, Serialize};
impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(v) => v.serialize(serializer),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = tri!(serializer.serialize_map(Some(m.len())));
                for (k, v) in m {
                    tri!(map.serialize_entry(k, v));
                }
                map.end()
            }
        }
    }
}
/// Serializer whose output is a `Value`.
///
/// This is the serializer that backs [`serde_json::to_value`][crate::to_value].
/// Unlike the main serde_json serializer which goes from some serializable
/// value of type `T` to JSON text, this one goes from `T` to
/// `serde_json::Value`.
///
/// The `to_value` function is implementable as:
///
/// ```
/// use serde::Serialize;
/// use serde_json::{Error, Value};
///
/// pub fn to_value<T>(input: T) -> Result<Value, Error>
/// where
///     T: Serialize,
/// {
///     input.serialize(serde_json::value::Serializer)
/// }
/// ```
pub struct Serializer;
impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;
    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::Bool(value))
    }
    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }
    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }
    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }
    fn serialize_i64(self, value: i64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }
    fn serialize_i128(self, value: i128) -> Result<Value> {
        #[cfg(feature = "arbitrary_precision")] { Ok(Value::Number(value.into())) }
        #[cfg(not(feature = "arbitrary_precision"))]
        {
            if let Ok(value) = u64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else if let Ok(value) = i64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
            }
        }
    }
    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_u64(value as u64)
    }
    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_u64(value as u64)
    }
    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_u64(value as u64)
    }
    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }
    fn serialize_u128(self, value: u128) -> Result<Value> {
        #[cfg(feature = "arbitrary_precision")] { Ok(Value::Number(value.into())) }
        #[cfg(not(feature = "arbitrary_precision"))]
        {
            if let Ok(value) = u64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
            }
        }
    }
    #[inline]
    fn serialize_f32(self, float: f32) -> Result<Value> {
        Ok(Value::from(float))
    }
    #[inline]
    fn serialize_f64(self, float: f64) -> Result<Value> {
        Ok(Value::from(float))
    }
    #[inline]
    fn serialize_char(self, value: char) -> Result<Value> {
        let mut s = String::new();
        s.push(value);
        Ok(Value::String(s))
    }
    #[inline]
    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.to_owned()))
    }
    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }
    #[inline]
    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::Null)
    }
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }
    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        self.serialize_str(variant)
    }
    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), tri!(to_value(value)));
        Ok(Value::Object(values))
    }
    #[inline]
    fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }
    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::Map {
            map: Map::new(),
            next_key: None,
        })
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => {
                Ok(SerializeMap::Number {
                    out_value: None,
                })
            }
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => {
                Ok(SerializeMap::RawValue {
                    out_value: None,
                })
            }
            _ => self.serialize_map(Some(len)),
        }
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: String::from(variant),
            map: Map::new(),
        })
    }
    fn collect_str<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }
}
pub struct SerializeVec {
    vec: Vec<Value>,
}
pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<Value>,
}
pub enum SerializeMap {
    Map { map: Map<String, Value>, next_key: Option<String> },
    #[cfg(feature = "arbitrary_precision")]
    Number { out_value: Option<Value> },
    #[cfg(feature = "raw_value")]
    RawValue { out_value: Option<Value> },
}
pub struct SerializeStructVariant {
    name: String,
    map: Map<String, Value>,
}
impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(tri!(to_value(value)));
        Ok(())
    }
    fn end(self) -> Result<Value> {
        Ok(Value::Array(self.vec))
    }
}
impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl serde::ser::SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(tri!(to_value(value)));
        Ok(())
    }
    fn end(self) -> Result<Value> {
        let mut object = Map::new();
        object.insert(self.name, Value::Array(self.vec));
        Ok(Value::Object(object))
    }
}
impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { next_key, .. } => {
                *next_key = Some(tri!(key.serialize(MapKeySerializer)));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { map, next_key } => {
                let key = next_key.take();
                let key = key.expect("serialize_value called before serialize_key");
                map.insert(key, tri!(to_value(value)));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }
    fn end(self) -> Result<Value> {
        match self {
            SerializeMap::Map { map, .. } => Ok(Value::Object(map)),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }
}
struct MapKeySerializer;
fn key_must_be_a_string() -> Error {
    Error::syntax(ErrorCode::KeyMustBeAString, 0, 0)
}
impl serde::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;
    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {
        Ok(variant.to_owned())
    }
    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_bool(self, _value: bool) -> Result<String> {
        Err(key_must_be_a_string())
    }
    fn serialize_i8(self, value: i8) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_i16(self, value: i16) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_i32(self, value: i32) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_i64(self, value: i64) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_u8(self, value: u8) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_u16(self, value: u16) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_u32(self, value: u32) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_u64(self, value: u64) -> Result<String> {
        Ok(value.to_string())
    }
    fn serialize_f32(self, _value: f32) -> Result<String> {
        Err(key_must_be_a_string())
    }
    fn serialize_f64(self, _value: f64) -> Result<String> {
        Err(key_must_be_a_string())
    }
    #[inline]
    fn serialize_char(self, value: char) -> Result<String> {
        Ok({
            let mut s = String::new();
            s.push(value);
            s
        })
    }
    #[inline]
    fn serialize_str(self, value: &str) -> Result<String> {
        Ok(value.to_owned())
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
        Err(key_must_be_a_string())
    }
    fn serialize_unit(self) -> Result<String> {
        Err(key_must_be_a_string())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(key_must_be_a_string())
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }
    fn serialize_none(self) -> Result<String> {
        Err(key_must_be_a_string())
    }
    fn serialize_some<T>(self, _value: &T) -> Result<String>
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
    fn collect_str<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Display,
    {
        Ok(value.to_string())
    }
}
impl serde::ser::SerializeStruct for SerializeMap {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { .. } => {
                serde::ser::SerializeMap::serialize_entry(self, key, value)
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { out_value } => {
                if key == crate::number::TOKEN {
                    *out_value = Some(value.serialize(NumberValueEmitter)?);
                    Ok(())
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { out_value } => {
                if key == crate::raw::TOKEN {
                    *out_value = Some(value.serialize(RawValueEmitter)?);
                    Ok(())
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }
    fn end(self) -> Result<Value> {
        match self {
            SerializeMap::Map { .. } => serde::ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { out_value, .. } => {
                Ok(out_value.expect("number value was not emitted"))
            }
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { out_value, .. } => {
                Ok(out_value.expect("raw value was not emitted"))
            }
        }
    }
}
impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map.insert(String::from(key), tri!(to_value(value)));
        Ok(())
    }
    fn end(self) -> Result<Value> {
        let mut object = Map::new();
        object.insert(self.name, Value::Object(self.map));
        Ok(Value::Object(object))
    }
}
#[cfg(feature = "arbitrary_precision")]
struct NumberValueEmitter;
#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
    Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}
#[cfg(feature = "arbitrary_precision")]
impl serde::ser::Serializer for NumberValueEmitter {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = Impossible<Value, Error>;
    type SerializeTuple = Impossible<Value, Error>;
    type SerializeTupleStruct = Impossible<Value, Error>;
    type SerializeTupleVariant = Impossible<Value, Error>;
    type SerializeMap = Impossible<Value, Error>;
    type SerializeStruct = Impossible<Value, Error>;
    type SerializeStructVariant = Impossible<Value, Error>;
    fn serialize_bool(self, _v: bool) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_i8(self, _v: i8) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_i16(self, _v: i16) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_i32(self, _v: i32) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_i64(self, _v: i64) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_u8(self, _v: u8) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_u16(self, _v: u16) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_u32(self, _v: u32) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_u64(self, _v: u64) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_f32(self, _v: f32) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_f64(self, _v: f64) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_char(self, _v: char) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_str(self, value: &str) -> Result<Value> {
        let n = tri!(value.to_owned().parse());
        Ok(Value::Number(n))
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_none(self) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_some<T>(self, _value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }
    fn serialize_unit(self) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Value> {
        Err(invalid_number())
    }
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Value>
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
    ) -> Result<Value>
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
struct RawValueEmitter;
#[cfg(feature = "raw_value")]
fn invalid_raw_value() -> Error {
    Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0)
}
#[cfg(feature = "raw_value")]
impl serde::ser::Serializer for RawValueEmitter {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = Impossible<Value, Error>;
    type SerializeTuple = Impossible<Value, Error>;
    type SerializeTupleStruct = Impossible<Value, Error>;
    type SerializeTupleVariant = Impossible<Value, Error>;
    type SerializeMap = Impossible<Value, Error>;
    type SerializeStruct = Impossible<Value, Error>;
    type SerializeStructVariant = Impossible<Value, Error>;
    fn serialize_bool(self, _v: bool) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_i8(self, _v: i8) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_i16(self, _v: i16) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_i32(self, _v: i32) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_i64(self, _v: i64) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_u8(self, _v: u8) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_u16(self, _v: u16) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_u32(self, _v: u32) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_u64(self, _v: u64) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_f32(self, _v: f32) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_f64(self, _v: f64) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_char(self, _v: char) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_str(self, value: &str) -> Result<Value> {
        crate::from_str(value)
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_none(self) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_some<T>(self, _value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_raw_value())
    }
    fn serialize_unit(self) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Value> {
        Err(invalid_raw_value())
    }
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_raw_value())
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_raw_value())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(invalid_raw_value())
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(invalid_raw_value())
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(invalid_raw_value())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(invalid_raw_value())
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(invalid_raw_value())
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(invalid_raw_value())
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(invalid_raw_value())
    }
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        self.serialize_str(&value.to_string())
    }
}
#[cfg(test)]
mod tests_llm_16_358 {
    use super::*;
    use crate::*;
    use crate::value::ser::MapKeySerializer;
    use serde::Serializer;
    use std::fmt::Display;
    struct TestStruct;
    impl Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestStruct Display")
        }
    }
    #[test]
    fn test_collect_str() {
        let _rug_st_tests_llm_16_358_rrrruuuugggg_test_collect_str = 0;
        let serializer = MapKeySerializer;
        let test_value = TestStruct;
        let result = serializer.collect_str(&test_value).unwrap();
        debug_assert_eq!(result, "TestStruct Display");
        let _rug_ed_tests_llm_16_358_rrrruuuugggg_test_collect_str = 0;
    }
    #[test]
    fn test_collect_str_with_primitive_type() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.collect_str(&rug_fuzz_0).unwrap();
        debug_assert_eq!(result, "123");
             }
});    }
    #[test]
    fn test_collect_str_with_string_type() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let test_value = rug_fuzz_0;
        let result = serializer.collect_str(&test_value).unwrap();
        debug_assert_eq!(result, "test string");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_359 {
    use serde::Serializer;
    use crate::error::Category;
    use crate::value::ser::{MapKeySerializer, Error};
    #[test]
    fn serialize_bool_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.serialize_bool(rug_fuzz_0);
        debug_assert!(result.is_err());
        match result {
            Err(e) => debug_assert_eq!(e.classify(), Category::Data),
            Ok(_) => panic!("Expected an error for bool serialization as map key"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_361_llm_16_361 {
    use crate::value::ser::MapKeySerializer;
    use serde::Serializer;
    #[test]
    fn test_serialize_char() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.serialize_char(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), "A".to_owned());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_362 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    #[test]
    fn test_serialize_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.serialize_f32(rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(
            result.unwrap_err().to_string(), "key must be a string".to_string()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_363 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    #[test]
    fn test_serialize_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.serialize_f64(rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(
            * result.unwrap_err().to_string(), "key must be a string".to_string()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_364_llm_16_364 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    #[test]
    fn serialize_i16_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = MapKeySerializer.serialize_i16(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), "123");
        let result = MapKeySerializer.serialize_i16(-rug_fuzz_1);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), "-123");
        let result = MapKeySerializer.serialize_i16(rug_fuzz_2);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), "0");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_366_llm_16_366 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    #[test]
    fn test_serialize_i64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = MapKeySerializer.serialize_i64(rug_fuzz_0).unwrap();
        debug_assert_eq!(result, "123");
        let result = MapKeySerializer.serialize_i64(-rug_fuzz_1).unwrap();
        debug_assert_eq!(result, "-123");
        let result = MapKeySerializer.serialize_i64(i64::MIN).unwrap();
        debug_assert_eq!(result, i64::MIN.to_string());
        let result = MapKeySerializer.serialize_i64(i64::MAX).unwrap();
        debug_assert_eq!(result, i64::MAX.to_string());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_367 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    #[test]
    fn test_serialize_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.serialize_i8(rug_fuzz_0).unwrap();
        debug_assert_eq!(result, "42");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_368_llm_16_368 {
    use crate::value::ser::MapKeySerializer;
    use serde::ser::Serializer;
    #[test]
    fn serialize_map_error() {
        let _rug_st_tests_llm_16_368_llm_16_368_rrrruuuugggg_serialize_map_error = 0;
        let serializer = MapKeySerializer;
        let result = serializer.serialize_map(None);
        debug_assert!(matches!(result, Err(_)));
        let _rug_ed_tests_llm_16_368_llm_16_368_rrrruuuugggg_serialize_map_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_371_llm_16_371 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    use crate::value::ser::key_must_be_a_string;
    #[test]
    fn test_serialize_none() {
        let _rug_st_tests_llm_16_371_llm_16_371_rrrruuuugggg_test_serialize_none = 0;
        let serializer = MapKeySerializer;
        let result = serializer.serialize_none();
        debug_assert!(result.is_err());
        debug_assert_eq!(
            result.unwrap_err().to_string(), key_must_be_a_string().to_string()
        );
        let _rug_ed_tests_llm_16_371_llm_16_371_rrrruuuugggg_test_serialize_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_374 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::Error;
    #[test]
    fn test_serialize_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let input = rug_fuzz_0;
        let expected = rug_fuzz_1.to_owned();
        let result: Result<String, Error> = serializer.serialize_str(input);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_375 {
    use serde::ser::{Error as SerError, Impossible, Serializer};
    use serde::Serialize;
    use crate::value::ser::MapKeySerializer;
    use crate::value::ser::key_must_be_a_string;
    #[test]
    fn serialize_struct_test() {
        let _rug_st_tests_llm_16_375_rrrruuuugggg_serialize_struct_test = 0;
        let rug_fuzz_0 = "TestStruct";
        let rug_fuzz_1 = 0;
        let serializer = MapKeySerializer;
        let result = serializer.serialize_struct(rug_fuzz_0, rug_fuzz_1);
        match result {
            Ok(_) => panic!("Expected error, got Ok"),
            Err(e) => {
                let expected_error = key_must_be_a_string();
                debug_assert_eq!(
                    e.to_string(), expected_error.to_string(),
                    "Error message did not match expected"
                );
            }
        }
        let _rug_ed_tests_llm_16_375_rrrruuuugggg_serialize_struct_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_376_llm_16_376 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    #[test]
    fn test_serialize_struct_variant() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_serialize_struct_variant = 0;
        let rug_fuzz_0 = "StructName";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "VariantName";
        let rug_fuzz_3 = 0;
        let serializer = MapKeySerializer;
        let result = serializer
            .serialize_struct_variant(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        match result {
            Ok(_) => panic!("Expected an error, but serialization succeeded"),
            Err(e) => debug_assert_eq!(e.to_string(), "key must be a string"),
        }
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_serialize_struct_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_377 {
    use serde::ser::Serializer;
    use serde::Serialize;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    #[test]
    fn test_serialize_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let result = serializer.serialize_tuple(rug_fuzz_0);
        match result {
            Ok(_) => panic!("Expected error, but got Ok"),
            Err(e) => {
                match e.classify() {
                    crate::error::Category::Data => {}
                    _ => {
                        panic!(
                            "Expected error of category Data, but got {:?}", e.classify()
                        )
                    }
                }
            }
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_379 {
    use super::*;
    use crate::*;
    use serde::ser::{Serialize, Serializer};
    use serde::ser::Impossible;
    use crate::value::ser::MapKeySerializer;
    use crate::Error;
    #[test]
    fn test_serialize_tuple_variant() {
        let _rug_st_tests_llm_16_379_rrrruuuugggg_test_serialize_tuple_variant = 0;
        let rug_fuzz_0 = "TupleVariant";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "variant";
        let rug_fuzz_3 = 0;
        let serializer = MapKeySerializer;
        let result = serializer
            .serialize_tuple_variant(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        match result {
            Ok(_) => {
                panic!("Expected an error for tuple variant serialization, but got Ok")
            }
            Err(e) => {
                match e.classify() {
                    crate::error::Category::Data => {}
                    _ => {
                        panic!(
                            "Expected a different error category for tuple variant serialization"
                        )
                    }
                }
            }
        }
        let _rug_ed_tests_llm_16_379_rrrruuuugggg_test_serialize_tuple_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_380 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    #[test]
    fn test_serialize_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = MapKeySerializer;
        let value: u16 = rug_fuzz_0;
        debug_assert_eq!(serializer.serialize_u16(value).unwrap(), "42");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_382_llm_16_382 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    #[test]
    fn test_serialize_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_cases = vec![
            (rug_fuzz_0, rug_fuzz_1.to_string()), (1u64, "1".to_string()), (u64::MAX,
            u64::MAX.to_string())
        ];
        for (input, expected) in test_cases {
            let serializer = MapKeySerializer;
            let result = serializer.serialize_u64(input).unwrap();
            debug_assert_eq!(result, expected);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_384 {
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::Error;
    #[test]
    fn test_serialize_unit() {
        let serializer = MapKeySerializer;
        let result = serializer.serialize_unit();
        assert!(result.is_err());
        if let Err(e) = result {
            match e.classify() {
                crate::error::Category::Data => {}
                _ => panic!("Expected error of `Category::Data`, got {:?}", e),
            }
        }
    }
    fn key_must_be_a_string() -> Error {
        crate::error::Error::syntax(crate::error::ErrorCode::KeyMustBeAString, 0, 0)
    }
}
#[cfg(test)]
mod tests_llm_16_385 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::value::ser::MapKeySerializer;
    use crate::error::Error;
    #[test]
    fn test_serialize_unit_struct() {
        let _rug_st_tests_llm_16_385_rrrruuuugggg_test_serialize_unit_struct = 0;
        let rug_fuzz_0 = "UnitStruct";
        let serializer = MapKeySerializer;
        let result = serializer.serialize_unit_struct(rug_fuzz_0);
        debug_assert!(result.is_err());
        match result {
            Err(e) => debug_assert_eq!(e.to_string(), "key must be a string"),
            _ => panic!("Expected error, got Ok"),
        }
        let _rug_ed_tests_llm_16_385_rrrruuuugggg_test_serialize_unit_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_386 {
    use serde::Serializer;
    use crate::value::ser::{MapKeySerializer, Error};
    #[test]
    fn test_serialize_unit_variant() -> Result<(), Error> {
        let serializer = MapKeySerializer;
        let name = "TestEnum";
        let variant_index = 0;
        let variant = "VariantA";
        let result = serializer.serialize_unit_variant(name, variant_index, variant)?;
        assert_eq!(variant, result);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_393 {
    use serde::ser::SerializeStructVariant;
    use serde::Serialize;
    use crate::value::{Map, Value};
    use crate::{json, to_value};
    #[derive(Serialize)]
    struct TestStruct {
        int_field: i32,
        str_field: String,
        bool_field: bool,
    }
    #[test]
    fn test_serialize_struct_variant_serialize_field() {
        let _rug_st_tests_llm_16_393_rrrruuuugggg_test_serialize_struct_variant_serialize_field = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = "TestStruct";
        let rug_fuzz_4 = "int_field";
        let rug_fuzz_5 = "str_field";
        let rug_fuzz_6 = "bool_field";
        let test_struct = TestStruct {
            int_field: rug_fuzz_0,
            str_field: rug_fuzz_1.to_owned(),
            bool_field: rug_fuzz_2,
        };
        struct TestSerializeStructVariant {
            name: String,
            map: Map<String, Value>,
        }
        impl SerializeStructVariant for TestSerializeStructVariant {
            type Ok = Value;
            type Error = crate::Error;
            fn serialize_field<T>(
                &mut self,
                key: &'static str,
                value: &T,
            ) -> Result<(), Self::Error>
            where
                T: ?Sized + Serialize,
            {
                self.map.insert(String::from(key), to_value(value)?);
                Ok(())
            }
            fn end(self) -> Result<Self::Ok, Self::Error> {
                let mut object = Map::new();
                object.insert(self.name, Value::Object(self.map));
                Ok(Value::Object(object))
            }
        }
        let mut serialize_struct_variant = TestSerializeStructVariant {
            name: rug_fuzz_3.to_owned(),
            map: Map::new(),
        };
        serialize_struct_variant
            .serialize_field(rug_fuzz_4, &test_struct.int_field)
            .unwrap();
        serialize_struct_variant
            .serialize_field(rug_fuzz_5, &test_struct.str_field)
            .unwrap();
        serialize_struct_variant
            .serialize_field(rug_fuzz_6, &test_struct.bool_field)
            .unwrap();
        let expected = json!(
            { "TestStruct" : { "int_field" : 42, "str_field" : "test", "bool_field" :
            true, } }
        );
        let result = serialize_struct_variant.end().unwrap();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_393_rrrruuuugggg_test_serialize_struct_variant_serialize_field = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_396 {
    use crate::value::ser::SerializeVec;
    use crate::value::Value;
    use crate::Error;
    use serde::ser::{SerializeSeq, Serialize};
    #[test]
    fn test_serialize_vec_end() -> Result<(), Error> {
        let mut serialize_vec = SerializeVec { vec: Vec::new() };
        serialize_vec.serialize_element(&1)?;
        serialize_vec.serialize_element(&2)?;
        serialize_vec.serialize_element(&3)?;
        let expected = Value::Array(
            vec![
                Value::Number(1.into()), Value::Number(2.into()), Value::Number(3
                .into()),
            ],
        );
        let result = serialize_vec.end()?;
        assert_eq!(result, expected);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_403 {
    use crate::value::ser::Serializer;
    use crate::{Value, Error};
    use serde::Serializer as _;
    #[test]
    fn test_serialize_bool_true() -> Result<(), Error> {
        let serializer = Serializer;
        let true_val = true;
        let expected = Value::Bool(true_val);
        let result = serializer.serialize_bool(true_val)?;
        assert_eq!(result, expected);
        Ok(())
    }
    #[test]
    fn test_serialize_bool_false() -> Result<(), Error> {
        let serializer = Serializer;
        let false_val = false;
        let expected = Value::Bool(false_val);
        let result = serializer.serialize_bool(false_val)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_404 {
    use crate::value::ser::Serializer;
    use crate::{Value, Number};
    use serde::Serializer as _;
    #[test]
    fn test_serialize_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let serializer = Serializer;
        let serialized = serializer.serialize_bytes(bytes).unwrap();
        let expected = Value::Array(
            vec![
                Value::Number(Number::from(rug_fuzz_5)), Value::Number(Number::from(2)),
                Value::Number(Number::from(3)), Value::Number(Number::from(4)),
                Value::Number(Number::from(5))
            ],
        );
        debug_assert_eq!(serialized, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_406_llm_16_406 {
    use crate::value::ser::Serializer;
    use crate::{Number, Value};
    use serde::Serializer as _;
    #[test]
    fn test_serialize_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = Serializer;
        let f32_val = rug_fuzz_0;
        let expected_value = Value::Number(Number::from_f32(f32_val).unwrap());
        let result = serializer.serialize_f32(f32_val).unwrap();
        debug_assert_eq!(result, expected_value);
        let serializer = Serializer;
        let f32_val_nan = f32::NAN;
        let result_nan = serializer.serialize_f32(f32_val_nan).unwrap();
        debug_assert!(result_nan.is_number());
        debug_assert!(result_nan.as_f64().unwrap().is_nan());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_407_llm_16_407 {
    use crate::value::{self, Value};
    use crate::error::Error;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_f64() -> Result<(), Error> {
        let serializer = value::Serializer;
        let float_normal = 123.456;
        let value_normal = serializer.serialize_f64(float_normal)?;
        assert_eq!(value_normal, Value::from(float_normal));
        let serializer = value::Serializer;
        let float_nan = std::f64::NAN;
        let value_nan = serializer.serialize_f64(float_nan)?;
        assert!(value_nan.is_f64());
        assert!(value_nan.as_f64().unwrap().is_nan());
        let serializer = value::Serializer;
        let float_inf = std::f64::INFINITY;
        let value_inf = serializer.serialize_f64(float_inf)?;
        assert!(value_inf.is_f64());
        assert_eq!(value_inf.as_f64().unwrap(), float_inf);
        let serializer = value::Serializer;
        let float_neg_inf = std::f64::NEG_INFINITY;
        let value_neg_inf = serializer.serialize_f64(float_neg_inf)?;
        assert!(value_neg_inf.is_f64());
        assert_eq!(value_neg_inf.as_f64().unwrap(), float_neg_inf);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_409 {
    use serde::Serializer;
    use crate::value::{self, Value};
    use crate::error::Error;
    #[test]
    fn test_serialize_i16() -> Result<(), Error> {
        let serializer = value::Serializer;
        let i16_value = 123i16;
        let expected_value = Value::Number(i16_value.into());
        let serialized_value = serializer.serialize_i16(i16_value)?;
        assert_eq!(serialized_value, expected_value);
        Ok(())
    }
    #[test]
    fn test_serialize_i16_negative() -> Result<(), Error> {
        let serializer = value::Serializer;
        let i16_value = -123i16;
        let expected_value = Value::Number(i16_value.into());
        let serialized_value = serializer.serialize_i16(i16_value)?;
        assert_eq!(serialized_value, expected_value);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_410_llm_16_410 {
    use crate::value::{self, Value};
    use crate::Number;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = value::Serializer;
        let i32_value: i32 = rug_fuzz_0;
        let expected = Value::Number(Number::from(i32_value));
        let result = serializer.serialize_i32(i32_value).unwrap();
        debug_assert_eq!(result, expected);
        let serializer = value::Serializer;
        let i32_value: i32 = -rug_fuzz_1;
        let expected = Value::Number(Number::from(i32_value));
        let result = serializer.serialize_i32(i32_value).unwrap();
        debug_assert_eq!(result, expected);
        let serializer = value::Serializer;
        let i32_value: i32 = i32::MAX;
        let expected = Value::Number(Number::from(i32_value));
        let result = serializer.serialize_i32(i32_value).unwrap();
        debug_assert_eq!(result, expected);
        let serializer = value::Serializer;
        let i32_value: i32 = i32::MIN;
        let expected = Value::Number(Number::from(i32_value));
        let result = serializer.serialize_i32(i32_value).unwrap();
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_411 {
    use crate::value::ser::Serializer;
    use crate::value::Value;
    use serde::{Serialize, Serializer as SerdeSerializer};
    #[test]
    fn test_serialize_i64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = Serializer;
        let value_to_serialize: i64 = rug_fuzz_0;
        let serialized_value = serializer.serialize_i64(value_to_serialize).unwrap();
        match serialized_value {
            Value::Number(n) => {
                debug_assert_eq!(n.as_i64(), Some(value_to_serialize));
            }
            _ => panic!("Serialized value is not a number"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_414_llm_16_414 {
    use serde::{Serialize, Serializer};
    use crate::{
        value::{self, Value},
        map::Map,
    };
    #[derive(Serialize)]
    struct NewtypeStruct(i32);
    #[test]
    fn test_serialize_newtype_struct() {
        let _rug_st_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_serialize_newtype_struct = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "NewtypeStruct";
        let newtype_struct = NewtypeStruct(rug_fuzz_0);
        let serializer = value::Serializer;
        let expected_json = Value::Number(rug_fuzz_1.into());
        let result = serializer
            .serialize_newtype_struct(rug_fuzz_2, &newtype_struct)
            .unwrap();
        debug_assert_eq!(expected_json, result);
        let _rug_ed_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_serialize_newtype_struct = 0;
    }
    #[derive(Serialize)]
    struct NewtypeStructString(String);
    #[test]
    fn test_serialize_newtype_struct_string() {
        let _rug_st_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_serialize_newtype_struct_string = 0;
        let rug_fuzz_0 = "Hello World";
        let rug_fuzz_1 = "Hello World";
        let rug_fuzz_2 = "NewtypeStructString";
        let newtype_struct = NewtypeStructString(rug_fuzz_0.to_string());
        let serializer = value::Serializer;
        let expected_json = Value::String(rug_fuzz_1.to_string());
        let result = serializer
            .serialize_newtype_struct(rug_fuzz_2, &newtype_struct)
            .unwrap();
        debug_assert_eq!(expected_json, result);
        let _rug_ed_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_serialize_newtype_struct_string = 0;
    }
    #[derive(Serialize)]
    struct NewtypeStructMap(Map<String, Value>);
    #[test]
    fn test_serialize_newtype_struct_map() {
        let _rug_st_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_serialize_newtype_struct_map = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "value";
        let rug_fuzz_4 = "NewtypeStructMap";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let newtype_struct = NewtypeStructMap(map);
        let serializer = value::Serializer;
        let mut expected_map = Map::new();
        expected_map
            .insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let expected_json = Value::Object(expected_map);
        let result = serializer
            .serialize_newtype_struct(rug_fuzz_4, &newtype_struct)
            .unwrap();
        debug_assert_eq!(expected_json, result);
        let _rug_ed_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_serialize_newtype_struct_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_417 {
    use crate::value::ser::{SerializeVec, Serializer};
    use serde::ser::{SerializeSeq, Serializer as SerdeSerializer};
    #[test]
    fn serialize_seq_with_none_len() {
        let _rug_st_tests_llm_16_417_rrrruuuugggg_serialize_seq_with_none_len = 0;
        let serializer = Serializer;
        let result = serializer.serialize_seq(None);
        debug_assert!(result.is_ok());
        let serialize_seq = result.unwrap();
        debug_assert_eq!(serialize_seq.vec.capacity(), 0);
        let _rug_ed_tests_llm_16_417_rrrruuuugggg_serialize_seq_with_none_len = 0;
    }
    #[test]
    fn serialize_seq_with_some_len() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = Serializer;
        let result = serializer.serialize_seq(Some(rug_fuzz_0));
        debug_assert!(result.is_ok());
        let serialize_seq = result.unwrap();
        debug_assert_eq!(serialize_seq.vec.capacity(), 10);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_418 {
    use serde::Serialize;
    use crate::value::{self, Value, to_value};
    use crate::map::Map;
    use crate::error::Error;
    #[derive(Serialize)]
    struct TestStruct {
        id: i32,
        name: String,
        flag: bool,
    }
    #[test]
    fn serialize_some_with_struct() -> Result<(), Error> {
        let test_struct = TestStruct {
            id: 1,
            name: "Test".to_owned(),
            flag: true,
        };
        let serialized = to_value(test_struct)?;
        let expected = json!({ "id" : 1, "name" : "Test", "flag" : true });
        assert_eq!(serialized, expected);
        Ok(())
    }
    #[test]
    fn serialize_some_with_map() -> Result<(), Error> {
        let mut test_map = Map::new();
        test_map.insert("key1".to_string(), Value::String("value1".to_string()));
        test_map.insert("key2".to_string(), Value::Number(2.into()));
        let serialized = to_value(test_map)?;
        let mut expected_map = Map::new();
        expected_map.insert("key1".to_string(), Value::String("value1".to_string()));
        expected_map.insert("key2".to_string(), Value::Number(2.into()));
        let expected = Value::Object(expected_map);
        assert_eq!(serialized, expected);
        Ok(())
    }
    #[test]
    fn serialize_some_with_option() -> Result<(), Error> {
        let test_option: Option<String> = Some("test".to_owned());
        let serialized = to_value(test_option)?;
        let expected = Value::String("test".to_owned());
        assert_eq!(serialized, expected);
        Ok(())
    }
    #[test]
    fn serialize_some_with_none() -> Result<(), Error> {
        let test_option: Option<String> = None;
        let serialized = to_value(test_option)?;
        assert_eq!(serialized, Value::Null);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_421 {
    use super::*;
    use crate::*;
    use crate::{Map, Value};
    use serde::{Serialize, Serializer};
    #[test]
    fn test_serialize_struct_variant() {
        let _rug_st_tests_llm_16_421_rrrruuuugggg_test_serialize_struct_variant = 0;
        let rug_fuzz_0 = "test_variant";
        let rug_fuzz_1 = "TestStruct";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let serializer = Serializer;
        let variant = rug_fuzz_0;
        let result = serializer
            .serialize_struct_variant(rug_fuzz_1, rug_fuzz_2, variant, rug_fuzz_3);
        debug_assert!(result.is_ok());
        let struct_variant = result.unwrap();
        debug_assert_eq!(struct_variant.name, variant);
        debug_assert!(struct_variant.map.is_empty());
        let _rug_ed_tests_llm_16_421_rrrruuuugggg_test_serialize_struct_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_423 {
    use serde::Serializer;
    use crate::value::Serializer as JsonSerializer;
    use crate::{to_value, Value};
    #[test]
    fn test_serialize_tuple_struct() {
        let _rug_st_tests_llm_16_423_rrrruuuugggg_test_serialize_tuple_struct = 0;
        let rug_fuzz_0 = "MyTupleStruct";
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = "a";
        let rug_fuzz_3 = "b";
        let rug_fuzz_4 = "c";
        let serializer = JsonSerializer;
        let tuple_struct_name = rug_fuzz_0;
        let len = rug_fuzz_1;
        let result = serializer.serialize_tuple_struct(tuple_struct_name, len).unwrap();
        debug_assert_eq!(result.vec.capacity(), len);
        let expected = Value::Array(vec![Value::Null; len]);
        let my_tuple_struct = (rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let my_tuple_struct_value = to_value(my_tuple_struct).unwrap();
        debug_assert_eq!(my_tuple_struct_value, expected);
        let _rug_ed_tests_llm_16_423_rrrruuuugggg_test_serialize_tuple_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_427 {
    use crate::{value::ser::Serializer, Value};
    use serde::Serializer as SerdeSerializer;
    #[test]
    fn serialize_u32_min_value() {
        let _rug_st_tests_llm_16_427_rrrruuuugggg_serialize_u32_min_value = 0;
        let serializer = Serializer;
        let min_u32 = u32::MIN;
        let serialized_min_u32 = serializer.serialize_u32(min_u32).unwrap();
        debug_assert_eq!(serialized_min_u32, Value::Number(min_u32.into()));
        let _rug_ed_tests_llm_16_427_rrrruuuugggg_serialize_u32_min_value = 0;
    }
    #[test]
    fn serialize_u32_max_value() {
        let _rug_st_tests_llm_16_427_rrrruuuugggg_serialize_u32_max_value = 0;
        let serializer = Serializer;
        let max_u32 = u32::MAX;
        let serialized_max_u32 = serializer.serialize_u32(max_u32).unwrap();
        debug_assert_eq!(serialized_max_u32, Value::Number(max_u32.into()));
        let _rug_ed_tests_llm_16_427_rrrruuuugggg_serialize_u32_max_value = 0;
    }
    #[test]
    fn serialize_u32_arbitrary_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = Serializer;
        let value: u32 = rug_fuzz_0;
        let serialized_value = serializer.serialize_u32(value).unwrap();
        debug_assert_eq!(serialized_value, Value::Number(value.into()));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_429 {
    use crate::value::ser::Serializer;
    use serde::Serializer as _;
    use crate::{Value, Number};
    #[test]
    fn test_serialize_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = Serializer;
        let value = rug_fuzz_0;
        let result = serializer.serialize_u8(value).unwrap();
        match result {
            Value::Number(num) => {
                debug_assert_eq!(num, Number::from(value));
            }
            _ => panic!("serialize_u8 did not produce a Number"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_430_llm_16_430 {
    use serde::Serializer;
    use crate::value::ser::Serializer as ValueSerializer;
    use crate::Value;
    #[test]
    fn test_serialize_unit() {
        let _rug_st_tests_llm_16_430_llm_16_430_rrrruuuugggg_test_serialize_unit = 0;
        let serializer = ValueSerializer;
        let value = serializer.serialize_unit().unwrap();
        debug_assert_eq!(value, Value::Null);
        let _rug_ed_tests_llm_16_430_llm_16_430_rrrruuuugggg_test_serialize_unit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_431 {
    use crate::value::Serializer;
    use serde::{Serialize, Serializer as _};
    use crate::{Value, Error};
    #[derive(Serialize)]
    struct UnitStruct;
    #[test]
    fn test_serialize_unit_struct() -> Result<(), Error> {
        let serializer = Serializer;
        let value = UnitStruct.serialize(serializer)?;
        assert_eq!(value, Value::Null);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_432 {
    use super::*;
    use crate::*;
    use serde::{Serialize, Serializer};
    use crate::{value::Serializer as JsonSerializer, Value};
    #[test]
    fn test_serialize_unit_variant() {
        let _rug_st_tests_llm_16_432_rrrruuuugggg_test_serialize_unit_variant = 0;
        let rug_fuzz_0 = "TestEnum";
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = "VariantName";
        let serializer = JsonSerializer;
        let name = rug_fuzz_0;
        let variant_index = rug_fuzz_1;
        let variant = rug_fuzz_2;
        let result = serializer.serialize_unit_variant(name, variant_index, variant);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), Value::String(variant.to_owned()));
        let _rug_ed_tests_llm_16_432_rrrruuuugggg_test_serialize_unit_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_796 {
    use super::*;
    use crate::*;
    use crate::{json, value::Value, Map};
    #[test]
    fn test_serialize_null() {
        let _rug_st_tests_llm_16_796_rrrruuuugggg_test_serialize_null = 0;
        let value = Value::Null;
        let serialized = crate::to_string(&value).unwrap();
        debug_assert_eq!(serialized, "null");
        let _rug_ed_tests_llm_16_796_rrrruuuugggg_test_serialize_null = 0;
    }
    #[test]
    fn test_serialize_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        let serialized = crate::to_string(&value).unwrap();
        debug_assert_eq!(serialized, "true");
        let value = Value::Bool(rug_fuzz_1);
        let serialized = crate::to_string(&value).unwrap();
        debug_assert_eq!(serialized, "false");
             }
});    }
    #[test]
    fn test_serialize_number() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(crate::Number::from(rug_fuzz_0));
        let serialized = crate::to_string(&value).unwrap();
        debug_assert_eq!(serialized, "42");
             }
});    }
    #[test]
    fn test_serialize_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_string());
        let serialized = crate::to_string(&value).unwrap();
        debug_assert_eq!(serialized, "\"Hello, World!\"");
             }
});    }
    #[test]
    fn test_serialize_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(
            vec![json!(rug_fuzz_0), json!("two"), json!(null), json!([true, false])],
        );
        let serialized = crate::to_string(&value).unwrap();
        debug_assert_eq!(serialized, "[1,\"two\",null,[true,false]]");
             }
});    }
    #[test]
    fn test_serialize_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, bool, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), json!(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), json!(rug_fuzz_3));
        let value = Value::Object(map);
        let serialized = crate::to_string(&value).unwrap();
        debug_assert!(serialized.contains(rug_fuzz_4));
        debug_assert!(serialized.contains(rug_fuzz_5));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_797 {
    use crate::error::{Error, ErrorCode};
    use crate::value::ser::key_must_be_a_string;
    #[test]
    fn test_key_must_be_a_string_error() {
        let _rug_st_tests_llm_16_797_rrrruuuugggg_test_key_must_be_a_string_error = 0;
        let error = key_must_be_a_string();
        debug_assert!(error.is_syntax());
        debug_assert_eq!(error.line(), 0);
        debug_assert_eq!(error.column(), 0);
        match error.classify() {
            crate::error::Category::Syntax => {}
            _ => panic!("Error should be of syntax error category"),
        }
        debug_assert_eq!(error.to_string(), "key must be a string at line 0 column 0");
        let _rug_ed_tests_llm_16_797_rrrruuuugggg_test_key_must_be_a_string_error = 0;
    }
}
