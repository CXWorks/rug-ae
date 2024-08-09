use std::iter::FromIterator;

use crate::key::Key;
use crate::repr::Decor;
use crate::table::{Iter, IterMut, KeyValuePairs, TableKeyValue, TableLike};
use crate::{InternalString, Item, KeyMut, RawString, Table, Value};

/// Type representing a TOML inline table,
/// payload of the `Value::InlineTable` variant
#[derive(Debug, Default, Clone)]
pub struct InlineTable {
    // `preamble` represents whitespaces in an empty table
    preamble: RawString,
    // prefix before `{` and suffix after `}`
    decor: Decor,
    pub(crate) span: Option<std::ops::Range<usize>>,
    // whether this is a proxy for dotted keys
    dotted: bool,
    pub(crate) items: KeyValuePairs,
}

/// Constructors
///
/// See also `FromIterator`
impl InlineTable {
    /// Creates an empty table.
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn with_pairs(items: KeyValuePairs) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    /// Convert to a table
    pub fn into_table(self) -> Table {
        let mut t = Table::with_pairs(self.items);
        t.fmt();
        t
    }
}

/// Formatting
impl InlineTable {
    /// Get key/values for values that are visually children of this table
    ///
    /// For example, this will return dotted keys
    pub fn get_values(&self) -> Vec<(Vec<&Key>, &Value)> {
        let mut values = Vec::new();
        let root = Vec::new();
        self.append_values(&root, &mut values);
        values
    }

    pub(crate) fn append_values<'s, 'c>(
        &'s self,
        parent: &[&'s Key],
        values: &'c mut Vec<(Vec<&'s Key>, &'s Value)>,
    ) {
        for value in self.items.values() {
            let mut path = parent.to_vec();
            path.push(&value.key);
            match &value.value {
                Item::Value(Value::InlineTable(table)) if table.is_dotted() => {
                    table.append_values(&path, values);
                }
                Item::Value(value) => {
                    values.push((path, value));
                }
                _ => {}
            }
        }
    }

    /// Auto formats the table.
    pub fn fmt(&mut self) {
        decorate_inline_table(self);
    }

    /// Sorts the key/value pairs by key.
    pub fn sort_values(&mut self) {
        // Assuming standard tables have their position set and this won't negatively impact them
        self.items.sort_keys();
        for kv in self.items.values_mut() {
            match &mut kv.value {
                Item::Value(Value::InlineTable(table)) if table.is_dotted() => {
                    table.sort_values();
                }
                _ => {}
            }
        }
    }

    /// Sort Key/Value Pairs of the table using the using the comparison function `compare`.
    ///
    /// The comparison function receives two key and value pairs to compare (you can sort by keys or
    /// values or their combination as needed).
    pub fn sort_values_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&Key, &Value, &Key, &Value) -> std::cmp::Ordering,
    {
        self.sort_values_by_internal(&mut compare);
    }

    fn sort_values_by_internal<F>(&mut self, compare: &mut F)
    where
        F: FnMut(&Key, &Value, &Key, &Value) -> std::cmp::Ordering,
    {
        let modified_cmp = |_: &InternalString,
                            val1: &TableKeyValue,
                            _: &InternalString,
                            val2: &TableKeyValue|
         -> std::cmp::Ordering {
            match (val1.value.as_value(), val2.value.as_value()) {
                (Some(v1), Some(v2)) => compare(&val1.key, v1, &val2.key, v2),
                (Some(_), None) => std::cmp::Ordering::Greater,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (None, None) => std::cmp::Ordering::Equal,
            }
        };

        self.items.sort_by(modified_cmp);
        for kv in self.items.values_mut() {
            match &mut kv.value {
                Item::Value(Value::InlineTable(table)) if table.is_dotted() => {
                    table.sort_values_by_internal(compare);
                }
                _ => {}
            }
        }
    }

    /// Change this table's dotted status
    pub fn set_dotted(&mut self, yes: bool) {
        self.dotted = yes;
    }

    /// Check if this is a wrapper for dotted keys, rather than a standard table
    pub fn is_dotted(&self) -> bool {
        self.dotted
    }

    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        &self.decor
    }

    /// Returns the decor associated with a given key of the table.
    pub fn key_decor_mut(&mut self, key: &str) -> Option<&mut Decor> {
        self.items.get_mut(key).map(|kv| &mut kv.key.decor)
    }

    /// Returns the decor associated with a given key of the table.
    pub fn key_decor(&self, key: &str) -> Option<&Decor> {
        self.items.get(key).map(|kv| &kv.key.decor)
    }

    /// Set whitespace after before element
    pub fn set_preamble(&mut self, preamble: impl Into<RawString>) {
        self.preamble = preamble.into();
    }

    /// Whitespace after before element
    pub fn preamble(&self) -> &RawString {
        &self.preamble
    }

    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }

    pub(crate) fn despan(&mut self, input: &str) {
        self.span = None;
        self.decor.despan(input);
        self.preamble.despan(input);
        for kv in self.items.values_mut() {
            kv.key.despan(input);
            kv.value.despan(input);
        }
    }
}

