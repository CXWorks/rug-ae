use std::iter::FromIterator;
use std::mem;

use crate::repr::Decor;
use crate::value::{DEFAULT_LEADING_VALUE_DECOR, DEFAULT_VALUE_DECOR};
use crate::{Item, RawString, Value};

/// Type representing a TOML array,
/// payload of the `Value::Array` variant's value
#[derive(Debug, Default, Clone)]
pub struct Array {
    // `trailing` represents whitespaces, newlines
    // and comments in an empty array or after the trailing comma
    trailing: RawString,
    trailing_comma: bool,
    // prefix before `[` and suffix after `]`
    decor: Decor,
    pub(crate) span: Option<std::ops::Range<usize>>,
    // always Vec<Item::Value>
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
        self.value_op(v.into(), true, |items, value| {
            items.push(Item::Value(value))
        })
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
        self.value_op(v.into(), true, |items, value| {
            items.insert(index, Item::Value(value))
        })
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
        // Read the existing value's decor and preserve it.
        let existing_decor = self
            .get(index)
            .unwrap_or_else(|| panic!("index {} out of bounds (len = {})", index, self.len()))
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
            self.values
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
    for (i, value) in array
        .values
        .iter_mut()
        .filter_map(Item::as_value_mut)
        .enumerate()
    {
        // [value1, value2, value3]
        if i == 0 {
            value.decorate(DEFAULT_LEADING_VALUE_DECOR.0, DEFAULT_LEADING_VALUE_DECOR.1);
        } else {
            value.decorate(DEFAULT_VALUE_DECOR.0, DEFAULT_VALUE_DECOR.1);
        }
    }
    // Since everything is now on the same line, remove trailing commas and whitespace.
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
        let arr: Array = Array::from_iter(Vec::<Value>::new());
        assert!(arr.is_empty());
    }

    #[test]
    fn from_iter_with_integers() {
        let values = vec![Value::from(1), Value::from(2), Value::from(3)];
        let arr: Array = Array::from_iter(values.clone());

        assert!(!arr.is_empty());
        assert_eq!(arr.len(), values.len());
        for (index, value) in values.iter().enumerate() {
            assert_eq!(arr.get(index).unwrap().as_integer(), value.as_integer());
        }
    }

    #[test]
    fn from_iter_with_strings() {
        let values = vec![Value::from("foo"), Value::from("bar"), Value::from("baz")];
        let arr: Array = Array::from_iter(values.clone());

        assert!(!arr.is_empty());
        assert_eq!(arr.len(), values.len());
        for (index, value) in values.iter().enumerate() {
            assert_eq!(arr.get(index).unwrap().as_str(), value.as_str());
        }
    }

    #[test]
    fn from_iter_with_different_types() {
        let values: Vec<Value> = vec![Value::from(42), Value::from("foo"), Value::from(3.14)];
        let arr: Array = Array::from_iter(values.clone());

        assert!(!arr.is_empty());
        assert_eq!(arr.len(), values.len());

        assert_eq!(arr.get(0).unwrap().as_integer(), Some(42));
        assert_eq!(arr.get(1).unwrap().as_str(), Some("foo"));
        assert!(arr.get(2).unwrap().as_float().unwrap() - 3.14 < std::f64::EPSILON);
    }
}#[cfg(test)]
mod tests_llm_16_138 {
    use super::*;

use crate::*;

    #[test]
    fn test_array_clear() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        array.push(3);

        assert!(!array.is_empty());
        assert_eq!(array.len(), 3);

        array.clear();

        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }
}#[cfg(test)]
mod tests_llm_16_140_llm_16_140 {
    use crate::array::Array;
    use crate::repr::Decor;
    use crate::raw_string::RawString;

