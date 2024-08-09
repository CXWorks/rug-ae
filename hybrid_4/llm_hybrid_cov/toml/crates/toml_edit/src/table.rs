use std::iter::FromIterator;

use indexmap::map::IndexMap;

use crate::key::Key;
use crate::repr::Decor;
use crate::value::DEFAULT_VALUE_DECOR;
use crate::{InlineTable, InternalString, Item, KeyMut, Value};

/// Type representing a TOML non-inline table
#[derive(Clone, Debug, Default)]
pub struct Table {
    // Comments/spaces before and after the header
    pub(crate) decor: Decor,
    // Whether to hide an empty table
    pub(crate) implicit: bool,
    // Whether this is a proxy for dotted keys
    pub(crate) dotted: bool,
    // Used for putting tables back in their original order when serialising.
    //
    // `None` for user created tables (can be overridden with `set_position`)
    doc_position: Option<usize>,
    pub(crate) span: Option<std::ops::Range<usize>>,
    pub(crate) items: KeyValuePairs,
}

/// Constructors
///
/// See also `FromIterator`
impl Table {
    /// Creates an empty table.
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn with_pos(doc_position: Option<usize>) -> Self {
        Self {
            doc_position,
            ..Default::default()
        }
    }

    pub(crate) fn with_pairs(items: KeyValuePairs) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    /// Convert to an inline table
    pub fn into_inline_table(mut self) -> InlineTable {
        for (_, kv) in self.items.iter_mut() {
            kv.value.make_value();
        }
        let mut t = InlineTable::with_pairs(self.items);
        t.fmt();
        t
    }
}

/// Formatting
impl Table {
    /// Get key/values for values that are visually children of this table
    ///
    /// For example, this will return dotted keys
    pub fn get_values(&self) -> Vec<(Vec<&Key>, &Value)> {
        let mut values = Vec::new();
        let root = Vec::new();
        self.append_values(&root, &mut values);
        values
    }

    fn append_values<'s, 'c>(
        &'s self,
        parent: &[&'s Key],
        values: &'c mut Vec<(Vec<&'s Key>, &'s Value)>,
    ) {
        for value in self.items.values() {
            let mut path = parent.to_vec();
            path.push(&value.key);
            match &value.value {
                Item::Table(table) if table.is_dotted() => {
                    table.append_values(&path, values);
                }
                Item::Value(value) => {
                    if let Some(table) = value.as_inline_table() {
                        if table.is_dotted() {
                            table.append_values(&path, values);
                        } else {
                            values.push((path, value));
                        }
                    } else {
                        values.push((path, value));
                    }
                }
                _ => {}
            }
        }
    }

    /// Auto formats the table.
    pub fn fmt(&mut self) {
        decorate_table(self);
    }

    /// Sorts Key/Value Pairs of the table.
    ///
    /// Doesn't affect subtables or subarrays.
    pub fn sort_values(&mut self) {
        // Assuming standard tables have their doc_position set and this won't negatively impact them
        self.items.sort_keys();
        for kv in self.items.values_mut() {
            match &mut kv.value {
                Item::Table(table) if table.is_dotted() => {
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
        F: FnMut(&Key, &Item, &Key, &Item) -> std::cmp::Ordering,
    {
        self.sort_values_by_internal(&mut compare);
    }

    fn sort_values_by_internal<F>(&mut self, compare: &mut F)
    where
        F: FnMut(&Key, &Item, &Key, &Item) -> std::cmp::Ordering,
    {
        let modified_cmp = |_: &InternalString,
                            val1: &TableKeyValue,
                            _: &InternalString,
                            val2: &TableKeyValue|
         -> std::cmp::Ordering {
            compare(&val1.key, &val1.value, &val2.key, &val2.value)
        };

        self.items.sort_by(modified_cmp);

        for kv in self.items.values_mut() {
            match &mut kv.value {
                Item::Table(table) if table.is_dotted() => {
                    table.sort_values_by_internal(compare);
                }
                _ => {}
            }
        }
    }

    /// If a table has no key/value pairs and implicit, it will not be displayed.
    ///
    /// # Examples
    ///
    /// ```notrust
    /// [target."x86_64/windows.json".dependencies]
    /// ```
    ///
    /// In the document above, tables `target` and `target."x86_64/windows.json"` are implicit.
    ///
    /// ```
    /// use toml_edit::Document;
    /// let mut doc = "[a]\n[a.b]\n".parse::<Document>().expect("invalid toml");
    ///
    /// doc["a"].as_table_mut().unwrap().set_implicit(true);
    /// assert_eq!(doc.to_string(), "[a.b]\n");
    /// ```
    pub fn set_implicit(&mut self, implicit: bool) {
        self.implicit = implicit;
    }

    /// If a table has no key/value pairs and implicit, it will not be displayed.
    pub fn is_implicit(&self) -> bool {
        self.implicit
    }

    /// Change this table's dotted status
    pub fn set_dotted(&mut self, yes: bool) {
        self.dotted = yes;
    }

    /// Check if this is a wrapper for dotted keys, rather than a standard table
    pub fn is_dotted(&self) -> bool {
        self.dotted
    }

    /// Sets the position of the `Table` within the `Document`.
    pub fn set_position(&mut self, doc_position: usize) {
        self.doc_position = Some(doc_position);
    }

    /// The position of the `Table` within the `Document`.
    ///
    /// Returns `None` if the `Table` was created manually (i.e. not via parsing)
    /// in which case its position is set automatically.  This can be overridden with
    /// [`Table::set_position`].
    pub fn position(&self) -> Option<usize> {
        self.doc_position
    }

    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    /// Returns the decor associated with a given key of the table.
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

    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }

    pub(crate) fn despan(&mut self, input: &str) {
        self.span = None;
        self.decor.despan(input);
        for kv in self.items.values_mut() {
            kv.key.despan(input);
            kv.value.despan(input);
        }
    }
}

impl Table {
    /// Returns an iterator over all key/value pairs, including empty.
    pub fn iter(&self) -> Iter<'_> {
        Box::new(
            self.items
                .iter()
                .filter(|(_, kv)| !kv.value.is_none())
                .map(|(key, kv)| (&key[..], &kv.value)),
        )
    }

    /// Returns an mutable iterator over all key/value pairs, including empty.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        Box::new(
            self.items
                .iter_mut()
                .filter(|(_, kv)| !kv.value.is_none())
                .map(|(_, kv)| (kv.key.as_mut(), &mut kv.value)),
        )
    }

    /// Returns the number of non-empty items in the table.
    pub fn len(&self) -> usize {
        self.items.iter().filter(|i| !(i.1).value.is_none()).count()
    }

    /// Returns true if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the table, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.items.clear()
    }

    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    pub fn entry<'a>(&'a mut self, key: &str) -> Entry<'a> {
        // Accept a `&str` rather than an owned type to keep `InternalString`, well, internal
        match self.items.entry(key.into()) {
            indexmap::map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry { entry }),
            indexmap::map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry { entry, key: None }),
        }
    }

    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    pub fn entry_format<'a>(&'a mut self, key: &Key) -> Entry<'a> {
        // Accept a `&Key` to be consistent with `entry`
        match self.items.entry(key.get().into()) {
            indexmap::map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry { entry }),
            indexmap::map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                entry,
                key: Some(key.to_owned()),
            }),
        }
    }

    /// Returns an optional reference to an item given the key.
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Item> {
        self.items.get(key).and_then(|kv| {
            if !kv.value.is_none() {
                Some(&kv.value)
            } else {
                None
            }
        })
    }

    /// Returns an optional mutable reference to an item given the key.
    pub fn get_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut Item> {
        self.items.get_mut(key).and_then(|kv| {
            if !kv.value.is_none() {
                Some(&mut kv.value)
            } else {
                None
            }
        })
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

    /// Returns true if the table contains an item with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            !kv.value.is_none()
        } else {
            false
        }
    }

    /// Returns true if the table contains a table with the given key.
    pub fn contains_table(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_table()
        } else {
            false
        }
    }

    /// Returns true if the table contains a value with the given key.
    pub fn contains_value(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_value()
        } else {
            false
        }
    }

    /// Returns true if the table contains an array of tables with the given key.
    pub fn contains_array_of_tables(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) {
            kv.value.is_array_of_tables()
        } else {
            false
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: &str, item: Item) -> Option<Item> {
        let kv = TableKeyValue::new(Key::new(key), item);
        self.items.insert(key.into(), kv).map(|kv| kv.value)
    }

    /// Inserts a key-value pair into the map.
    pub fn insert_formatted(&mut self, key: &Key, item: Item) -> Option<Item> {
        let kv = TableKeyValue::new(key.to_owned(), item);
        self.items.insert(key.get().into(), kv).map(|kv| kv.value)
    }

    /// Removes an item given the key.
    pub fn remove(&mut self, key: &str) -> Option<Item> {
        self.items.shift_remove(key).map(|kv| kv.value)
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    pub fn remove_entry(&mut self, key: &str) -> Option<(Key, Item)> {
        self.items.shift_remove(key).map(|kv| (kv.key, kv.value))
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::encode::Encode;
        let children = self.get_values();
        // print table body
        for (key_path, value) in children {
            key_path.as_slice().encode(f, None, DEFAULT_KEY_DECOR)?;
            write!(f, "=")?;
            value.encode(f, None, DEFAULT_VALUE_DECOR)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<K: Into<Key>, V: Into<Value>> Extend<(K, V)> for Table {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            let key = key.into();
            let value = Item::Value(value.into());
            let value = TableKeyValue::new(key, value);
            self.items.insert(value.key.get().into(), value);
        }
    }
}

impl<K: Into<Key>, V: Into<Value>> FromIterator<(K, V)> for Table {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut table = Table::new();
        table.extend(iter);
        table
    }
}

