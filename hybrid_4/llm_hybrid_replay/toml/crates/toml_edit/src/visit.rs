#![allow(missing_docs)]

//! Document tree traversal to walk a shared borrow of a document tree.
//!
//! Each method of the [`Visit`] trait is a hook that can be overridden
//! to customize the behavior when mutating the corresponding type of node.
//! By default, every method recursively visits the substructure of the
//! input by invoking the right visitor method of each of its fields.
//!
//! ```
//! # use toml_edit::{Item, ArrayOfTables, Table, Value};
//!
//! pub trait Visit<'doc> {
//!     /* ... */
//!
//!     fn visit_item(&mut self, i: &'doc Item) {
//!         visit_item(self, i);
//!     }
//!
//!     /* ... */
//!     # fn visit_value(&mut self, i: &'doc Value);
//!     # fn visit_table(&mut self, i: &'doc Table);
//!     # fn visit_array_of_tables(&mut self, i: &'doc ArrayOfTables);
//! }
//!
//! pub fn visit_item<'doc, V>(v: &mut V, node: &'doc Item)
//! where
//!     V: Visit<'doc> + ?Sized,
//! {
//!     match node {
//!         Item::None => {}
//!         Item::Value(value) => v.visit_value(value),
//!         Item::Table(table) => v.visit_table(table),
//!         Item::ArrayOfTables(array) => v.visit_array_of_tables(array),
//!     }
//! }
//! ```
//!
//! The API is modeled after [`syn::visit`](https://docs.rs/syn/1/syn/visit).
//!
//! # Examples
//!
//! This visitor stores every string in the document.
//!
//! ```
//! # use toml_edit::*;
//! use toml_edit::visit::*;
//!
//! #[derive(Default)]
//! struct StringCollector<'doc> {
//!     strings: Vec<&'doc str>,
//! }
//!
//! impl<'doc> Visit<'doc> for StringCollector<'doc> {
//!     fn visit_string(&mut self, node: &'doc Formatted<String>) {
//!          self.strings.push(node.value().as_str());
//!     }
//! }
//!
//! let input = r#"
//! laputa = "sky-castle"
//! the-force = { value = "surrounds-you" }
//! "#;
//!
//! let mut document: Document = input.parse().unwrap();
//! let mut visitor = StringCollector::default();
//! visitor.visit_document(&document);
//!
//! assert_eq!(visitor.strings, vec!["sky-castle", "surrounds-you"]);
//! ```
//!
//! For a more complex example where the visitor has internal state, see `examples/visit.rs`
//! [on GitHub](https://github.com/ordian/toml_edit/blob/master/examples/visit.rs).

use crate::{
    Array, ArrayOfTables, Datetime, Document, Formatted, InlineTable, Item, Table, TableLike, Value,
};

/// Document tree traversal to mutate an exclusive borrow of a document tree in-place.
///
/// See the [module documentation](self) for details.
pub trait Visit<'doc> {
    fn visit_document(&mut self, node: &'doc Document) {
        visit_document(self, node);
    }

    fn visit_item(&mut self, node: &'doc Item) {
        visit_item(self, node);
    }

    fn visit_table(&mut self, node: &'doc Table) {
        visit_table(self, node);
    }

    fn visit_inline_table(&mut self, node: &'doc InlineTable) {
        visit_inline_table(self, node)
    }

    fn visit_table_like(&mut self, node: &'doc dyn TableLike) {
        visit_table_like(self, node);
    }

    fn visit_table_like_kv(&mut self, key: &'doc str, node: &'doc Item) {
        visit_table_like_kv(self, key, node);
    }

    fn visit_array(&mut self, node: &'doc Array) {
        visit_array(self, node);
    }

    fn visit_array_of_tables(&mut self, node: &'doc ArrayOfTables) {
        visit_array_of_tables(self, node);
    }

    fn visit_value(&mut self, node: &'doc Value) {
        visit_value(self, node);
    }

    fn visit_boolean(&mut self, node: &'doc Formatted<bool>) {
        visit_boolean(self, node)
    }

    fn visit_datetime(&mut self, node: &'doc Formatted<Datetime>) {
        visit_datetime(self, node);
    }

    fn visit_float(&mut self, node: &'doc Formatted<f64>) {
        visit_float(self, node)
    }

    fn visit_integer(&mut self, node: &'doc Formatted<i64>) {
        visit_integer(self, node)
    }

    fn visit_string(&mut self, node: &'doc Formatted<String>) {
        visit_string(self, node)
    }
}