    #[test]
    fn test_decor_mut() {
        let mut arr = Array::new();
        {
            let decor = arr.decor_mut();
            assert_eq!(decor.prefix(), None);
            assert_eq!(decor.suffix(), None);

            decor.set_prefix(RawString::from(" "));
            decor.set_suffix(RawString::from(" "));
        }

        assert_eq!(arr.decor().prefix().unwrap().as_str(), Some(" "));
        assert_eq!(arr.decor().suffix().unwrap().as_str(), Some(" "));

        arr.decor_mut().clear();
        assert_eq!(arr.decor().prefix(), None);
        assert_eq!(arr.decor().suffix(), None);
    }
}#[cfg(test)]
mod tests_llm_16_141 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Value;

    #[test]
    fn test_despan() {
        let mut array = Array::new();
        array.set_trailing_comma(true);
        array.set_trailing("    ");
        array.push(42);
        array.push("Hello");
        array.decor_mut().set_prefix(" ");
        array.decor_mut().set_suffix(" ");

        let initial_span = 0..42;
        array.span = Some(initial_span.clone());
        array.decor_mut().set_prefix(RawString::with_span(initial_span.clone()));
        array.decor_mut().set_suffix(RawString::with_span(initial_span.clone()));
        array.trailing = RawString::with_span(initial_span.clone());

        let input = "Some input string that doesn't really matter in this context.";

        array.despan(&input);

        assert_eq!(array.span, None);
        for value in &array.values {
            assert!(matches!(value, Item::Value(Value::Integer(v)) if v.span() == None));
        }
        assert_eq!(array.decor().prefix().unwrap().as_str(), Some(" "));
        assert_eq!(array.decor().suffix().unwrap().as_str(), Some(" "));
        assert_eq!(array.trailing().as_str(), Some("    "));
    }
}#[cfg(test)]
mod tests_llm_16_144 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_get_mut_within_bounds() {
        let mut array = Array::new();
        array.push(42);
        array.push("hello");

        // Test getting mutable references within bounds
        if let Some(val) = array.get_mut(0) {
            assert_eq!(val.as_integer(), Some(42));
        } else {
            panic!("Expected a value at index 0");
        }

        if let Some(val) = array.get_mut(1) {
            assert_eq!(val.as_str(), Some("hello"));
        } else {
            panic!("Expected a value at index 1");
        }
    }

    #[test]
    fn test_get_mut_out_of_bounds() {
        let mut array = Array::new();
        array.push(42);

        // Test getting mutable reference out of bounds
        assert!(array.get_mut(1).is_none());
    }

    #[test]
    fn test_get_mut_modify_value() {
        let mut array = Array::new();
        array.push(42);

        // Modify value through mutable reference
        if let Some(val) = array.get_mut(0) {
            *val = Value::from(43);
        }

        if let Some(val) = array.get(0) {
            assert_eq!(val.as_integer(), Some(43));
        } else {
            panic!("Expected a value at index 0 after modification");
        }
    }
}#[cfg(test)]
mod tests_llm_16_145 {
    use super::*;

use crate::*;

    #[test]
    fn test_insert() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");

        arr.insert(0, "start");
        assert!(arr.get(0).unwrap().as_str().unwrap() == "start");
        assert!(arr.get(1).unwrap().as_integer().unwrap() == 1);
        assert!(arr.get(2).unwrap().as_str().unwrap() == "foo");

