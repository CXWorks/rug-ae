use std::ops;
use crate::document::Document;
use crate::key::Key;
use crate::table::TableKeyValue;
use crate::{value, InlineTable, InternalString, Item, Table, Value};
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
            Item::Value(ref a) if a.is_array() => {
                a.as_array().and_then(|a| a.values.get(*self))
            }
            _ => None,
        }
    }
    fn index_mut<'v>(&self, v: &'v mut Item) -> Option<&'v mut Item> {
        match *v {
            Item::ArrayOfTables(ref mut vec) => vec.values.get_mut(*self),
            Item::Value(ref mut a) => {
                a.as_array_mut().and_then(|a| a.values.get_mut(*self))
            }
            _ => None,
        }
    }
}
impl Index for str {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        match *v {
            Item::Table(ref t) => t.get(self),
            Item::Value(ref v) => {
                v
                    .as_inline_table()
                    .and_then(|t| t.items.get(self))
                    .and_then(|kv| {
                        if !kv.value.is_none() { Some(&kv.value) } else { None }
                    })
            }
            _ => None,
        }
    }
    fn index_mut<'v>(&self, v: &'v mut Item) -> Option<&'v mut Item> {
        if let Item::None = *v {
            let mut t = InlineTable::default();
            t.items
                .insert(
                    InternalString::from(self),
                    TableKeyValue::new(Key::new(self), Item::None),
                );
            *v = value(Value::InlineTable(t));
        }
        match *v {
            Item::Table(ref mut t) => Some(t.entry(self).or_insert(Item::None)),
            Item::Value(ref mut v) => {
                v
                    .as_inline_table_mut()
                    .map(|t| {
                        &mut t
                            .items
                            .entry(InternalString::from(self))
                            .or_insert_with(|| TableKeyValue::new(
                                Key::new(self),
                                Item::None,
                            ))
                            .value
                    })
            }
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
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_table = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        let item = Item::Table(table);
        let indexed = rug_fuzz_2.index(&item);
        debug_assert!(matches!(indexed, Some(Item::Value(Value::String(_)))));
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_table = 0;
    }
    #[test]
    fn index_string_into_array() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_array = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "0";
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        let item = Item::Value(Value::Array(array));
        let indexed = rug_fuzz_1.index(&item);
        debug_assert!(matches!(indexed, Some(Item::Value(Value::Integer(_)))));
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_array = 0;
    }
    #[test]
    fn index_string_into_value_string() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_value_string = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "invalid";
        let val = Value::from(rug_fuzz_0);
        let item = Item::Value(val);
        let indexed = rug_fuzz_1.index(&item);
        debug_assert!(indexed.is_none());
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_value_string = 0;
    }
    #[test]
    fn index_string_into_inline_table() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_inline_table = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        let item = Item::Value(Value::InlineTable(table));
        let indexed = rug_fuzz_2.index(&item);
        debug_assert!(matches!(indexed, Some(Item::Value(Value::String(_)))));
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_inline_table = 0;
    }
    #[test]
    fn invalid_index_string_into_table() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_invalid_index_string_into_table = 0;
        let rug_fuzz_0 = "nonexistent";
        let table = Table::new();
        let item = Item::Table(table);
        let indexed = rug_fuzz_0.index(&item);
        debug_assert!(indexed.is_none());
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_invalid_index_string_into_table = 0;
    }
    #[test]
    fn index_string_into_array_of_tables() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_array_of_tables = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "0";
        let mut array_of_tables = crate::array_of_tables::ArrayOfTables::new();
        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        array_of_tables.push(table);
        let item = Item::ArrayOfTables(array_of_tables);
        let indexed = rug_fuzz_2.index(&item);
        debug_assert!(matches!(indexed, Some(Item::Table(_))));
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_array_of_tables = 0;
    }
    #[test]
    fn invalid_index_string_into_array_of_tables() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_invalid_index_string_into_array_of_tables = 0;
        let rug_fuzz_0 = "nonexistent";
        let array_of_tables = crate::array_of_tables::ArrayOfTables::new();
        let item = Item::ArrayOfTables(array_of_tables);
        let indexed = rug_fuzz_0.index(&item);
        debug_assert!(indexed.is_none());
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_invalid_index_string_into_array_of_tables = 0;
    }
    #[test]
    fn index_string_into_none() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_none = 0;
        let rug_fuzz_0 = "key";
        let none = Item::None;
        let indexed = rug_fuzz_0.index(&none);
        debug_assert!(indexed.is_none());
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_index_string_into_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_118 {
    use super::*;
    use crate::*;
    use crate::index::Index;
    use crate::Item;
    #[test]
    fn test_index_array_of_tables() {
        let _rug_st_tests_llm_16_118_rrrruuuugggg_test_index_array_of_tables = 0;
        let rug_fuzz_0 = 0_usize;
        let mut aot = ArrayOfTables::new();
        let idx = rug_fuzz_0;
        debug_assert!(idx.index(& Item::ArrayOfTables(aot.clone())).is_none());
        aot.push(Table::new());
        debug_assert!(idx.index(& Item::ArrayOfTables(aot.clone())).is_some());
        let _rug_ed_tests_llm_16_118_rrrruuuugggg_test_index_array_of_tables = 0;
    }
    #[test]
    fn test_index_value_array() {
        let _rug_st_tests_llm_16_118_rrrruuuugggg_test_index_value_array = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 42;
        let mut arr = Array::new();
        let idx = rug_fuzz_0;
        debug_assert!(idx.index(& Item::Value(Value::Array(arr.clone()))).is_none());
        arr.push(Value::Integer(Formatted::new(rug_fuzz_1)));
        debug_assert!(idx.index(& Item::Value(Value::Array(arr.clone()))).is_some());
        let _rug_ed_tests_llm_16_118_rrrruuuugggg_test_index_value_array = 0;
    }
    #[test]
    fn test_index_other() {
        let _rug_st_tests_llm_16_118_rrrruuuugggg_test_index_other = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 42;
        let idx = rug_fuzz_0;
        let value_item = Item::Value(Value::Integer(Formatted::new(rug_fuzz_1)));
        let table_item = Item::Table(Table::new());
        debug_assert!(idx.index(& value_item).is_none());
        debug_assert!(idx.index(& table_item).is_none());
        let _rug_ed_tests_llm_16_118_rrrruuuugggg_test_index_other = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_201 {
    use super::*;
    use crate::*;
    use crate::Document;
    use crate::Item;
    use crate::Value;
    #[test]
    fn test_document_index() {
        let _rug_st_tests_llm_16_201_rrrruuuugggg_test_document_index = 0;
        let rug_fuzz_0 = r#"
            [server]
            host = "localhost"
            port = 80
        "#;
        let rug_fuzz_1 = "server";
        let rug_fuzz_2 = "host";
        let rug_fuzz_3 = "server";
        let rug_fuzz_4 = "port";
        let toml = rug_fuzz_0;
        let document = toml.parse::<Document>().unwrap();
        debug_assert_eq!(
            document[rug_fuzz_1] [rug_fuzz_2].as_value().unwrap().as_str(),
            Some("localhost")
        );
        debug_assert_eq!(
            document[rug_fuzz_3] [rug_fuzz_4].as_value().unwrap().as_integer(), Some(80)
        );
        let _rug_ed_tests_llm_16_201_rrrruuuugggg_test_document_index = 0;
    }
    #[test]
    #[should_panic]
    fn test_document_index_missing() {
        let _rug_st_tests_llm_16_201_rrrruuuugggg_test_document_index_missing = 0;
        let rug_fuzz_0 = r#"
            [server]
            host = "localhost"
        "#;
        let rug_fuzz_1 = "server";
        let rug_fuzz_2 = "port";
        let toml = rug_fuzz_0;
        let document = toml.parse::<Document>().unwrap();
        let _ = document[rug_fuzz_1][rug_fuzz_2];
        let _rug_ed_tests_llm_16_201_rrrruuuugggg_test_document_index_missing = 0;
    }
    #[test]
    fn test_document_index_set_and_retrieve() {
        let _rug_st_tests_llm_16_201_rrrruuuugggg_test_document_index_set_and_retrieve = 0;
        let rug_fuzz_0 = "localhost";
        let rug_fuzz_1 = "server";
        let rug_fuzz_2 = "host";
        let rug_fuzz_3 = "server";
        let rug_fuzz_4 = "host";
        let mut document = Document::new();
        let host = Item::Value(Value::from(rug_fuzz_0));
        document[rug_fuzz_1][rug_fuzz_2] = host;
        debug_assert_eq!(
            document[rug_fuzz_3] [rug_fuzz_4].as_value().unwrap().as_str(),
            Some("localhost")
        );
        let _rug_ed_tests_llm_16_201_rrrruuuugggg_test_document_index_set_and_retrieve = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_205_llm_16_205 {
    use crate::{value::Value, Item, Document};
    #[test]
    fn test_index_mut() {
        let _rug_st_tests_llm_16_205_llm_16_205_rrrruuuugggg_test_index_mut = 0;
        let rug_fuzz_0 = "[pkg]\nname = \"my-package\"\n";
        let rug_fuzz_1 = "pkg";
        let rug_fuzz_2 = "name";
        let rug_fuzz_3 = "pkg";
        let rug_fuzz_4 = "name";
        let rug_fuzz_5 = "my-package-updated";
        let rug_fuzz_6 = "pkg";
        let rug_fuzz_7 = "name";
        let rug_fuzz_8 = "dependencies";
        let rug_fuzz_9 = "my_dep";
        let rug_fuzz_10 = "1.0";
        let rug_fuzz_11 = "dependencies";
        let rug_fuzz_12 = "my_dep";
        let rug_fuzz_13 = "dep1";
        let rug_fuzz_14 = "features";
        let rug_fuzz_15 = "extras";
        let rug_fuzz_16 = "features";
        let rug_fuzz_17 = "extras";
        let mut doc = rug_fuzz_0.parse::<Document>().unwrap();
        if let Item::Value(value) = doc[rug_fuzz_1][rug_fuzz_2].clone() {
            debug_assert_eq!(value.as_str(), Some("my-package"));
            doc[rug_fuzz_3][rug_fuzz_4] = Item::Value(Value::from(rug_fuzz_5));
        }
        debug_assert_eq!(
            doc[rug_fuzz_6] [rug_fuzz_7].as_value().unwrap().as_str(),
            Some("my-package-updated")
        );
        doc[rug_fuzz_8][rug_fuzz_9] = Item::Value(Value::from(rug_fuzz_10));
        debug_assert_eq!(
            doc[rug_fuzz_11] [rug_fuzz_12].as_value().unwrap().as_str(), Some("1.0")
        );
        let array_of_deps = Value::Array(
            crate::Array::from_iter(vec![rug_fuzz_13, "dep2"]),
        );
        doc[rug_fuzz_14][rug_fuzz_15] = Item::Value(array_of_deps);
        debug_assert!(doc[rug_fuzz_16] [rug_fuzz_17].as_array().is_some());
        let _rug_ed_tests_llm_16_205_llm_16_205_rrrruuuugggg_test_index_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_206_llm_16_206 {
    use crate::{InlineTable, Value};
    use std::ops::IndexMut;
    #[test]
    fn index_mut_existing_key() {
        let _rug_st_tests_llm_16_206_llm_16_206_rrrruuuugggg_index_mut_existing_key = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = 99;
        let rug_fuzz_4 = "key";
        let mut table = InlineTable::new();
        table.insert(rug_fuzz_0, Value::from(rug_fuzz_1));
        {
            let value = table.index_mut(rug_fuzz_2);
            *value = Value::from(rug_fuzz_3);
        }
        debug_assert_eq!(table.get(rug_fuzz_4).and_then(| v | v.as_integer()), Some(99));
        let _rug_ed_tests_llm_16_206_llm_16_206_rrrruuuugggg_index_mut_existing_key = 0;
    }
    #[test]
    #[should_panic(expected = "index not found")]
    fn index_mut_non_existing_key() {
        let _rug_st_tests_llm_16_206_llm_16_206_rrrruuuugggg_index_mut_non_existing_key = 0;
        let rug_fuzz_0 = "key";
        let mut table = InlineTable::new();
        table.index_mut(rug_fuzz_0);
        let _rug_ed_tests_llm_16_206_llm_16_206_rrrruuuugggg_index_mut_non_existing_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_207_llm_16_207 {
    use crate::{Document, Item, Value, value};
    #[test]
    fn test_index_mut() {
        let _rug_st_tests_llm_16_207_llm_16_207_rrrruuuugggg_test_index_mut = 0;
        let rug_fuzz_0 = "[table]\nkey = 'value'";
        let rug_fuzz_1 = "invalid toml";
        let rug_fuzz_2 = "table";
        let rug_fuzz_3 = "key";
        let rug_fuzz_4 = "new value";
        let mut doc = rug_fuzz_0.parse::<Document>().expect(rug_fuzz_1);
        {
            let table = doc[rug_fuzz_2].as_table_mut().unwrap();
            let value: &mut Item = &mut table[rug_fuzz_3];
            if let Item::Value(ref mut v) = value {
                *v = value::Value::from(rug_fuzz_4);
            }
        }
        debug_assert_eq!(doc.to_string(), "[table]\nkey = \"new value\"\n");
        let _rug_ed_tests_llm_16_207_llm_16_207_rrrruuuugggg_test_index_mut = 0;
    }
}