pub fn visit_document<'doc, V>(v: &mut V, node: &'doc Document)
where
    V: Visit<'doc> + ?Sized,
{
    v.visit_table(node.as_table());
}

pub fn visit_item<'doc, V>(v: &mut V, node: &'doc Item)
where
    V: Visit<'doc> + ?Sized,
{
    match node {
        Item::None => {}
        Item::Value(value) => v.visit_value(value),
        Item::Table(table) => v.visit_table(table),
        Item::ArrayOfTables(array) => v.visit_array_of_tables(array),
    }
}

pub fn visit_table<'doc, V>(v: &mut V, node: &'doc Table)
where
    V: Visit<'doc> + ?Sized,
{
    v.visit_table_like(node)
}

pub fn visit_inline_table<'doc, V>(v: &mut V, node: &'doc InlineTable)
where
    V: Visit<'doc> + ?Sized,
{
    v.visit_table_like(node)
}

pub fn visit_table_like<'doc, V>(v: &mut V, node: &'doc dyn TableLike)
where
    V: Visit<'doc> + ?Sized,
{
    for (key, item) in node.iter() {
        v.visit_table_like_kv(key, item)
    }
}

pub fn visit_table_like_kv<'doc, V>(v: &mut V, _key: &'doc str, node: &'doc Item)
where
    V: Visit<'doc> + ?Sized,
{
    v.visit_item(node)
}

pub fn visit_array<'doc, V>(v: &mut V, node: &'doc Array)
where
    V: Visit<'doc> + ?Sized,
{
    for value in node.iter() {
        v.visit_value(value);
    }
}

pub fn visit_array_of_tables<'doc, V>(v: &mut V, node: &'doc ArrayOfTables)
where
    V: Visit<'doc> + ?Sized,
{
    for table in node.iter() {
        v.visit_table(table);
    }
}

pub fn visit_value<'doc, V>(v: &mut V, node: &'doc Value)
where
    V: Visit<'doc> + ?Sized,
{
    match node {
        Value::String(s) => v.visit_string(s),
        Value::Integer(i) => v.visit_integer(i),
        Value::Float(f) => v.visit_float(f),
        Value::Boolean(b) => v.visit_boolean(b),
        Value::Datetime(dt) => v.visit_datetime(dt),
        Value::Array(array) => v.visit_array(array),
        Value::InlineTable(table) => v.visit_inline_table(table),
    }
}

macro_rules! empty_visit {
    ($name: ident, $t: ty) => {
        fn $name<'doc, V>(_v: &mut V, _node: &'doc $t)
        where
            V: Visit<'doc> + ?Sized,
        {
        }
    };
}

empty_visit!(visit_boolean, Formatted<bool>);
empty_visit!(visit_datetime, Formatted<Datetime>);
empty_visit!(visit_float, Formatted<f64>);
empty_visit!(visit_integer, Formatted<i64>);
empty_visit!(visit_string, Formatted<String>);
#[cfg(test)]
mod tests_llm_16_545 {
    use super::*;

use crate::*;
    use crate::visit::Visit;
    use crate::array_of_tables::ArrayOfTables;
    use crate::item::Item;
    use crate::table::Table;

    struct MockVisitor {
        visited: bool,
    }

    impl<'doc> Visit<'doc> for MockVisitor {
        fn visit_array_of_tables(&mut self, _: &'doc ArrayOfTables) {
            self.visited = true;
        }
    }

    #[test]
    fn test_visit_array_of_tables() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);

        let mut visitor = MockVisitor { visited: false };
        visitor.visit_array_of_tables(&array_of_tables);
        assert!(visitor.visited);
    }
}#[cfg(test)]
mod tests_llm_16_546 {
    use crate::Formatted;
    use crate::repr::Decor;
    use crate::visit::Visit;

    struct MockVisitor {
        visited_boolean: Option<bool>,
    }

    impl Visit<'_> for MockVisitor {
        fn visit_boolean(&mut self, node: &Formatted<bool>) {
            self.visited_boolean = Some(*node.value());
        }