        arr.insert(2, 3.14);
        assert!(arr.get(2).unwrap().as_float().unwrap() == 3.14);
        assert!(arr.get(3).unwrap().as_str().unwrap() == "foo");
    }

    #[test]
    #[should_panic]
    fn test_insert_out_of_bounds() {
        let mut arr = Array::new();
        arr.insert(1, "panic");
    }

    #[test]
    fn test_insert_empty() {
        let mut arr = Array::new();
        arr.insert(0, "only");
        assert!(arr.get(0).unwrap().as_str().unwrap() == "only");
        assert!(arr.len() == 1);
    }
}#[cfg(test)]
mod tests_llm_16_146 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn insert_formatted_at_start() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");
        let formatted_value = "'start'".parse::<Value>().unwrap();
        arr.insert_formatted(0, formatted_value);
        assert_eq!(arr.get(0).unwrap().as_str(), Some("start"));
        assert_eq!(arr.get(1).unwrap().as_integer(), Some(1));
        assert_eq!(arr.get(2).unwrap().as_str(), Some("foo"));
    }

    #[test]
    #[should_panic(expected = "index 3 out of bounds (len = 2)")]
    fn insert_formatted_out_of_bounds() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");
        let formatted_value = "'start'".parse::<Value>().unwrap();
        arr.insert_formatted(3, formatted_value);
    }

    #[test]
    fn insert_formatted_at_end() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");
        let formatted_value = "'end'".parse::<Value>().unwrap();
        arr.insert_formatted(2, formatted_value);
        assert_eq!(arr.get(2).unwrap().as_str(), Some("end"));
    }

    #[test]
    fn insert_formatted_preserves_format() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");
        let formatted_value = " 'preserved' ".parse::<Value>().unwrap();
        arr.insert_formatted(1, formatted_value);
        let inserted_value = arr.get(1).unwrap();
        assert_eq!(inserted_value.as_str(), Some("preserved"));
        // Assuming the `Decor` for the inserted `Value` includes the leading and trailing space
        assert_eq!(inserted_value.decor().prefix().unwrap().as_str(), Some(" "));
        assert_eq!(inserted_value.decor().suffix().unwrap().as_str(), Some(" "));
    }
}#[cfg(test)]
mod tests_llm_16_147 {
    use crate::{Array, Item, Value};