impl IntoIterator for Table {
    type Item = (InternalString, Item);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.items.into_iter().map(|(k, kv)| (k, kv.value)))
    }
}

impl<'s> IntoIterator for &'s Table {
    type Item = (&'s str, &'s Item);
    type IntoIter = Iter<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub(crate) type KeyValuePairs = IndexMap<InternalString, TableKeyValue>;

fn decorate_table(table: &mut Table) {
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

// `key1 = value1`
pub(crate) const DEFAULT_KEY_DECOR: (&str, &str) = ("", " ");
pub(crate) const DEFAULT_TABLE_DECOR: (&str, &str) = ("\n", "");
pub(crate) const DEFAULT_KEY_PATH_DECOR: (&str, &str) = ("", "");

#[derive(Debug, Clone)]
pub(crate) struct TableKeyValue {
    pub(crate) key: Key,
    pub(crate) value: Item,
}

impl TableKeyValue {
    pub(crate) fn new(key: Key, value: Item) -> Self {
        TableKeyValue { key, value }
    }
}

/// An owned iterator type over `Table`'s key/value pairs.
pub type IntoIter = Box<dyn Iterator<Item = (InternalString, Item)>>;
/// An iterator type over `Table`'s key/value pairs.
pub type Iter<'a> = Box<dyn Iterator<Item = (&'a str, &'a Item)> + 'a>;
/// A mutable iterator type over `Table`'s key/value pairs.
pub type IterMut<'a> = Box<dyn Iterator<Item = (KeyMut<'a>, &'a mut Item)> + 'a>;

/// This trait represents either a `Table`, or an `InlineTable`.
pub trait TableLike: crate::private::Sealed {
    /// Returns an iterator over key/value pairs.
    fn iter(&self) -> Iter<'_>;
    /// Returns an mutable iterator over all key/value pairs, including empty.
    fn iter_mut(&mut self) -> IterMut<'_>;
    /// Returns the number of nonempty items.
    fn len(&self) -> usize {
        self.iter().filter(|&(_, v)| !v.is_none()).count()
    }
    /// Returns true if the table is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Clears the table, removing all key-value pairs. Keeps the allocated memory for reuse.
    fn clear(&mut self);
    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    fn entry<'a>(&'a mut self, key: &str) -> Entry<'a>;
    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    fn entry_format<'a>(&'a mut self, key: &Key) -> Entry<'a>;
    /// Returns an optional reference to an item given the key.
    fn get<'s>(&'s self, key: &str) -> Option<&'s Item>;
    /// Returns an optional mutable reference to an item given the key.
    fn get_mut<'s>(&'s mut self, key: &str) -> Option<&'s mut Item>;
    /// Return references to the key-value pair stored for key, if it is present, else None.
    fn get_key_value<'a>(&'a self, key: &str) -> Option<(&'a Key, &'a Item)>;
    /// Return mutable references to the key-value pair stored for key, if it is present, else None.
    fn get_key_value_mut<'a>(&'a mut self, key: &str) -> Option<(KeyMut<'a>, &'a mut Item)>;
    /// Returns true if the table contains an item with the given key.
    fn contains_key(&self, key: &str) -> bool;
    /// Inserts a key-value pair into the map.
    fn insert(&mut self, key: &str, value: Item) -> Option<Item>;
    /// Removes an item given the key.
    fn remove(&mut self, key: &str) -> Option<Item>;

    /// Get key/values for values that are visually children of this table
    ///
    /// For example, this will return dotted keys
    fn get_values(&self) -> Vec<(Vec<&Key>, &Value)>;

    /// Auto formats the table.
    fn fmt(&mut self);
    /// Sorts Key/Value Pairs of the table.
    ///
    /// Doesn't affect subtables or subarrays.
    fn sort_values(&mut self);
    /// Change this table's dotted status
    fn set_dotted(&mut self, yes: bool);
    /// Check if this is a wrapper for dotted keys, rather than a standard table
    fn is_dotted(&self) -> bool;

    /// Returns the decor associated with a given key of the table.
    fn key_decor_mut(&mut self, key: &str) -> Option<&mut Decor>;
    /// Returns the decor associated with a given key of the table.
    fn key_decor(&self, key: &str) -> Option<&Decor>;
}