impl InlineTable {
    /// Returns an iterator over key/value pairs.
    pub fn iter(&self) -> InlineTableIter<'_> {
        Box::new(
            self.items
                .iter()
                .filter(|&(_, kv)| kv.value.is_value())
                .map(|(k, kv)| (&k[..], kv.value.as_value().unwrap())),
        )
    }

    /// Returns an iterator over key/value pairs.
    pub fn iter_mut(&mut self) -> InlineTableIterMut<'_> {
        Box::new(
            self.items
                .iter_mut()
                .filter(|(_, kv)| kv.value.is_value())
                .map(|(_, kv)| (kv.key.as_mut(), kv.value.as_value_mut().unwrap())),
        )
    }

    /// Returns the number of key/value pairs.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns true iff the table is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the table, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.items.clear()
    }

    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    pub fn entry(&'_ mut self, key: impl Into<InternalString>) -> InlineEntry<'_> {
        match self.items.entry(key.into()) {
            indexmap::map::Entry::Occupied(mut entry) => {
                // Ensure it is a `Value` to simplify `InlineOccupiedEntry`'s code.
                let scratch = std::mem::take(&mut entry.get_mut().value);
                let scratch = Item::Value(
                    scratch
                        .into_value()
                        // HACK: `Item::None` is a corner case of a corner case, let's just pick a
                        // "safe" value
                        .unwrap_or_else(|_| Value::InlineTable(Default::default())),
                );
                entry.get_mut().value = scratch;

                InlineEntry::Occupied(InlineOccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                InlineEntry::Vacant(InlineVacantEntry { entry, key: None })
            }
        }
    }

    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    pub fn entry_format<'a>(&'a mut self, key: &Key) -> InlineEntry<'a> {
        // Accept a `&Key` to be consistent with `entry`
        match self.items.entry(key.get().into()) {
            indexmap::map::Entry::Occupied(mut entry) => {
                // Ensure it is a `Value` to simplify `InlineOccupiedEntry`'s code.
                let scratch = std::mem::take(&mut entry.get_mut().value);
                let scratch = Item::Value(
                    scratch
                        .into_value()
                        // HACK: `Item::None` is a corner case of a corner case, let's just pick a
                        // "safe" value
                        .unwrap_or_else(|_| Value::InlineTable(Default::default())),
                );
                entry.get_mut().value = scratch;

                InlineEntry::Occupied(InlineOccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => InlineEntry::Vacant(InlineVacantEntry {
                entry,
                key: Some(key.clone()),
            }),
        }
    }
    /// Return an optional reference to the value at the given the key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.items.get(key).and_then(|kv| kv.value.as_value())
    }

    /// Return an optional mutable reference to the value at the given the key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.items
            .get_mut(key)
            .and_then(|kv| kv.value.as_value_mut())
    }

    /// Return references to the key-value pair stored for key, if it is present, else None.
    pub fn get_key_value<'a>(&'a self, key: &str) -> Option<(&'a Key, &'a Item)> {
        self.items.get(key).and_then(|kv| {
            if !kv.value.is_none() {
                Some((&kv.key, &kv.value))
            } else {
                None
            }
        })
    }

    /// Return mutable references to the key-value pair stored for key, if it is present, else None.
    pub fn get_key_value_mut<'a>(&'a mut self, key: &str) -> Option<(KeyMut<'a>, &'a mut Item)> {
        self.items.get_mut(key).and_then(|kv| {
            if !kv.value.is_none() {
                Some((kv.key.as_mut(), &mut kv.value))
            } else {
                None
            }
        })
    }

    /// Returns true iff the table contains given key.
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_value()
        } else {
            false
        }
    }

    /// Inserts a key/value pair if the table does not contain the key.
    /// Returns a mutable reference to the corresponding value.
    pub fn get_or_insert<V: Into<Value>>(
        &mut self,
        key: impl Into<InternalString>,
        value: V,
    ) -> &mut Value {
        let key = key.into();
        self.items
            .entry(key.clone())
            .or_insert(TableKeyValue::new(Key::new(key), Item::Value(value.into())))
            .value
            .as_value_mut()
            .expect("non-value type in inline table")
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: impl Into<InternalString>, value: Value) -> Option<Value> {
        let key = key.into();
        let kv = TableKeyValue::new(Key::new(key.clone()), Item::Value(value));
        self.items
            .insert(key, kv)
            .and_then(|kv| kv.value.into_value().ok())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert_formatted(&mut self, key: &Key, value: Value) -> Option<Value> {
        let kv = TableKeyValue::new(key.to_owned(), Item::Value(value));
        self.items
            .insert(InternalString::from(key.get()), kv)
            .filter(|kv| kv.value.is_value())
            .map(|kv| kv.value.into_value().unwrap())
    }

    /// Removes an item given the key.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.items
            .shift_remove(key)
            .and_then(|kv| kv.value.into_value().ok())
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    pub fn remove_entry(&mut self, key: &str) -> Option<(Key, Value)> {
        self.items.shift_remove(key).and_then(|kv| {
            let key = kv.key;
            kv.value.into_value().ok().map(|value| (key, value))
        })
    }
}

impl std::fmt::Display for InlineTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, None, ("", ""))
    }
}

impl<K: Into<Key>, V: Into<Value>> Extend<(K, V)> for InlineTable {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            let key = key.into();
            let value = Item::Value(value.into());
            let value = TableKeyValue::new(key, value);
            self.items
                .insert(InternalString::from(value.key.get()), value);
        }
    }
}

impl<K: Into<Key>, V: Into<Value>> FromIterator<(K, V)> for InlineTable {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut table = InlineTable::new();
        table.extend(iter);
        table
    }
}