        // Implement other visit methods with empty bodies, as needed.
        // ...
    }

    #[test]
    fn test_visit_boolean() {
        let mut visitor = MockVisitor {
            visited_boolean: None,
        };
        let boolean_value = true;
        let decor = Decor::default();
        let formatted_bool = Formatted::new(boolean_value);

        visitor.visit_boolean(&formatted_bool);

        assert_eq!(visitor.visited_boolean, Some(boolean_value));
    }
}#[cfg(test)]
mod tests_llm_16_548 {
    use crate::{Document, Item, Value, visit::*};

    #[derive(Default)]
    struct TestVisitor {
        visited_documents: usize,
    }

    impl<'doc> Visit<'doc> for TestVisitor {
        fn visit_document(&mut self, node: &'doc Document) {
            self.visited_documents += 1;
            visit_document(self, node);
        }
        // Since only `visit_document` is relevant to the test, the other
        // visit methods are left unimplemented for brevity. Implement them
        // if your application requires them.
    }

    #[test]
    fn test_visit_document() {
        let toml_str = r#"
            [package]
            name = "toml_edit"
            version = "1.0.0"
        "#;

        let mut doc = toml_str.parse::<Document>().expect("Parsing failed");
        let mut visitor = TestVisitor::default();

        // Before visiting, the visitor should not have visited any documents
        assert_eq!(visitor.visited_documents, 0);

        // Visit the document with the visitor
        visitor.visit_document(&doc);

        // After visiting, the visitor should have visited 1 document
        assert_eq!(visitor.visited_documents, 1);

        // Make changes to the document
        doc["package"]["name"] = Item::Value(Value::from("different_edit"));
        doc["package"]["version"] = Item::Value(Value::from("2.0.0"));

        // Visit the document again after changes
        visitor.visit_document(&doc);

        // The visitor should have visited 2 documents now
        assert_eq!(visitor.visited_documents, 2);
    }
}#[cfg(test)]
mod tests_llm_16_550_llm_16_550 {
    use super::*;

use crate::*;
    use crate::visit::Visit;

    #[derive(Default)]
    struct TestVisitor {
        visited_inline_tables: Vec<InlineTable>,
    }

    impl<'doc> Visit<'doc> for TestVisitor {
        fn visit_inline_table(&mut self, node: &'doc InlineTable) {
            self.visited_inline_tables.push(node.clone());
        }
    }

    #[test]
    fn test_visit_inline_table() {
        let mut visitor = TestVisitor::default();
        let table = InlineTable::new();
        visitor.visit_inline_table(&table);
        assert_eq!(visitor.visited_inline_tables.len(), 1);
        assert!(visitor.visited_inline_tables[0].is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_551 {
    use crate::{visit::Visit, repr::{Formatted, Decor}};

    struct TestVisitor {
        visited_integer: Option<i64>,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor {
                visited_integer: None,
            }
        }
    }

    impl<'doc> Visit<'doc> for TestVisitor {
        fn visit_integer(&mut self, node: &'doc Formatted<i64>) {
            self.visited_integer = Some(*node.value());
        }
    }

    #[test]
    fn visit_integer_test() {
        let mut visitor = TestVisitor::new();
        let formatted_integer = Formatted::new(42);
        visitor.visit_integer(&formatted_integer);

        assert_eq!(visitor.visited_integer, Some(42));
    }
}#[cfg(test)]
mod tests_llm_16_554 {
    use crate::visit::Visit;
    use crate::table::Table;

    // MockVisit is a simple implementation of the Visit trait
    // that will help us test the `visit_table` function
    struct MockVisit {
        visited_table: bool,
    }

    impl MockVisit {
        fn new() -> Self {
            MockVisit {
                visited_table: false,
            }
        }
    }

    impl Visit<'_> for MockVisit {
        fn visit_table(&mut self, _node: &'_ Table) {
            self.visited_table = true;
        }

        // Implement other visit_ methods as no-ops
        // ... (omitted for brevity)
    }

    #[test]
    fn test_visit_table() {
        let mut visitor = MockVisit::new();
        let table = Table::new();
        assert!(!visitor.visited_table, "Table should not be visited yet");

        visitor.visit_table(&table);
        assert!(visitor.visited_table, "Table should be visited after calling visit_table");
    }
}#[cfg(test)]
mod tests_llm_16_555_llm_16_555 {
    use super::*;

use crate::*;

