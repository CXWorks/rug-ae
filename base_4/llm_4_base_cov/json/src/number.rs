use crate::de::ParserNumber;
use crate::error::Error;
#[cfg(feature = "arbitrary_precision")]
use crate::error::ErrorCode;
#[cfg(feature = "arbitrary_precision")]
use alloc::borrow::ToOwned;
#[cfg(feature = "arbitrary_precision")]
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
#[cfg(not(feature = "arbitrary_precision"))]
use core::hash::{Hash, Hasher};
use serde::de::{self, Unexpected, Visitor};
#[cfg(feature = "arbitrary_precision")]
use serde::de::{IntoDeserializer, MapAccess};
use serde::{
    forward_to_deserialize_any, Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "arbitrary_precision")]
pub(crate) const TOKEN: &str = "$serde_json::private::Number";
/// Represents a JSON number, whether integer or floating point.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}
#[cfg(not(feature = "arbitrary_precision"))]
impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}
#[cfg(not(feature = "arbitrary_precision"))]
impl Eq for N {}
#[cfg(not(feature = "arbitrary_precision"))]
impl Hash for N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}
#[cfg(feature = "arbitrary_precision")]
type N = String;
impl Number {
    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// ```
    #[inline]
    pub fn is_i64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(v) => v <= i64::max_value() as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")] self.as_i64().is_some()
    }
    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// ```
    #[inline]
    pub fn is_u64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(_) => true,
            N::NegInt(_) | N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")] self.as_u64().is_some()
    }
    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// ```
    #[inline]
    pub fn is_f64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::Float(_) => true,
            N::PosInt(_) | N::NegInt(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        {
            for c in self.n.chars() {
                if c == '.' || c == 'e' || c == 'E' {
                    return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                }
            }
            false
        }
    }
    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// ```
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => {
                if n <= i64::max_value() as u64 { Some(n as i64) } else { None }
            }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")] self.n.parse().ok()
    }
    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None);
    /// assert_eq!(v["c"].as_u64(), None);
    /// ```
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n),
            N::NegInt(_) | N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")] self.n.parse().ok()
    }
    /// Represents the number as f64 if possible. Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// ```
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f64>().ok().filter(|float| float.is_finite())
    }
    /// Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
    /// numbers.
    ///
    /// ```
    /// # use std::f64;
    /// #
    /// # use serde_json::Number;
    /// #
    /// assert!(Number::from_f64(256.0).is_some());
    ///
    /// assert!(Number::from_f64(f64::NAN).is_none());
    /// ```
    #[inline]
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            let n = {
                #[cfg(not(feature = "arbitrary_precision"))] { N::Float(f) }
                #[cfg(feature = "arbitrary_precision")]
                { ryu::Buffer::new().format_finite(f).to_owned() }
            };
            Some(Number { n })
        } else {
            None
        }
    }
    pub(crate) fn as_f32(&self) -> Option<f32> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f32),
            N::NegInt(n) => Some(n as f32),
            N::Float(n) => Some(n as f32),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f32>().ok().filter(|float| float.is_finite())
    }
    pub(crate) fn from_f32(f: f32) -> Option<Number> {
        if f.is_finite() {
            let n = {
                #[cfg(not(feature = "arbitrary_precision"))] { N::Float(f as f64) }
                #[cfg(feature = "arbitrary_precision")]
                { ryu::Buffer::new().format_finite(f).to_owned() }
            };
            Some(Number { n })
        } else {
            None
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    /// Not public API. Only tests use this.
    #[doc(hidden)]
    #[inline]
    pub fn from_string_unchecked(n: String) -> Self {
        Number { n }
    }
}
impl Display for Number {
    #[cfg(not(feature = "arbitrary_precision"))]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
            N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
            N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.n, formatter)
    }
}
impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Number({})", self)
    }
}
impl Serialize for Number {
    #[cfg(not(feature = "arbitrary_precision"))]
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(u) => serializer.serialize_u64(u),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct(TOKEN, 1)?;
        s.serialize_field(TOKEN, &self.n)?;
        s.end()
    }
}
impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;
        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON number")
            }
            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }
            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }
            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_f64(value)
                    .ok_or_else(|| de::Error::custom("not a JSON number"))
            }
            #[cfg(feature = "arbitrary_precision")]
            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Number, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let value = visitor.next_key::<NumberKey>()?;
                if value.is_none() {
                    return Err(de::Error::invalid_type(Unexpected::Map, &self));
                }
                let v: NumberFromString = visitor.next_value()?;
                Ok(v.value)
            }
        }
        deserializer.deserialize_any(NumberVisitor)
    }
}
#[cfg(feature = "arbitrary_precision")]
struct NumberKey;
#[cfg(feature = "arbitrary_precision")]
impl<'de> de::Deserialize<'de> for NumberKey {
    fn deserialize<D>(deserializer: D) -> Result<NumberKey, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct FieldVisitor;
        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ();
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid number field")
            }
            fn visit_str<E>(self, s: &str) -> Result<(), E>
            where
                E: de::Error,
            {
                if s == TOKEN {
                    Ok(())
                } else {
                    Err(de::Error::custom("expected field with custom name"))
                }
            }
        }
        deserializer.deserialize_identifier(FieldVisitor)?;
        Ok(NumberKey)
    }
}
#[cfg(feature = "arbitrary_precision")]
pub struct NumberFromString {
    pub value: Number,
}
#[cfg(feature = "arbitrary_precision")]
impl<'de> de::Deserialize<'de> for NumberFromString {
    fn deserialize<D>(deserializer: D) -> Result<NumberFromString, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = NumberFromString;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string containing a number")
            }
            fn visit_str<E>(self, s: &str) -> Result<NumberFromString, E>
            where
                E: de::Error,
            {
                let n = tri!(s.parse().map_err(de::Error::custom));
                Ok(NumberFromString { value: n })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
    Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}
