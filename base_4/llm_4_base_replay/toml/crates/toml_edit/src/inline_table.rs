use std::iter::FromIterator;
use crate::key::Key;
use crate::repr::Decor;
use crate::table::{Iter, IterMut, KeyValuePairs, TableKeyValue, TableLike};
use crate::{InternalString, Item, KeyMut, RawString, Table, Value};
/// Type representing a TOML inline table,
/// payload of the `Value::InlineTable` variant
#[derive(Debug, Default, Clone)]
pub struct InlineTable {
    preamble: RawString,
    decor: Decor,
    pub(crate) span: Option<std::ops::Range<usize>>,
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
        let modified_cmp = |
            _: &InternalString,
            val1: &TableKeyValue,
            _: &InternalString,
            val2: &TableKeyValue,
        | -> std::cmp::Ordering {
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
            self
                .items
                .iter()
                .filter(|&(_, kv)| kv.value.is_value())
                .map(|(k, kv)| (&k[..], kv.value.as_value().unwrap())),
        )
    }
    /// Returns an iterator over key/value pairs.
    pub fn iter_mut(&mut self) -> InlineTableIterMut<'_> {
        Box::new(
            self
                .items
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
                let scratch = std::mem::take(&mut entry.get_mut().value);
                let scratch = Item::Value(
                    scratch
                        .into_value()
                        .unwrap_or_else(|_| Value::InlineTable(Default::default())),
                );
                entry.get_mut().value = scratch;
                InlineEntry::Occupied(InlineOccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                InlineEntry::Vacant(InlineVacantEntry {
                    entry,
                    key: None,
                })
            }
        }
    }
    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    pub fn entry_format<'a>(&'a mut self, key: &Key) -> InlineEntry<'a> {
        match self.items.entry(key.get().into()) {
            indexmap::map::Entry::Occupied(mut entry) => {
                let scratch = std::mem::take(&mut entry.get_mut().value);
                let scratch = Item::Value(
                    scratch
                        .into_value()
                        .unwrap_or_else(|_| Value::InlineTable(Default::default())),
                );
                entry.get_mut().value = scratch;
                InlineEntry::Occupied(InlineOccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                InlineEntry::Vacant(InlineVacantEntry {
                    entry,
                    key: Some(key.clone()),
                })
            }
        }
    }
    /// Return an optional reference to the value at the given the key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.items.get(key).and_then(|kv| kv.value.as_value())
    }
    /// Return an optional mutable reference to the value at the given the key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.items.get_mut(key).and_then(|kv| kv.value.as_value_mut())
    }
    /// Return references to the key-value pair stored for key, if it is present, else None.
    pub fn get_key_value<'a>(&'a self, key: &str) -> Option<(&'a Key, &'a Item)> {
        self.items
            .get(key)
            .and_then(|kv| {
                if !kv.value.is_none() { Some((&kv.key, &kv.value)) } else { None }
            })
    }
    /// Return mutable references to the key-value pair stored for key, if it is present, else None.
    pub fn get_key_value_mut<'a>(
        &'a mut self,
        key: &str,
    ) -> Option<(KeyMut<'a>, &'a mut Item)> {
        self.items
            .get_mut(key)
            .and_then(|kv| {
                if !kv.value.is_none() {
                    Some((kv.key.as_mut(), &mut kv.value))
                } else {
                    None
                }
            })
    }
    /// Returns true iff the table contains given key.
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) { kv.value.is_value() } else { false }
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
    pub fn insert(
        &mut self,
        key: impl Into<InternalString>,
        value: Value,
    ) -> Option<Value> {
        let key = key.into();
        let kv = TableKeyValue::new(Key::new(key.clone()), Item::Value(value));
        self.items.insert(key, kv).and_then(|kv| kv.value.into_value().ok())
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
        self.items.shift_remove(key).and_then(|kv| kv.value.into_value().ok())
    }
    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    pub fn remove_entry(&mut self, key: &str) -> Option<(Key, Value)> {
        self.items
            .shift_remove(key)
            .and_then(|kv| {
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
            self.items.insert(InternalString::from(value.key.get()), value);
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
            self
                .items
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
pub type InlineTableIterMut<'a> = Box<
    dyn Iterator<Item = (KeyMut<'a>, &'a mut Value)> + 'a,
>;
impl TableLike for InlineTable {
    fn iter(&self) -> Iter<'_> {
        Box::new(self.items.iter().map(|(key, kv)| (&key[..], &kv.value)))
    }
    fn iter_mut(&mut self) -> IterMut<'_> {
        Box::new(self.items.iter_mut().map(|(_, kv)| (kv.key.as_mut(), &mut kv.value)))
    }
    fn clear(&mut self) {
        self.clear();
    }
    fn entry<'a>(&'a mut self, key: &str) -> crate::Entry<'a> {
        match self.items.entry(key.into()) {
            indexmap::map::Entry::Occupied(entry) => {
                crate::Entry::Occupied(crate::OccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                crate::Entry::Vacant(crate::VacantEntry {
                    entry,
                    key: None,
                })
            }
        }
    }
    fn entry_format<'a>(&'a mut self, key: &Key) -> crate::Entry<'a> {
        match self.items.entry(key.get().into()) {
            indexmap::map::Entry::Occupied(entry) => {
                crate::Entry::Occupied(crate::OccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                crate::Entry::Vacant(crate::VacantEntry {
                    entry,
                    key: Some(key.to_owned()),
                })
            }
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
    fn get_key_value_mut<'a>(
        &'a mut self,
        key: &str,
    ) -> Option<(KeyMut<'a>, &'a mut Item)> {
        self.get_key_value_mut(key)
    }
    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }
    fn insert(&mut self, key: &str, value: Item) -> Option<Item> {
        self.insert(key, value.into_value().unwrap()).map(Item::Value)
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
        entry.insert(TableKeyValue::new(key, value)).value.as_value_mut().unwrap()
    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use crate::{InlineTable, Item, Value};
    #[test]
    fn extend_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key_value_pairs = vec![
            (rug_fuzz_0, Value::from(rug_fuzz_1)), ("key2", Value::from("value2"))
        ];
        table.extend(key_value_pairs);
        debug_assert_eq!(table.len(), 2);
        let key1 = table.get(rug_fuzz_2).unwrap().as_integer().unwrap();
        let key2 = table.get(rug_fuzz_3).unwrap().as_str().unwrap();
        debug_assert_eq!(key1, 42);
        debug_assert_eq!(key2, "value2");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;
    use crate::*;
    use crate::table::TableLike;
    #[test]
    fn test_clear_empty_inline_table() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_test_clear_empty_inline_table = 0;
        let mut table = InlineTable::new();
        debug_assert_eq!(table.is_empty(), true);
        table.clear();
        debug_assert_eq!(table.is_empty(), true);
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_test_clear_empty_inline_table = 0;
    }
    #[test]
    fn test_clear_non_empty_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        table.insert(rug_fuzz_2, Value::from(rug_fuzz_3));
        debug_assert_eq!(table.is_empty(), false);
        table.clear();
        debug_assert_eq!(table.is_empty(), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use crate::{InlineTable, table::TableLike};
    #[test]
    fn test_contains_key_existing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, rug_fuzz_1.into());
        debug_assert!(table.contains_key(rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_contains_key_missing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = InlineTable::new();
        debug_assert!(! table.contains_key(rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_contains_key_with_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(bool, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.set_dotted(rug_fuzz_0);
        table.insert(rug_fuzz_1, rug_fuzz_2.into());
        debug_assert!(table.contains_key(rug_fuzz_3));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    use crate::{InlineTable, Item, Value};
    #[test]
    fn test_get_key_value_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        if let Some((mut key, value_mut)) = table.get_key_value_mut(rug_fuzz_2) {
            debug_assert_eq!(key.get(), "key");
            debug_assert_eq!(value_mut.is_value(), true);
            if let Item::Value(value) = value_mut {
                debug_assert_eq!(value.as_str(), Some("value"));
                *value = Value::from(rug_fuzz_3);
            } else {
                panic!("Expected a value item");
            }
        } else {
            panic!("Expected to find the key value pair");
        }
        debug_assert_eq!(table.get(rug_fuzz_4).unwrap().as_str(), Some("new_value"));
             }
}
}
}    }
    #[test]
    fn test_get_key_value_mut_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        debug_assert!(table.get_key_value_mut(rug_fuzz_0).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;
    use crate::*;
    #[test]
    fn it_checks_inline_table_as_not_dotted() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_it_checks_inline_table_as_not_dotted = 0;
        let table = InlineTable::new();
        debug_assert_eq!(table.is_dotted(), false);
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_it_checks_inline_table_as_not_dotted = 0;
    }
    #[test]
    fn it_checks_inline_table_as_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.set_dotted(rug_fuzz_0);
        debug_assert_eq!(table.is_dotted(), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use crate::{table::TableLike, InlineTable, Value};
    #[test]
    fn key_decor_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let decor = inline_table.key_decor_mut(rug_fuzz_2);
        debug_assert!(decor.is_some());
             }
}
}
}    }
    #[test]
    fn key_decor_mut_missing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        let decor = inline_table.key_decor_mut(rug_fuzz_0);
        debug_assert!(decor.is_none());
             }
}
}
}    }
    #[test]
    fn key_decor_mut_modify_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        {
            let decor = inline_table.key_decor_mut(rug_fuzz_2).unwrap();
            decor.set_prefix(rug_fuzz_3);
            decor.set_suffix(rug_fuzz_4);
        }
        let decor = inline_table.key_decor(rug_fuzz_5).unwrap();
        let expected_prefix = rug_fuzz_6;
        let expected_suffix = rug_fuzz_7;
        debug_assert_eq!(decor.prefix().unwrap().as_str().unwrap(), expected_prefix);
        debug_assert_eq!(decor.suffix().unwrap().as_str().unwrap(), expected_suffix);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;
    use crate::*;
    #[test]
    fn test_set_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        debug_assert!(! table.is_dotted());
        table.set_dotted(rug_fuzz_0);
        debug_assert!(table.is_dotted());
        table.set_dotted(rug_fuzz_1);
        debug_assert!(! table.is_dotted());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_211 {
    use crate::inline_table::{InlineTable, InlineEntry};
    use crate::value::Value;
    #[test]
    fn test_or_insert_with_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let value = table.entry(rug_fuzz_2).or_insert_with(|| Value::from(rug_fuzz_3));
        debug_assert_eq!(value.as_integer(), Some(42));
             }
}
}
}    }
    #[test]
    fn test_or_insert_with_vacant() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let value = table.entry(rug_fuzz_0).or_insert_with(|| Value::from(rug_fuzz_1));
        debug_assert_eq!(value.as_integer(), Some(99));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_212_llm_16_212 {
    use crate::{Document, Item, inline_table::InlineTable, value::Value};
    #[test]
    fn test_get_inline_table_value() {
        let _rug_st_tests_llm_16_212_llm_16_212_rrrruuuugggg_test_get_inline_table_value = 0;
        let rug_fuzz_0 = r#"
        [my_table]
        key = "value"
        "#;
        let rug_fuzz_1 = "Parsing toml string";
        let rug_fuzz_2 = "my_table";
        let rug_fuzz_3 = "key";
        let toml_str = rug_fuzz_0;
        let mut doc = toml_str.parse::<Document>().expect(rug_fuzz_1);
        let my_table = doc[rug_fuzz_2].as_inline_table_mut().unwrap();
        let occupied_entry = my_table.get_mut(rug_fuzz_3).unwrap();
        debug_assert_eq!(occupied_entry.as_str(), Some("value"));
        let _rug_ed_tests_llm_16_212_llm_16_212_rrrruuuugggg_test_get_inline_table_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_220 {
    use crate::inline_table::InlineTable;
    use crate::Value;
    #[test]
    fn clear_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        table.insert(rug_fuzz_2, Value::from(rug_fuzz_3));
        table.insert(rug_fuzz_4, Value::from(rug_fuzz_5));
        debug_assert!(! table.is_empty());
        table.clear();
        debug_assert!(table.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_223 {
    use super::*;
    use crate::*;
    use crate::repr::Decor;
    #[test]
    fn decor_mut_returns_mut_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let decor = table.decor_mut();
        debug_assert_eq!(Decor::default(), * decor);
        let new_decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        *decor = new_decor.clone();
        debug_assert_eq!(new_decor, * table.decor_mut());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_226 {
    use crate::{InlineTable, Value, Key, Item};
    #[test]
    fn test_entry_format_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = Key::new(rug_fuzz_0);
        table.get_or_insert(rug_fuzz_1, Value::from(rug_fuzz_2));
        let entry = table.entry_format(&key);
        debug_assert!(matches!(entry, crate ::InlineEntry::Occupied(_)));
        debug_assert_eq!(entry.key(), "foo");
             }
}
}
}    }
    #[test]
    fn test_entry_format_vacant() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = Key::new(rug_fuzz_0);
        let entry = table.entry_format(&key);
        debug_assert!(matches!(entry, crate ::InlineEntry::Vacant(_)));
        debug_assert_eq!(entry.key(), "foo");
             }
}
}
}    }
    #[test]
    fn test_entry_format_vacant_insert() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = Key::new(rug_fuzz_0);
        let entry = table.entry_format(&key);
        let value = match entry {
            crate::InlineEntry::Vacant(v) => v.insert(Value::from(rug_fuzz_1)),
            _ => unreachable!(),
        };
        debug_assert_eq!(value.as_integer(), Some(42));
        debug_assert_eq!(table.get(rug_fuzz_2).unwrap().as_integer(), Some(42));
             }
}
}
}    }
    #[test]
    fn test_entry_format_occupied_insert() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, i64, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = Key::new(rug_fuzz_0);
        table.get_or_insert(rug_fuzz_1, Value::from(rug_fuzz_2));
        let mut entry = table.entry_format(&key);
        let value = match &mut entry {
            crate::InlineEntry::Occupied(o) => {
                let value = o.get_mut();
                *value = Value::from(rug_fuzz_3);
                value
            }
            _ => unreachable!(),
        };
        debug_assert_eq!(value.as_integer(), Some(42));
        debug_assert_eq!(table.get(rug_fuzz_4).unwrap().as_integer(), Some(42));
             }
}
}
}    }
    #[test]
    fn test_entry_format_or_insert() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = Key::new(rug_fuzz_0);
        let value = table.entry_format(&key).or_insert(Value::from(rug_fuzz_1));
        debug_assert_eq!(value.as_integer(), Some(42));
        debug_assert_eq!(table.get(rug_fuzz_2).unwrap().as_integer(), Some(42));
             }
}
}
}    }
    #[test]
    fn test_entry_format_or_insert_with() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = Key::new(rug_fuzz_0);
        let value = table.entry_format(&key).or_insert_with(|| Value::from(rug_fuzz_1));
        debug_assert_eq!(value.as_integer(), Some(42));
        debug_assert_eq!(table.get(rug_fuzz_2).unwrap().as_integer(), Some(42));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_227 {
    use crate::{InlineTable, InternalString, Value};
    #[test]
    fn test_fmt() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        table.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        table.fmt();
        let output = table.to_string();
        debug_assert_eq!(output, r#"{a=42, b="value"}"#);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_229_llm_16_229 {
    use super::*;
    use crate::*;
    use crate::table::TableKeyValue;
    use crate::{Item, Value};
    #[test]
    fn get_key_value_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = rug_fuzz_0;
        let value = Value::from(rug_fuzz_1);
        table.insert(key, value);
        let retrieved = table.get_key_value(key);
        debug_assert!(retrieved.is_some());
        let (retrieved_key, retrieved_item) = retrieved.unwrap();
        debug_assert_eq!(retrieved_key.get(), key);
        debug_assert_eq!(retrieved_item.as_value().unwrap().as_str(), Some("value1"));
             }
}
}
}    }
    #[test]
    fn get_key_value_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = InlineTable::new();
        let non_existing_key = rug_fuzz_0;
        let retrieved = table.get_key_value(non_existing_key);
        debug_assert!(retrieved.is_none());
             }
}
}
}    }
    #[test]
    fn get_key_value_for_empty_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let key = rug_fuzz_0;
        let value = Item::None;
        table.items.insert(key.into(), TableKeyValue::new(key.into(), value));
        let retrieved = table.get_key_value(key);
        debug_assert!(retrieved.is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_230_llm_16_230 {
    use crate::{Item, Value, InlineTable};
    #[test]
    fn test_get_key_value_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.get_or_insert(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(table.get_key_value_mut(rug_fuzz_2).is_some());
        debug_assert!(table.get_key_value_mut(rug_fuzz_3).is_none());
        if let Some((key_mut, item_mut)) = table.get_key_value_mut(rug_fuzz_4) {
            debug_assert_eq!(key_mut.get(), "a_key");
            if let Item::Value(value) = item_mut {
                debug_assert_eq!(value.as_str(), Some("a_value"));
            } else {
                panic!("Item is not a value");
            }
        } else {
            panic!("Key-value pair not found");
        }
        {
            let item_mut = table.get_mut(rug_fuzz_5).unwrap();
            *item_mut = Value::from(rug_fuzz_6);
        }
        debug_assert_eq!(
            table.get(rug_fuzz_7).unwrap().as_str(), Some("modified_value")
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_231 {
    use crate::{InlineTable, Value};
    #[test]
    fn test_get_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        if let Some(value) = table.get_mut(rug_fuzz_2) {
            debug_assert_eq!(value.as_str().unwrap(), "value1");
        } else {
            panic!("Expected a Value for key `key1`");
        }
             }
}
}
}    }
    #[test]
    fn test_get_mut_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        debug_assert!(table.get_mut(rug_fuzz_2).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_232 {
    use super::*;
    use crate::*;
    use crate::{InternalString, Value, InlineTable};
    #[test]
    fn get_or_insert_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        debug_assert!(table.get(rug_fuzz_0).is_none());
        let value = table.get_or_insert(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value.as_integer(), Some(42));
        debug_assert_eq!(table.get(rug_fuzz_3).unwrap().as_integer(), Some(42));
             }
}
}
}    }
    #[test]
    fn get_or_insert_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, i64, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.get_or_insert(rug_fuzz_0, rug_fuzz_1);
        {
            let value = table.get_or_insert(rug_fuzz_2, rug_fuzz_3);
            debug_assert_eq!(value.as_str(), Some("initial value"));
            *value = Value::from(rug_fuzz_4);
        }
        debug_assert_eq!(table.get(rug_fuzz_5).unwrap().as_integer(), Some(10));
             }
}
}
}    }
}
#[cfg(test)]
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        let table = inline_table.into_table();
        debug_assert!(! table.is_empty());
        debug_assert!(table.contains_key(rug_fuzz_2));
        debug_assert_eq!(
            table.get(rug_fuzz_3).unwrap().as_value().unwrap().as_str().unwrap(), "value"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_237 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_dotted_false_for_empty_table() {
        let _rug_st_tests_llm_16_237_rrrruuuugggg_test_is_dotted_false_for_empty_table = 0;
        let table = InlineTable::new();
        debug_assert!(! table.is_dotted());
        let _rug_ed_tests_llm_16_237_rrrruuuugggg_test_is_dotted_false_for_empty_table = 0;
    }
    #[test]
    fn test_is_dotted_false_for_standard_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.set_dotted(rug_fuzz_0);
        debug_assert!(! table.is_dotted());
             }
}
}
}    }
    #[test]
    fn test_is_dotted_true_for_dotted_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.set_dotted(rug_fuzz_0);
        debug_assert!(table.is_dotted());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_239 {
    use crate::{Document, InlineTable, Value};
    #[test]
    fn iter_empty_inline_table() {
        let _rug_st_tests_llm_16_239_rrrruuuugggg_iter_empty_inline_table = 0;
        let table = InlineTable::new();
        let mut iter = table.iter();
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_239_rrrruuuugggg_iter_empty_inline_table = 0;
    }
    #[test]
    fn iter_single_pair_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let mut iter = table.iter();
        let (key, value) = iter.next().unwrap();
        debug_assert_eq!(key, "key");
        debug_assert_eq!(value.as_str().unwrap(), "value");
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
    #[test]
    fn iter_multiple_pairs_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        table.insert(rug_fuzz_2, Value::from(rug_fuzz_3));
        let mut iter = table.iter();
        let (key1, value1) = iter.next().unwrap();
        debug_assert_eq!(key1, "first");
        debug_assert_eq!(value1.as_integer().unwrap(), 123);
        let (key2, value2) = iter.next().unwrap();
        debug_assert_eq!(key2, "second");
        debug_assert_eq!(value2.as_integer().unwrap(), 456);
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
    #[test]
    fn iter_dotted_pairs_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        table.insert(rug_fuzz_2, Value::from(rug_fuzz_3));
        table.set_dotted(rug_fuzz_4);
        let mut iter = table.iter();
        let (path, value) = iter.next().unwrap();
        debug_assert_eq!(path, "parent.first");
        debug_assert_eq!(value.as_str().unwrap(), "value1");
        let (path, value) = iter.next().unwrap();
        debug_assert_eq!(path, "parent.second");
        debug_assert_eq!(value.as_str().unwrap(), "value2");
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
    #[test]
    fn iter_nested_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut doc = rug_fuzz_0.parse::<Document>().unwrap();
        let table = doc[rug_fuzz_1].as_inline_table().unwrap();
        let mut iter = table.iter();
        let (key, value) = iter.next().unwrap();
        debug_assert_eq!(key, "name");
        debug_assert_eq!(value.as_str().unwrap(), "Tom Preston-Werner");
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_241_llm_16_241 {
    use crate::{Decor, InlineTable, RawString, Value};
    #[test]
    fn test_key_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(&str, &str, &str, &str, &str, i64, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        debug_assert_eq!(table.key_decor(rug_fuzz_0), None);
        table.decor_mut().set_prefix(RawString::from(rug_fuzz_1));
        table.decor_mut().set_suffix(RawString::from(rug_fuzz_2));
        debug_assert_eq!(table.key_decor(rug_fuzz_3), None);
        table.insert(rug_fuzz_4, Value::from(rug_fuzz_5));
        debug_assert!(table.key_decor(rug_fuzz_6).is_some());
        debug_assert_eq!(
            table.key_decor(rug_fuzz_7).unwrap().prefix().unwrap().as_str(), Some("")
        );
        debug_assert_eq!(
            table.key_decor(rug_fuzz_8).unwrap().suffix().unwrap().as_str(), Some("")
        );
        table
            .key_decor_mut(rug_fuzz_9)
            .unwrap()
            .set_prefix(RawString::from(rug_fuzz_10));
        table
            .key_decor_mut(rug_fuzz_11)
            .unwrap()
            .set_suffix(RawString::from(rug_fuzz_12));
        debug_assert_eq!(
            table.key_decor(rug_fuzz_13).unwrap().prefix().unwrap().as_str(), Some("  ")
        );
        debug_assert_eq!(
            table.key_decor(rug_fuzz_14).unwrap().suffix().unwrap().as_str(), Some("  ")
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_243_llm_16_243 {
    use crate::inline_table::InlineTable;
    use crate::value::Value;
    use crate::internal_string::InternalString;
    #[test]
    fn test_len_empty_table() {
        let _rug_st_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_len_empty_table = 0;
        let table = InlineTable::new();
        debug_assert_eq!(table.len(), 0);
        let _rug_ed_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_len_empty_table = 0;
    }
    #[test]
    fn test_len_with_items() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        table.insert(InternalString::from(rug_fuzz_2), Value::from(rug_fuzz_3));
        debug_assert_eq!(table.len(), 2);
             }
}
}
}    }
    #[test]
    fn test_len_after_removal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, i64, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        table.insert(InternalString::from(rug_fuzz_2), Value::from(rug_fuzz_3));
        table.remove(rug_fuzz_4);
        debug_assert_eq!(table.len(), 1);
             }
}
}
}    }
    #[test]
    fn test_len_after_clear() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        table.insert(InternalString::from(rug_fuzz_2), Value::from(rug_fuzz_3));
        table.clear();
        debug_assert_eq!(table.len(), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_244 {
    use crate::inline_table::InlineTable;
    #[test]
    fn test_new_inline_table() {
        let _rug_st_tests_llm_16_244_rrrruuuugggg_test_new_inline_table = 0;
        let table = InlineTable::new();
        debug_assert!(table.is_empty());
        let _rug_ed_tests_llm_16_244_rrrruuuugggg_test_new_inline_table = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_245 {
    use super::*;
    use crate::*;
    use crate::inline_table::InlineTable;
    use crate::raw_string::RawString;
    #[test]
    fn test_preamble_empty_table() {
        let _rug_st_tests_llm_16_245_rrrruuuugggg_test_preamble_empty_table = 0;
        let table = InlineTable::new();
        let preamble = table.preamble();
        debug_assert_eq!(preamble.as_str(), Some(""));
        let _rug_ed_tests_llm_16_245_rrrruuuugggg_test_preamble_empty_table = 0;
    }
    #[test]
    fn test_preamble_with_raw_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let raw_string: RawString = rug_fuzz_0.into();
        table.set_preamble(raw_string.clone());
        let preamble = table.preamble();
        debug_assert_eq!(preamble, & raw_string);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_246 {
    use crate::inline_table::InlineTable;
    use crate::value::Value;
    use crate::InternalString;
    #[test]
    fn test_remove_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i64, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        table.insert(InternalString::from(rug_fuzz_2), Value::from(rug_fuzz_3));
        debug_assert_eq!(table.len(), 2);
        let removed = table.remove(rug_fuzz_4).unwrap();
        debug_assert_eq!(removed.as_integer().unwrap(), 42);
        debug_assert_eq!(table.len(), 1);
        debug_assert!(table.get(rug_fuzz_5).is_some());
             }
}
}
}    }
    #[test]
    fn test_remove_nonexistent_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        debug_assert_eq!(table.len(), 1);
        let removed = table.remove(rug_fuzz_2);
        debug_assert!(removed.is_none());
        debug_assert_eq!(table.len(), 1);
        debug_assert!(table.get(rug_fuzz_3).is_some());
             }
}
}
}    }
    #[test]
    fn test_remove_key_from_empty_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        let removed = table.remove(rug_fuzz_0);
        debug_assert!(removed.is_none());
        debug_assert_eq!(table.len(), 0);
             }
}
}
}    }
    #[test]
    fn test_remove_only_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        debug_assert_eq!(table.len(), 1);
        let removed = table.remove(rug_fuzz_2).unwrap();
        debug_assert_eq!(removed.as_integer().unwrap(), 42);
        debug_assert_eq!(table.len(), 0);
             }
}
}
}    }
    #[test]
    fn test_remove_and_reinsert_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i64, &str, &str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(InternalString::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        debug_assert_eq!(table.len(), 1);
        let removed = table.remove(rug_fuzz_2).unwrap();
        debug_assert_eq!(removed.as_integer().unwrap(), 42);
        debug_assert_eq!(table.len(), 0);
        table.insert(InternalString::from(rug_fuzz_3), Value::from(rug_fuzz_4));
        debug_assert_eq!(table.len(), 1);
        let value = table.get(rug_fuzz_5).unwrap();
        debug_assert_eq!(value.as_integer().unwrap(), 100);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_247 {
    use crate::{InlineTable, Value};
    #[test]
    fn test_remove_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        table.insert(rug_fuzz_2, Value::from(rug_fuzz_3));
        let removed = table.remove_entry(rug_fuzz_4);
        debug_assert!(removed.is_some());
        let (key, value) = removed.unwrap();
        debug_assert_eq!(key.get(), "key1");
        debug_assert_eq!(value.as_str(), Some("value1"));
        debug_assert!(! table.contains_key(rug_fuzz_5));
        debug_assert!(table.contains_key(rug_fuzz_6));
        debug_assert!(table.remove_entry(rug_fuzz_7).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_248 {
    use super::*;
    use crate::*;
    #[test]
    fn test_set_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        debug_assert_eq!(inline_table.is_dotted(), false);
        inline_table.set_dotted(rug_fuzz_0);
        debug_assert_eq!(inline_table.is_dotted(), true);
        inline_table.set_dotted(rug_fuzz_1);
        debug_assert_eq!(inline_table.is_dotted(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_249 {
    use super::*;
    use crate::*;
    #[test]
    fn test_set_preamble_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.set_preamble(rug_fuzz_0);
        debug_assert_eq!(inline_table.preamble().as_str(), Some(""));
             }
}
}
}    }
    #[test]
    fn test_set_preamble_whitespace() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.set_preamble(rug_fuzz_0);
        debug_assert_eq!(inline_table.preamble().as_str(), Some("  "));
             }
}
}
}    }
    #[test]
    fn test_set_preamble_with_content() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.set_preamble(rug_fuzz_0);
        debug_assert_eq!(inline_table.preamble().as_str(), Some("  # Comment"));
             }
}
}
}    }
    #[test]
    fn test_set_preamble_twice_overwrites() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table.set_preamble(rug_fuzz_0);
        inline_table.set_preamble(rug_fuzz_1);
        debug_assert_eq!(inline_table.preamble().as_str(), Some("Second"));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_253 {
    use crate::{Document, InlineTable};
    use std::ops::Range;
    #[test]
    fn test_span() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = InlineTable::new();
        debug_assert!(table.span().is_none());
        let mut doc = rug_fuzz_0.parse::<Document>().expect(rug_fuzz_1);
        let inline_table_span = doc[rug_fuzz_2]
            .as_inline_table()
            .expect(rug_fuzz_3)
            .span();
        debug_assert!(inline_table_span.is_some());
        let manual_span = Range {
            start: rug_fuzz_4,
            end: rug_fuzz_5,
        };
        let mut table_with_span = InlineTable::new();
        table_with_span.span = Some(manual_span.clone());
        debug_assert_eq!(table_with_span.span(), Some(manual_span));
        table_with_span.despan(rug_fuzz_6);
        debug_assert!(table_with_span.span().is_none());
             }
}
}
}    }
}
