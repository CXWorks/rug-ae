//! Definition of a TOML [value][Value]
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::Hash;
use std::mem::discriminant;
use std::ops;
use std::vec;
use serde::de;
use serde::de::IntoDeserializer;
use serde::ser;
use toml_datetime::__unstable as datetime;
pub use toml_datetime::{Date, Datetime, DatetimeParseError, Offset, Time};
/// Type representing a TOML array, payload of the `Value::Array` variant
pub type Array = Vec<Value>;
#[doc(no_inline)]
pub use crate::Table;
/// Representation of a TOML value.
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    /// Represents a TOML string
    String(String),
    /// Represents a TOML integer
    Integer(i64),
    /// Represents a TOML float
    Float(f64),
    /// Represents a TOML boolean
    Boolean(bool),
    /// Represents a TOML datetime
    Datetime(Datetime),
    /// Represents a TOML array
    Array(Array),
    /// Represents a TOML table
    Table(Table),
}
impl Value {
    /// Convert a `T` into `toml::Value` which is an enum that can represent
    /// any valid TOML data.
    ///
    /// This conversion can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    pub fn try_from<T>(value: T) -> Result<Value, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(ValueSerializer)
    }
    /// Interpret a `toml::Value` as an instance of type `T`.
    ///
    /// This conversion can fail if the structure of the `Value` does not match the
    /// structure expected by `T`, for example if `T` is a struct type but the
    /// `Value` contains something other than a TOML table. It can also fail if the
    /// structure is correct but `T`'s implementation of `Deserialize` decides that
    /// something is wrong with the data, for example required struct fields are
    /// missing from the TOML map or some number is too big to fit in the expected
    /// primitive type.
    pub fn try_into<'de, T>(self) -> Result<T, crate::de::Error>
    where
        T: de::Deserialize<'de>,
    {
        de::Deserialize::deserialize(self)
    }
    /// Index into a TOML array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index(self)
    }
    /// Mutably index into a TOML array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_mut(self)
    }
    /// Extracts the integer value if it is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match *self {
            Value::Integer(i) => Some(i),
            _ => None,
        }
    }
    /// Tests whether this value is an integer.
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }
    /// Extracts the float value if it is a float.
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }
    /// Tests whether this value is a float.
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }
    /// Extracts the boolean value if it is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Boolean(b) => Some(b),
            _ => None,
        }
    }
    /// Tests whether this value is a boolean.
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }
    /// Extracts the string of this value if it is a string.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(&**s),
            _ => None,
        }
    }
    /// Tests if this value is a string.
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }
    /// Extracts the datetime value if it is a datetime.
    ///
    /// Note that a parsed TOML value will only contain ISO 8601 dates. An
    /// example date is:
    ///
    /// ```notrust
    /// 1979-05-27T07:32:00Z
    /// ```
    pub fn as_datetime(&self) -> Option<&Datetime> {
        match *self {
            Value::Datetime(ref s) => Some(s),
            _ => None,
        }
    }
    /// Tests whether this value is a datetime.
    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }
    /// Extracts the array value if it is an array.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref s) => Some(s),
            _ => None,
        }
    }
    /// Extracts the array value if it is an array.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut s) => Some(s),
            _ => None,
        }
    }
    /// Tests whether this value is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    /// Extracts the table value if it is a table.
    pub fn as_table(&self) -> Option<&Table> {
        match *self {
            Value::Table(ref s) => Some(s),
            _ => None,
        }
    }
    /// Extracts the table value if it is a table.
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match *self {
            Value::Table(ref mut s) => Some(s),
            _ => None,
        }
    }
    /// Tests whether this value is a table.
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }
    /// Tests whether this and another value have the same type.
    pub fn same_type(&self, other: &Value) -> bool {
        discriminant(self) == discriminant(other)
    }
    /// Returns a human-readable representation of the type of this value.
    pub fn type_str(&self) -> &'static str {
        match *self {
            Value::String(..) => "string",
            Value::Integer(..) => "integer",
            Value::Float(..) => "float",
            Value::Boolean(..) => "boolean",
            Value::Datetime(..) => "datetime",
            Value::Array(..) => "array",
            Value::Table(..) => "table",
        }
    }
}
impl<I> ops::Index<I> for Value
where
    I: Index,
{
    type Output = Value;
    fn index(&self, index: I) -> &Value {
        self.get(index).expect("index not found")
    }
}
impl<I> ops::IndexMut<I> for Value
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Value {
        self.get_mut(index).expect("index not found")
    }
}
impl<'a> From<&'a str> for Value {
    #[inline]
    fn from(val: &'a str) -> Value {
        Value::String(val.to_string())
    }
}
impl<V: Into<Value>> From<Vec<V>> for Value {
    fn from(val: Vec<V>) -> Value {
        Value::Array(val.into_iter().map(|v| v.into()).collect())
    }
}
impl<S: Into<String>, V: Into<Value>> From<BTreeMap<S, V>> for Value {
    fn from(val: BTreeMap<S, V>) -> Value {
        let table = val.into_iter().map(|(s, v)| (s.into(), v.into())).collect();
        Value::Table(table)
    }
}
impl<S: Into<String> + Hash + Eq, V: Into<Value>> From<HashMap<S, V>> for Value {
    fn from(val: HashMap<S, V>) -> Value {
        let table = val.into_iter().map(|(s, v)| (s.into(), v.into())).collect();
        Value::Table(table)
    }
}
macro_rules! impl_into_value {
    ($variant:ident : $T:ty) => {
        impl From <$T > for Value { #[inline] fn from(val : $T) -> Value {
        Value::$variant (val.into()) } }
    };
}
impl_into_value!(String : String);
impl_into_value!(Integer : i64);
impl_into_value!(Integer : i32);
impl_into_value!(Integer : i8);
impl_into_value!(Integer : u8);
impl_into_value!(Integer : u32);
impl_into_value!(Float : f64);
impl_into_value!(Float : f32);
impl_into_value!(Boolean : bool);
impl_into_value!(Datetime : Datetime);
impl_into_value!(Table : Table);
/// Types that can be used to index a `toml::Value`
///
/// Currently this is implemented for `usize` to index arrays and `str` to index
/// tables.
///
/// This trait is sealed and not intended for implementation outside of the
/// `toml` crate.
pub trait Index: Sealed {
    #[doc(hidden)]
    fn index<'a>(&self, val: &'a Value) -> Option<&'a Value>;
    #[doc(hidden)]
    fn index_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value>;
}
/// An implementation detail that should not be implemented, this will change in
/// the future and break code otherwise.
#[doc(hidden)]
pub trait Sealed {}
impl Sealed for usize {}
impl Sealed for str {}
impl Sealed for String {}
impl<'a, T: Sealed + ?Sized> Sealed for &'a T {}
impl Index for usize {
    fn index<'a>(&self, val: &'a Value) -> Option<&'a Value> {
        match *val {
            Value::Array(ref a) => a.get(*self),
            _ => None,
        }
    }
    fn index_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
        match *val {
            Value::Array(ref mut a) => a.get_mut(*self),
            _ => None,
        }
    }
}
impl Index for str {
    fn index<'a>(&self, val: &'a Value) -> Option<&'a Value> {
        match *val {
            Value::Table(ref a) => a.get(self),
            _ => None,
        }
    }
    fn index_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
        match *val {
            Value::Table(ref mut a) => a.get_mut(self),
            _ => None,
        }
    }
}
impl Index for String {
    fn index<'a>(&self, val: &'a Value) -> Option<&'a Value> {
        self[..].index(val)
    }
    fn index_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
        self[..].index_mut(val)
    }
}
impl<'s, T: ?Sized> Index for &'s T
where
    T: Index,
{
    fn index<'a>(&self, val: &'a Value) -> Option<&'a Value> {
        (**self).index(val)
    }
    fn index_mut<'a>(&self, val: &'a mut Value) -> Option<&'a mut Value> {
        (**self).index_mut(val)
    }
}
#[cfg(feature = "display")]
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use serde::Serialize as _;
        let mut output = String::new();
        let serializer = crate::ser::ValueSerializer::new(&mut output);
        self.serialize(serializer).unwrap();
        output.fmt(f)
    }
}
#[cfg(feature = "parse")]
impl std::str::FromStr for Value {
    type Err = crate::de::Error;
    fn from_str(s: &str) -> Result<Value, Self::Err> {
        crate::from_str(s)
    }
}
impl ser::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use serde::ser::SerializeMap;
        match *self {
            Value::String(ref s) => serializer.serialize_str(s),
            Value::Integer(i) => serializer.serialize_i64(i),
            Value::Float(f) => serializer.serialize_f64(f),
            Value::Boolean(b) => serializer.serialize_bool(b),
            Value::Datetime(ref s) => s.serialize(serializer),
            Value::Array(ref a) => a.serialize(serializer),
            Value::Table(ref t) => {
                let mut map = serializer.serialize_map(Some(t.len()))?;
                for (k, v) in t {
                    if !v.is_table() && !v.is_array()
                        || (v
                            .as_array()
                            .map(|a| !a.iter().any(|v| v.is_table()))
                            .unwrap_or(false))
                    {
                        map.serialize_entry(k, v)?;
                    }
                }
                for (k, v) in t {
                    if v
                        .as_array()
                        .map(|a| a.iter().any(|v| v.is_table()))
                        .unwrap_or(false)
                    {
                        map.serialize_entry(k, v)?;
                    }
                }
                for (k, v) in t {
                    if v.is_table() {
                        map.serialize_entry(k, v)?;
                    }
                }
                map.end()
            }
        }
    }
}
impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("any valid TOML value")
            }
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Boolean(value))
            }
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Integer(value))
            }
            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Value, E> {
                if value <= i64::max_value() as u64 {
                    Ok(Value::Integer(value as i64))
                } else {
                    Err(de::Error::custom("u64 value was too large"))
                }
            }
            fn visit_u32<E>(self, value: u32) -> Result<Value, E> {
                Ok(Value::Integer(value.into()))
            }
            fn visit_i32<E>(self, value: i32) -> Result<Value, E> {
                Ok(Value::Integer(value.into()))
            }
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::Float(value))
            }
            fn visit_str<E>(self, value: &str) -> Result<Value, E> {
                Ok(Value::String(value.into()))
            }
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value))
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                de::Deserialize::deserialize(deserializer)
            }
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut key = String::new();
                let datetime = visitor.next_key_seed(DatetimeOrTable { key: &mut key })?;
                match datetime {
                    Some(true) => {
                        let date: datetime::DatetimeFromString = visitor.next_value()?;
                        return Ok(Value::Datetime(date.value));
                    }
                    None => return Ok(Value::Table(Table::new())),
                    Some(false) => {}
                }
                let mut map = Table::new();
                map.insert(key, visitor.next_value()?);
                while let Some(key) = visitor.next_key::<String>()? {
                    if let crate::map::Entry::Vacant(vacant) = map.entry(&key) {
                        vacant.insert(visitor.next_value()?);
                    } else {
                        let msg = format!("duplicate key: `{}`", key);
                        return Err(de::Error::custom(msg));
                    }
                }
                Ok(Value::Table(map))
            }
        }
        deserializer.deserialize_any(ValueVisitor)
    }
}
impl<'de> de::Deserializer<'de> for Value {
    type Error = crate::de::Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::Boolean(v) => visitor.visit_bool(v),
            Value::Integer(n) => visitor.visit_i64(n),
            Value::Float(n) => visitor.visit_f64(n),
            Value::String(v) => visitor.visit_string(v),
            Value::Datetime(v) => visitor.visit_string(v.to_string()),
            Value::Array(v) => {
                let len = v.len();
                let mut deserializer = SeqDeserializer::new(v);
                let seq = visitor.visit_seq(&mut deserializer)?;
                let remaining = deserializer.iter.len();
                if remaining == 0 {
                    Ok(seq)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in array"))
                }
            }
            Value::Table(v) => {
                let len = v.len();
                let mut deserializer = MapDeserializer::new(v);
                let map = visitor.visit_map(&mut deserializer)?;
                let remaining = deserializer.iter.len();
                if remaining == 0 {
                    Ok(map)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in map"))
                }
            }
        }
    }
    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::String(variant) => visitor.visit_enum(variant.into_deserializer()),
            Value::Table(variant) => {
                use de::Error;
                if variant.is_empty() {
                    Err(
                        crate::de::Error::custom(
                            "wanted exactly 1 element, found 0 elements",
                        ),
                    )
                } else if variant.len() != 1 {
                    Err(
                        crate::de::Error::custom(
                            "wanted exactly 1 element, more than 1 element",
                        ),
                    )
                } else {
                    let deserializer = MapDeserializer::new(variant);
                    visitor.visit_enum(deserializer)
                }
            }
            _ => {
                Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"string only"))
            }
        }
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq bytes
        byte_buf map unit_struct tuple_struct struct tuple ignored_any identifier
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
impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = crate::de::Error;
    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, crate::de::Error>
    where
        T: de::DeserializeSeed<'de>,
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
    iter: <Table as IntoIterator>::IntoIter,
    value: Option<(String, Value)>,
}
impl MapDeserializer {
    fn new(map: Table) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}