macro_rules! deserialize_any {
    (@ expand[$($num_string:tt)*]) => {
        #[cfg(not(feature = "arbitrary_precision"))] #[inline] fn deserialize_any < V >
        (self, visitor : V) -> Result < V::Value, Error > where V : Visitor <'de >, {
        match self.n { N::PosInt(u) => visitor.visit_u64(u), N::NegInt(i) => visitor
        .visit_i64(i), N::Float(f) => visitor.visit_f64(f), } } #[cfg(feature =
        "arbitrary_precision")] #[inline] fn deserialize_any < V > (self, visitor : V) ->
        Result < V::Value, Error > where V : Visitor <'de > { if let Some(u) = self
        .as_u64() { return visitor.visit_u64(u); } else if let Some(i) = self.as_i64() {
        return visitor.visit_i64(i); } else if let Some(f) = self.as_f64() { if
        ryu::Buffer::new().format_finite(f) == self.n || f.to_string() == self.n { return
        visitor.visit_f64(f); } } visitor.visit_map(NumberDeserializer { number :
        Some(self.$($num_string)*), }) }
    };
    (owned) => {
        deserialize_any!(@ expand[n]);
    };
    (ref) => {
        deserialize_any!(@ expand[n.clone()]);
    };
}
macro_rules! deserialize_number {
    ($deserialize:ident => $visit:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn $deserialize < V > (self, visitor
        : V) -> Result < V::Value, Error > where V : Visitor <'de >, { self
        .deserialize_any(visitor) } #[cfg(feature = "arbitrary_precision")] fn
        $deserialize < V > (self, visitor : V) -> Result < V::Value, Error > where V :
        de::Visitor <'de >, { visitor.$visit (self.n.parse().map_err(| _ |
        invalid_number()) ?) }
    };
}
impl<'de> Deserializer<'de> for Number {
    type Error = Error;
    deserialize_any!(owned);
    deserialize_number!(deserialize_i8 => visit_i8);
    deserialize_number!(deserialize_i16 => visit_i16);
    deserialize_number!(deserialize_i32 => visit_i32);
    deserialize_number!(deserialize_i64 => visit_i64);
    deserialize_number!(deserialize_i128 => visit_i128);
    deserialize_number!(deserialize_u8 => visit_u8);
    deserialize_number!(deserialize_u16 => visit_u16);
    deserialize_number!(deserialize_u32 => visit_u32);
    deserialize_number!(deserialize_u64 => visit_u64);
    deserialize_number!(deserialize_u128 => visit_u128);
    deserialize_number!(deserialize_f32 => visit_f32);
    deserialize_number!(deserialize_f64 => visit_f64);
    forward_to_deserialize_any! {
        bool char str string bytes byte_buf option unit unit_struct newtype_struct seq
        tuple tuple_struct map struct enum identifier ignored_any
    }
}
impl<'de, 'a> Deserializer<'de> for &'a Number {
    type Error = Error;
    deserialize_any!(ref);
    deserialize_number!(deserialize_i8 => visit_i8);
    deserialize_number!(deserialize_i16 => visit_i16);
    deserialize_number!(deserialize_i32 => visit_i32);
    deserialize_number!(deserialize_i64 => visit_i64);
    deserialize_number!(deserialize_i128 => visit_i128);
    deserialize_number!(deserialize_u8 => visit_u8);
    deserialize_number!(deserialize_u16 => visit_u16);
    deserialize_number!(deserialize_u32 => visit_u32);
    deserialize_number!(deserialize_u64 => visit_u64);
    deserialize_number!(deserialize_u128 => visit_u128);
    deserialize_number!(deserialize_f32 => visit_f32);
    deserialize_number!(deserialize_f64 => visit_f64);
    forward_to_deserialize_any! {
        bool char str string bytes byte_buf option unit unit_struct newtype_struct seq
        tuple tuple_struct map struct enum identifier ignored_any
    }
}
#[cfg(feature = "arbitrary_precision")]
pub(crate) struct NumberDeserializer {
    pub number: Option<String>,
}
#[cfg(feature = "arbitrary_precision")]
impl<'de> MapAccess<'de> for NumberDeserializer {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.number.is_none() {
            return Ok(None);
        }
        seed.deserialize(NumberFieldDeserializer).map(Some)
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.number.take().unwrap().into_deserializer())
    }
}
#[cfg(feature = "arbitrary_precision")]
struct NumberFieldDeserializer;
#[cfg(feature = "arbitrary_precision")]
impl<'de> Deserializer<'de> for NumberFieldDeserializer {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(TOKEN)
    }
    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string seq bytes
        byte_buf map struct option unit newtype_struct ignored_any unit_struct
        tuple_struct tuple enum identifier
    }
}
impl From<ParserNumber> for Number {
    fn from(value: ParserNumber) -> Self {
        let n = match value {
            ParserNumber::F64(f) => {
                #[cfg(not(feature = "arbitrary_precision"))] { N::Float(f) }
                #[cfg(feature = "arbitrary_precision")] { f.to_string() }
            }
            ParserNumber::U64(u) => {
                #[cfg(not(feature = "arbitrary_precision"))] { N::PosInt(u) }
                #[cfg(feature = "arbitrary_precision")] { u.to_string() }
            }
            ParserNumber::I64(i) => {
                #[cfg(not(feature = "arbitrary_precision"))] { N::NegInt(i) }
                #[cfg(feature = "arbitrary_precision")] { i.to_string() }
            }
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(s) => s,
        };
        Number { n }
    }
}
macro_rules! impl_from_unsigned {
    ($($ty:ty),*) => {
        $(impl From <$ty > for Number { #[inline] fn from(u : $ty) -> Self { let n = {
        #[cfg(not(feature = "arbitrary_precision"))] { N::PosInt(u as u64) }
        #[cfg(feature = "arbitrary_precision")] { itoa::Buffer::new().format(u)
        .to_owned() } }; Number { n } } })*
    };
}
macro_rules! impl_from_signed {
    ($($ty:ty),*) => {
        $(impl From <$ty > for Number { #[inline] fn from(i : $ty) -> Self { let n = {
        #[cfg(not(feature = "arbitrary_precision"))] { if i < 0 { N::NegInt(i as i64) }
        else { N::PosInt(i as u64) } } #[cfg(feature = "arbitrary_precision")] {
        itoa::Buffer::new().format(i).to_owned() } }; Number { n } } })*
    };
}
impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);
#[cfg(feature = "arbitrary_precision")]
impl_from_unsigned!(u128);
#[cfg(feature = "arbitrary_precision")]
impl_from_signed!(i128);
impl Number {
    #[cfg(not(feature = "arbitrary_precision"))]
    #[cold]
    pub(crate) fn unexpected(&self) -> Unexpected {
        match self.n {
            N::PosInt(u) => Unexpected::Unsigned(u),
            N::NegInt(i) => Unexpected::Signed(i),
            N::Float(f) => Unexpected::Float(f),
        }
    }
    #[cfg(feature = "arbitrary_precision")]
    #[cold]
    pub(crate) fn unexpected(&self) -> Unexpected {
        Unexpected::Other("number")
    }
}
#[cfg(test)]
mod tests_llm_16_79 {
    use crate::Number;
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any number")
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            assert_eq!(value, 42);
            Ok(())
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            assert_eq!(value, - 42);
            Ok(())
        }
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            assert!((value - 42.0).abs() < f64::EPSILON);
            Ok(())
        }
    }
    #[test]
    fn test_deserialize_any_posint() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_deserialize_any_posint = 0;
        let rug_fuzz_0 = 42;
        let number = Number {
            n: super::N::PosInt(rug_fuzz_0),
        };
        number.deserialize_any(TestVisitor).unwrap();
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_deserialize_any_posint = 0;
    }
    #[test]
    fn test_deserialize_any_negint() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_deserialize_any_negint = 0;
        let rug_fuzz_0 = 42;
        let number = Number {
            n: super::N::NegInt(-rug_fuzz_0),
        };
        number.deserialize_any(TestVisitor).unwrap();
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_deserialize_any_negint = 0;
    }
    #[test]
    fn test_deserialize_any_float() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_deserialize_any_float = 0;
        let rug_fuzz_0 = 42.0;
        let number = Number {
            n: super::N::Float(rug_fuzz_0),
        };
        number.deserialize_any(TestVisitor).unwrap();
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_deserialize_any_float = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_80_llm_16_80 {
    use serde::de::DeserializeOwned;
    use crate::number::Number;
    use crate::error::Error;
    use std::fmt;
    #[test]
    fn test_deserialize_f32() -> Result<(), Error> {
        let pos_float = Number::from_f64(123.456f64).unwrap();
        let neg_float = Number::from_f64(-123.456f64).unwrap();
        let pos_int = Number::from_f64(123f64).unwrap();
        let neg_int = Number::from_f64(-123f64).unwrap();
        let zero = Number::from_f64(0f64).unwrap();
        let small_float = Number::from_f64(1.23e-4f64).unwrap();
        let test_cases = vec![
            (pos_float, 123.456f32), (neg_float, - 123.456f32), (pos_int, 123f32),
            (neg_int, - 123f32), (zero, 0f32), (small_float, 1.23e-4f32),
        ];
        for (number, expected) in test_cases {
            let f: f32 = crate::from_value(crate::Value::Number(number))?;
            assert!(
                (f - expected).abs() < f32::EPSILON, "Expected {:?}, got {:?}", expected,
                f
            );
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_83 {
    use serde::{Deserialize, Deserializer};
    use crate::number::{N, Number};
    use serde::de::{self, Unexpected, Visitor};
    use std::fmt;
    #[test]
    fn test_deserialize_i16() {
        struct TestVisitor;
        impl<'de> Visitor<'de> for TestVisitor {
            type Value = i16;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an i16 integer")
            }
            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value)
            }
        }
        fn deserialize_i16<'de, D>(deserializer: D) -> Result<i16, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_i16(TestVisitor)
        }
        let num = Number { n: N::PosInt(16) };
        let deserializer = &mut crate::Deserializer::new(crate::de::StrRead::new("16"));
        let result: i16 = deserialize_i16(num).unwrap();
        assert_eq!(result, 16);
        let num = Number { n: N::NegInt(-16) };
        let deserializer = &mut crate::Deserializer::new(crate::de::StrRead::new("-16"));
        let result: i16 = deserialize_i16(num).unwrap();
        assert_eq!(result, - 16);
        let num = Number { n: N::PosInt(1 << 31) };
        let deserializer = &mut crate::Deserializer::new(
            crate::de::StrRead::new("1 << 31"),
        );
        let result: Result<i16, _> = deserialize_i16(num);
        assert!(result.is_err());
        let num = Number { n: N::Float(16.1) };
        let deserializer = &mut crate::Deserializer::new(
            crate::de::StrRead::new("16.1"),
        );
        let result: Result<i16, _> = deserialize_i16(num);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_88_llm_16_88 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::number::Number;
    use crate::error::Error;
    use std::fmt;
    use std::str::FromStr;
    struct U16Visitor;
    impl<'de> Visitor<'de> for U16Visitor {
        type Value = u16;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned 16-bit integer")
        }
        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    fn deserialize_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u16(U16Visitor)
    }
    #[test]
    fn test_deserialize_u16() {
        let s = "123";
        let num = Number::from_str(s)
            .map_err(de::Error::custom)
            .and_then(|number| deserialize_u16(number));
        assert!(matches!(num, Ok(123)));
    }
    #[test]
    fn test_deserialize_u16_out_of_range() {
        let s = "70000";
        let num = Number::from_str(s)
            .map_err(de::Error::custom)
            .and_then(|number| deserialize_u16(number));
        assert!(num.is_err());
    }
    #[test]
    fn test_deserialize_u16_negative() {
        let s = "-123";
        let num = Number::from_str(s)
            .map_err(de::Error::custom)
            .and_then(|number| deserialize_u16(number));
        assert!(num.is_err());
    }
    #[test]
    fn test_deserialize_u16_float() {
        let s = "123.45";
        let num = Number::from_str(s)
            .map_err(de::Error::custom)
            .and_then(|number| deserialize_u16(number));
        assert!(num.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::number::Number;
    use crate::value::Value;
    use crate::Error;
    use std::fmt;
    use std::str::FromStr;
    struct U32Visitor;
    impl<'de> Visitor<'de> for U32Visitor {
        type Value = u32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned 32-bit integer")
        }
        fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(U32Visitor)
    }
    #[test]
    fn test_deserialize_u32_from_number() {
        let pos_number = Number::from_str("42").unwrap();
        let deserialized: Result<u32, _> = deserialize_u32(pos_number);
        assert_eq!(deserialized.unwrap(), 42);
        let neg_number = Number::from_str("-1").unwrap();
        let deserialized: Result<u32, _> = deserialize_u32(neg_number);
        assert!(deserialized.is_err());
        let float_number = Number::from_str("42.5").unwrap();
        let deserialized: Result<u32, _> = deserialize_u32(float_number);
        assert!(deserialized.is_err());
        let big_number = Number::from_str(&(u32::MAX as u64 + 1).to_string()).unwrap();
        let deserialized: Result<u32, _> = deserialize_u32(big_number);
        assert!(deserialized.is_err());
        let big_neg_number = Number::from_str(&(i64::MIN).to_string()).unwrap();
        let deserialized: Result<u32, _> = deserialize_u32(big_neg_number);
        assert!(deserialized.is_err());
    }
    #[test]
    fn test_deserialize_u32_from_value() {
        let value = Value::String("42".to_string());
        let deserialized: Result<u32, _> = deserialize_u32(value);
        assert!(deserialized.is_err());
        let value = Value::Number(42.into());
        let deserialized: Result<u32, _> = deserialize_u32(value);
        assert_eq!(deserialized.unwrap(), 42);
        let value = Value::Number(crate::Number::from(42u64));
        let deserialized: Result<u32, _> = deserialize_u32(value);
        assert_eq!(deserialized.unwrap(), 42);
    }
}
#[cfg(test)]
mod tests_llm_16_90 {
    use serde::de::{Deserializer, Visitor};
    use crate::number::Number;
    use crate::value::Value;
    use crate::Error;
    use std::fmt;
    use std::str::FromStr;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u64;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a u64 JSON number")
        }
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("expected u64, found i64"))
        }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("expected u64, found f64"))
        }
    }
    #[test]
    fn deserialize_u64() -> Result<(), Error> {
        let tests = vec![
            ("0", 0_u64), ("42", 42_u64), ("18446744073709551615", u64::max_value())
        ];
        for (input, expected) in tests {
            let num: Value = Value::from_str(input)?;
            match num {
                Value::Number(ref number) => {
                    let u64_value: u64 = number.deserialize_u64(TestVisitor)?;
                    assert_eq!(u64_value, expected);
                }
                _ => panic!("Expected a number"),
            }
        }
        Ok(())
    }
    #[test]
    fn deserialize_u64_invalid() {
        let tests = vec![
            "-1", "18446744073709551616", "3.14", "\"42\"", "null", "[]", "{}"
        ];
        for input in tests {
            let num: Result<Value, _> = Value::from_str(input);
            if let Ok(Value::Number(ref number)) = num {
                let result: Result<u64, _> = number.deserialize_u64(TestVisitor);
                assert!(result.is_err());
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_91 {
    use serde::de::{self, Deserialize, Deserializer, Error, Visitor};
    use crate::number::Number;
    use crate::value::Value;
    use std::fmt;
    use std::str::FromStr;
    struct U8Visitor;
    impl<'de> Visitor<'de> for U8Visitor {
        type Value = u8;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned 8-bit integer")
        }
        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value)
        }
    }
    fn deserialize_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(U8Visitor)
    }
    #[test]
    fn test_deserialize_u8() {
        let valid_u8 = "34";
        let valid_u8_json = Value::from_str(valid_u8).unwrap();
        let deserialized: u8 = deserialize_u8(valid_u8_json).unwrap();
        assert_eq!(deserialized, 34u8);
        let invalid_u8 = "256";
        let invalid_u8_json = Value::from_str(invalid_u8).unwrap();
        let result: Result<u8, _> = deserialize_u8(invalid_u8_json);
        assert!(result.is_err());
        let invalid_type = "true";
        let invalid_type_json = Value::from_str(invalid_type).unwrap();
        let result: Result<u8, _> = deserialize_u8(invalid_type_json);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_97_llm_16_97 {
    use serde::de::{self, Deserialize, Visitor, Error as SerdeError};
    use crate::number::Number;
    use crate::error::Error;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Number;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a JSON number")
        }
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: SerdeError,
        {
            Number::from_f64(value).ok_or_else(|| E::custom("not a JSON number"))
        }
    }
    #[test]
    fn test_visit_f64_valid() {
        let _rug_st_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_visit_f64_valid = 0;
        let rug_fuzz_0 = 123.456f64;
        let visitor = TestVisitor;
        let num = rug_fuzz_0;
        let result: Result<Number, Error> = visitor.visit_f64(num);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), Number::from_f64(num).unwrap());
        let _rug_ed_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_visit_f64_valid = 0;
    }
    #[test]
    fn test_visit_f64_invalid() {
        let _rug_st_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_visit_f64_invalid = 0;
        let visitor = TestVisitor;
        let num = f64::NAN;
        let result: Result<Number, Error> = visitor.visit_f64(num);
        debug_assert!(result.is_err());
        let error = result.unwrap_err();
        debug_assert_eq!(error.to_string(), "not a JSON number");
        let _rug_ed_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_visit_f64_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_172_llm_16_172 {
    use crate::number::N;
    #[test]
    fn test_eq_pos_int() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_pos_int = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 7;
        let a = N::PosInt(rug_fuzz_0);
        let b = N::PosInt(rug_fuzz_1);
        let c = N::PosInt(rug_fuzz_2);
        debug_assert!(a.eq(& b));
        debug_assert!(! a.eq(& c));
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_pos_int = 0;
    }
    #[test]
    fn test_eq_neg_int() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_neg_int = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 7;
        let a = N::NegInt(-rug_fuzz_0);
        let b = N::NegInt(-rug_fuzz_1);
        let c = N::NegInt(-rug_fuzz_2);
        debug_assert!(a.eq(& b));
        debug_assert!(! a.eq(& c));
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_neg_int = 0;
    }
    #[test]
    fn test_eq_float() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_float = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 7.0;
        let a = N::Float(rug_fuzz_0);
        let b = N::Float(rug_fuzz_1);
        let c = N::Float(rug_fuzz_2);
        debug_assert!(a.eq(& b));
        debug_assert!(! a.eq(& c));
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_float = 0;
    }
    #[test]
    fn test_eq_mixed() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_mixed = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42.0;
        let pos_int = N::PosInt(rug_fuzz_0);
        let neg_int = N::NegInt(-rug_fuzz_1);
        let float = N::Float(rug_fuzz_2);
        debug_assert!(! pos_int.eq(& neg_int));
        debug_assert!(! pos_int.eq(& float));
        debug_assert!(! neg_int.eq(& float));
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_mixed = 0;
    }
    #[test]
    fn test_eq_zero_float() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_zero_float = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let a = N::Float(rug_fuzz_0);
        let b = N::Float(-rug_fuzz_1);
        debug_assert!(a.eq(& b));
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_zero_float = 0;
    }
    #[test]
    fn test_eq_nan_float() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_nan_float = 0;
        let a = N::Float(f64::NAN);
        let b = N::Float(f64::NAN);
        debug_assert!(! a.eq(& b));
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_eq_nan_float = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_173 {
    use super::*;
    use crate::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    #[test]
    fn test_hash_pos_int() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_test_hash_pos_int = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42u64;
        let value = N::PosInt(rug_fuzz_0);
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hashed_value = hasher.finish();
        let mut hasher_control = DefaultHasher::new();
        rug_fuzz_1.hash(&mut hasher_control);
        let control_hash = hasher_control.finish();
        debug_assert_eq!(hashed_value, control_hash);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_test_hash_pos_int = 0;
    }
    #[test]
    fn test_hash_neg_int() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_test_hash_neg_int = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42i64;
        let value = N::NegInt(-rug_fuzz_0);
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hashed_value = hasher.finish();
        let mut hasher_control = DefaultHasher::new();
        (-rug_fuzz_1).hash(&mut hasher_control);
        let control_hash = hasher_control.finish();
        debug_assert_eq!(hashed_value, control_hash);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_test_hash_neg_int = 0;
    }
    #[test]
    fn test_hash_float_positive_zero() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_test_hash_float_positive_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0f64;
        let value = N::Float(rug_fuzz_0);
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hashed_value = hasher.finish();
        let mut hasher_control = DefaultHasher::new();
        rug_fuzz_1.to_bits().hash(&mut hasher_control);
        let control_hash = hasher_control.finish();
        debug_assert_eq!(hashed_value, control_hash);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_test_hash_float_positive_zero = 0;
    }
    #[test]
    fn test_hash_float_negative_zero() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_test_hash_float_negative_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0f64;
        let value = N::Float(-rug_fuzz_0);
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hashed_value = hasher.finish();
        let mut hasher_control = DefaultHasher::new();
        (-rug_fuzz_1).to_bits().hash(&mut hasher_control);
        let control_hash = hasher_control.finish();
        debug_assert_eq!(hashed_value, control_hash);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_test_hash_float_negative_zero = 0;
    }
    #[test]
    fn test_hash_float_non_zero() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_test_hash_float_non_zero = 0;
        let rug_fuzz_0 = 3.1415;
        let rug_fuzz_1 = 3.1415f64;
        let value = N::Float(rug_fuzz_0);
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hashed_value = hasher.finish();
        let mut hasher_control = DefaultHasher::new();
        rug_fuzz_1.to_bits().hash(&mut hasher_control);
        let control_hash = hasher_control.finish();
        debug_assert_eq!(hashed_value, control_hash);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_test_hash_float_non_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_176 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::number::Number;
    use crate::Error;
    use std::fmt;
    use std::str::FromStr;
    struct F32Visitor;
    impl<'de> Visitor<'de> for F32Visitor {
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
    fn deserialize_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_f32(F32Visitor)
    }
    #[test]
    fn test_deserialize_f32() {
        let num_str = "2.5";
        let num = Number::from_str(num_str).unwrap();
        let deserialized: Result<f32, Error> = deserialize_f32(num);
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), 2.5_f32);
    }
    #[test]
    fn test_deserialize_f32_negative() {
        let num_str = "-3.5";
        let num = Number::from_str(num_str).unwrap();
        let deserialized: Result<f32, Error> = deserialize_f32(num);
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), - 3.5_f32);
    }
    #[test]
    fn test_deserialize_f32_invalid_type() {
        let num_str = "invalid";
        let num = Number::from_str(num_str);
        assert!(num.is_err());
    }
    #[test]
    fn test_deserialize_f32_out_of_range() {
        let num_str = "3.4028235e39";
        let num = Number::from_str(num_str).unwrap();
        let deserialized: Result<f32, Error> = deserialize_f32(num);
        assert!(deserialized.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_177_llm_16_177 {
    use serde::de::{self, Deserializer, Visitor};
    use crate::error::Error;
    use crate::number::Number;
    use std::fmt;
    use crate::from_str;
    struct F64Visitor;
    impl<'de> Visitor<'de> for F64Visitor {
        type Value = f64;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float")
        }
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    #[test]
    fn test_deserialize_f64() {
        let _rug_st_tests_llm_16_177_llm_16_177_rrrruuuugggg_test_deserialize_f64 = 0;
        let rug_fuzz_0 = "42.0";
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = "\"not_a_number\"";
        let valid_number_str = rug_fuzz_0;
        let valid_number: Number = from_str(valid_number_str).unwrap();
        let expected_number = Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert_eq!(valid_number, expected_number);
        let invalid_number_str = rug_fuzz_2;
        let invalid_number: Result<Number, Error> = from_str(invalid_number_str);
        debug_assert!(invalid_number.is_err());
        let _rug_ed_tests_llm_16_177_llm_16_177_rrrruuuugggg_test_deserialize_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_179 {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use crate::number::{N, Number};
    use crate::Error;
    use std::fmt;
    struct I16Visitor;
    impl<'de> Visitor<'de> for I16Visitor {
        type Value = i16;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an i16")
        }
        fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    #[test]
    fn test_deserialize_i16_with_positive_int() -> Result<(), Error> {
        let n = Number { n: N::PosInt(123) };
        let i16_val = n.deserialize_i16(I16Visitor)?;
        assert_eq!(i16_val, 123);
        Ok(())
    }
    #[test]
    fn test_deserialize_i16_with_negative_int() -> Result<(), Error> {
        let n = Number { n: N::NegInt(-123) };
        let i16_val = n.deserialize_i16(I16Visitor)?;
        assert_eq!(i16_val, - 123);
        Ok(())
    }
    #[test]
    fn test_deserialize_i16_with_out_of_range_positive_int() -> Result<(), Error> {
        let n = Number {
            n: N::PosInt(i32::MAX as u64 + 1),
        };
        let result = n.deserialize_i16(I16Visitor);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_deserialize_i16_with_out_of_range_negative_int() -> Result<(), Error> {
        let n = Number {
            n: N::NegInt(i32::MIN as i64 - 1),
        };
        let result = n.deserialize_i16(I16Visitor);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_deserialize_i16_with_float() -> Result<(), Error> {
        let n = Number { n: N::Float(123.456) };
        let result = n.deserialize_i16(I16Visitor);
        assert!(result.is_err());
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_182 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::number::Number;
    use crate::value::{self, Value};
    use std::fmt;
    use std::str::FromStr;
    struct I8Visitor;
    impl<'de> de::Visitor<'de> for I8Visitor {
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
    fn deserialize_i8<'de, D>(deserializer: D) -> Result<i8, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i8(I8Visitor)
    }
    #[test]
    fn test_deserialize_i8() {
        let json_number = Number::from_str("-123").unwrap();
        let i8_val: Result<i8, _> = deserialize_i8(json_number);
        assert_eq!(i8_val.unwrap(), - 123i8);
        let json_number = Number::from_str("123").unwrap();
        let i8_val: Result<i8, _> = deserialize_i8(json_number);
        assert_eq!(i8_val.unwrap(), 123i8);
        let json_number = Number::from_str("128").unwrap();
        let i8_val: Result<i8, _> = deserialize_i8(json_number);
        assert!(i8_val.is_err());
        let json_number = Number::from_str("-129").unwrap();
        let i8_val: Result<i8, _> = deserialize_i8(json_number);
        assert!(i8_val.is_err());
        let json_number = Number::from_str("12.3").unwrap();
        let i8_val: Result<i8, _> = deserialize_i8(json_number);
        assert!(i8_val.is_err());
        let json_str = Value::String("not a number".to_string());
        let i8_val: Result<i8, _> = deserialize_i8(json_str);
        assert!(i8_val.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_183 {
    use serde::de::{self, Deserialize, Deserializer};
    use crate::value::Number;
    use std::fmt;
    use std::str::FromStr;
    struct U128Visitor;
    impl<'de> de::Visitor<'de> for U128Visitor {
        type Value = u128;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer within range u128")
        }
        fn visit_u128<E>(self, value: u128) -> Result<u128, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }
    fn deserialize_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u128(U128Visitor)
    }
    #[test]
    fn deserialize_unsigned_128() {
        let numbers = vec![
            ("0", 0u128), ("255", 255u128), ("65535", 65535u128), ("4294967295",
            4294967295u128), ("18446744073709551615", 18446744073709551615u128),
            ("340282366920938463463374607431768211455", u128::MAX),
        ];
        for (num_str, expected) in numbers {
            let num = Number::from_str(num_str).unwrap();
            let deserialized: Result<u128, _> = deserialize_u128(num);
            assert_eq!(deserialized.unwrap(), expected);
        }
    }
    #[test]
    #[should_panic(expected = "invalid type: string")]
    fn deserialize_out_of_range() {
        let num_str = "340282366920938463463374607431768211456";
        let num = Number::from_str(num_str).unwrap();
        let _deserialized: u128 = deserialize_u128(num).unwrap();
    }
    #[test]
    #[should_panic(expected = "not a JSON number")]
    fn deserialize_negative() {
        let num_str = "-1";
        let num = Number::from_str(num_str).unwrap();
        let _deserialized: u128 = deserialize_u128(num).unwrap();
    }
    #[test]
    #[should_panic(expected = "not a JSON number")]
    fn deserialize_float() {
        let num_str = "0.1";
        let num = Number::from_str(num_str).unwrap();
        let _deserialized: u128 = deserialize_u128(num).unwrap();
    }
}
#[cfg(test)]
mod tests_llm_16_185 {
    use serde::{Deserialize, Deserializer};
    use crate::{Number, Error};
    use std::fmt;
    struct U32Visitor;
    impl<'de> serde::de::Visitor<'de> for U32Visitor {
        type Value = u32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a u32 integer")
        }
        fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
    }
    fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(U32Visitor)
    }
    #[test]
    fn test_deserialize_u32() {
        let number: Number = 42u64.into();
        let u32_value: Result<u32, Error> = deserialize_u32(number);
        assert_eq!(u32_value.unwrap(), 42u32);
    }
    #[test]
    fn test_deserialize_u32_out_of_range() {
        let number: Number = (u32::MAX as u64 + 1).into();
        let u32_value: Result<u32, Error> = deserialize_u32(number);
        assert!(u32_value.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_188_llm_16_188 {
    use crate::{Number, number::N, ser::Serializer};
    use crate::value::{self, to_value};
    use serde::Serialize;
    fn create_serializer() -> Serializer<Vec<u8>> {
        Serializer::new(Vec::new())
    }
    #[test]
    fn test_serialize_pos_int() {
        let number = Number { n: N::PosInt(42u64) };
        let mut serializer = create_serializer();
        number.serialize(&mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, "42");
    }
    #[test]
    fn test_serialize_neg_int() {
        let number = Number { n: N::NegInt(-42i64) };
        let mut serializer = create_serializer();
        number.serialize(&mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, "-42");
    }
    #[test]
    fn test_serialize_float() {
        let number = Number { n: N::Float(42.0) };
        let mut serializer = create_serializer();
        number.serialize(&mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, "42.0");
    }
    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            number: Number,
        }
        let test_struct = TestStruct {
            number: Number { n: N::PosInt(42u64) },
        };
        let serialized = to_value(test_struct).unwrap();
        let serialized_str = crate::to_string(&serialized).unwrap();
        assert_eq!(serialized_str, r#"{"number":42}"#);
    }
}
#[cfg(test)]
mod tests_llm_16_189 {
    use super::*;
    use crate::*;
    use crate::number::Number;
    use crate::de::ParserNumber;
    use std::str::FromStr;
    #[test]
    #[cfg(not(feature = "arbitrary_precision"))]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.5;
        let v = rug_fuzz_0;
        let parser_num = ParserNumber::F64(v);
        let num = Number::from(parser_num);
        debug_assert_eq!(num.is_f64(), true);
        debug_assert_eq!(num.is_i64(), false);
        debug_assert_eq!(num.is_u64(), false);
        debug_assert_eq!(num.as_f64(), Some(v));
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_f64 = 0;
    }
    #[test]
    #[cfg(not(feature = "arbitrary_precision"))]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 42;
        let v = rug_fuzz_0;
        let parser_num = ParserNumber::U64(v);
        let num = Number::from(parser_num);
        debug_assert_eq!(num.is_u64(), true);
        debug_assert_eq!(num.is_i64(), false);
        debug_assert_eq!(num.is_f64(), false);
        debug_assert_eq!(num.as_u64(), Some(v));
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_u64 = 0;
    }
    #[test]
    #[cfg(not(feature = "arbitrary_precision"))]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 42;
        let v = -rug_fuzz_0;
        let parser_num = ParserNumber::I64(v);
        let num = Number::from(parser_num);
        debug_assert_eq!(num.is_i64(), true);
        debug_assert_eq!(num.is_u64(), false);
        debug_assert_eq!(num.is_f64(), false);
        debug_assert_eq!(num.as_i64(), Some(v));
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_i64 = 0;
    }
    #[test]
    #[cfg(feature = "arbitrary_precision")]
    fn test_from_f64_arbitrary_precision() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_f64_arbitrary_precision = 0;
        let rug_fuzz_0 = 42.5;
        let v = rug_fuzz_0;
        let parser_num = ParserNumber::F64(v);
        let num = Number::from(parser_num);
        debug_assert_eq!(num.is_f64(), false);
        debug_assert_eq!(num.to_string(), v.to_string());
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_f64_arbitrary_precision = 0;
    }
    #[test]
    #[cfg(feature = "arbitrary_precision")]
    fn test_from_u64_arbitrary_precision() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_u64_arbitrary_precision = 0;
        let rug_fuzz_0 = 42;
        let v = rug_fuzz_0;
        let parser_num = ParserNumber::U64(v);
        let num = Number::from(parser_num);
        debug_assert_eq!(num.is_u64(), false);
        debug_assert_eq!(num.to_string(), v.to_string());
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_u64_arbitrary_precision = 0;
    }
    #[test]
    #[cfg(feature = "arbitrary_precision")]
    fn test_from_i64_arbitrary_precision() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_i64_arbitrary_precision = 0;
        let rug_fuzz_0 = 42;
        let v = -rug_fuzz_0;
        let parser_num = ParserNumber::I64(v);
        let num = Number::from(parser_num);
        debug_assert_eq!(num.is_i64(), false);
        debug_assert_eq!(num.to_string(), v.to_string());
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_i64_arbitrary_precision = 0;
    }
    #[test]
    #[cfg(feature = "arbitrary_precision")]
    fn test_from_string() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_string = 0;
        let rug_fuzz_0 = "42.5";
        let v = rug_fuzz_0;
        let parser_num = ParserNumber::String(v.to_owned());
        let num = Number::from(parser_num);
        debug_assert_eq!(num.to_string(), v);
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_string = 0;
    }
    #[test]
    fn test_from_str() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_from_str = 0;
        let rug_fuzz_0 = "42";
        let v = rug_fuzz_0;
        let num = Number::from_str(v).unwrap();
        debug_assert_eq!(num.to_string(), v);
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_from_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_190 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_190_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 123;
        let rug_fuzz_2 = 0;
        let pos_i16: i16 = rug_fuzz_0;
        let pos_num = Number::from(pos_i16);
        debug_assert_eq!(pos_num, Number { n : N::PosInt(123) });
        let neg_i16: i16 = -rug_fuzz_1;
        let neg_num = Number::from(neg_i16);
        debug_assert_eq!(neg_num, Number { n : N::NegInt(- 123) });
        let zero_i16: i16 = rug_fuzz_2;
        let zero_num = Number::from(zero_i16);
        debug_assert_eq!(zero_num, Number { n : N::PosInt(0) });
        let max_i16: i16 = i16::MAX;
        let max_num = Number::from(max_i16);
        debug_assert_eq!(max_num, Number { n : N::PosInt(i16::MAX as u64) });
        let min_i16: i16 = i16::MIN;
        let min_num = Number::from(min_i16);
        debug_assert_eq!(min_num, Number { n : N::NegInt(i16::MIN as i64) });
        let _rug_ed_tests_llm_16_190_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_191_llm_16_191 {
    use crate::number::Number;
    use crate::number::N;
    use std::convert::From;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_191_llm_16_191_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let positive_i32: i32 = rug_fuzz_0;
        let negative_i32: i32 = -rug_fuzz_1;
        let zero_i32: i32 = rug_fuzz_2;
        let positive_number = Number::from(positive_i32);
        let negative_number = Number::from(negative_i32);
        let zero_number = Number::from(zero_i32);
        #[cfg(not(feature = "arbitrary_precision"))]
        {
            debug_assert!(
                matches!(positive_number.n, N::PosInt(u) if u == positive_i32 as u64)
            );
            debug_assert!(
                matches!(negative_number.n, N::NegInt(i) if i == negative_i32 as i64)
            );
            debug_assert!(matches!(zero_number.n, N::PosInt(u) if u == 0));
        }
        #[cfg(feature = "arbitrary_precision")]
        {
            debug_assert_eq!(positive_number.to_string(), positive_i32.to_string());
            debug_assert_eq!(negative_number.to_string(), negative_i32.to_string());
            debug_assert_eq!(zero_number.to_string(), zero_i32.to_string());
        }
        let _rug_ed_tests_llm_16_191_llm_16_191_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_192 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_192_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let rug_fuzz_1 = 42i64;
        let pos_i64 = rug_fuzz_0;
        let pos_number = Number::from(pos_i64);
        debug_assert!(pos_number.is_i64());
        debug_assert_eq!(pos_number.as_i64(), Some(42i64));
        debug_assert_eq!(pos_number.as_u64(), Some(42u64));
        debug_assert!(pos_number.is_u64());
        debug_assert!(! pos_number.is_f64());
        let neg_i64 = -rug_fuzz_1;
        let neg_number = Number::from(neg_i64);
        debug_assert!(neg_number.is_i64());
        debug_assert_eq!(neg_number.as_i64(), Some(- 42i64));
        debug_assert!(! neg_number.is_u64());
        debug_assert!(! neg_number.is_f64());
        let _rug_ed_tests_llm_16_192_rrrruuuugggg_test_from_i64 = 0;
    }
    #[test]
    fn test_number_eq() {
        let _rug_st_tests_llm_16_192_rrrruuuugggg_test_number_eq = 0;
        let rug_fuzz_0 = 123i64;
        let rug_fuzz_1 = 123u64;
        let rug_fuzz_2 = 123i64;
        let number_from_i64 = Number::from(rug_fuzz_0);
        let number_from_u64 = Number::from(rug_fuzz_1);
        debug_assert_eq!(number_from_i64, number_from_u64);
        let number_from_i64_neg = Number::from(-rug_fuzz_2);
        debug_assert_ne!(number_from_i64_neg, number_from_i64);
        let _rug_ed_tests_llm_16_192_rrrruuuugggg_test_number_eq = 0;
    }
    #[test]
    fn test_number_hash() {
        let _rug_st_tests_llm_16_192_rrrruuuugggg_test_number_hash = 0;
        let rug_fuzz_0 = 123i64;
        let rug_fuzz_1 = 123i64;
        let rug_fuzz_2 = 456i64;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        let number = Number::from(rug_fuzz_0);
        number.hash(&mut hasher);
        let hashed = hasher.finish();
        let mut hasher_same = DefaultHasher::new();
        let number_same = Number::from(rug_fuzz_1);
        number_same.hash(&mut hasher_same);
        let hashed_same = hasher_same.finish();
        debug_assert_eq!(hashed, hashed_same);
        let mut hasher_different = DefaultHasher::new();
        let number_different = Number::from(rug_fuzz_2);
        number_different.hash(&mut hasher_different);
        let hashed_different = hasher_different.finish();
        debug_assert_ne!(hashed, hashed_different);
        let _rug_ed_tests_llm_16_192_rrrruuuugggg_test_number_hash = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_193 {
    use super::*;
    use crate::*;
    use std::convert::From;
    use crate::number::Number;
    #[test]
    fn test_from_i8_for_number() {
        let _rug_st_tests_llm_16_193_rrrruuuugggg_test_from_i8_for_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let positive_i8: i8 = rug_fuzz_0;
        let positive_number = Number::from(positive_i8);
        debug_assert!(positive_number.is_u64());
        debug_assert_eq!(positive_number.as_u64(), Some(42));
        debug_assert!(! positive_number.is_i64());
        debug_assert!(! positive_number.is_f64());
        let negative_i8: i8 = -rug_fuzz_1;
        let negative_number = Number::from(negative_i8);
        debug_assert!(! negative_number.is_u64());
        debug_assert!(negative_number.is_i64());
        debug_assert_eq!(negative_number.as_i64(), Some(- 42));
        debug_assert!(! negative_number.is_f64());
        let zero_i8: i8 = rug_fuzz_2;
        let zero_number = Number::from(zero_i8);
        debug_assert!(zero_number.is_u64());
        debug_assert_eq!(zero_number.as_u64(), Some(0));
        debug_assert!(zero_number.is_i64());
        debug_assert_eq!(zero_number.as_i64(), Some(0));
        debug_assert!(! zero_number.is_f64());
        let _rug_ed_tests_llm_16_193_rrrruuuugggg_test_from_i8_for_number = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_194 {
    use crate::Number;
    use std::convert::From;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_194_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 42_isize;
        let rug_fuzz_1 = 42_isize;
        let pos_isize = rug_fuzz_0;
        let pos_number = Number::from(pos_isize);
        debug_assert!(pos_number.is_u64());
        debug_assert_eq!(pos_number.as_u64(), Some(pos_isize as u64));
        let neg_isize = -rug_fuzz_1;
        let neg_number = Number::from(neg_isize);
        debug_assert!(neg_number.is_i64());
        debug_assert_eq!(neg_number.as_i64(), Some(neg_isize as i64));
        let max_isize = isize::MAX;
        let max_number = Number::from(max_isize);
        debug_assert!(max_number.is_i64());
        debug_assert_eq!(max_number.as_i64(), Some(max_isize as i64));
        let min_isize = isize::MIN;
        let min_number = Number::from(min_isize);
        debug_assert!(min_number.is_i64());
        debug_assert_eq!(min_number.as_i64(), Some(min_isize as i64));
        let _rug_ed_tests_llm_16_194_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_195 {
    use crate::Number;
    use std::convert::From;
    #[test]
    fn test_u16_into_number() {
        let _rug_st_tests_llm_16_195_rrrruuuugggg_test_u16_into_number = 0;
        let rug_fuzz_0 = 42u16;
        let num = Number::from(rug_fuzz_0);
        debug_assert!(num.is_u64());
        debug_assert_eq!(num.as_u64(), Some(42));
        debug_assert!(! num.is_i64());
        debug_assert_eq!(num.as_i64(), Some(42));
        debug_assert!(! num.is_f64());
        debug_assert_eq!(num.as_f64(), Some(42.0));
        let _rug_ed_tests_llm_16_195_rrrruuuugggg_test_u16_into_number = 0;
    }
    #[test]
    fn test_u16_into_number_boundary() {
        let _rug_st_tests_llm_16_195_rrrruuuugggg_test_u16_into_number_boundary = 0;
        let max = u16::MAX;
        let num = Number::from(max);
        debug_assert!(num.is_u64());
        debug_assert_eq!(num.as_u64(), Some(u64::from(max)));
        debug_assert!(! num.is_i64());
        debug_assert_eq!(num.as_i64(), Some(i64::from(max)));
        debug_assert!(! num.is_f64());
        debug_assert_eq!(num.as_f64(), Some(f64::from(max)));
        let _rug_ed_tests_llm_16_195_rrrruuuugggg_test_u16_into_number_boundary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_196 {
    use crate::number::Number;
    use std::convert::From;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = "0";
        let rug_fuzz_2 = 42_u32;
        let rug_fuzz_3 = "42";
        let rug_fuzz_4 = "4294967295";
        let test_cases = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (u32::MAX, rug_fuzz_4),
        ];
        for (input, expected) in test_cases {
            let number = Number::from(input);
            debug_assert_eq!(number.to_string(), expected);
        }
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_197 {
    use super::*;
    use crate::*;
    use crate::Number;
    use std::convert::From;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_197_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let num_u64 = rug_fuzz_0;
        let number: Number = Number::from(num_u64);
        #[cfg(not(feature = "arbitrary_precision"))]
        debug_assert_eq!(number, Number { n : number::N::PosInt(num_u64 as u64) });
        #[cfg(feature = "arbitrary_precision")]
        debug_assert_eq!(number, Number::from(num_u64.to_string().as_str()));
        let _rug_ed_tests_llm_16_197_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_198_llm_16_198 {
    use crate::number::{N, Number};
    use std::convert::From;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_198_llm_16_198_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 0_u64;
        let rug_fuzz_2 = 1_u8;
        let rug_fuzz_3 = 1_u64;
        let rug_fuzz_4 = 255_u8;
        let rug_fuzz_5 = 255_u64;
        let test_cases = [
            (rug_fuzz_0, N::PosInt(rug_fuzz_1)),
            (rug_fuzz_2, N::PosInt(rug_fuzz_3)),
            (rug_fuzz_4, N::PosInt(rug_fuzz_5)),
        ];
        for (input, expected_n) in test_cases {
            let number = Number::from(input);
            debug_assert!(matches!(number.n, expected_n));
        }
        let _rug_ed_tests_llm_16_198_llm_16_198_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_199 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn from_usize_arbitrary_precision_disabled() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_from_usize_arbitrary_precision_disabled = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let number = Number::from(value);
        #[cfg(not(feature = "arbitrary_precision"))]
        match number.n {
            N::PosInt(v) => {
                debug_assert_eq!(v, value as u64);
            }
            _ => panic!("Expected PosInt variant"),
        }
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_from_usize_arbitrary_precision_disabled = 0;
    }
    #[test]
    fn from_usize_arbitrary_precision_enabled() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_from_usize_arbitrary_precision_enabled = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let number = Number::from(value);
        #[cfg(feature = "arbitrary_precision")]
        debug_assert_eq!(number.n, value.to_string());
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_from_usize_arbitrary_precision_enabled = 0;
    }
    #[test]
    fn from_usize_check_is_u64() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_from_usize_check_is_u64 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let number = Number::from(value);
        debug_assert!(number.is_u64());
        debug_assert_eq!(number.as_u64(), Some(value as u64));
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_from_usize_check_is_u64 = 0;
    }
    #[test]
    fn from_usize_check_is_i64() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_from_usize_check_is_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let number = Number::from(value);
        debug_assert!(number.is_i64());
        debug_assert_eq!(number.as_i64(), Some(value as i64));
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_from_usize_check_is_i64 = 0;
    }
    #[test]
    fn from_usize_is_not_negative() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_from_usize_is_not_negative = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let number = Number::from(value);
        debug_assert!(! number.is_f64());
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_from_usize_is_not_negative = 0;
    }
    #[test]
    fn from_usize_is_displayable() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_from_usize_is_displayable = 0;
        let rug_fuzz_0 = 42;
        use std::fmt::Display;
        let value: usize = rug_fuzz_0;
        let number = Number::from(value);
        debug_assert_eq!(number.to_string(), value.to_string());
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_from_usize_is_displayable = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_533 {
    use crate::Number;
    use crate::number::N;
    #[cfg(not(feature = "arbitrary_precision"))]
    #[test]
    fn test_as_f32() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_as_f32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42.5;
        let rug_fuzz_3 = 2.0;
        let num_pos = Number { n: N::PosInt(rug_fuzz_0) };
        debug_assert_eq!(num_pos.as_f32(), Some(42.0_f32));
        let num_neg = Number {
            n: N::NegInt(-rug_fuzz_1),
        };
        debug_assert_eq!(num_neg.as_f32(), Some(- 42.0_f32));
        let num_float = Number { n: N::Float(rug_fuzz_2) };
        debug_assert_eq!(num_float.as_f32(), Some(42.5_f32));
        let num_big_float = Number {
            n: N::Float(f64::from(f32::MAX) * rug_fuzz_3),
        };
        debug_assert!(num_big_float.as_f32().is_some());
        debug_assert!(num_big_float.as_f32().unwrap().is_infinite());
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_as_f32 = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_as_f32_with_arbitrary_precision() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_as_f32_with_arbitrary_precision = 0;
        let rug_fuzz_0 = "42.5";
        let rug_fuzz_1 = "1e40";
        let rug_fuzz_2 = "not a number";
        let num_str_finite = Number::from_string_unchecked(rug_fuzz_0.to_string());
        debug_assert_eq!(num_str_finite.as_f32(), Some(42.5_f32));
        let num_str_infinite = Number::from_string_unchecked(rug_fuzz_1.to_string());
        debug_assert!(num_str_infinite.as_f32().is_some());
        debug_assert!(num_str_infinite.as_f32().unwrap().is_infinite());
        let num_str_invalid = Number::from_string_unchecked(rug_fuzz_2.to_string());
        debug_assert_eq!(num_str_invalid.as_f32(), None);
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_as_f32_with_arbitrary_precision = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_534 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_f64_with_finite_float() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_finite_float = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 2.71;
        let rug_fuzz_2 = 0.0;
        let pos_float = Number::from_f64(rug_fuzz_0).unwrap();
        let neg_float = Number::from_f64(-rug_fuzz_1).unwrap();
        let zero_float = Number::from_f64(rug_fuzz_2).unwrap();
        debug_assert_eq!(pos_float.as_f64(), Some(3.14));
        debug_assert_eq!(neg_float.as_f64(), Some(- 2.71));
        debug_assert_eq!(zero_float.as_f64(), Some(0.0));
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_finite_float = 0;
    }
    #[test]
    fn test_as_f64_with_positive_and_negative_integers() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_positive_and_negative_integers = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42_i64;
        let pos_int = Number::from(rug_fuzz_0);
        let neg_int = Number::from(-rug_fuzz_1);
        debug_assert_eq!(pos_int.as_f64(), Some(42.0));
        debug_assert_eq!(neg_int.as_f64(), Some(- 42.0));
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_positive_and_negative_integers = 0;
    }
    #[test]
    fn test_as_f64_with_positive_and_negative_integer_edge_cases() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_positive_and_negative_integer_edge_cases = 0;
        let max_u64 = Number::from(u64::MAX);
        let max_i64 = Number::from(i64::MAX);
        let min_i64 = Number::from(i64::MIN);
        debug_assert_eq!(max_u64.as_f64(), Some(u64::MAX as f64));
        debug_assert_eq!(max_i64.as_f64(), Some(i64::MAX as f64));
        debug_assert_eq!(min_i64.as_f64(), Some(i64::MIN as f64));
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_positive_and_negative_integer_edge_cases = 0;
    }
    #[test]
    fn test_as_f64_with_precise_and_imprecise_conversion() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_precise_and_imprecise_conversion = 0;
        let large_i64 = Number::from(i64::MAX);
        let large_u64 = Number::from(u64::MAX);
        debug_assert!(large_u64.as_f64().unwrap() != u64::MAX as f64);
        debug_assert_eq!(large_i64.as_f64(), Some(i64::MAX as f64));
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_precise_and_imprecise_conversion = 0;
    }
    #[test]
    fn test_as_f64_with_infinite_and_nan() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_infinite_and_nan = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 0_u64;
        let rug_fuzz_2 = 0_u64;
        let positive_infinity = Number::from_f64(f64::INFINITY)
            .unwrap_or_else(|| Number::from(rug_fuzz_0));
        let negative_infinity = Number::from_f64(f64::NEG_INFINITY)
            .unwrap_or_else(|| Number::from(rug_fuzz_1));
        let nan = Number::from_f64(f64::NAN).unwrap_or_else(|| Number::from(rug_fuzz_2));
        debug_assert_eq!(positive_infinity.as_f64(), None);
        debug_assert_eq!(negative_infinity.as_f64(), None);
        debug_assert_eq!(nan.as_f64(), None);
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_test_as_f64_with_infinite_and_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_535 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_i64_with_positive_integer() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_positive_integer = 0;
        let rug_fuzz_0 = 42u64;
        let pos_int = Number::from(rug_fuzz_0);
        debug_assert_eq!(pos_int.as_i64(), Some(42i64));
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_positive_integer = 0;
    }
    #[test]
    fn test_as_i64_with_negative_integer() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_negative_integer = 0;
        let rug_fuzz_0 = 42i64;
        let neg_int = Number::from(-rug_fuzz_0);
        debug_assert_eq!(neg_int.as_i64(), Some(- 42i64));
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_negative_integer = 0;
    }
    #[test]
    fn test_as_i64_with_large_positive_integer() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_large_positive_integer = 0;
        let rug_fuzz_0 = 1;
        let large_pos_int = Number::from(i64::max_value() as u64 + rug_fuzz_0);
        debug_assert_eq!(large_pos_int.as_i64(), None);
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_large_positive_integer = 0;
    }
    #[test]
    fn test_as_i64_with_float() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_float = 0;
        let rug_fuzz_0 = 42.1;
        let float = Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert_eq!(float.as_i64(), None);
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_float = 0;
    }
    #[test]
    fn test_as_i64_with_zero() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_zero = 0;
        let rug_fuzz_0 = 0i64;
        let zero = Number::from(rug_fuzz_0);
        debug_assert_eq!(zero.as_i64(), Some(0i64));
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_zero = 0;
    }
    #[test]
    fn test_as_i64_with_large_negative_integer() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_large_negative_integer = 0;
        let large_neg_int = Number::from(i64::min_value());
        debug_assert_eq!(large_neg_int.as_i64(), Some(i64::min_value()));
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_large_negative_integer = 0;
    }
    #[test]
    fn test_as_i64_with_f64_max_value() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_f64_max_value = 0;
        let max_f64_as_int = Number::from_f64(f64::MAX).unwrap();
        debug_assert_eq!(max_f64_as_int.as_i64(), None);
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_f64_max_value = 0;
    }
    #[test]
    fn test_as_i64_with_f64_min_value() {
        let _rug_st_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_f64_min_value = 0;
        let min_f64_as_int = Number::from_f64(f64::MIN).unwrap();
        debug_assert_eq!(min_f64_as_int.as_i64(), None);
        let _rug_ed_tests_llm_16_535_rrrruuuugggg_test_as_i64_with_f64_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_536 {
    use crate::value::Number;
    #[test]
    fn test_as_u64() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_as_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = 42i64;
        let rug_fuzz_2 = 42.0f64;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 0u64;
        let rug_fuzz_5 = 0.0f64;
        let pos_int = Number::from(rug_fuzz_0);
        debug_assert_eq!(pos_int.as_u64(), Some(42u64));
        let neg_int = Number::from(-rug_fuzz_1);
        debug_assert_eq!(neg_int.as_u64(), None);
        let float = Number::from_f64(rug_fuzz_2).unwrap();
        debug_assert_eq!(float.as_u64(), None);
        let max_u64 = Number::from(u64::MAX);
        debug_assert_eq!(max_u64.as_u64(), Some(u64::MAX));
        let min_i64 = Number::from(i64::MIN);
        debug_assert_eq!(min_i64.as_u64(), None);
        let big_float = Number::from_f64((u64::MAX as f64) + rug_fuzz_3).unwrap();
        debug_assert_eq!(big_float.as_u64(), None);
        let zero = Number::from(rug_fuzz_4);
        debug_assert_eq!(zero.as_u64(), Some(0u64));
        let neg_zero = Number::from_f64(-rug_fuzz_5).unwrap();
        debug_assert_eq!(neg_zero.as_u64(), None);
        let max_as_float = Number::from_f64(u64::MAX as f64).unwrap();
        debug_assert_eq!(max_as_float.as_u64(), None);
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_as_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_537 {
    use crate::Number;
    #[test]
    fn test_from_f32_finite() {
        let _rug_st_tests_llm_16_537_rrrruuuugggg_test_from_f32_finite = 0;
        let rug_fuzz_0 = 3.14f32;
        let finite_f32 = rug_fuzz_0;
        if let Some(number) = Number::from_f32(finite_f32) {
            #[cfg(not(feature = "arbitrary_precision"))]
            debug_assert_eq!(number.as_f64(), Some(finite_f32 as f64));
            #[cfg(feature = "arbitrary_precision")]
            debug_assert_eq!(number.as_string(), finite_f32.to_string());
        } else {
            panic!("from_f32 failed to create a Number from a finite f32");
        }
        let _rug_ed_tests_llm_16_537_rrrruuuugggg_test_from_f32_finite = 0;
    }
    #[test]
    fn test_from_f32_infinite() {
        let _rug_st_tests_llm_16_537_rrrruuuugggg_test_from_f32_infinite = 0;
        let infinite_f32 = f32::INFINITY;
        debug_assert!(Number::from_f32(infinite_f32).is_none());
        let _rug_ed_tests_llm_16_537_rrrruuuugggg_test_from_f32_infinite = 0;
    }
    #[test]
    fn test_from_f32_nan() {
        let _rug_st_tests_llm_16_537_rrrruuuugggg_test_from_f32_nan = 0;
        let nan_f32 = f32::NAN;
        debug_assert!(Number::from_f32(nan_f32).is_none());
        let _rug_ed_tests_llm_16_537_rrrruuuugggg_test_from_f32_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_538 {
    use crate::Number;
    use std::f64;
    #[test]
    fn test_from_f64_finite() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_finite = 0;
        let rug_fuzz_0 = 256.0;
        let finite_number = rug_fuzz_0;
        debug_assert!(Number::from_f64(finite_number).is_some());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_finite = 0;
    }
    #[test]
    fn test_from_f64_infinite() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_infinite = 0;
        let infinity = f64::INFINITY;
        debug_assert!(Number::from_f64(infinity).is_none());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_infinite = 0;
    }
    #[test]
    fn test_from_f64_neg_infinite() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_neg_infinite = 0;
        let neg_infinity = f64::NEG_INFINITY;
        debug_assert!(Number::from_f64(neg_infinity).is_none());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_neg_infinite = 0;
    }
    #[test]
    fn test_from_f64_nan() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_nan = 0;
        let nan = f64::NAN;
        debug_assert!(Number::from_f64(nan).is_none());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_nan = 0;
    }
    #[test]
    fn test_from_f64_zero() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_zero = 0;
        let rug_fuzz_0 = 0.0;
        let zero = rug_fuzz_0;
        debug_assert!(Number::from_f64(zero).is_some());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_zero = 0;
    }
    #[test]
    fn test_from_f64_negative() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_negative = 0;
        let rug_fuzz_0 = 123.45;
        let negative = -rug_fuzz_0;
        debug_assert!(Number::from_f64(negative).is_some());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_negative = 0;
    }
    #[test]
    fn test_from_f64_subnormal() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_from_f64_subnormal = 0;
        let rug_fuzz_0 = 2.0;
        let subnormal = f64::MIN_POSITIVE / rug_fuzz_0;
        debug_assert!(Number::from_f64(subnormal).is_some());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_from_f64_subnormal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_539 {
    use crate::Number;
    #[cfg(not(feature = "arbitrary_precision"))]
    #[test]
    fn test_is_f64_for_finite_float() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_finite_float = 0;
        let rug_fuzz_0 = 256.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 256.0;
        let finite_float = Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert!(finite_float.is_f64());
        let zero = Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert!(zero.is_f64());
        let negative = Number::from_f64(-rug_fuzz_2).unwrap();
        debug_assert!(negative.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_finite_float = 0;
    }
    #[cfg(not(feature = "arbitrary_precision"))]
    #[test]
    fn test_is_f64_for_posint() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_posint = 0;
        let rug_fuzz_0 = 256u64;
        let posint = Number::from(rug_fuzz_0);
        debug_assert!(! posint.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_posint = 0;
    }
    #[cfg(not(feature = "arbitrary_precision"))]
    #[test]
    fn test_is_f64_for_negint() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_negint = 0;
        let rug_fuzz_0 = 256i64;
        let negint = Number::from(-rug_fuzz_0);
        debug_assert!(! negint.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_negint = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_is_f64_for_finite_float_ap() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_finite_float_ap = 0;
        let rug_fuzz_0 = 256.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 256.0;
        let finite_float = Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert!(finite_float.is_f64());
        let zero = Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert!(zero.is_f64());
        let negative = Number::from_f64(-rug_fuzz_2).unwrap();
        debug_assert!(negative.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_finite_float_ap = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_is_f64_for_non_finite_float_ap() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_non_finite_float_ap = 0;
        let inf = Number::from_f64(std::f64::INFINITY).unwrap();
        debug_assert!(! inf.is_f64());
        let neg_inf = Number::from_f64(std::f64::NEG_INFINITY).unwrap();
        debug_assert!(! neg_inf.is_f64());
        let nan = Number::from_f64(std::f64::NAN).unwrap();
        debug_assert!(! nan.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_non_finite_float_ap = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_is_f64_for_posint_ap() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_posint_ap = 0;
        let rug_fuzz_0 = 256u64;
        let posint = Number::from(rug_fuzz_0);
        debug_assert!(! posint.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_posint_ap = 0;
    }
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_is_f64_for_negint_ap() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_negint_ap = 0;
        let rug_fuzz_0 = 256i64;
        let negint = Number::from(-rug_fuzz_0);
        debug_assert!(! negint.is_f64());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_f64_for_negint_ap = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_540 {
    use crate::number::{Number, N};
    #[test]
    fn test_is_i64_with_pos_int_within_i64_range() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_pos_int_within_i64_range = 0;
        let num = Number {
            n: N::PosInt(i64::max_value() as u64),
        };
        debug_assert!(num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_pos_int_within_i64_range = 0;
    }
    #[test]
    fn test_is_i64_with_pos_int_exceeding_i64_range() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_pos_int_exceeding_i64_range = 0;
        let rug_fuzz_0 = 1;
        let num = Number {
            n: N::PosInt(i64::max_value() as u64 + rug_fuzz_0),
        };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_pos_int_exceeding_i64_range = 0;
    }
    #[test]
    fn test_is_i64_with_neg_int() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_neg_int = 0;
        let num = Number {
            n: N::NegInt(i64::min_value()),
        };
        debug_assert!(num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_neg_int = 0;
    }
    #[test]
    fn test_is_i64_with_float() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_float = 0;
        let rug_fuzz_0 = 0.0;
        let num = Number { n: N::Float(rug_fuzz_0) };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_float = 0;
    }
    #[test]
    fn test_is_i64_with_neg_float() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_neg_float = 0;
        let rug_fuzz_0 = 1.0;
        let num = Number { n: N::Float(-rug_fuzz_0) };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_neg_float = 0;
    }
    #[test]
    fn test_is_i64_with_pos_float() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_pos_float = 0;
        let rug_fuzz_0 = 1.0;
        let num = Number { n: N::Float(rug_fuzz_0) };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_pos_float = 0;
    }
    #[test]
    fn test_is_i64_with_large_float() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_large_float = 0;
        let num = Number { n: N::Float(f64::MAX) };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_large_float = 0;
    }
    #[test]
    fn test_is_i64_with_nan() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_nan = 0;
        let num = Number { n: N::Float(f64::NAN) };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_nan = 0;
    }
    #[test]
    fn test_is_i64_with_infinity() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_infinity = 0;
        let num = Number {
            n: N::Float(f64::INFINITY),
        };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_infinity = 0;
    }
    #[test]
    fn test_is_i64_with_neg_infinity() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_neg_infinity = 0;
        let num = Number {
            n: N::Float(f64::NEG_INFINITY),
        };
        debug_assert!(! num.is_i64());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_i64_with_neg_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_541 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_u64_pos_int() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_pos_int = 0;
        let rug_fuzz_0 = 42_u64;
        let n = Number::from(rug_fuzz_0);
        debug_assert!(n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_pos_int = 0;
    }
    #[test]
    fn test_is_u64_neg_int() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_neg_int = 0;
        let rug_fuzz_0 = 42_i64;
        let n = Number::from(-rug_fuzz_0);
        debug_assert!(! n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_neg_int = 0;
    }
    #[test]
    fn test_is_u64_float() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_float = 0;
        let rug_fuzz_0 = 42.0;
        let n = Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert!(! n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_float = 0;
    }
    #[test]
    fn test_is_u64_large_float() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_large_float = 0;
        let rug_fuzz_0 = 1.1;
        let n = Number::from_f64(u64::MAX as f64 * rug_fuzz_0).unwrap();
        debug_assert!(! n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_large_float = 0;
    }
    #[test]
    fn test_is_u64_zero_float() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_zero_float = 0;
        let rug_fuzz_0 = 0.0;
        let n = Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert!(! n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_zero_float = 0;
    }
    #[test]
    fn test_is_u64_max_u64() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_max_u64 = 0;
        let n = Number::from(u64::MAX);
        debug_assert!(n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_max_u64 = 0;
    }
    #[test]
    fn test_is_u64_i64_max() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_i64_max = 0;
        let n = Number::from(i64::MAX);
        debug_assert!(n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_i64_max = 0;
    }
    #[test]
    fn test_is_u64_i64_min() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_u64_i64_min = 0;
        let n = Number::from(i64::MIN);
        debug_assert!(! n.is_u64());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_u64_i64_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_542_llm_16_542 {
    use super::*;
    use crate::*;
    use serde::de::Unexpected;
    #[test]
    fn test_unexpected_pos_int() {
        let _rug_st_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_pos_int = 0;
        let rug_fuzz_0 = 42u64;
        let num = Number::from(rug_fuzz_0);
        debug_assert_eq!(num.unexpected(), Unexpected::Unsigned(42u64));
        let _rug_ed_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_pos_int = 0;
    }
    #[test]
    fn test_unexpected_neg_int() {
        let _rug_st_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_neg_int = 0;
        let rug_fuzz_0 = 42i64;
        let num = Number::from(-rug_fuzz_0);
        debug_assert_eq!(num.unexpected(), Unexpected::Signed(- 42i64));
        let _rug_ed_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_neg_int = 0;
    }
    #[test]
    fn test_unexpected_float() {
        let _rug_st_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_float = 0;
        let rug_fuzz_0 = 42.0;
        let num = Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert_eq!(num.unexpected(), Unexpected::Float(42.0));
        let _rug_ed_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_float = 0;
    }
    #[test]
    fn test_unexpected_for_eq() {
        let _rug_st_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_for_eq = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = 42u64;
        let rug_fuzz_2 = 42i64;
        let num1 = Number::from(rug_fuzz_0);
        let num2 = Number::from(rug_fuzz_1);
        let num3 = Number::from(-rug_fuzz_2);
        debug_assert_eq!(num1.unexpected(), num2.unexpected());
        debug_assert_ne!(num1.unexpected(), num3.unexpected());
        let _rug_ed_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_for_eq = 0;
    }
    #[test]
    fn test_unexpected_for_hash() {
        let _rug_st_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_for_hash = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = 42u64;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let num = Number::from(rug_fuzz_0);
        let unexpected = num.unexpected();
        let mut hasher = DefaultHasher::new();
        num.hash(&mut hasher);
        let hash1 = hasher.finish();
        let num2 = Number::from(rug_fuzz_1);
        let mut hasher = DefaultHasher::new();
        num2.hash(&mut hasher);
        let hash2 = hasher.finish();
        debug_assert_eq!(hash1, hash2);
        let _rug_ed_tests_llm_16_542_llm_16_542_rrrruuuugggg_test_unexpected_for_hash = 0;
    }
}
