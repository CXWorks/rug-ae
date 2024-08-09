use std::iter::FromIterator;
use indexmap::map::IndexMap;
use crate::key::Key;
use crate::repr::Decor;
use crate::value::DEFAULT_VALUE_DECOR;
use crate::{InlineTable, InternalString, Item, KeyMut, Value};
/// Type representing a TOML non-inline table
#[derive(Clone, Debug, Default)]
pub struct Table {
    pub(crate) decor: Decor,
    pub(crate) implicit: bool,
    pub(crate) dotted: bool,
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
        let modified_cmp = |
            _: &InternalString,
            val1: &TableKeyValue,
            _: &InternalString,
            val2: &TableKeyValue,
        | -> std::cmp::Ordering {
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
            self
                .items
                .iter()
                .filter(|(_, kv)| !kv.value.is_none())
                .map(|(key, kv)| (&key[..], &kv.value)),
        )
    }
    /// Returns an mutable iterator over all key/value pairs, including empty.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        Box::new(
            self
                .items
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
        match self.items.entry(key.into()) {
            indexmap::map::Entry::Occupied(entry) => {
                Entry::Occupied(OccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                Entry::Vacant(VacantEntry { entry, key: None })
            }
        }
    }
    /// Gets the given key's corresponding entry in the Table for in-place manipulation.
    pub fn entry_format<'a>(&'a mut self, key: &Key) -> Entry<'a> {
        match self.items.entry(key.get().into()) {
            indexmap::map::Entry::Occupied(entry) => {
                Entry::Occupied(OccupiedEntry { entry })
            }
            indexmap::map::Entry::Vacant(entry) => {
                Entry::Vacant(VacantEntry {
                    entry,
                    key: Some(key.to_owned()),
                })
            }
        }
    }
    /// Returns an optional reference to an item given the key.
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Item> {
        self.items
            .get(key)
            .and_then(|kv| { if !kv.value.is_none() { Some(&kv.value) } else { None } })
    }
    /// Returns an optional mutable reference to an item given the key.
    pub fn get_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut Item> {
        self.items
            .get_mut(key)
            .and_then(|kv| {
                if !kv.value.is_none() { Some(&mut kv.value) } else { None }
            })
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
    /// Returns true if the table contains an item with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) { !kv.value.is_none() } else { false }
    }
    /// Returns true if the table contains a table with the given key.
    pub fn contains_table(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) { kv.value.is_table() } else { false }
    }
    /// Returns true if the table contains a value with the given key.
    pub fn contains_value(&self, key: &str) -> bool {
        if let Some(kv) = self.items.get(key) { kv.value.is_value() } else { false }
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
    fn get_key_value_mut<'a>(
        &'a mut self,
        key: &str,
    ) -> Option<(KeyMut<'a>, &'a mut Item)>;
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.entry(rug_fuzz_0).or_insert_with(|| rug_fuzz_1.parse().unwrap());
        table.entry(rug_fuzz_2).or_insert_with(|| rug_fuzz_3.parse().unwrap());
        debug_assert!(! table.is_empty());
        table.clear();
        debug_assert!(table.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_101 {
    use crate::table::Table;
    #[test]
    fn table_contains_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert!(
            ! table.contains_key(rug_fuzz_0), "Table should not contain 'key1'"
        );
        table.insert(rug_fuzz_1, rug_fuzz_2.parse().unwrap());
        debug_assert!(table.contains_key(rug_fuzz_3), "Table should contain 'key1'");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_103 {
    use crate::{Document, Item, Table, TableLike, Value};
    #[test]
    fn entry_format_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = rug_fuzz_0;
        let entry = table.entry_format(&key.parse().unwrap());
        debug_assert_eq!(entry.key(), key);
        let value = Value::from(rug_fuzz_1);
        table.insert_formatted(&key.parse().unwrap(), Item::Value(value.clone()));
        let entry = table.entry_format(&key.parse().unwrap());
        match entry {
            crate::Entry::Occupied(occupied) => {
                debug_assert_eq!(occupied.get().as_integer(), Some(42));
            }
            crate::Entry::Vacant(_) => panic!("Expected an occupied entry"),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_105 {
    use crate::{Table, Item, Value};
    #[test]
    fn get_key_value_existing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        let (key, item) = table.get_key_value(rug_fuzz_2).unwrap();
        debug_assert_eq!(key.get(), "key");
        debug_assert_eq!(item.as_value().unwrap().as_str().unwrap(), "value");
             }
}
}
}    }
    #[test]
    fn get_key_value_non_existing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Table::new();
        debug_assert!(table.get_key_value(rug_fuzz_0).is_none());
             }
}
}
}    }
    #[test]
    fn get_key_value_empty_item() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::None;
        debug_assert!(table.get_key_value(rug_fuzz_1).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_106 {
    use super::*;
    use crate::*;
    use crate::Item;
    #[test]
    fn test_get_key_value_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.parse().unwrap()));
        table.insert(rug_fuzz_2, Item::Value(rug_fuzz_3.parse().unwrap()));
        {
            let (_keymut, value_mut) = table.get_key_value_mut(rug_fuzz_4).unwrap();
            if let Item::Value(value) = value_mut {
                *value = rug_fuzz_5.parse().unwrap();
            }
        }
        if let Item::Value(value) = table.get(rug_fuzz_6).unwrap() {
            debug_assert_eq!(value.as_str(), Some("updated_value1"));
        } else {
            panic!("Value not found for 'key1'");
        }
        debug_assert!(table.get_key_value_mut(rug_fuzz_7).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_107_llm_16_107 {
    use crate::{Table, Item, Value, TableLike};
    #[test]
    fn get_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = rug_fuzz_0;
        table.insert(key, Item::Value(Value::from(rug_fuzz_1)));
        if let Some(item) = table.get_mut(key) {
            if let Item::Value(value) = item {
                if let Some(value_str) = value.as_str() {
                    let mut new_value = Value::from(format!("{}_modified", value_str));
                    *value = new_value;
                }
            }
        }
        let expected_value = rug_fuzz_2;
        debug_assert_eq!(
            table.get(key).unwrap().as_value().unwrap().as_str().unwrap(), expected_value
        );
             }
}
}
}    }
    #[test]
    fn get_mut_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert!(table.get_mut(rug_fuzz_0).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_108 {
    use crate::table::Table;
    use crate::value::Value;
    use crate::key::Key;
    use crate::item::Item;
    #[test]
    fn get_values_empty_table() {
        let _rug_st_tests_llm_16_108_rrrruuuugggg_get_values_empty_table = 0;
        let table = Table::new();
        debug_assert!(table.get_values().is_empty());
        let _rug_ed_tests_llm_16_108_rrrruuuugggg_get_values_empty_table = 0;
    }
    #[test]
    fn get_values_with_single_pair() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = Key::new(rug_fuzz_0);
        let value = Value::from(rug_fuzz_1);
        table.insert(key.get(), Item::Value(value));
        let values = table.get_values();
        debug_assert_eq!(values.len(), 1);
        debug_assert_eq!(values[rug_fuzz_2].0[rug_fuzz_3].get(), "key");
        debug_assert_eq!(values[rug_fuzz_4].1.as_str().unwrap(), "value");
             }
}
}
}    }
    #[test]
    fn get_values_nested_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, i64, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = Key::new(rug_fuzz_0);
        let mut subtable = Table::new();
        let subkey = Key::new(rug_fuzz_1);
        let value = Value::from(rug_fuzz_2);
        subtable.insert(subkey.get(), Item::Value(value));
        table.insert(key.get(), Item::Table(subtable));
        let values = table.get_values();
        debug_assert_eq!(values.len(), 1);
        debug_assert_eq!(values[rug_fuzz_3].0[rug_fuzz_4].get(), "parent");
        debug_assert_eq!(values[rug_fuzz_5].0[rug_fuzz_6].get(), "child");
        debug_assert_eq!(values[rug_fuzz_7].1.as_integer().unwrap(), 42);
             }
}
}
}    }
    #[test]
    fn get_values_with_dotted_keys() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, bool, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = Key::new(rug_fuzz_0);
        let value = Value::from(rug_fuzz_1);
        table.insert_formatted(&key, Item::Value(value));
        table.set_dotted(rug_fuzz_2);
        let values = table.get_values();
        debug_assert_eq!(values.len(), 1);
        debug_assert_eq!(values[rug_fuzz_3].0[rug_fuzz_4].get(), "parent");
        debug_assert_eq!(values[rug_fuzz_5].0[rug_fuzz_6].get(), "child");
        debug_assert_eq!(values[rug_fuzz_7].1.as_str().unwrap(), "value");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_110 {
    use super::*;
    use crate::*;
    use crate::table::Table;
    use crate::repr::Decor;
    #[test]
    fn test_is_not_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.set_dotted(rug_fuzz_0);
        debug_assert_eq!(table.is_dotted(), false);
             }
}
}
}    }
    #[test]
    fn test_is_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.set_dotted(rug_fuzz_0);
        debug_assert_eq!(table.is_dotted(), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_112_llm_16_112 {
    use crate::{table::Table, Item, Value, table::KeyMut};
    #[test]
    fn test_iter_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(&str, &str, &str, &str, i32, i32, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        table.insert(rug_fuzz_2, Item::Value(Value::from(rug_fuzz_3)));
        let mut count = rug_fuzz_4;
        {
            let mut iter_mut = table.iter_mut();
            while let Some((key, value)) = iter_mut.next() {
                count += rug_fuzz_5;
                if key.get() == rug_fuzz_6 {
                    *value = Item::Value(Value::from(rug_fuzz_7));
                }
            }
        }
        debug_assert_eq!(count, 2);
        debug_assert_eq!(table.get(rug_fuzz_8).and_then(Item::as_str), Some("changed"));
        debug_assert_eq!(table.get(rug_fuzz_9).and_then(Item::as_str), Some("value2"));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_114 {
    use super::*;
    use crate::*;
    use crate::repr::Decor;
    #[test]
    fn key_decor_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        let decor = table.key_decor_mut(rug_fuzz_2).unwrap();
        decor.set_prefix(rug_fuzz_3);
        decor.set_suffix(rug_fuzz_4);
        let modified_decor = table.key_decor(rug_fuzz_5).unwrap();
        debug_assert_eq!(modified_decor.prefix(), Some(& "prefix_".into()));
        debug_assert_eq!(modified_decor.suffix(), Some(& "_suffix".into()));
             }
}
}
}    }
    #[test]
    fn key_decor_mut_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert!(table.key_decor_mut(rug_fuzz_0).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_115 {
    use crate::{Item, Table};
    #[test]
    fn remove_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.into()));
        table.insert(rug_fuzz_2, Item::Value(rug_fuzz_3.into()));
        let removed = table.remove(rug_fuzz_4);
        debug_assert_eq!(removed.is_some(), true);
        debug_assert_eq!(table.contains_key(rug_fuzz_5), false);
             }
}
}
}    }
    #[test]
    fn remove_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.into()));
        let removed = table.remove(rug_fuzz_2);
        debug_assert_eq!(removed.is_none(), true);
        debug_assert_eq!(table.contains_key(rug_fuzz_3), false);
             }
}
}
}    }
    #[test]
    fn remove_key_from_empty_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let removed = table.remove(rug_fuzz_0);
        debug_assert_eq!(removed.is_none(), true);
        debug_assert_eq!(table.contains_key(rug_fuzz_1), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_116 {
    use crate::{Table, Item, table::TableLike};
    #[test]
    fn test_set_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
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
mod tests_llm_16_464_llm_16_464 {
    use crate::{Entry, Item, InternalString, Key};
    use crate::table::TableKeyValue;
    #[test]
    fn test_entry_key_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = crate::Table::new();
        table[rug_fuzz_0] = Item::Value(rug_fuzz_1.parse().unwrap());
        if let Entry::Occupied(oe) = table.entry(rug_fuzz_2) {
            debug_assert_eq!(oe.key(), "test_key");
        } else {
            panic!("Expected entry to be occupied");
        }
             }
}
}
}    }
    #[test]
    fn test_entry_key_vacant() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = crate::Table::new();
        if let Entry::Vacant(ve) = table.entry(rug_fuzz_0) {
            debug_assert_eq!(ve.key(), "test_key");
        } else {
            panic!("Expected entry to be vacant");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_465 {
    use crate::table::{Table, Item, Entry};
    use crate::Value;
    #[test]
    fn test_or_insert_with_existing_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        debug_assert_eq!(
            table.entry(rug_fuzz_2).or_insert(Item::Value(Value::from(rug_fuzz_3)))
            .as_value().unwrap().as_integer(), Some(42)
        );
             }
}
}
}    }
    #[test]
    fn test_or_insert_with_vacant_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert!(table.get(rug_fuzz_0).is_none());
        debug_assert_eq!(
            table.entry(rug_fuzz_1).or_insert(Item::Value(Value::from(rug_fuzz_2)))
            .as_value().unwrap().as_integer(), Some(99)
        );
        debug_assert_eq!(
            table.get(rug_fuzz_3).unwrap().as_value().unwrap().as_integer(), Some(99)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_466 {
    use super::*;
    use crate::*;
    use crate::Item;
    use crate::Formatted;
    use crate::value::Value;
    #[test]
    fn test_or_insert_with_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = rug_fuzz_0;
        table.insert(key, Item::Value(Value::Integer(Formatted::new(rug_fuzz_1))));
        let entry = table.entry(key);
        let value = entry
            .or_insert_with(|| Item::Value(Value::Integer(Formatted::new(rug_fuzz_2))));
        match value {
            Item::Value(v) => {
                match v {
                    Value::Integer(i) => debug_assert_eq!(* i.value(), 42),
                    _ => panic!("Not an integer"),
                }
            }
            _ => panic!("Not a value"),
        }
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

        let mut table = Table::new();
        let key = rug_fuzz_0;
        let entry = table.entry(key);
        let value = entry
            .or_insert_with(|| Item::Value(Value::Integer(Formatted::new(rug_fuzz_1))));
        match value {
            Item::Value(v) => {
                match v {
                    Value::Integer(i) => debug_assert_eq!(* i.value(), 100),
                    _ => panic!("Not an integer"),
                }
            }
            _ => panic!("Not a value"),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_467_llm_16_467 {
    use super::*;
    use crate::*;
    use crate::Item;
    use crate::Value;
    #[test]
    fn test_get() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let value = Value::from(rug_fuzz_0);
        table.insert(rug_fuzz_1, Item::Value(value));
        let entry = table.entry(rug_fuzz_2);
        if let Entry::Occupied(occupied_entry) = entry {
            let item = occupied_entry.get();
            debug_assert!(item.is_value());
            debug_assert_eq!(item.as_value().unwrap().as_integer().unwrap(), 42);
        } else {
            panic!("Expected entry to be occupied");
        }
             }
}
}
}    }
}
#[cfg(test)]
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::Integer(Formatted::new(rug_fuzz_1)));
        let mut entry = table.entry(rug_fuzz_2);
        if let Entry::Occupied(mut occupied) = entry {
            let old_value = occupied
                .insert(
                    Item::Value(Value::String(Formatted::new(rug_fuzz_3.to_string()))),
                );
            if let Item::Value(Value::Integer(old_int_value)) = old_value {
                debug_assert_eq!(* old_int_value.value(), 42);
            } else {
                panic!("Old value was not an integer");
            }
            if let Item::Value(Value::String(new_value)) = occupied.get() {
                debug_assert_eq!(new_value.value(), "new");
            } else {
                panic!("New value was not inserted");
            }
        } else {
            panic!("Entry expected to be occupied");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_470_llm_16_470 {
    use crate::{Document, Item, Value, table::Table};
    #[test]
    fn test_table_occupied_entry_into_mut() {
        let _rug_st_tests_llm_16_470_llm_16_470_rrrruuuugggg_test_table_occupied_entry_into_mut = 0;
        let rug_fuzz_0 = r#"
        [package]
        name = "your_package"
        "#;
        let rug_fuzz_1 = "Parsing failed";
        let rug_fuzz_2 = "package";
        let rug_fuzz_3 = "Not a table";
        let rug_fuzz_4 = "name";
        let rug_fuzz_5 = "my_package";
        let rug_fuzz_6 = "name";
        let toml_content = rug_fuzz_0;
        let mut doc = toml_content.parse::<Document>().expect(rug_fuzz_1);
        let package_entry = doc.as_table_mut().entry(rug_fuzz_2);
        if let crate::table::Entry::Occupied(mut entry) = package_entry {
            let package_table = entry.get_mut().as_table_mut().expect(rug_fuzz_3);
            package_table[rug_fuzz_4] = Item::Value(Value::from(rug_fuzz_5));
            debug_assert_eq!(package_table[rug_fuzz_6].as_str(), Some("my_package"));
        } else {
            panic!("package entry is not occupied");
        }
        let _rug_ed_tests_llm_16_470_llm_16_470_rrrruuuugggg_test_table_occupied_entry_into_mut = 0;
    }
}
#[cfg(test)]
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
    fn get_occupied_entry<'a>(
        table: &'a mut Table,
        key: &str,
    ) -> table::OccupiedEntry<'a> {
        if let crate::table::Entry::Occupied(entry) = table.entry(key) {
            entry
        } else {
            panic!("Expected key to be occupied.")
        }
    }
    #[test]
    fn test_remove_entry() {
        let mut table = Table::new();
        table.insert("key1", Item::Value(Value::String(formatted_string("value1"))));
        table.insert("key2", Item::Value(Value::String(formatted_string("value2"))));
        table.insert("key3", Item::Value(Value::String(formatted_string("value3"))));
        let occupied_entry = get_occupied_entry(&mut table, "key2");
        let removed_item = occupied_entry.remove();
        assert!(
            matches!(removed_item, Item::Value(Value::String(v)) if v.value() ==
            "value2")
        );
        assert!(table.get("key2").is_none());
        assert_eq!(table.len(), 2);
    }
}
#[cfg(test)]
mod tests_llm_16_475 {
    use crate::table::Table;
    use crate::Item;
    use std::str::FromStr;
    #[test]
    fn test_table_clear_empty() {
        let _rug_st_tests_llm_16_475_rrrruuuugggg_test_table_clear_empty = 0;
        let mut table = Table::new();
        table.clear();
        debug_assert!(table.is_empty());
        let _rug_ed_tests_llm_16_475_rrrruuuugggg_test_table_clear_empty = 0;
    }
    #[test]
    fn test_table_clear_with_entries() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.parse().unwrap()));
        table.insert(rug_fuzz_2, Item::Value(rug_fuzz_3.parse().unwrap()));
        debug_assert_eq!(table.len(), 2);
        table.clear();
        debug_assert!(table.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_476_llm_16_476 {
    use super::*;
    use crate::*;
    #[test]
    fn test_contains_array_of_tables() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert!(! table.contains_array_of_tables(rug_fuzz_0));
        table.insert(rug_fuzz_1, Item::ArrayOfTables(crate::ArrayOfTables::new()));
        debug_assert!(table.contains_array_of_tables(rug_fuzz_2));
        table.insert(rug_fuzz_3, Item::Value(crate::Value::from(rug_fuzz_4)));
        debug_assert!(! table.contains_array_of_tables(rug_fuzz_5));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_477_llm_16_477 {
    use crate as toml_edit;
    use crate::Item;
    use crate::Value;
    use crate::Table;
    #[test]
    fn test_contains_key_present() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        debug_assert!(table.contains_key(rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_contains_key_absent() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Table::new();
        debug_assert!(! table.contains_key(rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_contains_key_empty_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::None);
        debug_assert!(! table.contains_key(rug_fuzz_1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_478 {
    use crate::table::Table;
    #[test]
    fn test_contains_table_with_existing_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, crate::Item::Table(Table::new()));
        debug_assert!(table.contains_table(rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_contains_table_without_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Table::new();
        debug_assert!(! table.contains_table(rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_contains_table_with_non_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, crate::Item::Value(crate::Value::from(rug_fuzz_1)));
        debug_assert!(! table.contains_table(rug_fuzz_2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_479 {
    use crate::{Table, Item, Value};
    #[test]
    fn test_contains_value_with_existing_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        debug_assert!(table.contains_value(rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_contains_value_with_non_existing_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Table::new();
        debug_assert!(! table.contains_value(rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_contains_value_with_existing_non_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Table(Table::new()));
        debug_assert!(! table.contains_value(rug_fuzz_1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_484 {
    use super::*;
    use crate::*;
    use crate::table::Table;
    use crate::key::Key;
    use crate::key::KeyMut;
    use crate::item::Item;
    #[test]
    fn test_entry_format_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = rug_fuzz_0;
        table.insert(key, Item::Value(rug_fuzz_1.into()));
        let key = Key::new(key);
        if let Entry::Occupied(occupied) = table.entry_format(&key) {
            let key = occupied.key();
            debug_assert_eq!(key, "key1");
            let value = occupied.get();
            debug_assert_eq!(value.as_integer(), Some(1));
        } else {
            panic!("Expected occupied entry");
        }
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

        let mut table = Table::new();
        let key = Key::new(rug_fuzz_0);
        if let Entry::Vacant(vacant) = table.entry_format(&key) {
            let key = vacant.key();
            debug_assert_eq!(key, "key2");
        } else {
            panic!("Expected vacant entry");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_488 {
    use crate::table::Table;
    use crate::Item;
    use std::str::FromStr;
    #[test]
    fn test_get_key_value_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = rug_fuzz_0;
        let value = rug_fuzz_1;
        table.insert(key, Item::Value(value.into()));
        let (key_mut, value_mut) = table.get_key_value_mut(key).unwrap();
        let key_str = key_mut.get();
        let val_str = value_mut.as_value().unwrap().as_str().unwrap();
        debug_assert_eq!(key_str, key);
        debug_assert_eq!(val_str, value);
        *value_mut = Item::Value(FromStr::from_str(rug_fuzz_2).unwrap());
        let modified_value = table
            .get(key)
            .unwrap()
            .as_value()
            .unwrap()
            .as_str()
            .unwrap();
        debug_assert_eq!(modified_value, "modified_value");
        debug_assert!(table.get_key_value_mut(rug_fuzz_3).is_none());
        table.insert(key, Item::None);
        debug_assert!(table.get_key_value_mut(key).is_none());
        let mut sub_table = Table::new();
        let nested_key = rug_fuzz_4;
        sub_table.insert(nested_key, Item::Value(rug_fuzz_5.into()));
        table.insert(key, Item::Table(sub_table));
        debug_assert!(table.get_key_value_mut(key).is_none());
        table.clear();
        debug_assert!(table.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_494 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_dotted_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.set_dotted(rug_fuzz_0);
        debug_assert!(table.is_dotted());
             }
}
}
}    }
    #[test]
    fn test_is_dotted_false() {
        let _rug_st_tests_llm_16_494_rrrruuuugggg_test_is_dotted_false = 0;
        let table = Table::new();
        debug_assert!(! table.is_dotted());
        let _rug_ed_tests_llm_16_494_rrrruuuugggg_test_is_dotted_false = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_495 {
    use super::*;
    use crate::*;
    use crate::Item;
    #[test]
    fn test_table_is_empty_with_empty_table() {
        let _rug_st_tests_llm_16_495_rrrruuuugggg_test_table_is_empty_with_empty_table = 0;
        let table = Table::new();
        debug_assert!(table.is_empty());
        let _rug_ed_tests_llm_16_495_rrrruuuugggg_test_table_is_empty_with_empty_table = 0;
    }
    #[test]
    fn test_table_is_empty_with_non_empty_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.parse().unwrap()));
        debug_assert!(! table.is_empty());
             }
}
}
}    }
    #[test]
    fn test_table_is_empty_after_clear() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.parse().unwrap()));
        table.clear();
        debug_assert!(table.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_496 {
    use crate::{Table, Item};
    #[test]
    fn test_implicit_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(bool, bool, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.set_implicit(rug_fuzz_0);
        debug_assert!(! table.is_implicit());
        let mut implicit_table = Table::new();
        implicit_table.set_implicit(rug_fuzz_1);
        debug_assert!(implicit_table.is_implicit());
        implicit_table.insert(rug_fuzz_2, Item::Value(rug_fuzz_3.parse().unwrap()));
        debug_assert!(implicit_table.is_implicit());
        implicit_table.remove(rug_fuzz_4);
        debug_assert!(implicit_table.is_implicit());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_499_llm_16_499 {
    use crate::Table;
    use crate::repr::Decor;
    use crate::Item;
    use crate::RawString;
    #[test]
    fn test_key_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = rug_fuzz_0;
        let value = Item::Value(rug_fuzz_1.into());
        debug_assert!(table.key_decor(key).is_none());
        let mut decor = Decor::new(rug_fuzz_2, rug_fuzz_3);
        table.insert_formatted(&key.into(), value);
        {
            let decor_mut = table.key_decor_mut(key).unwrap();
            *decor_mut = decor.clone();
        }
        let key_decor = table.key_decor(key).expect(rug_fuzz_4);
        debug_assert_eq!(key_decor.prefix(), Some(& RawString::from("/* prefix */")));
        debug_assert_eq!(key_decor.suffix(), Some(& RawString::from("/* suffix */")));
        debug_assert_eq!(key_decor, & decor);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_501 {
    use crate::table::Table;
    use crate::item::Item;
    use crate::key::Key;
    use crate::value::Value;
    #[test]
    fn table_len_empty() {
        let _rug_st_tests_llm_16_501_rrrruuuugggg_table_len_empty = 0;
        let table = Table::new();
        debug_assert_eq!(table.len(), 0, "Table should be empty");
        let _rug_ed_tests_llm_16_501_rrrruuuugggg_table_len_empty = 0;
    }
    #[test]
    fn table_len_non_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        table.insert(rug_fuzz_2, Item::Value(Value::from(rug_fuzz_3)));
        debug_assert_eq!(table.len(), 2, "Table should contain 2 items");
             }
}
}
}    }
    #[test]
    fn table_len_with_empty_items() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, i64, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        table.insert(rug_fuzz_2, Item::Value(Value::from(rug_fuzz_3)));
        table.insert(rug_fuzz_4, Item::None);
        debug_assert_eq!(table.len(), 2, "Table should count only non-empty items");
             }
}
}
}    }
    #[test]
    fn table_len_with_nested_tables() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i64, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let mut sub_table1 = Table::new();
        sub_table1.insert(rug_fuzz_0, Item::Value(Value::from(rug_fuzz_1)));
        table.insert(rug_fuzz_2, Item::Table(sub_table1));
        let mut sub_table2 = Table::new();
        sub_table2.insert(rug_fuzz_3, Item::Value(Value::from(rug_fuzz_4)));
        table.insert(rug_fuzz_5, Item::Table(sub_table2));
        debug_assert_eq!(table.len(), 2, "Table should count non-empty subtable items");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_502 {
    use crate::table::Table;
    #[test]
    fn test_table_new() {
        let _rug_st_tests_llm_16_502_rrrruuuugggg_test_table_new = 0;
        let table = Table::new();
        debug_assert!(table.is_empty());
        debug_assert!(! table.is_implicit());
        debug_assert!(! table.is_dotted());
        debug_assert_eq!(table.position(), None);
        debug_assert_eq!(table.decor(), & Default::default());
        let _rug_ed_tests_llm_16_502_rrrruuuugggg_test_table_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_503 {
    use crate::table::Table;
    #[test]
    fn test_position_when_created_manually() {
        let _rug_st_tests_llm_16_503_rrrruuuugggg_test_position_when_created_manually = 0;
        let table = Table::new();
        debug_assert_eq!(table.position(), None);
        let _rug_ed_tests_llm_16_503_rrrruuuugggg_test_position_when_created_manually = 0;
    }
    #[test]
    fn test_position_when_set_explicitly() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table.set_position(rug_fuzz_0);
        debug_assert_eq!(table.position(), Some(42));
             }
}
}
}    }
    #[test]
    fn test_position_when_created_with_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Table::with_pos(Some(rug_fuzz_0));
        debug_assert_eq!(table.position(), Some(10));
             }
}
}
}    }
    #[test]
    fn test_position_after_insertion() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::with_pos(None);
        table.set_position(rug_fuzz_0);
        table.insert(rug_fuzz_1, crate::Item::Value(crate::Value::from(rug_fuzz_2)));
        debug_assert_eq!(table.position(), Some(7));
             }
}
}
}    }
    #[test]
    fn test_position_when_cleared() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::with_pos(Some(rug_fuzz_0));
        table.clear();
        debug_assert_eq!(table.position(), Some(10));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_506 {
    use crate::table::Table;
    #[test]
    fn test_set_dotted() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert_eq!(table.is_dotted(), false);
        table.set_dotted(rug_fuzz_0);
        debug_assert_eq!(table.is_dotted(), true);
        table.set_dotted(rug_fuzz_1);
        debug_assert_eq!(table.is_dotted(), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_507 {
    use super::*;
    use crate::*;
    use crate::Document;
    #[test]
    fn test_set_implicit_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut doc = rug_fuzz_0.parse::<Document>().expect(rug_fuzz_1);
        debug_assert!(! doc[rug_fuzz_2].as_table().unwrap().is_implicit());
        doc[rug_fuzz_3].as_table_mut().unwrap().set_implicit(rug_fuzz_4);
        debug_assert!(doc[rug_fuzz_5].as_table().unwrap().is_implicit());
        debug_assert_eq!(doc.to_string(), "[a.b]\n");
             }
}
}
}    }
    #[test]
    fn test_set_implicit_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, bool, &str, &str, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut doc = rug_fuzz_0.parse::<Document>().expect(rug_fuzz_1);
        doc[rug_fuzz_2].as_table_mut().unwrap().set_implicit(rug_fuzz_3);
        debug_assert!(doc[rug_fuzz_4].as_table().unwrap().is_implicit());
        doc[rug_fuzz_5].as_table_mut().unwrap().set_implicit(rug_fuzz_6);
        debug_assert!(! doc[rug_fuzz_7].as_table().unwrap().is_implicit());
        debug_assert_eq!(doc.to_string(), "[a]\n[a.b]\n");
             }
}
}
}    }
    #[test]
    fn test_set_implicit_on_empty_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut doc = rug_fuzz_0.parse::<Document>().expect(rug_fuzz_1);
        debug_assert!(! doc[rug_fuzz_2].as_table().unwrap().is_implicit());
        doc[rug_fuzz_3].as_table_mut().unwrap().set_implicit(rug_fuzz_4);
        debug_assert!(doc[rug_fuzz_5].as_table().unwrap().is_implicit());
        debug_assert_eq!(doc.to_string(), "");
             }
}
}
}    }
    #[test]
    fn test_set_implicit_on_non_empty_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut doc = rug_fuzz_0.parse::<Document>().expect(rug_fuzz_1);
        debug_assert!(! doc[rug_fuzz_2].as_table().unwrap().is_implicit());
        doc[rug_fuzz_3].as_table_mut().unwrap().set_implicit(rug_fuzz_4);
        debug_assert!(doc[rug_fuzz_5].as_table().unwrap().is_implicit());
        debug_assert_eq!(doc.to_string(), "[a]\nx = 1\n[a.b]\n");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_508 {
    use super::*;
    use crate::*;
    #[test]
    fn set_position_updates_doc_position() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert_eq!(table.position(), None);
        table.set_position(rug_fuzz_0);
        debug_assert_eq!(table.position(), Some(42));
        table.set_position(rug_fuzz_1);
        debug_assert_eq!(table.position(), Some(7));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_509_llm_16_509 {
    use crate::{Item, table::Table, value::Value};
    #[test]
    fn test_sort_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i64, &str, i64, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        table[rug_fuzz_2] = Item::Value(Value::from(rug_fuzz_3));
        table[rug_fuzz_4] = Item::Value(Value::from(rug_fuzz_5));
        table.sort_values();
        let keys: Vec<String> = table.iter().map(|(k, _)| k.to_string()).collect();
        debug_assert_eq!(keys, vec!["a", "b", "c"]);
        let values: Vec<i32> = table
            .iter()
            .map(|(_, v)| v.as_integer().unwrap() as i32)
            .collect();
        debug_assert_eq!(values, vec![1, 2, 3]);
             }
}
}
}    }
    #[test]
    fn test_sort_values_with_dotted_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(&str, i64, &str, i64, &str, i64, &str, i64, &str, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        table[rug_fuzz_2] = Item::Value(Value::from(rug_fuzz_3));
        table[rug_fuzz_4] = Item::Value(Value::from(rug_fuzz_5));
        table[rug_fuzz_6] = Item::Value(Value::from(rug_fuzz_7));
        table[rug_fuzz_8].as_table_mut().unwrap().set_dotted(rug_fuzz_9);
        table.sort_values();
        let sorted_keys: Vec<String> = table
            .iter()
            .map(|(k, _)| k.to_string())
            .collect();
        debug_assert_eq!(sorted_keys, vec!["a", "b"]);
        let b_table = table[rug_fuzz_10].as_table().unwrap();
        let b_keys: Vec<String> = b_table.iter().map(|(k, _)| k.to_string()).collect();
        debug_assert_eq!(b_keys, vec!["a", "c"]);
        let b_values: Vec<i32> = b_table
            .iter()
            .map(|(_, v)| v.as_integer().unwrap() as i32)
            .collect();
        debug_assert_eq!(b_values, vec![3, 2]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_512 {
    use super::*;
    use crate::*;
    #[test]
    fn span_none_on_new_table() {
        let _rug_st_tests_llm_16_512_rrrruuuugggg_span_none_on_new_table = 0;
        let table = Table::new();
        debug_assert_eq!(table.span(), None);
        let _rug_ed_tests_llm_16_512_rrrruuuugggg_span_none_on_new_table = 0;
    }
    #[test]
    fn span_some_on_table_with_span() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let example_span = rug_fuzz_0..rug_fuzz_1;
        table.span = Some(example_span.clone());
        debug_assert_eq!(table.span(), Some(example_span));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_514 {
    use super::*;
    use crate::*;
    use crate::table::Table;
    #[test]
    fn table_with_pos_none() {
        let _rug_st_tests_llm_16_514_rrrruuuugggg_table_with_pos_none = 0;
        let table = Table::with_pos(None);
        debug_assert_eq!(table.position(), None);
        let _rug_ed_tests_llm_16_514_rrrruuuugggg_table_with_pos_none = 0;
    }
    #[test]
    fn table_with_pos_some() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Table::with_pos(Some(rug_fuzz_0));
        debug_assert_eq!(table.position(), Some(42));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_517_llm_16_517 {
    use crate::{Table, Item, Value};
    #[test]
    fn test_table_like_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(&str, i64, &str, &str, &str, bool, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        debug_assert_eq!(table.len(), 0);
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        debug_assert_eq!(table.len(), 1);
        table[rug_fuzz_2] = Item::Value(Value::from(rug_fuzz_3));
        debug_assert_eq!(table.len(), 2);
        table[rug_fuzz_4] = Item::Value(Value::from(rug_fuzz_5));
        debug_assert_eq!(table.len(), 3);
        table[rug_fuzz_6] = Item::None;
        debug_assert_eq!(table.len(), 3);
        table.remove(rug_fuzz_7);
        debug_assert_eq!(table.len(), 2);
        table.remove(rug_fuzz_8);
        debug_assert_eq!(table.len(), 2);
        table[rug_fuzz_9] = Item::None;
        debug_assert_eq!(table.len(), 2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_519 {
    use crate::{Table, Item, Value};
    #[test]
    fn key_vacant_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let vacant_entry = table.entry(rug_fuzz_0);
        debug_assert_eq!(vacant_entry.key(), "baz");
             }
}
}
}    }
}