    use crate::visit::Visit;
    use crate::table::Table;
    use crate::{TableLike, Item};

    struct MockVisitor {
        visited: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor { visited: false }
        }
    }

    impl<'doc> Visit<'doc> for MockVisitor {
        fn visit_table_like(&mut self, _node: &'doc dyn TableLike) {
            self.visited = true;
        }
    }

    #[test]
    fn test_visit_table_like() {
        let mut table_like = Table::new();
        let mut visitor = MockVisitor::new();
        assert!(!visitor.visited, "Visitor should not be marked as visited initially");

        visitor.visit_table_like(&table_like as &dyn TableLike);
        assert!(visitor.visited, "Visitor should be marked as visited after visit_table_like");
    }
}#[cfg(test)]
mod tests_llm_16_559 {
    use super::*;

use crate::*;
    use crate::ArrayOfTables;
    use crate::Table;
    use crate::visit::Visit;

    struct MockVisitor<'doc> {
        visited_tables: Vec<&'doc Table>,
    }

    impl<'doc> Visit<'doc> for MockVisitor<'doc> {
        fn visit_table(&mut self, node: &'doc Table) {
            self.visited_tables.push(node);
        }
    }

    #[test]
    fn visit_array_of_tables_visits_all_tables() {
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());

        let mut visitor = MockVisitor {
            visited_tables: Vec::new(),
        };

        visit_array_of_tables(&mut visitor, &array_of_tables);

        assert_eq!(visitor.visited_tables.len(), 3);
        assert!(visitor.visited_tables.iter().all(|t| array_of_tables.iter().any(|at| std::ptr::eq(*t, at))));
    }
}#[cfg(test)]
mod tests_llm_16_560 {
    use super::*;

use crate::*;
    use crate::{
        visit::{Visit, visit_boolean},
        repr::{Decor, Formatted},
    };

    struct BooleanVisitor {
        pub visited: bool,
    }

    impl<'doc> Visit<'doc> for BooleanVisitor {
        fn visit_boolean(&mut self, node: &'doc Formatted<bool>) {
            self.visited = *node.value();
        }
    }

    #[test]
    fn test_visit_boolean() {
        let mut visitor = BooleanVisitor { visited: false };
        let value = true;
        let formatted = Formatted::new(value);
        visit_boolean(&mut visitor, &formatted);
        assert!(visitor.visited);
    }
}#[cfg(test)]
mod tests_llm_16_562 {
    use super::*;

use crate::*;
    use crate::visit::Visit;

    struct MockVisitor;

    impl<'doc> Visit<'doc> for MockVisitor {
        fn visit_table(&mut self, node: &'doc Table) {
            // Mock behavior for visit_table
        }
    }

    #[test]
    fn test_visit_document() {
        let doc_str = "[table]\nkey = \"value\"";
        let doc = doc_str.parse::<Document>().expect("parsing failed");
        let mut visitor = MockVisitor;
        visit_document(&mut visitor, &doc);
    }
}#[cfg(test)]
mod tests_llm_16_564 {
    use crate::visit::{self, Visit};
    use crate::inline_table::InlineTable;
    use crate::{Item, Value, TableLike};
    use std::collections::HashMap;

    struct TestVisitor<'doc> {
        values: HashMap<&'doc str, &'doc Value>,
    }

    impl<'doc> Visit<'doc> for TestVisitor<'doc> {
        fn visit_table_like(&mut self, node: &'doc dyn TableLike) {
            for (k, v) in node.iter() {
                if let Item::Value(val) = v {
                    self.values.insert(k, val);
                }
            }
        }
    }

    #[test]
    fn visit_inline_table_test() {
        let mut inline_table = InlineTable::new();
        inline_table.insert("test_key", Value::from(42));

        let mut visitor = TestVisitor {
            values: HashMap::new(),
        };
        visit::visit_inline_table(&mut visitor, &inline_table);

        assert!(visitor.values.contains_key("test_key"));
        assert_eq!(visitor.values.get("test_key").unwrap().as_integer(), Some(42));
    }
}#[cfg(test)]
mod tests_llm_16_565_llm_16_565 {
    use crate::{Formatted, Item, Value, visit::{self, Visit}, Document};

