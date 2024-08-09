use std::ops;

use crate::document::Document;
use crate::key::Key;
use crate::table::TableKeyValue;
use crate::{value, InlineTable, InternalString, Item, Table, Value};

// copied from
// https://github.com/serde-rs/json/blob/master/src/value/index.rs

pub trait Index: crate::private::Sealed {
    #[doc(hidden)]
    fn index<'v>(&self, val: &'v Item) -> Option<&'v Item>;
    #[doc(hidden)]
    fn index_mut<'v>(&self, val: &'v mut Item) -> Option<&'v mut Item>;
}

impl Index for usize {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        match *v {
            Item::ArrayOfTables(ref aot) => aot.values.get(*self),
            Item::Value(ref a) if a.is_array() => a.as_array().and_then(|a| a.values.get(*self)),
            _ => None,
        }
    }
    fn index_mut<'v>(&self, v: &'v mut Item) -> Option<&'v mut Item> {
        match *v {
            Item::ArrayOfTables(ref mut vec) => vec.values.get_mut(*self),
            Item::Value(ref mut a) => a.as_array_mut().and_then(|a| a.values.get_mut(*self)),
            _ => None,
        }
    }
}

impl Index for str {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        match *v {
            Item::Table(ref t) => t.get(self),
            Item::Value(ref v) => v
                .as_inline_table()
                .and_then(|t| t.items.get(self))
                .and_then(|kv| {
                    if !kv.value.is_none() {
                        Some(&kv.value)
                    } else {
                        None
                    }
                }),
            _ => None,
        }
    }
    fn index_mut<'v>(&self, v: &'v mut Item) -> Option<&'v mut Item> {
        if let Item::None = *v {
            let mut t = InlineTable::default();
            t.items.insert(
                InternalString::from(self),
                TableKeyValue::new(Key::new(self), Item::None),
            );
            *v = value(Value::InlineTable(t));
        }
        match *v {
            Item::Table(ref mut t) => Some(t.entry(self).or_insert(Item::None)),
            Item::Value(ref mut v) => v.as_inline_table_mut().map(|t| {
                &mut t
                    .items
                    .entry(InternalString::from(self))
                    .or_insert_with(|| TableKeyValue::new(Key::new(self), Item::None))
                    .value
            }),
            _ => None,
        }
    }
}

impl Index for String {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        self[..].index(v)
    }
    fn index_mut<'v>(&self, v: &'v mut Item) -> Option<&'v mut Item> {
        self[..].index_mut(v)
    }
}

impl<'a, T: ?Sized> Index for &'a T
where
    T: Index,
{
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        (**self).index(v)
    }
    fn index_mut<'v>(&self, v: &'v mut Item) -> Option<&'v mut Item> {
        (**self).index_mut(v)
    }
}

impl<I> ops::Index<I> for Item
where
    I: Index,
{
    type Output = Item;

    fn index(&self, index: I) -> &Item {
        index.index(self).expect("index not found")
    }
}

impl<I> ops::IndexMut<I> for Item
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Item {
        index.index_mut(self).expect("index not found")
    }
}

impl<'s> ops::Index<&'s str> for Table {
    type Output = Item;

    fn index(&self, key: &'s str) -> &Item {
        self.get(key).expect("index not found")
    }
}

impl<'s> ops::IndexMut<&'s str> for Table {
    fn index_mut(&mut self, key: &'s str) -> &mut Item {
        self.entry(key).or_insert(Item::None)
    }
}

impl<'s> ops::Index<&'s str> for InlineTable {
    type Output = Value;

    fn index(&self, key: &'s str) -> &Value {
        self.get(key).expect("index not found")
    }
}

impl<'s> ops::IndexMut<&'s str> for InlineTable {
    fn index_mut(&mut self, key: &'s str) -> &mut Value {
        self.get_mut(key).expect("index not found")
    }
}

impl<'s> ops::Index<&'s str> for Document {
    type Output = Item;

    fn index(&self, key: &'s str) -> &Item {
        self.root.index(key)
    }
}

impl<'s> ops::IndexMut<&'s str> for Document {
    fn index_mut(&mut self, key: &'s str) -> &mut Item {
        self.root.index_mut(key)
    }
}
#[cfg(test)]
mod tests_llm_16_93_llm_16_93 {
    use crate::index::Index;
    use crate::item::Item;
    use crate::value::Value;
    use crate::array::Array;
    use crate::inline_table::InlineTable;
    use crate::table::Table;
    use std::str::FromStr;

    #[test]
    fn index_string_into_table() {
        let mut table = Table::new();
        table["key"] = Item::Value(Value::from("value"));
        let item = Item::Table(table);
        let indexed = "key".index(&item);
        assert!(matches!(indexed, Some(Item::Value(Value::String(_)))));
    }

    #[test]
    fn index_string_into_array() {
        let mut array = Array::new();
        array.push(42);
        let item = Item::Value(Value::Array(array));
        let indexed = "0".index(&item);
        assert!(matches!(indexed, Some(Item::Value(Value::Integer(_)))));
    }

    #[test]
    fn index_string_into_value_string() {
        let val = Value::from("test");
        let item = Item::Value(val);
        let indexed = "invalid".index(&item);
        assert!(indexed.is_none());
    }

    #[test]
    fn index_string_into_inline_table() {
        let mut table = InlineTable::new();
        table.insert("key", Value::from("value"));
        let item = Item::Value(Value::InlineTable(table));
        let indexed = "key".index(&item);
        assert!(matches!(indexed, Some(Item::Value(Value::String(_)))));
    }

    #[test]
    fn invalid_index_string_into_table() {
        let table = Table::new();
        let item = Item::Table(table);
        let indexed = "nonexistent".index(&item);
        assert!(indexed.is_none());
    }

