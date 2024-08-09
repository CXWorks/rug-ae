use std::iter::FromIterator;
use std::str::FromStr;

use toml_datetime::*;

use crate::key::Key;
use crate::parser;
use crate::repr::{Decor, Formatted};
use crate::{Array, InlineTable, InternalString, RawString};

/// Representation of a TOML Value (as part of a Key/Value Pair).
#[derive(Debug, Clone)]
pub enum Value {
    /// A string value.
    String(Formatted<String>),
    /// A 64-bit integer value.
    Integer(Formatted<i64>),
    /// A 64-bit float value.
    Float(Formatted<f64>),
    /// A boolean value.
    Boolean(Formatted<bool>),
    /// An RFC 3339 formatted date-time with offset.
    Datetime(Formatted<Datetime>),
    /// An inline array of values.
    Array(Array),
    /// An inline table of key/value pairs.
    InlineTable(InlineTable),
}

/// Downcasting
impl Value {
    /// Text description of value type
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(..) => "string",
            Value::Integer(..) => "integer",
            Value::Float(..) => "float",
            Value::Boolean(..) => "boolean",
            Value::Datetime(..) => "datetime",
            Value::Array(..) => "array",
            Value::InlineTable(..) => "inline table",
        }
    }

    /// Casts `self` to str.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a string.
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Casts `self` to integer.
    pub fn as_integer(&self) -> Option<i64> {
        match *self {
            Value::Integer(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is an integer.
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    /// Casts `self` to float.
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a float.
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    /// Casts `self` to boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Boolean(ref value) => Some(*value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a boolean.
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// Casts `self` to date-time.
    pub fn as_datetime(&self) -> Option<&Datetime> {
        match *self {
            Value::Datetime(ref value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns true iff `self` is a date-time.
    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }

    /// Casts `self` to array.
    pub fn as_array(&self) -> Option<&Array> {
        match *self {
            Value::Array(ref value) => Some(value),
            _ => None,
        }
    }

    /// Casts `self` to mutable array.
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match *self {
            Value::Array(ref mut value) => Some(value),
            _ => None,
        }
    }

    /// Returns true iff `self` is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Casts `self` to inline table.
    pub fn as_inline_table(&self) -> Option<&InlineTable> {
        match *self {
            Value::InlineTable(ref value) => Some(value),
            _ => None,
        }
    }

    /// Casts `self` to mutable inline table.
    pub fn as_inline_table_mut(&mut self) -> Option<&mut InlineTable> {
        match *self {
            Value::InlineTable(ref mut value) => Some(value),
            _ => None,
        }
    }

    /// Returns true iff `self` is an inline table.
    pub fn is_inline_table(&self) -> bool {
        self.as_inline_table().is_some()
    }
}

impl Value {
    /// Get the decoration of the value.
    /// # Example
    /// ```rust
    /// let v = toml_edit::Value::from(true);
    /// assert_eq!(v.decor().suffix(), None);
    ///```
    pub fn decor_mut(&mut self) -> &mut Decor {
        match self {
            Value::String(f) => f.decor_mut(),
            Value::Integer(f) => f.decor_mut(),
            Value::Float(f) => f.decor_mut(),
            Value::Boolean(f) => f.decor_mut(),
            Value::Datetime(f) => f.decor_mut(),
            Value::Array(a) => a.decor_mut(),
            Value::InlineTable(t) => t.decor_mut(),
        }
    }

    /// Get the decoration of the value.
    /// # Example
    /// ```rust
    /// let v = toml_edit::Value::from(true);
    /// assert_eq!(v.decor().suffix(), None);
    ///```
    pub fn decor(&self) -> &Decor {
        match *self {
            Value::String(ref f) => f.decor(),
            Value::Integer(ref f) => f.decor(),
            Value::Float(ref f) => f.decor(),
            Value::Boolean(ref f) => f.decor(),
            Value::Datetime(ref f) => f.decor(),
            Value::Array(ref a) => a.decor(),
            Value::InlineTable(ref t) => t.decor(),
        }
    }

    /// Sets the prefix and the suffix for value.
    /// # Example
    /// ```rust
    /// let mut v = toml_edit::Value::from(42);
    /// assert_eq!(&v.to_string(), "42");
    /// let d = v.decorated(" ", " ");
    /// assert_eq!(&d.to_string(), " 42 ");
    /// ```
    pub fn decorated(mut self, prefix: impl Into<RawString>, suffix: impl Into<RawString>) -> Self {
        self.decorate(prefix, suffix);
        self
    }

    pub(crate) fn decorate(&mut self, prefix: impl Into<RawString>, suffix: impl Into<RawString>) {
        let decor = self.decor_mut();
        *decor = Decor::new(prefix, suffix);
    }

    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        match self {
            Value::String(f) => f.span(),
            Value::Integer(f) => f.span(),
            Value::Float(f) => f.span(),
            Value::Boolean(f) => f.span(),
            Value::Datetime(f) => f.span(),
            Value::Array(a) => a.span(),
            Value::InlineTable(t) => t.span(),
        }
    }

    pub(crate) fn despan(&mut self, input: &str) {
        match self {
            Value::String(f) => f.despan(input),
            Value::Integer(f) => f.despan(input),
            Value::Float(f) => f.despan(input),
            Value::Boolean(f) => f.despan(input),
            Value::Datetime(f) => f.despan(input),
            Value::Array(a) => a.despan(input),
            Value::InlineTable(t) => t.despan(input),
        }
    }
}

impl FromStr for Value {
    type Err = crate::TomlError;

    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse_value(s)
    }
}

impl<'b> From<&'b Value> for Value {
    fn from(s: &'b Value) -> Self {
        s.clone()
    }
}

impl<'b> From<&'b str> for Value {
    fn from(s: &'b str) -> Self {
        s.to_owned().into()
    }
}

impl<'b> From<&'b String> for Value {
    fn from(s: &'b String) -> Self {
        s.to_owned().into()
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(Formatted::new(s))
    }
}

impl<'b> From<&'b InternalString> for Value {
    fn from(s: &'b InternalString) -> Self {
        s.as_str().into()
    }
}

impl From<InternalString> for Value {
    fn from(s: InternalString) -> Self {
        s.as_str().into()
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(Formatted::new(i))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(Formatted::new(f))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(Formatted::new(b))
    }
}

impl From<Datetime> for Value {
    fn from(d: Datetime) -> Self {
        Value::Datetime(Formatted::new(d))
    }
}

impl From<Date> for Value {
    fn from(d: Date) -> Self {
        let d: Datetime = d.into();
        d.into()
    }
}

impl From<Time> for Value {
    fn from(d: Time) -> Self {
        let d: Datetime = d.into();
        d.into()
    }
}

impl From<Array> for Value {
    fn from(array: Array) -> Self {
        Value::Array(array)
    }
}

impl From<InlineTable> for Value {
    fn from(table: InlineTable) -> Self {
        Value::InlineTable(table)
    }
}

impl<V: Into<Value>> FromIterator<V> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        let array: Array = iter.into_iter().collect();
        Value::Array(array)
    }
}