impl<'de> de::MapAccess<'de> for MapDeserializer {
    type Error = crate::de::Error;
    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, crate::de::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some((key.clone(), value));
                seed.deserialize(Value::String(key)).map(Some)
            }
            None => Ok(None),
        }
    }
    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, crate::de::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let (key, res) = match self.value.take() {
            Some((key, value)) => (key, seed.deserialize(value)),
            None => return Err(de::Error::custom("value is missing")),
        };
        res.map_err(|mut error| {
            error.add_key(key);
            error
        })
    }
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
impl<'de> de::EnumAccess<'de> for MapDeserializer {
    type Error = crate::de::Error;
    type Variant = MapEnumDeserializer;
    fn variant_seed<V>(
        mut self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        use de::Error;
        let (key, value) = match self.iter.next() {
            Some(pair) => pair,
            None => {
                return Err(
                    Error::custom(
                        "expected table with exactly 1 entry, found empty table",
                    ),
                );
            }
        };
        let val = seed.deserialize(key.into_deserializer())?;
        let variant = MapEnumDeserializer::new(value);
        Ok((val, variant))
    }
}
/// Deserializes table values into enum variants.
pub(crate) struct MapEnumDeserializer {
    value: Value,
}
impl MapEnumDeserializer {
    pub(crate) fn new(value: Value) -> Self {
        MapEnumDeserializer { value }
    }
}
impl<'de> serde::de::VariantAccess<'de> for MapEnumDeserializer {
    type Error = crate::de::Error;
    fn unit_variant(self) -> Result<(), Self::Error> {
        use de::Error;
        match self.value {
            Value::Table(values) => {
                if values.is_empty() {
                    Ok(())
                } else {
                    Err(Error::custom("expected empty table"))
                }
            }
            e => Err(Error::custom(format!("expected table, found {}", e.type_str()))),
        }
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.value.into_deserializer())
    }
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        use de::Error;
        match self.value {
            Value::Table(values) => {
                let tuple_values = values
                    .into_iter()
                    .enumerate()
                    .map(|(index, (key, value))| match key.parse::<usize>() {
                        Ok(key_index) if key_index == index => Ok(value),
                        Ok(_) | Err(_) => {
                            Err(
                                Error::custom(
                                    format!("expected table key `{}`, but was `{}`", index, key),
                                ),
                            )
                        }
                    })
                    .fold(
                        Ok(Vec::with_capacity(len)),
                        |result, value_result| {
                            result
                                .and_then(move |mut tuple_values| match value_result {
                                    Ok(value) => {
                                        tuple_values.push(value);
                                        Ok(tuple_values)
                                    }
                                    Err(e) => Err(e),
                                })
                        },
                    )?;
                if tuple_values.len() == len {
                    serde::de::Deserializer::deserialize_seq(
                        tuple_values.into_deserializer(),
                        visitor,
                    )
                } else {
                    Err(Error::custom(format!("expected tuple with length {}", len)))
                }
            }
            e => Err(Error::custom(format!("expected table, found {}", e.type_str()))),
        }
    }
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_struct(
            self.value.into_deserializer(),
            "",
            fields,
            visitor,
        )
    }
}
impl<'de> de::IntoDeserializer<'de, crate::de::Error> for Value {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}
struct ValueSerializer;
impl ser::Serializer for ValueSerializer {
    type Ok = Value;
    type Error = crate::ser::Error;
    type SerializeSeq = ValueSerializeVec;
    type SerializeTuple = ValueSerializeVec;
    type SerializeTupleStruct = ValueSerializeVec;
    type SerializeTupleVariant = ValueSerializeVec;
    type SerializeMap = ValueSerializeMap;
    type SerializeStruct = ValueSerializeMap;
    type SerializeStructVariant = ser::Impossible<Value, crate::ser::Error>;
    fn serialize_bool(self, value: bool) -> Result<Value, crate::ser::Error> {
        Ok(Value::Boolean(value))
    }
    fn serialize_i8(self, value: i8) -> Result<Value, crate::ser::Error> {
        self.serialize_i64(value.into())
    }
    fn serialize_i16(self, value: i16) -> Result<Value, crate::ser::Error> {
        self.serialize_i64(value.into())
    }
    fn serialize_i32(self, value: i32) -> Result<Value, crate::ser::Error> {
        self.serialize_i64(value.into())
    }
    fn serialize_i64(self, value: i64) -> Result<Value, crate::ser::Error> {
        Ok(Value::Integer(value))
    }
    fn serialize_u8(self, value: u8) -> Result<Value, crate::ser::Error> {
        self.serialize_i64(value.into())
    }
    fn serialize_u16(self, value: u16) -> Result<Value, crate::ser::Error> {
        self.serialize_i64(value.into())
    }
    fn serialize_u32(self, value: u32) -> Result<Value, crate::ser::Error> {
        self.serialize_i64(value.into())
    }
    fn serialize_u64(self, value: u64) -> Result<Value, crate::ser::Error> {
        if value <= i64::max_value() as u64 {
            self.serialize_i64(value as i64)
        } else {
            Err(ser::Error::custom("u64 value was too large"))
        }
    }
    fn serialize_f32(self, value: f32) -> Result<Value, crate::ser::Error> {
        self.serialize_f64(value.into())
    }
    fn serialize_f64(self, value: f64) -> Result<Value, crate::ser::Error> {
        Ok(Value::Float(value))
    }
    fn serialize_char(self, value: char) -> Result<Value, crate::ser::Error> {
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }
    fn serialize_str(self, value: &str) -> Result<Value, crate::ser::Error> {
        Ok(Value::String(value.to_owned()))
    }
    fn serialize_bytes(self, value: &[u8]) -> Result<Value, crate::ser::Error> {
        let vec = value.iter().map(|&b| Value::Integer(b.into())).collect();
        Ok(Value::Array(vec))
    }
    fn serialize_unit(self) -> Result<Value, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some("unit")))
    }
    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> Result<Value, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some(name)))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Value, crate::ser::Error> {
        self.serialize_str(_variant)
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(ValueSerializer)?;
        let mut table = Table::new();
        table.insert(variant.to_owned(), value);
        Ok(table.into())
    }
    fn serialize_none(self) -> Result<Value, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_none())
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Value, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, crate::ser::Error> {
        Ok(ValueSerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }
    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple, crate::ser::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, crate::ser::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, crate::ser::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, crate::ser::Error> {
        Ok(ValueSerializeMap {
            ser: SerializeMap {
                map: Table::new(),
                next_key: None,
            },
        })
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, crate::ser::Error> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some(name)))
    }
}
pub(crate) struct TableSerializer;
impl ser::Serializer for TableSerializer {
    type Ok = Table;
    type Error = crate::ser::Error;
    type SerializeSeq = ser::Impossible<Table, crate::ser::Error>;
    type SerializeTuple = ser::Impossible<Table, crate::ser::Error>;
    type SerializeTupleStruct = ser::Impossible<Table, crate::ser::Error>;
    type SerializeTupleVariant = ser::Impossible<Table, crate::ser::Error>;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = ser::Impossible<Table, crate::ser::Error>;
    fn serialize_bool(self, _value: bool) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_i8(self, _value: i8) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_i16(self, _value: i16) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_i32(self, _value: i32) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_i64(self, _value: i64) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_u8(self, _value: u8) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_u16(self, _value: u16) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_u32(self, _value: u32) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_u64(self, _value: u64) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_f32(self, _value: f32) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_f64(self, _value: f64) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_char(self, _value: char) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_str(self, _value: &str) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_bytes(self, _value: &[u8]) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_unit(self) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some(name)))
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Table, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Table, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(ValueSerializer)?;
        let mut table = Table::new();
        table.insert(variant.to_owned(), value);
        Ok(table)
    }
    fn serialize_none(self) -> Result<Table, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_none())
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Table, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(None))
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some(name)))
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some(name)))
    }
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, crate::ser::Error> {
        Ok(SerializeMap {
            map: Table::new(),
            next_key: None,
        })
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, crate::ser::Error> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, crate::ser::Error> {
        Err(crate::ser::Error::unsupported_type(Some(name)))
    }
}
struct ValueSerializeVec {
    vec: Vec<Value>,
}
impl ser::SerializeSeq for ValueSerializeVec {
    type Ok = Value;
    type Error = crate::ser::Error;
    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        self.vec.push(Value::try_from(value)?);
        Ok(())
    }
    fn end(self) -> Result<Value, crate::ser::Error> {
        Ok(Value::Array(self.vec))
    }
}
impl ser::SerializeTuple for ValueSerializeVec {
    type Ok = Value;
    type Error = crate::ser::Error;
    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Value, crate::ser::Error> {
        ser::SerializeSeq::end(self)
    }
}
impl ser::SerializeTupleStruct for ValueSerializeVec {
    type Ok = Value;
    type Error = crate::ser::Error;
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Value, crate::ser::Error> {
        ser::SerializeSeq::end(self)
    }
}
impl ser::SerializeTupleVariant for ValueSerializeVec {
    type Ok = Value;
    type Error = crate::ser::Error;
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Value, crate::ser::Error> {
        ser::SerializeSeq::end(self)
    }
}
pub(crate) struct SerializeMap {
    map: Table,
    next_key: Option<String>,
}
impl ser::SerializeMap for SerializeMap {
    type Ok = Table;
    type Error = crate::ser::Error;
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        match Value::try_from(key)? {
            Value::String(s) => self.next_key = Some(s),
            _ => return Err(crate::ser::Error::key_not_string()),
        };
        Ok(())
    }
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        let key = self.next_key.take();
        let key = key.expect("serialize_value called before serialize_key");
        match Value::try_from(value) {
            Ok(value) => {
                self.map.insert(key, value);
            }
            Err(
                crate::ser::Error { inner: crate::edit::ser::Error::UnsupportedNone },
            ) => {}
            Err(e) => return Err(e),
        }
        Ok(())
    }
    fn end(self) -> Result<Table, crate::ser::Error> {
        Ok(self.map)
    }
}
impl ser::SerializeStruct for SerializeMap {
    type Ok = Table;
    type Error = crate::ser::Error;
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)
    }
    fn end(self) -> Result<Table, crate::ser::Error> {
        ser::SerializeMap::end(self)
    }
}
struct ValueSerializeMap {
    ser: SerializeMap,
}
impl ser::SerializeMap for ValueSerializeMap {
    type Ok = Value;
    type Error = crate::ser::Error;
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        self.ser.serialize_key(key)
    }
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        self.ser.serialize_value(value)
    }
    fn end(self) -> Result<Value, crate::ser::Error> {
        self.ser.end().map(Value::Table)
    }
}
impl ser::SerializeStruct for ValueSerializeMap {
    type Ok = Value;
    type Error = crate::ser::Error;
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), crate::ser::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)
    }
    fn end(self) -> Result<Value, crate::ser::Error> {
        ser::SerializeMap::end(self)
    }
}
struct DatetimeOrTable<'a> {
    key: &'a mut String,
}
impl<'a, 'de> de::DeserializeSeed<'de> for DatetimeOrTable<'a> {
    type Value = bool;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}