impl IntoIterator for InlineTable {
    type Item = (InternalString, Value);
    type IntoIter = InlineTableIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.items
                .into_iter()
                .filter(|(_, kv)| kv.value.is_value())
                .map(|(k, kv)| (k, kv.value.into_value().unwrap())),
        )
    }
}

impl<'s> IntoIterator for &'s InlineTable {
    type Item = (&'s str, &'s Value);
    type IntoIter = InlineTableIter<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn decorate_inline_table(table: &mut InlineTable) {
    for (key_decor, value) in table
        .items
        .iter_mut()
        .filter(|&(_, ref kv)| kv.value.is_value())
        .map(|(_, kv)| (&mut kv.key.decor, kv.value.as_value_mut().unwrap()))
    {
        key_decor.clear();
        value.decor_mut().clear();
    }
}

/// An owned iterator type over key/value pairs of an inline table.
pub type InlineTableIntoIter = Box<dyn Iterator<Item = (InternalString, Value)>>;
/// An iterator type over key/value pairs of an inline table.
pub type InlineTableIter<'a> = Box<dyn Iterator<Item = (&'a str, &'a Value)> + 'a>;
/// A mutable iterator type over key/value pairs of an inline table.
pub type InlineTableIterMut<'a> = Box<dyn Iterator<Item = (KeyMut<'a>, &'a mut Value)> + 'a>;

impl TableLike for InlineTable {
    fn iter(&self) -> Iter<'_> {
        Box::new(self.items.iter().map(|(key, kv)| (&key[..], &kv.value)))
    }
    fn iter_mut(&mut self) -> IterMut<'_> {
        Box::new(
            self.items
                .iter_mut()
                .map(|(_, kv)| (kv.key.as_mut(), &mut kv.value)),
        )
    }
    fn clear(&mut self) {
        self.clear();
    }
    fn entry<'a>(&'a mut self, key: &str) -> crate::Entry<'a> {
        // Accept a `&str` rather than an owned type to keep `InternalString`, well, internal
        match self.items.entry(key.into()) {
            indexmap::map::Entry::Occupied(entry) => {
                crate::Entry::Occupied(crate::OccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                crate::Entry::Vacant(crate::VacantEntry { entry, key: None })
            }
        }
    }
    fn entry_format<'a>(&'a mut self, key: &Key) -> crate::Entry<'a> {
        // Accept a `&Key` to be consistent with `entry`
        match self.items.entry(key.get().into()) {
            indexmap::map::Entry::Occupied(entry) => {
                crate::Entry::Occupied(crate::OccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => crate::Entry::Vacant(crate::VacantEntry {
                entry,
                key: Some(key.to_owned()),
            }),
        }
    }
    fn get<'s>(&'s self, key: &str) -> Option<&'s Item> {
        self.items.get(key).map(|kv| &kv.value)
    }
    fn get_mut<'s>(&'s mut self, key: &str) -> Option<&'s mut Item> {
        self.items.get_mut(key).map(|kv| &mut kv.value)
    }
    fn get_key_value<'a>(&'a self, key: &str) -> Option<(&'a Key, &'a Item)> {
        self.get_key_value(key)
    }
    fn get_key_value_mut<'a>(&'a mut self, key: &str) -> Option<(KeyMut<'a>, &'a mut Item)> {
        self.get_key_value_mut(key)
    }
    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }
    fn insert(&mut self, key: &str, value: Item) -> Option<Item> {
        self.insert(key, value.into_value().unwrap())
            .map(Item::Value)
    }
    fn remove(&mut self, key: &str) -> Option<Item> {
        self.remove(key).map(Item::Value)
    }

    fn get_values(&self) -> Vec<(Vec<&Key>, &Value)> {
        self.get_values()
    }
    fn fmt(&mut self) {
        self.fmt()
    }
    fn sort_values(&mut self) {
        self.sort_values()
    }
    fn set_dotted(&mut self, yes: bool) {
        self.set_dotted(yes)
    }
    fn is_dotted(&self) -> bool {
        self.is_dotted()
    }

    fn key_decor_mut(&mut self, key: &str) -> Option<&mut Decor> {
        self.key_decor_mut(key)
    }
    fn key_decor(&self, key: &str) -> Option<&Decor> {
        self.key_decor(key)
    }
}

// `{ key1 = value1, ... }`
pub(crate) const DEFAULT_INLINE_KEY_DECOR: (&str, &str) = (" ", " ");

