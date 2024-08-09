use std::iter::FromIterator;
use std::mem;
use crate::repr::Decor;
use crate::value::{DEFAULT_LEADING_VALUE_DECOR, DEFAULT_VALUE_DECOR};
use crate::{Item, RawString, Value};
/// Type representing a TOML array,
/// payload of the `Value::Array` variant's value
#[derive(Debug, Default, Clone)]
pub struct Array {
    trailing: RawString,
    trailing_comma: bool,
    decor: Decor,
    pub(crate) span: Option<std::ops::Range<usize>>,
    pub(crate) values: Vec<Item>,
}
/// An owned iterator type over `Table`'s key/value pairs.
pub type ArrayIntoIter = Box<dyn Iterator<Item = Value>>;
/// An iterator type over `Array`'s values.
pub type ArrayIter<'a> = Box<dyn Iterator<Item = &'a Value> + 'a>;
/// An iterator type over `Array`'s values.
pub type ArrayIterMut<'a> = Box<dyn Iterator<Item = &'a mut Value> + 'a>;
/// Constructors
///
/// See also `FromIterator`
impl Array {
    /// Create an empty `Array`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    pub(crate) fn with_vec(values: Vec<Item>) -> Self {
        Self {
            values,
            ..Default::default()
        }
    }
}
/// Formatting
impl Array {
    /// Auto formats the array.
    pub fn fmt(&mut self) {
        decorate_array(self);
    }
    /// Set whether the array will use a trailing comma
    pub fn set_trailing_comma(&mut self, yes: bool) {
        self.trailing_comma = yes;
    }
    /// Whether the array will use a trailing comma
    pub fn trailing_comma(&self) -> bool {
        self.trailing_comma
    }
    /// Set whitespace after last element
    pub fn set_trailing(&mut self, trailing: impl Into<RawString>) {
        self.trailing = trailing.into();
    }
    /// Whitespace after last element
    pub fn trailing(&self) -> &RawString {
        &self.trailing
    }
    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }
    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        &self.decor
    }
    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }
    pub(crate) fn despan(&mut self, input: &str) {
        self.span = None;
        self.decor.despan(input);
        self.trailing.despan(input);
        for value in &mut self.values {
            value.despan(input);
        }
    }
}
impl Array {
    /// Returns an iterator over all values.
    pub fn iter(&self) -> ArrayIter<'_> {
        Box::new(self.values.iter().filter_map(Item::as_value))
    }
    /// Returns an iterator over all values.
    pub fn iter_mut(&mut self) -> ArrayIterMut<'_> {
        Box::new(self.values.iter_mut().filter_map(Item::as_value_mut))
    }
    /// Returns the length of the underlying Vec.
    ///
    /// In some rare cases, placeholder elements will exist.  For a more accurate count, call
    /// `a.iter().count()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    /// assert_eq!(arr.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Return true iff `self.len() == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// assert!(arr.is_empty());
    ///
    /// arr.push(1);
    /// arr.push("foo");
    /// assert!(! arr.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Clears the array, removing all values. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.values.clear()
    }
    /// Returns a reference to the value at the given index, or `None` if the index is out of
    /// bounds.
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index).and_then(Item::as_value)
    }
    /// Returns a reference to the value at the given index, or `None` if the index is out of
    /// bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.values.get_mut(index).and_then(Item::as_value_mut)
    }
    /// Appends a new value to the end of the array, applying default formatting to it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    /// ```
    pub fn push<V: Into<Value>>(&mut self, v: V) {
        self.value_op(v.into(), true, |items, value| { items.push(Item::Value(value)) })
    }
    /// Appends a new, already formatted value to the end of the array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let formatted_value = "'literal'".parse::<toml_edit::Value>().unwrap();
    /// let mut arr = toml_edit::Array::new();
    /// arr.push_formatted(formatted_value);
    /// ```
    pub fn push_formatted(&mut self, v: Value) {
        self.values.push(Item::Value(v));
    }
    /// Inserts an element at the given position within the array, applying default formatting to
    /// it and shifting all values after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    ///
    /// arr.insert(0, "start");
    /// ```
    pub fn insert<V: Into<Value>>(&mut self, index: usize, v: V) {
        self.value_op(
            v.into(),
            true,
            |items, value| { items.insert(index, Item::Value(value)) },
        )
    }
    /// Inserts an already formatted value at the given position within the array, shifting all
    /// values after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    ///
    /// let formatted_value = "'start'".parse::<toml_edit::Value>().unwrap();
    /// arr.insert_formatted(0, formatted_value);
    /// ```
    pub fn insert_formatted(&mut self, index: usize, v: Value) {
        self.values.insert(index, Item::Value(v))
    }
    /// Replaces the element at the given position within the array, preserving existing formatting.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    ///
    /// arr.replace(0, "start");
    /// ```
    pub fn replace<V: Into<Value>>(&mut self, index: usize, v: V) -> Value {
        let existing_decor = self
            .get(index)
            .unwrap_or_else(|| {
                panic!("index {} out of bounds (len = {})", index, self.len())
            })
            .decor();
        let mut value = v.into();
        *value.decor_mut() = existing_decor.clone();
        self.replace_formatted(index, value)
    }
    /// Replaces the element at the given position within the array with an already formatted value.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    ///
    /// let formatted_value = "'start'".parse::<toml_edit::Value>().unwrap();
    /// arr.replace_formatted(0, formatted_value);
    /// ```
    pub fn replace_formatted(&mut self, index: usize, v: Value) -> Value {
        match mem::replace(&mut self.values[index], Item::Value(v)) {
            Item::Value(old_value) => old_value,
            x => panic!("non-value item {:?} in an array", x),
        }
    }
    /// Removes the value at the given index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut arr = toml_edit::Array::new();
    /// arr.push(1);
    /// arr.push("foo");
    ///
    /// arr.remove(0);
    /// assert_eq!(arr.len(), 1);
    /// ```
    pub fn remove(&mut self, index: usize) -> Value {
        let removed = self.values.remove(index);
        match removed {
            Item::Value(v) => v,
            x => panic!("non-value item {:?} in an array", x),
        }
    }
    fn value_op<T>(
        &mut self,
        v: Value,
        decorate: bool,
        op: impl FnOnce(&mut Vec<Item>, Value) -> T,
    ) -> T {
        let mut value = v;
        if !self.is_empty() && decorate {
            value.decorate(" ", "");
        } else if decorate {
            value.decorate("", "");
        }
        op(&mut self.values, value)
    }
}
impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, None, ("", ""))
    }
}
impl<V: Into<Value>> Extend<V> for Array {
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        for value in iter {
            self.push_formatted(value.into());
        }
    }
}
impl<V: Into<Value>> FromIterator<V> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        let v = iter.into_iter().map(|a| Item::Value(a.into()));
        Array {
            values: v.collect(),
            ..Default::default()
        }
    }
}
impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = ArrayIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self
                .values
                .into_iter()
                .filter(|v| v.is_value())
                .map(|v| v.into_value().unwrap()),
        )
    }
}
impl<'s> IntoIterator for &'s Array {
    type Item = &'s Value;
    type IntoIter = ArrayIter<'s>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
fn decorate_array(array: &mut Array) {
    for (i, value) in array.values.iter_mut().filter_map(Item::as_value_mut).enumerate()
    {
        if i == 0 {
            value.decorate(DEFAULT_LEADING_VALUE_DECOR.0, DEFAULT_LEADING_VALUE_DECOR.1);
        } else {
            value.decorate(DEFAULT_VALUE_DECOR.0, DEFAULT_VALUE_DECOR.1);
        }
    }
    array.set_trailing_comma(false);
    array.set_trailing("");
}
#[cfg(test)]
mod tests_llm_16_11_llm_16_11 {
    use super::*;
    use crate::*;
    use crate::repr::Decor;
    use crate::value::Value;
    use crate::{Array, Item};
    #[test]
    fn from_iter_with_empty_iter() {
        let _rug_st_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_empty_iter = 0;
        let arr: Array = Array::from_iter(Vec::<Value>::new());
        debug_assert!(arr.is_empty());
        let _rug_ed_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_empty_iter = 0;
    }
    #[test]
    fn from_iter_with_integers() {
        let _rug_st_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_integers = 0;
        let rug_fuzz_0 = 1;
        let values = vec![Value::from(rug_fuzz_0), Value::from(2), Value::from(3)];
        let arr: Array = Array::from_iter(values.clone());
        debug_assert!(! arr.is_empty());
        debug_assert_eq!(arr.len(), values.len());
        for (index, value) in values.iter().enumerate() {
            debug_assert_eq!(arr.get(index).unwrap().as_integer(), value.as_integer());
        }
        let _rug_ed_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_integers = 0;
    }
    #[test]
    fn from_iter_with_strings() {
        let _rug_st_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_strings = 0;
        let rug_fuzz_0 = "foo";
        let values = vec![
            Value::from(rug_fuzz_0), Value::from("bar"), Value::from("baz")
        ];
        let arr: Array = Array::from_iter(values.clone());
        debug_assert!(! arr.is_empty());
        debug_assert_eq!(arr.len(), values.len());
        for (index, value) in values.iter().enumerate() {
            debug_assert_eq!(arr.get(index).unwrap().as_str(), value.as_str());
        }
        let _rug_ed_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_strings = 0;
    }
    #[test]
    fn from_iter_with_different_types() {
        let _rug_st_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_different_types = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 3.14;
        let values: Vec<Value> = vec![
            Value::from(rug_fuzz_0), Value::from("foo"), Value::from(3.14)
        ];
        let arr: Array = Array::from_iter(values.clone());
        debug_assert!(! arr.is_empty());
        debug_assert_eq!(arr.len(), values.len());
        debug_assert_eq!(arr.get(rug_fuzz_1).unwrap().as_integer(), Some(42));
        debug_assert_eq!(arr.get(rug_fuzz_2).unwrap().as_str(), Some("foo"));
        debug_assert!(
            arr.get(rug_fuzz_3).unwrap().as_float().unwrap() - rug_fuzz_4 <
            std::f64::EPSILON
        );
        let _rug_ed_tests_llm_16_11_llm_16_11_rrrruuuugggg_from_iter_with_different_types = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_138 {
    use super::*;
    use crate::*;
    #[test]
    fn test_array_clear() {
        let _rug_st_tests_llm_16_138_rrrruuuugggg_test_array_clear = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        debug_assert!(! array.is_empty());
        debug_assert_eq!(array.len(), 3);
        array.clear();
        debug_assert!(array.is_empty());
        debug_assert_eq!(array.len(), 0);
        let _rug_ed_tests_llm_16_138_rrrruuuugggg_test_array_clear = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_140_llm_16_140 {
    use crate::array::Array;
    use crate::repr::Decor;
    use crate::raw_string::RawString;
    #[test]
    fn test_decor_mut() {
        let _rug_st_tests_llm_16_140_llm_16_140_rrrruuuugggg_test_decor_mut = 0;
        let rug_fuzz_0 = " ";
        let rug_fuzz_1 = " ";
        let mut arr = Array::new();
        {
            let decor = arr.decor_mut();
            debug_assert_eq!(decor.prefix(), None);
            debug_assert_eq!(decor.suffix(), None);
            decor.set_prefix(RawString::from(rug_fuzz_0));
            decor.set_suffix(RawString::from(rug_fuzz_1));
        }
        debug_assert_eq!(arr.decor().prefix().unwrap().as_str(), Some(" "));
        debug_assert_eq!(arr.decor().suffix().unwrap().as_str(), Some(" "));
        arr.decor_mut().clear();
        debug_assert_eq!(arr.decor().prefix(), None);
        debug_assert_eq!(arr.decor().suffix(), None);
        let _rug_ed_tests_llm_16_140_llm_16_140_rrrruuuugggg_test_decor_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_141 {
    use super::*;
    use crate::*;
    use crate::Item;
    use crate::Value;
    #[test]
    fn test_despan() {
        let _rug_st_tests_llm_16_141_rrrruuuugggg_test_despan = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "    ";
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = "Hello";
        let rug_fuzz_4 = " ";
        let rug_fuzz_5 = " ";
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 42;
        let rug_fuzz_8 = "Some input string that doesn't really matter in this context.";
        let mut array = Array::new();
        array.set_trailing_comma(rug_fuzz_0);
        array.set_trailing(rug_fuzz_1);
        array.push(rug_fuzz_2);
        array.push(rug_fuzz_3);
        array.decor_mut().set_prefix(rug_fuzz_4);
        array.decor_mut().set_suffix(rug_fuzz_5);
        let initial_span = rug_fuzz_6..rug_fuzz_7;
        array.span = Some(initial_span.clone());
        array.decor_mut().set_prefix(RawString::with_span(initial_span.clone()));
        array.decor_mut().set_suffix(RawString::with_span(initial_span.clone()));
        array.trailing = RawString::with_span(initial_span.clone());
        let input = rug_fuzz_8;
        array.despan(&input);
        debug_assert_eq!(array.span, None);
        for value in &array.values {
            debug_assert!(
                matches!(value, Item::Value(Value::Integer(v)) if v.span() == None)
            );
        }
        debug_assert_eq!(array.decor().prefix().unwrap().as_str(), Some(" "));
        debug_assert_eq!(array.decor().suffix().unwrap().as_str(), Some(" "));
        debug_assert_eq!(array.trailing().as_str(), Some("    "));
        let _rug_ed_tests_llm_16_141_rrrruuuugggg_test_despan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_144 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn test_get_mut_within_bounds() {
        let _rug_st_tests_llm_16_144_rrrruuuugggg_test_get_mut_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "hello";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        if let Some(val) = array.get_mut(rug_fuzz_2) {
            debug_assert_eq!(val.as_integer(), Some(42));
        } else {
            panic!("Expected a value at index 0");
        }
        if let Some(val) = array.get_mut(rug_fuzz_3) {
            debug_assert_eq!(val.as_str(), Some("hello"));
        } else {
            panic!("Expected a value at index 1");
        }
        let _rug_ed_tests_llm_16_144_rrrruuuugggg_test_get_mut_within_bounds = 0;
    }
    #[test]
    fn test_get_mut_out_of_bounds() {
        let _rug_st_tests_llm_16_144_rrrruuuugggg_test_get_mut_out_of_bounds = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        debug_assert!(array.get_mut(rug_fuzz_1).is_none());
        let _rug_ed_tests_llm_16_144_rrrruuuugggg_test_get_mut_out_of_bounds = 0;
    }
    #[test]
    fn test_get_mut_modify_value() {
        let _rug_st_tests_llm_16_144_rrrruuuugggg_test_get_mut_modify_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 43;
        let rug_fuzz_3 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        if let Some(val) = array.get_mut(rug_fuzz_1) {
            *val = Value::from(rug_fuzz_2);
        }
        if let Some(val) = array.get(rug_fuzz_3) {
            debug_assert_eq!(val.as_integer(), Some(43));
        } else {
            panic!("Expected a value at index 0 after modification");
        }
        let _rug_ed_tests_llm_16_144_rrrruuuugggg_test_get_mut_modify_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_145 {
    use super::*;
    use crate::*;
    #[test]
    fn test_insert() {
        let _rug_st_tests_llm_16_145_rrrruuuugggg_test_insert = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "start";
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = "start";
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = "foo";
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3.14;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 3.14;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = "foo";
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        arr.insert(rug_fuzz_2, rug_fuzz_3);
        debug_assert!(arr.get(rug_fuzz_4).unwrap().as_str().unwrap() == rug_fuzz_5);
        debug_assert!(arr.get(rug_fuzz_6).unwrap().as_integer().unwrap() == rug_fuzz_7);
        debug_assert!(arr.get(rug_fuzz_8).unwrap().as_str().unwrap() == rug_fuzz_9);
        arr.insert(rug_fuzz_10, rug_fuzz_11);
        debug_assert!(arr.get(rug_fuzz_12).unwrap().as_float().unwrap() == rug_fuzz_13);
        debug_assert!(arr.get(rug_fuzz_14).unwrap().as_str().unwrap() == rug_fuzz_15);
        let _rug_ed_tests_llm_16_145_rrrruuuugggg_test_insert = 0;
    }
    #[test]
    #[should_panic]
    fn test_insert_out_of_bounds() {
        let _rug_st_tests_llm_16_145_rrrruuuugggg_test_insert_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "panic";
        let mut arr = Array::new();
        arr.insert(rug_fuzz_0, rug_fuzz_1);
        let _rug_ed_tests_llm_16_145_rrrruuuugggg_test_insert_out_of_bounds = 0;
    }
    #[test]
    fn test_insert_empty() {
        let _rug_st_tests_llm_16_145_rrrruuuugggg_test_insert_empty = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "only";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "only";
        let rug_fuzz_4 = 1;
        let mut arr = Array::new();
        arr.insert(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(arr.get(rug_fuzz_2).unwrap().as_str().unwrap() == rug_fuzz_3);
        debug_assert!(arr.len() == rug_fuzz_4);
        let _rug_ed_tests_llm_16_145_rrrruuuugggg_test_insert_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_146 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn insert_formatted_at_start() {
        let _rug_st_tests_llm_16_146_rrrruuuugggg_insert_formatted_at_start = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = "'start'";
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        let formatted_value = rug_fuzz_2.parse::<Value>().unwrap();
        arr.insert_formatted(rug_fuzz_3, formatted_value);
        debug_assert_eq!(arr.get(rug_fuzz_4).unwrap().as_str(), Some("start"));
        debug_assert_eq!(arr.get(rug_fuzz_5).unwrap().as_integer(), Some(1));
        debug_assert_eq!(arr.get(rug_fuzz_6).unwrap().as_str(), Some("foo"));
        let _rug_ed_tests_llm_16_146_rrrruuuugggg_insert_formatted_at_start = 0;
    }
    #[test]
    #[should_panic(expected = "index 3 out of bounds (len = 2)")]
    fn insert_formatted_out_of_bounds() {
        let _rug_st_tests_llm_16_146_rrrruuuugggg_insert_formatted_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = "'start'";
        let rug_fuzz_3 = 3;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        let formatted_value = rug_fuzz_2.parse::<Value>().unwrap();
        arr.insert_formatted(rug_fuzz_3, formatted_value);
        let _rug_ed_tests_llm_16_146_rrrruuuugggg_insert_formatted_out_of_bounds = 0;
    }
    #[test]
    fn insert_formatted_at_end() {
        let _rug_st_tests_llm_16_146_rrrruuuugggg_insert_formatted_at_end = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = "'end'";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 2;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        let formatted_value = rug_fuzz_2.parse::<Value>().unwrap();
        arr.insert_formatted(rug_fuzz_3, formatted_value);
        debug_assert_eq!(arr.get(rug_fuzz_4).unwrap().as_str(), Some("end"));
        let _rug_ed_tests_llm_16_146_rrrruuuugggg_insert_formatted_at_end = 0;
    }
    #[test]
    fn insert_formatted_preserves_format() {
        let _rug_st_tests_llm_16_146_rrrruuuugggg_insert_formatted_preserves_format = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = " 'preserved' ";
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        let formatted_value = rug_fuzz_2.parse::<Value>().unwrap();
        arr.insert_formatted(rug_fuzz_3, formatted_value);
        let inserted_value = arr.get(rug_fuzz_4).unwrap();
        debug_assert_eq!(inserted_value.as_str(), Some("preserved"));
        debug_assert_eq!(inserted_value.decor().prefix().unwrap().as_str(), Some(" "));
        debug_assert_eq!(inserted_value.decor().suffix().unwrap().as_str(), Some(" "));
        let _rug_ed_tests_llm_16_146_rrrruuuugggg_insert_formatted_preserves_format = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_147 {
    use crate::{Array, Item, Value};
    #[test]
    fn test_is_empty_on_new_array() {
        let _rug_st_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_new_array = 0;
        let arr = Array::new();
        debug_assert!(arr.is_empty());
        let _rug_ed_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_new_array = 0;
    }
    #[test]
    fn test_is_empty_on_non_empty_array() {
        let _rug_st_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_non_empty_array = 0;
        let rug_fuzz_0 = 42;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        debug_assert!(! arr.is_empty());
        let _rug_ed_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_non_empty_array = 0;
    }
    #[test]
    fn test_is_empty_on_cleared_array() {
        let _rug_st_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_cleared_array = 0;
        let rug_fuzz_0 = "test";
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.clear();
        debug_assert!(arr.is_empty());
        let _rug_ed_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_cleared_array = 0;
    }
    #[test]
    fn test_is_empty_on_array_with_removed_elements() {
        let _rug_st_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_array_with_removed_elements = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        arr.remove(rug_fuzz_2);
        debug_assert!(! arr.is_empty());
        arr.remove(rug_fuzz_3);
        debug_assert!(arr.is_empty());
        let _rug_ed_tests_llm_16_147_rrrruuuugggg_test_is_empty_on_array_with_removed_elements = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_148_llm_16_148 {
    use crate::{Array, Value};
    #[test]
    fn test_array_iter() {
        let _rug_st_tests_llm_16_148_llm_16_148_rrrruuuugggg_test_array_iter = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        let mut iter = array.iter();
        debug_assert_eq!(iter.next().map(| v | v.as_integer()), Some(Some(1)));
        debug_assert_eq!(iter.next().map(| v | v.as_integer()), Some(Some(2)));
        debug_assert_eq!(iter.next().map(| v | v.as_integer()), Some(Some(3)));
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_148_llm_16_148_rrrruuuugggg_test_array_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_150 {
    use super::*;
    use crate::*;
    #[test]
    fn array_len_initially_zero() {
        let _rug_st_tests_llm_16_150_rrrruuuugggg_array_len_initially_zero = 0;
        let arr = Array::new();
        debug_assert_eq!(arr.len(), 0);
        let _rug_ed_tests_llm_16_150_rrrruuuugggg_array_len_initially_zero = 0;
    }
    #[test]
    fn array_len_after_insert() {
        let _rug_st_tests_llm_16_150_rrrruuuugggg_array_len_after_insert = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "hello";
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        debug_assert_eq!(arr.len(), 2);
        let _rug_ed_tests_llm_16_150_rrrruuuugggg_array_len_after_insert = 0;
    }
    #[test]
    fn array_len_after_remove() {
        let _rug_st_tests_llm_16_150_rrrruuuugggg_array_len_after_remove = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "hello";
        let rug_fuzz_2 = 0;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        arr.remove(rug_fuzz_2);
        debug_assert_eq!(arr.len(), 1);
        let _rug_ed_tests_llm_16_150_rrrruuuugggg_array_len_after_remove = 0;
    }
    #[test]
    fn array_len_after_clear() {
        let _rug_st_tests_llm_16_150_rrrruuuugggg_array_len_after_clear = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "hello";
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        arr.clear();
        debug_assert_eq!(arr.len(), 0);
        let _rug_ed_tests_llm_16_150_rrrruuuugggg_array_len_after_clear = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_151 {
    use crate::array::Array;
    use crate::array::Item;
    use crate::repr::Decor;
    use crate::raw_string::RawString;
    #[test]
    fn test_array_new() {
        let _rug_st_tests_llm_16_151_rrrruuuugggg_test_array_new = 0;
        let arr = Array::new();
        debug_assert_eq!(arr.len(), 0);
        debug_assert!(arr.is_empty());
        debug_assert_eq!(arr.trailing(), & RawString::default());
        debug_assert_eq!(arr.trailing_comma(), false);
        debug_assert_eq!(arr.decor(), & Decor::default());
        debug_assert!(arr.iter().next().is_none());
        let _rug_ed_tests_llm_16_151_rrrruuuugggg_test_array_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_152 {
    use super::*;
    use crate::*;
    #[test]
    fn test_push_value_to_empty_array() {
        let _rug_st_tests_llm_16_152_rrrruuuugggg_test_push_value_to_empty_array = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        debug_assert_eq!(array.len(), 1);
        debug_assert_eq!(array.get(rug_fuzz_1).unwrap().as_integer(), Some(42));
        let _rug_ed_tests_llm_16_152_rrrruuuugggg_test_push_value_to_empty_array = 0;
    }
    #[test]
    fn test_push_multiple_values() {
        let _rug_st_tests_llm_16_152_rrrruuuugggg_test_push_multiple_values = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 2;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        debug_assert_eq!(array.len(), 3);
        debug_assert_eq!(array.get(rug_fuzz_3).unwrap().as_integer(), Some(42));
        debug_assert_eq!(array.get(rug_fuzz_4).unwrap().as_str(), Some("test"));
        debug_assert_eq!(array.get(rug_fuzz_5).unwrap().as_float(), Some(3.14));
        let _rug_ed_tests_llm_16_152_rrrruuuugggg_test_push_multiple_values = 0;
    }
    #[test]
    fn test_push_preserves_existing_elements() {
        let _rug_st_tests_llm_16_152_rrrruuuugggg_test_push_preserves_existing_elements = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = "new";
        let rug_fuzz_4 = 3;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        let initial_len = array.len();
        array.push(rug_fuzz_3);
        debug_assert_eq!(array.len(), initial_len + 1);
        debug_assert_eq!(array.get(rug_fuzz_4).unwrap().as_str(), Some("new"));
        let _rug_ed_tests_llm_16_152_rrrruuuugggg_test_push_preserves_existing_elements = 0;
    }
    #[test]
    fn test_push_trailing_comma_unchanged() {
        let _rug_st_tests_llm_16_152_rrrruuuugggg_test_push_trailing_comma_unchanged = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "test";
        let mut array = Array::new();
        array.set_trailing_comma(rug_fuzz_0);
        array.push(rug_fuzz_1);
        debug_assert!(array.trailing_comma());
        array.push(rug_fuzz_2);
        debug_assert!(array.trailing_comma());
        let _rug_ed_tests_llm_16_152_rrrruuuugggg_test_push_trailing_comma_unchanged = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_154 {
    use super::*;
    use crate::*;
    use crate::Item;
    #[test]
    fn test_remove_first_element() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_test_remove_first_element = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        let removed = array.remove(rug_fuzz_2);
        debug_assert_eq!(removed.as_integer(), Some(1));
        debug_assert_eq!(array.len(), 1);
        debug_assert_eq!(array.get(rug_fuzz_3).unwrap().as_integer(), Some(2));
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_test_remove_first_element = 0;
    }
    #[test]
    fn test_remove_middle_element() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_test_remove_middle_element = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        let removed = array.remove(rug_fuzz_3);
        debug_assert_eq!(removed.as_integer(), Some(2));
        debug_assert_eq!(array.len(), 2);
        debug_assert_eq!(array.get(rug_fuzz_4).unwrap().as_integer(), Some(1));
        debug_assert_eq!(array.get(rug_fuzz_5).unwrap().as_integer(), Some(3));
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_test_remove_middle_element = 0;
    }
    #[test]
    fn test_remove_last_element() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_test_remove_last_element = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        let removed = array.remove(rug_fuzz_2);
        debug_assert_eq!(removed.as_integer(), Some(2));
        debug_assert_eq!(array.len(), 1);
        debug_assert_eq!(array.get(rug_fuzz_3).unwrap().as_integer(), Some(1));
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_test_remove_last_element = 0;
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_remove_out_of_bounds() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_test_remove_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.remove(rug_fuzz_1);
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_test_remove_out_of_bounds = 0;
    }
    #[test]
    #[should_panic(expected = "non-value item")]
    fn test_remove_non_value_item() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_test_remove_non_value_item = 0;
        let rug_fuzz_0 = 0;
        let mut array = Array::new();
        array.values.push(Item::None);
        array.remove(rug_fuzz_0);
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_test_remove_non_value_item = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_155 {
    use super::*;
    use crate::*;
    use crate::{Array, Value};
    #[test]
    fn test_replace_preserving_decor() {
        let _rug_st_tests_llm_16_155_rrrruuuugggg_test_replace_preserving_decor = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = "start";
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 42;
        let rug_fuzz_9 = 1;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        let original_decor = arr.get(rug_fuzz_2).unwrap().decor().clone();
        arr.replace(rug_fuzz_3, rug_fuzz_4);
        let replaced_decor = arr.get(rug_fuzz_5).unwrap().decor();
        debug_assert_eq!(original_decor, * replaced_decor);
        let original_decor = arr.get(rug_fuzz_6).unwrap().decor().clone();
        arr.replace(rug_fuzz_7, rug_fuzz_8);
        let replaced_decor = arr.get(rug_fuzz_9).unwrap().decor();
        debug_assert_eq!(original_decor, * replaced_decor);
        let _rug_ed_tests_llm_16_155_rrrruuuugggg_test_replace_preserving_decor = 0;
    }
    #[test]
    #[should_panic]
    fn test_replace_out_of_bounds() {
        let _rug_st_tests_llm_16_155_rrrruuuugggg_test_replace_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "start";
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.replace(rug_fuzz_1, rug_fuzz_2);
        let _rug_ed_tests_llm_16_155_rrrruuuugggg_test_replace_out_of_bounds = 0;
    }
    #[test]
    fn test_replace_different_types() {
        let _rug_st_tests_llm_16_155_rrrruuuugggg_test_replace_different_types = 0;
        let rug_fuzz_0 = "foo";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = true;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 3.14;
        let rug_fuzz_9 = 0;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        let returned = arr.replace(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(returned.as_str(), Some("foo"));
        debug_assert_eq!(arr.get(rug_fuzz_3).unwrap().as_integer(), Some(42));
        let returned = arr.replace(rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(returned.as_integer(), Some(42));
        debug_assert_eq!(arr.get(rug_fuzz_6).unwrap().as_bool(), Some(true));
        let returned = arr.replace(rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(returned.as_bool(), Some(true));
        debug_assert_eq!(arr.get(rug_fuzz_9).unwrap().as_float(), Some(3.14));
        let _rug_ed_tests_llm_16_155_rrrruuuugggg_test_replace_different_types = 0;
    }
    #[test]
    fn test_replace_same_type() {
        let _rug_st_tests_llm_16_155_rrrruuuugggg_test_replace_same_type = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = "bar";
        let rug_fuzz_7 = 1;
        let mut arr = Array::new();
        arr.push(rug_fuzz_0);
        arr.push(rug_fuzz_1);
        let returned = arr.replace(rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(returned.as_integer(), Some(1));
        debug_assert_eq!(arr.get(rug_fuzz_4).unwrap().as_integer(), Some(42));
        let returned = arr.replace(rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(returned.as_str(), Some("foo"));
        debug_assert_eq!(arr.get(rug_fuzz_7).unwrap().as_str(), Some("bar"));
        let _rug_ed_tests_llm_16_155_rrrruuuugggg_test_replace_same_type = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_157_llm_16_157 {
    use crate::array::Array;
    use crate::raw_string::RawString;
    use crate::internal_string::InternalString;
    #[test]
    fn set_trailing_basic() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_basic = 0;
        let rug_fuzz_0 = "  ";
        let mut array = Array::new();
        let expected_trailing = rug_fuzz_0;
        array.set_trailing(expected_trailing);
        let internal_string: InternalString = expected_trailing.into();
        debug_assert_eq!(array.trailing().as_str(), Some(internal_string.as_str()));
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_basic = 0;
    }
    #[test]
    fn set_trailing_from_string() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_from_string = 0;
        let rug_fuzz_0 = "  ";
        let mut array = Array::new();
        let trailing_string: String = rug_fuzz_0.to_string();
        array.set_trailing(trailing_string.clone());
        let internal_string: InternalString = trailing_string.as_str().into();
        debug_assert_eq!(array.trailing().as_str(), Some(internal_string.as_str()));
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_from_string = 0;
    }
    #[test]
    fn set_trailing_from_raw_string() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_from_raw_string = 0;
        let rug_fuzz_0 = "  ";
        let mut array = Array::new();
        let raw_string_input = rug_fuzz_0;
        let raw_string: RawString = raw_string_input.into();
        array.set_trailing(raw_string.clone());
        debug_assert_eq!(array.trailing().as_str(), Some(raw_string_input));
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_from_raw_string = 0;
    }
    #[test]
    fn set_trailing_empty() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_empty = 0;
        let mut array = Array::new();
        let raw_string: RawString = RawString::default();
        array.set_trailing(raw_string);
        debug_assert_eq!(array.trailing().as_str(), Some(""));
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_empty = 0;
    }
    #[test]
    fn set_trailing_persists() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_persists = 0;
        let rug_fuzz_0 = "initial";
        let rug_fuzz_1 = "final";
        let mut array = Array::new();
        let initial_trailing = rug_fuzz_0;
        let final_trailing = rug_fuzz_1;
        array.set_trailing(initial_trailing);
        let initial_internal_string: InternalString = initial_trailing.into();
        let final_internal_string: InternalString = final_trailing.into();
        debug_assert_eq!(
            array.trailing().as_str(), Some(initial_internal_string.as_str())
        );
        array.set_trailing(final_trailing);
        debug_assert_eq!(
            array.trailing().as_str(), Some(final_internal_string.as_str())
        );
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_set_trailing_persists = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_158 {
    use crate::{array::Array, encode::Encode, raw_string::RawString, repr::Decor};
    #[test]
    fn test_set_trailing_comma() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_set_trailing_comma = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let mut array = Array::new();
        debug_assert!(! array.trailing_comma());
        array.set_trailing_comma(rug_fuzz_0);
        debug_assert!(array.trailing_comma());
        array.set_trailing_comma(rug_fuzz_1);
        debug_assert!(! array.trailing_comma());
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_set_trailing_comma = 0;
    }
    #[test]
    fn test_set_trailing_comma_with_encoding() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_set_trailing_comma_with_encoding = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = "";
        let rug_fuzz_3 = "";
        let rug_fuzz_4 = "";
        let rug_fuzz_5 = "";
        let mut array = Array::new();
        array.set_trailing_comma(rug_fuzz_0);
        array.set_trailing(RawString::from(rug_fuzz_1));
        *array.decor_mut() = Decor::new(rug_fuzz_2, rug_fuzz_3);
        let mut encoded = String::new();
        array.encode(&mut encoded, None, (rug_fuzz_4, rug_fuzz_5)).unwrap();
        debug_assert_eq!(encoded, "[,]");
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_set_trailing_comma_with_encoding = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_159 {
    use crate::array::Array;
    use crate::repr::Decor;
    use crate::Item;
    use std::ops::Range;
    #[test]
    fn span_empty_array() {
        let array = Array::new();
        assert_eq!(array.span(), None);
    }
    #[test]
    fn span_non_empty_array() {
        let array = Array {
            span: Some(Range { start: 5, end: 10 }),
            ..Array::new()
        };
        assert_eq!(array.span(), Some(Range { start : 5, end : 10 }));
    }
    #[test]
    fn span_set_and_unset() {
        let mut array = Array::new();
        array.span = Some(Range { start: 5, end: 10 });
        assert_eq!(array.span(), Some(Range { start : 5, end : 10 }));
        array.span = None;
        assert_eq!(array.span(), None);
    }
    #[test]
    fn span_array_with_decor() {
        let array = Array {
            decor: Decor::new("/* prefix */", "/* suffix */"),
            span: Some(Range { start: 5, end: 10 }),
            ..Array::new()
        };
        assert_eq!(array.span(), Some(Range { start : 5, end : 10 }));
        assert_eq!(array.decor.prefix().is_some(), true);
        assert_eq!(array.decor.suffix().is_some(), true);
    }
    fn array_with_span(span: Range<usize>) -> Array {
        Array {
            span: Some(span),
            values: Vec::new(),
            trailing: "".into(),
            trailing_comma: false,
            decor: Decor::default(),
        }
    }
    #[test]
    fn span_array_with_values() {
        let mut array = array_with_span(Range { start: 5, end: 10 });
        array.values.push(Item::Value("value1".into()));
        array.values.push(Item::Value("value2".into()));
        assert_eq!(array.span(), Some(Range { start : 5, end : 10 }));
        assert_eq!(array.values.len(), 2);
    }
}
#[cfg(test)]
mod tests_llm_16_160 {
    use crate::{Array, RawString};
    #[test]
    fn trailing_returns_correct_raw_string() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_trailing_returns_correct_raw_string = 0;
        let rug_fuzz_0 = "  \n  # Comment";
        let mut array = Array::new();
        let trailing_string = rug_fuzz_0;
        array.set_trailing(RawString::from(trailing_string));
        let trailing = array.trailing();
        debug_assert_eq!(* trailing, RawString::from(trailing_string));
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_trailing_returns_correct_raw_string = 0;
    }
    #[test]
    fn trailing_returns_empty_raw_string_for_new_array() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_trailing_returns_empty_raw_string_for_new_array = 0;
        let array = Array::new();
        let trailing = array.trailing();
        debug_assert_eq!(* trailing, RawString::default());
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_trailing_returns_empty_raw_string_for_new_array = 0;
    }
    #[test]
    fn trailing_persists_after_modifying_array() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_trailing_persists_after_modifying_array = 0;
        let rug_fuzz_0 = "  \n  # Comment";
        let rug_fuzz_1 = 42;
        let mut array = Array::new();
        let trailing_string = rug_fuzz_0;
        array.set_trailing(RawString::from(trailing_string));
        array.push(rug_fuzz_1);
        let trailing_after_push = array.trailing();
        debug_assert_eq!(* trailing_after_push, RawString::from(trailing_string));
        array.clear();
        let trailing_after_clear = array.trailing();
        debug_assert_eq!(* trailing_after_clear, RawString::from(trailing_string));
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_trailing_persists_after_modifying_array = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_161 {
    use super::*;
    use crate::*;
    #[test]
    fn trailing_comma_when_false() {
        let _rug_st_tests_llm_16_161_rrrruuuugggg_trailing_comma_when_false = 0;
        let rug_fuzz_0 = false;
        let mut array = Array::new();
        array.set_trailing_comma(rug_fuzz_0);
        debug_assert!(! array.trailing_comma());
        let _rug_ed_tests_llm_16_161_rrrruuuugggg_trailing_comma_when_false = 0;
    }
    #[test]
    fn trailing_comma_when_true() {
        let _rug_st_tests_llm_16_161_rrrruuuugggg_trailing_comma_when_true = 0;
        let rug_fuzz_0 = true;
        let mut array = Array::new();
        array.set_trailing_comma(rug_fuzz_0);
        debug_assert!(array.trailing_comma());
        let _rug_ed_tests_llm_16_161_rrrruuuugggg_trailing_comma_when_true = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_162 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn test_value_op_push() {
        let _rug_st_tests_llm_16_162_rrrruuuugggg_test_value_op_push = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        debug_assert_eq!(array.len(), 1);
        debug_assert!(matches!(array.get(rug_fuzz_1), Some(Value::Integer(_))));
        let _rug_ed_tests_llm_16_162_rrrruuuugggg_test_value_op_push = 0;
    }
    #[test]
    fn test_value_op_insert() {
        let _rug_st_tests_llm_16_162_rrrruuuugggg_test_value_op_insert = 0;
        let rug_fuzz_0 = "first";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "second";
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.insert(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(array.len(), 2);
        debug_assert!(matches!(array.get(rug_fuzz_3), Some(Value::String(_))));
        debug_assert_eq!(array.get(rug_fuzz_4).and_then(Value::as_str), Some("second"));
        let _rug_ed_tests_llm_16_162_rrrruuuugggg_test_value_op_insert = 0;
    }
    #[test]
    fn test_value_op_insert_decorated() {
        let _rug_st_tests_llm_16_162_rrrruuuugggg_test_value_op_insert_decorated = 0;
        let rug_fuzz_0 = "first";
        let rug_fuzz_1 = "second";
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array
            .value_op(
                rug_fuzz_1.into(),
                rug_fuzz_2,
                |items, value| { items.insert(rug_fuzz_3, Item::Value(value)) },
            );
        debug_assert_eq!(array.len(), 2);
        debug_assert!(matches!(array.get(rug_fuzz_4), Some(Value::String(_))));
        let value = array.get(rug_fuzz_5).unwrap();
        debug_assert_eq!(value.as_str(), Some("second"));
        debug_assert_eq!(value.decor().prefix(), Some(& RawString::from(" ")));
        debug_assert_eq!(value.decor().suffix(), Some(& RawString::from("")));
        let _rug_ed_tests_llm_16_162_rrrruuuugggg_test_value_op_insert_decorated = 0;
    }
    #[test]
    fn test_value_op_replace() {
        let _rug_st_tests_llm_16_162_rrrruuuugggg_test_value_op_replace = 0;
        let rug_fuzz_0 = "first";
        let rug_fuzz_1 = "second";
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = "third";
        let rug_fuzz_4 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.replace(rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(array.len(), 2);
        debug_assert_eq!(array.get(rug_fuzz_4).and_then(Value::as_str), Some("third"));
        let _rug_ed_tests_llm_16_162_rrrruuuugggg_test_value_op_replace = 0;
    }
    #[test]
    fn test_value_op_remove() {
        let _rug_st_tests_llm_16_162_rrrruuuugggg_test_value_op_remove = 0;
        let rug_fuzz_0 = "first";
        let rug_fuzz_1 = "second";
        let rug_fuzz_2 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        let removed = array.remove(rug_fuzz_2);
        debug_assert_eq!(array.len(), 1);
        debug_assert_eq!(removed.as_str(), Some("first"));
        let _rug_ed_tests_llm_16_162_rrrruuuugggg_test_value_op_remove = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_164 {
    use super::*;
    use crate::*;
    #[test]
    fn test_decorate_array_empty_array() {
        let _rug_st_tests_llm_16_164_rrrruuuugggg_test_decorate_array_empty_array = 0;
        let mut array = Array::new();
        decorate_array(&mut array);
        debug_assert!(! array.trailing_comma());
        debug_assert_eq!(array.trailing().as_str(), Some(""));
        let _rug_ed_tests_llm_16_164_rrrruuuugggg_test_decorate_array_empty_array = 0;
    }
    #[test]
    fn test_decorate_array_single_value() {
        let _rug_st_tests_llm_16_164_rrrruuuugggg_test_decorate_array_single_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        decorate_array(&mut array);
        let first_value_decor = array.get(rug_fuzz_1).unwrap().decor();
        debug_assert_eq!(
            first_value_decor.prefix().and_then(| p | p.as_str()), Some("")
        );
        debug_assert_eq!(
            first_value_decor.suffix().and_then(| s | s.as_str()), Some("")
        );
        debug_assert!(! array.trailing_comma());
        debug_assert_eq!(array.trailing().as_str(), Some(""));
        let _rug_ed_tests_llm_16_164_rrrruuuugggg_test_decorate_array_single_value = 0;
    }
    #[test]
    fn test_decorate_array_multiple_values() {
        let _rug_st_tests_llm_16_164_rrrruuuugggg_test_decorate_array_multiple_values = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        decorate_array(&mut array);
        let first_value_decor = array.get(rug_fuzz_3).unwrap().decor();
        debug_assert_eq!(
            first_value_decor.prefix().and_then(| p | p.as_str()), Some("")
        );
        debug_assert_eq!(
            first_value_decor.suffix().and_then(| s | s.as_str()), Some("")
        );
        for i in rug_fuzz_4..array.len() {
            let value_decor = array.get(i).unwrap().decor();
            debug_assert_eq!(value_decor.prefix().and_then(| p | p.as_str()), Some(" "));
            debug_assert_eq!(value_decor.suffix().and_then(| s | s.as_str()), Some(""));
        }
        debug_assert!(! array.trailing_comma());
        debug_assert_eq!(array.trailing().as_str(), Some(""));
        let _rug_ed_tests_llm_16_164_rrrruuuugggg_test_decorate_array_multiple_values = 0;
    }
}