    #[test]
    fn test_is_empty_on_new_array() {
        let arr = Array::new();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_is_empty_on_non_empty_array() {
        let mut arr = Array::new();
        arr.push(42);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_is_empty_on_cleared_array() {
        let mut arr = Array::new();
        arr.push("test");
        arr.clear();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_is_empty_on_array_with_removed_elements() {
        let mut arr = Array::new();
        arr.push("test");
        arr.push(42);
        arr.remove(0);
        assert!(!arr.is_empty());

        arr.remove(0);
        assert!(arr.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_148_llm_16_148 {
    use crate::{Array, Value};

    #[test]
    fn test_array_iter() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        array.push(3);

        let mut iter = array.iter();

        assert_eq!(iter.next().map(|v| v.as_integer()), Some(Some(1)));
        assert_eq!(iter.next().map(|v| v.as_integer()), Some(Some(2)));
        assert_eq!(iter.next().map(|v| v.as_integer()), Some(Some(3)));
        assert!(iter.next().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_150 {
    use super::*;

use crate::*;

    #[test]
    fn array_len_initially_zero() {
        let arr = Array::new();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn array_len_after_insert() {
        let mut arr = Array::new();
        arr.push(42);
        arr.push("hello");
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn array_len_after_remove() {
        let mut arr = Array::new();
        arr.push(42);
        arr.push("hello");
        arr.remove(0);
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn array_len_after_clear() {
        let mut arr = Array::new();
        arr.push(42);
        arr.push("hello");
        arr.clear();
        assert_eq!(arr.len(), 0);
    }
}#[cfg(test)]
mod tests_llm_16_151 {
    use crate::array::Array;
    use crate::array::Item;
    use crate::repr::Decor;
    use crate::raw_string::RawString;

    #[test]
    fn test_array_new() {
        let arr = Array::new();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
        assert_eq!(arr.trailing(), &RawString::default());
        assert_eq!(arr.trailing_comma(), false);
        assert_eq!(arr.decor(), &Decor::default());
        assert!(arr.iter().next().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_152 {
    use super::*;

use crate::*;

    #[test]
    fn test_push_value_to_empty_array() {
        let mut array = Array::new();

        array.push(42);
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0).unwrap().as_integer(), Some(42));
    }

    #[test]
    fn test_push_multiple_values() {
        let mut array = Array::new();

        array.push(42);
        array.push("test");
        array.push(3.14);
        assert_eq!(array.len(), 3);
        assert_eq!(array.get(0).unwrap().as_integer(), Some(42));
        assert_eq!(array.get(1).unwrap().as_str(), Some("test"));
        assert_eq!(array.get(2).unwrap().as_float(), Some(3.14));
    }

    #[test]
    fn test_push_preserves_existing_elements() {
        let mut array = Array::new();

        array.push(42);
        array.push("test");
        array.push(3.14);

        let initial_len = array.len();
        array.push("new");
        assert_eq!(array.len(), initial_len + 1);
        assert_eq!(array.get(3).unwrap().as_str(), Some("new"));
    }

    #[test]
    fn test_push_trailing_comma_unchanged() {
        let mut array = Array::new();
        array.set_trailing_comma(true);

        array.push(42);
        assert!(array.trailing_comma());
        array.push("test");
        assert!(array.trailing_comma());
    }
}#[cfg(test)]
mod tests_llm_16_154 {
    use super::*;

use crate::*;
    use crate::Item;

    #[test]
    fn test_remove_first_element() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        let removed = array.remove(0);
        assert_eq!(removed.as_integer(), Some(1));
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0).unwrap().as_integer(), Some(2));
    }

    #[test]
    fn test_remove_middle_element() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        array.push(3);
        let removed = array.remove(1);
        assert_eq!(removed.as_integer(), Some(2));
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(0).unwrap().as_integer(), Some(1));
        assert_eq!(array.get(1).unwrap().as_integer(), Some(3));
    }

    #[test]
    fn test_remove_last_element() {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        let removed = array.remove(1);
        assert_eq!(removed.as_integer(), Some(2));
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0).unwrap().as_integer(), Some(1));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_remove_out_of_bounds() {
        let mut array = Array::new();
        array.push(1);
        array.remove(1);
    }

    #[test]
    #[should_panic(expected = "non-value item")]
    fn test_remove_non_value_item() {
        let mut array = Array::new();
        array.values.push(Item::None);
        array.remove(0);
    }
}#[cfg(test)]
mod tests_llm_16_155 {
    use super::*;

use crate::*;
    use crate::{Array, Value};

    #[test]
    fn test_replace_preserving_decor() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");

        let original_decor = arr.get(0).unwrap().decor().clone();
        arr.replace(0, "start");
        let replaced_decor = arr.get(0).unwrap().decor();
        assert_eq!(original_decor, *replaced_decor);

        let original_decor = arr.get(1).unwrap().decor().clone();
        arr.replace(1, 42);
        let replaced_decor = arr.get(1).unwrap().decor();
        assert_eq!(original_decor, *replaced_decor);
    }

    #[test]
    #[should_panic]
    fn test_replace_out_of_bounds() {
        let mut arr = Array::new();
        arr.push(1);
        arr.replace(1, "start");
    }

    #[test]
    fn test_replace_different_types() {
        let mut arr = Array::new();
        arr.push("foo");

        // Replace String with Integer
        let returned = arr.replace(0, 42);
        assert_eq!(returned.as_str(), Some("foo"));
        assert_eq!(arr.get(0).unwrap().as_integer(), Some(42));

        // Replace Integer with Boolean
        let returned = arr.replace(0, true);
        assert_eq!(returned.as_integer(), Some(42));
        assert_eq!(arr.get(0).unwrap().as_bool(), Some(true));

        // Replace Boolean with Float
        let returned = arr.replace(0, 3.14);
        assert_eq!(returned.as_bool(), Some(true));
        assert_eq!(arr.get(0).unwrap().as_float(), Some(3.14));
    }

    #[test]
    fn test_replace_same_type() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push("foo");

        // Replace Integer with Integer
        let returned = arr.replace(0, 42);
        assert_eq!(returned.as_integer(), Some(1));
        assert_eq!(arr.get(0).unwrap().as_integer(), Some(42));

        // Replace String with String
        let returned = arr.replace(1, "bar");
        assert_eq!(returned.as_str(), Some("foo"));
        assert_eq!(arr.get(1).unwrap().as_str(), Some("bar"));
    }
}#[cfg(test)]
mod tests_llm_16_157_llm_16_157 {
    use crate::array::Array;
    use crate::raw_string::RawString;
    use crate::internal_string::InternalString;

    #[test]
    fn set_trailing_basic() {
        let mut array = Array::new();
        let expected_trailing = "  ";
        array.set_trailing(expected_trailing);
        let internal_string: InternalString = expected_trailing.into();
        assert_eq!(array.trailing().as_str(), Some(internal_string.as_str()));
    }

    #[test]
    fn set_trailing_from_string() {
        let mut array = Array::new();
        let trailing_string: String = "  ".to_string();
        array.set_trailing(trailing_string.clone());
        let internal_string: InternalString = trailing_string.as_str().into();
        assert_eq!(array.trailing().as_str(), Some(internal_string.as_str()));
    }

    #[test]
    fn set_trailing_from_raw_string() {
        let mut array = Array::new();
        let raw_string_input = "  ";
        let raw_string: RawString = raw_string_input.into();
        array.set_trailing(raw_string.clone());
        assert_eq!(array.trailing().as_str(), Some(raw_string_input));
    }

    #[test]
    fn set_trailing_empty() {
        let mut array = Array::new();
        let raw_string: RawString = RawString::default();
        array.set_trailing(raw_string);
        assert_eq!(array.trailing().as_str(), Some(""));
    }

    #[test]
    fn set_trailing_persists() {
        let mut array = Array::new();
        let initial_trailing = "initial";
        let final_trailing = "final";
        array.set_trailing(initial_trailing);
        let initial_internal_string: InternalString = initial_trailing.into();
        let final_internal_string: InternalString = final_trailing.into();
        assert_eq!(array.trailing().as_str(), Some(initial_internal_string.as_str()));
        array.set_trailing(final_trailing);
        assert_eq!(array.trailing().as_str(), Some(final_internal_string.as_str()));
    }
}#[cfg(test)]
mod tests_llm_16_158 {
    use crate::{array::Array, encode::Encode, raw_string::RawString, repr::Decor};

    #[test]
    fn test_set_trailing_comma() {
        let mut array = Array::new();
        // Initially, array should not have a trailing comma
        assert!(!array.trailing_comma());
        // Set a trailing comma and check if it is set
        array.set_trailing_comma(true);
        assert!(array.trailing_comma());
        // Remove the trailing comma and check if it is removed
        array.set_trailing_comma(false);
        assert!(!array.trailing_comma());
    }

    #[test]
    fn test_set_trailing_comma_with_encoding() {
        let mut array = Array::new();
        array.set_trailing_comma(true);
        // Explicitly set empty trailing and decor to have control over the output
        array.set_trailing(RawString::from(""));
        *array.decor_mut() = Decor::new("", "");
        // Create an encoded representation of the array into a string
        let mut encoded = String::new();
        array.encode(&mut encoded, None, ("", "")).unwrap();
        // Check if the encoded representation contains the trailing comma
        assert_eq!(encoded, "[,]");
    }
}#[cfg(test)]
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
        assert_eq!(array.span(), Some(Range { start: 5, end: 10 }));
    }

    #[test]
    fn span_set_and_unset() {
        let mut array = Array::new();
        array.span = Some(Range { start: 5, end: 10 });
        assert_eq!(array.span(), Some(Range { start: 5, end: 10 }));
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
        assert_eq!(array.span(), Some(Range { start: 5, end: 10 }));
        assert_eq!(array.decor.prefix().is_some(), true);
        assert_eq!(array.decor.suffix().is_some(), true);
    }

    // Utility to create an array with a span
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
        assert_eq!(array.span(), Some(Range { start: 5, end: 10 }));
        assert_eq!(array.values.len(), 2);
    }
}#[cfg(test)]
mod tests_llm_16_160 {
    use crate::{Array, RawString};

