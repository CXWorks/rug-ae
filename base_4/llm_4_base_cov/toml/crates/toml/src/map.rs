//! A map of `String` to [Value].
//!
//! By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
//! feature of toml-rs to use [`IndexMap`] instead.
//!
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`IndexMap`]: https://docs.rs/indexmap
use crate::value::Value;
use serde::{de, ser};
use std::borrow::Borrow;
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops;
#[cfg(not(feature = "preserve_order"))]
use std::collections::{btree_map, BTreeMap};
#[cfg(feature = "preserve_order")]
use indexmap::{self, IndexMap};
/// Represents a TOML key/value type.
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
#[cfg(not(feature = "preserve_order"))]
type MapImpl<K, V> = BTreeMap<K, V>;
#[cfg(feature = "preserve_order")]
type MapImpl<K, V> = IndexMap<K, V>;
impl Map<String, Value> {
    /// Makes a new empty Map.
    #[inline]
    pub fn new() -> Self {
        Map { map: MapImpl::new() }
    }
    #[cfg(not(feature = "preserve_order"))]
    /// Makes a new empty Map with the given initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let _ = capacity;
        Map { map: BTreeMap::new() }
    }
    #[cfg(feature = "preserve_order")]
    /// Makes a new empty Map with the given initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            map: IndexMap::with_capacity(capacity),
        }
    }
    /// Clears the map, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Value>
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.get(key)
    }
    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.contains_key(key)
    }
    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Value>
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.get_mut(key)
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[inline]
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.map.insert(k, v)
    }
    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.remove(key)
    }
    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    pub fn entry<S>(&mut self, key: S) -> Entry<'_>
    where
        S: Into<String>,
    {
        #[cfg(feature = "preserve_order")]
        use indexmap::map::Entry as EntryImpl;
        #[cfg(not(feature = "preserve_order"))]
        use std::collections::btree_map::Entry as EntryImpl;
        match self.map.entry(key.into()) {
            EntryImpl::Vacant(vacant) => Entry::Vacant(VacantEntry { vacant }),
            EntryImpl::Occupied(occupied) => Entry::Occupied(OccupiedEntry { occupied }),
        }
    }
    /// Returns the number of elements in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Returns true if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Gets an iterator over the entries of the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter { iter: self.map.iter() }
    }
    /// Gets a mutable iterator over the entries of the map.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
    /// Gets an iterator over the keys of the map.
    #[inline]
    pub fn keys(&self) -> Keys<'_> {
        Keys { iter: self.map.keys() }
    }
    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn values(&self) -> Values<'_> {
        Values { iter: self.map.values() }
    }
}
impl Default for Map<String, Value> {
    #[inline]
    fn default() -> Self {
        Map { map: MapImpl::new() }
    }
}
impl Clone for Map<String, Value> {
    #[inline]
    fn clone(&self) -> Self {
        Map { map: self.map.clone() }
    }
}
impl PartialEq for Map<String, Value> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}
/// Access an element of this map. Panics if the given key is not present in the
/// map.
impl<'a, Q: ?Sized> ops::Index<&'a Q> for Map<String, Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash,
{
    type Output = Value;
    fn index(&self, index: &Q) -> &Value {
        self.map.index(index)
    }
}
/// Mutably access an element of this map. Panics if the given key is not
/// present in the map.
impl<'a, Q: ?Sized> ops::IndexMut<&'a Q> for Map<String, Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Value {
        self.map.get_mut(index).expect("no entry found for key")
    }
}
impl Debug for Map<String, Value> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.map.fmt(formatter)
    }
}
impl ser::Serialize for Map<String, Value> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_key(k)?;
            map.serialize_value(v)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for Map<String, Value> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Map<String, Value>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }
            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Map::new())
            }
            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut values = Map::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
impl FromIterator<(String, Value)> for Map<String, Value> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        Map {
            map: FromIterator::from_iter(iter),
        }
    }
}
impl Extend<(String, Value)> for Map<String, Value> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        self.map.extend(iter);
    }
}
macro_rules! delegate_iterator {
    (($name:ident $($generics:tt)*) => $item:ty) => {
        impl $($generics)* Iterator for $name $($generics)* { type Item = $item;
        #[inline] fn next(& mut self) -> Option < Self::Item > { self.iter.next() }
        #[inline] fn size_hint(& self) -> (usize, Option < usize >) { self.iter
        .size_hint() } } impl $($generics)* DoubleEndedIterator for $name $($generics)* {
        #[inline] fn next_back(& mut self) -> Option < Self::Item > { self.iter
        .next_back() } } impl $($generics)* ExactSizeIterator for $name $($generics)* {
        #[inline] fn len(& self) -> usize { self.iter.len() } }
    };
}
/// A view into a single entry in a map, which may either be vacant or occupied.
/// This enum is constructed from the [`entry`] method on [`Map`].
///
/// [`entry`]: struct.Map.html#method.entry
/// [`Map`]: struct.Map.html
pub enum Entry<'a> {
    /// A vacant Entry.
    Vacant(VacantEntry<'a>),
    /// An occupied Entry.
    Occupied(OccupiedEntry<'a>),
}
/// A vacant Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct VacantEntry<'a> {
    vacant: VacantEntryImpl<'a>,
}
/// An occupied Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct OccupiedEntry<'a> {
    occupied: OccupiedEntryImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type VacantEntryImpl<'a> = indexmap::map::VacantEntry<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type OccupiedEntryImpl<'a> = indexmap::map::OccupiedEntry<'a, String, Value>;
impl<'a> Entry<'a> {
    /// Returns a reference to this entry's key.
    pub fn key(&self) -> &String {
        match *self {
            Entry::Vacant(ref e) => e.key(),
            Entry::Occupied(ref e) => e.key(),
        }
    }
    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: Value) -> &'a mut Value {
        match self {
            Entry::Vacant(entry) => entry.insert(default),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut Value
    where
        F: FnOnce() -> Value,
    {
        match self {
            Entry::Vacant(entry) => entry.insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
}
impl<'a> VacantEntry<'a> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the VacantEntry.
    #[inline]
    pub fn key(&self) -> &String {
        self.vacant.key()
    }
    /// Sets the value of the entry with the VacantEntry's key, and returns a
    /// mutable reference to it.
    #[inline]
    pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }
}
impl<'a> OccupiedEntry<'a> {
    /// Gets a reference to the key in the entry.
    #[inline]
    pub fn key(&self) -> &String {
        self.occupied.key()
    }
    /// Gets a reference to the value in the entry.
    #[inline]
    pub fn get(&self) -> &Value {
        self.occupied.get()
    }
    /// Gets a mutable reference to the value in the entry.
    #[inline]
    pub fn get_mut(&mut self) -> &mut Value {
        self.occupied.get_mut()
    }
    /// Converts the entry into a mutable reference to its value.
    #[inline]
    pub fn into_mut(self) -> &'a mut Value {
        self.occupied.into_mut()
    }
    /// Sets the value of the entry with the `OccupiedEntry`'s key, and returns
    /// the entry's old value.
    #[inline]
    pub fn insert(&mut self, value: Value) -> Value {
        self.occupied.insert(value)
    }
    /// Takes the value of the entry out of the map, and returns it.
    #[inline]
    pub fn remove(self) -> Value {
        self.occupied.remove()
    }
}
impl<'a> IntoIterator for &'a Map<String, Value> {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter { iter: self.map.iter() }
    }
}
/// An iterator over a toml::Map's entries.
pub struct Iter<'a> {
    iter: IterImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type IterImpl<'a> = btree_map::Iter<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type IterImpl<'a> = indexmap::map::Iter<'a, String, Value>;