    struct IntegerVisitor {
        visited: bool,
    }

    impl<'doc> Visit<'doc> for IntegerVisitor {
        fn visit_integer(&mut self, _: &'doc Formatted<i64>) {
            self.visited = true;
        }
    }

    #[test]
    fn test_visit_integer() {
        let mut doc = "key = 42".parse::<Document>().expect("Parsing toml failed");
        let mut visitor = IntegerVisitor { visited: false };
        if let Some(item) = doc.as_table_mut().get_mut("key") {
            if let Item::Value(Value::Integer(integer)) = item {
                visitor.visit_integer(integer);
                assert!(visitor.visited, "visit_integer should set visited to true");
            } else {
                panic!("Expected an integer value");
            }
        } else {
            panic!("Expected 'key' entry in the document");
        }
    }
}#[cfg(test)]
mod tests_llm_16_567 {
    use crate::visit::Visit;
    use crate::visit::visit_string;
    use crate::repr::Formatted;
    use std::borrow::Cow;

    struct MockVisitor;

    impl<'doc> Visit<'doc> for MockVisitor {
        fn visit_string(&mut self, node: &'doc Formatted<String>) {
            // Implement visit logic for tests here, if needed
        }
    }

    #[test]
    fn test_visit_string() {
        let mut visitor = MockVisitor;
        let formatted_string = Formatted::new(String::from("test_value"));

        visit_string(&mut visitor, &formatted_string);
        // Add assertions here to validate visit_string behavior
        // For example: assert_eq!(formatted_string.value(), "test_value");
    }
}#[cfg(test)]
mod tests_llm_16_568 {
    use crate::{
        table::Table,
        visit::{self, Visit}
    };

    struct MockVisitor {
        visit_table_like_called: bool,
    }

    impl<'doc> Visit<'doc> for MockVisitor {
        fn visit_table_like(&mut self, _: &'doc dyn crate::TableLike) {
            self.visit_table_like_called = true;
        }
    }

    #[test]
    fn test_visit_table() {
        let mut table = Table::new();
        let mut visitor = MockVisitor {
            visit_table_like_called: false,
        };

        visit::visit_table(&mut visitor, &table);
        assert!(visitor.visit_table_like_called);
    }
}#[cfg(test)]
mod tests_llm_16_570_llm_16_570 {
    use crate::{visit::visit_table_like_kv, visit::Visit, Item, Value, Array, InlineTable, Table, Formatted};

    struct TestVisitor;
    impl<'doc> Visit<'doc> for TestVisitor {
        fn visit_item(&mut self, node: &'doc Item) {
            if let Some(table) = node.as_table() {
                assert!(table.is_empty());
            }

            if let Some(array) = node.as_array() {
                assert!(array.is_empty());
            }

            if let Some(table) = node.as_inline_table() {
                assert!(table.is_empty());
            }

            if let Some(value) = node.as_value() {
                if let Value::String(ref s) = value {
                    assert_eq!(s.value(), "test");
                } else {
                    assert!(false, "Expected a string value");
                }
            }
        }
    }

    #[test]
    fn visit_table_like_kv_empty_table() {
        let table = Table::new();
        let item = Item::Table(table);
        let mut visitor = TestVisitor;
        crate::visit::visit_table_like_kv(&mut visitor, "table", &item);
    }

    #[test]
    fn visit_table_like_kv_empty_array() {
        let array = Array::new();
        let item = Item::Value(Value::Array(array));
        let mut visitor = TestVisitor;
        crate::visit::visit_table_like_kv(&mut visitor, "array", &item);
    }

    #[test]
    fn visit_table_like_kv_empty_inline_table() {
        let inline_table = InlineTable::new();
        let item = Item::Value(Value::InlineTable(inline_table));
        let mut visitor = TestVisitor;
        crate::visit::visit_table_like_kv(&mut visitor, "inline_table", &item);
    }

    #[test]
    fn visit_table_like_kv_string_value() {
        let value = Value::String(Formatted::new("test".to_string()));
        let item = Item::Value(value);
        let mut visitor = TestVisitor;
        crate::visit::visit_table_like_kv(&mut visitor, "string_value", &item);
    }
}