    #[test]
    fn trailing_returns_correct_raw_string() {
        let mut array = Array::new();
        let trailing_string = "  \n  # Comment";
        array.set_trailing(RawString::from(trailing_string));

        let trailing = array.trailing();
        assert_eq!(*trailing, RawString::from(trailing_string));
    }

    #[test]
    fn trailing_returns_empty_raw_string_for_new_array() {
        let array = Array::new();
        let trailing = array.trailing();
        assert_eq!(*trailing, RawString::default());
    }

    #[test]
    fn trailing_persists_after_modifying_array() {
        let mut array = Array::new();
        let trailing_string = "  \n  # Comment";
        array.set_trailing(RawString::from(trailing_string));

        array.push(42);
        let trailing_after_push = array.trailing();
        assert_eq!(*trailing_after_push, RawString::from(trailing_string));

        array.clear();
        let trailing_after_clear = array.trailing();
        assert_eq!(*trailing_after_clear, RawString::from(trailing_string));
    }
}#[cfg(test)]
mod tests_llm_16_161 {
    use super::*;

use crate::*;

    #[test]
    fn trailing_comma_when_false() {
        let mut array = Array::new();
        array.set_trailing_comma(false);
        assert!(!array.trailing_comma());
    }

    #[test]
    fn trailing_comma_when_true() {
        let mut array = Array::new();
        array.set_trailing_comma(true);
        assert!(array.trailing_comma());
    }
}#[cfg(test)]
mod tests_llm_16_162 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_value_op_push() {
        let mut array = Array::new();
        array.push(42);
        assert_eq!(array.len(), 1);
        assert!(matches!(array.get(0), Some(Value::Integer(_))));
    }