impl<K: Into<Key>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let table: InlineTable = iter.into_iter().collect();
        Value::InlineTable(table)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, None, ("", ""))
    }
}

// `key1 = value1`
pub(crate) const DEFAULT_VALUE_DECOR: (&str, &str) = (" ", "");
// `{ key = value }`
pub(crate) const DEFAULT_TRAILING_VALUE_DECOR: (&str, &str) = (" ", " ");
// `[value1, value2]`
pub(crate) const DEFAULT_LEADING_VALUE_DECOR: (&str, &str) = ("", "");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_iter_formatting() {
        let features = vec!["node".to_owned(), "mouth".to_owned()];
        let features: Value = features.iter().cloned().collect();
        assert_eq!(features.to_string(), r#"["node", "mouth"]"#);
    }
}
#[cfg(test)]
mod tests_llm_16_121 {
    use super::*;

use crate::*;
    use crate::internal_string::InternalString;
    use crate::value::Value;
    use std::convert::From;

    #[test]
    fn from_internal_string() {
        let internal_str = InternalString::from("test");
        let result_value: Value = Value::from(&internal_str);
        assert_eq!(result_value.as_str(), Some("test"));
    }
}#[cfg(test)]
mod tests_llm_16_122 {
    use super::*;

use crate::*;
    use std::convert::From;

    #[test]
    fn test_from_str_ref() {
        let s = String::from("test");
        let val = Value::from(&s);
        assert_eq!(val.as_str(), Some("test"));
    }
    
    #[test]
    fn test_from_string() {
        let s = String::from("ownership");
        let val = Value::from(s.clone());
        assert_eq!(val.as_str(), Some("ownership"));
    }
}#[cfg(test)]
mod tests_llm_16_123 {
    use super::*;

use crate::*;
    use std::convert::From;
    
    #[test]
    fn from_str_creates_string_value() {
        let val = Value::from("hello");
        assert!(val.is_str());
        assert_eq!(val.as_str(), Some("hello"));
    }