    #[test]
    fn index_string_into_array_of_tables() {
        let mut array_of_tables = crate::array_of_tables::ArrayOfTables::new();
        let mut table = Table::new();
        table["key"] = Item::Value(Value::from("value"));
        array_of_tables.push(table);
        let item = Item::ArrayOfTables(array_of_tables);
        let indexed = "0".index(&item);
        assert!(matches!(indexed, Some(Item::Table(_))));
    }

    #[test]
    fn invalid_index_string_into_array_of_tables() {
        let array_of_tables = crate::array_of_tables::ArrayOfTables::new();
        let item = Item::ArrayOfTables(array_of_tables);
        let indexed = "nonexistent".index(&item);
        assert!(indexed.is_none());
    }

    #[test]
    fn index_string_into_none() {
        let none = Item::None;
        let indexed = "key".index(&none);
        assert!(indexed.is_none());
    }
}#[cfg(test)]
mod tests_llm_16_118 {
    use super::*;

use crate::*;
    use crate::index::Index;
    use crate::Item;

    #[test]
    fn test_index_array_of_tables() {
        let mut aot = ArrayOfTables::new();
        let idx = 0_usize;

        // Test empty ArrayOfTables
        assert!(idx.index(&Item::ArrayOfTables(aot.clone())).is_none());

        // Add a table and test again
        aot.push(Table::new());
        assert!(idx.index(&Item::ArrayOfTables(aot.clone())).is_some());
    }

    #[test]
    fn test_index_value_array() {
        let mut arr = Array::new();
        let idx = 0_usize;

        // Test empty Array
        assert!(idx.index(&Item::Value(Value::Array(arr.clone()))).is_none());

        // Add a value and test again
        arr.push(Value::Integer(Formatted::new(42)));
        assert!(idx.index(&Item::Value(Value::Array(arr.clone()))).is_some());
    }

    #[test]
    fn test_index_other() {
        let idx = 0_usize;
        let value_item = Item::Value(Value::Integer(Formatted::new(42)));
        let table_item = Item::Table(Table::new());

        // Test Index on Value
        assert!(idx.index(&value_item).is_none());
        // Test Index on Table
        assert!(idx.index(&table_item).is_none());
    }
}#[cfg(test)]
mod tests_llm_16_201 {
    use super::*;

use crate::*;
    use crate::Document;
    use crate::Item;
    use crate::Value;

    #[test]
    fn test_document_index() {
        let toml = r#"
            [server]
            host = "localhost"
            port = 80
        "#;

        let document = toml.parse::<Document>().unwrap();
        assert_eq!(document["server"]["host"].as_value().unwrap().as_str(), Some("localhost"));
        assert_eq!(document["server"]["port"].as_value().unwrap().as_integer(), Some(80));
    }

    #[test]
    #[should_panic]
    fn test_document_index_missing() {
        let toml = r#"
            [server]
            host = "localhost"
        "#;

        let document = toml.parse::<Document>().unwrap();
        let _ = document["server"]["port"];
    }

    #[test]
    fn test_document_index_set_and_retrieve() {
        let mut document = Document::new();
        let host = Item::Value(Value::from("localhost"));
        document["server"]["host"] = host;
        assert_eq!(document["server"]["host"].as_value().unwrap().as_str(), Some("localhost"));
    }
}#[cfg(test)]
mod tests_llm_16_205_llm_16_205 {
    use crate::{value::Value, Item, Document};

    #[test]
    fn test_index_mut() {
        let mut doc = "[pkg]\nname = \"my-package\"\n".parse::<Document>().unwrap();
        if let Item::Value(value) = doc["pkg"]["name"].clone() {
            assert_eq!(value.as_str(), Some("my-package"));
            doc["pkg"]["name"] = Item::Value(Value::from("my-package-updated"));
        }
        assert_eq!(doc["pkg"]["name"].as_value().unwrap().as_str(), Some("my-package-updated"));

        // Test that indexing mutably into a non-existing table creates it
        doc["dependencies"]["my_dep"] = Item::Value(Value::from("1.0"));
        assert_eq!(doc["dependencies"]["my_dep"].as_value().unwrap().as_str(), Some("1.0"));

        // Test that indexing mutably into a non-existing table with nested key creates it
        let array_of_deps = Value::Array(crate::Array::from_iter(vec!["dep1", "dep2"]));
        doc["features"]["extras"] = Item::Value(array_of_deps);
        assert!(doc["features"]["extras"].as_array().is_some());
    }
}#[cfg(test)]
mod tests_llm_16_206_llm_16_206 {
    use crate::{InlineTable, Value};
    use std::ops::IndexMut; // Import IndexMut trait

    #[test]
    fn index_mut_existing_key() {
        let mut table = InlineTable::new();
        table.insert("key", Value::from(42));
        {
            let value = table.index_mut("key");
            *value = Value::from(99);
        }
        assert_eq!(table.get("key").and_then(|v| v.as_integer()), Some(99));
    }

    #[test]
    #[should_panic(expected = "index not found")]
    fn index_mut_non_existing_key() {
        let mut table = InlineTable::new();
        table.index_mut("key");
    }
}#[cfg(test)]
mod tests_llm_16_207_llm_16_207 {
    use crate::{Document, Item, Value, value};

    #[test]
    fn test_index_mut() {
        let mut doc = "[table]\nkey = 'value'".parse::<Document>().expect("invalid toml");
        {
            let table = doc["table"].as_table_mut().unwrap();
            let value: &mut Item = &mut table["key"];
            if let Item::Value(ref mut v) = value {
                *v = value::Value::from("new value");
            }
        }
        assert_eq!(doc.to_string(), "[table]\nkey = \"new value\"\n");
    }
}