impl TableLike for Table {
    fn iter(&self) -> Iter<'_> {
        self.iter()
    }
    fn iter_mut(&mut self) -> IterMut<'_> {
        self.iter_mut()
    }
    fn clear(&mut self) {
        self.clear();
    }
    fn entry<'a>(&'a mut self, key: &str) -> Entry<'a> {
        self.entry(key)
    }
    fn entry_format<'a>(&'a mut self, key: &Key) -> Entry<'a> {
        self.entry_format(key)
    }
    fn get<'s>(&'s self, key: &str) -> Option<&'s Item> {
        self.get(key)
    }
    fn get_mut<'s>(&'s mut self, key: &str) -> Option<&'s mut Item> {
        self.get_mut(key)
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
        self.insert(key, value)
    }
    fn remove(&mut self, key: &str) -> Option<Item> {
        self.remove(key)
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
    fn is_dotted(&self) -> bool {
        self.is_dotted()
    }
    fn set_dotted(&mut self, yes: bool) {
        self.set_dotted(yes)
    }

    fn key_decor_mut(&mut self, key: &str) -> Option<&mut Decor> {
        self.key_decor_mut(key)
    }
    fn key_decor(&self, key: &str) -> Option<&Decor> {
        self.key_decor(key)
    }
}

/// A view into a single location in a map, which may be vacant or occupied.
pub enum Entry<'a> {
    /// An occupied Entry.
    Occupied(OccupiedEntry<'a>),
    /// A vacant Entry.
    Vacant(VacantEntry<'a>),
}

impl<'a> Entry<'a> {
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
            Entry::Occupied(e) => e.key(),
            Entry::Vacant(e) => e.key(),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: Item) -> &'a mut Item {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> Item>(self, default: F) -> &'a mut Item {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }
}

/// A view into a single occupied location in a `IndexMap`.
pub struct OccupiedEntry<'a> {
    pub(crate) entry: indexmap::map::OccupiedEntry<'a, InternalString, TableKeyValue>,
}

impl<'a> OccupiedEntry<'a> {
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
    pub fn get(&self) -> &Item {
        &self.entry.get().value
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut Item {
        &mut self.entry.get_mut().value
    }

    /// Converts the OccupiedEntry into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself
    pub fn into_mut(self) -> &'a mut Item {
        &mut self.entry.into_mut().value
    }

    /// Sets the value of the entry, and returns the entry's old value
    pub fn insert(&mut self, mut value: Item) -> Item {
        std::mem::swap(&mut value, &mut self.entry.get_mut().value);
        value
    }

    /// Takes the value out of the entry, and returns it
    pub fn remove(self) -> Item {
        self.entry.shift_remove().value
    }
}

/// A view into a single empty location in a `IndexMap`.
pub struct VacantEntry<'a> {
    pub(crate) entry: indexmap::map::VacantEntry<'a, InternalString, TableKeyValue>,
    pub(crate) key: Option<Key>,
}

impl<'a> VacantEntry<'a> {
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
    pub fn insert(self, value: Item) -> &'a mut Item {
        let entry = self.entry;
        let key = self.key.unwrap_or_else(|| Key::new(entry.key().as_str()));
        &mut entry.insert(TableKeyValue::new(key, value)).value
    }
}
#[cfg(test)]
mod tests_llm_16_100 {
    use crate::table::Table;