    #[test]
    fn test_value_op_insert() {
        let mut array = Array::new();
        array.push("first");
        array.insert(0, "second");
        assert_eq!(array.len(), 2);
        assert!(matches!(array.get(0), Some(Value::String(_))));
        assert_eq!(array.get(0).and_then(Value::as_str), Some("second"));
    }

    #[test]
    fn test_value_op_insert_decorated() {
        let mut array = Array::new();
        array.push("first");
        array.value_op("second".into(), true, |items, value| {
            items.insert(0, Item::Value(value))
        });
        assert_eq!(array.len(), 2);
        assert!(matches!(array.get(0), Some(Value::String(_))));
        let value = array.get(0).unwrap();
        assert_eq!(value.as_str(), Some("second"));
        assert_eq!(value.decor().prefix(), Some(&RawString::from(" ")));
        assert_eq!(value.decor().suffix(), Some(&RawString::from("")));
    }

    #[test]
    fn test_value_op_replace() {
        let mut array = Array::new();
        array.push("first");
        array.push("second");
        array.replace(1, "third");
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(1).and_then(Value::as_str), Some("third"));
    }

    #[test]
    fn test_value_op_remove() {
        let mut array = Array::new();
        array.push("first");
        array.push("second");
        let removed = array.remove(0);
        assert_eq!(array.len(), 1);
        assert_eq!(removed.as_str(), Some("first"));
    }
}#[cfg(test)]
mod tests_llm_16_164 {
    use super::*; // This will import everything from the outer module

use crate::*;

    #[test]
    fn test_decorate_array_empty_array() {
        let mut array = Array::new();
        decorate_array(&mut array);
        assert!(!array.trailing_comma());
        assert_eq!(array.trailing().as_str(), Some(""));
    }

    #[test]
    fn test_decorate_array_single_value() {
        let mut array = Array::new();
        array.push(42);
        decorate_array(&mut array);
        let first_value_decor = array.get(0).unwrap().decor();
        assert_eq!(first_value_decor.prefix().and_then(|p| p.as_str()), Some(""));
        assert_eq!(first_value_decor.suffix().and_then(|s| s.as_str()), Some(""));
        assert!(!array.trailing_comma());
        assert_eq!(array.trailing().as_str(), Some(""));
    }

    #[test]
    fn test_decorate_array_multiple_values() {
        let mut array = Array::new();
        array.push(42);
        array.push("foo");
        array.push(3.14);
        decorate_array(&mut array);
        let first_value_decor = array.get(0).unwrap().decor();
        assert_eq!(first_value_decor.prefix().and_then(|p| p.as_str()), Some(""));
        assert_eq!(first_value_decor.suffix().and_then(|s| s.as_str()), Some(""));
        for i in 1..array.len() {
            let value_decor = array.get(i).unwrap().decor();
            assert_eq!(value_decor.prefix().and_then(|p| p.as_str()), Some(" "));
            assert_eq!(value_decor.suffix().and_then(|s| s.as_str()), Some(""));
        }
        assert!(!array.trailing_comma());
        assert_eq!(array.trailing().as_str(), Some(""));
    }
}