    #[test]
    fn from_str_preserves_lifetime() {
        let input = "world".to_string();
        let val = Value::from(input.as_str());
        assert_eq!(val.as_str(), Some("world"));
    }

    #[test]
    fn from_str_creates_owned_value() {
        let input = "owned value";
        let val = Value::from(input);
        assert_eq!(val.as_str(), Some("owned value"));
    }

    #[test]
    fn from_str_empty_string() {
        let val = Value::from("");
        assert!(val.is_str());
        assert_eq!(val.as_str(), Some(""));
    }

    #[test]
    fn from_str_with_whitespace() {
        let val = Value::from(" string ");
        assert_eq!(val.as_str(), Some(" string "));
    }
}#[cfg(test)]
mod tests_llm_16_124 {
    use super::*;

use crate::*;
    use crate::{Array, InlineTable, Value};

    #[test]
    fn test_from_value_ref() {
        let original_value = Value::from(42);
        let cloned_value: Value = Value::from(&original_value);
        assert_eq!(cloned_value.as_integer(), Some(42));
        
        let original_value = Value::from(3.14);
        let cloned_value: Value = Value::from(&original_value);
        assert_eq!(cloned_value.as_float(), Some(3.14));
        
        let original_value = Value::from("Hello, World!");
        let cloned_value: Value = Value::from(&original_value);
        assert_eq!(cloned_value.as_str(), Some("Hello, World!"));
        
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        let original_value = Value::from(array);
        let cloned_value: Value = Value::from(&original_value);
        assert_eq!(cloned_value.as_array().and_then(|a| a.get(0).and_then(|v| v.as_integer())), Some(1));
        assert_eq!(cloned_value.as_array().and_then(|a| a.get(1).and_then(|v| v.as_integer())), Some(2));
        
        let mut table = InlineTable::new();
        table.insert("key", Value::from("value"));
        let original_value = Value::from(table);
        let cloned_value: Value = Value::from(&original_value);
        assert_eq!(cloned_value.as_inline_table().and_then(|t| t.get("key").and_then(|v| v.as_str())), Some("value"));
    }
}#[cfg(test)]
mod tests_llm_16_125 {
    use super::*;

use crate::*;