delegate_iterator!((Iter <'a >) => (&'a String, &'a Value));
impl<'a> IntoIterator for &'a mut Map<String, Value> {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = IterMut<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
}
/// A mutable iterator over a toml::Map's entries.
pub struct IterMut<'a> {
    iter: IterMutImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type IterMutImpl<'a> = btree_map::IterMut<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type IterMutImpl<'a> = indexmap::map::IterMut<'a, String, Value>;
delegate_iterator!((IterMut <'a >) => (&'a String, &'a mut Value));
impl IntoIterator for Map<String, Value> {
    type Item = (String, Value);
    type IntoIter = IntoIter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
}
/// An owning iterator over a toml::Map's entries.
pub struct IntoIter {
    iter: IntoIterImpl,
}
#[cfg(not(feature = "preserve_order"))]
type IntoIterImpl = btree_map::IntoIter<String, Value>;
#[cfg(feature = "preserve_order")]
type IntoIterImpl = indexmap::map::IntoIter<String, Value>;
delegate_iterator!((IntoIter) => (String, Value));
/// An iterator over a toml::Map's keys.
pub struct Keys<'a> {
    iter: KeysImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type KeysImpl<'a> = btree_map::Keys<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type KeysImpl<'a> = indexmap::map::Keys<'a, String, Value>;
delegate_iterator!((Keys <'a >) => &'a String);
/// An iterator over a toml::Map's values.
pub struct Values<'a> {
    iter: ValuesImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type ValuesImpl<'a> = btree_map::Values<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesImpl<'a> = indexmap::map::Values<'a, String, Value>;
delegate_iterator!((Values <'a >) => &'a Value);
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    fn sample_map() -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("a".to_string(), Value::from("alpha"));
        map.insert("b".to_string(), Value::from("bravo"));
        map
    }
    #[test]
    fn test_into_iter() {
        let map = sample_map();
        let iter = map.into_iter();
        let collected: Vec<_> = iter.collect();
        assert_eq!(
            collected, vec![("a".to_string(), Value::from("alpha")), ("b".to_string(),
            Value::from("bravo")),]
        );
    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;
    use crate::*;
    #[test]
    fn test_into_iter() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_test_into_iter = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = "alpha";
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = "bravo";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let mut iter = map.into_iter();
        debug_assert_eq!(
            iter.next(), Some(("a".to_string(), Value::String("alpha".to_string())))
        );
        debug_assert_eq!(
            iter.next(), Some(("b".to_string(), Value::String("bravo".to_string())))
        );
        debug_assert_eq!(iter.next(), None);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_test_into_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_36_llm_16_36 {
    use crate::map::Map;
    use std::iter::DoubleEndedIterator;
    use crate::value::Value;
    #[test]
    fn test_intoiter_next_back() {
        let _rug_st_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_intoiter_next_back = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let rug_fuzz_4 = "key3";
        let rug_fuzz_5 = "value3";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut iter = map.into_iter();
        debug_assert_eq!(
            iter.next_back(), Some(("key3".to_string(), Value::from("value3")))
        );
        debug_assert_eq!(
            iter.next_back(), Some(("key2".to_string(), Value::from("value2")))
        );
        debug_assert_eq!(
            iter.next_back(), Some(("key1".to_string(), Value::from("value1")))
        );
        debug_assert_eq!(iter.next_back(), None);
        let _rug_ed_tests_llm_16_36_llm_16_36_rrrruuuugggg_test_intoiter_next_back = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_37 {
    use crate::map::Map;
    use std::iter::ExactSizeIterator;
    #[test]
    fn into_iter_len_test() {
        let _rug_st_tests_llm_16_37_rrrruuuugggg_into_iter_len_test = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let rug_fuzz_4 = "key3";
        let rug_fuzz_5 = "value3";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), rug_fuzz_1.into());
        map.insert(rug_fuzz_2.to_string(), rug_fuzz_3.into());
        map.insert(rug_fuzz_4.to_string(), rug_fuzz_5.into());
        let into_iter = map.into_iter();
        debug_assert_eq!(into_iter.len(), 3);
        let _rug_ed_tests_llm_16_37_rrrruuuugggg_into_iter_len_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_38 {
    use super::*;
    use crate::*;
    use std::iter::Iterator;
    #[test]
    fn test_next_empty_iter() {
        let _rug_st_tests_llm_16_38_rrrruuuugggg_test_next_empty_iter = 0;
        let map: Map<String, Value> = Map::new();
        let mut iter = map.into_iter();
        debug_assert_eq!(iter.next(), None);
        let _rug_ed_tests_llm_16_38_rrrruuuugggg_test_next_empty_iter = 0;
    }
    #[test]
    fn test_next_single_element_iter() {
        let _rug_st_tests_llm_16_38_rrrruuuugggg_test_next_single_element_iter = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let mut iter = map.into_iter();
        debug_assert_eq!(
            iter.next(), Some(("key".to_string(), Value::String("value".to_string())))
        );
        debug_assert_eq!(iter.next(), None);
        let _rug_ed_tests_llm_16_38_rrrruuuugggg_test_next_single_element_iter = 0;
    }
    #[test]
    fn test_next_multiple_elements_iter() {
        let _rug_st_tests_llm_16_38_rrrruuuugggg_test_next_multiple_elements_iter = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let mut iter = map.into_iter();
        let first = iter.next();
        let second = iter.next();
        debug_assert!(first.is_some() && second.is_some());
        debug_assert_ne!(first, second);
        debug_assert_eq!(iter.next(), None);
        let _rug_ed_tests_llm_16_38_rrrruuuugggg_test_next_multiple_elements_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use crate::map::{Map, IntoIter};
    use std::iter::Iterator;
    use crate::Value;
    #[test]
    fn test_size_hint() {
        let _rug_st_tests_llm_16_39_llm_16_39_rrrruuuugggg_test_size_hint = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let map: Map<String, Value> = Map::new();
        let into_iter = map.into_iter();
        let size_hints = into_iter.size_hint();
        debug_assert_eq!(size_hints, (0, Some(0)));
        let mut map_with_values: Map<String, Value> = Map::new();
        map_with_values
            .insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map_with_values
            .insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let into_iter_with_values = map_with_values.into_iter();
        let size_hints_with_values = into_iter_with_values.size_hint();
        debug_assert_eq!(size_hints_with_values, (2, Some(2)));
        let _rug_ed_tests_llm_16_39_llm_16_39_rrrruuuugggg_test_size_hint = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_40_llm_16_40 {
    use crate::map::{Iter, Map, Value};
    #[test]
    fn test_next_back() {
        let _rug_st_tests_llm_16_40_llm_16_40_rrrruuuugggg_test_next_back = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = "alpha";
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = "bravo";
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = "charlie";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let mut iter = map.iter();
        debug_assert_eq!(
            iter.next_back(), Some((& "c".to_string(), & Value::String("charlie"
            .to_string())))
        );
        debug_assert_eq!(
            iter.next_back(), Some((& "b".to_string(), & Value::String("bravo"
            .to_string())))
        );
        debug_assert_eq!(
            iter.next_back(), Some((& "a".to_string(), & Value::String("alpha"
            .to_string())))
        );
        debug_assert_eq!(iter.next_back(), None);
        let _rug_ed_tests_llm_16_40_llm_16_40_rrrruuuugggg_test_next_back = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_41_llm_16_41 {
    use super::*;
    use crate::*;
    #[test]
    fn iter_len() {
        let _rug_st_tests_llm_16_41_llm_16_41_rrrruuuugggg_iter_len = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let empty_map = Map::new();
        let empty_iter = empty_map.iter();
        debug_assert_eq!(empty_iter.len(), 0);
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let iter = map.iter();
        debug_assert_eq!(iter.len(), 2);
        let _rug_ed_tests_llm_16_41_llm_16_41_rrrruuuugggg_iter_len = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_42 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    #[test]
    fn test_iter_next() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_iter_next = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), crate::Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), crate::Value::Integer(rug_fuzz_5));
        let mut iter = map.iter();
        debug_assert_eq!(
            iter.next(), Some((& "a".to_string(), & crate ::Value::Integer(1)))
        );
        debug_assert_eq!(
            iter.next(), Some((& "b".to_string(), & crate ::Value::Integer(2)))
        );
        debug_assert_eq!(
            iter.next(), Some((& "c".to_string(), & crate ::Value::Integer(3)))
        );
        debug_assert_eq!(iter.next(), None);
        let mut iter = map.iter();
        debug_assert_eq!(iter.len(), 3);
        debug_assert_eq!(
            iter.next_back(), Some((& "c".to_string(), & crate ::Value::Integer(3)))
        );
        debug_assert_eq!(
            iter.next_back(), Some((& "b".to_string(), & crate ::Value::Integer(2)))
        );
        debug_assert_eq!(
            iter.next_back(), Some((& "a".to_string(), & crate ::Value::Integer(1)))
        );
        debug_assert_eq!(iter.next_back(), None);
        let mut iter = map.iter();
        debug_assert_eq!(iter.len(), 3);
        debug_assert_eq!(iter.size_hint(), (3, Some(3)));
        debug_assert_eq!(iter.len(), iter.size_hint().0);
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_iter_next = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_43_llm_16_43 {
    use crate::map::Map;
    use crate::value::Value;
    use std::iter::Iterator;
    #[test]
    fn size_hint_returns_correct_values() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_size_hint_returns_correct_values = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::Integer(rug_fuzz_5));
        let iter = map.iter();
        let size_hint = iter.size_hint();
        debug_assert_eq!(size_hint, (3, Some(3)));
        let mut iter = map.iter();
        let _ = iter.next_back();
        let size_hint_after_consume = iter.size_hint();
        debug_assert_eq!(size_hint_after_consume, (2, Some(2)));
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_size_hint_returns_correct_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use super::*;
    use crate::*;
    #[test]
    fn test_next_back() {
        let _rug_st_tests_llm_16_44_rrrruuuugggg_test_next_back = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(
            iter_mut.next_back().map(| (k, _v) | k.clone()), Some("c".to_string())
        );
        debug_assert_eq!(
            iter_mut.next_back().map(| (k, _v) | k.clone()), Some("b".to_string())
        );
        debug_assert_eq!(
            iter_mut.next_back().map(| (k, _v) | k.clone()), Some("a".to_string())
        );
        debug_assert_eq!(iter_mut.next_back(), None);
        let _rug_ed_tests_llm_16_44_rrrruuuugggg_test_next_back = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    #[test]
    fn iter_mut_len() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_iter_mut_len = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(iter_mut.len(), 3);
        iter_mut.next();
        debug_assert_eq!(iter_mut.len(), 2);
        iter_mut.next();
        iter_mut.next();
        debug_assert_eq!(iter_mut.len(), 0);
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_iter_mut_len = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_46_llm_16_46 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    use crate::Value;
    #[test]
    fn iter_mut_next_test() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_next_test = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::Integer(rug_fuzz_5));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(
            iter_mut.next(), Some((& "a".to_string(), & mut Value::Integer(1)))
        );
        debug_assert_eq!(
            iter_mut.next(), Some((& "b".to_string(), & mut Value::Integer(2)))
        );
        debug_assert_eq!(
            iter_mut.next(), Some((& "c".to_string(), & mut Value::Integer(3)))
        );
        debug_assert_eq!(iter_mut.next(), None);
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_next_test = 0;
    }
    #[test]
    fn iter_mut_next_back_test() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_next_back_test = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::Integer(rug_fuzz_5));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(
            iter_mut.next_back(), Some((& "c".to_string(), & mut Value::Integer(3)))
        );
        debug_assert_eq!(
            iter_mut.next_back(), Some((& "b".to_string(), & mut Value::Integer(2)))
        );
        debug_assert_eq!(
            iter_mut.next_back(), Some((& "a".to_string(), & mut Value::Integer(1)))
        );
        debug_assert_eq!(iter_mut.next_back(), None);
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_next_back_test = 0;
    }
    #[test]
    fn iter_mut_len_test() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_len_test = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        let iter_mut = map.iter_mut();
        debug_assert_eq!(iter_mut.len(), 2);
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_len_test = 0;
    }
    #[test]
    fn iter_mut_size_hint_test() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_size_hint_test = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::Integer(rug_fuzz_5));
        let iter_mut = map.iter_mut();
        debug_assert_eq!(iter_mut.size_hint(), (3, Some(3)));
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_iter_mut_size_hint_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use super::*;
    use crate::*;
    #[test]
    fn size_hint_test() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_size_hint_test = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        let mut iter = map.iter_mut();
        let initial_hint = iter.size_hint();
        debug_assert_eq!(initial_hint, (2, Some(2)));
        iter.next();
        let after_one_removal_hint = iter.size_hint();
        debug_assert_eq!(after_one_removal_hint, (1, Some(1)));
        iter.next();
        let after_all_removals_hint = iter.size_hint();
        debug_assert_eq!(after_all_removals_hint, (0, Some(0)));
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_size_hint_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_48_llm_16_48 {
    use crate::map::{Map, Keys};
    use crate::Value;
    #[test]
    fn test_keys_next_back() {
        let _rug_st_tests_llm_16_48_llm_16_48_rrrruuuugggg_test_keys_next_back = 0;
        let rug_fuzz_0 = "z";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "y";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "x";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_owned(), Value::Integer(rug_fuzz_5));
        let mut keys = map.keys();
        debug_assert_eq!(keys.next_back(), Some(& "z".to_owned()));
        debug_assert_eq!(keys.next_back(), Some(& "y".to_owned()));
        debug_assert_eq!(keys.next_back(), Some(& "x".to_owned()));
        debug_assert_eq!(keys.next_back(), None);
        let _rug_ed_tests_llm_16_48_llm_16_48_rrrruuuugggg_test_keys_next_back = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use crate::map::{Map, Keys};
    use std::iter::ExactSizeIterator;
    #[test]
    fn keys_len() {
        let _rug_st_tests_llm_16_49_rrrruuuugggg_keys_len = 0;
        let rug_fuzz_0 = "k1";
        let rug_fuzz_1 = "v1";
        let rug_fuzz_2 = "k2";
        let rug_fuzz_3 = "v2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), crate::Value::String(rug_fuzz_3.to_string()));
        let keys = map.keys();
        debug_assert_eq!(keys.len(), 2);
        let mut keys = map.keys();
        keys.next();
        debug_assert_eq!(keys.len(), 1);
        let mut keys = map.keys();
        keys.next_back();
        debug_assert_eq!(keys.len(), 1);
        let _rug_ed_tests_llm_16_49_rrrruuuugggg_keys_len = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_50_llm_16_50 {
    use super::*;
    use crate::*;
    use crate::*;
    use std::iter::Iterator;
    #[test]
    fn test_keys_next() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_keys_next = 0;
        let rug_fuzz_0 = "first";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "second";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "third";
        let rug_fuzz_5 = 3;
        let mut map = crate::map::Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), crate::Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), crate::Value::Integer(rug_fuzz_5));
        let mut keys = map.keys();
        let first_key = keys.next().map(|s| s.to_string());
        let second_key = keys.next().map(|s| s.to_string());
        let third_key = keys.next().map(|s| s.to_string());
        debug_assert_eq!(first_key, Some("first".to_string()));
        debug_assert_eq!(second_key, Some("second".to_string()));
        debug_assert_eq!(third_key, Some("third".to_string()));
        debug_assert_eq!(keys.next(), None);
        let mut keys = map.keys();
        let third_back_key = keys.next_back().map(|s| s.to_string());
        let second_back_key = keys.next_back().map(|s| s.to_string());
        let first_back_key = keys.next_back().map(|s| s.to_string());
        debug_assert_eq!(third_back_key, Some("third".to_string()));
        debug_assert_eq!(second_back_key, Some("second".to_string()));
        debug_assert_eq!(first_back_key, Some("first".to_string()));
        debug_assert_eq!(keys.next_back(), None);
        let keys = map.keys();
        debug_assert_eq!(keys.len(), 3);
        let mut keys = map.keys();
        debug_assert_eq!(keys.size_hint(), (3, Some(3)));
        keys.next();
        debug_assert_eq!(keys.size_hint(), (2, Some(2)));
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_keys_next = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_54 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn map_clone() {
        let _rug_st_tests_llm_16_54_rrrruuuugggg_map_clone = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let map_clone = map.clone();
        debug_assert_eq!(map, map_clone);
        let _rug_ed_tests_llm_16_54_rrrruuuugggg_map_clone = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_55 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::collections::BTreeMap;
    fn value_map(entries: Vec<(String, Value)>) -> Map<String, Value> {
        entries.into_iter().collect()
    }
    fn value_str(s: &str) -> Value {
        Value::String(s.into())
    }
    #[test]
    fn eq_maps_with_same_entries() {
        let map1 = value_map(
            vec![
                ("key1".into(), value_str("value1")), ("key2".into(),
                value_str("value2"))
            ],
        );
        let map2 = value_map(
            vec![
                ("key1".into(), value_str("value1")), ("key2".into(),
                value_str("value2"))
            ],
        );
        assert!(map1.eq(& map2));
    }
    #[test]
    fn eq_maps_with_different_entries() {
        let map1 = value_map(vec![("key1".into(), value_str("value1"))]);
        let map2 = value_map(vec![("key2".into(), value_str("value2"))]);
        assert!(! map1.eq(& map2));
    }
    #[test]
    fn eq_maps_with_same_entries_different_order() {
        let map1 = value_map(
            vec![
                ("key1".into(), value_str("value1")), ("key2".into(),
                value_str("value2"))
            ],
        );
        let map2 = value_map(
            vec![
                ("key2".into(), value_str("value2")), ("key1".into(),
                value_str("value1"))
            ],
        );
        assert!(map1.eq(& map2));
    }
    #[test]
    fn eq_maps_one_empty() {
        let map1 = value_map(vec![]);
        let map2 = value_map(vec![("key1".into(), value_str("value1"))]);
        assert!(! map1.eq(& map2));
    }
    #[test]
    fn eq_maps_both_empty() {
        let map1 = value_map(vec![]);
        let map2 = value_map(vec![]);
        assert!(map1.eq(& map2));
    }
}
#[cfg(test)]
mod tests_llm_16_56_llm_16_56 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_default_map() {
        let _rug_st_tests_llm_16_56_llm_16_56_rrrruuuugggg_test_default_map = 0;
        let default_map: Map<String, Value> = Map::default();
        debug_assert_eq!(default_map.len(), 0);
        debug_assert!(default_map.is_empty());
        let _rug_ed_tests_llm_16_56_llm_16_56_rrrruuuugggg_test_default_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_57 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_extend_with_no_elements() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_extend_with_no_elements = 0;
        let mut map = Map::new();
        let other: Vec<(String, Value)> = vec![];
        map.extend(other);
        debug_assert_eq!(map.len(), 0);
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_extend_with_no_elements = 0;
    }
    #[test]
    fn test_extend_with_single_element() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_extend_with_single_element = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "key";
        let mut map = Map::new();
        let other = vec![(String::from(rug_fuzz_0), Value::Integer(rug_fuzz_1))];
        map.extend(other);
        debug_assert_eq!(map.len(), 1);
        debug_assert_eq!(map.get(rug_fuzz_2), Some(& Value::Integer(1)));
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_extend_with_single_element = 0;
    }
    #[test]
    fn test_extend_with_multiple_elements() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_extend_with_multiple_elements = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "key1";
        let rug_fuzz_3 = "key2";
        let mut map = Map::new();
        let other = vec![
            (String::from(rug_fuzz_0), Value::Integer(rug_fuzz_1)),
            (String::from("key2"), Value::String(String::from("value")))
        ];
        map.extend(other);
        debug_assert_eq!(map.len(), 2);
        debug_assert_eq!(map.get(rug_fuzz_2), Some(& Value::Integer(1)));
        debug_assert_eq!(
            map.get(rug_fuzz_3), Some(& Value::String(String::from("value")))
        );
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_extend_with_multiple_elements = 0;
    }
    #[test]
    fn test_extend_with_overlapping_keys() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_extend_with_overlapping_keys = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "key1";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "key1";
        let mut map = Map::new();
        map.insert(String::from(rug_fuzz_0), Value::Integer(rug_fuzz_1));
        let other = vec![(String::from(rug_fuzz_2), Value::Integer(rug_fuzz_3))];
        map.extend(other);
        debug_assert_eq!(map.len(), 1);
        debug_assert_eq!(map.get(rug_fuzz_4), Some(& Value::Integer(2)));
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_extend_with_overlapping_keys = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_58 {
    use super::*;
    use crate::*;
    use std::iter::FromIterator;
    use crate::value::Value;
    #[test]
    fn test_from_iter() {
        let _rug_st_tests_llm_16_58_rrrruuuugggg_test_from_iter = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key1";
        let rug_fuzz_3 = "key2";
        let rug_fuzz_4 = "key3";
        let vec_of_tuples = vec![
            (rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string())), ("key2"
            .to_string(), Value::String("value2".to_string()))
        ];
        let map = Map::from_iter(vec_of_tuples);
        debug_assert_eq!(
            map.get(rug_fuzz_2), Some(& Value::String("value1".to_string()))
        );
        debug_assert_eq!(
            map.get(rug_fuzz_3), Some(& Value::String("value2".to_string()))
        );
        debug_assert!(map.get(rug_fuzz_4).is_none());
        let _rug_ed_tests_llm_16_58_rrrruuuugggg_test_from_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_59 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn into_iter_empty_map() {
        let _rug_st_tests_llm_16_59_rrrruuuugggg_into_iter_empty_map = 0;
        let map: Map<String, Value> = Map::new();
        let mut iterator = map.into_iter();
        debug_assert!(iterator.next().is_none());
        let _rug_ed_tests_llm_16_59_rrrruuuugggg_into_iter_empty_map = 0;
    }
    #[test]
    fn into_iter_single_element_map() {
        let _rug_st_tests_llm_16_59_rrrruuuugggg_into_iter_single_element_map = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        let mut iterator = map.into_iter();
        debug_assert_eq!(iterator.next(), Some(("key".to_string(), Value::from(42))));
        debug_assert!(iterator.next().is_none());
        let _rug_ed_tests_llm_16_59_rrrruuuugggg_into_iter_single_element_map = 0;
    }
    #[test]
    fn into_iter_multiple_elements_map() {
        let _rug_st_tests_llm_16_59_rrrruuuugggg_into_iter_multiple_elements_map = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        let mut iterator = map.into_iter();
        let mut items = iterator.collect::<Vec<(String, Value)>>();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        debug_assert_eq!(
            items, vec![("key1".to_string(), Value::from(42)), ("key2".to_string(),
            Value::from("value")),]
        );
        let _rug_ed_tests_llm_16_59_rrrruuuugggg_into_iter_multiple_elements_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_60_llm_16_60 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_index() {
        let _rug_st_tests_llm_16_60_llm_16_60_rrrruuuugggg_test_index = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = "key1";
        let rug_fuzz_5 = "key2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        debug_assert_eq!(& map[rug_fuzz_4], & Value::String("value1".to_string()));
        debug_assert_eq!(& map[rug_fuzz_5], & Value::Integer(42));
        let _rug_ed_tests_llm_16_60_llm_16_60_rrrruuuugggg_test_index = 0;
    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_nonexistent_key() {
        let _rug_st_tests_llm_16_60_llm_16_60_rrrruuuugggg_test_index_nonexistent_key = 0;
        let rug_fuzz_0 = "key3";
        let map = Map::new();
        let _ = map[rug_fuzz_0];
        let _rug_ed_tests_llm_16_60_llm_16_60_rrrruuuugggg_test_index_nonexistent_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_62 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    #[test]
    fn test_values_next_back() {
        let _rug_st_tests_llm_16_62_rrrruuuugggg_test_values_next_back = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.into(), rug_fuzz_1.into());
        map.insert(rug_fuzz_2.into(), rug_fuzz_3.into());
        map.insert(rug_fuzz_4.into(), rug_fuzz_5.into());
        let mut values = map.values();
        debug_assert_eq!(values.next_back(), Some(& 3.into()));
        debug_assert_eq!(values.next_back(), Some(& 2.into()));
        debug_assert_eq!(values.next_back(), Some(& 1.into()));
        debug_assert_eq!(values.next_back(), None);
        let _rug_ed_tests_llm_16_62_rrrruuuugggg_test_values_next_back = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_63_llm_16_63 {
    use crate::map::Map;
    use crate::Value;
    #[test]
    fn values_len() {
        let _rug_st_tests_llm_16_63_llm_16_63_rrrruuuugggg_values_len = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Integer(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::Integer(rug_fuzz_5));
        let values = map.values();
        debug_assert_eq!(values.len(), 3);
        let mut values_iter = map.values();
        values_iter.next();
        debug_assert_eq!(values_iter.len(), 2);
        let _rug_ed_tests_llm_16_63_llm_16_63_rrrruuuugggg_values_len = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_64_llm_16_64 {
    use crate::map::{Map, Values};
    use crate::value::Value;
    use std::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};
    #[test]
    fn test_values_iterator_next() {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_next = 0;
        let rug_fuzz_0 = "one";
        let rug_fuzz_1 = "1";
        let rug_fuzz_2 = "two";
        let rug_fuzz_3 = "2";
        let rug_fuzz_4 = "three";
        let rug_fuzz_5 = "3";
        let rug_fuzz_6 = "1";
        let rug_fuzz_7 = "2";
        let rug_fuzz_8 = "3";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let mut values = map.values();
        debug_assert_eq!(Some(& Value::String(rug_fuzz_6.to_string())), values.next());
        debug_assert_eq!(Some(& Value::String(rug_fuzz_7.to_string())), values.next());
        debug_assert_eq!(Some(& Value::String(rug_fuzz_8.to_string())), values.next());
        debug_assert_eq!(None, values.next());
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_next = 0;
    }
    #[test]
    fn test_values_iterator_next_back() {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_next_back = 0;
        let rug_fuzz_0 = "one";
        let rug_fuzz_1 = "1";
        let rug_fuzz_2 = "two";
        let rug_fuzz_3 = "2";
        let rug_fuzz_4 = "three";
        let rug_fuzz_5 = "3";
        let rug_fuzz_6 = "3";
        let rug_fuzz_7 = "2";
        let rug_fuzz_8 = "1";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let mut values = map.values();
        debug_assert_eq!(
            Some(& Value::String(rug_fuzz_6.to_string())), values.next_back()
        );
        debug_assert_eq!(
            Some(& Value::String(rug_fuzz_7.to_string())), values.next_back()
        );
        debug_assert_eq!(
            Some(& Value::String(rug_fuzz_8.to_string())), values.next_back()
        );
        debug_assert_eq!(None, values.next_back());
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_next_back = 0;
    }
    #[test]
    fn test_values_iterator_exact_size() {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_exact_size = 0;
        let rug_fuzz_0 = "one";
        let rug_fuzz_1 = "1";
        let rug_fuzz_2 = "two";
        let rug_fuzz_3 = "2";
        let rug_fuzz_4 = "three";
        let rug_fuzz_5 = "3";
        let rug_fuzz_6 = 3;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let values = map.values();
        debug_assert_eq!(rug_fuzz_6, values.len());
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_exact_size = 0;
    }
    #[test]
    fn test_values_iterator_size_hint() {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_size_hint = 0;
        let rug_fuzz_0 = "one";
        let rug_fuzz_1 = "1";
        let rug_fuzz_2 = "two";
        let rug_fuzz_3 = "2";
        let rug_fuzz_4 = "three";
        let rug_fuzz_5 = "3";
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let mut values = map.values();
        debug_assert_eq!((rug_fuzz_6, Some(rug_fuzz_7)), values.size_hint());
        values.next();
        debug_assert_eq!((rug_fuzz_8, Some(rug_fuzz_9)), values.size_hint());
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_values_iterator_size_hint = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_278_llm_16_278 {
    use super::*;
    use crate::*;
    use crate::map::{Entry, Map, OccupiedEntry, VacantEntry, Value};
    #[test]
    fn key_for_vacant_entry() {
        let _rug_st_tests_llm_16_278_llm_16_278_rrrruuuugggg_key_for_vacant_entry = 0;
        let rug_fuzz_0 = "test_key";
        let key = rug_fuzz_0.to_string();
        let mut map = Map::new();
        map.insert(key.clone(), Value::String(String::new()));
        map.remove(&key);
        if let Entry::Vacant(vacant_entry) = map.entry(key.clone()) {
            debug_assert_eq!(vacant_entry.key(), & key);
        } else {
            panic!("Entry for key '{}' is not vacant!", & key);
        }
        let _rug_ed_tests_llm_16_278_llm_16_278_rrrruuuugggg_key_for_vacant_entry = 0;
    }
    #[test]
    fn key_for_occupied_entry() {
        let _rug_st_tests_llm_16_278_llm_16_278_rrrruuuugggg_key_for_occupied_entry = 0;
        let rug_fuzz_0 = "test_key";
        let key = rug_fuzz_0.to_string();
        let mut map = Map::new();
        map.insert(key.clone(), Value::String(String::new()));
        if let Entry::Occupied(occupied_entry) = map.entry(key.clone()) {
            debug_assert_eq!(occupied_entry.key(), & key);
        } else {
            panic!("Entry for key '{}' is not occupied!", & key);
        }
        let _rug_ed_tests_llm_16_278_llm_16_278_rrrruuuugggg_key_for_occupied_entry = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_279 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::{Map, Entry};
    #[test]
    fn or_insert_vacant() {
        let _rug_st_tests_llm_16_279_rrrruuuugggg_or_insert_vacant = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::String(rug_fuzz_1.to_string());
        let or_inserted_value = map.entry(key.clone()).or_insert(value.clone());
        debug_assert_eq!(or_inserted_value, & value);
        debug_assert!(map.contains_key(& key));
        debug_assert_eq!(map.get(& key), Some(& value));
        let _rug_ed_tests_llm_16_279_rrrruuuugggg_or_insert_vacant = 0;
    }
    #[test]
    fn or_insert_occupied() {
        let _rug_st_tests_llm_16_279_rrrruuuugggg_or_insert_occupied = 0;
        let rug_fuzz_0 = "key2";
        let rug_fuzz_1 = "value2";
        let rug_fuzz_2 = "new_value";
        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::String(rug_fuzz_1.to_string());
        map.insert(key.clone(), value.clone());
        let value_new = Value::String(rug_fuzz_2.to_string());
        let or_inserted_value = map.entry(key.clone()).or_insert(value_new.clone());
        debug_assert_eq!(or_inserted_value, & value);
        debug_assert_eq!(map.get(& key), Some(& value));
        let _rug_ed_tests_llm_16_279_rrrruuuugggg_or_insert_occupied = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_280 {
    use crate::map::{Map, Entry, Value};
    #[test]
    fn test_or_insert_with_vacant_entry() {
        let _rug_st_tests_llm_16_280_rrrruuuugggg_test_or_insert_with_vacant_entry = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "some value";
        let rug_fuzz_2 = "some value";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        let value_ref = map.entry(key.clone()).or_insert_with(|| value.clone());
        debug_assert_eq!(Value::String(rug_fuzz_2.to_string()), * value_ref);
        debug_assert_eq!(value, * map.get(& key).unwrap());
        let _rug_ed_tests_llm_16_280_rrrruuuugggg_test_or_insert_with_vacant_entry = 0;
    }
    #[test]
    fn test_or_insert_with_occupied_entry() {
        let _rug_st_tests_llm_16_280_rrrruuuugggg_test_or_insert_with_occupied_entry = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "initial value";
        let rug_fuzz_2 = "new value";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        let new_value = Value::String(rug_fuzz_2.to_string());
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(value, * map.get(& key).unwrap());
        let value_ref = map.entry(key.clone()).or_insert_with(|| new_value.clone());
        debug_assert_eq!(value, * value_ref);
        debug_assert_eq!(value, * map.get(& key).unwrap());
        let _rug_ed_tests_llm_16_280_rrrruuuugggg_test_or_insert_with_occupied_entry = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_281 {
    use super::*;
    use crate::*;
    #[test]
    fn clear_empties_the_map() {
        let _rug_st_tests_llm_16_281_rrrruuuugggg_clear_empties_the_map = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        debug_assert!(! map.is_empty());
        map.clear();
        debug_assert!(map.is_empty());
        let _rug_ed_tests_llm_16_281_rrrruuuugggg_clear_empties_the_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_282 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_contains_key() {
        let _rug_st_tests_llm_16_282_rrrruuuugggg_test_contains_key = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "value";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        debug_assert!(! map.contains_key(& key), "Map should not contain key yet.");
        map.insert(key.clone(), value);
        debug_assert!(map.contains_key(& key), "Map should now contain key.");
        let _rug_ed_tests_llm_16_282_rrrruuuugggg_test_contains_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_284_llm_16_284 {
    use crate::map::Map;
    use crate::value::Value;
    use std::borrow::Borrow;
    #[test]
    fn test_get_existing_key() {
        let _rug_st_tests_llm_16_284_llm_16_284_rrrruuuugggg_test_get_existing_key = 0;
        let rug_fuzz_0 = "test_key";
        let rug_fuzz_1 = "test_value";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let val = Value::String(rug_fuzz_1.to_string());
        map.insert(key.clone(), val.clone());
        debug_assert_eq!(map.get(& key as & str).unwrap(), & val);
        let _rug_ed_tests_llm_16_284_llm_16_284_rrrruuuugggg_test_get_existing_key = 0;
    }
    #[test]
    fn test_get_non_existing_key() {
        let _rug_st_tests_llm_16_284_llm_16_284_rrrruuuugggg_test_get_non_existing_key = 0;
        let rug_fuzz_0 = "non_existing_key";
        let map = Map::new();
        let key = rug_fuzz_0;
        debug_assert!(map.get(key as & str).is_none());
        let _rug_ed_tests_llm_16_284_llm_16_284_rrrruuuugggg_test_get_non_existing_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_285 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn get_mut_existing_key() {
        let _rug_st_tests_llm_16_285_rrrruuuugggg_get_mut_existing_key = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "key1";
        let rug_fuzz_3 = 20;
        let rug_fuzz_4 = "key1";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        if let Some(value) = map.get_mut(rug_fuzz_2) {
            *value = Value::from(rug_fuzz_3);
        }
        debug_assert_eq!(map.get(rug_fuzz_4), Some(& Value::from(20)));
        let _rug_ed_tests_llm_16_285_rrrruuuugggg_get_mut_existing_key = 0;
    }
    #[test]
    fn get_mut_non_existing_key() {
        let _rug_st_tests_llm_16_285_rrrruuuugggg_get_mut_non_existing_key = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "key2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        debug_assert!(map.get_mut(rug_fuzz_2).is_none());
        let _rug_ed_tests_llm_16_285_rrrruuuugggg_get_mut_non_existing_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_286_llm_16_286 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_insert_new_key() {
        let _rug_st_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_new_key = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        debug_assert_eq!(map.insert(key.clone(), value.clone()), None);
        debug_assert_eq!(map.get(& key), Some(& value));
        let _rug_ed_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_new_key = 0;
    }
    #[test]
    fn test_insert_existing_key() {
        let _rug_st_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_existing_key = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "value2";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value1 = Value::String(rug_fuzz_1.to_string());
        let value2 = Value::String(rug_fuzz_2.to_string());
        map.insert(key.clone(), value1.clone());
        debug_assert_eq!(map.insert(key.clone(), value2.clone()), Some(value1));
        debug_assert_eq!(map.get(& key), Some(& value2));
        let _rug_ed_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_existing_key = 0;
    }
    #[test]
    fn test_insert_and_overwrite() {
        let _rug_st_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_and_overwrite = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "value2";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value1 = Value::String(rug_fuzz_1.to_string());
        let value2 = Value::String(rug_fuzz_2.to_string());
        map.insert(key.clone(), value1.clone());
        debug_assert_eq!(map.get(& key), Some(& value1));
        debug_assert_eq!(map.insert(key.clone(), value2.clone()), Some(value1));
        debug_assert_eq!(map.get(& key), Some(& value2));
        let _rug_ed_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_and_overwrite = 0;
    }
    #[test]
    fn test_insert_multiple_keys() {
        let _rug_st_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_multiple_keys = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        let key1 = rug_fuzz_0.to_string();
        let value1 = Value::String(rug_fuzz_1.to_string());
        let key2 = rug_fuzz_2.to_string();
        let value2 = Value::String(rug_fuzz_3.to_string());
        debug_assert_eq!(map.insert(key1.clone(), value1.clone()), None);
        debug_assert_eq!(map.insert(key2.clone(), value2.clone()), None);
        debug_assert_eq!(map.get(& key1), Some(& value1));
        debug_assert_eq!(map.get(& key2), Some(& value2));
        let _rug_ed_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_insert_multiple_keys = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_287 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_empty_with_empty_map() {
        let _rug_st_tests_llm_16_287_rrrruuuugggg_test_is_empty_with_empty_map = 0;
        let map: Map<String, Value> = Map::new();
        debug_assert!(map.is_empty());
        let _rug_ed_tests_llm_16_287_rrrruuuugggg_test_is_empty_with_empty_map = 0;
    }
    #[test]
    fn test_is_empty_with_non_empty_map() {
        let _rug_st_tests_llm_16_287_rrrruuuugggg_test_is_empty_with_non_empty_map = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        debug_assert!(! map.is_empty());
        let _rug_ed_tests_llm_16_287_rrrruuuugggg_test_is_empty_with_non_empty_map = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_289_llm_16_289 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_iter_mut() {
        let _rug_st_tests_llm_16_289_llm_16_289_rrrruuuugggg_test_iter_mut = 0;
        let rug_fuzz_0 = "apple";
        let rug_fuzz_1 = "red";
        let rug_fuzz_2 = "banana";
        let rug_fuzz_3 = "yellow";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let mut iter_mut = map.iter_mut();
        if let Some((k, v)) = iter_mut.next() {
            debug_assert_eq!(k, "apple");
            debug_assert_eq!(* v, Value::String("red".to_string()));
        } else {
            panic!("Expected at least one item from iter_mut");
        }
        if let Some((k, v)) = iter_mut.next() {
            debug_assert_eq!(k, "banana");
            debug_assert_eq!(* v, Value::String("yellow".to_string()));
        } else {
            panic!("Expected a second item from iter_mut");
        }
        debug_assert_eq!(iter_mut.next(), None);
        let _rug_ed_tests_llm_16_289_llm_16_289_rrrruuuugggg_test_iter_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_290_llm_16_290 {
    use super::*;
    use crate::*;
    #[test]
    fn test_keys_empty_map() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_empty_map = 0;
        let map: Map<String, Value> = Map::new();
        let mut keys = map.keys();
        debug_assert_eq!(keys.next(), None);
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_empty_map = 0;
    }
    #[test]
    fn test_keys_single_item() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_single_item = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let mut keys = map.keys();
        debug_assert_eq!(keys.next(), Some(& "key1".to_string()));
        debug_assert_eq!(keys.next(), None);
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_single_item = 0;
    }
    #[test]
    fn test_keys_multiple_items() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_multiple_items = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let mut keys = map.keys();
        let mut keys_collected: Vec<&String> = keys.collect();
        keys_collected.sort();
        debug_assert_eq!(
            keys_collected, vec![& "key1".to_string(), & "key2".to_string()]
        );
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_multiple_items = 0;
    }
    #[test]
    fn test_keys_with_removal() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_with_removal = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let rug_fuzz_4 = "key1";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.remove(&rug_fuzz_4.to_string());
        let mut keys = map.keys();
        debug_assert_eq!(keys.next(), Some(& "key2".to_string()));
        debug_assert_eq!(keys.next(), None);
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_with_removal = 0;
    }
    #[test]
    fn test_keys_iterator_length() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_iterator_length = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let keys = map.keys();
        debug_assert_eq!(keys.len(), 2);
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_iterator_length = 0;
    }
    #[test]
    fn test_keys_exact_size_iterator() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_exact_size_iterator = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let keys = map.keys();
        let (lower, upper) = keys.size_hint();
        debug_assert_eq!(lower, 2);
        debug_assert_eq!(upper, Some(2));
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_keys_exact_size_iterator = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_291 {
    use super::*;
    use crate::*;
    #[test]
    fn test_map_len_empty() {
        let _rug_st_tests_llm_16_291_rrrruuuugggg_test_map_len_empty = 0;
        let map: map::Map<String, value::Value> = map::Map::new();
        debug_assert_eq!(map.len(), 0);
        let _rug_ed_tests_llm_16_291_rrrruuuugggg_test_map_len_empty = 0;
    }
    #[test]
    fn test_map_len_non_empty() {
        let _rug_st_tests_llm_16_291_rrrruuuugggg_test_map_len_non_empty = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = "value2";
        let mut map: map::Map<String, value::Value> = map::Map::new();
        map.insert(rug_fuzz_0.to_string(), value::Value::String(rug_fuzz_1.to_string()));
        debug_assert_eq!(map.len(), 1);
        map.insert(rug_fuzz_2.to_string(), value::Value::String(rug_fuzz_3.to_string()));
        debug_assert_eq!(map.len(), 2);
        let _rug_ed_tests_llm_16_291_rrrruuuugggg_test_map_len_non_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_292 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_map_is_empty() {
        let _rug_st_tests_llm_16_292_rrrruuuugggg_test_new_map_is_empty = 0;
        let map: Map<String, Value> = Map::new();
        debug_assert!(map.is_empty());
        let _rug_ed_tests_llm_16_292_rrrruuuugggg_test_new_map_is_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_293 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_remove_existing_key() {
        let _rug_st_tests_llm_16_293_rrrruuuugggg_test_remove_existing_key = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.remove(& key), Some(value));
        debug_assert!(! map.contains_key(& key));
        let _rug_ed_tests_llm_16_293_rrrruuuugggg_test_remove_existing_key = 0;
    }
    #[test]
    fn test_remove_non_existing_key() {
        let _rug_st_tests_llm_16_293_rrrruuuugggg_test_remove_non_existing_key = 0;
        let rug_fuzz_0 = "key";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        debug_assert_eq!(map.remove(& key), None);
        let _rug_ed_tests_llm_16_293_rrrruuuugggg_test_remove_non_existing_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_294 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_values_empty() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_values_empty = 0;
        let map = Map::<String, Value>::new();
        let mut values = map.values();
        debug_assert!(values.next().is_none());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_values_empty = 0;
    }
    #[test]
    fn test_values_single() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_values_single = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let mut map = Map::<String, Value>::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let mut values = map.values();
        debug_assert_eq!(values.next(), Some(& Value::String("value1".to_string())));
        debug_assert!(values.next().is_none());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_values_single = 0;
    }
    #[test]
    fn test_values_multiple() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_values_multiple = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = "value1";
        let rug_fuzz_5 = 42;
        let mut map = Map::<String, Value>::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        let mut values = map.values();
        let values_collected: Vec<_> = values.collect();
        debug_assert_eq!(values_collected.len(), 2);
        debug_assert!(
            values_collected.contains(& & Value::String(rug_fuzz_4.to_string()))
        );
        debug_assert!(values_collected.contains(& & Value::Integer(rug_fuzz_5)));
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_values_multiple = 0;
    }
    #[test]
    fn test_values_order() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_values_order = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "value1";
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = 42;
        let mut map = Map::<String, Value>::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        let values_collected: Vec<_> = map.values().collect();
        debug_assert_eq!(
            values_collected, vec![& Value::String("value1".to_string()), &
            Value::Integer(42),]
        );
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_values_order = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_295 {
    use super::*;
    use crate::*;
    #[test]
    fn with_capacity_is_empty() {
        let _rug_st_tests_llm_16_295_rrrruuuugggg_with_capacity_is_empty = 0;
        let rug_fuzz_0 = 10;
        let map: Map<String, Value> = Map::with_capacity(rug_fuzz_0);
        debug_assert_eq!(map.len(), 0);
        let _rug_ed_tests_llm_16_295_rrrruuuugggg_with_capacity_is_empty = 0;
    }
    #[test]
    fn with_capacity_has_given_capacity() {
        let _rug_st_tests_llm_16_295_rrrruuuugggg_with_capacity_has_given_capacity = 0;
        let rug_fuzz_0 = 10;
        let capacity = rug_fuzz_0;
        let map: Map<String, Value> = Map::with_capacity(capacity);
        let _rug_ed_tests_llm_16_295_rrrruuuugggg_with_capacity_has_given_capacity = 0;
    }
    #[test]
    #[cfg(feature = "preserve_order")]
    fn with_capacity_has_given_capacity_for_indexmap() {
        let _rug_st_tests_llm_16_295_rrrruuuugggg_with_capacity_has_given_capacity_for_indexmap = 0;
        let rug_fuzz_0 = 10;
        let capacity = rug_fuzz_0;
        let map: Map<String, Value> = Map::with_capacity(capacity);
        debug_assert_eq!(map.len(), 0);
        let _rug_ed_tests_llm_16_295_rrrruuuugggg_with_capacity_has_given_capacity_for_indexmap = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_296_llm_16_296 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_get_existing_key() {
        let _rug_st_tests_llm_16_296_llm_16_296_rrrruuuugggg_test_get_existing_key = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 42;
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let val = Value::Integer(rug_fuzz_1);
        map.insert(key.clone(), val.clone());
        debug_assert_eq!(map.get(& key), Some(& val));
        let _rug_ed_tests_llm_16_296_llm_16_296_rrrruuuugggg_test_get_existing_key = 0;
    }
    #[test]
    fn test_get_non_existing_key() {
        let _rug_st_tests_llm_16_296_llm_16_296_rrrruuuugggg_test_get_non_existing_key = 0;
        let rug_fuzz_0 = "key1";
        let map = Map::<String, Value>::new();
        let key = rug_fuzz_0.to_string();
        debug_assert_eq!(map.get(& key), None);
        let _rug_ed_tests_llm_16_296_llm_16_296_rrrruuuugggg_test_get_non_existing_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_297 {
    use crate::map::{Map, OccupiedEntry, Entry};
    use crate::value::Value;
    #[test]
    fn test_occupied_entry_get_mut() {
        let _rug_st_tests_llm_16_297_rrrruuuugggg_test_occupied_entry_get_mut = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "mutated value";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        if let Entry::Occupied(mut entry) = map.entry(rug_fuzz_2.to_owned()) {
            {
                let value: &mut Value = entry.get_mut();
                if let Value::String(v) = value {
                    *v = rug_fuzz_3.to_owned();
                }
            }
            debug_assert_eq!(entry.get(), & Value::String("mutated value".to_owned()));
        } else {
            panic!("Entry::Occupied expected but found a different Entry variant");
        }
        let _rug_ed_tests_llm_16_297_rrrruuuugggg_test_occupied_entry_get_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_298 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_occupied_entry_insert() {
        let _rug_st_tests_llm_16_298_rrrruuuugggg_test_occupied_entry_insert = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "initial";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "new";
        let rug_fuzz_4 = "initial";
        let rug_fuzz_5 = "new";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        if let map::Entry::Occupied(mut oe) = map.entry(rug_fuzz_2.to_string()) {
            let old_value = oe.insert(Value::String(rug_fuzz_3.to_string()));
            debug_assert_eq!(Value::String(rug_fuzz_4.to_string()), old_value);
            debug_assert_eq!(Value::String(rug_fuzz_5.to_string()), * oe.get());
        } else {
            panic!("Entry for key is not occupied");
        }
        let _rug_ed_tests_llm_16_298_rrrruuuugggg_test_occupied_entry_insert = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_299_llm_16_299 {
    use super::*;
    use crate::*;
    #[test]
    fn into_mut_test() {
        let _rug_st_tests_llm_16_299_llm_16_299_rrrruuuugggg_into_mut_test = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "new_value";
        let rug_fuzz_4 = "key";
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let entry = map.entry(rug_fuzz_2.to_string());
        if let Entry::Occupied(mut oe) = entry {
            let value_mut_ref = oe.into_mut();
            *value_mut_ref = Value::String(rug_fuzz_3.to_string());
        }
        let value_ref = map.get(rug_fuzz_4).unwrap();
        debug_assert_eq!(value_ref, & Value::String("new_value".to_string()));
        let _rug_ed_tests_llm_16_299_llm_16_299_rrrruuuugggg_into_mut_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_301_llm_16_301 {
    use super::*;
    use crate::*;
    use crate::map::{Map, Entry};
    use crate::value::Value;
    #[test]
    fn occupied_entry_remove() {
        let _rug_st_tests_llm_16_301_llm_16_301_rrrruuuugggg_occupied_entry_remove = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::Integer(rug_fuzz_1);
        map.insert(key.clone(), value.clone());
        if let Entry::Occupied(occupied_entry) = map.entry(key.clone()) {
            let removed_value = occupied_entry.remove();
            debug_assert_eq!(value, removed_value);
        } else {
            panic!("Expected occupied entry for key {:?}", key);
        }
        debug_assert!(! map.contains_key(& key));
        let _rug_ed_tests_llm_16_301_llm_16_301_rrrruuuugggg_occupied_entry_remove = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_302_llm_16_302 {
    use crate::map::{Map, Entry, Value};
    #[test]
    fn insert_into_vacant_entry() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_insert_into_vacant_entry = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        debug_assert!(map.insert(key.clone(), value.clone()).is_none());
        if let Entry::Vacant(vacant_entry) = map.entry(key.clone()) {
            let inserted_value = vacant_entry.insert(value.clone());
            debug_assert_eq!(inserted_value, & value);
            debug_assert_eq!(map.get(& key), Some(& value));
        } else {
            panic!("entry should be vacant");
        }
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_insert_into_vacant_entry = 0;
    }
}
