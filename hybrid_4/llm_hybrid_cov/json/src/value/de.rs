use crate::error::Error;
use crate::map::Map;
use crate::number::Number;
use crate::value::Value;
use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
#[cfg(feature = "raw_value")]
use alloc::string::ToString;
use alloc::vec::{self, Vec};
use core::fmt;
use core::slice;
use core::str::FromStr;
use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, Expected, IntoDeserializer,
    MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;
#[cfg(feature = "arbitrary_precision")]
use crate::number::NumberFromString;
impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }
            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Bool(value))
            }
            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }
            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }
            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Number::from_f64(value).map_or(Value::Null, Value::Number))
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value))
            }
            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }
            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }
            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }
            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(elem) = tri!(visitor.next_element()) {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                match visitor.next_key_seed(KeyClassifier)? {
                    #[cfg(feature = "arbitrary_precision")]
                    Some(KeyClass::Number) => {
                        let number: NumberFromString = visitor.next_value()?;
                        Ok(Value::Number(number.value))
                    }
                    #[cfg(feature = "raw_value")]
                    Some(KeyClass::RawValue) => {
                        let value = visitor
                            .next_value_seed(crate::raw::BoxedFromString)?;
                        crate::from_str(value.get()).map_err(de::Error::custom)
                    }
                    Some(KeyClass::Map(first_key)) => {
                        let mut values = Map::new();
                        values.insert(first_key, tri!(visitor.next_value()));
                        while let Some((key, value)) = tri!(visitor.next_entry()) {
                            values.insert(key, value);
                        }
                        Ok(Value::Object(values))
                    }
                    None => Ok(Value::Object(Map::new())),
                }
            }
        }
        deserializer.deserialize_any(ValueVisitor)
    }
}
impl FromStr for Value {
    type Err = Error;
    fn from_str(s: &str) -> Result<Value, Error> {
        super::super::de::from_str(s)
    }
}
macro_rules! deserialize_number {
    ($method:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn $method < V > (self, visitor : V)
        -> Result < V::Value, Error > where V : Visitor <'de >, { match self {
        Value::Number(n) => n.deserialize_any(visitor), _ => Err(self.invalid_type(&
        visitor)), } } #[cfg(feature = "arbitrary_precision")] fn $method < V > (self,
        visitor : V) -> Result < V::Value, Error > where V : Visitor <'de >, { match self
        { Value::Number(n) => n.$method (visitor), _ => self.deserialize_any(visitor), }
        }
    };
}
fn visit_array<'de, V>(array: Vec<Value>, visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = array.len();
    let mut deserializer = SeqDeserializer::new(array);
    let seq = tri!(visitor.visit_seq(& mut deserializer));
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
    }
}
fn visit_object<'de, V>(
    object: Map<String, Value>,
    visitor: V,
) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = object.len();
    let mut deserializer = MapDeserializer::new(object);
    let map = tri!(visitor.visit_map(& mut deserializer));
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(map)
    } else {
        Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
    }
}
impl<'de> serde::Deserializer<'de> for Value {
    type Error = Error;
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Number(n) => n.deserialize_any(visitor),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            Value::Array(v) => visit_array(v, visitor),
            Value::Object(v) => visit_object(v, visitor),
        }
    }
    deserialize_number!(deserialize_i8);
    deserialize_number!(deserialize_i16);
    deserialize_number!(deserialize_i32);
    deserialize_number!(deserialize_i64);
    deserialize_number!(deserialize_i128);
    deserialize_number!(deserialize_u8);
    deserialize_number!(deserialize_u16);
    deserialize_number!(deserialize_u32);
    deserialize_number!(deserialize_u64);
    deserialize_number!(deserialize_u128);
    deserialize_number!(deserialize_f32);
    deserialize_number!(deserialize_f64);
    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }
    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self {
            Value::Object(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(
                            serde::de::Error::invalid_value(
                                Unexpected::Map,
                                &"map with a single key",
                            ),
                        );
                    }
                };
                if iter.next().is_some() {
                    return Err(
                        serde::de::Error::invalid_value(
                            Unexpected::Map,
                            &"map with a single key",
                        ),
                    );
                }
                (variant, Some(value))
            }
            Value::String(variant) => (variant, None),
            other => {
                return Err(
                    serde::de::Error::invalid_type(other.unexpected(), &"string or map"),
                );
            }
        };
        visitor.visit_enum(EnumDeserializer { variant, value })
    }
    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "raw_value")]
        {
            if name == crate::raw::TOKEN {
                return visitor
                    .visit_map(crate::raw::OwnedRawDeserializer {
                        raw_value: Some(self.to_string()),
                    });
            }
        }
        let _ = name;
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Bool(v) => visitor.visit_bool(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            Value::Array(v) => visit_array(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Array(v) => visit_array(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Object(v) => visit_object(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Array(v) => visit_array(v, visitor),
            Value::Object(v) => visit_object(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }
}
struct EnumDeserializer {
    variant: String,
    value: Option<Value>,
}
impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer {
            value: self.value,
        };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}
impl<'de> IntoDeserializer<'de, Error> for Value {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
struct VariantDeserializer {
    value: Option<Value>,
}
impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = Error;
    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Some(value) => Deserialize::deserialize(value),
            None => Ok(()),
        }
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => {
                Err(
                    serde::de::Error::invalid_type(
                        Unexpected::UnitVariant,
                        &"newtype variant",
                    ),
                )
            }
        }
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Array(v)) => {
                if v.is_empty() { visitor.visit_unit() } else { visit_array(v, visitor) }
            }
            Some(other) => {
                Err(serde::de::Error::invalid_type(other.unexpected(), &"tuple variant"))
            }
            None => {
                Err(
                    serde::de::Error::invalid_type(
                        Unexpected::UnitVariant,
                        &"tuple variant",
                    ),
                )
            }
        }
    }
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Object(v)) => visit_object(v, visitor),
            Some(other) => {
                Err(
                    serde::de::Error::invalid_type(other.unexpected(), &"struct variant"),
                )
            }
            None => {
                Err(
                    serde::de::Error::invalid_type(
                        Unexpected::UnitVariant,
                        &"struct variant",
                    ),
                )
            }
        }
    }
}
struct SeqDeserializer {
    iter: vec::IntoIter<Value>,
}
impl SeqDeserializer {
    fn new(vec: Vec<Value>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}
impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
struct MapDeserializer {
    iter: <Map<String, Value> as IntoIterator>::IntoIter,
    value: Option<Value>,
}
impl MapDeserializer {
    fn new(map: Map<String, Value>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}
impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = Error;
    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = MapKeyDeserializer {
                    key: Cow::Owned(key),
                };
                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }
    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
macro_rules! deserialize_value_ref_number {
    ($method:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn $method < V > (self, visitor : V)
        -> Result < V::Value, Error > where V : Visitor <'de >, { match self {
        Value::Number(n) => n.deserialize_any(visitor), _ => Err(self.invalid_type(&
        visitor)), } } #[cfg(feature = "arbitrary_precision")] fn $method < V > (self,
        visitor : V) -> Result < V::Value, Error > where V : Visitor <'de >, { match self
        { Value::Number(n) => n.$method (visitor), _ => self.deserialize_any(visitor), }
        }
    };
}
fn visit_array_ref<'de, V>(array: &'de [Value], visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = array.len();
    let mut deserializer = SeqRefDeserializer::new(array);
    let seq = tri!(visitor.visit_seq(& mut deserializer));
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
    }
}
fn visit_object_ref<'de, V>(
    object: &'de Map<String, Value>,
    visitor: V,
) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = object.len();
    let mut deserializer = MapRefDeserializer::new(object);
    let map = tri!(visitor.visit_map(& mut deserializer));
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(map)
    } else {
        Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
    }
}
impl<'de> serde::Deserializer<'de> for &'de Value {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::Number(n) => n.deserialize_any(visitor),
            Value::String(v) => visitor.visit_borrowed_str(v),
            Value::Array(v) => visit_array_ref(v, visitor),
            Value::Object(v) => visit_object_ref(v, visitor),
        }
    }
    deserialize_value_ref_number!(deserialize_i8);
    deserialize_value_ref_number!(deserialize_i16);
    deserialize_value_ref_number!(deserialize_i32);
    deserialize_value_ref_number!(deserialize_i64);
    deserialize_number!(deserialize_i128);
    deserialize_value_ref_number!(deserialize_u8);
    deserialize_value_ref_number!(deserialize_u16);
    deserialize_value_ref_number!(deserialize_u32);
    deserialize_value_ref_number!(deserialize_u64);
    deserialize_number!(deserialize_u128);
    deserialize_value_ref_number!(deserialize_f32);
    deserialize_value_ref_number!(deserialize_f64);
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self {
            Value::Object(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(
                            serde::de::Error::invalid_value(
                                Unexpected::Map,
                                &"map with a single key",
                            ),
                        );
                    }
                };
                if iter.next().is_some() {
                    return Err(
                        serde::de::Error::invalid_value(
                            Unexpected::Map,
                            &"map with a single key",
                        ),
                    );
                }
                (variant, Some(value))
            }
            Value::String(variant) => (variant, None),
            other => {
                return Err(
                    serde::de::Error::invalid_type(other.unexpected(), &"string or map"),
                );
            }
        };
        visitor
            .visit_enum(EnumRefDeserializer {
                variant,
                value,
            })
    }
    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "raw_value")]
        {
            if name == crate::raw::TOKEN {
                return visitor
                    .visit_map(crate::raw::OwnedRawDeserializer {
                        raw_value: Some(self.to_string()),
                    });
            }
        }
        let _ = name;
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            Value::Bool(v) => visitor.visit_bool(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(v) => visitor.visit_borrowed_str(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(v) => visitor.visit_borrowed_str(v),
            Value::Array(v) => visit_array_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            Value::Null => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Array(v) => visit_array_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Object(v) => visit_object_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Array(v) => visit_array_ref(v, visitor),
            Value::Object(v) => visit_object_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
struct EnumRefDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de Value>,
}
impl<'de> EnumAccess<'de> for EnumRefDeserializer<'de> {
    type Error = Error;
    type Variant = VariantRefDeserializer<'de>;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantRefDeserializer {
            value: self.value,
        };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}