/// A view into a single location in a map, which may be vacant or occupied.
pub enum InlineEntry<'a> {
    /// An occupied Entry.
    Occupied(InlineOccupiedEntry<'a>),
    /// A vacant Entry.
    Vacant(InlineVacantEntry<'a>),
}

impl<'a> InlineEntry<'a> {
    /// Returns the entry key
    ///
    /// # Examples
    ///
    /// ```
    /// use toml_edit::Table;
    ///
    /// let mut map = Table::new();
    ///
    /// assert_eq!("hello", map.entry("hello").key());
    /// ```
    pub fn key(&self) -> &str {
        match self {
            InlineEntry::Occupied(e) => e.key(),
            InlineEntry::Vacant(e) => e.key(),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: Value) -> &'a mut Value {
        match self {
            InlineEntry::Occupied(entry) => entry.into_mut(),
            InlineEntry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> Value>(self, default: F) -> &'a mut Value {
        match self {
            InlineEntry::Occupied(entry) => entry.into_mut(),
            InlineEntry::Vacant(entry) => entry.insert(default()),
        }
    }
}

/// A view into a single occupied location in a `IndexMap`.
pub struct InlineOccupiedEntry<'a> {
    entry: indexmap::map::OccupiedEntry<'a, InternalString, TableKeyValue>,
}

impl<'a> InlineOccupiedEntry<'a> {
    /// Gets a reference to the entry key
    ///
    /// # Examples
    ///
    /// ```
    /// use toml_edit::Table;
    ///
    /// let mut map = Table::new();
    ///
    /// assert_eq!("foo", map.entry("foo").key());
    /// ```
    pub fn key(&self) -> &str {
        self.entry.key().as_str()
    }

    /// Gets a mutable reference to the entry key
    pub fn key_mut(&mut self) -> KeyMut<'_> {
        self.entry.get_mut().key.as_mut()
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &Value {
        self.entry.get().value.as_value().unwrap()
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut Value {
        self.entry.get_mut().value.as_value_mut().unwrap()
    }

    /// Converts the OccupiedEntry into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself
    pub fn into_mut(self) -> &'a mut Value {
        self.entry.into_mut().value.as_value_mut().unwrap()
    }

    /// Sets the value of the entry, and returns the entry's old value
    pub fn insert(&mut self, value: Value) -> Value {
        let mut value = Item::Value(value);
        std::mem::swap(&mut value, &mut self.entry.get_mut().value);
        value.into_value().unwrap()
    }

    /// Takes the value out of the entry, and returns it
    pub fn remove(self) -> Value {
        self.entry.shift_remove().value.into_value().unwrap()
    }
}

/// A view into a single empty location in a `IndexMap`.
pub struct InlineVacantEntry<'a> {
    entry: indexmap::map::VacantEntry<'a, InternalString, TableKeyValue>,
    key: Option<Key>,
}

impl<'a> InlineVacantEntry<'a> {
    /// Gets a reference to the entry key
    ///
    /// # Examples
    ///
    /// ```
    /// use toml_edit::Table;
    ///
    /// let mut map = Table::new();
    ///
    /// assert_eq!("foo", map.entry("foo").key());
    /// ```
    pub fn key(&self) -> &str {
        self.entry.key().as_str()
    }

    /// Sets the value of the entry with the VacantEntry's key,
    /// and returns a mutable reference to it
    pub fn insert(self, value: Value) -> &'a mut Value {
        let entry = self.entry;
        let key = self.key.unwrap_or_else(|| Key::new(entry.key().as_str()));
        let value = Item::Value(value);
        entry
            .insert(TableKeyValue::new(key, value))
            .value
            .as_value_mut()
            .unwrap()
    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use crate::{InlineTable, Item, Value};

    #[test]
    fn extend_inline_table() {
        let mut table = InlineTable::new();
        let key_value_pairs = vec![
            ("key1", Value::from(42)),
            ("key2", Value::from("value2")),
        ];

        table.extend(key_value_pairs);

        // Check table length
        assert_eq!(table.len(), 2);

        // Check content
        let key1 = table.get("key1").unwrap().as_integer().unwrap();
        let key2 = table.get("key2").unwrap().as_str().unwrap();

        assert_eq!(key1, 42);
        assert_eq!(key2, "value2");
    }
}#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;

use crate::*;
    use crate::table::TableLike;

    #[test]
    fn test_clear_empty_inline_table() {
        let mut table = InlineTable::new();
        assert_eq!(table.is_empty(), true);
        table.clear();
        assert_eq!(table.is_empty(), true);
    }

    #[test]
    fn test_clear_non_empty_inline_table() {
        let mut table = InlineTable::new();
        table.insert("key1", Value::from("val1"));
        table.insert("key2", Value::from("val2"));
        assert_eq!(table.is_empty(), false);
        table.clear();
        assert_eq!(table.is_empty(), true);
    }
}#[cfg(test)]
mod tests_llm_16_26 {
    use crate::{
        InlineTable,
        table::TableLike,
    };

    #[test]
    fn test_contains_key_existing() {
        let mut table = InlineTable::new();
        table.insert("existing_key", "value".into());
        assert!(table.contains_key("existing_key"));
    }

    #[test]
    fn test_contains_key_missing() {
        let table = InlineTable::new();
        assert!(!table.contains_key("missing_key"));
    }

    #[test]
    fn test_contains_key_with_dotted() {
        let mut table = InlineTable::new();
        table.set_dotted(true);
        // Assuming the implementation allows adding dotted keys to a dotted table
        // This might involve a different API, but for the purposes of the example, we use `insert`
        table.insert("dotted.key", "value".into());
        assert!(table.contains_key("dotted.key"));
    }
}#[cfg(test)]
mod tests_llm_16_31 {
    use crate::{InlineTable, Item, Value};

    #[test]
    fn test_get_key_value_mut_existing_key() {
        let mut table = InlineTable::new();
        table.insert("key", Value::from("value"));
        if let Some((mut key, value_mut)) = table.get_key_value_mut("key") {
            assert_eq!(key.get(), "key");
            assert_eq!(value_mut.is_value(), true);
            if let Item::Value(value) = value_mut {
                assert_eq!(value.as_str(), Some("value"));
                // Modify the value to test mutability
                *value = Value::from("new_value");
            } else {
                panic!("Expected a value item");
            }
        } else {
            panic!("Expected to find the key value pair");
        }

        assert_eq!(table.get("key").unwrap().as_str(), Some("new_value"));
    }