impl<'a, 'de> de::Visitor<'de> for DatetimeOrTable<'a> {
    type Value = bool;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string key")
    }
    fn visit_str<E>(self, s: &str) -> Result<bool, E>
    where
        E: de::Error,
    {
        if s == datetime::FIELD {
            Ok(true)
        } else {
            self.key.push_str(s);
            Ok(false)
        }
    }
    fn visit_string<E>(self, s: String) -> Result<bool, E>
    where
        E: de::Error,
    {
        if s == datetime::FIELD {
            Ok(true)
        } else {
            *self.key = s;
            Ok(false)
        }
    }
}
#[cfg(test)]
mod tests_llm_16_3_llm_16_3 {
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn index_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let value = Value::Table(map);
        debug_assert_eq!(
            value.get(rug_fuzz_2), Some(& Value::String("value".to_string()))
        );
             }
});    }
    #[test]
    fn index_string_key_not_found() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        let value = Value::Table(map);
        debug_assert!(value.get(rug_fuzz_0).is_none());
             }
});    }
    #[test]
    fn index_integer_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        let value = Value::Table(map);
        debug_assert_eq!(value.get(rug_fuzz_2), Some(& Value::Integer(42)));
             }
});    }
    #[test]
    fn index_mut_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let mut value = Value::Table(map);
        if let Some(val) = value.get_mut(rug_fuzz_2) {
            *val = Value::String(rug_fuzz_3.to_string());
        }
        debug_assert_eq!(
            value.get(rug_fuzz_4), Some(& Value::String("new_value".to_string()))
        );
             }
});    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn index_mut_string_key_not_found() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let mut value = Value::Table(map);
        let _ = value.get_mut(rug_fuzz_0).expect(rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_4_llm_16_4 {
    use crate::map::Map;
    use crate::value::{Index, Value};
    #[test]
    fn test_index_mut_found_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let mut val = Value::String(String::from(rug_fuzz_1));
        map.insert(key.clone(), val.clone());
        let mut value = Value::Table(map);
        if let Some(v) = key.index_mut(&mut value) {
            debug_assert_eq!(v, & mut val);
        } else {
            panic!("Expected to find the key");
        }
             }
});    }
    #[test]
    fn test_index_mut_missing_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        let key = String::from(rug_fuzz_0);
        let mut value = Value::Table(map);
        debug_assert!(key.index_mut(& mut value).is_none());
             }
});    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_mut_panic_missing_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        let key = String::from(rug_fuzz_0);
        let mut value = Value::Table(map);
        let _ = key.index_mut(&mut value);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_150_llm_16_150 {
    use crate::value::{Value, Table};
    #[test]
    fn test_index_mut_string_key_exists() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let key = rug_fuzz_0.to_string();
        let mut table = Table::new();
        table.insert(key.clone(), Value::String(rug_fuzz_1.to_string()));
        let mut value = Value::Table(table);
        if let Value::Table(ref mut table) = &mut value {
            if let Some(result) = table.get_mut(&key) {
                debug_assert_eq!(result, & mut Value::String("bar".to_string()));
            } else {
                panic!("Expected a value for key 'foo'");
            }
        } else {
            panic!("Expected a table");
        }
             }
});    }
    #[test]
    fn test_index_mut_string_key_does_not_exist() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let key = rug_fuzz_0.to_string();
        let mut table = Table::new();
        table.insert(rug_fuzz_1.to_string(), Value::String(rug_fuzz_2.to_string()));
        let mut value = Value::Table(table);
        if let Value::Table(ref mut table) = &mut value {
            debug_assert!(table.get_mut(& key).is_none());
        } else {
            panic!("Expected a table");
        }
             }
});    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_mut_string_key_does_not_exist_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let key = rug_fuzz_0.to_string();
        let mut table = Table::new();
        table.insert(rug_fuzz_1.to_string(), Value::String(rug_fuzz_2.to_string()));
        let mut value = Value::Table(table);
        if let Value::Table(ref mut table) = &mut value {
            let _ = &mut table[&key];
        } else {
            panic!("Expected a table");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_151_llm_16_151 {
    use crate::value::{Index, Value};
    use crate::map::Map;
    use std::str::FromStr;
    use std::borrow::Borrow;
    use std::hash::Hash;
    #[test]
    fn index_string_key_in_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::String(String::from(rug_fuzz_1));
        map.insert(key.clone(), value.clone());
        let table = Value::Table(map);
        let index_key = key.clone();
        let result = index_key.index(&table);
        debug_assert_eq!(result, Some(& value));
             }
});    }
    #[test]
    fn index_string_key_not_in_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        let table = Value::Table(map);
        let index_key = String::from(rug_fuzz_0);
        let result = index_key.index(&table);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn index_non_string_in_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::Integer(rug_fuzz_1);
        map.insert(key.clone(), value.clone());
        let table = Value::Table(map);
        let index_key = String::from_str(&key).unwrap();
        let result = index_key.index(&table);
        debug_assert_eq!(result, Some(& value));
             }
});    }
    #[test]
    fn index_string_in_non_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index_key = String::from(rug_fuzz_0);
        let value = Value::String(String::from(rug_fuzz_1));
        let result = index_key.index(&value);
        debug_assert_eq!(result, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_153 {
    use super::*;
    use crate::*;
    #[test]
    fn test_index_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let array = Value::Array(
            vec![
                Value::String(rug_fuzz_1.to_string()), Value::String("one".to_string()),
                Value::String("two".to_string())
            ],
        );
        let result = index.index(&array);
        debug_assert!(result.is_some());
        debug_assert_eq!(result, Some(& Value::String("one".to_string())));
             }
});    }
    #[test]
    fn test_index_non_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let value = Value::String(rug_fuzz_1.to_string());
        let result = index.index(&value);
        debug_assert!(result.is_none());
             }
});    }
    #[test]
    fn test_index_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let array = Value::Array(
            vec![
                Value::String(rug_fuzz_1.to_string()), Value::String("one".to_string()),
                Value::String("two".to_string())
            ],
        );
        let result = index.index(&array);
        debug_assert!(result.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_154 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn index_mut_returns_none_for_non_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, bool, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v_integer = Value::Integer(rug_fuzz_0);
        let mut v_boolean = Value::Boolean(rug_fuzz_1);
        let mut v_string = Value::String(rug_fuzz_2.to_owned());
        let index = rug_fuzz_3;
        debug_assert_eq!(usize::index_mut(& index, & mut v_integer), None);
        debug_assert_eq!(usize::index_mut(& index, & mut v_boolean), None);
        debug_assert_eq!(usize::index_mut(& index, & mut v_string), None);
             }
});    }
    #[test]
    fn index_mut_returns_some_for_array_with_index() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v_array = Value::Array(
            vec![Value::Integer(rug_fuzz_0), Value::Integer(2), Value::Integer(3)],
        );
        let index = rug_fuzz_1;
        let v_index_mut = usize::index_mut(&index, &mut v_array);
        debug_assert!(v_index_mut.is_some());
        debug_assert_eq!(* v_index_mut.unwrap(), Value::Integer(2));
             }
});    }
    #[test]
    fn index_mut_returns_none_for_array_without_index() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v_array = Value::Array(
            vec![Value::Integer(rug_fuzz_0), Value::Integer(2), Value::Integer(3)],
        );
        let index = rug_fuzz_1;
        debug_assert_eq!(usize::index_mut(& index, & mut v_array), None);
             }
});    }
    #[test]
    fn index_mut_modifies_array_element() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, usize, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v_array = Value::Array(
            vec![Value::Integer(rug_fuzz_0), Value::Integer(2), Value::Integer(3)],
        );
        let index = rug_fuzz_1;
        if let Some(v) = usize::index_mut(&index, &mut v_array) {
            *v = Value::Integer(rug_fuzz_2);
        }
        debug_assert_eq!(
            v_array, Value::Array(vec![Value::Integer(1), Value::Integer(42),
            Value::Integer(3)])
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_166_llm_16_166 {
    use crate::de;
    use crate::value::{Value, Table, MapEnumDeserializer};
    use serde::de::VariantAccess;
    #[test]
    fn test_unit_variant_empty_table() {
        let _rug_st_tests_llm_16_166_llm_16_166_rrrruuuugggg_test_unit_variant_empty_table = 0;
        let table = Table::new();
        let value = Value::Table(table);
        let deserializer = MapEnumDeserializer::new(value);
        debug_assert!(deserializer.unit_variant().is_ok());
        let _rug_ed_tests_llm_16_166_llm_16_166_rrrruuuugggg_test_unit_variant_empty_table = 0;
    }
    #[test]
    fn test_unit_variant_non_empty_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let value = Value::Table(table);
        let deserializer = MapEnumDeserializer::new(value);
        debug_assert!(deserializer.unit_variant().is_err());
             }
});    }
    #[test]
    fn test_unit_variant_wrong_type() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_string());
        let deserializer = MapEnumDeserializer::new(value);
        debug_assert!(deserializer.unit_variant().is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_168_llm_16_168 {
    use crate::value::SeqDeserializer;
    use serde::de::SeqAccess;
    use crate::Value;
    #[test]
    fn test_size_hint_equal_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let values = vec![
            Value::String(rug_fuzz_0.to_string()), Value::String("b".to_string())
        ];
        let seq_deserializer = SeqDeserializer::new(values);
        debug_assert_eq!(seq_deserializer.size_hint(), Some(2));
             }
});    }
    #[test]
    fn test_size_hint_different_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let values = vec![
            Value::String(rug_fuzz_0.to_string()), Value::String("b".to_string())
        ];
        let mut seq_deserializer = SeqDeserializer::new(values);
        seq_deserializer.iter.next();
        debug_assert_eq!(seq_deserializer.size_hint(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_177 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    #[test]
    fn test_serialize_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table_serializer = TableSerializer;
        let result = table_serializer.serialize_f32(rug_fuzz_0);
        debug_assert!(result.is_err());
        match result {
            Err(e) => debug_assert_eq!(e, crate ::ser::Error::unsupported_type(None)),
            _ => panic!("Expected error for serialize_f32 with TableSerializer"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_183 {
    use super::*;
    use crate::*;
    use serde::ser::{Serialize, Serializer};
    use crate::value::{Table, TableSerializer, SerializeMap};
    #[test]
    fn serialize_map_creates_empty_table() {
        let _rug_st_tests_llm_16_183_rrrruuuugggg_serialize_map_creates_empty_table = 0;
        let serializer = TableSerializer;
        let result = serializer.serialize_map(None);
        debug_assert!(result.is_ok());
        let serialize_map = result.unwrap();
        debug_assert!(serialize_map.map.is_empty());
        debug_assert!(serialize_map.next_key.is_none());
        let _rug_ed_tests_llm_16_183_rrrruuuugggg_serialize_map_creates_empty_table = 0;
    }
    #[test]
    fn serialize_map_with_length_creates_empty_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = TableSerializer;
        let result = serializer.serialize_map(Some(rug_fuzz_0));
        debug_assert!(result.is_ok());
        let serialize_map = result.unwrap();
        debug_assert!(serialize_map.map.is_empty());
        debug_assert!(serialize_map.next_key.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_186 {
    use crate::value::TableSerializer;
    use serde::Serializer;
    #[test]
    fn test_serialize_none() {
        let _rug_st_tests_llm_16_186_rrrruuuugggg_test_serialize_none = 0;
        let serializer = TableSerializer;
        let result = serializer.serialize_none();
        debug_assert!(result.is_err());
        match result {
            Err(e) => {
                debug_assert_eq!(
                    format!("{}", e), "a None value is not supported in TOML"
                )
            }
            _ => panic!("Expected error for serialize_none"),
        }
        let _rug_ed_tests_llm_16_186_rrrruuuugggg_test_serialize_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_189 {
    use super::*;
    use crate::*;
    use crate::value::TableSerializer;
    use serde::Serializer;
    #[test]
    fn test_serialize_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = TableSerializer;
        let result = serializer.serialize_str(rug_fuzz_0);
        debug_assert!(result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_205_llm_16_205 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    use crate::value::Value;
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::fmt;
    struct NewtypeStructVisitor;
    impl<'de> Visitor<'de> for NewtypeStructVisitor {
        type Value = Value;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a newtype struct")
        }
        fn visit_newtype_struct<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Value::deserialize(deserializer)
        }
    }
    #[test]
    fn test_deserialize_newtype_struct() {
        let _rug_st_tests_llm_16_205_llm_16_205_rrrruuuugggg_test_deserialize_newtype_struct = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "NewtypeStruct";
        let value = Value::from(rug_fuzz_0);
        let deserializer = value.clone().into_deserializer();
        let result: Result<Value, crate::de::Error> = Value::deserialize_newtype_struct(
            deserializer,
            rug_fuzz_1,
            NewtypeStructVisitor,
        );
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.ok(), Some(value));
        let _rug_ed_tests_llm_16_205_llm_16_205_rrrruuuugggg_test_deserialize_newtype_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_206_llm_16_206 {
    use serde::de::{self, IntoDeserializer};
    use crate::value::{Value, Table};
    use crate::map::Map;
    #[test]
    fn deserialize_option_some() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        let deserializer = value.into_deserializer();
        let result: Result<Option<String>, crate::de::Error> = de::Deserialize::deserialize(
            deserializer,
        );
        debug_assert_eq!(result.unwrap(), Some("test".to_owned()));
             }
});    }
    #[test]
    fn deserialize_option_none() {
        let _rug_st_tests_llm_16_206_llm_16_206_rrrruuuugggg_deserialize_option_none = 0;
        let value = Value::Table(Table::new());
        let deserializer = value.into_deserializer();
        let result: Result<Option<Map<String, Value>>, crate::de::Error> = de::Deserialize::deserialize(
            deserializer,
        );
        debug_assert!(result.is_ok());
        debug_assert!(result.unwrap().is_none());
        let _rug_ed_tests_llm_16_206_llm_16_206_rrrruuuugggg_deserialize_option_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_208 {
    use crate::Value;
    use serde::de::IntoDeserializer;
    #[test]
    fn test_into_deserializer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, i64, f64, bool, &str, i64, f64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_string = Value::String(rug_fuzz_0.to_string());
        let value_int = Value::Integer(rug_fuzz_1);
        let value_float = Value::Float(rug_fuzz_2);
        let value_bool = Value::Boolean(rug_fuzz_3);
        let deserializer_string = value_string.into_deserializer();
        let deserializer_int = value_int.into_deserializer();
        let deserializer_float = value_float.into_deserializer();
        let deserializer_bool = value_bool.into_deserializer();
        debug_assert_eq!(Value::String(rug_fuzz_4.to_string()), deserializer_string);
        debug_assert_eq!(Value::Integer(rug_fuzz_5), deserializer_int);
        debug_assert_eq!(Value::Float(rug_fuzz_6), deserializer_float);
        debug_assert_eq!(Value::Boolean(rug_fuzz_7), deserializer_bool);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_209 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_str_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let expected = Value::String(String::from(input));
        let result = Value::from(input);
        debug_assert_eq!(expected, result);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_210 {
    use super::*;
    use crate::*;
    #[test]
    fn from_boolean_into_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bool_value: bool = rug_fuzz_0;
        let toml_value: Value = Value::from(bool_value);
        debug_assert!(toml_value.is_bool());
        debug_assert_eq!(toml_value, Value::Boolean(true));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_211 {
    use crate::value::Value;
    #[test]
    fn test_value_from_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let float_value: f32 = rug_fuzz_0;
        let value: Value = Value::from(float_value);
        match value {
            Value::Float(f) => debug_assert_eq!(f, float_value as f64),
            _ => panic!("from(f32) didn't create Value::Float"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_212 {
    use super::*;
    use crate::*;
    #[test]
    fn from_f64_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let float_value = rug_fuzz_0;
        let value = Value::from(float_value);
        debug_assert!(
            matches!(value, Value::Float(v) if (v - float_value).abs() <
            std::f64::EPSILON)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_213 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i32 = rug_fuzz_0;
        let value = Value::from(num);
        debug_assert_eq!(value, Value::Integer(42));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_214 {
    use super::*;
    use crate::*;
    #[test]
    fn from_i64_creates_integer_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i64 = rug_fuzz_0;
        let value = Value::from(num);
        debug_assert_eq!(value, Value::Integer(num));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_215 {
    use crate::Value;
    use std::convert::From;
    #[test]
    fn test_from_i8_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i8 = rug_fuzz_0;
        let value: Value = Value::from(num);
        debug_assert!(matches!(value, Value::Integer(42)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_216 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::Map;
    use std::string::String;
    #[test]
    fn test_from_map_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(String::from(rug_fuzz_0), Value::String(String::from(rug_fuzz_1)));
        map.insert(String::from(rug_fuzz_2), Value::Integer(rug_fuzz_3));
        let value = Value::from(map);
        match value {
            Value::Table(table) => {
                debug_assert_eq!(
                    table.get(rug_fuzz_4), Some(& Value::String(String::from("value1")))
                );
                debug_assert_eq!(table.get(rug_fuzz_5), Some(& Value::Integer(42)));
            }
            _ => panic!("Value::from(map) did not produce a Value::Table"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_217 {
    use super::*;
    use crate::*;
    use serde::de::{self, IntoDeserializer};
    use std::collections::BTreeMap;
    #[test]
    fn test_from_btree_map() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, i64, &str, bool, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = BTreeMap::new();
        map.insert(rug_fuzz_0, Value::String(rug_fuzz_1.into()));
        map.insert(rug_fuzz_2, Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4, Value::Boolean(rug_fuzz_5));
        let value = Value::from(map);
        match value {
            Value::Table(ref table) => {
                debug_assert_eq!(
                    table.get(rug_fuzz_6), Some(& Value::String("value1".into()))
                );
                debug_assert_eq!(table.get(rug_fuzz_7), Some(& Value::Integer(42)));
                debug_assert_eq!(table.get(rug_fuzz_8), Some(& Value::Boolean(true)));
            }
            _ => panic!("Value::from should have created a Value::Table"),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_219 {
    use crate::value::Value;
    use std::convert::From;
    #[test]
    fn test_from_string_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_string = rug_fuzz_0.to_string();
        let value_from_string = Value::from(test_string.clone());
        if let Value::String(value_str) = value_from_string {
            debug_assert_eq!(test_string, value_str);
        } else {
            panic!("Value::from did not convert to Value::String");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_220 {
    use crate::Value;
    use std::convert::From;
    #[test]
    fn test_from_vec_to_value_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let vec_of_integers = vec![
            Value::Integer(rug_fuzz_0), Value::Integer(2), Value::Integer(3)
        ];
        let value_from_vec = Value::from(vec_of_integers);
        if let Value::Array(array) = value_from_vec {
            debug_assert_eq!(array.len(), 3);
            debug_assert_eq!(array[rug_fuzz_1], Value::Integer(1));
            debug_assert_eq!(array[rug_fuzz_2], Value::Integer(2));
            debug_assert_eq!(array[rug_fuzz_3], Value::Integer(3));
        } else {
            panic!("Value::from did not produce a Value::Array");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_221 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use toml_datetime::Datetime;
    #[test]
    fn test_from_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_str = rug_fuzz_0;
        let datetime = datetime_str.parse::<Datetime>().unwrap();
        let value: Value = Value::from(datetime.clone());
        debug_assert!(value.is_datetime());
        debug_assert_eq!(value.as_datetime().unwrap(), & datetime);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_222 {
    use crate::Value;
    use std::convert::From;
    #[test]
    fn from_u32_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        let value = Value::from(num);
        debug_assert!(matches!(value, Value::Integer(42)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_223 {
    use super::*;
    use crate::*;
    #[test]
    fn test_value_from_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val: u8 = rug_fuzz_0;
        let value = Value::from(val);
        match value {
            Value::Integer(i) => debug_assert_eq!(i, 42i64),
            _ => panic!("Expected Value::Integer, found {:?}", value),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_226 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    use crate::Value;
    #[test]
    fn test_from_str_valid_toml() {
        let _rug_st_tests_llm_16_226_rrrruuuugggg_test_from_str_valid_toml = 0;
        let rug_fuzz_0 = r#"
            [package]
            name = "your_package"
            version = "0.1.0"
        "#;
        let rug_fuzz_1 = "package";
        let rug_fuzz_2 = "name";
        let rug_fuzz_3 = "version";
        let toml_str = rug_fuzz_0;
        let result = Value::from_str(toml_str);
        debug_assert!(result.is_ok());
        let value = result.unwrap();
        debug_assert!(value.is_table());
        let package = value.get(rug_fuzz_1).unwrap();
        debug_assert!(package.is_table());
        let name = package.get(rug_fuzz_2).unwrap();
        debug_assert_eq!(name.as_str(), Some("your_package"));
        let version = package.get(rug_fuzz_3).unwrap();
        debug_assert_eq!(version.as_str(), Some("0.1.0"));
        let _rug_ed_tests_llm_16_226_rrrruuuugggg_test_from_str_valid_toml = 0;
    }
    #[test]
    fn test_from_str_invalid_toml() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toml_str = rug_fuzz_0;
        let result = Value::from_str(toml_str);
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_from_str_empty_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toml_str = rug_fuzz_0;
        let result = Value::from_str(toml_str);
        debug_assert!(result.is_ok());
        let value = result.unwrap();
        debug_assert!(value.is_table());
        debug_assert!(value.as_table().unwrap().is_empty());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_227 {
    use crate::value::ValueSerializeMap;
    use crate::value::SerializeMap;
    use crate::value::Value;
    use crate::map::Map;
    use crate::ser::Error;
    use serde::ser::SerializeMap as _;
    #[test]
    fn test_value_serialize_map_end() {
        let _rug_st_tests_llm_16_227_rrrruuuugggg_test_value_serialize_map_end = 0;
        let m: Map<String, Value> = Map::new();
        let serialize_map = SerializeMap {
            map: m,
            next_key: None,
        };
        let value_serialize_map = ValueSerializeMap {
            ser: serialize_map,
        };
        debug_assert!(matches!(value_serialize_map.end(), Ok(Value::Table(_))));
        let _rug_ed_tests_llm_16_227_rrrruuuugggg_test_value_serialize_map_end = 0;
    }
    #[test]
    fn test_value_serialize_map_end_with_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serialize_map = SerializeMap {
            map: Map::new(),
            next_key: Some(rug_fuzz_0.to_string()),
        };
        let mut value_serialize_map = ValueSerializeMap {
            ser: serialize_map,
        };
        let value = rug_fuzz_1;
        let _ = value_serialize_map.serialize_value(&value);
        debug_assert!(matches!(value_serialize_map.end(), Err(Error { .. })));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_231_llm_16_231 {
    use crate::value::{Value, ValueSerializeMap, Table};
    use serde::{Serialize, ser::{SerializeStruct, Serializer}};
    use crate::ser::Error;
    #[derive(Serialize)]
    struct TestStruct {
        key: String,
    }
    impl TestStruct {
        fn new(key: &str) -> Self {
            TestStruct { key: key.to_owned() }
        }
    }
    struct NonSerializable;
    impl Serialize for NonSerializable {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom("NonSerializable cannot be serialized"))
        }
    }
    #[test]
    fn test_serialize_field() {
        let _rug_st_tests_llm_16_231_llm_16_231_rrrruuuugggg_test_serialize_field = 0;
        let rug_fuzz_0 = "value";
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "key";
        let value_map = Table::default();
        let mut value_serialize_map = ValueSerializeMap {
            ser: crate::value::SerializeMap {
                map: value_map,
                next_key: None,
            },
        };
        let test_struct = TestStruct::new(rug_fuzz_0);
        let result = value_serialize_map.serialize_field(rug_fuzz_1, &test_struct.key);
        debug_assert!(result.is_ok());
        debug_assert!(value_serialize_map.ser.map.contains_key(rug_fuzz_2));
        let _rug_ed_tests_llm_16_231_llm_16_231_rrrruuuugggg_test_serialize_field = 0;
    }
    #[test]
    fn test_serialize_field_error() {
        let _rug_st_tests_llm_16_231_llm_16_231_rrrruuuugggg_test_serialize_field_error = 0;
        let rug_fuzz_0 = "key";
        let value_map = Table::default();
        let mut value_serialize_map = ValueSerializeMap {
            ser: crate::value::SerializeMap {
                map: value_map,
                next_key: None,
            },
        };
        let non_serializable = NonSerializable {};
        let result = value_serialize_map.serialize_field(rug_fuzz_0, &non_serializable);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_231_llm_16_231_rrrruuuugggg_test_serialize_field_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_233_llm_16_233 {
    use serde::{Deserialize, Serialize};
    use crate::value::{Value, ValueSerializeVec};
    use crate::ser::Error as SerError;
    use serde::ser::SerializeSeq;
    use std::convert::TryFrom;
    #[derive(Serialize)]
    struct TestStruct {
        key: String,
        value: i32,
    }
    #[test]
    fn serialize_element_pushes_value() -> Result<(), SerError> {
        let mut value_vec = ValueSerializeVec {
            vec: Vec::new(),
        };
        let test_element = TestStruct {
            key: String::from("test_key"),
            value: 42,
        };
        value_vec.serialize_element(&test_element)?;
        assert_eq!(value_vec.vec.len(), 1);
        assert!(matches!(value_vec.vec[0], Value::Table(_)));
        if let Value::Table(table_map) = &value_vec.vec[0] {
            assert_eq!(
                table_map.get("key"), Some(& Value::String(String::from("test_key")))
            );
            assert_eq!(table_map.get("value"), Some(& Value::Integer(42)));
        } else {
            panic!("vec[0] is not a Table as expected");
        }
        Ok(())
    }
    #[test]
    fn serialize_element_returns_error_if_conversion_fails() {
        let mut value_vec = ValueSerializeVec {
            vec: Vec::new(),
        };
        let test_element = "non_serializable_element";
        let result = value_vec.serialize_element(&test_element);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_234 {
    use super::*;
    use crate::*;
    use serde::ser::SerializeTuple;
    use crate::value::ValueSerializeVec;
    use crate::value::Value;
    use crate::ser::Error;
    #[test]
    fn test_serialize_tuple_end() -> Result<(), Error> {
        let mut serializer = ValueSerializeVec {
            vec: vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)],
        };
        let result = serializer.end()?;
        let expected = Value::Array(
            vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)],
        );
        assert_eq!(result, expected);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_236 {
    use super::*;
    use crate::*;
    use serde::ser::SerializeTupleStruct;
    use crate::Value;
    #[test]
    fn test_value_serialize_vec_end() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, &str, i32, &str, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut serializer = ValueSerializeVec { vec: vec![] };
        serializer.serialize_field(&rug_fuzz_0).expect(rug_fuzz_1);
        serializer.serialize_field(&rug_fuzz_2).expect(rug_fuzz_3);
        let result = serializer.end().expect(rug_fuzz_4);
        let expected = Value::Array(vec![Value::Integer(rug_fuzz_5), Value::Integer(2)]);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_239_llm_16_239 {
    use crate::value::{Value, ValueSerializeVec};
    use serde::{Serialize, Serializer};
    use serde::ser::{SerializeSeq, SerializeTupleVariant};
    #[derive(Serialize)]
    struct TestStruct {
        key: String,
        value: i32,
    }
    #[test]
    fn serialize_field_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_value = TestStruct {
            key: rug_fuzz_0.to_owned(),
            value: rug_fuzz_1,
        };
        let mut serializer = ValueSerializeVec {
            vec: Vec::new(),
        };
        let result = SerializeTupleVariant::serialize_field(
            &mut serializer,
            &test_value,
        );
        debug_assert!(result.is_ok());
        let expected_value = Value::try_from(&test_value).expect(rug_fuzz_2);
        debug_assert_eq!(serializer.vec.first().unwrap(), & expected_value);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_240 {
    use super::*;
    use crate::*;
    use crate::value::ValueSerializer;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_bool_true() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let bool_value = rug_fuzz_0;
        let serialized = serializer.serialize_bool(bool_value).unwrap();
        debug_assert_eq!(serialized, Value::Boolean(true));
             }
});    }
    #[test]
    fn test_serialize_bool_false() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let bool_value = rug_fuzz_0;
        let serialized = serializer.serialize_bool(bool_value).unwrap();
        debug_assert_eq!(serialized, Value::Boolean(false));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_241 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::value::{Value, ValueSerializer};
    #[test]
    fn serialize_bytes_should_return_array_of_integers() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let expected = Value::Array(
            vec![
                Value::Integer(rug_fuzz_4), Value::Integer(2), Value::Integer(3),
                Value::Integer(255)
            ],
        );
        let serializer = ValueSerializer;
        let result = serializer.serialize_bytes(bytes).unwrap();
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_242 {
    use crate::Value;
    use crate::ser::Error;
    use crate::value::ValueSerializer;
    use serde::Serializer;
    #[test]
    fn serialize_char_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let char_to_serialize = rug_fuzz_0;
        let result = serializer.serialize_char(char_to_serialize);
        debug_assert!(result.is_ok());
        let value = result.unwrap();
        debug_assert!(matches!(value, Value::String(ref s) if s == "a"));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_243_llm_16_243 {
    use crate::value::{Value, ValueSerializer};
    use serde::Serializer;
    #[test]
    fn test_serialize_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let value = rug_fuzz_0;
        let serialized_value = serializer.serialize_f64(value.into()).unwrap();
        match serialized_value {
            Value::Float(float_value) => {
                let margin = rug_fuzz_1;
                debug_assert!(
                    (float_value - (value as f64)).abs() < margin,
                    "The serialized floating point value does not match the input value."
                );
            }
            _ => panic!("serialize_f32 did not return a Value::Float variant."),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_244_llm_16_244 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_values = vec![
            rug_fuzz_0, - 0.0, 1.0, - 1.0, std::f64::MIN, std::f64::MAX,
            std::f64::INFINITY, std::f64::NEG_INFINITY, std::f64::NAN
        ];
        for &test_val in &test_values {
            let value_serializer = ValueSerializer;
            let res = value_serializer.serialize_f64(test_val);
            match res {
                Ok(Value::Float(val)) => {
                    if test_val.is_nan() {
                        debug_assert!(val.is_nan());
                    } else {
                        debug_assert_eq!(val, test_val);
                    }
                }
                _ => panic!("Serialization failed for value {:?}", test_val),
            }
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_245_llm_16_245 {
    use crate::value::{Value, ValueSerializer};
    use serde::Serializer;
    #[test]
    fn test_serialize_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let result = serializer.serialize_i16(rug_fuzz_0).unwrap();
        debug_assert_eq!(result, Value::Integer(42));
        let serializer = ValueSerializer;
        let result = serializer.serialize_i16(-rug_fuzz_1).unwrap();
        debug_assert_eq!(result, Value::Integer(- 42));
        let serializer = ValueSerializer;
        let result = serializer.serialize_i16(i16::MAX).unwrap();
        debug_assert_eq!(result, Value::Integer(i16::MAX.into()));
        let serializer = ValueSerializer;
        let result = serializer.serialize_i16(i16::MIN).unwrap();
        debug_assert_eq!(result, Value::Integer(i16::MIN.into()));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_246_llm_16_246 {
    use crate::value::ValueSerializer;
    use crate::Value;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let value_i32: i32 = rug_fuzz_0;
        let expected = Value::Integer(value_i32 as i64);
        let result = Serializer::serialize_i32(serializer, value_i32);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_247 {
    use crate::value::{Value, ValueSerializer};
    use serde::Serializer;
    #[test]
    fn serialize_i64_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let i64_value: i64 = rug_fuzz_0;
        let expected = Value::Integer(i64_value);
        let result = serializer.serialize_i64(i64_value);
        debug_assert_eq!(result.unwrap(), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_249 {
    use super::*;
    use crate::*;
    use serde::ser::{SerializeMap, Serializer};
    struct MockSerializeMap {
        map: crate::value::Table,
        next_key: Option<String>,
    }
    impl SerializeMap for MockSerializeMap {
        type Ok = crate::Value;
        type Error = crate::ser::Error;
        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: serde::Serialize,
        {
            self
                .next_key = Some(
                key.serialize(crate::value::ValueSerializer)?.to_string(),
            );
            Ok(())
        }
        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::Serialize,
        {
            let value = value.serialize(crate::value::ValueSerializer)?;
            if let Some(key) = self.next_key.take() {
                self.map.insert(key, value);
            }
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(crate::Value::Table(self.map))
        }
    }
    struct ValueSerializeMap {
        ser: MockSerializeMap,
    }
    impl SerializeMap for ValueSerializeMap {
        type Ok = crate::Value;
        type Error = crate::ser::Error;
        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: serde::Serialize,
        {
            self.ser.serialize_key(key)
        }
        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::Serialize,
        {
            self.ser.serialize_value(value)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.ser.end()
        }
    }
    #[test]
    fn serialize_map_creates_empty_table() {
        let _rug_st_tests_llm_16_249_rrrruuuugggg_serialize_map_creates_empty_table = 0;
        let value_serializer = crate::value::ValueSerializer;
        let serialize_map_result = value_serializer.serialize_map(None);
        debug_assert!(serialize_map_result.is_ok(), "serialize_map should return Ok");
        let value_serialize_map = serialize_map_result.unwrap();
        debug_assert_eq!(
            value_serialize_map.ser.map.len(), 0, "initial table should be empty"
        );
        let _rug_ed_tests_llm_16_249_rrrruuuugggg_serialize_map_creates_empty_table = 0;
    }
    #[test]
    fn serialize_map_creates_valid_map_serializer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_serializer = crate::value::ValueSerializer;
        let mut serialize_map_result = value_serializer.serialize_map(None).unwrap();
        let mut map = serialize_map_result
            .serialize_key(rug_fuzz_0)
            .and_then(|_| serialize_map_result.serialize_value(&rug_fuzz_1));
        debug_assert!(map.is_ok(), "serialize_key and serialize_value should succeed");
        let map = serialize_map_result.end().unwrap();
        if let crate::Value::Table(table) = map {
            debug_assert_eq!(
                table[rug_fuzz_2], crate ::Value::Integer(42),
                "map should contain the key-value pair"
            );
        } else {
            panic!("serialize_map should return a table");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_253 {
    use super::*;
    use crate::*;
    use serde::ser::{Serialize, Serializer};
    #[test]
    fn test_serialize_seq_none() {
        let _rug_st_tests_llm_16_253_rrrruuuugggg_test_serialize_seq_none = 0;
        let serializer = ValueSerializer;
        let result = serializer.serialize_seq(None);
        debug_assert!(result.is_ok());
        let seq = result.unwrap();
        debug_assert_eq!(seq.vec.capacity(), 0);
        let _rug_ed_tests_llm_16_253_rrrruuuugggg_test_serialize_seq_none = 0;
    }
    #[test]
    fn test_serialize_seq_some() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let len = rug_fuzz_0;
        let serializer = ValueSerializer;
        let result = serializer.serialize_seq(Some(len));
        debug_assert!(result.is_ok());
        let seq = result.unwrap();
        debug_assert_eq!(seq.vec.capacity(), len);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_255 {
    use crate::value::{ValueSerializer, Value};
    use serde::Serializer;
    use crate::ser::Error;
    #[test]
    fn serialize_str_test() -> Result<(), Error> {
        let serializer = ValueSerializer;
        let test_str = "hello world";
        let expected = Value::String(test_str.to_owned());
        let result = serializer.serialize_str(test_str)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_256_llm_16_256 {
    use super::*;
    use crate::*;
    use crate::value::{ValueSerializer, Value};
    use serde::Serializer;
    use crate::ser::Error;
    #[test]
    fn test_serialize_struct() {
        let _rug_st_tests_llm_16_256_llm_16_256_rrrruuuugggg_test_serialize_struct = 0;
        let rug_fuzz_0 = "TestStruct";
        let rug_fuzz_1 = 1;
        let serializer = ValueSerializer;
        let result = serializer.serialize_struct(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(result.is_ok());
        let result_value_serialize_map = result.unwrap();
        let result_value = Value::Table(result_value_serialize_map.ser.map);
        match result_value {
            Value::Table(ref table) => debug_assert_eq!(table.len(), 1),
            _ => panic!("Expected Value::Table variant"),
        }
        let _rug_ed_tests_llm_16_256_llm_16_256_rrrruuuugggg_test_serialize_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_257 {
    use serde::ser::Serializer;
    use crate::value::ValueSerializer;
    use crate::Value;
    #[test]
    fn test_serialize_struct_variant_unsupported() {
        let _rug_st_tests_llm_16_257_rrrruuuugggg_test_serialize_struct_variant_unsupported = 0;
        let rug_fuzz_0 = "Example";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "variant";
        let rug_fuzz_3 = 0;
        let serializer = ValueSerializer;
        let result = serializer
            .serialize_struct_variant(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(result.is_err());
        match result {
            Err(e) => debug_assert_eq!(e.to_string(), "unsupported type: Example"),
            _ => panic!("Expected error for unsupported type"),
        }
        let _rug_ed_tests_llm_16_257_rrrruuuugggg_test_serialize_struct_variant_unsupported = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_258 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::Value;
    #[test]
    fn test_serialize_tuple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let len = rug_fuzz_0;
        let result = serializer.serialize_tuple(len);
        debug_assert!(result.is_ok());
        let value_serialize_vec = result.unwrap();
        debug_assert_eq!(value_serialize_vec.vec.capacity(), len);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_259_llm_16_259 {
    use crate::value::{Value, ValueSerializer, ValueSerializeVec};
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_tuple_struct() {
        let _rug_st_tests_llm_16_259_llm_16_259_rrrruuuugggg_test_serialize_tuple_struct = 0;
        let rug_fuzz_0 = "MyTupleStruct";
        let rug_fuzz_1 = 3;
        let serializer = ValueSerializer;
        let name = rug_fuzz_0;
        let len = rug_fuzz_1;
        let result = serializer.serialize_tuple_struct(name, len);
        debug_assert!(result.is_ok());
        if let Ok(ValueSerializeVec { vec }) = result {
            debug_assert_eq!(vec.len(), len);
        } else {
            panic!("Expected a ValueSerializeVec");
        }
        let _rug_ed_tests_llm_16_259_llm_16_259_rrrruuuugggg_test_serialize_tuple_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_260 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_tuple_variant() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_serialize_tuple_variant = 0;
        let rug_fuzz_0 = "Variant";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "variant_value";
        let rug_fuzz_3 = 3;
        let serializer = ValueSerializer;
        let variant_name = rug_fuzz_0;
        let variant_index = rug_fuzz_1;
        let variant_value = rug_fuzz_2;
        let len = rug_fuzz_3;
        match serializer
            .serialize_tuple_variant(variant_name, variant_index, variant_value, len)
        {
            Ok(value_serialize_vec) => {
                debug_assert_eq!(value_serialize_vec.vec.capacity(), len);
            }
            Err(e) => panic!("Failed to serialize tuple variant: {}", e),
        }
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_serialize_tuple_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_261 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::value::{Value, ValueSerializer};
    use crate::ser::Error;
    #[test]
    fn test_serialize_u16() -> Result<(), Error> {
        let serializer = ValueSerializer;
        let value_u16: u16 = 42;
        let toml_value = serializer.serialize_u16(value_u16)?;
        assert_eq!(toml_value, Value::Integer(value_u16 as i64));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_262 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::ser::Error;
    use crate::value::Value;
    #[test]
    fn test_serialize_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let serializer = ValueSerializer;
        let value: u32 = rug_fuzz_0;
        let serialized_value = serializer.serialize_u32(value).unwrap();
        if let Value::Integer(i) = serialized_value {
            debug_assert_eq!(i, value as i64);
        } else {
            panic!("Value was not serialized as Value::Integer");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_264_llm_16_264 {
    use crate::value::{Value, ValueSerializer};
    use crate::ser::Error;
    use serde::Serializer;
    #[test]
    fn test_serialize_u8() -> Result<(), Error> {
        let value = 123u8;
        let serializer = ValueSerializer;
        let serialized_value = Serializer::serialize_u8(serializer, value)?;
        assert_eq!(serialized_value, Value::Integer(value as i64));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_267 {
    use serde::Serializer;
    use crate::value::{Value, ValueSerializer};
    use crate::ser::Error;
    #[test]
    fn test_serialize_unit_variant() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_serialize_unit_variant = 0;
        let rug_fuzz_0 = "VariantName";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "Variant";
        let serializer = ValueSerializer;
        let variant_name = rug_fuzz_0;
        let variant_index = rug_fuzz_1;
        let variant = rug_fuzz_2;
        let result = serializer
            .serialize_unit_variant(variant_name, variant_index, variant);
        match result {
            Ok(Value::String(s)) => debug_assert_eq!(s, "Variant"),
            Ok(_) => panic!("serialize_unit_variant did not return a Value::String"),
            Err(e) => panic!("serialize_unit_variant returned an error: {}", e),
        }
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_serialize_unit_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_328 {
    use super::*;
    use crate::*;
    use crate::value::Table;
    #[test]
    fn test_map_deserializer_new() {
        let _rug_st_tests_llm_16_328_rrrruuuugggg_test_map_deserializer_new = 0;
        let map = Table::new();
        let map_deserializer = MapDeserializer::new(map.clone());
        debug_assert_eq!(map_deserializer.iter.count(), map.into_iter().count());
        let _rug_ed_tests_llm_16_328_rrrruuuugggg_test_map_deserializer_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_332 {
    use super::*;
    use crate::*;
    #[test]
    fn as_array_mut_with_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Array(
            vec![
                Value::Integer(rug_fuzz_0), Value::String("two".to_string()),
                Value::Boolean(true)
            ],
        );
        let array = value.as_array_mut().unwrap();
        debug_assert_eq!(array.len(), 3);
        debug_assert_eq!(array[rug_fuzz_1], Value::Integer(1));
        debug_assert_eq!(array[rug_fuzz_2], Value::String("two".to_string()));
        debug_assert_eq!(array[rug_fuzz_3], Value::Boolean(true));
             }
});    }
    #[test]
    fn as_array_mut_with_non_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::String(rug_fuzz_0.to_string());
        debug_assert!(value.as_array_mut().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_333_llm_16_333 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn as_bool_from_boolean_true() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Boolean(rug_fuzz_0);
        debug_assert_eq!(value.as_bool(), Some(true));
             }
});    }
    #[test]
    fn as_bool_from_boolean_false() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Boolean(rug_fuzz_0);
        debug_assert_eq!(value.as_bool(), Some(false));
             }
});    }
    #[test]
    fn as_bool_from_non_boolean_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_string());
        debug_assert_eq!(value.as_bool(), None);
             }
});    }
    #[test]
    fn as_bool_from_non_boolean_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Integer(rug_fuzz_0);
        debug_assert_eq!(value.as_bool(), None);
             }
});    }
    #[test]
    fn as_bool_from_non_boolean_float() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Float(rug_fuzz_0);
        debug_assert_eq!(value.as_bool(), None);
             }
});    }
    #[test]
    fn as_bool_from_non_boolean_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![Value::Boolean(rug_fuzz_0)]);
        debug_assert_eq!(value.as_bool(), None);
             }
});    }
    #[test]
    fn as_bool_from_non_boolean_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Map::new();
        table.insert(rug_fuzz_0.to_string(), Value::Boolean(rug_fuzz_1));
        let value = Value::Table(table);
        debug_assert_eq!(value.as_bool(), None);
             }
});    }
    #[test]
    fn as_bool_from_non_boolean_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::value::Datetime;
        let value = Value::Datetime(Datetime::from_str(rug_fuzz_0).unwrap());
        debug_assert_eq!(value.as_bool(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_334_llm_16_334 {
    use crate::value::{Datetime, Value};
    use std::str::FromStr;
    #[test]
    fn test_as_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_str = rug_fuzz_0;
        let datetime = Datetime::from_str(datetime_str).unwrap();
        let value = Value::Datetime(datetime.clone());
        debug_assert_eq!(value.as_datetime(), Some(& datetime));
             }
});    }
    #[test]
    fn test_as_datetime_fail() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let integer_value = Value::Integer(rug_fuzz_0);
        debug_assert_eq!(integer_value.as_datetime(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_335 {
    use crate::value::Value;
    #[test]
    fn as_float_from_float_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let float_value = Value::Float(rug_fuzz_0);
        debug_assert_eq!(float_value.as_float(), Some(42.0));
             }
});    }
    #[test]
    fn as_float_from_non_float_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, &str, bool, i64, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let integer_value = Value::Integer(rug_fuzz_0);
        debug_assert_eq!(integer_value.as_float(), None);
        let string_value = Value::String(rug_fuzz_1.to_owned());
        debug_assert_eq!(string_value.as_float(), None);
        let boolean_value = Value::Boolean(rug_fuzz_2);
        debug_assert_eq!(boolean_value.as_float(), None);
        let array_value = Value::Array(
            vec![Value::Integer(rug_fuzz_3), Value::Boolean(false)],
        );
        debug_assert_eq!(array_value.as_float(), None);
        let mut table = crate::map::Map::new();
        table.insert(rug_fuzz_4.to_owned(), Value::Integer(rug_fuzz_5));
        let table_value = Value::Table(table);
        debug_assert_eq!(table_value.as_float(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_336 {
    use crate::value::Value;
    #[test]
    fn as_integer_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let integer_value = Value::Integer(rug_fuzz_0);
        debug_assert_eq!(integer_value.as_integer(), Some(42));
             }
});    }
    #[test]
    fn as_integer_not_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_integer_values = vec![
            Value::String(rug_fuzz_0.to_owned()), Value::Float(3.14),
            Value::Boolean(true), Value::Datetime("2021-04-04T21:00:00Z".parse()
            .unwrap()), Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Table(crate ::value::Table::new())
        ];
        for non_integer_value in non_integer_values {
            debug_assert_eq!(non_integer_value.as_integer(), None);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_337 {
    use crate::value::Value;
    #[test]
    fn test_as_str_with_string_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = Value::String(String::from(rug_fuzz_0));
        debug_assert_eq!(val.as_str(), Some("test string"));
             }
});    }
    #[test]
    fn test_as_str_with_non_string_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = Value::Integer(rug_fuzz_0);
        debug_assert_eq!(val.as_str(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_338 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_as_table_some() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Map::new();
        table.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        let value = Value::Table(table);
        debug_assert!(value.as_table().is_some());
             }
});    }
    #[test]
    fn test_as_table_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        debug_assert!(value.as_table().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_339_llm_16_339 {
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_as_table_mut_some() {
        let _rug_st_tests_llm_16_339_llm_16_339_rrrruuuugggg_test_as_table_mut_some = 0;
        let mut val = Value::Table(Map::new());
        debug_assert!(val.as_table_mut().is_some());
        let _rug_ed_tests_llm_16_339_llm_16_339_rrrruuuugggg_test_as_table_mut_some = 0;
    }
    #[test]
    fn test_as_table_mut_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut val = Value::String(rug_fuzz_0.to_string());
        debug_assert!(val.as_table_mut().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_340_llm_16_340 {
    use super::*;
    use crate::*;
    use crate::value::{Value, Table as Map};
    #[test]
    fn test_get_from_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, i64, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        let value = Value::Table(map);
        debug_assert_eq!(
            value.get(rug_fuzz_4), Some(& Value::String("value".to_string()))
        );
        debug_assert_eq!(value.get(rug_fuzz_5), Some(& Value::Integer(42)));
        debug_assert_eq!(value.get(rug_fuzz_6), None);
             }
});    }
    #[test]
    fn test_get_from_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = vec![Value::String(rug_fuzz_0.to_string()), Value::Integer(42)];
        let value = Value::Array(array);
        debug_assert_eq!(
            value.get(rug_fuzz_1), Some(& Value::String("value".to_string()))
        );
        debug_assert_eq!(value.get(rug_fuzz_2), Some(& Value::Integer(42)));
        debug_assert_eq!(value.get(rug_fuzz_3), None);
             }
});    }
    #[test]
    fn test_get_from_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Integer(rug_fuzz_0);
        debug_assert_eq!(value.get(rug_fuzz_1), None);
             }
});    }
    #[test]
    fn test_get_from_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_string());
        debug_assert_eq!(value.get(rug_fuzz_1), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_341_llm_16_341 {
    use crate::Value;
    use crate::map::Map;
    #[test]
    fn test_get_mut_for_map() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Table(Map::new());
        let key = rug_fuzz_0.to_string();
        let test_value = Value::String(rug_fuzz_1.to_string());
        value.as_table_mut().unwrap().insert(key.clone(), test_value.clone());
        let result = value.get_mut(key.as_str()).unwrap();
        debug_assert_eq!(result, & test_value);
        *result = Value::String(rug_fuzz_2.to_string());
        debug_assert_eq!(
            value.get(key.as_str()).unwrap(), & Value::String("new_value".to_string())
        );
             }
});    }
    #[test]
    fn test_get_mut_for_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, usize, i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Array(
            vec![
                Value::String(rug_fuzz_0.to_string()), Value::String("value1"
                .to_string())
            ],
        );
        let result = value.get_mut(rug_fuzz_1).unwrap();
        debug_assert_eq!(result, & Value::String("value1".to_string()));
        *result = Value::Integer(rug_fuzz_2);
        debug_assert_eq!(value.get(rug_fuzz_3).unwrap(), & Value::Integer(123));
             }
});    }
    #[test]
    fn test_get_mut_key_not_exist() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Table(Map::new());
        debug_assert!(value.get_mut(rug_fuzz_0).is_none());
             }
});    }
    #[test]
    fn test_get_mut_index_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Array(Vec::new());
        debug_assert!(value.get_mut(rug_fuzz_0).is_none());
             }
});    }
    #[test]
    fn test_get_mut_wrong_type_map() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Table(Map::new());
        debug_assert!(value.get_mut(rug_fuzz_0).is_none());
             }
});    }
    #[test]
    fn test_get_mut_wrong_type_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Array(Vec::new());
        debug_assert!(value.get_mut(rug_fuzz_0).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_342 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_is_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, &str, i64, f64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array_value = Value::Array(
            vec![Value::Integer(rug_fuzz_0), Value::Integer(2), Value::Integer(3)],
        );
        let string_value = Value::String(rug_fuzz_1.to_string());
        let integer_value = Value::Integer(rug_fuzz_2);
        let float_value = Value::Float(rug_fuzz_3);
        let boolean_value = Value::Boolean(rug_fuzz_4);
        let table_value = Value::Table(Map::new());
        debug_assert!(array_value.is_array());
        debug_assert!(! string_value.is_array());
        debug_assert!(! integer_value.is_array());
        debug_assert!(! float_value.is_array());
        debug_assert!(! boolean_value.is_array());
        debug_assert!(! table_value.is_array());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_343 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(bool, bool, &str, i64, f64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_bool_true = Value::Boolean(rug_fuzz_0);
        debug_assert!(value_bool_true.is_bool());
        let value_bool_false = Value::Boolean(rug_fuzz_1);
        debug_assert!(value_bool_false.is_bool());
        let value_not_bool_string = Value::String(String::from(rug_fuzz_2));
        debug_assert!(! value_not_bool_string.is_bool());
        let value_not_bool_integer = Value::Integer(rug_fuzz_3);
        debug_assert!(! value_not_bool_integer.is_bool());
        let value_not_bool_float = Value::Float(rug_fuzz_4);
        debug_assert!(! value_not_bool_float.is_bool());
        let value_not_bool_array = Value::Array(
            vec![Value::Boolean(rug_fuzz_5), Value::Boolean(false)],
        );
        debug_assert!(! value_not_bool_array.is_bool());
        let value_not_bool_table = Value::Table(map::Map::new());
        debug_assert!(! value_not_bool_table.is_bool());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_345 {
    use crate::value::Value;
    #[test]
    fn test_is_float() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v_float = Value::Float(rug_fuzz_0);
        let v_integer = Value::Integer(rug_fuzz_1);
        let v_string = Value::String(rug_fuzz_2.to_string());
        debug_assert!(v_float.is_float());
        debug_assert!(! v_integer.is_float());
        debug_assert!(! v_string.is_float());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_346 {
    use crate::value::Value;
    #[test]
    fn test_is_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i64, &str, f64, bool, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Value::Integer(rug_fuzz_0).is_integer(), true);
        debug_assert_eq!(Value::Integer(- rug_fuzz_1).is_integer(), true);
        debug_assert_eq!(Value::String(rug_fuzz_2.to_owned()).is_integer(), false);
        debug_assert_eq!(Value::Float(rug_fuzz_3).is_integer(), false);
        debug_assert_eq!(Value::Boolean(rug_fuzz_4).is_integer(), false);
        debug_assert_eq!(
            Value::Array(vec![Value::Integer(rug_fuzz_5)]).is_integer(), false
        );
        debug_assert_eq!(
            Value::Datetime(rug_fuzz_6.parse().unwrap()).is_integer(), false
        );
        debug_assert_eq!(Value::Table(rug_fuzz_7.parse().unwrap()).is_integer(), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_347_llm_16_347 {
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_is_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, f64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Value::String(rug_fuzz_0.to_owned()).is_str());
        debug_assert!(! Value::Integer(rug_fuzz_1).is_str());
        debug_assert!(! Value::Float(rug_fuzz_2).is_str());
        debug_assert!(! Value::Boolean(rug_fuzz_3).is_str());
        debug_assert!(! Value::Array(vec![]).is_str());
        debug_assert!(! Value::Table(Map::new()).is_str());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_348_llm_16_348 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_is_table_with_table() {
        let _rug_st_tests_llm_16_348_llm_16_348_rrrruuuugggg_test_is_table_with_table = 0;
        let table = Value::Table(Map::new());
        debug_assert_eq!(table.is_table(), true);
        let _rug_ed_tests_llm_16_348_llm_16_348_rrrruuuugggg_test_is_table_with_table = 0;
    }
    #[test]
    fn test_is_table_with_non_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, f64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string = Value::String(rug_fuzz_0.to_string());
        debug_assert_eq!(string.is_table(), false);
        let integer = Value::Integer(rug_fuzz_1);
        debug_assert_eq!(integer.is_table(), false);
        let float = Value::Float(rug_fuzz_2);
        debug_assert_eq!(float.is_table(), false);
        let boolean = Value::Boolean(rug_fuzz_3);
        debug_assert_eq!(boolean.is_table(), false);
        let array = Value::Array(Vec::new());
        debug_assert_eq!(array.is_table(), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_349 {
    use crate::value::Value;
    #[test]
    fn test_same_type() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = Value::String(String::from(rug_fuzz_0));
        let same_string_value = Value::String(String::from(rug_fuzz_1));
        let integer_value = Value::Integer(rug_fuzz_2);
        debug_assert!(string_value.same_type(& same_string_value));
        debug_assert!(! string_value.same_type(& integer_value));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_350 {
    use super::*;
    use crate::*;
    use serde::Serialize;
    use std::collections::BTreeMap;
    #[derive(Serialize)]
    struct TestStruct {
        key: String,
        value: i32,
    }
    #[test]
    fn try_from_struct_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_struct = TestStruct {
            key: rug_fuzz_0.to_owned(),
            value: rug_fuzz_1,
        };
        let result = Value::try_from(test_struct);
        debug_assert!(result.is_ok());
        if let Ok(Value::Table(table)) = result {
            debug_assert!(table.contains_key(rug_fuzz_2));
            debug_assert_eq!(
                table.get(rug_fuzz_3).unwrap(), & Value::String("example".to_owned())
            );
            debug_assert!(table.contains_key(rug_fuzz_4));
            debug_assert_eq!(table.get(rug_fuzz_5).unwrap(), & Value::Integer(42));
        } else {
            panic!("Expected Value::Table");
        }
             }
});    }
    #[test]
    fn try_from_map_to_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, i32, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut test_map = BTreeMap::new();
        test_map.insert(rug_fuzz_0.to_owned(), rug_fuzz_1.to_owned());
        test_map.insert(rug_fuzz_2.to_owned(), rug_fuzz_3.to_string());
        let result = Value::try_from(test_map);
        debug_assert!(result.is_ok());
        if let Ok(Value::Table(table)) = result {
            debug_assert!(table.contains_key(rug_fuzz_4));
            debug_assert_eq!(
                table.get(rug_fuzz_5).unwrap(), & Value::String("example".to_owned())
            );
            debug_assert!(table.contains_key(rug_fuzz_6));
            debug_assert_eq!(
                table.get(rug_fuzz_7).unwrap(), & Value::String("42".to_owned())
            );
        } else {
            panic!("Expected Value::Table");
        }
             }
});    }
}