struct VariantRefDeserializer<'de> {
    value: Option<&'de Value>,
}
impl<'de> VariantAccess<'de> for VariantRefDeserializer<'de> {
    type Error = Error;
    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Some(value) => Deserialize::deserialize(value),
            None => Ok(()),
        }
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => {
                Err(
                    serde::de::Error::invalid_type(
                        Unexpected::UnitVariant,
                        &"newtype variant",
                    ),
                )
            }
        }
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Array(v)) => {
                if v.is_empty() {
                    visitor.visit_unit()
                } else {
                    visit_array_ref(v, visitor)
                }
            }
            Some(other) => {
                Err(serde::de::Error::invalid_type(other.unexpected(), &"tuple variant"))
            }
            None => {
                Err(
                    serde::de::Error::invalid_type(
                        Unexpected::UnitVariant,
                        &"tuple variant",
                    ),
                )
            }
        }
    }
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Object(v)) => visit_object_ref(v, visitor),
            Some(other) => {
                Err(
                    serde::de::Error::invalid_type(other.unexpected(), &"struct variant"),
                )
            }
            None => {
                Err(
                    serde::de::Error::invalid_type(
                        Unexpected::UnitVariant,
                        &"struct variant",
                    ),
                )
            }
        }
    }
}
struct SeqRefDeserializer<'de> {
    iter: slice::Iter<'de, Value>,
}
impl<'de> SeqRefDeserializer<'de> {
    fn new(slice: &'de [Value]) -> Self {
        SeqRefDeserializer {
            iter: slice.iter(),
        }
    }
}
impl<'de> SeqAccess<'de> for SeqRefDeserializer<'de> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
struct MapRefDeserializer<'de> {
    iter: <&'de Map<String, Value> as IntoIterator>::IntoIter,
    value: Option<&'de Value>,
}
impl<'de> MapRefDeserializer<'de> {
    fn new(map: &'de Map<String, Value>) -> Self {
        MapRefDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}
impl<'de> MapAccess<'de> for MapRefDeserializer<'de> {
    type Error = Error;
    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = MapKeyDeserializer {
                    key: Cow::Borrowed(&**key),
                };
                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }
    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