    #[test]
    fn test_get_key_value_mut_non_existing_key() {
        let mut table = InlineTable::new();
        assert!(table.get_key_value_mut("missing_key").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;

use crate::*;

    #[test]
    fn it_checks_inline_table_as_not_dotted() {
        let table = InlineTable::new();
        assert_eq!(table.is_dotted(), false);
    }

    #[test]
    fn it_checks_inline_table_as_dotted() {
        let mut table = InlineTable::new();
        table.set_dotted(true);
        assert_eq!(table.is_dotted(), true);
    }
}#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use crate::{
        table::{TableLike},
        InlineTable, Value
    };

    #[test]
    fn key_decor_mut_existing_key() {
        let mut inline_table = InlineTable::new();
        inline_table.insert("key", Value::from("value"));
        let decor = inline_table.key_decor_mut("key");
        assert!(decor.is_some());
    }

    #[test]
    fn key_decor_mut_missing_key() {
        let mut inline_table = InlineTable::new();
        let decor = inline_table.key_decor_mut("key");
        assert!(decor.is_none());
    }

    #[test]
    fn key_decor_mut_modify_decor() {
        let mut inline_table = InlineTable::new();
        inline_table.insert("key", Value::from("value"));
        {
            let decor = inline_table.key_decor_mut("key").unwrap();
            decor.set_prefix(" ");
            decor.set_suffix(" ");
        }
        let decor = inline_table.key_decor("key").unwrap();
        let expected_prefix = " ";
        let expected_suffix = " ";
        assert_eq!(decor.prefix().unwrap().as_str().unwrap(), expected_prefix);
        assert_eq!(decor.suffix().unwrap().as_str().unwrap(), expected_suffix);
    }
}#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;

use crate::*;

    #[test]
    fn test_set_dotted() {
        let mut table = InlineTable::new();
        assert!(!table.is_dotted()); // Initially not dotted
        table.set_dotted(true);
        assert!(table.is_dotted()); // Should be dotted after setting to true
        table.set_dotted(false);
        assert!(!table.is_dotted()); // Should not be dotted after setting to false
    }

    // Provide additional tests as necessary for your specific use cases
}#[cfg(test)]
mod tests_llm_16_211 {
    use crate::inline_table::{InlineTable, InlineEntry};
    use crate::value::Value;

    #[test]
    fn test_or_insert_with_occupied() {
        let mut table = InlineTable::new();
        table.insert("key", Value::from(42));
        let value = table.entry("key").or_insert_with(|| Value::from(99));
        assert_eq!(value.as_integer(), Some(42));
    }
    
    #[test]
    fn test_or_insert_with_vacant() {
        let mut table = InlineTable::new();
        let value = table.entry("key").or_insert_with(|| Value::from(99));
        assert_eq!(value.as_integer(), Some(99));
    }
}#[cfg(test)]
mod tests_llm_16_212_llm_16_212 {
    use crate::{Document, Item, inline_table::InlineTable, value::Value};

    #[test]
    fn test_get_inline_table_value() {
        let toml_str = r#"
        [my_table]
        key = "value"
        "#;
        let mut doc = toml_str.parse::<Document>().expect("Parsing toml string");
        let my_table = doc["my_table"].as_inline_table_mut().unwrap();
        let occupied_entry = my_table.get_mut("key").unwrap();

        assert_eq!(occupied_entry.as_str(), Some("value"));
    }
}#[cfg(test)]
mod tests_llm_16_220 {
    use crate::inline_table::InlineTable;
    use crate::Value;