    #[test]
    fn test_clear_table() {
        let mut table = Table::new();
        table
            .entry("key1")
            .or_insert_with(|| "value1".parse().unwrap());
        table
            .entry("key2")
            .or_insert_with(|| "value2".parse().unwrap());
        assert!(!table.is_empty());
        table.clear();
        assert!(table.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_101 {
    use crate::table::Table;

    #[test]
    fn table_contains_key() {
        let mut table = Table::new();
        assert!(!table.contains_key("key1"), "Table should not contain 'key1'");
        table.insert("key1", "value1".parse().unwrap());
        assert!(table.contains_key("key1"), "Table should contain 'key1'");
    }
}#[cfg(test)]
mod tests_llm_16_103 {
    use crate::{Document, Item, Table, TableLike, Value};

    #[test]
    fn entry_format_test() {
        let mut table = Table::new();
        let key = "test";
        let entry = table.entry_format(&key.parse().unwrap());
        assert_eq!(entry.key(), key);

        let value = Value::from(42);
        table.insert_formatted(&key.parse().unwrap(), Item::Value(value.clone()));
        let entry = table.entry_format(&key.parse().unwrap());
        match entry {
            crate::Entry::Occupied(occupied) => {
                assert_eq!(occupied.get().as_integer(), Some(42));
            }
            crate::Entry::Vacant(_) => panic!("Expected an occupied entry"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_105 {
    use crate::{Table, Item, Value};

    #[test]
    fn get_key_value_existing() {
        let mut table = Table::new();
        table["key"] = Item::Value(Value::from("value"));
        let (key, item) = table.get_key_value("key").unwrap();
        assert_eq!(key.get(), "key");
        assert_eq!(item.as_value().unwrap().as_str().unwrap(), "value");
    }

    #[test]
    fn get_key_value_non_existing() {
        let table = Table::new();
        assert!(table.get_key_value("key").is_none());
    }

    #[test]
    fn get_key_value_empty_item() {
        let mut table = Table::new();
        table["key"] = Item::None;
        assert!(table.get_key_value("key").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_106 {
    use super::*;

use crate::*;
    use crate::Item;

    #[test]
    fn test_get_key_value_mut() {
        let mut table = Table::new();
        table.insert("key1", Item::Value("value1".parse().unwrap()));
        table.insert("key2", Item::Value("value2".parse().unwrap()));

        {
            let (_keymut, value_mut) = table.get_key_value_mut("key1").unwrap();
            if let Item::Value(value) = value_mut {
                *value = "updated_value1".parse().unwrap();
            }
        }

        if let Item::Value(value) = table.get("key1").unwrap() {
            assert_eq!(value.as_str(), Some("updated_value1"));
        } else {
            panic!("Value not found for 'key1'");
        }

        assert!(table.get_key_value_mut("key3").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_107_llm_16_107 {
    use crate::{Table, Item, Value, TableLike};

    #[test]
    fn get_mut_existing_key() {
        let mut table = Table::new();
        let key = "key1";
        table.insert(key, Item::Value(Value::from("value1")));
        if let Some(item) = table.get_mut(key) {
            if let Item::Value(value) = item {
                if let Some(value_str) = value.as_str() {
                    let mut new_value = Value::from(format!("{}_modified", value_str));
                    *value = new_value;
                }
            }
        }

        let expected_value = "value1_modified";
        assert_eq!(
            table.get(key).unwrap().as_value().unwrap().as_str().unwrap(),
            expected_value
        );
    }

    #[test]
    fn get_mut_non_existing_key() {
        let mut table = Table::new();
        assert!(table.get_mut("non_existing_key").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_108 {
    use crate::table::Table;
    use crate::value::Value;
    use crate::key::Key;
    use crate::item::Item;
    
    #[test]
    fn get_values_empty_table() {
        let table = Table::new();
        assert!(table.get_values().is_empty());
    }
    
    #[test]
    fn get_values_with_single_pair() {
        let mut table = Table::new();
        let key = Key::new("key");
        let value = Value::from("value");
        table.insert(key.get(), Item::Value(value));
        let values = table.get_values();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].0[0].get(), "key");
        assert_eq!(values[0].1.as_str().unwrap(), "value");
    }
    
    #[test]
    fn get_values_nested_table() {
        let mut table = Table::new();
        let key = Key::new("parent");
        let mut subtable = Table::new();
        let subkey = Key::new("child");
        let value = Value::from(42);
        subtable.insert(subkey.get(), Item::Value(value));
        table.insert(key.get(), Item::Table(subtable));
        
        let values = table.get_values();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].0[0].get(), "parent");
        assert_eq!(values[0].0[1].get(), "child");
        assert_eq!(values[0].1.as_integer().unwrap(), 42);
    }
    
    #[test]
    fn get_values_with_dotted_keys() {
        let mut table = Table::new();
        let key = Key::new("parent.child");
        let value = Value::from("value");
        table.insert_formatted(&key, Item::Value(value));
        table.set_dotted(true);
        
        let values = table.get_values();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].0[0].get(), "parent");
        assert_eq!(values[0].0[1].get(), "child");
        assert_eq!(values[0].1.as_str().unwrap(), "value");
    }
}#[cfg(test)]
mod tests_llm_16_110 {
    use super::*;

use crate::*;
    use crate::table::Table;
    use crate::repr::Decor;

    #[test]
    fn test_is_not_dotted() {
        let mut table = Table::new();
        table.set_dotted(false);
        assert_eq!(table.is_dotted(), false);
    }

    #[test]
    fn test_is_dotted() {
        let mut table = Table::new();
        table.set_dotted(true);
        assert_eq!(table.is_dotted(), true);
    }
}#[cfg(test)]
mod tests_llm_16_112_llm_16_112 {
    use crate::{table::Table, Item, Value, table::KeyMut};

    #[test]
    fn test_iter_mut() {
        let mut table = Table::new();
        table.insert("key1", Item::Value(Value::from("value1")));
        table.insert("key2", Item::Value(Value::from("value2")));
        let mut count = 0;

        {
            let mut iter_mut = table.iter_mut();
            while let Some((key, value)) = iter_mut.next() {
                count += 1;
                if key.get() == "key1" {
                    *value = Item::Value(Value::from("changed"));
                }
            }
        }
        
        assert_eq!(count, 2);
        assert_eq!(table.get("key1").and_then(Item::as_str), Some("changed"));
        assert_eq!(table.get("key2").and_then(Item::as_str), Some("value2"));
    }
}#[cfg(test)]
mod tests_llm_16_114 {
    use super::*;

use crate::*;
    use crate::repr::Decor;

    #[test]
    fn key_decor_mut_existing_key() {
        let mut table = Table::new();
        table.insert("test_key", Item::Value(Value::from("test_value")));
        let decor = table.key_decor_mut("test_key").unwrap();
        decor.set_prefix("prefix_");
        decor.set_suffix("_suffix");
        let modified_decor = table.key_decor("test_key").unwrap();
        assert_eq!(modified_decor.prefix(), Some(&"prefix_".into()));
        assert_eq!(modified_decor.suffix(), Some(&"_suffix".into()));
    }

    #[test]
    fn key_decor_mut_non_existing_key() {
        let mut table = Table::new();
        assert!(table.key_decor_mut("non_existing_key").is_none());
    }
}#[cfg(test)]
mod tests_llm_16_115 {
    use crate::{Item, Table};

    #[test]
    fn remove_existing_key() {
        let mut table = Table::new();
        table.insert("key1", Item::Value("value1".into()));
        table.insert("key2", Item::Value("value2".into()));
        
        let removed = table.remove("key1");
        assert_eq!(removed.is_some(), true);
        assert_eq!(table.contains_key("key1"), false);
    }

    #[test]
    fn remove_non_existing_key() {
        let mut table = Table::new();
        table.insert("key1", Item::Value("value1".into()));

        let removed = table.remove("key2");
        assert_eq!(removed.is_none(), true);
        assert_eq!(table.contains_key("key2"), false);
    }

    #[test]
    fn remove_key_from_empty_table() {
        let mut table = Table::new();
        
        let removed = table.remove("key1");
        assert_eq!(removed.is_none(), true);
        assert_eq!(table.contains_key("key1"), false);
    }
}#[cfg(test)]
mod tests_llm_16_116 {
    use crate::{Table, Item, table::TableLike};

    #[test]
    fn test_set_dotted() {
        let mut table = Table::new();
        // Table should start with dotted being false
        assert!(!table.is_dotted());
        
        // Set dotted to true
        table.set_dotted(true);
        assert!(table.is_dotted());
        
        // Set dotted to false
        table.set_dotted(false);
        assert!(!table.is_dotted());
    }
}#[cfg(test)]
mod tests_llm_16_464_llm_16_464 {
    use crate::{Entry, Item, InternalString, Key};
    use crate::table::TableKeyValue;

    #[test]
    fn test_entry_key_occupied() {
        let mut table = crate::Table::new();
        table["test_key"] = Item::Value("test_value".parse().unwrap());
        if let Entry::Occupied(oe) = table.entry("test_key") {
            assert_eq!(oe.key(), "test_key");
        } else {
            panic!("Expected entry to be occupied");
        }
    }

    #[test]
    fn test_entry_key_vacant() {
        let mut table = crate::Table::new();
        if let Entry::Vacant(ve) = table.entry("test_key") {
            assert_eq!(ve.key(), "test_key");
        } else {
            panic!("Expected entry to be vacant");
        }
    }
}#[cfg(test)]
mod tests_llm_16_465 {
    use crate::table::{Table, Item, Entry};
    use crate::Value;

    #[test]
    fn test_or_insert_with_existing_value() {
        let mut table = Table::new();
        table["key"] = Item::Value(Value::from(42));
        assert_eq!(table.entry("key").or_insert(Item::Value(Value::from(99)))
                   .as_value().unwrap().as_integer(), Some(42));
    }

    #[test]
    fn test_or_insert_with_vacant_entry() {
        let mut table = Table::new();
        assert!(table.get("key").is_none());
        assert_eq!(table.entry("key").or_insert(Item::Value(Value::from(99)))
                   .as_value().unwrap().as_integer(), Some(99));
        assert_eq!(table.get("key").unwrap().as_value().unwrap().as_integer(), Some(99));
    }
}#[cfg(test)]
mod tests_llm_16_466 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Formatted;
    use crate::value::Value;

    #[test]
    fn test_or_insert_with_occupied() {
        let mut table = Table::new();
        let key = "existing_key";
        table.insert(key, Item::Value(Value::Integer(Formatted::new(42))));

        let entry = table.entry(key);
        let value = entry.or_insert_with(|| Item::Value(Value::Integer(Formatted::new(100))));

        match value {
            Item::Value(v) => match v {
                Value::Integer(i) => assert_eq!(*i.value(), 42),
                _ => panic!("Not an integer"),
            },
            _ => panic!("Not a value"),
        }
    }

    #[test]
    fn test_or_insert_with_vacant() {
        let mut table = Table::new();
        let key = "non_existing_key";

        let entry = table.entry(key);
        let value = entry.or_insert_with(|| Item::Value(Value::Integer(Formatted::new(100))));

        match value {
            Item::Value(v) => match v {
                Value::Integer(i) => assert_eq!(*i.value(), 100),
                _ => panic!("Not an integer"),
            },
            _ => panic!("Not a value"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_467_llm_16_467 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Value;

    #[test]
    fn test_get() {
        let mut table = Table::new();
        let value = Value::from(42);
        table.insert("key", Item::Value(value));

        let entry = table.entry("key");
        if let Entry::Occupied(occupied_entry) = entry {
            let item = occupied_entry.get();
            assert!(item.is_value()); 
            assert_eq!(item.as_value().unwrap().as_integer().unwrap(), 42);
        } else {
            panic!("Expected entry to be occupied");
        }
    }
}#[cfg(test)]
mod tests_llm_16_469 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Value;
    use crate::Formatted;
    use crate::table::Table;
    use crate::table::Entry;

    #[test]
    fn test_occupied_entry_insert() {
        let mut table = Table::new();
        table["key"] = Item::Value(Value::Integer(Formatted::new(42)));
        let mut entry = table.entry("key");
        if let Entry::Occupied(mut occupied) = entry {
            let old_value = occupied.insert(Item::Value(Value::String(Formatted::new("new".to_string()))));
            if let Item::Value(Value::Integer(old_int_value)) = old_value {
                assert_eq!(*old_int_value.value(), 42);
            } else {
                panic!("Old value was not an integer");
            }
            if let Item::Value(Value::String(new_value)) = occupied.get() {
                assert_eq!(new_value.value(), "new");
            } else {
                panic!("New value was not inserted");
            }
        } else {
            panic!("Entry expected to be occupied");
        }
    }
}#[cfg(test)]
mod tests_llm_16_470_llm_16_470 {
    use crate::{Document, Item, Value, table::Table};

    #[test]
    fn test_table_occupied_entry_into_mut() {
        let toml_content = r#"
        [package]
        name = "your_package"
        "#;

        let mut doc = toml_content.parse::<Document>().expect("Parsing failed");
        let package_entry = doc.as_table_mut().entry("package");
        
        if let crate::table::Entry::Occupied(mut entry) = package_entry {
            let package_table = entry.get_mut().as_table_mut().expect("Not a table");
            package_table["name"] = Item::Value(Value::from("my_package"));
            assert_eq!(package_table["name"].as_str(), Some("my_package"));
        } else {
            panic!("package entry is not occupied");
        }
    }
}#[cfg(test)]
mod tests_llm_16_473 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Value;
    use crate::repr::Formatted;
    use crate::key::Key;

    /// Helper function to create a Formatted<String> value with default decor.
    fn formatted_string(value: &str) -> Formatted<String> {
        Formatted::new(value.to_owned())
    }

    /// Helper function to retrieve an OccupiedEntry for testing.
    /// Takes a key and a table and returns an OccupiedEntry.
    /// 
    /// # Panics
    /// Panics if the key does not exist.
    fn get_occupied_entry<'a>(table: &'a mut Table, key: &str) -> table::OccupiedEntry<'a> {
        if let crate::table::Entry::Occupied(entry) = table.entry(key) {
            entry
        } else {
            panic!("Expected key to be occupied.")
        }
    }

    #[test]
    fn test_remove_entry() {
        let mut table = Table::new();
        
        // Insert key-value pairs
        table.insert("key1", Item::Value(Value::String(formatted_string("value1"))));
        table.insert("key2", Item::Value(Value::String(formatted_string("value2"))));
        table.insert("key3", Item::Value(Value::String(formatted_string("value3"))));
        
        // Get an occupied entry for key2
        let occupied_entry = get_occupied_entry(&mut table, "key2");
        
        // Remove the entry
        let removed_item = occupied_entry.remove();
        
        // Verify the item was removed
        assert!(matches!(removed_item, Item::Value(Value::String(v)) if v.value() == "value2"));
        assert!(table.get("key2").is_none());
        assert_eq!(table.len(), 2);
    }
}#[cfg(test)]
mod tests_llm_16_475 {
    use crate::table::Table;
    use crate::Item;
    use std::str::FromStr;

    #[test]
    fn test_table_clear_empty() {
        let mut table = Table::new();
        table.clear();
        assert!(table.is_empty());
    }

    #[test]
    fn test_table_clear_with_entries() {
        let mut table = Table::new();
        table.insert("key1", Item::Value("value1".parse().unwrap()));
        table.insert("key2", Item::Value("value2".parse().unwrap()));
        assert_eq!(table.len(), 2);
        table.clear();
        assert!(table.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_476_llm_16_476 {
    use super::*;

use crate::*;

    #[test]
    fn test_contains_array_of_tables() {
        let mut table = Table::new();

        // Test empty table
        assert!(!table.contains_array_of_tables("key"));

        // Insert array of tables and test
        table.insert("key", Item::ArrayOfTables(crate::ArrayOfTables::new()));
        assert!(table.contains_array_of_tables("key"));

        // Insert value that is not array of tables and test
        table.insert("key", Item::Value(crate::Value::from("value")));
        assert!(!table.contains_array_of_tables("key"));
    }
}#[cfg(test)]
mod tests_llm_16_477_llm_16_477 {
    use crate as toml_edit;
    use crate::Item;
    use crate::Value;
    use crate::Table;

    #[test]
    fn test_contains_key_present() {
        let mut table = Table::new();
        table.insert("key", Item::Value(Value::from("value")));
        assert!(table.contains_key("key"));
    }

    #[test]
    fn test_contains_key_absent() {
        let table = Table::new();
        assert!(!table.contains_key("key"));
    }

    #[test]
    fn test_contains_key_empty_value() {
        let mut table = Table::new();
        table.insert("key", Item::None);
        assert!(!table.contains_key("key"));
    }
}#[cfg(test)]
mod tests_llm_16_478 {
    use crate::table::Table;

    #[test]
    fn test_contains_table_with_existing_table() {
        let mut table = Table::new();
        table.insert("child_table", crate::Item::Table(Table::new()));
        assert!(table.contains_table("child_table"));
    }

    #[test]
    fn test_contains_table_without_table() {
        let table = Table::new();
        assert!(!table.contains_table("nonexistent"));
    }

    #[test]
    fn test_contains_table_with_non_table() {
        let mut table = Table::new();
        table.insert("key", crate::Item::Value(crate::Value::from("value")));
        assert!(!table.contains_table("key"));
    }
}#[cfg(test)]
mod tests_llm_16_479 {
    use crate::{Table, Item, Value};

    #[test]
    fn test_contains_value_with_existing_value() {
        let mut table = Table::new();
        table.insert("key", Item::Value(Value::from("value")));
        assert!(table.contains_value("key"));
    }

    #[test]
    fn test_contains_value_with_non_existing_value() {
        let table = Table::new();
        assert!(!table.contains_value("key"));
    }

    #[test]
    fn test_contains_value_with_existing_non_value() {
        let mut table = Table::new();
        table.insert("key", Item::Table(Table::new()));
        assert!(!table.contains_value("key"));
    }
}#[cfg(test)]
mod tests_llm_16_484 {
    use super::*;

use crate::*;
    use crate::table::Table;
    use crate::key::Key;
    use crate::key::KeyMut;
    use crate::item::Item;

    #[test]
    fn test_entry_format_occupied() {
        let mut table = Table::new();
        let key = "key1";
        table.insert(key, Item::Value(1.into()));
        let key = Key::new(key);
        if let Entry::Occupied(occupied) = table.entry_format(&key) {
            let key = occupied.key();
            assert_eq!(key, "key1");
            let value = occupied.get();
            assert_eq!(value.as_integer(), Some(1));
        } else {
            panic!("Expected occupied entry");
        }
    }

    #[test]
    fn test_entry_format_vacant() {
        let mut table = Table::new();
        let key = Key::new("key2");
        if let Entry::Vacant(vacant) = table.entry_format(&key) {
            let key = vacant.key();
            assert_eq!(key, "key2");
        } else {
            panic!("Expected vacant entry");
        }
    }
}#[cfg(test)]
mod tests_llm_16_488 {
    use crate::table::Table;
    use crate::Item;
    use std::str::FromStr;

    #[test]
    fn test_get_key_value_mut() {
        let mut table = Table::new();
        let key = "test_key";
        let value = "test_value";
        table.insert(key, Item::Value(value.into()));

        // Check that get_key_value_mut returns a mutable reference to the correct key-value pair
        let (key_mut, value_mut) = table.get_key_value_mut(key).unwrap();
        let key_str = key_mut.get();
        let val_str = value_mut.as_value().unwrap().as_str().unwrap();
        assert_eq!(key_str, key);
        assert_eq!(val_str, value);

        // Modify the value
        *value_mut = Item::Value(FromStr::from_str("modified_value").unwrap());
        let modified_value = table.get(key).unwrap().as_value().unwrap().as_str().unwrap();
        assert_eq!(modified_value, "modified_value");

        // Check that get_key_value_mut returns None when the key does not exist
        assert!(table.get_key_value_mut("non_existent_key").is_none());

        // Check that get_key_value_mut returns None when the value is empty (Item::None)
        table.insert(key, Item::None);
        assert!(table.get_key_value_mut(key).is_none());

        // Check that get_key_value_mut returns None when the value is a table itself
        let mut sub_table = Table::new();
        let nested_key = "nested";
        sub_table.insert(nested_key, Item::Value("nested_value".into()));
        table.insert(key, Item::Table(sub_table));
        assert!(table.get_key_value_mut(key).is_none());

        // Cleanup for subsequent tests
        table.clear();
        assert!(table.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_494 {
    use super::*;

use crate::*;

    #[test]
    fn test_is_dotted_true() {
        let mut table = Table::new();
        table.set_dotted(true);
        assert!(table.is_dotted());
    }

    #[test]
    fn test_is_dotted_false() {
        let table = Table::new();
        assert!(!table.is_dotted());
    }
}#[cfg(test)]
mod tests_llm_16_495 {
    use super::*;

use crate::*;
    use crate::Item;

    #[test]
    fn test_table_is_empty_with_empty_table() {
        let table = Table::new();
        assert!(table.is_empty());
    }

    #[test]
    fn test_table_is_empty_with_non_empty_table() {
        let mut table = Table::new();
        table.insert("key", Item::Value("value".parse().unwrap()));
        assert!(!table.is_empty());
    }

    #[test]
    fn test_table_is_empty_after_clear() {
        let mut table = Table::new();
        table.insert("key", Item::Value("value".parse().unwrap()));
        table.clear();
        assert!(table.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_496 {
    use crate::{Table, Item};

    #[test]
    fn test_implicit_table() {
        // An explicit table should not be implicit
        let mut table = Table::new();
        table.set_implicit(false);
        assert!(!table.is_implicit());

        // An implicit table should be implicit
        let mut implicit_table = Table::new();
        implicit_table.set_implicit(true);
        assert!(implicit_table.is_implicit());

        // Ensure that implicit status doesn't change with adding/removing elements
        implicit_table.insert("key", Item::Value("value".parse().unwrap()));
        assert!(implicit_table.is_implicit());

        implicit_table.remove("key");
        assert!(implicit_table.is_implicit());
    }
}#[cfg(test)]
mod tests_llm_16_499_llm_16_499 {
    use crate::Table;
    use crate::repr::Decor;
    use crate::Item;
    use crate::RawString;

    #[test]
    fn test_key_decor() {
        let mut table = Table::new();
        let key = "key1";
        let value = Item::Value("value1".into());

        // No decor should be available for a key with no associated value
        assert!(table.key_decor(key).is_none());

        // Insert a key-value pair with decor
        let mut decor = Decor::new("/* prefix */", "/* suffix */");
        table.insert_formatted(&key.into(), value);
        {
            let decor_mut = table.key_decor_mut(key).unwrap();
            *decor_mut = decor.clone();
        }

        // Decor should be available for existing key
        let key_decor = table.key_decor(key).expect("expected decor to be present");
        assert_eq!(key_decor.prefix(), Some(&RawString::from("/* prefix */")));
        assert_eq!(key_decor.suffix(), Some(&RawString::from("/* suffix */")));

        // Decor should be equal to the manually set decor
        assert_eq!(key_decor, &decor);
    }
}#[cfg(test)]
mod tests_llm_16_501 {
    use crate::table::Table;
    use crate::item::Item;
    use crate::key::Key;
    use crate::value::Value;

    #[test]
    fn table_len_empty() {
        let table = Table::new();
        assert_eq!(table.len(), 0, "Table should be empty");
    }

    #[test]
    fn table_len_non_empty() {
        let mut table = Table::new();
        table.insert("key1", Item::Value(Value::from(42)));
        table.insert("key2", Item::Value(Value::from("value")));
        assert_eq!(table.len(), 2, "Table should contain 2 items");
    }

    #[test]
    fn table_len_with_empty_items() {
        let mut table = Table::new();
        table.insert("key1", Item::Value(Value::from(42)));
        table.insert("key2", Item::Value(Value::from("value")));
        table.insert("key3", Item::None);
        assert_eq!(table.len(), 2, "Table should count only non-empty items");
    }

    #[test]
    fn table_len_with_nested_tables() {
        let mut table = Table::new();
        let mut sub_table1 = Table::new();
        sub_table1.insert("subkey1", Item::Value(Value::from(12)));
        table.insert("key1", Item::Table(sub_table1));

        let mut sub_table2 = Table::new();
        sub_table2.insert("subkey2", Item::Value(Value::from("subvalue")));
        table.insert("key2", Item::Table(sub_table2));
        
        assert_eq!(table.len(), 2, "Table should count non-empty subtable items");
    }
}#[cfg(test)]
mod tests_llm_16_502 {
    use crate::table::Table;

    #[test]
    fn test_table_new() {
        let table = Table::new();
        assert!(table.is_empty());
        assert!(!table.is_implicit());
        assert!(!table.is_dotted());
        assert_eq!(table.position(), None);
        assert_eq!(table.decor(), &Default::default());
    }
}#[cfg(test)]
mod tests_llm_16_503 {
    use crate::table::Table;

    #[test]
    fn test_position_when_created_manually() {
        let table = Table::new();
        assert_eq!(table.position(), None);
    }

    #[test]
    fn test_position_when_set_explicitly() {
        let mut table = Table::new();
        table.set_position(42);
        assert_eq!(table.position(), Some(42));
    }

    #[test]
    fn test_position_when_created_with_position() {
        let table = Table::with_pos(Some(10));
        assert_eq!(table.position(), Some(10));
    }

    #[test]
    fn test_position_after_insertion() {
        let mut table = Table::with_pos(None);
        table.set_position(7);
        table.insert("key", crate::Item::Value(crate::Value::from(42)));
        assert_eq!(table.position(), Some(7));
    }

    #[test]
    fn test_position_when_cleared() {
        let mut table = Table::with_pos(Some(10));
        table.clear();
        assert_eq!(table.position(), Some(10));
    }
}#[cfg(test)]
mod tests_llm_16_506 {
    use crate::table::Table;

    #[test]
    fn test_set_dotted() {
        let mut table = Table::new();
        
        // Check initial dotted status (should be false)
        assert_eq!(table.is_dotted(), false);
        
        // Set dotted status to true
        table.set_dotted(true);
        assert_eq!(table.is_dotted(), true);
        
        // Set dotted status back to false
        table.set_dotted(false);
        assert_eq!(table.is_dotted(), false);
    }
}#[cfg(test)]
mod tests_llm_16_507 {
    use super::*;

use crate::*;
    use crate::Document;

    #[test]
    fn test_set_implicit_true() {
        let mut doc = "[a]\n[a.b]\n".parse::<Document>().expect("invalid toml");
        assert!(!doc["a"].as_table().unwrap().is_implicit());
        doc["a"].as_table_mut().unwrap().set_implicit(true);
        assert!(doc["a"].as_table().unwrap().is_implicit());
        assert_eq!(doc.to_string(), "[a.b]\n");
    }

    #[test]
    fn test_set_implicit_false() {
        let mut doc = "[a]\n[a.b]\n".parse::<Document>().expect("invalid toml");
        doc["a"].as_table_mut().unwrap().set_implicit(true);
        assert!(doc["a"].as_table().unwrap().is_implicit());
        doc["a"].as_table_mut().unwrap().set_implicit(false);
        assert!(!doc["a"].as_table().unwrap().is_implicit());
        assert_eq!(doc.to_string(), "[a]\n[a.b]\n");
    }

    #[test]
    fn test_set_implicit_on_empty_table() {
        let mut doc = "[a]\n".parse::<Document>().expect("invalid toml");
        assert!(!doc["a"].as_table().unwrap().is_implicit());
        doc["a"].as_table_mut().unwrap().set_implicit(true);
        assert!(doc["a"].as_table().unwrap().is_implicit());
        assert_eq!(doc.to_string(), "");
    }

    #[test]
    fn test_set_implicit_on_non_empty_table() {
        let mut doc = "[a]\nx = 1\n[a.b]\n".parse::<Document>().expect("invalid toml");
        assert!(!doc["a"].as_table().unwrap().is_implicit());
        doc["a"].as_table_mut().unwrap().set_implicit(true);
        assert!(doc["a"].as_table().unwrap().is_implicit());
        // Non-empty table should still be displayed
        assert_eq!(doc.to_string(), "[a]\nx = 1\n[a.b]\n");
    }
}#[cfg(test)]
mod tests_llm_16_508 {
    use super::*;

use crate::*;

    #[test]
    fn set_position_updates_doc_position() {
        let mut table = Table::new();
        assert_eq!(table.position(), None); // Initially, position is None

        table.set_position(42); // Set position to an arbitrary number
        assert_eq!(table.position(), Some(42)); // Position should be updated

        table.set_position(7); // Change position to a different number
        assert_eq!(table.position(), Some(7)); // Position should be updated again
    }
}#[cfg(test)]
mod tests_llm_16_509_llm_16_509 {
    use crate::{Item, table::Table, value::Value};

    #[test]
    fn test_sort_values() {
        // Given an unsorted table
        let mut table = Table::new();
        table["c"] = Item::Value(Value::from(3));
        table["a"] = Item::Value(Value::from(1));
        table["b"] = Item::Value(Value::from(2));

        // When sorting the table
        table.sort_values();

        // Then the keys should be sorted
        let keys: Vec<String> = table.iter().map(|(k, _)| k.to_string()).collect();
        assert_eq!(keys, vec!["a", "b", "c"]);

        // And the values should be ordered according to the keys
        let values: Vec<i32> = table.iter().map(|(_, v)| v.as_integer().unwrap() as i32).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_sort_values_with_dotted_table() {
        // Given a table with dotted keys
        let mut table = Table::new();
        table["b.c"] = Item::Value(Value::from(2));
        table["a"] = Item::Value(Value::from(1));
        table["b.a"] = Item::Value(Value::from(3));
        table["b"] = Item::Value(Value::from(4));
        table["b"].as_table_mut().unwrap().set_dotted(true);

        // When sorting the table
        table.sort_values();

        // Then the keys should be sorted, including the dotted keys
        // 'b.a' and 'b.c' should be sorted, but the order of 'a' and 'b' should be preserved
        let sorted_keys: Vec<String> = table.iter().map(|(k, _)| k.to_string()).collect();
        assert_eq!(sorted_keys, vec!["a", "b"]);

        // Verifying the order inside the dotted table 'b'
        let b_table = table["b"].as_table().unwrap();
        let b_keys: Vec<String> = b_table.iter().map(|(k, _)| k.to_string()).collect();
        assert_eq!(b_keys, vec!["a", "c"]);

        let b_values: Vec<i32> = b_table.iter().map(|(_, v)| v.as_integer().unwrap() as i32).collect();
        assert_eq!(b_values, vec![3, 2]);
    }
}#[cfg(test)]
mod tests_llm_16_512 {
    use super::*;

use crate::*;

    #[test]
    fn span_none_on_new_table() {
        let table = Table::new();
        assert_eq!(table.span(), None);
    }

    #[test]
    fn span_some_on_table_with_span() {
        let mut table = Table::new();
        let example_span = 5..42;
        table.span = Some(example_span.clone());
        assert_eq!(table.span(), Some(example_span));
    }
}#[cfg(test)]
mod tests_llm_16_514 {
    use super::*;

use crate::*;
    use crate::table::Table;

    #[test]
    fn table_with_pos_none() {
        let table = Table::with_pos(None);
        assert_eq!(table.position(), None);
    }

    #[test]
    fn table_with_pos_some() {
        let table = Table::with_pos(Some(42));
        assert_eq!(table.position(), Some(42));
    }
}#[cfg(test)]
mod tests_llm_16_517_llm_16_517 {
    use crate::{Table, Item, Value};

    #[test]
    fn test_table_like_len() {
        let mut table = Table::new();

        assert_eq!(table.len(), 0);

        table["a"] = Item::Value(Value::from(42));
        assert_eq!(table.len(), 1);

        table["b"] = Item::Value(Value::from("string"));
        assert_eq!(table.len(), 2);

        table["c"] = Item::Value(Value::from(true));
        assert_eq!(table.len(), 3);

        // Add an empty value, this actually inserts an empty Item, not a Value::None
        table["d"] = Item::None;
        assert_eq!(table.len(), 3);

        // Removing an element should decrease len
        table.remove("b");
        assert_eq!(table.len(), 2);

        // Removing a non-existent element should not change len
        table.remove("non_existent_key");
        assert_eq!(table.len(), 2);

        // Inserting an Item::None should not change len
        table["e"] = Item::None;
        assert_eq!(table.len(), 2);
    }
}#[cfg(test)]
mod tests_llm_16_519 {
    use crate::{Table, Item, Value};

    #[test]
    fn key_vacant_entry() {
        let mut table = Table::new();
        let vacant_entry = table.entry("baz");
        assert_eq!(vacant_entry.key(), "baz");
    }
}