struct MapKeyDeserializer<'de> {
    key: Cow<'de, str>,
}
macro_rules! deserialize_integer_key {
    ($method:ident => $visit:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value, Error > where V :
        Visitor <'de >, { match (self.key.parse(), self.key) { (Ok(integer), _) =>
        visitor.$visit (integer), (Err(_), Cow::Borrowed(s)) => visitor
        .visit_borrowed_str(s), #[cfg(any(feature = "std", feature = "alloc"))] (Err(_),
        Cow::Owned(s)) => visitor.visit_string(s), } }
    };
}
impl<'de> serde::Deserializer<'de> for MapKeyDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        BorrowedCowStrDeserializer::new(self.key).deserialize_any(visitor)
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
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }
    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.key.into_deserializer().deserialize_enum(name, variants, visitor)
    }
    forward_to_deserialize_any! {
        bool f32 f64 char str string bytes byte_buf unit unit_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
struct KeyClassifier;
enum KeyClass {
    Map(String),
    #[cfg(feature = "arbitrary_precision")]
    Number,
    #[cfg(feature = "raw_value")]
    RawValue,
}
impl<'de> DeserializeSeed<'de> for KeyClassifier {
    type Value = KeyClass;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}
impl<'de> Visitor<'de> for KeyClassifier {
    type Value = KeyClass;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string key")
    }
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match s {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(KeyClass::Number),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(KeyClass::RawValue),
            _ => Ok(KeyClass::Map(s.to_owned())),
        }
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match s.as_str() {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(KeyClass::Number),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(KeyClass::RawValue),
            _ => Ok(KeyClass::Map(s)),
        }
    }
}
impl Value {
    #[cold]
    fn invalid_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(self.unexpected(), exp)
    }
    #[cold]
    fn unexpected(&self) -> Unexpected {
        match self {
            Value::Null => Unexpected::Unit,
            Value::Bool(b) => Unexpected::Bool(*b),
            Value::Number(n) => n.unexpected(),
            Value::String(s) => Unexpected::Str(s),
            Value::Array(_) => Unexpected::Seq,
            Value::Object(_) => Unexpected::Map,
        }
    }
}
struct BorrowedCowStrDeserializer<'de> {
    value: Cow<'de, str>,
}
impl<'de> BorrowedCowStrDeserializer<'de> {
    fn new(value: Cow<'de, str>) -> Self {
        BorrowedCowStrDeserializer {
            value,
        }
    }
}
impl<'de> de::Deserializer<'de> for BorrowedCowStrDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Cow::Owned(string) => visitor.visit_string(string),
        }
    }
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }
    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct
        identifier ignored_any
    }
}
impl<'de> de::EnumAccess<'de> for BorrowedCowStrDeserializer<'de> {
    type Error = Error;
    type Variant = UnitOnly;
    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self)?;
        Ok((value, UnitOnly))
    }
}
struct UnitOnly;
impl<'de> de::VariantAccess<'de> for UnitOnly {
    type Error = Error;
    fn unit_variant(self) -> Result<(), Error> {
        Ok(())
    }
    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
    }
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
    }
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant"))
    }
}
#[cfg(test)]
mod tests_llm_16_308 {
    use crate::{Value, Deserializer};
    use serde::de::{self, Deserialize, Visitor, SeqAccess};
    use std::fmt;
    #[test]
    fn test_visit_some() {
        struct TestVisitor;
        impl<'de> Visitor<'de> for TestVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Null)
            }
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut seq = Vec::new();
                while let Some(elem) = visitor.next_element()? {
                    seq.push(elem);
                }
                Ok(Value::Array(seq))
            }
            fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("Not expecting a bool"))
            }
        }
        let json_str = r#"["test", 1, true, null]"#;
        let mut de = Deserializer::from_str(json_str);
        let visitor = TestVisitor;
        let value = visitor.visit_some(&mut de).unwrap();
        assert_eq!(
            value, Value::Array(vec![Value::String("test".to_owned()), Value::Number(1
            .into()), Value::Bool(true), Value::Null,])
        );
    }
}
#[cfg(test)]
mod tests_llm_16_309_llm_16_309 {
    use crate::{value::Value, Error};
    use serde::de::{self, Visitor};
    use std::fmt;
    struct TestValueVisitor;
    impl<'de> Visitor<'de> for TestValueVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any valid JSON value")
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn visit_str<E>(self, value: &str) -> Result<Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::String(value.to_owned()))
        }
    }
    #[test]
    fn test_visit_str() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_visit_str = 0;
        let rug_fuzz_0 = "test string";
        let visitor = TestValueVisitor;
        let test_str = rug_fuzz_0;
        let result: Result<Value, Error> = visitor.visit_str(test_str);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), Value::String(test_str.to_string()));
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_visit_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_313_llm_16_313 {
    use super::*;
    use crate::*;
    use crate::error::Error;
    use crate::value::de::BorrowedCowStrDeserializer;
    use serde::de::{
        self, Deserialize, DeserializeSeed, Deserializer, EnumAccess,
        Error as SerdeError, MapAccess, Visitor,
    };
    use crate::Value;
    use std::borrow::Cow;
    use std::collections::HashMap as Map;
    use std::fmt;
    use std::string::String;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a JSON value")
        }
        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: SerdeError,
        {
            Ok(Value::String(v.to_owned()))
        }
        fn visit_borrowed_str<E>(
            self,
            v: &'de str,
        ) -> std::result::Result<Self::Value, E>
        where
            E: SerdeError,
        {
            Ok(Value::String(v.to_owned()))
        }
        fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
        where
            E: SerdeError,
        {
            Ok(Value::String(v))
        }
    }
    #[test]
    fn test_deserialize_any_borrowed() {
        let _rug_st_tests_llm_16_313_llm_16_313_rrrruuuugggg_test_deserialize_any_borrowed = 0;
        let rug_fuzz_0 = "borrowed string";
        let str = rug_fuzz_0;
        let deserializer = BorrowedCowStrDeserializer::new(Cow::Borrowed(str));
        let result: Value = deserializer.deserialize_any(TestVisitor).unwrap();
        debug_assert_eq!(result, Value::String(str.to_owned()));
        let _rug_ed_tests_llm_16_313_llm_16_313_rrrruuuugggg_test_deserialize_any_borrowed = 0;
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn test_deserialize_any_owned() {
        let _rug_st_tests_llm_16_313_llm_16_313_rrrruuuugggg_test_deserialize_any_owned = 0;
        let rug_fuzz_0 = "owned string";
        let str = rug_fuzz_0.to_owned();
        let deserializer = BorrowedCowStrDeserializer::new(Cow::Owned(str.clone()));
        let result: Value = deserializer.deserialize_any(TestVisitor).unwrap();
        debug_assert_eq!(result, Value::String(str));
        let _rug_ed_tests_llm_16_313_llm_16_313_rrrruuuugggg_test_deserialize_any_owned = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_314_llm_16_314 {
    use serde::de::{self, Deserialize, DeserializeSeed, IntoDeserializer};
    use serde::de::value::BorrowedStrDeserializer;
    use crate::error::Error;
    use std::borrow::Cow;
    use std::fmt;
    #[derive(Debug, PartialEq)]
    enum TestEnum {
        VariantA,
        VariantB,
    }
    struct TestEnumVisitor;
    impl<'de> de::Visitor<'de> for TestEnumVisitor {
        type Value = TestEnum;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a test enum")
        }
        fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where
            A: de::EnumAccess<'de>,
        {
            let (variant, _) = data.variant_seed(TestEnumVariantSeed)?;
            Ok(variant)
        }
    }
    struct TestEnumVariantSeed;
    impl<'de> DeserializeSeed<'de> for TestEnumVariantSeed {
        type Value = TestEnum;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct VariantVisitor;
            impl<'de> de::Visitor<'de> for VariantVisitor {
                type Value = TestEnum;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a test enum variant")
                }
                fn visit_str<E>(self, value: &str) -> Result<TestEnum, E>
                where
                    E: de::Error,
                {
                    match value {
                        "VariantA" => Ok(TestEnum::VariantA),
                        "VariantB" => Ok(TestEnum::VariantB),
                        _ => Err(E::custom("unexpected variant")),
                    }
                }
            }
            deserializer.deserialize_identifier(VariantVisitor)
        }
    }
    impl<'de> Deserialize<'de> for TestEnum {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer
                .deserialize_enum("TestEnum", &["VariantA", "VariantB"], TestEnumVisitor)
        }
    }
    #[test]
    fn test_deserialize_enum_variant_a() -> Result<(), Error> {
        let value = "VariantA";
        let deserializer = BorrowedStrDeserializer::<Error>::new(value);
        let enum_value: TestEnum = TestEnum::deserialize(deserializer)?;
        assert_eq!(enum_value, TestEnum::VariantA);
        Ok(())
    }
    #[test]
    fn test_deserialize_enum_variant_b() -> Result<(), Error> {
        let value = "VariantB";
        let deserializer = BorrowedStrDeserializer::<Error>::new(value);
        let enum_value: TestEnum = TestEnum::deserialize(deserializer)?;
        assert_eq!(enum_value, TestEnum::VariantB);
        Ok(())
    }
    #[test]
    fn test_deserialize_enum_invalid_variant() {
        let value = "VariantC";
        let deserializer = BorrowedStrDeserializer::<Error>::new(value);
        let result: Result<TestEnum, _> = TestEnum::deserialize(deserializer);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_325_llm_16_325 {
    use serde::de::{self, Deserializer, Visitor};
    use crate::error::Error;
    use crate::value::de::MapKeyDeserializer;
    use crate::Map;
    use crate::Value;
    use std::borrow::Cow;
    use std::fmt;
    use crate::number::Number;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any valid JSON value")
        }
        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::Number(Number::from(v)))
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Value::String(v.to_owned()))
        }
    }
    #[test]
    fn test_deserialize_any() {
        let _rug_st_tests_llm_16_325_llm_16_325_rrrruuuugggg_test_deserialize_any = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = "hello";
        let key = Cow::Borrowed(rug_fuzz_0);
        let map_key_deserializer = MapKeyDeserializer {
            key: key.clone(),
        };
        let test_visitor = TestVisitor;
        let result: Result<Value, Error> = map_key_deserializer
            .deserialize_any(test_visitor);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), Value::Number(Number::from(42i8)));
        let key = Cow::Borrowed(rug_fuzz_1);
        let map_key_deserializer = MapKeyDeserializer {
            key: key.clone(),
        };
        let test_visitor = TestVisitor;
        let result: Result<Value, Error> = map_key_deserializer
            .deserialize_any(test_visitor);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), Value::String("hello".to_owned()));
        let _rug_ed_tests_llm_16_325_llm_16_325_rrrruuuugggg_test_deserialize_any = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_333 {
    use serde::de::{self, Deserialize, Deserializer, Visitor, Error};
    use crate::value::{self, Map, Value};
    use crate::Error as SerdeJsonError;
    use std::borrow::Cow;
    use std::fmt;
    use std::marker::PhantomData;
    struct MockVisitor<'de> {
        marker: PhantomData<&'de ()>,
    }
    impl<'de> MockVisitor<'de> {
        fn new() -> Self {
            MockVisitor { marker: PhantomData }
        }
    }
    impl<'de> Visitor<'de> for MockVisitor<'de> {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a map value")
        }
        fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Map::new())
        }
    }
    struct TestDeserializer<'de> {
        value: Cow<'de, str>,
    }
    impl<'de> Deserializer<'de> for TestDeserializer<'de> {
        type Error = SerdeJsonError;
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            value::de::MapKeyDeserializer::<'de> {
                key: self.value,
            }
                .deserialize_option(visitor)
        }
        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 i128 u128 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map
            struct enum identifier ignored_any
        }
    }
    #[test]
    fn deserialize_option_for_map_key_deserializer() {
        let _rug_st_tests_llm_16_333_rrrruuuugggg_deserialize_option_for_map_key_deserializer = 0;
        let rug_fuzz_0 = "test_key";
        let key = Cow::Borrowed(rug_fuzz_0);
        let deserializer = TestDeserializer { value: key };
        let result: Result<Map<String, Value>, SerdeJsonError> = Deserialize::deserialize(
            deserializer,
        );
        debug_assert!(result.is_ok());
        debug_assert!(result.unwrap().is_empty());
        let _rug_ed_tests_llm_16_333_rrrruuuugggg_deserialize_option_for_map_key_deserializer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_336 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::value::{
        de::{MapKeyDeserializer, Error},
        Map, Value,
    };
    use std::borrow::Cow;
    use std::fmt;
    struct U32Visitor;
    impl<'de> Visitor<'de> for U32Visitor {
        type Value = u32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned 32-bit integer")
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
    #[test]
    fn test_deserialize_u32() -> Result<(), Error> {
        let deserializer = MapKeyDeserializer {
            key: Cow::Borrowed("123"),
        };
        let visitor = U32Visitor;
        let value = deserializer.deserialize_u32(visitor)?;
        assert_eq!(value, 123);
        let deserializer = MapKeyDeserializer {
            key: Cow::Borrowed("abc"),
        };
        let visitor = U32Visitor;
        let result = deserializer.deserialize_u32(visitor);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn test_deserialize_u32_owned_string() -> Result<(), Error> {
        let deserializer = MapKeyDeserializer {
            key: Cow::Owned("456".to_string()),
        };
        let visitor = U32Visitor;
        let value = deserializer.deserialize_u32(visitor)?;
        assert_eq!(value, 456);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_348_llm_16_348 {
    use serde::de::{self, Visitor, SeqAccess, VariantAccess};
    use crate::value::{self, Map, Value};
    use crate::error::Error;
    use std::fmt;
    use std::string::String;
    struct TupleVisitor;
    impl<'de> Visitor<'de> for TupleVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple variant")
        }
        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Ok(Map::new())
        }
    }
    #[test]
    fn test_tuple_variant_error() {
        let _rug_st_tests_llm_16_348_llm_16_348_rrrruuuugggg_test_tuple_variant_error = 0;
        let rug_fuzz_0 = 0;
        let unit_only = value::de::UnitOnly;
        let visitor = TupleVisitor;
        let result: Result<Map<String, Value>, Error> = unit_only
            .tuple_variant(rug_fuzz_0, visitor);
        debug_assert!(result.is_err());
        debug_assert_eq!(
            result.unwrap_err().to_string(),
            "invalid type: unit variant, expected a tuple variant".to_string()
        );
        let _rug_ed_tests_llm_16_348_llm_16_348_rrrruuuugggg_test_tuple_variant_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_349_llm_16_349 {
    use crate::error::Error;
    use crate::value::de::UnitOnly;
    use serde::de::{self, DeserializeSeed, VariantAccess};
    #[test]
    fn test_unit_variant() {
        let unit_only = UnitOnly;
        let result = unit_only.unit_variant();
        assert!(result.is_ok());
    }
    #[test]
    fn test_newtype_variant_seed() {
        struct TestSeed;
        impl<'de> DeserializeSeed<'de> for TestSeed {
            type Value = String;
            fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                Ok(String::from("test"))
            }
        }
        let unit_only = UnitOnly;
        let seed = TestSeed;
        let result: Result<String, Error> = unit_only.newtype_variant_seed(seed);
        assert!(result.is_err());
    }
    #[test]
    fn test_tuple_variant() {
        struct TestVisitor;
        impl<'de> de::Visitor<'de> for TestVisitor {
            type Value = ();
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("not expecting anything")
            }
            fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                Ok(())
            }
        }
        let unit_only = UnitOnly;
        let visitor = TestVisitor;
        let result = unit_only.tuple_variant(0, visitor);
        assert!(result.is_err());
    }
    #[test]
    fn test_struct_variant() {
        struct TestVisitor;
        impl<'de> de::Visitor<'de> for TestVisitor {
            type Value = ();
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("not expecting anything")
            }
            fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                Ok(())
            }
        }
        let unit_only = UnitOnly;
        let visitor = TestVisitor;
        let result = unit_only.struct_variant(&[], visitor);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_357_llm_16_357 {
    use crate::value::{self, Value};
    use crate::error::Error;
    use serde::de::{VariantAccess, Deserialize};
    #[test]
    fn test_unit_variant_with_none() {
        let _rug_st_tests_llm_16_357_llm_16_357_rrrruuuugggg_test_unit_variant_with_none = 0;
        let deserializer = value::de::VariantRefDeserializer {
            value: None,
        };
        let result = <value::de::VariantRefDeserializer as serde::de::VariantAccess>::unit_variant(
            deserializer,
        );
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_357_llm_16_357_rrrruuuugggg_test_unit_variant_with_none = 0;
    }
    #[test]
    fn test_unit_variant_with_some() {
        let _rug_st_tests_llm_16_357_llm_16_357_rrrruuuugggg_test_unit_variant_with_some = 0;
        let value = Value::Null;
        let deserializer = value::de::VariantRefDeserializer {
            value: Some(&value),
        };
        let result = <value::de::VariantRefDeserializer as serde::de::VariantAccess>::unit_variant(
            deserializer,
        );
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_357_llm_16_357_rrrruuuugggg_test_unit_variant_with_some = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_628 {
    use crate::{value::Value, Deserializer};
    use serde::Deserialize;
    use std::fmt;
    #[test]
    fn deserialize_null() -> Result<(), crate::Error> {
        let json = "null";
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        assert_eq!(value, Value::Null);
        Ok(())
    }
    #[test]
    fn deserialize_bool() -> Result<(), crate::Error> {
        let json = "true";
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        assert_eq!(value, Value::Bool(true));
        Ok(())
    }
    #[test]
    fn deserialize_number() -> Result<(), crate::Error> {
        let json = "1234";
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        assert_eq!(value, Value::Number(1234.into()));
        Ok(())
    }
    #[test]
    fn deserialize_string() -> Result<(), crate::Error> {
        let json = "\"Hello, World!\"";
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        assert_eq!(value, Value::String("Hello, World!".to_string()));
        Ok(())
    }
    #[test]
    fn deserialize_array() -> Result<(), crate::Error> {
        let json = "[1, true, null, \"test\"]";
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        assert_eq!(
            value, Value::Array(vec![Value::Number(1.into()), Value::Bool(true),
            Value::Null, Value::String("test".to_string()),])
        );
        Ok(())
    }
    #[test]
    fn deserialize_object() -> Result<(), crate::Error> {
        let json = "{\"key1\": \"value1\", \"key2\": 2, \"key3\": true}";
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        let mut expected = crate::Map::new();
        expected.insert("key1".to_string(), Value::String("value1".to_string()));
        expected.insert("key2".to_string(), Value::Number(2.into()));
        expected.insert("key3".to_string(), Value::Bool(true));
        assert_eq!(value, Value::Object(expected));
        Ok(())
    }
    #[test]
    fn deserialize_complex_object() -> Result<(), crate::Error> {
        let json = r#"{
            "key1": "value1",
            "key2": 2,
            "key3": {
                "key3_1": true,
                "key3_2": [1, 2, 3]
            }
        }"#;
        let mut deserializer = Deserializer::from_str(json);
        let value = Value::deserialize(&mut deserializer)?;
        let mut key3_map = crate::Map::new();
        key3_map.insert("key3_1".to_string(), Value::Bool(true));
        key3_map
            .insert(
                "key3_2".to_string(),
                Value::Array(
                    vec![
                        Value::Number(1.into()), Value::Number(2.into()), Value::Number(3
                        .into()),
                    ],
                ),
            );
        let mut expected = crate::Map::new();
        expected.insert("key1".to_string(), Value::String("value1".to_string()));
        expected.insert("key2".to_string(), Value::Number(2.into()));
        expected.insert("key3".to_string(), Value::Object(key3_map));
        assert_eq!(value, Value::Object(expected));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_637_llm_16_637 {
    use serde::Deserialize;
    use crate::{json, Value, Error};
    #[test]
    fn test_deserialize_i128_number_within_i128_range() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_number_within_i128_range = 0;
        let value = json!(i128::MAX);
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert_eq!(result.unwrap(), i128::MAX);
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_number_within_i128_range = 0;
    }
    #[test]
    fn test_deserialize_i128_number_outside_i128_range() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_number_outside_i128_range = 0;
        let rug_fuzz_0 = 1;
        let big_value = i128::MAX as u128 + rug_fuzz_0;
        let value = json!(big_value.to_string());
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_number_outside_i128_range = 0;
    }
    #[test]
    fn test_deserialize_i128_string() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_string = 0;
        let rug_fuzz_0 = "not a number";
        let value = json!(rug_fuzz_0);
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_string = 0;
    }
    #[test]
    fn test_deserialize_i128_null() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_null = 0;
        let value = json!(null);
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_null = 0;
    }
    #[test]
    fn test_deserialize_i128_bool() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_bool = 0;
        let rug_fuzz_0 = true;
        let value = json!(rug_fuzz_0);
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_bool = 0;
    }
    #[test]
    fn test_deserialize_i128_array() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_array = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let value = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_array = 0;
    }
    #[test]
    fn test_deserialize_i128_object() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_object = 0;
        let value = json!({ "key" : "value" });
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_object = 0;
    }
    #[test]
    fn test_deserialize_i128_float() {
        let _rug_st_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_float = 0;
        let rug_fuzz_0 = 12.34;
        let value = json!(rug_fuzz_0);
        let result: Result<i128, Error> = crate::from_value(value);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_637_llm_16_637_rrrruuuugggg_test_deserialize_i128_float = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_638 {
    use crate::{Number, Value, Error};
    #[test]
    fn deserialize_i16_valid_number() {
        let n = Number::from(-32768i16);
        let v = Value::Number(n);
        let i16_val: Result<i16, Error> = crate::from_value(v);
        assert_eq!(i16_val.unwrap(), - 32768);
    }
    #[test]
    fn deserialize_i16_invalid_number() {
        let v = Value::Number(Number::from_f64(1.5).unwrap());
        let i16_val: Result<i16, Error> = crate::from_value(v);
        assert!(i16_val.is_err());
    }
    #[test]
    fn deserialize_i16_out_of_range_positive() {
        let v = Value::Number(Number::from(32768i32));
        let i16_val: Result<i16, Error> = crate::from_value(v);
        assert!(i16_val.is_err());
    }
    #[test]
    fn deserialize_i16_out_of_range_negative() {
        let v = Value::Number(Number::from(-32769i32));
        let i16_val: Result<i16, Error> = crate::from_value(v);
        assert!(i16_val.is_err());
    }
    #[test]
    fn deserialize_i16_non_number() {
        let v = Value::String("not a number".to_owned());
        let i16_val: Result<i16, Error> = crate::from_value(v);
        assert!(i16_val.is_err());
    }
    #[test]
    fn deserialize_i16_null_value() {
        let v = Value::Null;
        let i16_val: Result<i16, Error> = crate::from_value(v);
        assert!(i16_val.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_640_llm_16_640 {
    use crate::value::{Value, Number};
    use crate::error::Error;
    use serde::Deserializer;
    use std::str::FromStr;
    use crate::from_value;
    #[test]
    fn deserialize_i64_from_number() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_number = 0;
        let rug_fuzz_0 = "-42";
        let n = Number::from_str(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        let i: Result<i64, Error> = from_value(value);
        debug_assert_eq!(i.unwrap(), - 42);
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_number = 0;
    }
    #[test]
    fn deserialize_i64_from_string() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_string = 0;
        let rug_fuzz_0 = "-42";
        let value = Value::String(rug_fuzz_0.to_owned());
        let i: Result<i64, Error> = from_value(value);
        debug_assert!(i.is_err());
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_string = 0;
    }
    #[test]
    fn deserialize_i64_from_bool() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_bool = 0;
        let rug_fuzz_0 = true;
        let value = Value::Bool(rug_fuzz_0);
        let i: Result<i64, Error> = from_value(value);
        debug_assert!(i.is_err());
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_bool = 0;
    }
    #[test]
    fn deserialize_i64_from_null() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_null = 0;
        let value = Value::Null;
        let i: Result<i64, Error> = from_value(value);
        debug_assert!(i.is_err());
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_null = 0;
    }
    #[test]
    fn deserialize_i64_from_array() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_array = 0;
        let rug_fuzz_0 = 42;
        let value = Value::Array(vec![Value::from(rug_fuzz_0)]);
        let i: Result<i64, Error> = from_value(value);
        debug_assert!(i.is_err());
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_array = 0;
    }
    #[test]
    fn deserialize_i64_from_object() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_object = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        let value = Value::Object(map);
        let i: Result<i64, Error> = from_value(value);
        debug_assert!(i.is_err());
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_object = 0;
    }
    #[test]
    fn deserialize_i64_from_u64() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_u64 = 0;
        let rug_fuzz_0 = 42;
        let n = Number::from(rug_fuzz_0);
        let value = Value::Number(n);
        let i: Result<i64, Error> = from_value(value);
        debug_assert_eq!(i.unwrap(), 42);
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_u64 = 0;
    }
    #[test]
    fn deserialize_i64_from_out_of_bounds_u64() {
        let _rug_st_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_out_of_bounds_u64 = 0;
        let n = Number::from(u64::max_value());
        let value = Value::Number(n);
        let i: Result<i64, Error> = from_value(value);
        debug_assert!(i.is_err());
        let _rug_ed_tests_llm_16_640_llm_16_640_rrrruuuugggg_deserialize_i64_from_out_of_bounds_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_646 {
    use serde::{de::DeserializeOwned, Deserialize, Deserializer as _};
    use crate::value::{self, Value};
    fn from_value<T>(value: Value) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
    {
        crate::from_value(value)
    }
    #[test]
    fn test_deserialize_none_from_null() {
        let null_value = Value::Null;
        let option: Option<Value> = from_value(null_value).unwrap();
        assert!(option.is_none());
    }
    #[test]
    fn test_deserialize_some_bool_from_value() {
        let bool_value = Value::Bool(true);
        let option: Option<Value> = from_value(bool_value).unwrap();
        assert_eq!(option, Some(Value::Bool(true)));
    }
    #[test]
    fn test_deserialize_some_number_from_value() {
        let num_value = Value::Number(42.into());
        let option: Option<Value> = from_value(num_value).unwrap();
        assert_eq!(option, Some(Value::Number(42.into())));
    }
    #[test]
    fn test_deserialize_some_string_from_value() {
        let string_value = Value::String("hello".to_owned());
        let option: Option<Value> = from_value(string_value).unwrap();
        assert_eq!(option, Some(Value::String("hello".to_owned())));
    }
    #[test]
    fn test_deserialize_some_array_from_value() {
        let array_value = Value::Array(vec!["hello".into(), "world".into()]);
        let option: Option<Value> = from_value(array_value).unwrap();
        assert_eq!(option, Some(Value::Array(vec!["hello".into(), "world".into()])));
    }
    #[test]
    fn test_deserialize_some_object_from_value() {
        let mut map = crate::Map::new();
        map.insert("hello".to_owned(), Value::String("world".to_owned()));
        let object_value = Value::Object(map);
        let option: Option<Value> = from_value(object_value).unwrap();
        let mut expected_map = crate::Map::new();
        expected_map.insert("hello".to_owned(), Value::String("world".to_owned()));
        assert_eq!(option, Some(Value::Object(expected_map)));
    }
}
#[cfg(test)]
mod tests_llm_16_649 {
    use serde::de::{Deserialize, Deserializer};
    use crate::{Error, Value};
    use serde::de::IntoDeserializer;
    #[test]
    fn test_deserialize_string() {
        let _rug_st_tests_llm_16_649_rrrruuuugggg_test_deserialize_string = 0;
        let rug_fuzz_0 = "a string";
        let data = Value::String(rug_fuzz_0.to_owned());
        let deserializer = data.into_deserializer();
        let result: Result<String, Error> = Deserialize::deserialize(deserializer);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), "a string".to_owned());
        let _rug_ed_tests_llm_16_649_rrrruuuugggg_test_deserialize_string = 0;
    }
    #[test]
    fn test_deserialize_string_fail() {
        let _rug_st_tests_llm_16_649_rrrruuuugggg_test_deserialize_string_fail = 0;
        let rug_fuzz_0 = true;
        let data = Value::Bool(rug_fuzz_0);
        let deserializer = data.into_deserializer();
        let result: Result<String, Error> = Deserialize::deserialize(deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_649_rrrruuuugggg_test_deserialize_string_fail = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_656_llm_16_656 {
    use crate::{Value, Error, Number};
    #[test]
    fn test_deserialize_u64_with_valid_u64() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_valid_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let n = rug_fuzz_0;
        let value = Value::Number(Number::from(n));
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_ok());
        debug_assert_eq!(deserialized.unwrap(), n);
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_valid_u64 = 0;
    }
    #[test]
    fn test_deserialize_u64_with_negative_i64() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_negative_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let n = -rug_fuzz_0;
        let value = Value::Number(Number::from(n));
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_negative_i64 = 0;
    }
    #[test]
    fn test_deserialize_u64_with_valid_i64() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_valid_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let n = rug_fuzz_0;
        let value = Value::Number(Number::from(n));
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_ok());
        debug_assert_eq!(deserialized.unwrap() as i64, n);
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_valid_i64 = 0;
    }
    #[test]
    fn test_deserialize_u64_with_f64() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_f64 = 0;
        let rug_fuzz_0 = 42.0_f64;
        let n = rug_fuzz_0;
        let value = Value::Number(Number::from_f64(n).unwrap());
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_f64 = 0;
    }
    #[test]
    fn test_deserialize_u64_with_string() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_string = 0;
        let rug_fuzz_0 = "42";
        let value = Value::String(rug_fuzz_0.to_string());
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_string = 0;
    }
    #[test]
    fn test_deserialize_u64_with_null() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_null = 0;
        let value = Value::Null;
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_null = 0;
    }
    #[test]
    fn test_deserialize_u64_with_array() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_array = 0;
        let rug_fuzz_0 = 42u64;
        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_array = 0;
    }
    #[test]
    fn test_deserialize_u64_with_object() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_object = 0;
        let rug_fuzz_0 = "value";
        let rug_fuzz_1 = 42u64;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(Number::from(rug_fuzz_1)));
        let value = Value::Object(map);
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_object = 0;
    }
    #[test]
    fn test_deserialize_u64_with_bool() {
        let _rug_st_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_bool = 0;
        let rug_fuzz_0 = true;
        let value = Value::Bool(rug_fuzz_0);
        let deserialized: Result<u64, Error> = crate::from_value(value);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_656_llm_16_656_rrrruuuugggg_test_deserialize_u64_with_bool = 0;
    }
}
#[cfg(test)]
#[cfg(feature = "serde_json")]
mod tests_llm_16_657 {
    use crate::value::Value;
    use serde::Deserialize;
    use crate::de::Deserializer;
    #[test]
    fn test_deserialize_u8_from_valid_number() {
        let _rug_st_tests_llm_16_657_rrrruuuugggg_test_deserialize_u8_from_valid_number = 0;
        let rug_fuzz_0 = 255;
        let json_number = Value::Number(rug_fuzz_0.into());
        let mut deserializer = Deserializer::new(json_number);
        let result: Result<u8, _> = Deserialize::deserialize(&mut deserializer);
        debug_assert_eq!(result.unwrap(), 255u8);
        let _rug_ed_tests_llm_16_657_rrrruuuugggg_test_deserialize_u8_from_valid_number = 0;
    }
    #[test]
    fn test_deserialize_u8_from_invalid_number() {
        let _rug_st_tests_llm_16_657_rrrruuuugggg_test_deserialize_u8_from_invalid_number = 0;
        let rug_fuzz_0 = 256;
        let json_number = Value::Number(rug_fuzz_0.into());
        let mut deserializer = Deserializer::new(json_number);
        let result: Result<u8, _> = Deserialize::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_657_rrrruuuugggg_test_deserialize_u8_from_invalid_number = 0;
    }
    #[test]
    fn test_deserialize_u8_from_invalid_type() {
        let _rug_st_tests_llm_16_657_rrrruuuugggg_test_deserialize_u8_from_invalid_type = 0;
        let rug_fuzz_0 = "not a number";
        let json_string = Value::String(rug_fuzz_0.into());
        let mut deserializer = Deserializer::new(json_string);
        let result: Result<u8, _> = Deserialize::deserialize(&mut deserializer);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_657_rrrruuuugggg_test_deserialize_u8_from_invalid_type = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_669 {
    use crate::{Value, Number};
    use serde::de::Error as DeError;
    #[test]
    fn deserialize_i16_from_i16_number() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_i16_number = 0;
        let rug_fuzz_0 = 123i16;
        let n = Number::from(rug_fuzz_0);
        let v = Value::Number(n);
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert_eq!(deserialized.unwrap(), 123i16);
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_i16_number = 0;
    }
    #[test]
    fn deserialize_i16_from_i16_str() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_i16_str = 0;
        let rug_fuzz_0 = "123";
        let v = Value::String(rug_fuzz_0.to_owned());
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_i16_str = 0;
    }
    #[test]
    fn deserialize_i16_from_out_of_range_number() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_out_of_range_number = 0;
        let rug_fuzz_0 = 35000i32;
        let n = Number::from(rug_fuzz_0);
        let v = Value::Number(n);
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_out_of_range_number = 0;
    }
    #[test]
    fn deserialize_i16_from_f64_number() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_f64_number = 0;
        let rug_fuzz_0 = 123.0;
        let n = Number::from_f64(rug_fuzz_0).unwrap();
        let v = Value::Number(n);
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert_eq!(deserialized.unwrap(), 123i16);
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_f64_number = 0;
    }
    #[test]
    fn deserialize_i16_from_f64_str() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_f64_str = 0;
        let rug_fuzz_0 = "123.0";
        let v = Value::String(rug_fuzz_0.to_owned());
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_f64_str = 0;
    }
    #[test]
    fn deserialize_i16_from_bool() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_bool = 0;
        let rug_fuzz_0 = true;
        let v = Value::Bool(rug_fuzz_0);
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_bool = 0;
    }
    #[test]
    fn deserialize_i16_from_null() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_null = 0;
        let v = Value::Null;
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_null = 0;
    }
    #[test]
    fn deserialize_i16_from_object() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_object = 0;
        let v = Value::Object(crate::Map::new());
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_object = 0;
    }
    #[test]
    fn deserialize_i16_from_array() {
        let _rug_st_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_array = 0;
        let rug_fuzz_0 = 123i16;
        let v = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        let deserialized: Result<i16, _> = crate::from_value(v);
        debug_assert!(deserialized.is_err());
        let _rug_ed_tests_llm_16_669_rrrruuuugggg_deserialize_i16_from_array = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_671 {
    use crate::{Value, Number, Error};
    #[test]
    fn deserialize_i64_from_valid_number() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_valid_number = 0;
        let rug_fuzz_0 = 42_i64;
        let val = Value::Number(Number::from(rug_fuzz_0));
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert_eq!(n.unwrap(), 42_i64);
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_valid_number = 0;
    }
    #[test]
    fn deserialize_i64_from_negative_number() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_negative_number = 0;
        let rug_fuzz_0 = 42_i64;
        let val = Value::Number(Number::from(-rug_fuzz_0));
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert_eq!(n.unwrap(), - 42_i64);
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_negative_number = 0;
    }
    #[test]
    fn deserialize_i64_from_float() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_float = 0;
        let rug_fuzz_0 = 42.0;
        let val = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_float = 0;
    }
    #[test]
    fn deserialize_i64_from_out_of_range_number() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_out_of_range_number = 0;
        let rug_fuzz_0 = 1;
        let val = Value::Number(Number::from((i64::MAX as u64) + rug_fuzz_0));
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_out_of_range_number = 0;
    }
    #[test]
    fn deserialize_i64_from_string() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_string = 0;
        let rug_fuzz_0 = "42";
        let val = Value::String(rug_fuzz_0.to_owned());
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_string = 0;
    }
    #[test]
    fn deserialize_i64_from_bool() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_bool = 0;
        let rug_fuzz_0 = true;
        let val = Value::Bool(rug_fuzz_0);
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_bool = 0;
    }
    #[test]
    fn deserialize_i64_from_null() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_null = 0;
        let val = Value::Null;
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_null = 0;
    }
    #[test]
    fn deserialize_i64_from_array() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_array = 0;
        let rug_fuzz_0 = 42_i64;
        let val = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_array = 0;
    }
    #[test]
    fn deserialize_i64_from_object() {
        let _rug_st_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_object = 0;
        let val = Value::Object(crate::Map::new());
        let n: Result<i64, Error> = serde::Deserialize::deserialize(val);
        debug_assert!(n.is_err());
        let _rug_ed_tests_llm_16_671_rrrruuuugggg_deserialize_i64_from_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_672 {
    use crate::{Number, Value, Error};
    #[test]
    fn test_deserialize_i8_with_number_within_bounds() {
        let _rug_st_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_number_within_bounds = 0;
        let rug_fuzz_0 = 10_i64;
        let json_number = Value::Number(Number::from(rug_fuzz_0));
        let res: Result<i8, Error> = serde::Deserialize::deserialize(json_number);
        debug_assert_eq!(res.unwrap(), 10i8);
        let _rug_ed_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_number_within_bounds = 0;
    }
    #[test]
    fn test_deserialize_i8_with_number_below_bounds() {
        let _rug_st_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_number_below_bounds = 0;
        let rug_fuzz_0 = 130_i64;
        let json_number = Value::Number(Number::from(-rug_fuzz_0));
        let res: Result<i8, Error> = serde::Deserialize::deserialize(json_number);
        debug_assert!(res.is_err());
        let _rug_ed_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_number_below_bounds = 0;
    }
    #[test]
    fn test_deserialize_i8_with_number_above_bounds() {
        let _rug_st_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_number_above_bounds = 0;
        let rug_fuzz_0 = 130_i64;
        let json_number = Value::Number(Number::from(rug_fuzz_0));
        let res: Result<i8, Error> = serde::Deserialize::deserialize(json_number);
        debug_assert!(res.is_err());
        let _rug_ed_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_number_above_bounds = 0;
    }
    #[test]
    fn test_deserialize_i8_with_non_number() {
        let _rug_st_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_non_number = 0;
        let rug_fuzz_0 = "not a number";
        let json_str = Value::String(rug_fuzz_0.to_owned());
        let res: Result<i8, Error> = serde::Deserialize::deserialize(json_str);
        debug_assert!(res.is_err());
        let _rug_ed_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_non_number = 0;
    }
    #[test]
    fn test_deserialize_i8_with_null() {
        let _rug_st_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_null = 0;
        let json_null = Value::Null;
        let res: Result<i8, Error> = serde::Deserialize::deserialize(json_null);
        debug_assert!(res.is_err());
        let _rug_ed_tests_llm_16_672_rrrruuuugggg_test_deserialize_i8_with_null = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_679 {
    use serde::de::{Deserialize, Deserializer, Visitor};
    use crate::value::{Error, Value};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::String(v.to_owned()))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::String(v))
        }
    }
    #[test]
    fn test_deserialize_str() -> Result<(), Error> {
        let val = Value::String("Hello world!".to_string());
        let deserialized = val.deserialize_str(TestVisitor {})?;
        assert_eq!(deserialized, Value::String("Hello world!".to_string()));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_691 {
    use serde::de::IntoDeserializer;
    use crate::value::Value;
    #[test]
    fn into_deserializer_null() {
        let _rug_st_tests_llm_16_691_rrrruuuugggg_into_deserializer_null = 0;
        let v = Value::Null;
        let de = v.into_deserializer();
        debug_assert_eq!(de, Value::Null);
        let _rug_ed_tests_llm_16_691_rrrruuuugggg_into_deserializer_null = 0;
    }
    #[test]
    fn into_deserializer_bool() {
        let _rug_st_tests_llm_16_691_rrrruuuugggg_into_deserializer_bool = 0;
        let rug_fuzz_0 = true;
        let v = Value::Bool(rug_fuzz_0);
        let de = v.into_deserializer();
        debug_assert_eq!(de, Value::Bool(true));
        let _rug_ed_tests_llm_16_691_rrrruuuugggg_into_deserializer_bool = 0;
    }
    #[test]
    fn into_deserializer_number() {
        let _rug_st_tests_llm_16_691_rrrruuuugggg_into_deserializer_number = 0;
        let rug_fuzz_0 = 42;
        let v = Value::Number(crate::Number::from(rug_fuzz_0));
        let de = v.into_deserializer();
        debug_assert_eq!(de, Value::Number(crate ::Number::from(42)));
        let _rug_ed_tests_llm_16_691_rrrruuuugggg_into_deserializer_number = 0;
    }
    #[test]
    fn into_deserializer_string() {
        let _rug_st_tests_llm_16_691_rrrruuuugggg_into_deserializer_string = 0;
        let rug_fuzz_0 = "hello";
        let v = Value::String(rug_fuzz_0.to_owned());
        let de = v.into_deserializer();
        debug_assert_eq!(de, Value::String("hello".to_owned()));
        let _rug_ed_tests_llm_16_691_rrrruuuugggg_into_deserializer_string = 0;
    }
    #[test]
    fn into_deserializer_array() {
        let _rug_st_tests_llm_16_691_rrrruuuugggg_into_deserializer_array = 0;
        let v = Value::Array(vec![Value::Null, Value::Bool(true)]);
        let de = v.into_deserializer();
        debug_assert_eq!(de, Value::Array(vec![Value::Null, Value::Bool(true)]));
        let _rug_ed_tests_llm_16_691_rrrruuuugggg_into_deserializer_array = 0;
    }
    #[test]
    fn into_deserializer_object() {
        let _rug_st_tests_llm_16_691_rrrruuuugggg_into_deserializer_object = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let mut object = crate::Map::new();
        object
            .insert(
                rug_fuzz_0.to_owned(),
                Value::Number(crate::Number::from(rug_fuzz_1)),
            );
        let v = Value::Object(object.clone());
        let de = v.into_deserializer();
        debug_assert_eq!(de, Value::Object(object));
        let _rug_ed_tests_llm_16_691_rrrruuuugggg_into_deserializer_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_693 {
    use serde::de::Error as DeError;
    use crate::value::Value;
    use crate::Error;
    #[test]
    fn test_invalid_type_error() {
        let _rug_st_tests_llm_16_693_rrrruuuugggg_test_invalid_type_error = 0;
        let rug_fuzz_0 = "Not a number";
        let rug_fuzz_1 = "number";
        let value = Value::String(rug_fuzz_0.to_owned());
        let expected = &rug_fuzz_1;
        let error: Error = value.invalid_type(expected);
        let expected_error = Error::custom(
            format!("invalid type: string \"Not a number\", expected number"),
        );
        debug_assert_eq!(error.is_data(), expected_error.is_data());
        debug_assert_eq!(error.is_eof(), expected_error.is_eof());
        debug_assert_eq!(error.is_io(), expected_error.is_io());
        debug_assert_eq!(error.is_syntax(), expected_error.is_syntax());
        debug_assert_eq!(error.line(), expected_error.line());
        debug_assert_eq!(error.column(), expected_error.column());
        debug_assert_eq!(format!("{:?}", error), format!("{:?}", expected_error));
        let _rug_ed_tests_llm_16_693_rrrruuuugggg_test_invalid_type_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_694_llm_16_694 {
    use crate::value::Value;
    use serde::de::Unexpected;
    use crate::Number;
    #[test]
    fn test_unexpected_null() {
        let _rug_st_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_null = 0;
        let value = Value::Null;
        debug_assert_eq!(value.unexpected(), Unexpected::Unit);
        let _rug_ed_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_null = 0;
    }
    #[test]
    fn test_unexpected_bool() {
        let _rug_st_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let value = Value::Bool(rug_fuzz_0);
        debug_assert_eq!(value.unexpected(), Unexpected::Bool(true));
        let value = Value::Bool(rug_fuzz_1);
        debug_assert_eq!(value.unexpected(), Unexpected::Bool(false));
        let _rug_ed_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_bool = 0;
    }
    #[test]
    fn test_unexpected_number() {
        let _rug_st_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42_i64;
        let rug_fuzz_2 = 3.14;
        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(matches!(value.unexpected(), Unexpected::Unsigned(42)));
        let value = Value::Number(Number::from(-rug_fuzz_1));
        debug_assert!(matches!(value.unexpected(), Unexpected::Signed(- 42)));
        let value = Value::Number(Number::from_f64(rug_fuzz_2).unwrap());
        debug_assert!(
            matches!(value.unexpected(), Unexpected::Float(v) if (v - 3.14).abs() <
            std::f64::EPSILON)
        );
        let _rug_ed_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_number = 0;
    }
    #[test]
    fn test_unexpected_string() {
        let _rug_st_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_string = 0;
        let rug_fuzz_0 = "test";
        let value = Value::String(rug_fuzz_0.into());
        debug_assert_eq!(value.unexpected(), Unexpected::Str("test"));
        let _rug_ed_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_string = 0;
    }
    #[test]
    fn test_unexpected_array() {
        let _rug_st_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_array = 0;
        let value = Value::Array(vec![Value::Null]);
        debug_assert_eq!(value.unexpected(), Unexpected::Seq);
        let _rug_ed_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_array = 0;
    }
    #[test]
    fn test_unexpected_object() {
        let _rug_st_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_object = 0;
        let value = Value::Object(crate::map::Map::new());
        debug_assert_eq!(value.unexpected(), Unexpected::Map);
        let _rug_ed_tests_llm_16_694_llm_16_694_rrrruuuugggg_test_unexpected_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_696 {
    use super::*;
    use crate::*;
    use crate::value::{Value, Map};
    use crate::error::Error;
    #[test]
    fn test_map_deserializer_new() {
        let _rug_st_tests_llm_16_696_rrrruuuugggg_test_map_deserializer_new = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Number(rug_fuzz_1.into()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let deserializer = MapDeserializer::new(map.clone());
        let expected: Vec<(String, Value)> = map.into_iter().collect();
        let result: Vec<(String, Value)> = deserializer.iter.collect();
        debug_assert_eq!(expected, result);
        debug_assert!(deserializer.value.is_none());
        let _rug_ed_tests_llm_16_696_rrrruuuugggg_test_map_deserializer_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_151 {
    use super::*;
    use crate::value::Value;
    use crate::map::Map;
    use serde::{de, Deserialize};
    use std::fmt;
    #[test]
    fn test_visit_array() {
        let mut p0: Vec<Value> = Vec::<Value>::new();
        p0.push(Value::String("Sample data".to_owned()));
        struct TestVisitor;
        impl<'de> de::Visitor<'de> for TestVisitor {
            type Value = Map<String, Value>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }
            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Map::new())
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut values = Map::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }
                Ok(values)
            }
        }
        let test_visitor = TestVisitor;
        crate::value::de::visit_array(p0, test_visitor).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_152 {
    use crate::{map::Map, value::Value, Error};
    use serde::de::{self, Visitor};
    use std::fmt;
    #[test]
    fn test_visit_object() {
        let _rug_st_tests_rug_152_rrrruuuugggg_test_visit_object = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut p0: Map<String, Value> = Map::new();
        p0.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        p0.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut p1 = VisitorImpl;
        let result = crate::value::de::visit_object(p0, p1);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_152_rrrruuuugggg_test_visit_object = 0;
    }
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_153 {
    use super::*;
    use serde::de::{self, Visitor};
    use crate::value::{Value, Map};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_visit_array_ref() {
        let _rug_st_tests_rug_153_rrrruuuugggg_test_visit_array_ref = 0;
        let rug_fuzz_0 = true;
        let v40 = vec![
            Value::Bool(rug_fuzz_0), Value::Number(40.into()), Value::String("Sample"
            .to_owned())
        ];
        let mut p0: &[Value] = v40.as_ref();
        let p1_visitor = TestVisitor;
        let result = crate::value::de::visit_array_ref(p0, p1_visitor);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_153_rrrruuuugggg_test_visit_array_ref = 0;
    }
}
#[cfg(test)]
mod tests_rug_154 {
    use serde::de::{self, Visitor};
    use crate::{Error, map::Map, value::Value};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_visit_object_ref() {
        let _rug_st_tests_rug_154_rrrruuuugggg_test_visit_object_ref = 0;
        let p0 = &Map::<String, Value>::new();
        let p1 = TestVisitor;
        crate::value::de::visit_object_ref(p0, p1).unwrap();
        let _rug_ed_tests_rug_154_rrrruuuugggg_test_visit_object_ref = 0;
    }
}
#[cfg(test)]
mod tests_rug_165 {
    use super::*;
    use std::str::FromStr;
    use crate::value::Value;
    use crate::Error;
    #[test]
    fn test_from_str() {
        let _rug_st_tests_rug_165_rrrruuuugggg_test_from_str = 0;
        let rug_fuzz_0 = "{\"key\":\"value\"}";
        let rug_fuzz_1 = "key";
        let p0: &str = rug_fuzz_0;
        let result: Result<Value, Error> = Value::from_str(p0);
        debug_assert!(result.is_ok());
        let value = result.unwrap();
        debug_assert_eq!(value[rug_fuzz_1], "value");
        let _rug_ed_tests_rug_165_rrrruuuugggg_test_from_str = 0;
    }
}
#[cfg(test)]
mod tests_rug_166 {
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value};
    use std::{fmt, string::String};
    #[test]
    fn test_deserialize_any() {
        let mut p0 = Value::Bool(true);
        let mut p1 = Visitor;
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Map<String, Value>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }
            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Map::new())
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut values = Map::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }
                Ok(values)
            }
        }
        <Value as de::Deserializer<'_>>::deserialize_any(p0, p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_167 {
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value, Error};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_i32() {
        let _rug_st_tests_rug_167_rrrruuuugggg_test_deserialize_i32 = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1 = TestVisitor;
        let result: Result<Map<String, Value>, Error> = <Value as de::Deserializer<
            '_,
        >>::deserialize_i32(p0, p1);
        debug_assert!(result.is_err());
        let _rug_ed_tests_rug_167_rrrruuuugggg_test_deserialize_i32 = 0;
    }
}
#[cfg(test)]
mod tests_rug_172 {
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value};
    use std::fmt;
    use std::string::String;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_172_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1 = VisitorImpl;
        <Value as serde::Deserializer<'_>>::deserialize_u64(p0, p1).unwrap();
        let _rug_ed_tests_rug_172_rrrruuuugggg_test_rug = 0;
    }
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_173 {
    use serde::Deserializer;
    use crate::Value;
    use crate::map::Map;
    use serde::de::{self, Visitor};
    use std::fmt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_173_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1 = TestingVisitor;
        p0.deserialize_u128(p1).unwrap_err();
        let _rug_ed_tests_rug_173_rrrruuuugggg_test_rug = 0;
    }
    struct TestingVisitor;
    impl<'de> Visitor<'de> for TestingVisitor {
        type Value = u128;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned 128-bit integer")
        }
        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
}
#[cfg(test)]
mod tests_rug_174 {
    use crate::value::{self, Value};
    use serde::de::{self, Visitor};
    use crate::map::Map;
    use std::fmt;
    use std::string::String;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_174_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1 = MyVisitor;
        <value::Value as serde::Deserializer>::deserialize_f32(p0, p1).unwrap();
        let _rug_ed_tests_rug_174_rrrruuuugggg_test_rug = 0;
    }
    struct MyVisitor;
    impl<'de> Visitor<'de> for MyVisitor {
        type Value = f32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a single f32 value")
        }
        #[inline]
        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
}
#[cfg(test)]
mod tests_rug_176 {
    use crate::value::Value;
    use crate::map::Map;
    use serde::{Deserialize, Deserializer, de::{self, Visitor}};
    use std::string::String;
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_option() {
        let _rug_st_tests_rug_176_rrrruuuugggg_test_deserialize_option = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1 = TestVisitor;
        p0.deserialize_option(p1).unwrap();
        let _rug_ed_tests_rug_176_rrrruuuugggg_test_deserialize_option = 0;
    }
}
#[cfg(test)]
mod tests_rug_177 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::map::Map;
    use crate::value::{Value, self};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_enum() {
        let mut p0 = Value::Bool(true);
        let p1: &str = "ExampleEnum";
        let p2: &'static [&'static str] = &["Variant1", "Variant2", "Variant3"];
        let p3 = TestVisitor;
        let _ = p0.deserialize_enum(p1, p2, p3);
    }
}
#[cfg(test)]
mod tests_rug_178 {
    use crate::value::Value;
    use crate::map::Map;
    use serde::{Deserialize, Deserializer};
    use std::fmt;
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_178_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "some_newtype_struct";
        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1 = rug_fuzz_1;
        let mut p2 = Visitor;
        let _result = p0.deserialize_newtype_struct(p1, p2).unwrap();
        let _rug_ed_tests_rug_178_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_179 {
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value, Error};
    struct BoolVisitor;
    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
    #[test]
    fn test_deserialize_bool() {
        let _rug_st_tests_rug_179_rrrruuuugggg_test_deserialize_bool = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1 = BoolVisitor;
        let result = <Value as de::Deserializer<'_>>::deserialize_bool(p0, p1);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), true);
        let _rug_ed_tests_rug_179_rrrruuuugggg_test_deserialize_bool = 0;
    }
}
#[cfg(test)]
mod tests_rug_180 {
    use crate::{value::Value, Error};
    use serde::de::{self, Deserialize, Deserializer};
    use crate::map::Map;
    use std::fmt;
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_char() {
        let _rug_st_tests_rug_180_rrrruuuugggg_test_deserialize_char = 0;
        let rug_fuzz_0 = true;
        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let p1: Visitor = Visitor;
        let _result = Value::deserialize_char(p0, p1);
        let _rug_ed_tests_rug_180_rrrruuuugggg_test_deserialize_char = 0;
    }
}
#[cfg(test)]
mod tests_rug_181 {
    use crate::{map::Map, value::Value, Error};
    use serde::de::{self, Deserializer};
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_string() {
        let _rug_st_tests_rug_181_rrrruuuugggg_test_deserialize_string = 0;
        let rug_fuzz_0 = "test string";
        let mut p0: Value = Value::String(String::from(rug_fuzz_0));
        let p1: Visitor = Visitor;
        let result: Result<Map<String, Value>, Error> = p0.deserialize_string(p1);
        debug_assert!(result.is_ok());
        let map = result.unwrap();
        debug_assert!(map.is_empty());
        let _rug_ed_tests_rug_181_rrrruuuugggg_test_deserialize_string = 0;
    }
}
#[cfg(test)]
mod tests_rug_182 {
    use serde::de::Deserializer;
    use serde::de::Visitor;
    use crate::map::Map;
    use crate::value::Value;
    use std::fmt;
    use std::string::String;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_182_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1 = VisitorTest;
        p0.deserialize_bytes(p1).unwrap();
        let _rug_ed_tests_rug_182_rrrruuuugggg_test_rug = 0;
    }
    struct VisitorTest;
    impl<'de> Visitor<'de> for VisitorTest {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            let mut values = Map::<String, Value>::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_186 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::{map::Map, Value, Error};
    use std::fmt;
    #[test]
    fn test_deserialize_seq() {
        let _rug_st_tests_rug_186_rrrruuuugggg_test_deserialize_seq = 0;
        let rug_fuzz_0 = true;
        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let mut p1 = Visitor;
        let result = <Value as Deserializer<'_>>::deserialize_seq(p0, p1);
        debug_assert!(result.is_err());
        let _rug_ed_tests_rug_186_rrrruuuugggg_test_deserialize_seq = 0;
    }
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    use serde::Deserializer;
    use crate::Value;
    use crate::map::Map;
    use serde::de::{self, Visitor as _};
    use std::fmt;
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_187_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 2;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1: usize = rug_fuzz_1;
        let p2 = Visitor;
        p0.deserialize_tuple(p1, p2).unwrap();
        let _rug_ed_tests_rug_187_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_188 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::{map::Map, value::Value, Error};
    use std::fmt;
    use std::string::String;
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_tuple_struct() {
        let _rug_st_tests_rug_188_rrrruuuugggg_test_deserialize_tuple_struct = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "MyTupleStruct";
        let rug_fuzz_2 = 2_usize;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        let p2 = rug_fuzz_2;
        let p3 = Visitor;
        p0.deserialize_tuple_struct(p1, p2, p3).unwrap();
        let _rug_ed_tests_rug_188_rrrruuuugggg_test_deserialize_tuple_struct = 0;
    }
}
#[cfg(test)]
mod tests_rug_190 {
    use serde::de::{self, Visitor};
    use crate::map::Map;
    use crate::value::Value;
    use crate::Error;
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_struct() {
        let mut p0 = Value::Bool(true);
        let p1 = "TestStruct";
        let p2 = &["field1", "field2", "field3"];
        let p3 = TestVisitor;
        match <Value as de::Deserializer<'_>>::deserialize_struct(p0, p1, p2, p3) {
            Ok(_) => {}
            Err(e) => panic!("Failed during deserialization: {}", e),
        }
    }
}
#[cfg(test)]
mod tests_rug_191 {
    use serde::de::{self, Deserializer};
    use crate::{map::Map, value::Value, Error};
    use std::fmt;
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_191_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let mut p1 = Visitor;
        let _result = <Value as Deserializer>::deserialize_identifier(p0, p1);
        debug_assert!(_result.is_ok());
        debug_assert!(! _result.unwrap().is_empty());
        let _rug_ed_tests_rug_191_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_192 {
    use serde::de::{self, Visitor};
    use crate::value::Value;
    use crate::map::Map;
    use std::fmt;
    struct IgnoredAnyVisitor;
    impl<'de> Visitor<'de> for IgnoredAnyVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any value")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
    }
    #[test]
    fn test_deserialize_ignored_any() {
        let _rug_st_tests_rug_192_rrrruuuugggg_test_deserialize_ignored_any = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1 = IgnoredAnyVisitor;
        <Value as de::Deserializer<'_>>::deserialize_ignored_any(p0, p1).unwrap();
        let _rug_ed_tests_rug_192_rrrruuuugggg_test_deserialize_ignored_any = 0;
    }
}
#[cfg(test)]
mod tests_rug_193 {
    use serde::de::{DeserializeSeed, EnumAccess};
    use crate::value::{self, Value, de::KeyClassifier};
    use crate::error::Error;
    #[test]
    fn test_variant_seed() {
        let _rug_st_tests_rug_193_rrrruuuugggg_test_variant_seed = 0;
        let rug_fuzz_0 = "my_variant";
        let mut p0 = value::de::EnumDeserializer {
            variant: rug_fuzz_0.to_owned(),
            value: Some(Value::Null),
        };
        let p1 = KeyClassifier;
        let result: Result<
            (value::de::KeyClass, value::de::VariantDeserializer),
            Error,
        > = p0.variant_seed(p1);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_193_rrrruuuugggg_test_variant_seed = 0;
    }
}
#[cfg(test)]
mod tests_rug_194 {
    use super::*;
    use crate::value::de::VariantAccess;
    use crate::value::de::VariantDeserializer;
    use crate::Value;
    use crate::error::Error;
    #[test]
    fn test_unit_variant() {
        let _rug_st_tests_rug_194_rrrruuuugggg_test_unit_variant = 0;
        let mut p0 = VariantDeserializer {
            value: Some(Value::Null),
        };
        debug_assert!(matches!(p0.unit_variant(), Ok(())));
        let _rug_ed_tests_rug_194_rrrruuuugggg_test_unit_variant = 0;
    }
}
#[cfg(test)]
mod tests_rug_195 {
    use serde::de::DeserializeSeed;
    use serde::de::{self, VariantAccess};
    use crate::value::de::{KeyClassifier, VariantDeserializer};
    use crate::Value;
    use std::fmt;
    #[derive(Debug, PartialEq)]
    enum KeyClass {
        Number,
        RawValue,
        Map(String),
    }
    #[test]
    fn test_newtype_variant_seed() {
        let _rug_st_tests_rug_195_rrrruuuugggg_test_newtype_variant_seed = 0;
        let mut p0 = VariantDeserializer {
            value: Some(Value::Null),
        };
        let mut p1 = KeyClassifier;
        let result = p0.newtype_variant_seed(p1);
        debug_assert!(result.is_err());
        let _rug_ed_tests_rug_195_rrrruuuugggg_test_newtype_variant_seed = 0;
    }
}
#[cfg(test)]
mod tests_rug_197 {
    use crate::value::{self, Value};
    use crate::map::{self, Map};
    use serde::de::{self, VariantAccess, Visitor};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_struct_variant() {
        let fields: &'static [&'static str] = &["field1", "field2", "field3"];
        let value = Some(Value::Object(Map::new()));
        let mut deserializer = value::de::VariantDeserializer {
            value,
        };
        let visitor = TestVisitor;
        let result = deserializer.struct_variant(fields, visitor);
        assert!(result.is_ok());
    }
}
#[cfg(test)]
mod tests_rug_198 {
    use crate::value::{self, Value};
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_198_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = "Sample data";
        let mut p0: Vec<Value> = Vec::new();
        p0.push(Value::String(rug_fuzz_0.to_owned()));
        value::de::SeqDeserializer::new(p0);
        let _rug_ed_tests_rug_198_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_199 {
    use super::*;
    use crate::value::de::{KeyClassifier, SeqDeserializer};
    use crate::Value;
    use serde::de::{DeserializeSeed, SeqAccess};
    #[test]
    fn test_next_element_seed() {
        let _rug_st_tests_rug_199_rrrruuuugggg_test_next_element_seed = 0;
        let mut seq_deserializer = SeqDeserializer::new(
            vec![Value::Null, Value::Bool(true), Value::Number(42.into())],
        );
        let key_classifier = KeyClassifier {};
        seq_deserializer.next_element_seed(key_classifier).unwrap();
        let _rug_ed_tests_rug_199_rrrruuuugggg_test_next_element_seed = 0;
    }
}
#[cfg(test)]
mod tests_rug_203 {
    use super::*;
    use crate::map::Map;
    use crate::value;
    use crate::Value;
    use crate::value::de::MapDeserializer;
    #[test]
    fn test_size_hint() {
        let _rug_st_tests_rug_203_rrrruuuugggg_test_size_hint = 0;
        let map: Map<String, Value> = Map::new();
        let p0 = MapDeserializer::new(map);
        debug_assert_eq!(MapDeserializer::size_hint(& p0), Some(0));
        let _rug_ed_tests_rug_203_rrrruuuugggg_test_size_hint = 0;
    }
}
#[cfg(test)]
mod tests_rug_205 {
    use serde::de::{self, Visitor};
    use crate::map::Map;
    use crate::{value, Value};
    use std::fmt;
    #[test]
    fn test_deserialize_i8() {
        let mut p0 = Value::Bool(true);
        let p1 = VisitorImpl;
        struct VisitorImpl;
        impl<'de> Visitor<'de> for VisitorImpl {
            type Value = Map<String, value::Value>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Map::new())
            }
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut values = Map::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }
                Ok(values)
            }
        }
        let deserializer = &p0;
        let _ = <&value::Value as serde::Deserializer<
            '_,
        >>::deserialize_i8(deserializer, p1);
    }
}
#[cfg(test)]
mod tests_rug_206 {
    use serde::Deserializer;
    use crate::Value;
    use crate::map::Map;
    use std::string::String;
    use serde::de::{self, Visitor};
    #[test]
    fn test_deserialize_i32() {
        let _rug_st_tests_rug_206_rrrruuuugggg_test_deserialize_i32 = 0;
        let rug_fuzz_0 = true;
        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1 = VisitorImpl;
        p0.deserialize_i32(p1).unwrap_err();
        let _rug_ed_tests_rug_206_rrrruuuugggg_test_deserialize_i32 = 0;
    }
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<M>(self, mut visitor: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_208 {
    use crate::{Value, map::Map};
    use serde::{Deserialize, de::{self, Visitor}};
    use std::fmt::{self, Formatter};
    use crate::value;
    #[test]
    fn test_deserialize_u32() {
        let _rug_st_tests_rug_208_rrrruuuugggg_test_deserialize_u32 = 0;
        let rug_fuzz_0 = true;
        let mut p0: &Value = &Value::Bool(rug_fuzz_0);
        let p1: VisitorImpl = VisitorImpl;
        <&value::Value as serde::Deserializer<'_>>::deserialize_u32(p0, p1).unwrap();
        let _rug_ed_tests_rug_208_rrrruuuugggg_test_deserialize_u32 = 0;
    }
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_210 {
    use serde::Deserializer;
    use serde::de::{self, Visitor};
    use crate::value::Value;
    use crate::map::Map;
    use std::{fmt, string::String};
    #[test]
    fn test_deserialize_f32() {
        let _rug_st_tests_rug_210_rrrruuuugggg_test_deserialize_f32 = 0;
        let rug_fuzz_0 = true;
        let mut p0: &Value = &Value::Bool(rug_fuzz_0);
        let mut p1 = VisitorImpl;
        <&Value as Deserializer>::deserialize_f32(p0, p1).unwrap_err();
        let _rug_ed_tests_rug_210_rrrruuuugggg_test_deserialize_f32 = 0;
    }
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_211 {
    use serde::Deserializer;
    use crate::value::Value;
    use crate::map::Map;
    use serde::{de, Deserialize};
    use std::fmt;
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_f64() {
        let _rug_st_tests_rug_211_rrrruuuugggg_test_deserialize_f64 = 0;
        let rug_fuzz_0 = true;
        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let p1: Visitor = Visitor;
        p0.deserialize_f64(p1).unwrap_err();
        let _rug_ed_tests_rug_211_rrrruuuugggg_test_deserialize_f64 = 0;
    }
}
#[cfg(test)]
mod tests_rug_213 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::map::Map;
    use crate::value::{self, Value};
    use std::{fmt, marker::PhantomData};
    #[test]
    fn test_deserialize_newtype_struct() {
        let _rug_st_tests_rug_213_rrrruuuugggg_test_deserialize_newtype_struct = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "NewtypeStruct";
        let mut p0: &value::Value = &Value::Bool(rug_fuzz_0);
        let p1: &'static str = rug_fuzz_1;
        let p2: TestVisitor = TestVisitor(PhantomData);
        <&value::Value as Deserializer>::deserialize_newtype_struct(p0, p1, p2).unwrap();
        let _rug_ed_tests_rug_213_rrrruuuugggg_test_deserialize_newtype_struct = 0;
    }
    struct TestVisitor(PhantomData<Map<String, Value>>);
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value, Error};
    #[test]
    fn test_deserialize_bool() {
        let _rug_st_tests_rug_214_rrrruuuugggg_test_deserialize_bool = 0;
        let rug_fuzz_0 = true;
        let mut p0 = &Value::Bool(rug_fuzz_0);
        let p1 = BoolVisitor;
        <&Value as serde::Deserializer>::deserialize_bool(p0, p1).unwrap();
        let _rug_ed_tests_rug_214_rrrruuuugggg_test_deserialize_bool = 0;
    }
    struct BoolVisitor;
    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a bool")
        }
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
}
#[cfg(test)]
mod tests_rug_216 {
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value, Value as JsonValue};
    use std::fmt;
    use serde::Deserializer;
    #[test]
    fn test_deserialize_str() {
        let _rug_st_tests_rug_216_rrrruuuugggg_test_deserialize_str = 0;
        let rug_fuzz_0 = "a string value";
        let mut p0: &JsonValue = &JsonValue::String(String::from(rug_fuzz_0));
        let p1: VisitorImpl = VisitorImpl;
        <&JsonValue as serde::Deserializer<'_>>::deserialize_str(p0, p1).unwrap();
        let _rug_ed_tests_rug_216_rrrruuuugggg_test_deserialize_str = 0;
    }
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Map<String, JsonValue>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string value")
        }
        #[inline]
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut map = Map::new();
            map.insert(v.to_owned(), JsonValue::String(v.to_owned()));
            Ok(map)
        }
    }
}
#[cfg(test)]
mod tests_rug_217 {
    use crate::{map::Map, Value};
    use serde::{de, Deserializer};
    use crate::error::Error;
    use std::fmt;
    #[test]
    fn test_deserialize_bytes() {
        let _rug_st_tests_rug_217_rrrruuuugggg_test_deserialize_bytes = 0;
        let rug_fuzz_0 = true;
        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let mut p1: Visitor = Visitor;
        let result: Result<Map<String, Value>, Error> = p0.deserialize_bytes(p1);
        let _rug_ed_tests_rug_217_rrrruuuugggg_test_deserialize_bytes = 0;
    }
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_222 {
    use super::*;
    use crate::value::{self, Value};
    use serde::de::{self, Visitor};
    use crate::map::Map;
    use std::{fmt, string::String};
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_tuple() {
        let p0: &'static Value = &Value::Bool(true);
        let p1: usize = 0;
        let p2 = TestVisitor;
        <&'static value::Value as serde::Deserializer<
            'static,
        >>::deserialize_tuple(p0, p1, p2)
            .unwrap();
    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    use crate::value::Value;
    use crate::map::Map;
    use serde::{Deserializer, de};
    use std::{fmt, str};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_223_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "TupleStructName";
        let rug_fuzz_2 = 3;
        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let p1: &'static str = rug_fuzz_1;
        let p2: usize = rug_fuzz_2;
        let p3 = Visitor;
        p0.deserialize_tuple_struct(p1, p2, p3).unwrap();
        let _rug_ed_tests_rug_223_rrrruuuugggg_test_rug = 0;
    }
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_227 {
    use serde::de::{self, Visitor};
    use crate::{map::Map, value::Value, Error};
    use std::fmt;
    #[test]
    fn test_deserialize_ignored_any() {
        let _rug_st_tests_rug_227_rrrruuuugggg_test_deserialize_ignored_any = 0;
        let rug_fuzz_0 = true;
        let mut p0: &Value = &Value::Bool(rug_fuzz_0);
        let mut p1 = VisitorImpl;
        <&Value as de::Deserializer>::deserialize_ignored_any(p0, p1).unwrap();
        let _rug_ed_tests_rug_227_rrrruuuugggg_test_deserialize_ignored_any = 0;
    }
    struct VisitorImpl;
    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[inline]
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::<String, Value>::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
}
#[cfg(test)]
mod tests_rug_232 {
    use crate::value::{self, Value};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_232_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 40;
        let rug_fuzz_2 = "Sample";
        let mut p0: &[Value] = &[
            Value::Bool(rug_fuzz_0),
            Value::Number(rug_fuzz_1.into()),
            Value::String(rug_fuzz_2.to_owned()),
        ];
        value::de::SeqRefDeserializer::new(p0);
        let _rug_ed_tests_rug_232_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_233 {
    use serde::de::{DeserializeSeed, SeqAccess};
    use crate::value::de::{KeyClassifier, SeqRefDeserializer};
    use crate::Value;
    #[test]
    fn test_next_element_seed() {
        let _rug_st_tests_rug_233_rrrruuuugggg_test_next_element_seed = 0;
        let rug_fuzz_0 = 56;
        let sample_data = Value::Array(
            vec![Value::Number(rug_fuzz_0.into()), Value::Number(78.into())],
        );
        let mut p0 = SeqRefDeserializer::new(sample_data.as_array().unwrap());
        let p1 = KeyClassifier;
        debug_assert!(p0.next_element_seed(p1).is_ok());
        let _rug_ed_tests_rug_233_rrrruuuugggg_test_next_element_seed = 0;
    }
}
#[cfg(test)]
mod tests_rug_234 {
    use crate::Value;
    use crate::value::de::SeqRefDeserializer;
    use serde::de::SeqAccess;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_234_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 56;
        let sample_data = Value::Array(
            vec![Value::Number(rug_fuzz_0.into()), Value::Number(78.into())],
        );
        let mut p0 = SeqRefDeserializer::new(sample_data.as_array().unwrap());
        p0.size_hint();
        let _rug_ed_tests_rug_234_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_235 {
    use crate::map::Map;
    use crate::value::{self, Value};
    use std::string::String;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_235_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = 2;
        let mut p0: Map<String, Value> = Map::new();
        p0.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        p0.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        value::de::MapRefDeserializer::new(&p0);
        let _rug_ed_tests_rug_235_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_254 {
    use crate::value::de::BorrowedCowStrDeserializer;
    use std::borrow::Cow;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_254_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = "Sample data";
        let mut p0: Cow<'_, str> = Cow::Borrowed(rug_fuzz_0);
        BorrowedCowStrDeserializer::new(p0);
        let _rug_ed_tests_rug_254_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_255 {
    use serde::de::{DeserializeSeed, EnumAccess};
    use crate::value::de::{BorrowedCowStrDeserializer, KeyClassifier, KeyClass};
    #[test]
    fn test_variant_seed() {
        let _rug_st_tests_rug_255_rrrruuuugggg_test_variant_seed = 0;
        let rug_fuzz_0 = "sample string";
        let s = rug_fuzz_0;
        let mut p0 = BorrowedCowStrDeserializer::<'_>::new(s.into());
        let p1 = KeyClassifier;
        let result = <BorrowedCowStrDeserializer<'_>>::variant_seed(p0, p1);
        debug_assert!(result.is_ok());
        match result {
            Ok((value, _)) => {
                match value {
                    KeyClass::Map(ref map) => debug_assert_eq!(map, s),
                    _ => panic!("Unexpected variant returned"),
                }
            }
            Err(err) => panic!("Error occurred: {:?}", err),
        }
        let _rug_ed_tests_rug_255_rrrruuuugggg_test_variant_seed = 0;
    }
}
#[cfg(test)]
mod tests_rug_257 {
    use crate::value::de::UnitOnly;
    use crate::map::Map;
    use serde::{de, Deserialize};
    use crate::value::Value;
    use std::{fmt, marker::PhantomData};
    struct Visitor<'de> {
        marker: PhantomData<&'de ()>,
    }
    impl<'de> de::Visitor<'de> for Visitor<'de> {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn visit_map<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_rug() {
        let p0 = UnitOnly;
        let p1_strs: &[&'static str] = &["field1", "field2", "field3"];
        let p2 = Visitor { marker: PhantomData };
        <UnitOnly as de::VariantAccess>::struct_variant(p0, p1_strs, p2).unwrap_err();
    }
}