    #[test]
    fn clear_table() {
        let mut table = InlineTable::new();
        table.insert("key1", Value::from("value1"));
        table.insert("key2", Value::from("value2"));
        table.insert("key3", Value::from("value3"));
        assert!(!table.is_empty());
        table.clear();
        assert!(table.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_223 {
    use super::*;

use crate::*;
    use crate::repr::Decor;

    #[test]
    fn decor_mut_returns_mut_decor() {
        let mut table = InlineTable::new();
        let decor = table.decor_mut();
        assert_eq!(Decor::default(), *decor);

        let new_decor = Decor::new("/* Prefix */", "/* Suffix */");
        *decor = new_decor.clone();
        assert_eq!(new_decor, *table.decor_mut());
    }
}#[cfg(test)]
mod tests_llm_16_226 {
    use crate::{InlineTable, Value, Key, Item};

    #[test]
    fn test_entry_format_occupied() {
        let mut table = InlineTable::new();
        let key = Key::new("foo");
        table.get_or_insert("foo", Value::from(42));
        let entry = table.entry_format(&key);
        assert!(matches!(entry, crate::InlineEntry::Occupied(_)));
        assert_eq!(entry.key(), "foo");
    }

    #[test]
    fn test_entry_format_vacant() {
        let mut table = InlineTable::new();
        let key = Key::new("foo");
        let entry = table.entry_format(&key);
        assert!(matches!(entry, crate::InlineEntry::Vacant(_)));
        assert_eq!(entry.key(), "foo");
    }

    #[test]
    fn test_entry_format_vacant_insert() {
        let mut table = InlineTable::new();
        let key = Key::new("foo");
        let entry = table.entry_format(&key);
        let value = match entry {
            crate::InlineEntry::Vacant(v) => v.insert(Value::from(42)),
            _ => unreachable!(),
        };
        assert_eq!(value.as_integer(), Some(42));
        assert_eq!(table.get("foo").unwrap().as_integer(), Some(42));
    }

    #[test]
    fn test_entry_format_occupied_insert() {
        let mut table = InlineTable::new();
        let key = Key::new("foo");
        table.get_or_insert("foo", Value::from(10));
        let mut entry = table.entry_format(&key);
        let value = match &mut entry {
            crate::InlineEntry::Occupied(o) => {
                let value = o.get_mut();
                *value = Value::from(42);
                value
            }
            _ => unreachable!(),
        };
        assert_eq!(value.as_integer(), Some(42));
        assert_eq!(table.get("foo").unwrap().as_integer(), Some(42));
    }

    #[test]
    fn test_entry_format_or_insert() {
        let mut table = InlineTable::new();
        let key = Key::new("foo");
        let value = table.entry_format(&key).or_insert(Value::from(42));
        assert_eq!(value.as_integer(), Some(42));
        assert_eq!(table.get("foo").unwrap().as_integer(), Some(42));
    }

    #[test]
    fn test_entry_format_or_insert_with() {
        let mut table = InlineTable::new();
        let key = Key::new("foo");
        let value = table.entry_format(&key).or_insert_with(|| Value::from(42));
        assert_eq!(value.as_integer(), Some(42));
        assert_eq!(table.get("foo").unwrap().as_integer(), Some(42));
    }
}#[cfg(test)]
mod tests_llm_16_227 {
    use crate::{InlineTable, InternalString, Value};

    #[test]
    fn test_fmt() {
        let mut table = InlineTable::new();
        table.insert("a".to_owned(), Value::from(42));
        table.insert("b".to_owned(), Value::from("value"));
        table.fmt();
        let output = table.to_string();
        assert_eq!(output, r#"{a=42, b="value"}"#);
    }
}#[cfg(test)]
mod tests_llm_16_229_llm_16_229 {
    use super::*;

use crate::*;
    use crate::table::TableKeyValue;
    use crate::{Item, Value};

    #[test]
    fn get_key_value_existing_key() {
        let mut table = InlineTable::new();
        let key = "key1";
        let value = Value::from("value1");
        table.insert(key, value);

        let retrieved = table.get_key_value(key);

        assert!(retrieved.is_some());
        let (retrieved_key, retrieved_item) = retrieved.unwrap();
        assert_eq!(retrieved_key.get(), key);
        assert_eq!(retrieved_item.as_value().unwrap().as_str(), Some("value1"));
    }

    #[test]
    fn get_key_value_non_existing_key() {
        let table = InlineTable::new();
        let non_existing_key = "key2";

        let retrieved = table.get_key_value(non_existing_key);

        assert!(retrieved.is_none());
    }

    #[test]
    fn get_key_value_for_empty_value() {
        let mut table = InlineTable::new();
        let key = "key3";
        let value = Item::None;
        table.items.insert(key.into(), TableKeyValue::new(key.into(), value));

        let retrieved = table.get_key_value(key);

        assert!(retrieved.is_none());
    }
}#[cfg(test)]
mod tests_llm_16_230_llm_16_230 {
    use crate::{Item, Value, InlineTable};

    #[test]
    fn test_get_key_value_mut() {
        let mut table = InlineTable::new();
        table.get_or_insert("a_key", "a_value");
        assert!(table.get_key_value_mut("a_key").is_some());
        assert!(table.get_key_value_mut("non_existent_key").is_none());
        
        if let Some((key_mut, item_mut)) = table.get_key_value_mut("a_key") {
            assert_eq!(key_mut.get(), "a_key");
            if let Item::Value(value) = item_mut {
                assert_eq!(value.as_str(), Some("a_value"));
            } else {
                panic!("Item is not a value");
            }
        } else {
            panic!("Key-value pair not found");
        }
        
        // Ensure the mutable reference can actually modify the item
        {
            let item_mut = table.get_mut("a_key").unwrap();
            *item_mut = Value::from("modified_value");
        }
        assert_eq!(table.get("a_key").unwrap().as_str(), Some("modified_value"));
    }
}#[cfg(test)]
mod tests_llm_16_231 {
    use crate::{InlineTable, Value};

    #[test]
    fn test_get_mut_existing_key() {
        let mut table = InlineTable::new();
        table.insert("key1", Value::from("value1"));
        if let Some(value) = table.get_mut("key1") {
            assert_eq!(value.as_str().unwrap(), "value1");
        } else {
            panic!("Expected a Value for key `key1`");
        }
    }

    #[test]
    fn test_get_mut_non_existing_key() {
        let mut table = InlineTable::new();
        table.insert("key1", Value::from("value1"));
        assert!(table.get_mut("key2").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_232 {
    use super::*;

use crate::*;
    use crate::{InternalString, Value, InlineTable};

    #[test]
    fn get_or_insert_non_existing_key() {
        let mut table = InlineTable::new();
        assert!(table.get("key").is_none());
        let value = table.get_or_insert("key", 42);
        assert_eq!(value.as_integer(), Some(42));
        assert_eq!(table.get("key").unwrap().as_integer(), Some(42));
    }

    #[test]
    fn get_or_insert_existing_key() {
        let mut table = InlineTable::new();
        table.get_or_insert("key", "initial value");
        {
            let value = table.get_or_insert("key", 42);
            assert_eq!(value.as_str(), Some("initial value"));
            *value = Value::from(10);
        }
        assert_eq!(table.get("key").unwrap().as_integer(), Some(10));
    }
}#[cfg(test)]
mod tests_llm_16_236_llm_16_236 {
    use super::*;

use crate::*;
    use crate::table::Table;
    use crate::value::Value;
    use crate::table::TableLike;
    use crate::inline_table::InlineTable;
    use crate::internal_string::InternalString;

    #[test]
    fn test_into_table() {
        let mut inline_table = InlineTable::new();
        inline_table.insert(InternalString::from("key"), Value::from("value"));
        let table = inline_table.into_table();

        assert!(!table.is_empty());
        assert!(table.contains_key("key"));
        assert_eq!(table.get("key").unwrap().as_value().unwrap().as_str().unwrap(), "value");
    }
}#[cfg(test)]
mod tests_llm_16_237 {
    use super::*;

use crate::*;

    #[test]
    fn test_is_dotted_false_for_empty_table() {
        let table = InlineTable::new();
        assert!(!table.is_dotted());
    }

    #[test]
    fn test_is_dotted_false_for_standard_table() {
        let mut table = InlineTable::new();
        table.set_dotted(false);
        assert!(!table.is_dotted());
    }

    #[test]
    fn test_is_dotted_true_for_dotted_table() {
        let mut table = InlineTable::new();
        table.set_dotted(true);
        assert!(table.is_dotted());
    }
}#[cfg(test)]
mod tests_llm_16_239 {
    use crate::{Document, InlineTable, Value};

    #[test]
    fn iter_empty_inline_table() {
        let table = InlineTable::new();
        let mut iter = table.iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_single_pair_inline_table() {
        let mut table = InlineTable::new();
        table.insert("key", Value::from("value"));
        let mut iter = table.iter();
        let (key, value) = iter.next().unwrap();
        assert_eq!(key, "key");
        assert_eq!(value.as_str().unwrap(), "value");
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_multiple_pairs_inline_table() {
        let mut table = InlineTable::new();
        table.insert("first", Value::from(123));
        table.insert("second", Value::from(456));
        let mut iter = table.iter();
        let (key1, value1) = iter.next().unwrap();
        assert_eq!(key1, "first");
        assert_eq!(value1.as_integer().unwrap(), 123);
        let (key2, value2) = iter.next().unwrap();
        assert_eq!(key2, "second");
        assert_eq!(value2.as_integer().unwrap(), 456);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_dotted_pairs_inline_table() {
        let mut table = InlineTable::new();
        table.insert("parent.first", Value::from("value1"));
        table.insert("parent.second", Value::from("value2"));
        table.set_dotted(true);
        let mut iter = table.iter();
        let (path, value) = iter.next().unwrap();
        assert_eq!(path, "parent.first");
        assert_eq!(value.as_str().unwrap(), "value1");
        let (path, value) = iter.next().unwrap();
        assert_eq!(path, "parent.second");
        assert_eq!(value.as_str().unwrap(), "value2");
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_nested_inline_table() {
        let mut doc = "title = 'TOML Example'\n[owner]\nname = 'Tom Preston-Werner'\n".parse::<Document>().unwrap();
        let table = doc["owner"].as_inline_table().unwrap();
        let mut iter = table.iter();
        let (key, value) = iter.next().unwrap();
        assert_eq!(key, "name");
        assert_eq!(value.as_str().unwrap(), "Tom Preston-Werner");
        assert!(iter.next().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_241_llm_16_241 {
    use crate::{Decor, InlineTable, RawString, Value};

    #[test]
    fn test_key_decor() {
        let mut table = InlineTable::new();
        assert_eq!(table.key_decor("key1"), None);

        table.decor_mut().set_prefix(RawString::from(" "));
        table.decor_mut().set_suffix(RawString::from(" "));
        assert_eq!(table.key_decor("key1"), None);

        table.insert("key1", Value::from(42));
        assert!(table.key_decor("key1").is_some());
        assert_eq!(table.key_decor("key1").unwrap().prefix().unwrap().as_str(), Some(""));
        assert_eq!(table.key_decor("key1").unwrap().suffix().unwrap().as_str(), Some(""));

        table.key_decor_mut("key1").unwrap().set_prefix(RawString::from("  "));
        table.key_decor_mut("key1").unwrap().set_suffix(RawString::from("  "));
        assert_eq!(table.key_decor("key1").unwrap().prefix().unwrap().as_str(), Some("  "));
        assert_eq!(table.key_decor("key1").unwrap().suffix().unwrap().as_str(), Some("  "));
    }
}#[cfg(test)]
mod tests_llm_16_243_llm_16_243 {
    use crate::inline_table::InlineTable;
    use crate::value::Value;
    use crate::internal_string::InternalString;

    #[test]
    fn test_len_empty_table() {
        let table = InlineTable::new();
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_len_with_items() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        table.insert(InternalString::from("key2"), Value::from("value"));
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn test_len_after_removal() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        table.insert(InternalString::from("key2"), Value::from("value"));
        table.remove("key1");
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn test_len_after_clear() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        table.insert(InternalString::from("key2"), Value::from("value"));
        table.clear();
        assert_eq!(table.len(), 0);
    }
}#[cfg(test)]
mod tests_llm_16_244 {
    use crate::inline_table::InlineTable;

    #[test]
    fn test_new_inline_table() {
        let table = InlineTable::new();
        assert!(table.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_245 {
    use super::*;

use crate::*;
    use crate::inline_table::InlineTable;
    use crate::raw_string::RawString;

    #[test]
    fn test_preamble_empty_table() {
        let table = InlineTable::new();
        let preamble = table.preamble();
        assert_eq!(preamble.as_str(), Some(""));
    }

    #[test]
    fn test_preamble_with_raw_string() {
        let mut table = InlineTable::new();
        let raw_string: RawString = " ".into();
        table.set_preamble(raw_string.clone());
        let preamble = table.preamble();
        assert_eq!(preamble, &raw_string);
    }
}#[cfg(test)]
mod tests_llm_16_246 {
    use crate::inline_table::InlineTable;
    use crate::value::Value;
    use crate::InternalString;

    #[test]
    fn test_remove_existing_key() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        table.insert(InternalString::from("key2"), Value::from("value2"));
        assert_eq!(table.len(), 2);
        let removed = table.remove("key1").unwrap();
        assert_eq!(removed.as_integer().unwrap(), 42);
        assert_eq!(table.len(), 1);
        assert!(table.get("key2").is_some());
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        assert_eq!(table.len(), 1);
        let removed = table.remove("key2");
        assert!(removed.is_none());
        assert_eq!(table.len(), 1);
        assert!(table.get("key1").is_some());
    }

    #[test]
    fn test_remove_key_from_empty_table() {
        let mut table = InlineTable::new();
        let removed = table.remove("key1");
        assert!(removed.is_none());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_remove_only_key() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        assert_eq!(table.len(), 1);
        let removed = table.remove("key1").unwrap();
        assert_eq!(removed.as_integer().unwrap(), 42);
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_remove_and_reinsert_key() {
        let mut table = InlineTable::new();
        table.insert(InternalString::from("key1"), Value::from(42));
        assert_eq!(table.len(), 1);
        let removed = table.remove("key1").unwrap();
        assert_eq!(removed.as_integer().unwrap(), 42);
        assert_eq!(table.len(), 0);

        table.insert(InternalString::from("key1"), Value::from(100));
        assert_eq!(table.len(), 1);
        let value = table.get("key1").unwrap();
        assert_eq!(value.as_integer().unwrap(), 100);
    }
}#[cfg(test)]
mod tests_llm_16_247 {
    use crate::{InlineTable, Value};

    #[test]
    fn test_remove_entry() {
        let mut table = InlineTable::new();
        table.insert("key1", Value::from("value1"));
        table.insert("key2", Value::from("value2"));
        
        // Remove an existing key
        let removed = table.remove_entry("key1");
        assert!(removed.is_some());
        let (key, value) = removed.unwrap();
        assert_eq!(key.get(), "key1");
        assert_eq!(value.as_str(), Some("value1"));
        
        // Verify key1 is removed
        assert!(!table.contains_key("key1"));
        
        // Verify key2 is still present
        assert!(table.contains_key("key2"));
        
        // Try to remove a non-existent key
        assert!(table.remove_entry("non_existent_key").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_248 {
    use super::*;

use crate::*;

    #[test]
    fn test_set_dotted() {
        let mut inline_table = InlineTable::new();
        assert_eq!(inline_table.is_dotted(), false);
        inline_table.set_dotted(true);
        assert_eq!(inline_table.is_dotted(), true);
        inline_table.set_dotted(false);
        assert_eq!(inline_table.is_dotted(), false);
    }
}#[cfg(test)]
mod tests_llm_16_249 {
    use super::*;

use crate::*;

    #[test]
    fn test_set_preamble_empty() {
        let mut inline_table = InlineTable::new();
        inline_table.set_preamble("");
        assert_eq!(inline_table.preamble().as_str(), Some(""));
    }

    #[test]
    fn test_set_preamble_whitespace() {
        let mut inline_table = InlineTable::new();
        inline_table.set_preamble("  ");
        assert_eq!(inline_table.preamble().as_str(), Some("  "));
    }

    #[test]
    fn test_set_preamble_with_content() {
        let mut inline_table = InlineTable::new();
        inline_table.set_preamble("  # Comment");
        assert_eq!(inline_table.preamble().as_str(), Some("  # Comment"));
    }

    #[test]
    fn test_set_preamble_twice_overwrites() {
        let mut inline_table = InlineTable::new();
        inline_table.set_preamble("First");
        inline_table.set_preamble("Second");
        assert_eq!(inline_table.preamble().as_str(), Some("Second"));
    }
}#[cfg(test)]
mod tests_llm_16_253 {
    use crate::{Document, InlineTable};
    use std::ops::Range;

    #[test]
    fn test_span() {
        // Create a new inline table
        let mut table = InlineTable::new();
        // Assert span is none for a new table
        assert!(table.span().is_none());

        // Parse a document with inline table to get a span
        let mut doc = "key = { inner_key = 'inner_value' }".parse::<Document>().expect("document to be valid");
        let inline_table_span = doc["key"].as_inline_table().expect("key to be an inline table").span();
        // Assert we have a span
        assert!(inline_table_span.is_some());

        // Create an inline table with a set span
        let manual_span = Range { start: 7, end: 37 };
        let mut table_with_span = InlineTable::new();
        table_with_span.span = Some(manual_span.clone());
        // Assert span matches the one we set
        assert_eq!(table_with_span.span(), Some(manual_span));

        // Modify the inline table, despanning it
        table_with_span.despan("");
        // Span should be now gone after despanning
        assert!(table_with_span.span().is_none());
    }
}