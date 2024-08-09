use super::Value;
use crate::map::Map;
use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::{self, Display};
use core::ops;
/// A type that can be used to index into a `serde_json::Value`.
///
/// The [`get`] and [`get_mut`] methods of `Value` accept any type that
/// implements `Index`, as does the [square-bracket indexing operator]. This
/// trait is implemented for strings which are used as the index into a JSON
/// map, and for `usize` which is used as the index into a JSON array.
///
/// [`get`]: ../enum.Value.html#method.get
/// [`get_mut`]: ../enum.Value.html#method.get_mut
/// [square-bracket indexing operator]: ../enum.Value.html#impl-Index%3CI%3E
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json`.
///
/// # Examples
///
/// ```
/// # use serde_json::json;
/// #
/// let data = json!({ "inner": [1, 2, 3] });
///
/// // Data is a JSON map so it can be indexed with a string.
/// let inner = &data["inner"];
///
/// // Inner is a JSON array so it can be indexed with an integer.
/// let first = &inner[0];
///
/// assert_eq!(first, 1);
/// ```
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
    /// Panic if array index out of bounds. If key is not already in the object,
    /// insert it with a value of null. Panic if Value is a type that cannot be
    /// indexed into, except if Value is null then it can be treated as an empty
    /// object.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}
impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        match v {
            Value::Array(vec) => {
                let len = vec.len();
                vec.get_mut(*self)
                    .unwrap_or_else(|| {
                        panic!(
                            "cannot access index {} of JSON array of length {}", self,
                            len
                        )
                    })
            }
            _ => panic!("cannot access index {} of JSON {}", self, Type(v)),
        }
    }
}
impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Object(map) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Object(map) => map.get_mut(self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = v {
            *v = Value::Object(Map::new());
        }
        match v {
            Value::Object(map) => map.entry(self.to_owned()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in JSON {}", self, Type(v)),
        }
    }
}
impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }
}
impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for alloc::string::String {}
    impl<'a, T> Sealed for &'a T
    where
        T: ?Sized + Sealed,
    {}
}
/// Used in panic messages.
struct Type<'a>(&'a Value);
impl<'a> Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Value::Null => formatter.write_str("null"),
            Value::Bool(_) => formatter.write_str("boolean"),
            Value::Number(_) => formatter.write_str("number"),
            Value::String(_) => formatter.write_str("string"),
            Value::Array(_) => formatter.write_str("array"),
            Value::Object(_) => formatter.write_str("object"),
        }
    }
}
impl<I> ops::Index<I> for Value
where
    I: Index,
{
    type Output = Value;
    /// Index into a `serde_json::Value` using the syntax `value[0]` or
    /// `value["k"]`.
    ///
    /// Returns `Value::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is an array
    /// or a number. Also returns `Value::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the array.
    ///
    /// For retrieving deeply nested values, you should have a look at the
    /// `Value::pointer` method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let data = json!({
    ///     "x": {
    ///         "y": ["z", "zz"]
    ///     }
    /// });
    ///
    /// assert_eq!(data["x"]["y"], json!(["z", "zz"]));
    /// assert_eq!(data["x"]["y"][0], json!("z"));
    ///
    /// assert_eq!(data["a"], json!(null)); // returns null for undefined values
    /// assert_eq!(data["a"]["b"], json!(null)); // does not panic
    /// ```
    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}