    #[test]
    fn test_from_array_to_value() {
        let mut array = Array::new();
        array.push(1);
        array.push("two");
        array.push(3.0);
        let value: Value = Value::from(array);
        match value {
            Value::Array(a) => {
                assert_eq!(a.len(), 3);
                assert_eq!(a.get(0).unwrap().as_integer(), Some(1));
                assert_eq!(a.get(1).unwrap().as_str(), Some("two"));
                assert_eq!(a.get(2).unwrap().as_float(), Some(3.0));
            }
            _ => panic!("Value is not an Array"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_126 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_bool() {
        let val_true: Value = Value::from(true);
        let val_false: Value = Value::from(false);
        
        if let Value::Boolean(formatted_bool) = val_true {
            assert_eq!(*formatted_bool.value(), true);
        } else {
            panic!("Value::from(true) did not produce a Value::Boolean");
        }

        if let Value::Boolean(formatted_bool) = val_false {
            assert_eq!(*formatted_bool.value(), false);
        } else {
            panic!("Value::from(false) did not produce a Value::Boolean");
        }
    }
}#[cfg(test)]
mod tests_llm_16_127 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use std::convert::From;

    #[test]
    fn test_value_from_f64() {
        let num = 42f64;
        let value = Value::from(num);
        if let Value::Float(f) = value {
            assert_eq!(*f.value(), num);
        } else {
            panic!("Value::from did not produce a Value::Float");
        }
    }
}#[cfg(test)]
mod tests_llm_16_128 {
    use super::*;

use crate::*;
    use std::convert::From;

    #[test]
    fn from_i64_creates_integer_value() {
        let num = 42i64;
        let value = Value::from(num);
        match value {
            Value::Integer(formatted) => {
                assert_eq!(*formatted.value(), num);
            }
            _ => panic!("Value::from(i64) did not produce an Integer variant"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_129 {
    use crate::*;

    #[test]
    fn from_inline_table_to_value() {
        let mut table = InlineTable::new();
        table.insert("key", Value::from(42));
        let value_from_table = Value::from(table.clone());
        if let Value::InlineTable(v) = value_from_table {
            assert_eq!(v.len(), table.len());
            assert_eq!(v.iter().count(), table.iter().count());
            assert_eq!(v.get("key").unwrap().as_integer(), Some(42));
        } else {
            panic!("Value::from(InlineTable) did not produce a Value::InlineTable");
        }
    }
}#[cfg(test)]
mod tests_llm_16_130 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use crate::internal_string::InternalString;

    #[test]
    fn from_internal_string_creates_string_value() {
        let internal_string = InternalString::from("test");
        let value: Value = <Value as From<InternalString>>::from(internal_string.clone());
        if let Value::String(formatted_string) = value {
            assert_eq!(formatted_string.value(), internal_string.as_str());
        } else {
            panic!("Value created from InternalString must be of type Value::String");
        }
    }
}#[cfg(test)]
mod tests_llm_16_131 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use std::convert::From;
    
    #[test]
    fn value_from_string() {
        let input = String::from("test_string");
        let value: Value = Value::from(input.clone());
        match value {
            Value::String(formatted) => assert_eq!(formatted.value(), &input),
            _ => panic!("Value is not of type String"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_135_llm_16_135 {
    use super::*;

use crate::*;
    use crate::Value;
    use std::iter::FromIterator;
    
    #[test]
    fn test_from_iter_empty() {
        let empty: Vec<(String, Value)> = Vec::new();
        let table: Value = Value::from_iter(empty);
        assert!(matches!(table, Value::InlineTable(t) if t.is_empty()));
    }

    #[test]
    fn test_from_iter_with_items() {
        let items = vec![
            ("key1".to_string(), Value::from(true)),
            ("key2".to_string(), Value::from(42)),
        ];
        let table: Value = Value::from_iter(items);
        if let Value::InlineTable(t) = table {
            assert_eq!(2, t.len());
            assert_eq!(t.get("key1").and_then(|v|v.as_bool()), Some(true));
            assert_eq!(t.get("key2").and_then(|v|v.as_integer()), Some(42));
        } else {
            panic!("Value::InlineTable expected");
        }
    }
}#[cfg(test)]
mod tests_llm_16_136 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn from_iter_with_integers() {
        let values = vec![1, 2, 3, 4, 5];
        let value = Value::from_iter(values);
        assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            assert_eq!(array.len(), 5);
            for (i, v) in array.iter().enumerate() {
                assert_eq!(v.as_integer(), Some((i + 1) as i64));
            }
        } else {
            panic!("Value is not an array");
        }
    }

    #[test]
    fn from_iter_with_strings() {
        let values = vec!["rust", "cargo", "toml"];
        let value = Value::from_iter(values);
        assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            assert_eq!(array.len(), 3);
            let expected = ["rust", "cargo", "toml"];
            for (i, v) in array.iter().enumerate() {
                assert_eq!(v.as_str(), Some(expected[i]));
            }
        } else {
            panic!("Value is not an array");
        }
    }

    #[test]
    fn from_iter_with_mixed_types() {
        let values = vec![Value::from(42), Value::from("example"), Value::from(true)];
        let value = Value::from_iter(values);
        assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            assert_eq!(array.len(), 3);
            assert_eq!(array.get(0).unwrap().as_integer(), Some(42));
            assert_eq!(array.get(1).unwrap().as_str(), Some("example"));
            assert_eq!(array.get(2).unwrap().as_bool(), Some(true));
        } else {
            panic!("Value is not an array");
        }
    }

    #[test]
    fn from_iter_empty() {
        let values: Vec<Value> = Vec::new();
        let value = Value::from_iter(values);
        assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            assert!(array.is_empty());
        } else {
            panic!("Value is not an array");
        }
    }
}#[cfg(test)]
mod tests_llm_16_137 {
    use super::*;

use crate::*;
    use std::str::FromStr;
    use crate::Value;

    #[test]
    fn test_from_str_valid_input() {
        let valid_str = r#""Hello, World!""#;
        let value = Value::from_str(valid_str).unwrap();
        assert_eq!(value.as_str(), Some("Hello, World!"));
    }

    #[test]
    fn test_from_str_invalid_input() {
        let invalid_str = "Not a TOML value";
        let result = Value::from_str(invalid_str);
        assert!(result.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_521 {
    use crate::{Array, Value};

    #[test]
    fn test_as_array_on_array_value() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        let value = Value::Array(array);

        let array_ref = value.as_array().expect("Value should be an array");
        assert_eq!(array_ref.len(), 2);
        assert_eq!(array_ref.get(0).and_then(|v| v.as_integer()), Some(1));
        assert_eq!(array_ref.get(1).and_then(|v| v.as_integer()), Some(2));
    }

    #[test]
    fn test_as_array_on_non_array_value() {
        let value = Value::from(42);
        assert!(value.as_array().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_522 {
    use super::*;

use crate::*;
    use crate::array::Array;
    use crate::value::Value;

    #[test]
    fn test_as_array_mut_some() {
        let mut value = Value::Array(Array::new());
        assert!(value.as_array_mut().is_some());
    }

    #[test]
    fn test_as_array_mut_none() {
        let mut value = Value::String(Formatted::new(String::from("test")));
        assert!(value.as_array_mut().is_none());
    }

    #[test]
    fn test_as_array_mut_mutate() {
        let mut value = Value::Array(Array::new());
        if let Some(array) = value.as_array_mut() {
            array.push(42);
        }
        assert_eq!(value.as_array().unwrap().len(), 1);
    }
}#[cfg(test)]
mod tests_llm_16_523 {
    use super::*;

use crate::*;

    #[test]
    fn test_as_bool_success() {
        // Check as_bool when the value is actually a boolean
        let value_bool = Value::Boolean(Formatted::new(true));
        assert_eq!(value_bool.as_bool(), Some(true));
    }

    #[test]
    fn test_as_bool_failure() {
        // Check as_bool when the value is not a boolean
        let value_int = Value::Integer(Formatted::new(42));
        let value_str = Value::String(Formatted::new(String::from("true")));

        assert_eq!(value_int.as_bool(), None);
        assert_eq!(value_str.as_bool(), None);
    }
}#[cfg(test)]
mod tests_llm_16_524 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use crate::Datetime;

    #[test]
    fn test_as_datetime() {
        // Test with a datetime
        let datetime_str = "1979-05-27T07:32:00Z";
        let datetime = datetime_str.parse::<Datetime>().unwrap();
        let value = Value::Datetime(Formatted::new(datetime));
        assert_eq!(value.as_datetime().map(|dt| dt.to_string()), Some(datetime_str.to_string()));

        // Test with a string
        let string_value = Value::String(Formatted::new("Not a datetime".to_string()));
        assert_eq!(string_value.as_datetime(), None);

        // Test with an integer
        let int_value = Value::Integer(Formatted::new(42));
        assert_eq!(int_value.as_datetime(), None);

        // Test with a float
        let float_value = Value::Float(Formatted::new(3.14));
        assert_eq!(float_value.as_datetime(), None);

        // Test with a boolean
        let bool_value = Value::Boolean(Formatted::new(true));
        assert_eq!(bool_value.as_datetime(), None);

        // Test with an array
        let array_value = Value::Array(Array::new());
        assert_eq!(array_value.as_datetime(), None);

        // Test with an inline table
        let table_value = Value::InlineTable(InlineTable::new());
        assert_eq!(table_value.as_datetime(), None);
    }
}#[cfg(test)]
mod tests_llm_16_525 {
    use super::*;

use crate::*;

    #[test]
    fn test_as_float() {
        let float_value = Value::Float(Formatted::new(42.0));
        assert_eq!(float_value.as_float(), Some(42.0));

        let int_value = Value::Integer(Formatted::new(42));
        assert_eq!(int_value.as_float(), None);

        let str_value = Value::String(Formatted::new(String::from("42")));
        assert_eq!(str_value.as_float(), None);

        let bool_value = Value::Boolean(Formatted::new(true));
        assert_eq!(bool_value.as_float(), None);

        let datetime_value = Value::Datetime(Formatted::new("1979-05-27T07:32:00Z".parse().unwrap()));
        assert_eq!(datetime_value.as_float(), None);

        let array_value = Value::Array(Array::new());
        assert_eq!(array_value.as_float(), None);

        let inline_table_value = Value::InlineTable(InlineTable::new());
        assert_eq!(inline_table_value.as_float(), None);
    }
}#[cfg(test)]
mod tests_llm_16_527 {
    use super::*;

use crate::*;
    use crate::{InlineTable, Value};

    #[test]
    fn as_inline_table_mut_some() {
        let mut table = InlineTable::new();
        table.insert("hello", Value::from("world"));

        let mut value = Value::from(table);
        let inline_table_mut = value.as_inline_table_mut();
        assert!(inline_table_mut.is_some());
    }

    #[test]
    fn as_inline_table_mut_none() {
        let mut value = Value::from("not a table");
        let inline_table_mut = value.as_inline_table_mut();
        assert!(inline_table_mut.is_none());
    }
}#[cfg(test)]
mod tests_llm_16_528_llm_16_528 {
    use crate::repr::Formatted;
    use crate::value::Value;

    #[test]
    fn as_integer_some() {
        // Previous: let value = Value::Integer(42.into());
        // New: use the `Formatted::new` constructor
        let value = Value::Integer(Formatted::new(42));
        assert_eq!(value.as_integer(), Some(42));
    }

    #[test]
    fn as_integer_none() {
        // Previous: let value = Value::String("Hello".into());
        // New: use the `Formatted::new` constructor
        let value = Value::String(Formatted::new(String::from("Hello")));
        assert_eq!(value.as_integer(), None);
    }
}#[cfg(test)]
mod tests_llm_16_529 {
    use super::*;

use crate::*;
    use crate::value::Value;

    #[test]
    fn test_as_str_string_value() {
        let value = Value::String(Formatted::new("test".to_string()));
        assert_eq!(value.as_str(), Some("test"));
    }

    #[test]
    fn test_as_str_non_string_value() {
        let value = Value::Integer(Formatted::new(42));
        assert_eq!(value.as_str(), None);
    }
}#[cfg(test)]
mod tests_llm_16_530_llm_16_530 {
    use crate::{Array, Datetime, Decor, InlineTable, RawString, Value};

    #[test]
    fn test_decor_string() {
        let v = Value::from("test");
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_integer() {
        let v = Value::from(42);
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_float() {
        let v = Value::from(3.14);
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_boolean() {
        let v = Value::from(true);
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_datetime() {
        let v = Value::from("2020-05-02T07:30:00Z".parse::<Datetime>().unwrap());
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_array() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        let v = Value::from(arr);
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_inline_table() {
        let mut tbl = InlineTable::new();
        tbl.insert("key", Value::from(3.14));
        let v = Value::from(tbl);
        assert_eq!(v.decor().prefix(), None);
        assert_eq!(v.decor().suffix(), None);
    }

    #[test]
    fn test_decor_custom() {
        let mut v = Value::from("test");
        v.decorate(RawString::from("/* prefix */"), RawString::from("/* suffix */"));
        assert_eq!(v.decor().prefix().unwrap().as_str(), Some("/* prefix */"));
        assert_eq!(v.decor().suffix().unwrap().as_str(), Some("/* suffix */"));
    }
}#[cfg(test)]
mod tests_llm_16_532 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use crate::repr::Decor;
    use crate::raw_string::RawString;

    #[test]
    fn decorate_value_with_prefix_suffix() {
        let mut value = Value::from("example");
        value.decorate("/* ", " */");
        match value {
            Value::String(ref formatted) => {
                assert_eq!(formatted.decor().prefix().unwrap().as_str(), Some("/* "));
                assert_eq!(formatted.decor().suffix().unwrap().as_str(), Some(" */"));
            }
            _ => panic!("Value is not a String"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_533 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use crate::raw_string::RawString;
    
    #[test]
    fn test_decorated() {
        let original = Value::from(42);
        let decorated = original.clone().decorated(" ", " ");
        assert_eq!(decorated.to_string(), " 42 ");
        assert!(decorated.decor().prefix().unwrap().as_str() == Some(" "));
        assert!(decorated.decor().suffix().unwrap().as_str() == Some(" "));
    }

    #[test]
    fn test_decorated_string() {
        let original = Value::from("hello");
        let decorated = original.clone().decorated("//", "**");
        assert_eq!(decorated.to_string(), "\"hello\""); // Note: to_string does not include the prefix and suffix
        assert!(decorated.decor().prefix().unwrap().as_str() == Some("//"));
        assert!(decorated.decor().suffix().unwrap().as_str() == Some("**"));
    }

    #[test]
    fn test_decorated_empty() {
        let original = Value::from("");
        let decorated = original.clone().decorated("", " ");
        assert_eq!(decorated.to_string(), "\"\" "); // Note: to_string prints the actual value with suffix
        assert!(decorated.decor().prefix().unwrap().as_str() == Some(""));
        assert!(decorated.decor().suffix().unwrap().as_str() == Some(" "));
    }
    
    #[test]
    fn test_decorated_array() {
        let original = Value::Array(Array::new());
        let decorated = original.clone().decorated("/*", "*/");
        assert_eq!(decorated.to_string(), "[]");
        assert!(decorated.decor().prefix().unwrap().as_str() == Some("/*"));
        assert!(decorated.decor().suffix().unwrap().as_str() == Some("*/"));
    }
    
    #[test]
    fn test_decorated_inline_table() {
        let original = Value::InlineTable(InlineTable::new());
        let decorated = original.clone().decorated("/*", "*/");
        assert_eq!(decorated.to_string(), "{}");
        assert!(decorated.decor().prefix().unwrap().as_str() == Some("/*"));
        assert!(decorated.decor().suffix().unwrap().as_str() == Some("*/"));
    }
    
    #[test]
    fn test_decorated_with_raw_string() {
        let prefix_rs = RawString::from("/*");
        let suffix_rs = RawString::from("*/");
        let original = Value::from(3.14);
        let decorated = original.clone().decorated(prefix_rs, suffix_rs);
        assert_eq!(decorated.to_string(), "3.14");
        assert!(decorated.decor().prefix().unwrap().as_str() == Some("/*"));
        assert!(decorated.decor().suffix().unwrap().as_str() == Some("*/"));
    }
}#[cfg(test)]
mod tests_llm_16_534 {
    use crate::{value::Value, Array, InlineTable};
    
    #[test]
    fn despan_string() {
        let input = "value";
        let mut val = Value::from("example");
        val.despan(input);
        matches!(val, Value::String(s) if s.span().is_none());
    }
    
    #[test]
    fn despan_integer() {
        let input = "42";
        let mut val = Value::from(42);
        val.despan(input);
        matches!(val, Value::Integer(i) if i.span().is_none());
    }
    
    #[test]
    fn despan_float() {
        let input = "3.14";
        let mut val = Value::from(3.14);
        val.despan(input);
        matches!(val, Value::Float(f) if f.span().is_none());
    }
    
    #[test]
    fn despan_boolean() {
        let input = "false";
        let mut val = Value::from(false);
        val.despan(input);
        matches!(val, Value::Boolean(b) if b.span().is_none());
    }
    
    #[test]
    fn despan_datetime() {
        let input = "2021-04-04T19:49:02Z";
        let mut val = Value::from(input.parse::<crate::Datetime>().unwrap());
        val.despan(input);
        matches!(val, Value::Datetime(dt) if dt.span().is_none());
    }
    
    #[test]
    fn despan_array() {
        let input = "[1, 2, 3]";
        let mut val = Value::from(Array::from_iter(vec![1, 2, 3]));
        val.despan(input);
        matches!(val, Value::Array(a) if a.span().is_none());
    }

    #[test]
    fn despan_inline_table() {
        let input = "{ x = 2, y = 3 }";
        let mut val = Value::from(InlineTable::from_iter(vec![
            ("x", Value::from(2)),
            ("y", Value::from(3)),
        ]));
        val.despan(input);
        matches!(val, Value::InlineTable(it) if it.span().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_537 {
    use super::*;

use crate::*;

    #[test]
    fn test_is_datetime_for_datetime_value() {
        let datetime_str = "1979-05-27T07:32:00Z";
        let datetime = datetime_str.parse::<Datetime>().unwrap();
        let value = Value::Datetime(Formatted::new(datetime));
        assert!(value.is_datetime());
    }

    #[test]
    fn test_is_datetime_for_non_datetime_value() {
        let value = Value::String(Formatted::new("a string".to_string()));
        assert!(!value.is_datetime());

        let value = Value::Integer(Formatted::new(42));
        assert!(!value.is_datetime());

        let value = Value::Float(Formatted::new(3.14));
        assert!(!value.is_datetime());

        let value = Value::Boolean(Formatted::new(true));
        assert!(!value.is_datetime());

        let value = Value::Array(Array::new());
        assert!(!value.is_datetime());

        let value = Value::InlineTable(InlineTable::new());
        assert!(!value.is_datetime());
    }
}#[cfg(test)]
mod tests_llm_16_538 {
    use super::*;

use crate::*;

    #[test]
    fn test_is_float_for_float_value() {
        let value = Value::Float(Formatted::new(1.23));
        assert!(value.is_float());
    }

    #[test]
    fn test_is_float_for_non_float_value() {
        let value = Value::Integer(Formatted::new(123));
        assert!(!value.is_float());
    }
}#[cfg(test)]
mod tests_llm_16_539 {
    use crate::value::Value;
    use crate::inline_table::InlineTable;
    use crate::Array;

    #[test]
    fn test_is_inline_table_with_inline_table() {
        let inline_table = InlineTable::new();
        let value = Value::InlineTable(inline_table);
        assert!(value.is_inline_table());
    }

    #[test]
    fn test_is_inline_table_with_array() {
        let array = Array::new();
        let value = Value::Array(array);
        assert!(!value.is_inline_table());
    }

    // Additional tests can be implemented for other variants if necessary...
}#[cfg(test)]
mod tests_llm_16_540 {
    use crate::value::Value;
    use crate::Formatted;
    use crate::repr::Decor;

    #[test]
    fn test_is_integer() {
        let integer_value = Value::Integer(Formatted::new(42));
        let float_value = Value::Float(Formatted::new(42.0));
        let string_value = Value::String(Formatted::new("42".to_owned()));
        let boolean_value = Value::Boolean(Formatted::new(true));
        let mut array_value = Value::Array(crate::array::Array::new());
        array_value
            .as_array_mut()
            .unwrap()
            .push(Value::Integer(Formatted::new(42)));
        let mut table_value = Value::InlineTable(crate::inline_table::InlineTable::new());
        table_value
            .as_inline_table_mut()
            .unwrap()
            .insert("key", Value::Integer(Formatted::new(42)));

        assert!(integer_value.is_integer());
        assert!(!float_value.is_integer());
        assert!(!string_value.is_integer());
        assert!(!boolean_value.is_integer());
        assert!(!array_value.is_integer());
        assert!(!table_value.is_integer());
    }
}#[cfg(test)]
mod tests_llm_16_541 {
    use crate::value::Value;

    #[test]
    fn test_is_str() {
        let string_value = Value::from("test");
        let int_value = Value::from(42);
        let float_value = Value::from(3.14);
        let bool_value = Value::from(true);
        let array_value = Value::from_iter(vec![1, 2, 3]);
        let table_value = Value::from_iter(vec![("key", "value")]);

        assert!(string_value.is_str());
        assert!(!int_value.is_str());
        assert!(!float_value.is_str());
        assert!(!bool_value.is_str());
        assert!(!array_value.is_str());
        assert!(!table_value.is_str());
    }
}#[cfg(test)]
mod tests_llm_16_542 {
    use super::*;

use crate::*;
    use crate::value::Value;
    use std::ops::Range;

    #[test]
    fn test_value_span_string() {
        let mut value = Value::String(Formatted::new("Hello".to_string()));
        value.span();
    }

    #[test]
    fn test_value_span_integer() {
        let mut value = Value::Integer(Formatted::new(42));
        value.span();
    }

    #[test]
    fn test_value_span_float() {
        let mut value = Value::Float(Formatted::new(3.14));
        value.span();
    }

    #[test]
    fn test_value_span_boolean() {
        let mut value = Value::Boolean(Formatted::new(true));
        value.span();
    }

    #[test]
    fn test_value_span_datetime() {
        let mut value = Value::Datetime(Formatted::new("1979-05-27T07:32:00Z".parse().unwrap()));
        value.span();
    }

    #[test]
    fn test_value_span_array() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        let value = Value::Array(array);
        value.span();
    }

    #[test]
    fn test_value_span_inline_table() {
        let mut inline_table = InlineTable::new();
        inline_table.insert("key", Value::from(42));
        let value = Value::InlineTable(inline_table);
        value.span();
    }
}#[cfg(test)]
mod tests_llm_16_543 {
    use super::*;

use crate::*;

    #[test]
    fn type_name_string() {
        let value = Value::String(Formatted::new(String::from("test")));
        assert_eq!(value.type_name(), "string");
    }

    #[test]
    fn type_name_integer() {
        let value = Value::Integer(Formatted::new(42_i64));
        assert_eq!(value.type_name(), "integer");
    }

    #[test]
    fn type_name_float() {
        let value = Value::Float(Formatted::new(3.14_f64));
        assert_eq!(value.type_name(), "float");
    }

    #[test]
    fn type_name_boolean() {
        let value = Value::Boolean(Formatted::new(true));
        assert_eq!(value.type_name(), "boolean");
    }

    #[test]
    fn type_name_datetime() {
        let value = Value::Datetime(Formatted::new("1979-05-27T07:32:00Z".parse().unwrap()));
        assert_eq!(value.type_name(), "datetime");
    }

    #[test]
    fn type_name_array() {
        let value = Value::Array(Array::new());
        assert_eq!(value.type_name(), "array");
    }

    #[test]
    fn type_name_inline_table() {
        let value = Value::InlineTable(InlineTable::new());
        assert_eq!(value.type_name(), "inline table");
    }
}#[cfg(test)]
mod tests_rug_4 {
    use toml_edit::value::Value;

    #[test]
    fn test_is_datetime() {
        let mut p0 = Value::from("1979-05-27T07:32:00Z");
        assert!(p0.is_datetime());
        
        let mut p0 = Value::from("Sample string value");
        assert!(!p0.is_datetime());
    }
}#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::Value;
    
    #[test]
    fn test_as_array() {
        let mut p0 = Value::Array(vec![
            Value::from("Sample string value"),
            Value::from(42),
        ]);

        assert_eq!(p0.as_array().unwrap().len(), 2);
        assert!(p0.as_array().unwrap().contains(&Value::from("Sample string value")));
        assert!(p0.as_array().unwrap().contains(&Value::from(42)));
    }
}