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
    pub fn decorated(
        mut self,
        prefix: impl Into<RawString>,
        suffix: impl Into<RawString>,
    ) -> Self {
        self.decorate(prefix, suffix);
        self
    }
    pub(crate) fn decorate(
        &mut self,
        prefix: impl Into<RawString>,
        suffix: impl Into<RawString>,
    ) {
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
pub(crate) const DEFAULT_VALUE_DECOR: (&str, &str) = (" ", "");
pub(crate) const DEFAULT_TRAILING_VALUE_DECOR: (&str, &str) = (" ", " ");
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
        let _rug_st_tests_llm_16_121_rrrruuuugggg_from_internal_string = 0;
        let rug_fuzz_0 = "test";
        let internal_str = InternalString::from(rug_fuzz_0);
        let result_value: Value = Value::from(&internal_str);
        debug_assert_eq!(result_value.as_str(), Some("test"));
        let _rug_ed_tests_llm_16_121_rrrruuuugggg_from_internal_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_122 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn test_from_str_ref() {
        let _rug_st_tests_llm_16_122_rrrruuuugggg_test_from_str_ref = 0;
        let rug_fuzz_0 = "test";
        let s = String::from(rug_fuzz_0);
        let val = Value::from(&s);
        debug_assert_eq!(val.as_str(), Some("test"));
        let _rug_ed_tests_llm_16_122_rrrruuuugggg_test_from_str_ref = 0;
    }
    #[test]
    fn test_from_string() {
        let _rug_st_tests_llm_16_122_rrrruuuugggg_test_from_string = 0;
        let rug_fuzz_0 = "ownership";
        let s = String::from(rug_fuzz_0);
        let val = Value::from(s.clone());
        debug_assert_eq!(val.as_str(), Some("ownership"));
        let _rug_ed_tests_llm_16_122_rrrruuuugggg_test_from_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_123 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn from_str_creates_string_value() {
        let _rug_st_tests_llm_16_123_rrrruuuugggg_from_str_creates_string_value = 0;
        let rug_fuzz_0 = "hello";
        let val = Value::from(rug_fuzz_0);
        debug_assert!(val.is_str());
        debug_assert_eq!(val.as_str(), Some("hello"));
        let _rug_ed_tests_llm_16_123_rrrruuuugggg_from_str_creates_string_value = 0;
    }
    #[test]
    fn from_str_preserves_lifetime() {
        let _rug_st_tests_llm_16_123_rrrruuuugggg_from_str_preserves_lifetime = 0;
        let rug_fuzz_0 = "world";
        let input = rug_fuzz_0.to_string();
        let val = Value::from(input.as_str());
        debug_assert_eq!(val.as_str(), Some("world"));
        let _rug_ed_tests_llm_16_123_rrrruuuugggg_from_str_preserves_lifetime = 0;
    }
    #[test]
    fn from_str_creates_owned_value() {
        let _rug_st_tests_llm_16_123_rrrruuuugggg_from_str_creates_owned_value = 0;
        let rug_fuzz_0 = "owned value";
        let input = rug_fuzz_0;
        let val = Value::from(input);
        debug_assert_eq!(val.as_str(), Some("owned value"));
        let _rug_ed_tests_llm_16_123_rrrruuuugggg_from_str_creates_owned_value = 0;
    }
    #[test]
    fn from_str_empty_string() {
        let _rug_st_tests_llm_16_123_rrrruuuugggg_from_str_empty_string = 0;
        let rug_fuzz_0 = "";
        let val = Value::from(rug_fuzz_0);
        debug_assert!(val.is_str());
        debug_assert_eq!(val.as_str(), Some(""));
        let _rug_ed_tests_llm_16_123_rrrruuuugggg_from_str_empty_string = 0;
    }
    #[test]
    fn from_str_with_whitespace() {
        let _rug_st_tests_llm_16_123_rrrruuuugggg_from_str_with_whitespace = 0;
        let rug_fuzz_0 = " string ";
        let val = Value::from(rug_fuzz_0);
        debug_assert_eq!(val.as_str(), Some(" string "));
        let _rug_ed_tests_llm_16_123_rrrruuuugggg_from_str_with_whitespace = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_124 {
    use super::*;
    use crate::*;
    use crate::{Array, InlineTable, Value};
    #[test]
    fn test_from_value_ref() {
        let _rug_st_tests_llm_16_124_rrrruuuugggg_test_from_value_ref = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 3.14;
        let rug_fuzz_2 = "Hello, World!";
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = "key";
        let rug_fuzz_8 = "value";
        let rug_fuzz_9 = "key";
        let original_value = Value::from(rug_fuzz_0);
        let cloned_value: Value = Value::from(&original_value);
        debug_assert_eq!(cloned_value.as_integer(), Some(42));
        let original_value = Value::from(rug_fuzz_1);
        let cloned_value: Value = Value::from(&original_value);
        debug_assert_eq!(cloned_value.as_float(), Some(3.14));
        let original_value = Value::from(rug_fuzz_2);
        let cloned_value: Value = Value::from(&original_value);
        debug_assert_eq!(cloned_value.as_str(), Some("Hello, World!"));
        let mut array = Array::new();
        array.push(rug_fuzz_3);
        array.push(rug_fuzz_4);
        let original_value = Value::from(array);
        let cloned_value: Value = Value::from(&original_value);
        debug_assert_eq!(
            cloned_value.as_array().and_then(| a | a.get(rug_fuzz_5).and_then(| v | v
            .as_integer())), Some(1)
        );
        debug_assert_eq!(
            cloned_value.as_array().and_then(| a | a.get(rug_fuzz_6).and_then(| v | v
            .as_integer())), Some(2)
        );
        let mut table = InlineTable::new();
        table.insert(rug_fuzz_7, Value::from(rug_fuzz_8));
        let original_value = Value::from(table);
        let cloned_value: Value = Value::from(&original_value);
        debug_assert_eq!(
            cloned_value.as_inline_table().and_then(| t | t.get(rug_fuzz_9).and_then(| v
            | v.as_str())), Some("value")
        );
        let _rug_ed_tests_llm_16_124_rrrruuuugggg_test_from_value_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_125 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_array_to_value() {
        let _rug_st_tests_llm_16_125_rrrruuuugggg_test_from_array_to_value = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "two";
        let rug_fuzz_2 = 3.0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 2;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        let value: Value = Value::from(array);
        match value {
            Value::Array(a) => {
                debug_assert_eq!(a.len(), 3);
                debug_assert_eq!(a.get(rug_fuzz_3).unwrap().as_integer(), Some(1));
                debug_assert_eq!(a.get(rug_fuzz_4).unwrap().as_str(), Some("two"));
                debug_assert_eq!(a.get(rug_fuzz_5).unwrap().as_float(), Some(3.0));
            }
            _ => panic!("Value is not an Array"),
        }
        let _rug_ed_tests_llm_16_125_rrrruuuugggg_test_from_array_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_126 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn test_from_bool() {
        let _rug_st_tests_llm_16_126_rrrruuuugggg_test_from_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let val_true: Value = Value::from(rug_fuzz_0);
        let val_false: Value = Value::from(rug_fuzz_1);
        if let Value::Boolean(formatted_bool) = val_true {
            debug_assert_eq!(* formatted_bool.value(), true);
        } else {
            panic!("Value::from(true) did not produce a Value::Boolean");
        }
        if let Value::Boolean(formatted_bool) = val_false {
            debug_assert_eq!(* formatted_bool.value(), false);
        } else {
            panic!("Value::from(false) did not produce a Value::Boolean");
        }
        let _rug_ed_tests_llm_16_126_rrrruuuugggg_test_from_bool = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_127 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::convert::From;
    #[test]
    fn test_value_from_f64() {
        let _rug_st_tests_llm_16_127_rrrruuuugggg_test_value_from_f64 = 0;
        let rug_fuzz_0 = 42f64;
        let num = rug_fuzz_0;
        let value = Value::from(num);
        if let Value::Float(f) = value {
            debug_assert_eq!(* f.value(), num);
        } else {
            panic!("Value::from did not produce a Value::Float");
        }
        let _rug_ed_tests_llm_16_127_rrrruuuugggg_test_value_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_128 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn from_i64_creates_integer_value() {
        let _rug_st_tests_llm_16_128_rrrruuuugggg_from_i64_creates_integer_value = 0;
        let rug_fuzz_0 = 42i64;
        let num = rug_fuzz_0;
        let value = Value::from(num);
        match value {
            Value::Integer(formatted) => {
                debug_assert_eq!(* formatted.value(), num);
            }
            _ => panic!("Value::from(i64) did not produce an Integer variant"),
        }
        let _rug_ed_tests_llm_16_128_rrrruuuugggg_from_i64_creates_integer_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_129 {
    use crate::*;
    #[test]
    fn from_inline_table_to_value() {
        let _rug_st_tests_llm_16_129_rrrruuuugggg_from_inline_table_to_value = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "key";
        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let value_from_table = Value::from(table.clone());
        if let Value::InlineTable(v) = value_from_table {
            debug_assert_eq!(v.len(), table.len());
            debug_assert_eq!(v.iter().count(), table.iter().count());
            debug_assert_eq!(v.get(rug_fuzz_2).unwrap().as_integer(), Some(42));
        } else {
            panic!("Value::from(InlineTable) did not produce a Value::InlineTable");
        }
        let _rug_ed_tests_llm_16_129_rrrruuuugggg_from_inline_table_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_130 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::internal_string::InternalString;
    #[test]
    fn from_internal_string_creates_string_value() {
        let _rug_st_tests_llm_16_130_rrrruuuugggg_from_internal_string_creates_string_value = 0;
        let rug_fuzz_0 = "test";
        let internal_string = InternalString::from(rug_fuzz_0);
        let value: Value = <Value as From<
            InternalString,
        >>::from(internal_string.clone());
        if let Value::String(formatted_string) = value {
            debug_assert_eq!(formatted_string.value(), internal_string.as_str());
        } else {
            panic!("Value created from InternalString must be of type Value::String");
        }
        let _rug_ed_tests_llm_16_130_rrrruuuugggg_from_internal_string_creates_string_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_131 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::convert::From;
    #[test]
    fn value_from_string() {
        let _rug_st_tests_llm_16_131_rrrruuuugggg_value_from_string = 0;
        let rug_fuzz_0 = "test_string";
        let input = String::from(rug_fuzz_0);
        let value: Value = Value::from(input.clone());
        match value {
            Value::String(formatted) => debug_assert_eq!(formatted.value(), & input),
            _ => panic!("Value is not of type String"),
        }
        let _rug_ed_tests_llm_16_131_rrrruuuugggg_value_from_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_135_llm_16_135 {
    use super::*;
    use crate::*;
    use crate::Value;
    use std::iter::FromIterator;
    #[test]
    fn test_from_iter_empty() {
        let _rug_st_tests_llm_16_135_llm_16_135_rrrruuuugggg_test_from_iter_empty = 0;
        let empty: Vec<(String, Value)> = Vec::new();
        let table: Value = Value::from_iter(empty);
        debug_assert!(matches!(table, Value::InlineTable(t) if t.is_empty()));
        let _rug_ed_tests_llm_16_135_llm_16_135_rrrruuuugggg_test_from_iter_empty = 0;
    }
    #[test]
    fn test_from_iter_with_items() {
        let _rug_st_tests_llm_16_135_llm_16_135_rrrruuuugggg_test_from_iter_with_items = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = "key1";
        let rug_fuzz_4 = "key2";
        let items = vec![
            (rug_fuzz_0.to_string(), Value::from(rug_fuzz_1)), ("key2".to_string(),
            Value::from(42))
        ];
        let table: Value = Value::from_iter(items);
        if let Value::InlineTable(t) = table {
            debug_assert_eq!(rug_fuzz_2, t.len());
            debug_assert_eq!(t.get(rug_fuzz_3).and_then(| v | v.as_bool()), Some(true));
            debug_assert_eq!(t.get(rug_fuzz_4).and_then(| v | v.as_integer()), Some(42));
        } else {
            panic!("Value::InlineTable expected");
        }
        let _rug_ed_tests_llm_16_135_llm_16_135_rrrruuuugggg_test_from_iter_with_items = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_136 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn from_iter_with_integers() {
        let _rug_st_tests_llm_16_136_rrrruuuugggg_from_iter_with_integers = 0;
        let rug_fuzz_0 = 1;
        let values = vec![rug_fuzz_0, 2, 3, 4, 5];
        let value = Value::from_iter(values);
        debug_assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            debug_assert_eq!(array.len(), 5);
            for (i, v) in array.iter().enumerate() {
                debug_assert_eq!(v.as_integer(), Some((i + 1) as i64));
            }
        } else {
            panic!("Value is not an array");
        }
        let _rug_ed_tests_llm_16_136_rrrruuuugggg_from_iter_with_integers = 0;
    }
    #[test]
    fn from_iter_with_strings() {
        let _rug_st_tests_llm_16_136_rrrruuuugggg_from_iter_with_strings = 0;
        let rug_fuzz_0 = "rust";
        let rug_fuzz_1 = "rust";
        let rug_fuzz_2 = "cargo";
        let rug_fuzz_3 = "toml";
        let values = vec![rug_fuzz_0, "cargo", "toml"];
        let value = Value::from_iter(values);
        debug_assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            debug_assert_eq!(array.len(), 3);
            let expected = [rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
            for (i, v) in array.iter().enumerate() {
                debug_assert_eq!(v.as_str(), Some(expected[i]));
            }
        } else {
            panic!("Value is not an array");
        }
        let _rug_ed_tests_llm_16_136_rrrruuuugggg_from_iter_with_strings = 0;
    }
    #[test]
    fn from_iter_with_mixed_types() {
        let _rug_st_tests_llm_16_136_rrrruuuugggg_from_iter_with_mixed_types = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let values = vec![
            Value::from(rug_fuzz_0), Value::from("example"), Value::from(true)
        ];
        let value = Value::from_iter(values);
        debug_assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            debug_assert_eq!(array.len(), 3);
            debug_assert_eq!(array.get(rug_fuzz_1).unwrap().as_integer(), Some(42));
            debug_assert_eq!(array.get(rug_fuzz_2).unwrap().as_str(), Some("example"));
            debug_assert_eq!(array.get(rug_fuzz_3).unwrap().as_bool(), Some(true));
        } else {
            panic!("Value is not an array");
        }
        let _rug_ed_tests_llm_16_136_rrrruuuugggg_from_iter_with_mixed_types = 0;
    }
    #[test]
    fn from_iter_empty() {
        let _rug_st_tests_llm_16_136_rrrruuuugggg_from_iter_empty = 0;
        let values: Vec<Value> = Vec::new();
        let value = Value::from_iter(values);
        debug_assert!(matches!(value, Value::Array(_)));
        if let Value::Array(array) = value {
            debug_assert!(array.is_empty());
        } else {
            panic!("Value is not an array");
        }
        let _rug_ed_tests_llm_16_136_rrrruuuugggg_from_iter_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_137 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    use crate::Value;
    #[test]
    fn test_from_str_valid_input() {
        let _rug_st_tests_llm_16_137_rrrruuuugggg_test_from_str_valid_input = 0;
        let rug_fuzz_0 = r#""Hello, World!""#;
        let valid_str = rug_fuzz_0;
        let value = Value::from_str(valid_str).unwrap();
        debug_assert_eq!(value.as_str(), Some("Hello, World!"));
        let _rug_ed_tests_llm_16_137_rrrruuuugggg_test_from_str_valid_input = 0;
    }
    #[test]
    fn test_from_str_invalid_input() {
        let _rug_st_tests_llm_16_137_rrrruuuugggg_test_from_str_invalid_input = 0;
        let rug_fuzz_0 = "Not a TOML value";
        let invalid_str = rug_fuzz_0;
        let result = Value::from_str(invalid_str);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_137_rrrruuuugggg_test_from_str_invalid_input = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_521 {
    use crate::{Array, Value};
    #[test]
    fn test_as_array_on_array_value() {
        let _rug_st_tests_llm_16_521_rrrruuuugggg_test_as_array_on_array_value = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = "Value should be an array";
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        let value = Value::Array(array);
        let array_ref = value.as_array().expect(rug_fuzz_2);
        debug_assert_eq!(array_ref.len(), 2);
        debug_assert_eq!(
            array_ref.get(rug_fuzz_3).and_then(| v | v.as_integer()), Some(1)
        );
        debug_assert_eq!(
            array_ref.get(rug_fuzz_4).and_then(| v | v.as_integer()), Some(2)
        );
        let _rug_ed_tests_llm_16_521_rrrruuuugggg_test_as_array_on_array_value = 0;
    }
    #[test]
    fn test_as_array_on_non_array_value() {
        let _rug_st_tests_llm_16_521_rrrruuuugggg_test_as_array_on_non_array_value = 0;
        let rug_fuzz_0 = 42;
        let value = Value::from(rug_fuzz_0);
        debug_assert!(value.as_array().is_none());
        let _rug_ed_tests_llm_16_521_rrrruuuugggg_test_as_array_on_non_array_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_522 {
    use super::*;
    use crate::*;
    use crate::array::Array;
    use crate::value::Value;
    #[test]
    fn test_as_array_mut_some() {
        let _rug_st_tests_llm_16_522_rrrruuuugggg_test_as_array_mut_some = 0;
        let mut value = Value::Array(Array::new());
        debug_assert!(value.as_array_mut().is_some());
        let _rug_ed_tests_llm_16_522_rrrruuuugggg_test_as_array_mut_some = 0;
    }
    #[test]
    fn test_as_array_mut_none() {
        let _rug_st_tests_llm_16_522_rrrruuuugggg_test_as_array_mut_none = 0;
        let rug_fuzz_0 = "test";
        let mut value = Value::String(Formatted::new(String::from(rug_fuzz_0)));
        debug_assert!(value.as_array_mut().is_none());
        let _rug_ed_tests_llm_16_522_rrrruuuugggg_test_as_array_mut_none = 0;
    }
    #[test]
    fn test_as_array_mut_mutate() {
        let _rug_st_tests_llm_16_522_rrrruuuugggg_test_as_array_mut_mutate = 0;
        let rug_fuzz_0 = 42;
        let mut value = Value::Array(Array::new());
        if let Some(array) = value.as_array_mut() {
            array.push(rug_fuzz_0);
        }
        debug_assert_eq!(value.as_array().unwrap().len(), 1);
        let _rug_ed_tests_llm_16_522_rrrruuuugggg_test_as_array_mut_mutate = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_523 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_bool_success() {
        let _rug_st_tests_llm_16_523_rrrruuuugggg_test_as_bool_success = 0;
        let rug_fuzz_0 = true;
        let value_bool = Value::Boolean(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(value_bool.as_bool(), Some(true));
        let _rug_ed_tests_llm_16_523_rrrruuuugggg_test_as_bool_success = 0;
    }
    #[test]
    fn test_as_bool_failure() {
        let _rug_st_tests_llm_16_523_rrrruuuugggg_test_as_bool_failure = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "true";
        let value_int = Value::Integer(Formatted::new(rug_fuzz_0));
        let value_str = Value::String(Formatted::new(String::from(rug_fuzz_1)));
        debug_assert_eq!(value_int.as_bool(), None);
        debug_assert_eq!(value_str.as_bool(), None);
        let _rug_ed_tests_llm_16_523_rrrruuuugggg_test_as_bool_failure = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_524 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Datetime;
    #[test]
    fn test_as_datetime() {
        let _rug_st_tests_llm_16_524_rrrruuuugggg_test_as_datetime = 0;
        let rug_fuzz_0 = "1979-05-27T07:32:00Z";
        let rug_fuzz_1 = "Not a datetime";
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 3.14;
        let rug_fuzz_4 = true;
        let datetime_str = rug_fuzz_0;
        let datetime = datetime_str.parse::<Datetime>().unwrap();
        let value = Value::Datetime(Formatted::new(datetime));
        debug_assert_eq!(
            value.as_datetime().map(| dt | dt.to_string()), Some(datetime_str
            .to_string())
        );
        let string_value = Value::String(Formatted::new(rug_fuzz_1.to_string()));
        debug_assert_eq!(string_value.as_datetime(), None);
        let int_value = Value::Integer(Formatted::new(rug_fuzz_2));
        debug_assert_eq!(int_value.as_datetime(), None);
        let float_value = Value::Float(Formatted::new(rug_fuzz_3));
        debug_assert_eq!(float_value.as_datetime(), None);
        let bool_value = Value::Boolean(Formatted::new(rug_fuzz_4));
        debug_assert_eq!(bool_value.as_datetime(), None);
        let array_value = Value::Array(Array::new());
        debug_assert_eq!(array_value.as_datetime(), None);
        let table_value = Value::InlineTable(InlineTable::new());
        debug_assert_eq!(table_value.as_datetime(), None);
        let _rug_ed_tests_llm_16_524_rrrruuuugggg_test_as_datetime = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_525 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_float() {
        let _rug_st_tests_llm_16_525_rrrruuuugggg_test_as_float = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "42";
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = "1979-05-27T07:32:00Z";
        let float_value = Value::Float(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(float_value.as_float(), Some(42.0));
        let int_value = Value::Integer(Formatted::new(rug_fuzz_1));
        debug_assert_eq!(int_value.as_float(), None);
        let str_value = Value::String(Formatted::new(String::from(rug_fuzz_2)));
        debug_assert_eq!(str_value.as_float(), None);
        let bool_value = Value::Boolean(Formatted::new(rug_fuzz_3));
        debug_assert_eq!(bool_value.as_float(), None);
        let datetime_value = Value::Datetime(
            Formatted::new(rug_fuzz_4.parse().unwrap()),
        );
        debug_assert_eq!(datetime_value.as_float(), None);
        let array_value = Value::Array(Array::new());
        debug_assert_eq!(array_value.as_float(), None);
        let inline_table_value = Value::InlineTable(InlineTable::new());
        debug_assert_eq!(inline_table_value.as_float(), None);
        let _rug_ed_tests_llm_16_525_rrrruuuugggg_test_as_float = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_527 {
    use super::*;
    use crate::*;
    use crate::{InlineTable, Value};
    #[test]
    fn as_inline_table_mut_some() {
        let _rug_st_tests_llm_16_527_rrrruuuugggg_as_inline_table_mut_some = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "world";
        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let mut value = Value::from(table);
        let inline_table_mut = value.as_inline_table_mut();
        debug_assert!(inline_table_mut.is_some());
        let _rug_ed_tests_llm_16_527_rrrruuuugggg_as_inline_table_mut_some = 0;
    }
    #[test]
    fn as_inline_table_mut_none() {
        let _rug_st_tests_llm_16_527_rrrruuuugggg_as_inline_table_mut_none = 0;
        let rug_fuzz_0 = "not a table";
        let mut value = Value::from(rug_fuzz_0);
        let inline_table_mut = value.as_inline_table_mut();
        debug_assert!(inline_table_mut.is_none());
        let _rug_ed_tests_llm_16_527_rrrruuuugggg_as_inline_table_mut_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_528_llm_16_528 {
    use crate::repr::Formatted;
    use crate::value::Value;
    #[test]
    fn as_integer_some() {
        let _rug_st_tests_llm_16_528_llm_16_528_rrrruuuugggg_as_integer_some = 0;
        let rug_fuzz_0 = 42;
        let value = Value::Integer(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(value.as_integer(), Some(42));
        let _rug_ed_tests_llm_16_528_llm_16_528_rrrruuuugggg_as_integer_some = 0;
    }
    #[test]
    fn as_integer_none() {
        let _rug_st_tests_llm_16_528_llm_16_528_rrrruuuugggg_as_integer_none = 0;
        let rug_fuzz_0 = "Hello";
        let value = Value::String(Formatted::new(String::from(rug_fuzz_0)));
        debug_assert_eq!(value.as_integer(), None);
        let _rug_ed_tests_llm_16_528_llm_16_528_rrrruuuugggg_as_integer_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_529 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_as_str_string_value() {
        let _rug_st_tests_llm_16_529_rrrruuuugggg_test_as_str_string_value = 0;
        let rug_fuzz_0 = "test";
        let value = Value::String(Formatted::new(rug_fuzz_0.to_string()));
        debug_assert_eq!(value.as_str(), Some("test"));
        let _rug_ed_tests_llm_16_529_rrrruuuugggg_test_as_str_string_value = 0;
    }
    #[test]
    fn test_as_str_non_string_value() {
        let _rug_st_tests_llm_16_529_rrrruuuugggg_test_as_str_non_string_value = 0;
        let rug_fuzz_0 = 42;
        let value = Value::Integer(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(value.as_str(), None);
        let _rug_ed_tests_llm_16_529_rrrruuuugggg_test_as_str_non_string_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_530_llm_16_530 {
    use crate::{Array, Datetime, Decor, InlineTable, RawString, Value};
    #[test]
    fn test_decor_string() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_string = 0;
        let rug_fuzz_0 = "test";
        let v = Value::from(rug_fuzz_0);
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_string = 0;
    }
    #[test]
    fn test_decor_integer() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_integer = 0;
        let rug_fuzz_0 = 42;
        let v = Value::from(rug_fuzz_0);
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_integer = 0;
    }
    #[test]
    fn test_decor_float() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_float = 0;
        let rug_fuzz_0 = 3.14;
        let v = Value::from(rug_fuzz_0);
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_float = 0;
    }
    #[test]
    fn test_decor_boolean() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_boolean = 0;
        let rug_fuzz_0 = true;
        let v = Value::from(rug_fuzz_0);
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_boolean = 0;
    }
    #[test]
    fn test_decor_datetime() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_datetime = 0;
        let rug_fuzz_0 = "2020-05-02T07:30:00Z";
        let v = Value::from(rug_fuzz_0.parse::<Datetime>().unwrap());
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_datetime = 0;
    }
    #[test]
    fn test_decor_array() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_array = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        arr.push(rug_fuzz_2);
        let v = Value::from(arr);
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_array = 0;
    }
    #[test]
    fn test_decor_inline_table() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_inline_table = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 3.14;
        let mut tbl = InlineTable::new();
        tbl.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let v = Value::from(tbl);
        debug_assert_eq!(v.decor().prefix(), None);
        debug_assert_eq!(v.decor().suffix(), None);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_inline_table = 0;
    }
    #[test]
    fn test_decor_custom() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_custom = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "/* prefix */";
        let rug_fuzz_2 = "/* suffix */";
        let mut v = Value::from(rug_fuzz_0);
        v.decorate(RawString::from(rug_fuzz_1), RawString::from(rug_fuzz_2));
        debug_assert_eq!(v.decor().prefix().unwrap().as_str(), Some("/* prefix */"));
        debug_assert_eq!(v.decor().suffix().unwrap().as_str(), Some("/* suffix */"));
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_test_decor_custom = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_532 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::repr::Decor;
    use crate::raw_string::RawString;
    #[test]
    fn decorate_value_with_prefix_suffix() {
        let _rug_st_tests_llm_16_532_rrrruuuugggg_decorate_value_with_prefix_suffix = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = "/* ";
        let rug_fuzz_2 = " */";
        let mut value = Value::from(rug_fuzz_0);
        value.decorate(rug_fuzz_1, rug_fuzz_2);
        match value {
            Value::String(ref formatted) => {
                debug_assert_eq!(
                    formatted.decor().prefix().unwrap().as_str(), Some("/* ")
                );
                debug_assert_eq!(
                    formatted.decor().suffix().unwrap().as_str(), Some(" */")
                );
            }
            _ => panic!("Value is not a String"),
        }
        let _rug_ed_tests_llm_16_532_rrrruuuugggg_decorate_value_with_prefix_suffix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_533 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::raw_string::RawString;
    #[test]
    fn test_decorated() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_decorated = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = " ";
        let rug_fuzz_2 = " ";
        let rug_fuzz_3 = " ";
        let rug_fuzz_4 = " ";
        let original = Value::from(rug_fuzz_0);
        let decorated = original.clone().decorated(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(decorated.to_string(), " 42 ");
        debug_assert!(decorated.decor().prefix().unwrap().as_str() == Some(rug_fuzz_3));
        debug_assert!(decorated.decor().suffix().unwrap().as_str() == Some(rug_fuzz_4));
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_decorated = 0;
    }
    #[test]
    fn test_decorated_string() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_decorated_string = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "//";
        let rug_fuzz_2 = "**";
        let rug_fuzz_3 = "//";
        let rug_fuzz_4 = "**";
        let original = Value::from(rug_fuzz_0);
        let decorated = original.clone().decorated(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(decorated.to_string(), "\"hello\"");
        debug_assert!(decorated.decor().prefix().unwrap().as_str() == Some(rug_fuzz_3));
        debug_assert!(decorated.decor().suffix().unwrap().as_str() == Some(rug_fuzz_4));
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_decorated_string = 0;
    }
    #[test]
    fn test_decorated_empty() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_decorated_empty = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = " ";
        let rug_fuzz_3 = "";
        let rug_fuzz_4 = " ";
        let original = Value::from(rug_fuzz_0);
        let decorated = original.clone().decorated(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(decorated.to_string(), "\"\" ");
        debug_assert!(decorated.decor().prefix().unwrap().as_str() == Some(rug_fuzz_3));
        debug_assert!(decorated.decor().suffix().unwrap().as_str() == Some(rug_fuzz_4));
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_decorated_empty = 0;
    }
    #[test]
    fn test_decorated_array() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_decorated_array = 0;
        let rug_fuzz_0 = "/*";
        let rug_fuzz_1 = "*/";
        let rug_fuzz_2 = "/*";
        let rug_fuzz_3 = "*/";
        let original = Value::Array(Array::new());
        let decorated = original.clone().decorated(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(decorated.to_string(), "[]");
        debug_assert!(decorated.decor().prefix().unwrap().as_str() == Some(rug_fuzz_2));
        debug_assert!(decorated.decor().suffix().unwrap().as_str() == Some(rug_fuzz_3));
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_decorated_array = 0;
    }
    #[test]
    fn test_decorated_inline_table() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_decorated_inline_table = 0;
        let rug_fuzz_0 = "/*";
        let rug_fuzz_1 = "*/";
        let rug_fuzz_2 = "/*";
        let rug_fuzz_3 = "*/";
        let original = Value::InlineTable(InlineTable::new());
        let decorated = original.clone().decorated(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(decorated.to_string(), "{}");
        debug_assert!(decorated.decor().prefix().unwrap().as_str() == Some(rug_fuzz_2));
        debug_assert!(decorated.decor().suffix().unwrap().as_str() == Some(rug_fuzz_3));
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_decorated_inline_table = 0;
    }
    #[test]
    fn test_decorated_with_raw_string() {
        let _rug_st_tests_llm_16_533_rrrruuuugggg_test_decorated_with_raw_string = 0;
        let rug_fuzz_0 = "/*";
        let rug_fuzz_1 = "*/";
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = "/*";
        let rug_fuzz_4 = "*/";
        let prefix_rs = RawString::from(rug_fuzz_0);
        let suffix_rs = RawString::from(rug_fuzz_1);
        let original = Value::from(rug_fuzz_2);
        let decorated = original.clone().decorated(prefix_rs, suffix_rs);
        debug_assert_eq!(decorated.to_string(), "3.14");
        debug_assert!(decorated.decor().prefix().unwrap().as_str() == Some(rug_fuzz_3));
        debug_assert!(decorated.decor().suffix().unwrap().as_str() == Some(rug_fuzz_4));
        let _rug_ed_tests_llm_16_533_rrrruuuugggg_test_decorated_with_raw_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_534 {
    use crate::{value::Value, Array, InlineTable};
    #[test]
    fn despan_string() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_string = 0;
        let rug_fuzz_0 = "value";
        let rug_fuzz_1 = "example";
        let input = rug_fuzz_0;
        let mut val = Value::from(rug_fuzz_1);
        val.despan(input);
        matches!(val, Value::String(s) if s.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_string = 0;
    }
    #[test]
    fn despan_integer() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_integer = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = 42;
        let input = rug_fuzz_0;
        let mut val = Value::from(rug_fuzz_1);
        val.despan(input);
        matches!(val, Value::Integer(i) if i.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_integer = 0;
    }
    #[test]
    fn despan_float() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_float = 0;
        let rug_fuzz_0 = "3.14";
        let rug_fuzz_1 = 3.14;
        let input = rug_fuzz_0;
        let mut val = Value::from(rug_fuzz_1);
        val.despan(input);
        matches!(val, Value::Float(f) if f.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_float = 0;
    }
    #[test]
    fn despan_boolean() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_boolean = 0;
        let rug_fuzz_0 = "false";
        let rug_fuzz_1 = false;
        let input = rug_fuzz_0;
        let mut val = Value::from(rug_fuzz_1);
        val.despan(input);
        matches!(val, Value::Boolean(b) if b.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_boolean = 0;
    }
    #[test]
    fn despan_datetime() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_datetime = 0;
        let rug_fuzz_0 = "2021-04-04T19:49:02Z";
        let input = rug_fuzz_0;
        let mut val = Value::from(input.parse::<crate::Datetime>().unwrap());
        val.despan(input);
        matches!(val, Value::Datetime(dt) if dt.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_datetime = 0;
    }
    #[test]
    fn despan_array() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_array = 0;
        let rug_fuzz_0 = "[1, 2, 3]";
        let rug_fuzz_1 = 1;
        let input = rug_fuzz_0;
        let mut val = Value::from(Array::from_iter(vec![rug_fuzz_1, 2, 3]));
        val.despan(input);
        matches!(val, Value::Array(a) if a.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_array = 0;
    }
    #[test]
    fn despan_inline_table() {
        let _rug_st_tests_llm_16_534_rrrruuuugggg_despan_inline_table = 0;
        let rug_fuzz_0 = "{ x = 2, y = 3 }";
        let rug_fuzz_1 = "x";
        let rug_fuzz_2 = 2;
        let input = rug_fuzz_0;
        let mut val = Value::from(
            InlineTable::from_iter(
                vec![(rug_fuzz_1, Value::from(rug_fuzz_2)), ("y", Value::from(3))],
            ),
        );
        val.despan(input);
        matches!(val, Value::InlineTable(it) if it.span().is_none());
        let _rug_ed_tests_llm_16_534_rrrruuuugggg_despan_inline_table = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_537 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_datetime_for_datetime_value() {
        let _rug_st_tests_llm_16_537_rrrruuuugggg_test_is_datetime_for_datetime_value = 0;
        let rug_fuzz_0 = "1979-05-27T07:32:00Z";
        let datetime_str = rug_fuzz_0;
        let datetime = datetime_str.parse::<Datetime>().unwrap();
        let value = Value::Datetime(Formatted::new(datetime));
        debug_assert!(value.is_datetime());
        let _rug_ed_tests_llm_16_537_rrrruuuugggg_test_is_datetime_for_datetime_value = 0;
    }
    #[test]
    fn test_is_datetime_for_non_datetime_value() {
        let _rug_st_tests_llm_16_537_rrrruuuugggg_test_is_datetime_for_non_datetime_value = 0;
        let rug_fuzz_0 = "a string";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = true;
        let value = Value::String(Formatted::new(rug_fuzz_0.to_string()));
        debug_assert!(! value.is_datetime());
        let value = Value::Integer(Formatted::new(rug_fuzz_1));
        debug_assert!(! value.is_datetime());
        let value = Value::Float(Formatted::new(rug_fuzz_2));
        debug_assert!(! value.is_datetime());
        let value = Value::Boolean(Formatted::new(rug_fuzz_3));
        debug_assert!(! value.is_datetime());
        let value = Value::Array(Array::new());
        debug_assert!(! value.is_datetime());
        let value = Value::InlineTable(InlineTable::new());
        debug_assert!(! value.is_datetime());
        let _rug_ed_tests_llm_16_537_rrrruuuugggg_test_is_datetime_for_non_datetime_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_538 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_float_for_float_value() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_is_float_for_float_value = 0;
        let rug_fuzz_0 = 1.23;
        let value = Value::Float(Formatted::new(rug_fuzz_0));
        debug_assert!(value.is_float());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_is_float_for_float_value = 0;
    }
    #[test]
    fn test_is_float_for_non_float_value() {
        let _rug_st_tests_llm_16_538_rrrruuuugggg_test_is_float_for_non_float_value = 0;
        let rug_fuzz_0 = 123;
        let value = Value::Integer(Formatted::new(rug_fuzz_0));
        debug_assert!(! value.is_float());
        let _rug_ed_tests_llm_16_538_rrrruuuugggg_test_is_float_for_non_float_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_539 {
    use crate::value::Value;
    use crate::inline_table::InlineTable;
    use crate::Array;
    #[test]
    fn test_is_inline_table_with_inline_table() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_inline_table_with_inline_table = 0;
        let inline_table = InlineTable::new();
        let value = Value::InlineTable(inline_table);
        debug_assert!(value.is_inline_table());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_inline_table_with_inline_table = 0;
    }
    #[test]
    fn test_is_inline_table_with_array() {
        let _rug_st_tests_llm_16_539_rrrruuuugggg_test_is_inline_table_with_array = 0;
        let array = Array::new();
        let value = Value::Array(array);
        debug_assert!(! value.is_inline_table());
        let _rug_ed_tests_llm_16_539_rrrruuuugggg_test_is_inline_table_with_array = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_540 {
    use crate::value::Value;
    use crate::Formatted;
    use crate::repr::Decor;
    #[test]
    fn test_is_integer() {
        let _rug_st_tests_llm_16_540_rrrruuuugggg_test_is_integer = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = "42";
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = 42;
        let rug_fuzz_5 = "key";
        let rug_fuzz_6 = 42;
        let integer_value = Value::Integer(Formatted::new(rug_fuzz_0));
        let float_value = Value::Float(Formatted::new(rug_fuzz_1));
        let string_value = Value::String(Formatted::new(rug_fuzz_2.to_owned()));
        let boolean_value = Value::Boolean(Formatted::new(rug_fuzz_3));
        let mut array_value = Value::Array(crate::array::Array::new());
        array_value
            .as_array_mut()
            .unwrap()
            .push(Value::Integer(Formatted::new(rug_fuzz_4)));
        let mut table_value = Value::InlineTable(
            crate::inline_table::InlineTable::new(),
        );
        table_value
            .as_inline_table_mut()
            .unwrap()
            .insert(rug_fuzz_5, Value::Integer(Formatted::new(rug_fuzz_6)));
        debug_assert!(integer_value.is_integer());
        debug_assert!(! float_value.is_integer());
        debug_assert!(! string_value.is_integer());
        debug_assert!(! boolean_value.is_integer());
        debug_assert!(! array_value.is_integer());
        debug_assert!(! table_value.is_integer());
        let _rug_ed_tests_llm_16_540_rrrruuuugggg_test_is_integer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_541 {
    use crate::value::Value;
    #[test]
    fn test_is_str() {
        let _rug_st_tests_llm_16_541_rrrruuuugggg_test_is_str = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = "key";
        let rug_fuzz_6 = "value";
        let string_value = Value::from(rug_fuzz_0);
        let int_value = Value::from(rug_fuzz_1);
        let float_value = Value::from(rug_fuzz_2);
        let bool_value = Value::from(rug_fuzz_3);
        let array_value = Value::from_iter(vec![rug_fuzz_4, 2, 3]);
        let table_value = Value::from_iter(vec![(rug_fuzz_5, rug_fuzz_6)]);
        debug_assert!(string_value.is_str());
        debug_assert!(! int_value.is_str());
        debug_assert!(! float_value.is_str());
        debug_assert!(! bool_value.is_str());
        debug_assert!(! array_value.is_str());
        debug_assert!(! table_value.is_str());
        let _rug_ed_tests_llm_16_541_rrrruuuugggg_test_is_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_542 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::ops::Range;
    #[test]
    fn test_value_span_string() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_string = 0;
        let rug_fuzz_0 = "Hello";
        let mut value = Value::String(Formatted::new(rug_fuzz_0.to_string()));
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_string = 0;
    }
    #[test]
    fn test_value_span_integer() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_integer = 0;
        let rug_fuzz_0 = 42;
        let mut value = Value::Integer(Formatted::new(rug_fuzz_0));
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_integer = 0;
    }
    #[test]
    fn test_value_span_float() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_float = 0;
        let rug_fuzz_0 = 3.14;
        let mut value = Value::Float(Formatted::new(rug_fuzz_0));
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_float = 0;
    }
    #[test]
    fn test_value_span_boolean() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_boolean = 0;
        let rug_fuzz_0 = true;
        let mut value = Value::Boolean(Formatted::new(rug_fuzz_0));
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_boolean = 0;
    }
    #[test]
    fn test_value_span_datetime() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_datetime = 0;
        let rug_fuzz_0 = "1979-05-27T07:32:00Z";
        let mut value = Value::Datetime(Formatted::new(rug_fuzz_0.parse().unwrap()));
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_datetime = 0;
    }
    #[test]
    fn test_value_span_array() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_array = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        let value = Value::Array(array);
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_array = 0;
    }
    #[test]
    fn test_value_span_inline_table() {
        let _rug_st_tests_llm_16_542_rrrruuuugggg_test_value_span_inline_table = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let mut inline_table = InlineTable::new();
        inline_table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let value = Value::InlineTable(inline_table);
        value.span();
        let _rug_ed_tests_llm_16_542_rrrruuuugggg_test_value_span_inline_table = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_543 {
    use super::*;
    use crate::*;
    #[test]
    fn type_name_string() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_string = 0;
        let rug_fuzz_0 = "test";
        let value = Value::String(Formatted::new(String::from(rug_fuzz_0)));
        debug_assert_eq!(value.type_name(), "string");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_string = 0;
    }
    #[test]
    fn type_name_integer() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_integer = 0;
        let rug_fuzz_0 = 42_i64;
        let value = Value::Integer(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(value.type_name(), "integer");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_integer = 0;
    }
    #[test]
    fn type_name_float() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_float = 0;
        let rug_fuzz_0 = 3.14_f64;
        let value = Value::Float(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(value.type_name(), "float");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_float = 0;
    }
    #[test]
    fn type_name_boolean() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_boolean = 0;
        let rug_fuzz_0 = true;
        let value = Value::Boolean(Formatted::new(rug_fuzz_0));
        debug_assert_eq!(value.type_name(), "boolean");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_boolean = 0;
    }
    #[test]
    fn type_name_datetime() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_datetime = 0;
        let rug_fuzz_0 = "1979-05-27T07:32:00Z";
        let value = Value::Datetime(Formatted::new(rug_fuzz_0.parse().unwrap()));
        debug_assert_eq!(value.type_name(), "datetime");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_datetime = 0;
    }
    #[test]
    fn type_name_array() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_array = 0;
        let value = Value::Array(Array::new());
        debug_assert_eq!(value.type_name(), "array");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_array = 0;
    }
    #[test]
    fn type_name_inline_table() {
        let _rug_st_tests_llm_16_543_rrrruuuugggg_type_name_inline_table = 0;
        let value = Value::InlineTable(InlineTable::new());
        debug_assert_eq!(value.type_name(), "inline table");
        let _rug_ed_tests_llm_16_543_rrrruuuugggg_type_name_inline_table = 0;
    }
}