impl<I> ops::IndexMut<I> for Value
where
    I: Index,
{
    /// Write into a `serde_json::Value` using the syntax `value[0] = ...` or
    /// `value["k"] = ...`.
    ///
    /// If the index is a number, the value must be an array of length bigger
    /// than the index. Indexing into a value that is not an array or an array
    /// that is too small will panic.
    ///
    /// If the index is a string, the value must be an object or null which is
    /// treated like an empty object. If the key is not already present in the
    /// object, it will be inserted with a value of null. Indexing into a value
    /// that is neither an object nor null will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut data = json!({ "x": 0 });
    ///
    /// // replace an existing key
    /// data["x"] = json!(1);
    ///
    /// // insert a new key
    /// data["y"] = json!([false, false, false]);
    ///
    /// // replace an array value
    /// data["y"][0] = json!(true);
    ///
    /// // inserted a deeply nested key
    /// data["a"]["b"]["c"]["d"] = json!(true);
    ///
    /// println!("{}", data);
    /// ```
    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_index_into_null() {
        let index = "";
        let value = Value::Null;
        let result = index_into(&index, &value);
        assert_eq!(result, None);
    }
    #[test]
    fn test_index_into_bool() {
        let index = "";
        let value = Value::Bool(true);
        let result = index_into(&index, &value);
        assert_eq!(result, None);
    }
    #[test]
    fn test_index_into_number() {
        let index = "";
        let value = Value::Number(Number::from(42));
        let result = index_into(&index, &value);
        assert_eq!(result, None);
    }
    #[test]
    fn test_index_into_string() {
        let index = "example";
        let value = Value::String("example".to_string());
        let result = index_into(index, &value);
        assert_eq!(result, Some(& value));
    }
    #[test]
    fn test_index_into_array() {
        let index = "1";
        let value = Value::Array(
            vec![Value::String("a".to_string()), Value::String("b".to_string())],
        );
        let result = index_into(index, &value);
        assert_eq!(result, None);
    }
    #[test]
    fn test_index_into_object() {
        let index = "b";
        let mut map = Map::new();
        map.insert("a".to_string(), Value::String("A".to_string()));
        map.insert("b".to_string(), Value::String("B".to_string()));
        let value = Value::Object(map);
        let result = index_into(index, &value);
        assert_eq!(result, Some(& Value::String("B".to_string())));
    }
    fn index_into<'a, 'v>(index: &'a str, v: &'v Value) -> Option<&'v Value> {
        index[..].index_into(v)
    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use crate::value::{Index, Value};
    use crate::map::Map;
    use std::iter::FromIterator;
    #[test]
    fn index_into_mut_with_string_key_in_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut val = Value::Object(Map::new());
        val.as_object_mut()
            .unwrap()
            .insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let index = rug_fuzz_2;
        let result = index.index_into_mut(&mut val);
        debug_assert!(result.is_some());
        debug_assert_eq!(result.unwrap(), & mut Value::String("value".to_string()));
             }
});    }
    #[test]
    fn index_into_mut_with_nonexistent_string_key_in_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut val = Value::Object(Map::new());
        val.as_object_mut()
            .unwrap()
            .insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let index = rug_fuzz_2;
        let result = index.index_into_mut(&mut val);
        debug_assert!(result.is_none());
             }
});    }
    #[test]
    fn index_into_mut_with_string_key_in_non_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut val = Value::String(rug_fuzz_0.to_string());
        let index = rug_fuzz_1;
        let result = index.index_into_mut(&mut val);
        debug_assert!(result.is_none());
             }
});    }
    #[test]
    fn index_into_mut_with_string_key_in_nested_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut val = Value::Object(Map::new());
        let mut inner = Map::new();
        inner.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        val.as_object_mut()
            .unwrap()
            .insert(rug_fuzz_2.to_string(), Value::Object(inner));
        let index = rug_fuzz_3;
        let result = index.index_into_mut(&mut val);
        debug_assert!(result.is_some());
        debug_assert_eq!(
            result.unwrap(), & mut Value::Object(Map::from_iter(vec![("inner_key"
            .to_string(), Value::String("inner_value".to_string()))]))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use crate::value::{Index, Value};
    #[test]
    fn test_index_or_insert_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let key = rug_fuzz_0.to_string();
        let mut object = Value::Object(crate::Map::new());
        let value = key.index_or_insert(&mut object);
        *value = Value::String(rug_fuzz_1.to_string());
        debug_assert_eq!(object[rug_fuzz_2], Value::String("new_value".to_string()));
        let value = key.index_or_insert(&mut object);
        debug_assert_eq!(value, & Value::String("new_value".to_string()));
             }
});    }
    #[test]
    fn test_index_or_insert_str_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let key = rug_fuzz_0;
        let mut object = Value::Object(crate::Map::new());
        object
            .as_object_mut()
            .unwrap()
            .insert(key.to_string(), Value::String(rug_fuzz_1.to_string()));
        let value = key.index_or_insert(&mut object);
        debug_assert_eq!(value, & Value::String("existing_value".to_string()));
        debug_assert_eq!(
            object[rug_fuzz_2], Value::String("existing_value".to_string())
        );
             }
});    }
    #[test]
    fn test_index_or_insert_nested_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let key1 = rug_fuzz_0.to_string();
        let key2 = rug_fuzz_1.to_string();
        let mut object = Value::Object(crate::Map::new());
        let nested = key1.index_or_insert(&mut object);
        let value = key2.index_or_insert(nested);
        *value = Value::String(rug_fuzz_2.to_string());
        debug_assert_eq!(
            object[rug_fuzz_3] [rug_fuzz_4], Value::String("nested_value".to_string())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_290_llm_16_290 {
    use crate::{map::Map, value::{Value, Index}};
    #[test]
    fn test_index_into_existing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let value = Value::Object(map);
        let key = rug_fuzz_2.to_string();
        let result = <String as Index>::index_into(&key, &value);
        debug_assert_eq!(result, Some(& Value::String("value".to_string())));
             }
});    }
    #[test]
    fn test_index_into_non_existing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        let value = Value::Object(map);
        let key = rug_fuzz_0.to_string();
        let result = <String as Index>::index_into(&key, &value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_wrong_type() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![Value::String(rug_fuzz_0.to_string())]);
        let key = rug_fuzz_1.to_string();
        let result = <String as Index>::index_into(&key, &value);
        debug_assert_eq!(result, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_291 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn index_into_mut_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut json = json!(
            { "name" : "John Doe", "age" : 30, "phones" : ["+44 1234567", "+44 2345678"]
            }
        );
        let name = rug_fuzz_0.to_string();
        debug_assert!(name.index_into_mut(& mut json).is_some());
        debug_assert_eq!(
            name.index_into_mut(& mut json).unwrap(), & mut Value::String("John Doe"
            .to_string())
        );
        let age = rug_fuzz_1.to_string();
        debug_assert!(age.index_into_mut(& mut json).is_some());
        debug_assert_eq!(
            age.index_into_mut(& mut json).unwrap(), & mut Value::Number(30.into())
        );
        let phones = rug_fuzz_2.to_string();
        debug_assert!(phones.index_into_mut(& mut json).is_some());
        debug_assert_eq!(
            phones.index_into_mut(& mut json).unwrap(), & mut
            Value::Array(vec![Value::String("+44 1234567".to_string()),
            Value::String("+44 2345678".to_string())])
        );
        let non_existing = rug_fuzz_3.to_string();
        debug_assert!(non_existing.index_into_mut(& mut json).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_292 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn test_index_or_insert_existing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Object(Map::new());
        value
            .as_object_mut()
            .unwrap()
            .insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let string_key = rug_fuzz_2.to_string();
        let result = string_key.index_or_insert(&mut value);
        debug_assert_eq!(result, & mut Value::String("value".to_string()));
             }
});    }
    #[test]
    fn test_index_or_insert_new_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Object(Map::new());
        let string_key = rug_fuzz_0.to_string();
        let result = string_key.index_or_insert(&mut value);
        debug_assert_eq!(result, & mut Value::Null);
        debug_assert!(value.as_object().unwrap().contains_key(rug_fuzz_1));
             }
});    }
    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_or_insert_non_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Array(vec![]);
        let string_key = rug_fuzz_0.to_string();
        let _ = string_key.index_or_insert(&mut value);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_293 {
    use super::*;
    use crate::*;
    use crate::value::{Value, Index};
    #[test]
    fn test_index_into_with_object_containing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let value = Value::Object(map);
        let index = rug_fuzz_2;
        debug_assert_eq!(
            index.index_into(& value), Some(& Value::String("value".to_string()))
        );
             }
});    }
    #[test]
    fn test_index_into_with_object_not_containing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map = Map::new();
        let value = Value::Object(map);
        let index = rug_fuzz_0;
        debug_assert_eq!(index.index_into(& value), None);
             }
});    }
    #[test]
    fn test_index_into_with_non_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![Value::String(rug_fuzz_0.to_string())]);
        let index = rug_fuzz_1;
        debug_assert_eq!(index.index_into(& value), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_294 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn index_into_mut_existing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Object(Map::new());
        value
            .as_object_mut()
            .unwrap()
            .insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        let key = rug_fuzz_2;
        let result = key.index_into_mut(&mut value);
        debug_assert_eq!(result, Some(& mut Value::String("value".to_owned())));
             }
});    }
    #[test]
    fn index_into_mut_non_existing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Object(Map::new());
        value
            .as_object_mut()
            .unwrap()
            .insert(rug_fuzz_0.to_owned(), Value::String(rug_fuzz_1.to_owned()));
        let key = rug_fuzz_2;
        let result = key.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn index_into_mut_wrong_type() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = Value::Array(vec![Value::String(rug_fuzz_0.to_owned())]);
        let key = rug_fuzz_1;
        let result = key.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_297 {
    use crate::Value;
    use crate::value::index::Index;
    #[test]
    fn test_index_into_mut_with_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, Some(& mut crate ::json!(10)));
             }
});    }
    #[test]
    fn test_index_into_mut_with_null() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!(null);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!({ "key" : "value" });
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_empty_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!([]);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_out_of_bounds_index() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!(rug_fuzz_1);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_number() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!(rug_fuzz_1);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!(rug_fuzz_1);
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_index_into_mut_with_nested_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let index = rug_fuzz_0;
        let mut value = crate::json!(
            [[rug_fuzz_1, rug_fuzz_2], [rug_fuzz_3, rug_fuzz_4]]
        );
        let result = index.index_into_mut(&mut value);
        debug_assert_eq!(result, Some(& mut crate ::json!([10, 20])));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_729 {
    use crate::json;
    use crate::Value;
    #[test]
    fn test_index_with_string_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, usize, &str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!(
            { "name" : "John Doe", "age" : 30, "phones" : ["+44 1234567", "+44 2345678"]
            }
        );
        debug_assert_eq!(data[rug_fuzz_0], "John Doe");
        debug_assert_eq!(data[rug_fuzz_1], 30);
        debug_assert_eq!(data[rug_fuzz_2] [rug_fuzz_3], "+44 1234567");
        debug_assert_eq!(data[rug_fuzz_4] [rug_fuzz_5], "+44 2345678");
        debug_assert_eq!(data[rug_fuzz_6], Value::Null);
             }
});    }
    #[test]
    fn test_index_with_array_index() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert_eq!(data[rug_fuzz_3], "zero");
        debug_assert_eq!(data[rug_fuzz_4], "one");
        debug_assert_eq!(data[rug_fuzz_5], "two");
        debug_assert_eq!(data[rug_fuzz_6], Value::Null);
             }
});    }
    #[test]
    fn test_index_with_nested_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!(
            { "user" : { "name" : { "first" : "John", "last" : "Doe" }, "age" : 30 } }
        );
        debug_assert_eq!(data[rug_fuzz_0] [rug_fuzz_1] [rug_fuzz_2], "John");
        debug_assert_eq!(data[rug_fuzz_3] [rug_fuzz_4] [rug_fuzz_5], "Doe");
        debug_assert_eq!(data[rug_fuzz_6] [rug_fuzz_7], 30);
        debug_assert_eq!(data[rug_fuzz_8] [rug_fuzz_9], Value::Null);
             }
});    }
    #[test]
    fn test_index_with_nonexistent_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!({ "product" : { "name" : "Book", "price" : 20 } });
        debug_assert_eq!(data[rug_fuzz_0] [rug_fuzz_1], Value::Null);
             }
});    }
    #[test]
    fn test_index_with_nonexistent_index() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert_eq!(data[rug_fuzz_3], Value::Null);
             }
});    }
    #[test]
    fn test_index_on_non_object_non_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!({ "name" : "John Doe", "active" : true });
        debug_assert_eq!(data[rug_fuzz_0] [rug_fuzz_1], Value::Null);
        debug_assert_eq!(data[rug_fuzz_2] [rug_fuzz_3], Value::Null);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_730 {
    use crate::{json, Value};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    #[test]
    fn test_index_mut_string_key_insertion() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, bool, bool, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({ "x" : 0 });
        data[rug_fuzz_0] = json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        debug_assert_eq!(data[rug_fuzz_4], json!([false, false, false]));
             }
});    }
    #[test]
    fn test_index_mut_existing_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({ "x" : 0 });
        data[rug_fuzz_0] = json!(rug_fuzz_1);
        debug_assert_eq!(data[rug_fuzz_2], json!(1));
             }
});    }
    #[test]
    fn test_index_mut_array_value_replacement() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, usize, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({ "x" : [0, 1, 2] });
        data[rug_fuzz_0][rug_fuzz_1] = json!(rug_fuzz_2);
        debug_assert_eq!(data[rug_fuzz_3], json!([0, 42, 2]));
             }
});    }
    #[test]
    fn test_index_mut_deeply_nested_key_insertion() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, &str, bool, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({});
        data[rug_fuzz_0][rug_fuzz_1][rug_fuzz_2][rug_fuzz_3] = json!(rug_fuzz_4);
        debug_assert_eq!(
            data[rug_fuzz_5] [rug_fuzz_6] [rug_fuzz_7] [rug_fuzz_8], json!(true)
        );
             }
});    }
    #[test]
    fn test_index_mut_insert_null_for_nonexistent_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({});
        data[rug_fuzz_0] = json!(null);
        debug_assert_eq!(data[rug_fuzz_1], json!(null));
             }
});    }
    #[test]
    fn test_index_mut_panic_when_index_on_non_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({});
        let result = catch_unwind(
            AssertUnwindSafe(|| data[rug_fuzz_0] = json!(rug_fuzz_1)),
        );
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_index_mut_panic_when_index_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!([]);
        let result = catch_unwind(
            AssertUnwindSafe(|| data[rug_fuzz_0] = json!(rug_fuzz_1)),
        );
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_index_mut_panic_when_index_on_non_object() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!(null);
        let result = catch_unwind(
            AssertUnwindSafe(|| data[rug_fuzz_0] = json!(rug_fuzz_1)),
        );
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_index_mut_array_indexing_within_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        data[rug_fuzz_3] = json!(rug_fuzz_4);
        debug_assert_eq!(data, json!([1, 42, 3]));
             }
});    }
    #[test]
    fn test_index_mut_object_insert_null_for_nonexistent_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!({ "x" : 42 });
        data[rug_fuzz_0] = json!(null);
        debug_assert!(data[rug_fuzz_1].is_null());
             }
});    }
}
