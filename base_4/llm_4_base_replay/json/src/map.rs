//! A map of String to serde_json::Value.
//!
//! By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
//! feature of serde_json to use [`IndexMap`] instead.
//!
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`IndexMap`]: https://docs.rs/indexmap/*/indexmap/map/struct.IndexMap.html
use crate::value::Value;
use alloc::string::String;
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::Hash;
use core::iter::{FromIterator, FusedIterator};
#[cfg(feature = "preserve_order")]
use core::mem;
use core::ops;
use serde::de;
#[cfg(not(feature = "preserve_order"))]
use alloc::collections::{btree_map, BTreeMap};
#[cfg(feature = "preserve_order")]
use indexmap::{self, IndexMap};
/// Represents a JSON key/value type.
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
    /// Makes a new empty Map with the given initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            #[cfg(not(feature = "preserve_order"))]
            map: {
                let _ = capacity;
                BTreeMap::new()
            },
            #[cfg(feature = "preserve_order")]
            map: IndexMap::with_capacity(capacity),
        }
    }
    /// Clears the map, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get(key)
    }
    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.contains_key(key)
    }
    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get_mut(key)
    }
    /// Returns the key-value pair matching the given key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    #[cfg(any(feature = "preserve_order", not(no_btreemap_get_key_value)))]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get_key_value(key)
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
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
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        #[cfg(feature = "preserve_order")] return self.map.swap_remove(key);
        #[cfg(not(feature = "preserve_order"))] return self.map.remove(key);
    }
    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        #[cfg(any(feature = "preserve_order", not(no_btreemap_remove_entry)))]
        return self.map.remove_entry(key);
        #[cfg(
            all(
                not(feature = "preserve_order"),
                no_btreemap_remove_entry,
                not(no_btreemap_get_key_value),
            )
        )]
        {
            let (key, _value) = self.map.get_key_value(key)?;
            let key = key.clone();
            let value = self.map.remove::<String>(&key)?;
            Some((key, value))
        }
        #[cfg(
            all(
                not(feature = "preserve_order"),
                no_btreemap_remove_entry,
                no_btreemap_get_key_value,
            )
        )]
        {
            use core::ops::{Bound, RangeBounds};
            struct Key<'a, Q: ?Sized>(&'a Q);
            impl<'a, Q: ?Sized> RangeBounds<Q> for Key<'a, Q> {
                fn start_bound(&self) -> Bound<&Q> {
                    Bound::Included(self.0)
                }
                fn end_bound(&self) -> Bound<&Q> {
                    Bound::Included(self.0)
                }
            }
            let mut range = self.map.range(Key(key));
            let (key, _value) = range.next()?;
            let key = key.clone();
            let value = self.map.remove::<String>(&key)?;
            Some((key, value))
        }
    }
    /// Moves all elements from other into self, leaving other empty.
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        #[cfg(feature = "preserve_order")]
        self.map.extend(mem::replace(&mut other.map, MapImpl::default()));
        #[cfg(not(feature = "preserve_order"))] self.map.append(&mut other.map);
    }
    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    pub fn entry<S>(&mut self, key: S) -> Entry
    where
        S: Into<String>,
    {
        #[cfg(not(feature = "preserve_order"))]
        use alloc::collections::btree_map::Entry as EntryImpl;
        #[cfg(feature = "preserve_order")]
        use indexmap::map::Entry as EntryImpl;
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
    pub fn iter(&self) -> Iter {
        Iter { iter: self.map.iter() }
    }
    /// Gets a mutable iterator over the entries of the map.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
    /// Gets an iterator over the keys of the map.
    #[inline]
    pub fn keys(&self) -> Keys {
        Keys { iter: self.map.keys() }
    }
    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn values(&self) -> Values {
        Values { iter: self.map.values() }
    }
    /// Gets an iterator over mutable values of the map.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut {
        ValuesMut {
            iter: self.map.values_mut(),
        }
    }
    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)`
    /// returns `false`.
    #[cfg(not(no_btreemap_retain))]
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&String, &mut Value) -> bool,
    {
        self.map.retain(f);
    }
}
#[allow(clippy::derivable_impls)]
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
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.map.clone_from(&source.map);
    }
}
impl PartialEq for Map<String, Value> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}
impl Eq for Map<String, Value> {}
/// Access an element of this map. Panics if the given key is not present in the
/// map.
///
/// ```
/// # use serde_json::Value;
/// #
/// # let val = &Value::String("".to_owned());
/// # let _ =
/// match val {
///     Value::String(s) => Some(s.as_str()),
///     Value::Array(arr) => arr[0].as_str(),
///     Value::Object(map) => map["type"].as_str(),
///     _ => None,
/// }
/// # ;
/// ```
impl<'a, Q> ops::Index<&'a Q> for Map<String, Value>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
{
    type Output = Value;
    fn index(&self, index: &Q) -> &Value {
        self.map.index(index)
    }
}
/// Mutably access an element of this map. Panics if the given key is not
/// present in the map.
///
/// ```
/// # use serde_json::json;
/// #
/// # let mut map = serde_json::Map::new();
/// # map.insert("key".to_owned(), serde_json::Value::Null);
/// #
/// map["key"] = json!("value");
/// ```
impl<'a, Q> ops::IndexMut<&'a Q> for Map<String, Value>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Value {
        self.map.get_mut(index).expect("no entry found for key")
    }
}
impl Debug for Map<String, Value> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.map.fmt(formatter)
    }
}
#[cfg(any(feature = "std", feature = "alloc"))]
impl serde::ser::Serialize for Map<String, Value> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = tri!(serializer.serialize_map(Some(self.len())));
        for (k, v) in self {
            tri!(map.serialize_entry(k, v));
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
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }
            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Map::new())
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut values = Map::new();
                while let Some((key, value)) = tri!(visitor.next_entry()) {
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
        #[inline] fn len(& self) -> usize { self.iter.len() } } impl $($generics)*
        FusedIterator for $name $($generics)* {}
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut map = serde_json::Map::new();
    /// assert_eq!(map.entry("serde").key(), &"serde");
    /// ```
    pub fn key(&self) -> &String {
        match self {
            Entry::Vacant(e) => e.key(),
            Entry::Occupied(e) => e.key(),
        }
    }
    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut map = serde_json::Map::new();
    /// map.entry("serde").or_insert(json!(12));
    ///
    /// assert_eq!(map["serde"], 12);
    /// ```
    pub fn or_insert(self, default: Value) -> &'a mut Value {
        match self {
            Entry::Vacant(entry) => entry.insert(default),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut map = serde_json::Map::new();
    /// map.entry("serde").or_insert_with(|| json!("hoho"));
    ///
    /// assert_eq!(map["serde"], "hoho".to_owned());
    /// ```
    pub fn or_insert_with<F>(self, default: F) -> &'a mut Value
    where
        F: FnOnce() -> Value,
    {
        match self {
            Entry::Vacant(entry) => entry.insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut map = serde_json::Map::new();
    /// map.entry("serde")
    ///     .and_modify(|e| *e = json!("rust"))
    ///     .or_insert(json!("cpp"));
    ///
    /// assert_eq!(map["serde"], "cpp");
    ///
    /// map.entry("serde")
    ///     .and_modify(|e| *e = json!("rust"))
    ///     .or_insert(json!("cpp"));
    ///
    /// assert_eq!(map["serde"], "rust");
    /// ```
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Value),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}
impl<'a> VacantEntry<'a> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the VacantEntry.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    ///
    /// match map.entry("serde") {
    ///     Entry::Vacant(vacant) => {
    ///         assert_eq!(vacant.key(), &"serde");
    ///     }
    ///     Entry::Occupied(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn key(&self) -> &String {
        self.vacant.key()
    }
    /// Sets the value of the entry with the VacantEntry's key, and returns a
    /// mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    ///
    /// match map.entry("serde") {
    ///     Entry::Vacant(vacant) => {
    ///         vacant.insert(json!("hoho"));
    ///     }
    ///     Entry::Occupied(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }
}
impl<'a> OccupiedEntry<'a> {
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    /// map.insert("serde".to_owned(), json!(12));
    ///
    /// match map.entry("serde") {
    ///     Entry::Occupied(occupied) => {
    ///         assert_eq!(occupied.key(), &"serde");
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn key(&self) -> &String {
        self.occupied.key()
    }
    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    /// map.insert("serde".to_owned(), json!(12));
    ///
    /// match map.entry("serde") {
    ///     Entry::Occupied(occupied) => {
    ///         assert_eq!(occupied.get(), 12);
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn get(&self) -> &Value {
        self.occupied.get()
    }
    /// Gets a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    /// map.insert("serde".to_owned(), json!([1, 2, 3]));
    ///
    /// match map.entry("serde") {
    ///     Entry::Occupied(mut occupied) => {
    ///         occupied.get_mut().as_array_mut().unwrap().push(json!(4));
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    ///
    /// assert_eq!(map["serde"].as_array().unwrap().len(), 4);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut Value {
        self.occupied.get_mut()
    }
    /// Converts the entry into a mutable reference to its value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    /// map.insert("serde".to_owned(), json!([1, 2, 3]));
    ///
    /// match map.entry("serde") {
    ///     Entry::Occupied(mut occupied) => {
    ///         occupied.into_mut().as_array_mut().unwrap().push(json!(4));
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    ///
    /// assert_eq!(map["serde"].as_array().unwrap().len(), 4);
    /// ```
    #[inline]
    pub fn into_mut(self) -> &'a mut Value {
        self.occupied.into_mut()
    }
    /// Sets the value of the entry with the `OccupiedEntry`'s key, and returns
    /// the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    /// map.insert("serde".to_owned(), json!(12));
    ///
    /// match map.entry("serde") {
    ///     Entry::Occupied(mut occupied) => {
    ///         assert_eq!(occupied.insert(json!(13)), 12);
    ///         assert_eq!(occupied.get(), 13);
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn insert(&mut self, value: Value) -> Value {
        self.occupied.insert(value)
    }
    /// Takes the value of the entry out of the map, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// use serde_json::map::Entry;
    ///
    /// let mut map = serde_json::Map::new();
    /// map.insert("serde".to_owned(), json!(12));
    ///
    /// match map.entry("serde") {
    ///     Entry::Occupied(occupied) => {
    ///         assert_eq!(occupied.remove(), 12);
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn remove(self) -> Value {
        #[cfg(feature = "preserve_order")] return self.occupied.swap_remove();
        #[cfg(not(feature = "preserve_order"))] return self.occupied.remove();
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
/// An iterator over a serde_json::Map's entries.
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
/// A mutable iterator over a serde_json::Map's entries.
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
/// An owning iterator over a serde_json::Map's entries.
pub struct IntoIter {
    iter: IntoIterImpl,
}
#[cfg(not(feature = "preserve_order"))]
type IntoIterImpl = btree_map::IntoIter<String, Value>;
#[cfg(feature = "preserve_order")]
type IntoIterImpl = indexmap::map::IntoIter<String, Value>;
delegate_iterator!((IntoIter) => (String, Value));
/// An iterator over a serde_json::Map's keys.
pub struct Keys<'a> {
    iter: KeysImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type KeysImpl<'a> = btree_map::Keys<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type KeysImpl<'a> = indexmap::map::Keys<'a, String, Value>;
delegate_iterator!((Keys <'a >) => &'a String);
/// An iterator over a serde_json::Map's values.
pub struct Values<'a> {
    iter: ValuesImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type ValuesImpl<'a> = btree_map::Values<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesImpl<'a> = indexmap::map::Values<'a, String, Value>;
delegate_iterator!((Values <'a >) => &'a Value);
/// A mutable iterator over a serde_json::Map's values.
pub struct ValuesMut<'a> {
    iter: ValuesMutImpl<'a>,
}
#[cfg(not(feature = "preserve_order"))]
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesMutImpl<'a> = indexmap::map::ValuesMut<'a, String, Value>;
delegate_iterator!((ValuesMut <'a >) => &'a mut Value);
#[cfg(test)]
mod tests_llm_16_4 {
    use crate::map::Map;
    use crate::value::Value;
    use std::collections::BTreeMap;
    #[test]
    fn test_into_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut expected = BTreeMap::new();
        expected.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        expected.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut map = Map::new();
        map.insert(rug_fuzz_4.to_owned(), Value::String(rug_fuzz_5.to_owned()));
        map.insert(rug_fuzz_6.to_owned(), Value::String(rug_fuzz_7.to_owned()));
        for (key, value) in map.into_iter() {
            debug_assert_eq!(Some(& value), expected.get(& key));
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_95_llm_16_95 {
    use serde::de::{self, Visitor};
    use crate::map::Map;
    use crate::value::Value;
    use crate::Error;
    use std::fmt;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Map::new())
        }
    }
    #[test]
    fn visit_unit_creates_empty_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let visitor = TestVisitor;
        let result: Result<Map<String, Value>, Error> = visitor.visit_unit();
        let map = result.expect(rug_fuzz_0);
        debug_assert!(map.is_empty(), "Expected map to be empty, but it was not");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_137 {
    use crate::map::{IntoIter, Map};
    use crate::value::Value;
    #[test]
    fn test_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(rug_fuzz_1.into()));
        map.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        map.insert(rug_fuzz_4.to_string(), Value::Number(rug_fuzz_5.into()));
        let mut into_iter: IntoIter = map.into_iter();
        debug_assert_eq!(
            into_iter.next_back(), Some(("key3".to_string(), Value::Number(3.into())))
        );
        debug_assert_eq!(
            into_iter.next_back(), Some(("key2".to_string(), Value::Number(2.into())))
        );
        debug_assert_eq!(
            into_iter.next_back(), Some(("key1".to_string(), Value::Number(1.into())))
        );
        debug_assert_eq!(into_iter.next_back(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_138 {
    use crate::Map;
    use crate::value::Value;
    use std::iter::ExactSizeIterator;
    #[test]
    fn test_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let into_iter = map.into_iter();
        debug_assert_eq!(into_iter.len(), 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_139 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let mut into_iter = map.into_iter();
        debug_assert_eq!(
            into_iter.next(), Some(("key1".to_string(), Value::String("value1"
            .to_string())))
        );
        debug_assert_eq!(
            into_iter.next(), Some(("key2".to_string(), Value::String("value2"
            .to_string())))
        );
        debug_assert_eq!(into_iter.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_140 {
    use crate::Map;
    use std::iter::Iterator;
    #[test]
    fn size_hint_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), crate::Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), crate::Value::from(rug_fuzz_5));
        let mut into_iter = map.into_iter();
        debug_assert_eq!(into_iter.size_hint(), (3, Some(3)));
        into_iter.next().unwrap();
        debug_assert_eq!(into_iter.size_hint(), (2, Some(2)));
        into_iter.next().unwrap();
        debug_assert_eq!(into_iter.size_hint(), (1, Some(1)));
        into_iter.next().unwrap();
        debug_assert_eq!(into_iter.size_hint(), (0, Some(0)));
        debug_assert!(into_iter.next().is_none());
        debug_assert_eq!(into_iter.size_hint(), (0, Some(0)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_141 {
    use crate::map::Map;
    use crate::value::Value;
    use std::iter::DoubleEndedIterator;
    #[test]
    fn test_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut iter = map.into_iter();
        debug_assert_eq!(iter.next_back(), Some(("c".to_string(), Value::from(3))));
        debug_assert_eq!(iter.next_back(), Some(("b".to_string(), Value::from(2))));
        debug_assert_eq!(iter.next_back(), Some(("a".to_string(), Value::from(1))));
        debug_assert_eq!(iter.next_back(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_142 {
    use crate::map::Map;
    use crate::Value;
    use std::iter::ExactSizeIterator;
    #[test]
    fn iter_len_empty() {
        let _rug_st_tests_llm_16_142_rrrruuuugggg_iter_len_empty = 0;
        let map: Map<String, Value> = Map::new();
        let iter = map.iter();
        debug_assert_eq!(iter.len(), 0);
        let _rug_ed_tests_llm_16_142_rrrruuuugggg_iter_len_empty = 0;
    }
    #[test]
    fn iter_len_non_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let iter = map.iter();
        debug_assert_eq!(iter.len(), 2);
             }
}
}
}    }
    #[test]
    fn iter_len_after_partial_consumption() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let mut iter = map.iter();
        iter.next();
        debug_assert_eq!(iter.len(), 2);
             }
}
}
}    }
    #[test]
    fn iter_len_after_full_consumption() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let mut iter = map.iter();
        iter.by_ref().for_each(drop);
        debug_assert_eq!(iter.len(), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_143 {
    use crate::map::{Map, Iter};
    #[test]
    fn iter_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, bool, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::Bool(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), crate::Value::Bool(rug_fuzz_3));
        let mut iter = map.iter();
        debug_assert_eq!(
            iter.next(), Some((& "key1".to_string(), & crate ::Value::Bool(true)))
        );
        debug_assert_eq!(
            iter.next(), Some((& "key2".to_string(), & crate ::Value::Bool(false)))
        );
        debug_assert_eq!(iter.next(), None);
        let mut iter_back = map.iter();
        debug_assert_eq!(
            iter_back.next_back(), Some((& "key2".to_string(), & crate
            ::Value::Bool(false)))
        );
        debug_assert_eq!(
            iter_back.next_back(), Some((& "key1".to_string(), & crate
            ::Value::Bool(true)))
        );
        debug_assert_eq!(iter_back.next_back(), None);
        let iter_exact_size = map.iter();
        debug_assert_eq!(iter_exact_size.len(), 2);
        let mut iter_fused = map.iter();
        iter_fused.next();
        iter_fused.next();
        debug_assert_eq!(iter_fused.next(), None);
        debug_assert_eq!(iter_fused.next(), None);
        let mut iter_hint = map.iter();
        debug_assert_eq!(iter_hint.size_hint(), (2, Some(2)));
        iter_hint.next();
        debug_assert_eq!(iter_hint.size_hint(), (1, Some(1)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_144 {
    use crate::map::Map;
    use std::iter::{DoubleEndedIterator, ExactSizeIterator};
    #[test]
    fn size_hint_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::Null);
        map.insert(rug_fuzz_1.to_string(), crate::Value::Null);
        map.insert(rug_fuzz_2.to_string(), crate::Value::Null);
        let mut iter = map.iter();
        debug_assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        debug_assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next_back();
        debug_assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        debug_assert_eq!(iter.size_hint(), (0, Some(0)));
        iter.next_back();
        debug_assert_eq!(iter.size_hint(), (0, Some(0)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_145_llm_16_145 {
    use crate::map::{Map, IterMut};
    use crate::Value;
    use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
    #[test]
    fn test_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut my_map = Map::new();
        my_map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        my_map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        my_map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut iter_mut = my_map.iter_mut();
        debug_assert_eq!(
            iter_mut.next_back().map(| (k, _) | k), Some(& "key3".to_string())
        );
        debug_assert_eq!(
            iter_mut.next_back().map(| (k, _) | k), Some(& "key2".to_string())
        );
        debug_assert_eq!(
            iter_mut.next_back().map(| (k, _) | k), Some(& "key1".to_string())
        );
        debug_assert_eq!(iter_mut.next_back(), None);
        let mut iter_mut = my_map.iter_mut();
        debug_assert_eq!(iter_mut.len(), 3);
        let mut iter_mut = my_map.iter_mut();
        iter_mut.next();
        iter_mut.next();
        iter_mut.next();
        debug_assert_eq!(iter_mut.next(), None);
        debug_assert_eq!(iter_mut.next(), None);
        let mut iter_mut = my_map.iter_mut();
        iter_mut.next();
        let (lower, upper) = iter_mut.size_hint();
        debug_assert_eq!(lower, 2);
        debug_assert_eq!(upper, Some(2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_146 {
    use crate::map::{Map, IterMut};
    use std::iter::ExactSizeIterator;
    #[test]
    fn iter_mut_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), crate::Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), crate::Value::String(rug_fuzz_3.to_string()));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(iter_mut.len(), 2);
        iter_mut.next();
        debug_assert_eq!(iter_mut.len(), 1);
        iter_mut.next();
        debug_assert_eq!(iter_mut.len(), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_147_llm_16_147 {
    use crate::{Map, Value};
    use std::iter::Iterator;
    #[test]
    fn test_iter_mut_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        map.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(
            iter_mut.next().map(| (k, _) | k.clone()), Some("key1".to_string())
        );
        debug_assert_eq!(
            iter_mut.next().map(| (k, _) | k.clone()), Some("key2".to_string())
        );
        debug_assert_eq!(
            iter_mut.next().map(| (k, _) | k.clone()), Some("key3".to_string())
        );
        debug_assert_eq!(iter_mut.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_148 {
    use crate::Map;
    use crate::Value;
    use std::iter::{ExactSizeIterator, FusedIterator};
    #[test]
    fn size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut iter_mut = map.iter_mut();
        let (lower, upper) = iter_mut.size_hint();
        debug_assert_eq!(lower, 3);
        debug_assert_eq!(upper, Some(3));
        iter_mut.next();
        let (lower, upper) = iter_mut.size_hint();
        debug_assert_eq!(lower, 2);
        debug_assert_eq!(upper, Some(2));
        iter_mut.next();
        let (lower, upper) = iter_mut.size_hint();
        debug_assert_eq!(lower, 1);
        debug_assert_eq!(upper, Some(1));
        iter_mut.next();
        let (lower, upper) = iter_mut.size_hint();
        debug_assert_eq!(lower, 0);
        debug_assert_eq!(upper, Some(0));
        debug_assert!(iter_mut.next().is_none());
        let (lower, upper) = iter_mut.size_hint();
        debug_assert_eq!(lower, 0);
        debug_assert_eq!(upper, Some(0));
        let mut iter_mut = map.iter_mut();
        debug_assert_eq!(iter_mut.len(), 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_149_llm_16_149 {
    use crate::Map;
    use crate::Value;
    use std::iter::DoubleEndedIterator;
    #[test]
    fn test_keys_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut my_map = Map::new();
        my_map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        my_map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        my_map.insert(rug_fuzz_4.to_owned(), Value::String(rug_fuzz_5.to_owned()));
        let mut keys = my_map.keys();
        debug_assert_eq!(keys.next_back(), Some(& "key3".to_owned()));
        debug_assert_eq!(keys.next_back(), Some(& "key2".to_owned()));
        debug_assert_eq!(keys.next_back(), Some(& "key1".to_owned()));
        debug_assert_eq!(keys.next_back(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_150 {
    use crate::map::Map;
    use crate::Value;
    #[test]
    fn keys_len_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(rug_fuzz_1.into()));
        map.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        map.insert(rug_fuzz_4.to_string(), Value::Number(rug_fuzz_5.into()));
        let keys = map.keys();
        debug_assert_eq!(keys.len(), 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_151 {
    use crate::map::Map;
    use crate::Value;
    #[test]
    fn test_keys_iterator_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5));
        let mut keys = map.keys();
        debug_assert_eq!(keys.next(), Some(& "a".to_owned()));
        debug_assert_eq!(keys.next(), Some(& "b".to_owned()));
        debug_assert_eq!(keys.next(), Some(& "c".to_owned()));
        debug_assert_eq!(keys.next(), None);
             }
}
}
}    }
    #[test]
    fn test_keys_iterator_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5));
        let mut keys = map.keys();
        debug_assert_eq!(keys.next_back(), Some(& "c".to_owned()));
        debug_assert_eq!(keys.next_back(), Some(& "b".to_owned()));
        debug_assert_eq!(keys.next_back(), Some(& "a".to_owned()));
        debug_assert_eq!(keys.next_back(), None);
             }
}
}
}    }
    #[test]
    fn test_keys_iterator_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5));
        let keys = map.keys();
        debug_assert_eq!(keys.len(), 3);
             }
}
}
}    }
    #[test]
    fn test_keys_iterator_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        let keys = map.keys();
        let (lower, upper) = keys.size_hint();
        debug_assert_eq!(lower, 2);
        debug_assert_eq!(upper, Some(2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_152 {
    use crate::map::Map;
    use crate::value::Value;
    use std::iter::ExactSizeIterator;
    #[test]
    fn keys_iterator_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let keys_iterator = map.keys();
        debug_assert_eq!(keys_iterator.size_hint(), (0, Some(0)));
        map.insert(rug_fuzz_0.to_string(), Value::Null);
        map.insert(rug_fuzz_1.to_string(), Value::Null);
        let mut keys_iterator = map.keys();
        debug_assert_eq!(keys_iterator.size_hint(), (2, Some(2)));
        keys_iterator.next();
        debug_assert_eq!(keys_iterator.size_hint(), (1, Some(1)));
        keys_iterator.next();
        debug_assert_eq!(keys_iterator.size_hint(), (0, Some(0)));
        keys_iterator.next();
        debug_assert_eq!(keys_iterator.size_hint(), (0, Some(0)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_154 {
    use crate::{Map, Value, Serializer};
    use serde::{Serialize, Serializer as SerdeSerializer};
    use std::collections::BTreeMap;
    #[test]
    fn serialize_map_empty() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_serialize_map_empty = 0;
        let map = Map::new();
        let mut buf = Vec::new();
        {
            let mut ser = Serializer::new(&mut buf);
            map.serialize(&mut ser).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        debug_assert_eq!(result, "{}");
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_serialize_map_empty = 0;
    }
    #[test]
    fn serialize_map_with_entries() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        let mut buf = Vec::new();
        {
            let mut ser = Serializer::new(&mut buf);
            map.serialize(&mut ser).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        let expected = crate::to_string(&map).unwrap();
        debug_assert_eq!(result, expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_155 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn clone_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut source_map = Map::new();
        source_map.insert(rug_fuzz_0.into(), Value::String(rug_fuzz_1.into()));
        source_map.insert(rug_fuzz_2.into(), Value::String(rug_fuzz_3.into()));
        let cloned_map = source_map.clone();
        debug_assert_eq!(source_map, cloned_map);
        source_map.insert(rug_fuzz_4.into(), Value::String(rug_fuzz_5.into()));
        debug_assert_ne!(source_map, cloned_map);
        debug_assert!(source_map.contains_key(rug_fuzz_6));
        debug_assert!(! cloned_map.contains_key(rug_fuzz_7));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_156 {
    use crate::value::Value;
    use crate::map::Map;
    use std::string::String;
    #[test]
    fn test_clone_from() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map1 = Map::new();
        map1.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map1.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut map2 = Map::new();
        map2.insert(rug_fuzz_4.to_owned(), Value::String(rug_fuzz_5.to_owned()));
        map2.insert(rug_fuzz_6.to_owned(), Value::String(rug_fuzz_7.to_owned()));
        map2.clone_from(&map1);
        debug_assert_eq!(map2, map1);
        map1.insert(rug_fuzz_8.to_owned(), Value::String(rug_fuzz_9.to_owned()));
        debug_assert_ne!(map2, map1);
        debug_assert_eq!(
            map2.get(rug_fuzz_10).unwrap(), & Value::String("value1".to_owned())
        );
        debug_assert_eq!(
            map2.get(rug_fuzz_11).unwrap(), & Value::String("value2".to_owned())
        );
        debug_assert!(map2.get(rug_fuzz_12).is_none());
        debug_assert!(map2.get(rug_fuzz_13).is_none());
        debug_assert!(map2.get(rug_fuzz_14).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_157 {
    use super::*;
    use crate::*;
    use crate::{Map, Value};
    #[test]
    fn test_map_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(&str, &str, &str, i32, &str, &str, &str, i32, &str, &str, &str, &str, &str, i32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map1 = Map::new();
        let mut map2 = Map::new();
        debug_assert!(map1.eq(& map2));
        map1.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map1.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        debug_assert!(! map1.eq(& map2));
        map2.insert(rug_fuzz_4.to_string(), Value::String(rug_fuzz_5.to_string()));
        map2.insert(rug_fuzz_6.to_string(), Value::Number(rug_fuzz_7.into()));
        debug_assert!(map1.eq(& map2));
        map1.insert(rug_fuzz_8.to_string(), Value::String(rug_fuzz_9.to_string()));
        debug_assert!(! map1.eq(& map2));
        map2.insert(rug_fuzz_10.to_string(), Value::String(rug_fuzz_11.to_string()));
        debug_assert!(! map1.eq(& map2));
        let mut map3 = Map::new();
        map3.insert(rug_fuzz_12.to_string(), Value::Number(rug_fuzz_13.into()));
        map3.insert(rug_fuzz_14.to_string(), Value::String(rug_fuzz_15.to_string()));
        debug_assert!(map1.eq(& map3));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_158 {
    use crate::value::Value;
    use crate::Map;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_default = 0;
        let map: Map<String, Value> = Map::default();
        debug_assert!(map.is_empty());
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_159 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::string::String;
    #[test]
    fn test_extend() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let items = vec![
            (rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string())), ("key2"
            .to_string(), Value::String("value2".to_string()))
        ];
        map.extend(items.clone());
        for (key, value) in items {
            debug_assert_eq!(map.get(& key), Some(& value));
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_160 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::iter::FromIterator;
    #[test]
    fn test_from_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = vec![
            (rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string())), ("key2"
            .to_string(), Value::Number(2.into()))
        ];
        let map: Map<String, Value> = Map::from_iter(data.clone());
        let mut expected_map = Map::new();
        for (k, v) in data {
            expected_map.insert(k, v);
        }
        debug_assert_eq!(map, expected_map);
             }
}
}
}    }
    #[test]
    fn test_from_iter_empty() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_test_from_iter_empty = 0;
        let data: Vec<(String, Value)> = Vec::new();
        let map: Map<String, Value> = Map::from_iter(data);
        debug_assert!(map.is_empty());
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_test_from_iter_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_161 {
    use crate::value::Value;
    use crate::Map;
    use std::iter::FromIterator;
    #[test]
    fn test_into_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key1 = rug_fuzz_0.to_owned();
        let key2 = rug_fuzz_1.to_owned();
        let value1 = Value::String(rug_fuzz_2.to_owned());
        let value2 = Value::String(rug_fuzz_3.to_owned());
        map.insert(key1.clone(), value1.clone());
        map.insert(key2.clone(), value2.clone());
        let iter = map.into_iter();
        let collected: Map<String, Value> = Map::from_iter(iter);
        debug_assert_eq!(collected.get(& key1), Some(& value1));
        debug_assert_eq!(collected.get(& key2), Some(& value2));
        debug_assert_eq!(collected.len(), 2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_162 {
    use crate::map::Map;
    use crate::value::Value;
    use std::ops::Index;
    #[test]
    fn index_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        debug_assert_eq!(map.index(rug_fuzz_4), & Value::String("value1".to_string()));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn index_nonexistent_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        map.index(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    fn index_existing_nested_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let mut nested_map = Map::new();
        nested_map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::Object(nested_map));
        if let Value::Object(inner_map) = map.index(rug_fuzz_3) {
            debug_assert_eq!(
                inner_map.index(rug_fuzz_4), & Value::String("nested_value".to_string())
            );
        } else {
            panic!("Value at 'key1' is not an object.");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_163 {
    use crate::Map;
    use crate::value::Value;
    use std::ops::IndexMut;
    #[test]
    fn index_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        {
            let value = map.index_mut(rug_fuzz_2);
            debug_assert_eq!(* value, Value::String("value".to_string()));
            *value = Value::String(rug_fuzz_3.to_string());
        }
        debug_assert_eq!(map[rug_fuzz_4], Value::String("new_value".to_string()));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn index_mut_nonexistent_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let _ = map.index_mut(rug_fuzz_2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_164 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_values_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(rug_fuzz_1.into()));
        map.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        map.insert(rug_fuzz_4.to_string(), Value::Number(rug_fuzz_5.into()));
        let mut values = map.values();
        debug_assert_eq!(values.next_back(), Some(& Value::Number(3.into())));
        debug_assert_eq!(values.next_back(), Some(& Value::Number(2.into())));
        debug_assert_eq!(values.next_back(), Some(& Value::Number(1.into())));
        debug_assert_eq!(values.next_back(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_165 {
    use crate::map::Map;
    use crate::value::Value;
    use std::iter::ExactSizeIterator;
    #[test]
    fn values_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let values_iter = map.values();
        debug_assert_eq!(values_iter.len(), 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_166 {
    use crate::map::{Map, Values};
    use crate::Value;
    use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
    #[test]
    fn test_values_iterator_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut values = map.values();
        debug_assert_eq!(values.next(), Some(& Value::from(1)));
        debug_assert_eq!(values.next(), Some(& Value::from(2)));
        debug_assert_eq!(values.next(), Some(& Value::from(3)));
        debug_assert_eq!(values.next(), None);
             }
}
}
}    }
    #[test]
    fn test_values_iterator_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut values = map.values();
        debug_assert_eq!(values.next_back(), Some(& Value::from(3)));
        debug_assert_eq!(values.next_back(), Some(& Value::from(2)));
        debug_assert_eq!(values.next_back(), Some(& Value::from(1)));
        debug_assert_eq!(values.next_back(), None);
             }
}
}
}    }
    #[test]
    fn test_values_iterator_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let values = map.values();
        debug_assert_eq!(values.len(), 3);
             }
}
}
}    }
    #[test]
    fn test_values_iterator_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let values = map.values();
        debug_assert_eq!(values.size_hint(), (3, Some(3)));
             }
}
}
}    }
    #[test]
    fn test_values_iterator_fused() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut values = map.values();
        values.by_ref().for_each(drop);
        debug_assert_eq!(values.next(), None);
        debug_assert_eq!(values.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_167 {
    use crate::map::Map;
    use crate::Value;
    use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
    #[test]
    fn test_values_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let values = map.values();
        let size_hint = values.size_hint();
        debug_assert_eq!(size_hint, (3, Some(3)));
        let mut values = map.values();
        values.next();
        let size_hint_after_next = values.size_hint();
        debug_assert_eq!(size_hint_after_next, (2, Some(2)));
        let mut values = map.values();
        values.next_back();
        let size_hint_after_next_back = values.size_hint();
        debug_assert_eq!(size_hint_after_next_back, (2, Some(2)));
        let mut values = map.values();
        while let Some(_) = values.next() {}
        let size_hint_after_exhaust = values.size_hint();
        debug_assert_eq!(size_hint_after_exhaust, (0, Some(0)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_168 {
    use crate::map::Map;
    use crate::value::Value;
    use std::iter::DoubleEndedIterator;
    #[test]
    fn values_mut_next_back() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut json_map = Map::new();
        json_map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        json_map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        json_map.insert(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5));
        let mut values_mut = json_map.values_mut();
        debug_assert_eq!(values_mut.next_back(), Some(& mut Value::from(3)));
        debug_assert_eq!(values_mut.next_back(), Some(& mut Value::from(2)));
        debug_assert_eq!(values_mut.next_back(), Some(& mut Value::from(1)));
        debug_assert_eq!(values_mut.next_back(), None);
             }
}
}
}    }
    #[test]
    fn values_mut_empty_next_back() {
        let _rug_st_tests_llm_16_168_rrrruuuugggg_values_mut_empty_next_back = 0;
        let mut json_map: Map<String, Value> = Map::new();
        let mut values_mut = json_map.values_mut();
        debug_assert_eq!(values_mut.next_back(), None);
        let _rug_ed_tests_llm_16_168_rrrruuuugggg_values_mut_empty_next_back = 0;
    }
    #[test]
    fn values_mut_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut json_map = Map::new();
        json_map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        json_map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        json_map.insert(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5));
        let values_mut = json_map.values_mut();
        debug_assert_eq!(values_mut.len(), 3);
             }
}
}
}    }
    #[test]
    fn values_mut_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut json_map = Map::new();
        json_map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        json_map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        json_map.insert(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5));
        let mut values_mut = json_map.values_mut();
        debug_assert_eq!(values_mut.size_hint(), (3, Some(3)));
        values_mut.next();
        debug_assert_eq!(values_mut.size_hint(), (2, Some(2)));
             }
}
}
}    }
    #[test]
    fn values_mut_fused_trait() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut json_map = Map::new();
        json_map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        let mut values_mut = json_map.values_mut();
        debug_assert_eq!(values_mut.next(), Some(& mut Value::from(1)));
        debug_assert_eq!(values_mut.next(), None);
        debug_assert_eq!(values_mut.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_169 {
    use crate::Map;
    use crate::Value;
    use std::iter::ExactSizeIterator;
    #[test]
    fn values_mut_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, i32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Null);
        map.insert(rug_fuzz_1.to_owned(), Value::from(rug_fuzz_2));
        map.insert(rug_fuzz_3.to_owned(), Value::from(rug_fuzz_4));
        let mut values_mut = map.values_mut();
        debug_assert_eq!(values_mut.len(), 3);
        values_mut.next();
        debug_assert_eq!(values_mut.len(), 2);
        values_mut.next_back();
        debug_assert_eq!(values_mut.len(), 1);
        values_mut.next();
        debug_assert_eq!(values_mut.len(), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_170_llm_16_170 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_values_mut_next() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, i32, &str, i32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        map.insert(rug_fuzz_4.to_string(), Value::from(rug_fuzz_5));
        let mut values_mut = map.values_mut();
        debug_assert_eq!(values_mut.next().map(| v | v.as_i64()), Some(Some(1)));
        debug_assert_eq!(values_mut.next().map(| v | v.as_i64()), Some(Some(2)));
        debug_assert_eq!(values_mut.next().map(| v | v.as_i64()), Some(Some(3)));
        debug_assert_eq!(values_mut.next(), None);
        let mut values_mut = map.values_mut();
        debug_assert_eq!(values_mut.next_back().map(| v | v.as_i64()), Some(Some(3)));
        let values_mut = map.values_mut();
        debug_assert_eq!(values_mut.len(), 3);
        let values_mut = map.values_mut();
        debug_assert_eq!(values_mut.size_hint(), (3, Some(3)));
        let mut values_mut = map.values_mut();
        values_mut.next_back();
        values_mut.next_back();
        values_mut.next_back();
        debug_assert_eq!(values_mut.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_171 {
    use crate::map::Map;
    use crate::Value;
    #[test]
    fn values_mut_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let mut values_mut_iter = map.values_mut();
        debug_assert_eq!(values_mut_iter.size_hint(), (0, Some(0)));
        map.insert(rug_fuzz_0.to_string(), Value::Null);
        map.insert(rug_fuzz_1.to_string(), Value::Bool(rug_fuzz_2));
        let mut values_mut_iter = map.values_mut();
        debug_assert_eq!(values_mut_iter.size_hint(), (2, Some(2)));
        values_mut_iter.next();
        debug_assert_eq!(values_mut_iter.size_hint(), (1, Some(1)));
        values_mut_iter.next();
        debug_assert_eq!(values_mut_iter.size_hint(), (0, Some(0)));
        values_mut_iter.next();
        debug_assert_eq!(values_mut_iter.size_hint(), (0, Some(0)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_501 {
    use crate::{json, Map, Value};
    #[test]
    fn test_and_modify_on_occupied_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        let modified = map.entry(rug_fuzz_2).and_modify(|e| *e = json!(rug_fuzz_3));
        if let crate::map::Entry::Occupied(o) = modified {
            debug_assert_eq!(o.get(), & json!("Rust"));
        } else {
            panic!("Entry should be occupied");
        }
             }
}
}
}    }
    #[test]
    fn test_and_modify_on_vacant_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let unmodified = map.entry(rug_fuzz_0).and_modify(|e| *e = json!(rug_fuzz_1));
        if let crate::map::Entry::Vacant(_v) = unmodified {} else {
            panic!("Entry should be vacant");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_502 {
    use crate::Map;
    use crate::map::Entry;
    #[test]
    fn key_returns_correct_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0;
        map.insert(key.to_owned(), crate::Value::Null);
        match map.entry(key) {
            Entry::Occupied(occupied) => {
                debug_assert_eq!(occupied.key(), key);
            }
            Entry::Vacant(_) => panic!("Entry should be occupied"),
        }
        let vacant_key = rug_fuzz_1;
        match map.entry(vacant_key) {
            Entry::Occupied(_) => panic!("Entry should be vacant"),
            Entry::Vacant(vacant) => {
                debug_assert_eq!(vacant.key(), vacant_key);
            }
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_503 {
    use super::*;
    use crate::*;
    use crate::json;
    use crate::map::Map;
    #[test]
    fn test_or_insert_with_vacant_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.entry(rug_fuzz_0).or_insert(json!(rug_fuzz_1));
        debug_assert_eq!(map[rug_fuzz_2], json!("default_value"));
             }
}
}
}    }
    #[test]
    fn test_or_insert_with_occupied_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        map.entry(rug_fuzz_2).or_insert(json!(rug_fuzz_3));
        debug_assert_eq!(map[rug_fuzz_4], json!("old_value"));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_504 {
    use crate::Map;
    use crate::value::Value;
    #[test]
    fn test_or_insert_with_vacant_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.entry(rug_fuzz_0).or_insert_with(|| Value::String(rug_fuzz_1.to_owned()));
        debug_assert_eq!(map[rug_fuzz_2], Value::String("value1".to_owned()));
             }
}
}
}    }
    #[test]
    fn test_or_insert_with_occupied_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.entry(rug_fuzz_0).or_insert(Value::String(rug_fuzz_1.to_owned()));
        map.entry(rug_fuzz_2).or_insert_with(|| Value::String(rug_fuzz_3.to_owned()));
        debug_assert_eq!(map[rug_fuzz_4], Value::String("initial".to_owned()));
             }
}
}
}    }
    #[test]
    fn test_or_insert_with_updates_nothing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.entry(rug_fuzz_2).or_insert_with(|| Value::String(rug_fuzz_3.to_owned()));
        debug_assert_eq!(map[rug_fuzz_4], Value::String("value3".to_owned()));
             }
}
}
}    }
    #[test]
    fn test_or_insert_with_inserts_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.entry(rug_fuzz_0).or_insert_with(|| Value::String(rug_fuzz_1.to_owned()));
        debug_assert!(map.contains_key(rug_fuzz_2));
        debug_assert_eq!(map[rug_fuzz_3], Value::String("value4".to_owned()));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_505 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_append() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map1 = map::Map::new();
        map1.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        let mut map2 = map::Map::new();
        map2.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        map2.insert(rug_fuzz_4.to_owned(), Value::String(rug_fuzz_5.to_owned()));
        let map2_initial_len = map2.len();
        map1.append(&mut map2);
        debug_assert!(map2.is_empty(), "map2 should be empty after append");
        debug_assert_eq!(
            map1.len(), map2_initial_len + 1,
            "map1 should contain all items from map2 and its original items"
        );
        debug_assert_eq!(
            map1.get(rug_fuzz_6), Some(& Value::String("value1".to_owned()))
        );
        debug_assert_eq!(
            map1.get(rug_fuzz_7), Some(& Value::String("value2".to_owned()))
        );
        debug_assert_eq!(
            map1.get(rug_fuzz_8), Some(& Value::String("value3".to_owned()))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_506 {
    use crate::value::Value;
    use crate::map::Map;
    #[test]
    fn test_clear() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        debug_assert!(! map.is_empty());
        map.clear();
        debug_assert!(map.is_empty());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_507 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn contains_key_existing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        debug_assert!(map.contains_key(rug_fuzz_4));
        debug_assert!(map.contains_key(rug_fuzz_5));
             }
}
}
}    }
    #[test]
    fn contains_key_non_existing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        debug_assert!(! map.contains_key(rug_fuzz_4));
             }
}
}
}    }
    #[test]
    fn contains_key_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map: Map<String, Value> = Map::new();
        debug_assert!(! map.contains_key(rug_fuzz_0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_508_llm_16_508 {
    use crate::{Map, Value, map::Entry};
    #[test]
    fn entry_vacant() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        let key = rug_fuzz_0.to_owned();
        match map.entry(key.clone()) {
            Entry::Vacant(vacant_entry) => debug_assert_eq!(vacant_entry.key(), & key),
            Entry::Occupied(_) => {
                panic!("Expected a vacant entry, but got an occupied one")
            }
        }
        debug_assert!(map.is_empty());
             }
}
}
}    }
    #[test]
    fn entry_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        let key = rug_fuzz_0.to_owned();
        let value = Value::String(rug_fuzz_1.to_owned());
        map.insert(key.clone(), value.clone());
        match map.entry(key.clone()) {
            Entry::Occupied(occupied_entry) => {
                debug_assert_eq!(occupied_entry.key(), & key);
                debug_assert_eq!(occupied_entry.get(), & value);
            }
            Entry::Vacant(_) => {
                panic!("Expected an occupied entry, but got a vacant one")
            }
        }
             }
}
}
}    }
    #[test]
    fn insert_into_vacant() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        let key = rug_fuzz_0.to_owned();
        let value = Value::String(rug_fuzz_1.to_owned());
        if let Entry::Vacant(vacant_entry) = map.entry(key.clone()) {
            vacant_entry.insert(value.clone());
        } else {
            panic!("Expected a vacant entry to insert into");
        }
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn modify_occupied() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        let key = rug_fuzz_0.to_owned();
        let old_value = Value::String(rug_fuzz_1.to_owned());
        let new_value = Value::String(rug_fuzz_2.to_owned());
        map.insert(key.clone(), old_value);
        match map.entry(key.clone()) {
            Entry::Occupied(mut occupied_entry) => {
                if let Value::String(string) = occupied_entry.get_mut() {
                    *string = rug_fuzz_3.to_owned();
                }
                debug_assert_eq!(occupied_entry.get(), & new_value);
            }
            Entry::Vacant(_) => {
                panic!("Expected an occupied entry but found a vacant entry")
            }
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_509 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn test_map_get_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::String(String::from(rug_fuzz_1));
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_get_nonexistent_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map: Map<String, Value> = Map::new();
        let key = String::from(rug_fuzz_0);
        debug_assert_eq!(map.get(& key), None);
             }
}
}
}    }
    #[test]
    fn test_map_get_borrowed_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        map.insert(key.clone(), value.clone());
        let key_borrowed = rug_fuzz_2;
        debug_assert_eq!(map.get(key_borrowed), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_string_key_integer_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::Number(crate::Number::from(rug_fuzz_1));
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_string_key_float_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::Number(crate::Number::from_f64(rug_fuzz_1).unwrap());
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_string_key_null_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::Null;
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_string_key_boolean_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::Bool(rug_fuzz_1);
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_string_key_array_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let value = Value::Array(
            vec![Value::from(rug_fuzz_1), Value::from(2), Value::from(3)],
        );
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
    #[test]
    fn test_map_string_key_object_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = String::from(rug_fuzz_0);
        let inner_map = Map::from_iter(
            vec![
                (String::from(rug_fuzz_1), Value::from(rug_fuzz_2)),
                (String::from("inner_key2"), Value::from(2))
            ],
        );
        let value = Value::Object(inner_map);
        map.insert(key.clone(), value.clone());
        debug_assert_eq!(map.get(& key), Some(& value));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_511 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_get_mut_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0.to_owned();
        let value = Value::Bool(rug_fuzz_1);
        map.insert(key.clone(), value.clone());
        {
            let retrieved_value = map.get_mut(&key).unwrap();
            *retrieved_value = Value::Bool(rug_fuzz_2);
        }
        debug_assert_eq!(map.get(& key), Some(& Value::Bool(false)));
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
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0.to_owned();
        debug_assert!(map.get_mut(& key).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_512 {
    use super::*;
    use crate::*;
    use crate::{Map, Value};
    #[test]
    fn test_insert_adds_new_key_value_pair() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        debug_assert_eq!(map.len(), 0);
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        debug_assert_eq!(map.len(), 1);
        debug_assert!(map.get(rug_fuzz_2).is_some());
        debug_assert_eq!(
            map.get(rug_fuzz_3), Some(& Value::String("test_value".to_owned()))
        );
             }
}
}
}    }
    #[test]
    fn test_insert_overwrites_existing_key_value_pair() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        debug_assert_eq!(
            map.get(rug_fuzz_2), Some(& Value::String("test_value".to_owned()))
        );
        let old = map.insert(rug_fuzz_3.to_owned(), Value::Number(rug_fuzz_4.into()));
        debug_assert!(old.is_some());
        debug_assert_eq!(old, Some(Value::String("test_value".to_owned())));
        debug_assert_eq!(map.get(rug_fuzz_5), Some(& Value::Number(42.into())));
             }
}
}
}    }
    #[test]
    fn test_insert_returns_none_when_key_is_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let old = map.insert(rug_fuzz_0.to_owned(), Value::Null);
        debug_assert!(old.is_none());
             }
}
}
}    }
    #[test]
    fn test_insert_returns_some_when_key_is_present() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        let old = map.insert(rug_fuzz_2.to_owned(), Value::Number(rug_fuzz_3.into()));
        debug_assert!(old.is_some());
        debug_assert_eq!(old, Some(Value::String("test_value".to_owned())));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_513 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::string::String;
    #[test]
    fn map_is_empty_should_return_true_for_new_empty_map() {
        let _rug_st_tests_llm_16_513_rrrruuuugggg_map_is_empty_should_return_true_for_new_empty_map = 0;
        let map: Map<String, Value> = Map::new();
        debug_assert!(map.is_empty(), "Newly created map should be empty.");
        let _rug_ed_tests_llm_16_513_rrrruuuugggg_map_is_empty_should_return_true_for_new_empty_map = 0;
    }
    #[test]
    fn map_is_empty_should_return_false_for_map_with_elements() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Null);
        debug_assert!(! map.is_empty(), "Map with elements should not be empty.");
             }
}
}
}    }
    #[test]
    fn map_is_empty_should_return_true_after_clearing_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Null);
        map.clear();
        debug_assert!(map.is_empty(), "Map should be empty after being cleared.");
             }
}
}
}    }
    #[test]
    fn map_is_empty_should_return_true_for_map_created_with_default() {
        let _rug_st_tests_llm_16_513_rrrruuuugggg_map_is_empty_should_return_true_for_map_created_with_default = 0;
        let map: Map<String, Value> = Map::default();
        debug_assert!(map.is_empty(), "Map created with default should be empty.");
        let _rug_ed_tests_llm_16_513_rrrruuuugggg_map_is_empty_should_return_true_for_map_created_with_default = 0;
    }
    #[test]
    fn map_is_empty_should_return_true_for_map_created_from_empty_iterator() {
        let _rug_st_tests_llm_16_513_rrrruuuugggg_map_is_empty_should_return_true_for_map_created_from_empty_iterator = 0;
        let empty_iter: Vec<(String, Value)> = Vec::new();
        let map: Map<String, Value> = empty_iter.into_iter().collect();
        debug_assert!(
            map.is_empty(), "Map created from empty iterator should be empty."
        );
        let _rug_ed_tests_llm_16_513_rrrruuuugggg_map_is_empty_should_return_true_for_map_created_from_empty_iterator = 0;
    }
    #[test]
    fn map_is_empty_should_return_false_for_map_created_from_nonempty_iterator() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nonempty_iter = vec![(rug_fuzz_0.to_owned(), Value::Null)];
        let map: Map<String, Value> = nonempty_iter.into_iter().collect();
        debug_assert!(
            ! map.is_empty(), "Map created from non-empty iterator should not be empty."
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_514 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::string::String;
    use std::collections::BTreeMap as MapImpl;
    #[test]
    fn iter_empty_map() {
        let _rug_st_tests_llm_16_514_rrrruuuugggg_iter_empty_map = 0;
        let map: Map<String, Value> = Map { map: MapImpl::new() };
        let mut iter = map.iter();
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_514_rrrruuuugggg_iter_empty_map = 0;
    }
    #[test]
    fn iter_non_empty_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut iter = map.iter();
        debug_assert_eq!(
            iter.next(), Some((& "key1".to_owned(), & Value::String("value1"
            .to_owned())))
        );
        debug_assert_eq!(
            iter.next(), Some((& "key2".to_owned(), & Value::String("value2"
            .to_owned())))
        );
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
    #[test]
    fn iter_exact_size() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let iter = map.iter();
        debug_assert_eq!(iter.len(), 2);
             }
}
}
}    }
    #[test]
    fn iter_double_ended() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut iter = map.iter();
        debug_assert_eq!(
            iter.next(), Some((& "key1".to_owned(), & Value::String("value1"
            .to_owned())))
        );
        debug_assert_eq!(
            iter.next_back(), Some((& "key2".to_owned(), & Value::String("value2"
            .to_owned())))
        );
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
    #[test]
    fn iter_fused() {
        let _rug_st_tests_llm_16_514_rrrruuuugggg_iter_fused = 0;
        let map = Map::new();
        let mut iter = map.iter();
        debug_assert!(iter.next().is_none());
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_514_rrrruuuugggg_iter_fused = 0;
    }
    #[test]
    fn iter_size_hint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let iter = map.iter();
        let (lower, upper) = iter.size_hint();
        debug_assert_eq!(lower, 2);
        debug_assert_eq!(upper, Some(2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_515 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn iter_mut_empty() {
        let _rug_st_tests_llm_16_515_rrrruuuugggg_iter_mut_empty = 0;
        let mut map = Map::new();
        let mut iter_mut = map.iter_mut();
        debug_assert!(iter_mut.next().is_none());
        let _rug_ed_tests_llm_16_515_rrrruuuugggg_iter_mut_empty = 0;
    }
    #[test]
    fn iter_mut_key_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut iter_mut = map.iter_mut();
        {
            let (key, value) = iter_mut.next().unwrap();
            debug_assert_eq!(key, "key1");
            debug_assert_eq!(value, & mut Value::String("value1".to_owned()));
        }
        {
            let (key, value) = iter_mut.next().unwrap();
            debug_assert_eq!(key, "key2");
            debug_assert_eq!(value, & mut Value::String("value2".to_owned()));
        }
        debug_assert!(iter_mut.next().is_none());
             }
}
}
}    }
    #[test]
    fn iter_mut_modify_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        for (_, value) in map.iter_mut() {
            *value = Value::String(rug_fuzz_4.to_owned());
        }
        debug_assert_eq!(
            map.get(rug_fuzz_5), Some(& Value::String("modified".to_owned()))
        );
        debug_assert_eq!(
            map.get(rug_fuzz_6), Some(& Value::String("modified".to_owned()))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_517 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_map_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        debug_assert_eq!(map.len(), 0);
        map.insert(rug_fuzz_0.to_string(), Value::Null);
        debug_assert_eq!(map.len(), 1);
        map.insert(rug_fuzz_1.to_string(), Value::Bool(rug_fuzz_2));
        debug_assert_eq!(map.len(), 2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_518 {
    use crate::map::Map;
    use crate::value::Value;
    use std::string::String;
    #[test]
    fn test_map_new() {
        let _rug_st_tests_llm_16_518_rrrruuuugggg_test_map_new = 0;
        let map: Map<String, Value> = Map::new();
        debug_assert!(map.is_empty());
        debug_assert_eq!(map.len(), 0);
        let _rug_ed_tests_llm_16_518_rrrruuuugggg_test_map_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_519 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_remove_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::String(rug_fuzz_1.to_string());
        map.insert(key.clone(), value.clone());
        debug_assert!(map.contains_key(& key));
        let removed_value = map.remove(&key);
        debug_assert_eq!(removed_value, Some(value));
        debug_assert!(! map.contains_key(& key));
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
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0.to_string();
        let nonexistent_key = rug_fuzz_1.to_string();
        let value = Value::String(rug_fuzz_2.to_string());
        map.insert(key.clone(), value.clone());
        debug_assert!(map.contains_key(& key));
        let removed_value = map.remove(&nonexistent_key);
        debug_assert_eq!(removed_value, None);
        debug_assert!(map.contains_key(& key));
             }
}
}
}    }
    #[test]
    fn test_remove_from_empty_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        let key = rug_fuzz_0.to_string();
        let removed_value = map.remove(&key);
        debug_assert_eq!(removed_value, None);
        debug_assert!(! map.contains_key(& key));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_520 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_remove_entry_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0.to_owned();
        let value = Value::String(rug_fuzz_1.to_owned());
        map.insert(key.clone(), value.clone());
        debug_assert!(map.contains_key(& key));
        let removed = map.remove_entry(&key);
        debug_assert!(removed.is_some());
        let (removed_key, removed_value) = removed.unwrap();
        debug_assert_eq!(removed_key, key);
        debug_assert_eq!(removed_value, value);
        debug_assert!(! map.contains_key(& key));
             }
}
}
}    }
    #[test]
    fn test_remove_entry_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        debug_assert!(! map.contains_key(rug_fuzz_2));
        let removed = map.remove_entry(rug_fuzz_3);
        debug_assert!(removed.is_none());
             }
}
}
}    }
    #[test]
    fn test_remove_entry_empty_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: Map<String, Value> = Map::new();
        let removed = map.remove_entry(rug_fuzz_0);
        debug_assert!(removed.is_none());
             }
}
}
}    }
    #[test]
    fn test_remove_entry_with_multiple_keys() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        map.insert(rug_fuzz_4.to_owned(), Value::String(rug_fuzz_5.to_owned()));
        debug_assert_eq!(map.len(), 3);
        let removed = map.remove_entry(rug_fuzz_6);
        debug_assert!(removed.is_some());
        let (removed_key, removed_value) = removed.unwrap();
        debug_assert_eq!(removed_key, "key2");
        debug_assert_eq!(removed_value, Value::String("value2".to_owned()));
        debug_assert_eq!(map.len(), 2);
        debug_assert!(! map.contains_key(rug_fuzz_7));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_521_llm_16_521 {
    use crate::value::Value;
    use crate::map::Map;
    use std::string::String;
    #[test]
    fn test_retain() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(&str, i32, &str, i32, &str, i32, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(rug_fuzz_1.into()));
        map.insert(rug_fuzz_2.to_string(), Value::Number(rug_fuzz_3.into()));
        map.insert(rug_fuzz_4.to_string(), Value::Number(rug_fuzz_5.into()));
        map.retain(|k, _| k >= &rug_fuzz_6.to_string());
        debug_assert_eq!(map.len(), 2);
        debug_assert!(map.contains_key(& rug_fuzz_7.to_string()));
        debug_assert!(map.contains_key(& rug_fuzz_8.to_string()));
        debug_assert!(! map.contains_key(& rug_fuzz_9.to_string()));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_522 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn values_iterator_empty_map() {
        let _rug_st_tests_llm_16_522_rrrruuuugggg_values_iterator_empty_map = 0;
        let map: Map<String, Value> = Map::new();
        let mut values_iter = map.values();
        debug_assert_eq!(values_iter.next(), None);
        let _rug_ed_tests_llm_16_522_rrrruuuugggg_values_iterator_empty_map = 0;
    }
    #[test]
    fn values_iterator_non_empty_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut values_iter = map.values();
        debug_assert_eq!(values_iter.next(), Some(& Value::String("value1".to_owned())));
        debug_assert_eq!(values_iter.next(), Some(& Value::String("value2".to_owned())));
        debug_assert_eq!(values_iter.next(), None);
             }
}
}
}    }
    #[test]
    fn values_iterator_exact_size() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let values_iter = map.values();
        debug_assert_eq!(values_iter.len(), 2);
             }
}
}
}    }
    #[test]
    fn values_iterator_double_ended() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        map.insert(rug_fuzz_2.to_owned(), Value::String(rug_fuzz_3.to_owned()));
        let mut values_iter = map.values();
        debug_assert_eq!(values_iter.next(), Some(& Value::String("value1".to_owned())));
        debug_assert_eq!(
            values_iter.next_back(), Some(& Value::String("value2".to_owned()))
        );
        debug_assert_eq!(values_iter.next_back(), None);
             }
}
}
}    }
    #[test]
    fn values_iterator_fused() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        let mut values_iter = map.values();
        debug_assert_eq!(values_iter.next(), Some(& Value::String("value1".to_owned())));
        debug_assert_eq!(values_iter.next(), None);
        debug_assert_eq!(values_iter.next(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_523 {
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_values_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        map.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        {
            let mut values_mut = map.values_mut();
            let value1 = values_mut.next().unwrap();
            *value1 = Value::String(rug_fuzz_4.to_string());
            let value2 = values_mut.next().unwrap();
            *value2 = Value::String(rug_fuzz_5.to_string());
            debug_assert!(values_mut.next().is_none());
        }
        debug_assert_eq!(map[rug_fuzz_6], Value::String("modified1".to_string()));
        debug_assert_eq!(map[rug_fuzz_7], Value::String("modified2".to_string()));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_524 {
    use super::*;
    use crate::*;
    use crate::map::Map;
    use crate::value::Value;
    use std::string::String;
    #[test]
    fn test_with_capacity() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let capacity = rug_fuzz_0;
        let map: Map<String, Value> = Map::with_capacity(capacity);
        #[cfg(not(feature = "preserve_order"))]
        {
            debug_assert!(map.is_empty());
            debug_assert_eq!(map.len(), 0);
        }
        #[cfg(feature = "preserve_order")]
        {
            debug_assert!(map.is_empty());
            debug_assert_eq!(map.len(), 0);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_525 {
    use crate::map::{Entry, Map, OccupiedEntry};
    use crate::value::Value;
    use crate::json;
    #[test]
    fn test_get_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        if let Entry::Occupied(occupied) = map.entry(rug_fuzz_2) {
            debug_assert_eq!(* occupied.get(), json!(42));
        } else {
            panic!("Expected entry to be occupied.");
        }
             }
}
}
}    }
    #[test]
    fn test_get_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        if let Entry::Occupied(_) = map.entry(rug_fuzz_2) {
            panic!("Expected entry to be vacant.");
        }
             }
}
}
}    }
    #[test]
    fn test_get_with_different_value_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        if let Entry::Occupied(occupied) = map.entry(rug_fuzz_2) {
            debug_assert_eq!(* occupied.get(), json!("a string"));
        } else {
            panic!("Expected entry to be occupied.");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_526 {
    use super::*;
    use crate::*;
    use crate::{json, map::Entry, Value, Map};
    #[test]
    fn test_get_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, i32, i32, i32, &str, i32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]));
        match map.entry(rug_fuzz_4) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().as_array_mut().unwrap().push(json!(rug_fuzz_5));
            }
            Entry::Vacant(_) => panic!("Expected occupied entry"),
        }
        debug_assert_eq!(map[rug_fuzz_6].as_array().unwrap().len(), 4);
        debug_assert_eq!(
            map[rug_fuzz_7].as_array().unwrap(), & vec![json!(1), json!(2), json!(3),
            json!(4)]
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_527 {
    use crate::json;
    use crate::map::{Entry, Map};
    #[test]
    fn test_insert_replaces_and_returns_old_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        match map.entry(rug_fuzz_2) {
            Entry::Occupied(mut occupied) => {
                let old_value = occupied.insert(json!(rug_fuzz_3));
                debug_assert_eq!(old_value, json!("old_value"));
                debug_assert_eq!(occupied.get(), & json!("new_value"));
            }
            Entry::Vacant(_) => unreachable!(),
        }
             }
}
}
}    }
    #[test]
    fn test_insert_on_vacant_entry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        match map.entry(rug_fuzz_0) {
            Entry::Occupied(_) => unreachable!(),
            Entry::Vacant(mut vacant) => {
                vacant.insert(json!(rug_fuzz_1));
                debug_assert_eq!(map[rug_fuzz_2], json!("new_value"));
            }
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_528_llm_16_528 {
    use crate::json;
    use crate::map::{Map, Entry};
    #[test]
    fn test_into_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, i32, i32, i32, &str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]));
        match map.entry(rug_fuzz_4) {
            Entry::Occupied(mut occupied) => {
                occupied.into_mut().as_array_mut().unwrap().push(json!(rug_fuzz_5));
            }
            Entry::Vacant(_) => unreachable!(),
        }
        debug_assert_eq!(map[rug_fuzz_6], json!([1, 2, 3, 4]));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_529 {
    use crate::map::{Entry, Map};
    use crate::Value;
    #[test]
    fn test_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(
            rug_fuzz_0.to_owned(),
            Value::Number(crate::Number::from(rug_fuzz_1)),
        );
        match map.entry(rug_fuzz_2) {
            Entry::Occupied(occupied) => {
                debug_assert_eq!(occupied.key(), "serde");
            }
            Entry::Vacant(_) => {
                panic!("Expected entry to be occupied");
            }
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_530 {
    use crate::map::{Entry, Map};
    use crate::Value;
    #[test]
    fn test_remove_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0;
        let val = Value::Number(rug_fuzz_1.into());
        map.insert(key.to_string(), val.clone());
        debug_assert!(map.contains_key(key));
        match map.entry(key.to_string()) {
            Entry::Occupied(o) => {
                debug_assert_eq!(o.remove(), val);
            }
            Entry::Vacant(_) => {
                panic!("Expected occupied entry");
            }
        }
        debug_assert!(! map.contains_key(key));
             }
}
}
}    }
    #[test]
    fn test_remove_non_existing_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        let key = rug_fuzz_0;
        debug_assert!(! map.contains_key(key));
        let vacant_result = map.entry(key.to_string());
        match vacant_result {
            Entry::Occupied(_) => {
                panic!("Expected vacant entry");
            }
            Entry::Vacant(_) => {}
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_531 {
    use super::*;
    use crate::*;
    use crate::{json, map::{Map, Entry}};
    #[test]
    fn test_vacant_entry_insert() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        match map.entry(String::from(rug_fuzz_0)) {
            Entry::Vacant(vacant) => {
                let v = vacant.insert(json!(rug_fuzz_1));
                debug_assert_eq!(* v, json!("value"));
            }
            Entry::Occupied(_) => unreachable!(),
        }
        debug_assert_eq!(map.get(rug_fuzz_2), Some(& json!("value")));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_532 {
    use crate::map::{Entry, Map};
    use crate::value::Value;
    #[test]
    fn test_vacant_entry_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        match map.entry(rug_fuzz_0) {
            Entry::Vacant(vacant) => {
                debug_assert_eq!(vacant.key(), "serde");
            }
            Entry::Occupied(_) => panic!("Expected Entry::Vacant, found Entry::Occupied"),
        }
             }
}
}
}    }
}
