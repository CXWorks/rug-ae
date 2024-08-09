#![allow(missing_docs)]
//! Document tree traversal to mutate an exclusive borrow of a document tree in place.
//!
//!
//! Each method of the [`VisitMut`] trait is a hook that can be overridden
//! to customize the behavior when mutating the corresponding type of node.
//! By default, every method recursively visits the substructure of the
//! input by invoking the right visitor method of each of its fields.
//!
//! ```
//! # use toml_edit::{Item, ArrayOfTables, Table, Value};
//!
//! pub trait VisitMut {
//!     /* ... */
//!
//!     fn visit_item_mut(&mut self, i: &mut Item) {
//!         visit_item_mut(self, i);
//!     }
//!
//!     /* ... */
//!     # fn visit_value_mut(&mut self, i: &mut Value);
//!     # fn visit_table_mut(&mut self, i: &mut Table);
//!     # fn visit_array_of_tables_mut(&mut self, i: &mut ArrayOfTables);
//! }
//!
//! pub fn visit_item_mut<V>(v: &mut V, node: &mut Item)
//! where
//!     V: VisitMut + ?Sized,
//! {
//!     match node {
//!         Item::None => {}
//!         Item::Value(value) => v.visit_value_mut(value),
//!         Item::Table(table) => v.visit_table_mut(table),
//!         Item::ArrayOfTables(array) => v.visit_array_of_tables_mut(array),
//!     }
//! }
//! ```
//!
//! The API is modeled after [`syn::visit_mut`](https://docs.rs/syn/1/syn/visit_mut).
//!
//! # Examples
//!
//! This visitor replaces every floating point value with its decimal string representation, to
//! 2 decimal points.
//!
//! ```
//! # use toml_edit::*;
//! use toml_edit::visit_mut::*;
//!
//! struct FloatToString;
//!
//! impl VisitMut for FloatToString {
//!     fn visit_value_mut(&mut self, node: &mut Value) {
//!         if let Value::Float(f) = node {
//!             // Convert the float to a string.
//!             let mut s = Formatted::new(format!("{:.2}", f.value()));
//!             // Copy over the formatting.
//!             std::mem::swap(s.decor_mut(), f.decor_mut());
//!             *node = Value::String(s);
//!         }
//!         // Most of the time, you will also need to call the default implementation to recurse
//!         // further down the document tree.
//!         visit_value_mut(self, node);
//!     }
//! }
//!
//! let input = r#"
//! banana = 3.26
//! table = { apple = 4.5 }
//! "#;
//!
//! let mut document: Document = input.parse().unwrap();
//! let mut visitor = FloatToString;
//! visitor.visit_document_mut(&mut document);
//!
//! let output = r#"
//! banana = "3.26"
//! table = { apple = "4.50" }
//! "#;
//!
//! assert_eq!(format!("{}", document), output);
//! ```
//!
//! For a more complex example where the visitor has internal state, see `examples/visit.rs`
//! [on GitHub](https://github.com/ordian/toml_edit/blob/master/examples/visit.rs).
use crate::{
    Array, ArrayOfTables, Datetime, Document, Formatted, InlineTable, Item, KeyMut,
    Table, TableLike, Value,
};
/// Document tree traversal to mutate an exclusive borrow of a document tree in-place.
///
/// See the [module documentation](self) for details.
pub trait VisitMut {
    fn visit_document_mut(&mut self, node: &mut Document) {
        visit_document_mut(self, node);
    }
    fn visit_item_mut(&mut self, node: &mut Item) {
        visit_item_mut(self, node);
    }
    fn visit_table_mut(&mut self, node: &mut Table) {
        visit_table_mut(self, node);
    }
    fn visit_inline_table_mut(&mut self, node: &mut InlineTable) {
        visit_inline_table_mut(self, node)
    }
    /// [`visit_table_mut`](Self::visit_table_mut) and
    /// [`visit_inline_table_mut`](Self::visit_inline_table_mut) both recurse into this method.
    fn visit_table_like_mut(&mut self, node: &mut dyn TableLike) {
        visit_table_like_mut(self, node);
    }
    fn visit_table_like_kv_mut(&mut self, key: KeyMut<'_>, node: &mut Item) {
        visit_table_like_kv_mut(self, key, node);
    }
    fn visit_array_mut(&mut self, node: &mut Array) {
        visit_array_mut(self, node);
    }
    fn visit_array_of_tables_mut(&mut self, node: &mut ArrayOfTables) {
        visit_array_of_tables_mut(self, node);
    }
    fn visit_value_mut(&mut self, node: &mut Value) {
        visit_value_mut(self, node);
    }
    fn visit_boolean_mut(&mut self, node: &mut Formatted<bool>) {
        visit_boolean_mut(self, node)
    }
    fn visit_datetime_mut(&mut self, node: &mut Formatted<Datetime>) {
        visit_datetime_mut(self, node);
    }
    fn visit_float_mut(&mut self, node: &mut Formatted<f64>) {
        visit_float_mut(self, node)
    }
    fn visit_integer_mut(&mut self, node: &mut Formatted<i64>) {
        visit_integer_mut(self, node)
    }
    fn visit_string_mut(&mut self, node: &mut Formatted<String>) {
        visit_string_mut(self, node)
    }
}
pub fn visit_document_mut<V>(v: &mut V, node: &mut Document)
where
    V: VisitMut + ?Sized,
{
    v.visit_table_mut(node.as_table_mut());
}
pub fn visit_item_mut<V>(v: &mut V, node: &mut Item)
where
    V: VisitMut + ?Sized,
{
    match node {
        Item::None => {}
        Item::Value(value) => v.visit_value_mut(value),
        Item::Table(table) => v.visit_table_mut(table),
        Item::ArrayOfTables(array) => v.visit_array_of_tables_mut(array),
    }
}
pub fn visit_table_mut<V>(v: &mut V, node: &mut Table)
where
    V: VisitMut + ?Sized,
{
    v.visit_table_like_mut(node);
}
pub fn visit_inline_table_mut<V>(v: &mut V, node: &mut InlineTable)
where
    V: VisitMut + ?Sized,
{
    v.visit_table_like_mut(node);
}
pub fn visit_table_like_mut<V>(v: &mut V, node: &mut dyn TableLike)
where
    V: VisitMut + ?Sized,
{
    for (key, item) in node.iter_mut() {
        v.visit_table_like_kv_mut(key, item);
    }
}
pub fn visit_table_like_kv_mut<V>(v: &mut V, _key: KeyMut<'_>, node: &mut Item)
where
    V: VisitMut + ?Sized,
{
    v.visit_item_mut(node)
}
pub fn visit_array_mut<V>(v: &mut V, node: &mut Array)
where
    V: VisitMut + ?Sized,
{
    for value in node.iter_mut() {
        v.visit_value_mut(value);
    }
}
pub fn visit_array_of_tables_mut<V>(v: &mut V, node: &mut ArrayOfTables)
where
    V: VisitMut + ?Sized,
{
    for table in node.iter_mut() {
        v.visit_table_mut(table);
    }
}
pub fn visit_value_mut<V>(v: &mut V, node: &mut Value)
where
    V: VisitMut + ?Sized,
{
    match node {
        Value::String(s) => v.visit_string_mut(s),
        Value::Integer(i) => v.visit_integer_mut(i),
        Value::Float(f) => v.visit_float_mut(f),
        Value::Boolean(b) => v.visit_boolean_mut(b),
        Value::Datetime(dt) => v.visit_datetime_mut(dt),
        Value::Array(array) => v.visit_array_mut(array),
        Value::InlineTable(table) => v.visit_inline_table_mut(table),
    }
}
macro_rules! empty_visit_mut {
    ($name:ident, $t:ty) => {
        fn $name < V > (_v : & mut V, _node : & mut $t) where V : VisitMut + ? Sized, {}
    };
}
empty_visit_mut!(visit_boolean_mut, Formatted < bool >);
empty_visit_mut!(visit_datetime_mut, Formatted < Datetime >);
empty_visit_mut!(visit_float_mut, Formatted < f64 >);
empty_visit_mut!(visit_integer_mut, Formatted < i64 >);
empty_visit_mut!(visit_string_mut, Formatted < String >);
#[cfg(test)]
mod tests_llm_16_573 {
    use super::*;
    use crate::*;
    use crate::visit_mut::VisitMut;
    use crate::array_of_tables::ArrayOfTables;
    use crate::table::Table;
    struct TestVisitor;
    impl VisitMut for TestVisitor {
        fn visit_array_of_tables_mut(&mut self, node: &mut ArrayOfTables) {
            node.push(Table::new());
        }
    }
    #[test]
    fn test_visit_array_of_tables_mut() {
        let _rug_st_tests_llm_16_573_rrrruuuugggg_test_visit_array_of_tables_mut = 0;
        let mut tables = ArrayOfTables::new();
        tables.push(Table::new());
        let initial_length = tables.len();
        let mut visitor = TestVisitor;
        visitor.visit_array_of_tables_mut(&mut tables);
        let modified_length = tables.len();
        debug_assert_eq!(modified_length, initial_length + 1);
        let _rug_ed_tests_llm_16_573_rrrruuuugggg_test_visit_array_of_tables_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_576 {
    use super::*;
    use crate::*;
    use crate::Document;
    use crate::visit_mut::VisitMut;
    struct TestVisitor;
    impl VisitMut for TestVisitor {
        fn visit_document_mut(&mut self, node: &mut Document) {
            let doc_str = node.to_string();
            let doc_str_trimmed = doc_str.trim_start();
            let new_doc: Document = doc_str_trimmed.parse().unwrap();
            *node = new_doc;
        }
    }
    #[test]
    fn test_visit_document_mut() {
        let _rug_st_tests_llm_16_576_rrrruuuugggg_test_visit_document_mut = 0;
        let rug_fuzz_0 = r#"
            # Example TOML
            [test]
            key = "value"
        "#;
        let rug_fuzz_1 = r#"
# Example TOML
[test]
key = "value"
"#;
        let toml_str = rug_fuzz_0;
        let mut doc: Document = toml_str.parse().unwrap();
        let mut visitor = TestVisitor;
        visitor.visit_document_mut(&mut doc);
        let expected_toml_str = rug_fuzz_1;
        debug_assert_eq!(doc.to_string(), expected_toml_str);
        let _rug_ed_tests_llm_16_576_rrrruuuugggg_test_visit_document_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_577_llm_16_577 {
    use crate::Formatted;
    use crate::visit_mut::VisitMut;
    struct Visitor;
    impl VisitMut for Visitor {
        fn visit_float_mut(&mut self, node: &mut Formatted<f64>) {
            let value = node.value();
            let new_value = value + 1.0;
            *node = Formatted::new(new_value);
        }
    }
    #[test]
    fn test_visit_float_mut() {
        let _rug_st_tests_llm_16_577_llm_16_577_rrrruuuugggg_test_visit_float_mut = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 43.0;
        let mut float = Formatted::new(rug_fuzz_0);
        let mut visitor = Visitor;
        visitor.visit_float_mut(&mut float);
        let expected = Formatted::new(rug_fuzz_1);
        debug_assert_eq!(float.value(), expected.value());
        let _rug_ed_tests_llm_16_577_llm_16_577_rrrruuuugggg_test_visit_float_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_579 {
    use super::*;
    use crate::*;
    use crate::{Formatted, Decor};
    struct TestVisitor {
        called: bool,
    }
    impl TestVisitor {
        fn new() -> Self {
            Self { called: false }
        }
    }
    impl VisitMut for TestVisitor {
        fn visit_integer_mut(&mut self, _node: &mut Formatted<i64>) {
            self.called = true;
        }
    }
    #[test]
    fn test_visit_integer_mut() {
        let _rug_st_tests_llm_16_579_rrrruuuugggg_test_visit_integer_mut = 0;
        let rug_fuzz_0 = 123;
        let mut visitor = TestVisitor::new();
        let mut formatted_integer = Formatted::new(rug_fuzz_0);
        debug_assert!(! visitor.called);
        visitor.visit_integer_mut(&mut formatted_integer);
        debug_assert!(visitor.called);
        let _rug_ed_tests_llm_16_579_rrrruuuugggg_test_visit_integer_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_580 {
    use crate::visit_mut::VisitMut;
    use crate::Item;
    struct TestVisitor;
    impl VisitMut for TestVisitor {
        fn visit_item_mut(&mut self, _item: &mut Item) {
            unimplemented!();
        }
    }
    #[test]
    fn test_visit_item_mut() {
        let _rug_st_tests_llm_16_580_rrrruuuugggg_test_visit_item_mut = 0;
        let mut visitor = TestVisitor;
        let mut item = Item::None;
        visitor.visit_item_mut(&mut item);
        let _rug_ed_tests_llm_16_580_rrrruuuugggg_test_visit_item_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_581_llm_16_581 {
    use crate::visit_mut::VisitMut;
    use crate::repr::{Decor, Formatted};
    use crate::raw_string::RawString;
    struct MockVisitMut;
    impl VisitMut for MockVisitMut {
        fn visit_string_mut(&mut self, node: &mut Formatted<String>) {
            node.decor_mut().set_prefix(RawString::from("# "));
            node.decor_mut().set_suffix(RawString::from(" #"));
            node.fmt();
        }
    }
    #[test]
    fn test_visit_string_mut() {
        let _rug_st_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_visit_string_mut = 0;
        let rug_fuzz_0 = "value";
        let mut visit_mut = MockVisitMut;
        let mut node = Formatted::new(rug_fuzz_0.to_string());
        debug_assert_eq!(node.decor().prefix(), None);
        debug_assert_eq!(node.decor().suffix(), None);
        visit_mut.visit_string_mut(&mut node);
        debug_assert_eq!(node.decor().prefix().map(| s | s.as_str()), Some("# ".into()));
        debug_assert_eq!(node.decor().suffix().map(| s | s.as_str()), Some(" #".into()));
        debug_assert_eq!(node.value(), "value");
        let _rug_ed_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_visit_string_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_584_llm_16_584 {
    use crate::visit_mut::VisitMut;
    use crate::table::Table;
    use crate::table::KeyValuePairs;
    struct TestVisitor {
        visit_count: usize,
    }
    impl VisitMut for TestVisitor {
        fn visit_table_mut(&mut self, _node: &mut Table) {
            self.visit_count += 1;
        }
    }
    #[test]
    fn visit_table_mut_once() {
        let _rug_st_tests_llm_16_584_llm_16_584_rrrruuuugggg_visit_table_mut_once = 0;
        let rug_fuzz_0 = 0;
        let mut visitor = TestVisitor {
            visit_count: rug_fuzz_0,
        };
        let mut table = Table::new();
        visitor.visit_table_mut(&mut table);
        debug_assert_eq!(visitor.visit_count, 1);
        let _rug_ed_tests_llm_16_584_llm_16_584_rrrruuuugggg_visit_table_mut_once = 0;
    }
    #[test]
    fn visit_table_mut_twice() {
        let _rug_st_tests_llm_16_584_llm_16_584_rrrruuuugggg_visit_table_mut_twice = 0;
        let rug_fuzz_0 = 0;
        let mut visitor = TestVisitor {
            visit_count: rug_fuzz_0,
        };
        let mut table1 = Table::new();
        let mut table2 = Table::new();
        visitor.visit_table_mut(&mut table1);
        visitor.visit_table_mut(&mut table2);
        debug_assert_eq!(visitor.visit_count, 2);
        let _rug_ed_tests_llm_16_584_llm_16_584_rrrruuuugggg_visit_table_mut_twice = 0;
    }
    #[test]
    fn visit_table_mut_with_key_value_pairs() {
        let _rug_st_tests_llm_16_584_llm_16_584_rrrruuuugggg_visit_table_mut_with_key_value_pairs = 0;
        let rug_fuzz_0 = 0;
        let mut visitor = TestVisitor {
            visit_count: rug_fuzz_0,
        };
        let mut table = Table::with_pairs(KeyValuePairs::new());
        visitor.visit_table_mut(&mut table);
        debug_assert_eq!(visitor.visit_count, 1);
        let _rug_ed_tests_llm_16_584_llm_16_584_rrrruuuugggg_visit_table_mut_with_key_value_pairs = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_586 {
    use crate::{Array, Item, Value, visit_mut::VisitMut, visit_mut::visit_array_mut};
    struct ModifyArray;
    impl VisitMut for ModifyArray {
        fn visit_value_mut(&mut self, node: &mut Value) {
            *node = Value::from("modified");
        }
    }
    #[test]
    fn test_visit_array_mut() {
        let _rug_st_tests_llm_16_586_rrrruuuugggg_test_visit_array_mut = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "foo";
        let rug_fuzz_2 = 3.14;
        let mut array = Array::new();
        array.push(rug_fuzz_0);
        array.push(rug_fuzz_1);
        array.push(rug_fuzz_2);
        visit_array_mut(&mut ModifyArray, &mut array);
        for value in array.iter() {
            debug_assert_eq!(value.as_str(), Some("modified"));
        }
        debug_assert_eq!(array.len(), 3);
        let _rug_ed_tests_llm_16_586_rrrruuuugggg_test_visit_array_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_587_llm_16_587 {
    use super::*;
    use crate::*;
    use crate::{
        Item, Table, Value, Formatted, ArrayOfTables, Document, InlineTable, TableLike,
        KeyMut,
    };
    use crate::visit_mut::{visit_array_of_tables_mut, VisitMut};
    struct MockVisitor {
        visit_table_count: usize,
    }
    impl MockVisitor {
        fn new() -> Self {
            Self { visit_table_count: 0 }
        }
    }
    impl VisitMut for MockVisitor {
        fn visit_table_mut(&mut self, node: &mut Table) {
            self.visit_table_count += 1;
            node.insert(
                "key",
                Item::Value(Value::String(Formatted::new("value".to_string()))),
            );
        }
    }
    #[test]
    fn test_visit_array_of_tables_mut() {
        let _rug_st_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_visit_array_of_tables_mut = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = "key";
        let mut array_of_tables = ArrayOfTables::new();
        let mut table1 = Table::new();
        let mut table2 = Table::new();
        array_of_tables.push(table1);
        array_of_tables.push(table2);
        let mut visitor = MockVisitor::new();
        visit_array_of_tables_mut(&mut visitor, &mut array_of_tables);
        debug_assert_eq!(visitor.visit_table_count, 2);
        debug_assert!(
            array_of_tables.get(rug_fuzz_0).unwrap().get(rug_fuzz_1).is_some()
        );
        debug_assert!(
            array_of_tables.get(rug_fuzz_2).unwrap().get(rug_fuzz_3).is_some()
        );
        let _rug_ed_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_visit_array_of_tables_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_588 {
    use crate::visit_mut::VisitMut;
    use crate::Formatted;
    struct BooleanMutator;
    impl VisitMut for BooleanMutator {
        fn visit_boolean_mut(&mut self, node: &mut Formatted<bool>) {
            let new_value = !*node.value();
            *node = Formatted::new(new_value);
        }
    }
    #[test]
    fn test_visit_boolean_mut() {
        let _rug_st_tests_llm_16_588_rrrruuuugggg_test_visit_boolean_mut = 0;
        let rug_fuzz_0 = true;
        let mut boolean = Formatted::new(rug_fuzz_0);
        let mut visitor = BooleanMutator;
        visitor.visit_boolean_mut(&mut boolean);
        debug_assert_eq!(* boolean.value(), false);
        let _rug_ed_tests_llm_16_588_rrrruuuugggg_test_visit_boolean_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_589_llm_16_589 {
    use crate::visit_mut::VisitMut;
    use crate::{Formatted, Datetime, Decor};
    use std::str::FromStr;
    struct MockVisitor;
    impl VisitMut for MockVisitor {
        fn visit_datetime_mut(&mut self, node: &mut Formatted<Datetime>) {
            node.decor_mut().set_prefix("# ");
            node.decor_mut().set_suffix(" #");
        }
    }
    #[test]
    fn test_visit_datetime_mut_changes_decor() {
        let _rug_st_tests_llm_16_589_llm_16_589_rrrruuuugggg_test_visit_datetime_mut_changes_decor = 0;
        let rug_fuzz_0 = "1979-05-27T07:32:00Z";
        let mut datetime = Formatted::new(Datetime::from_str(rug_fuzz_0).unwrap());
        datetime.decor_mut().clear();
        let mut visitor = MockVisitor;
        visitor.visit_datetime_mut(&mut datetime);
        let prefix = datetime.decor().prefix().unwrap().as_str().unwrap();
        let suffix = datetime.decor().suffix().unwrap().as_str().unwrap();
        debug_assert_eq!(prefix, "# ");
        debug_assert_eq!(suffix, " #");
        let _rug_ed_tests_llm_16_589_llm_16_589_rrrruuuugggg_test_visit_datetime_mut_changes_decor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_594 {
    use crate::{
        Array, ArrayOfTables, Formatted, Item, Table, Value,
        visit_mut::{VisitMut, visit_item_mut},
    };
    struct MutVisitor;
    impl VisitMut for MutVisitor {
        fn visit_table_mut(&mut self, node: &mut Table) {
            node.fmt();
        }
        fn visit_array_mut(&mut self, node: &mut Array) {
            node.fmt();
        }
        fn visit_array_of_tables_mut(&mut self, node: &mut ArrayOfTables) {
            for table in node.iter_mut() {
                table.fmt();
            }
        }
        fn visit_value_mut(&mut self, node: &mut Value) {
            if let Value::String(formatted) = node {
                formatted.fmt();
            }
        }
    }
    #[test]
    fn test_visit_item_mut_table() {
        let _rug_st_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_table = 0;
        let rug_fuzz_0 = "should be a table after visit";
        let mut node = Item::Table(Table::new());
        let mut visitor = MutVisitor;
        visit_item_mut(&mut visitor, &mut node);
        let table = node.as_table().expect(rug_fuzz_0);
        debug_assert_eq!(table.decor().prefix().is_none(), true);
        debug_assert_eq!(table.decor().suffix().is_none(), true);
        let _rug_ed_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_table = 0;
    }
    #[test]
    fn test_visit_item_mut_array() {
        let _rug_st_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_array = 0;
        let rug_fuzz_0 = "should be a value after visit";
        let rug_fuzz_1 = "should be an array";
        let mut node = Item::Value(Value::Array(Array::new()));
        let mut visitor = MutVisitor;
        visit_item_mut(&mut visitor, &mut node);
        let array = node.as_value().expect(rug_fuzz_0).as_array().expect(rug_fuzz_1);
        debug_assert_eq!(array.decor().prefix().is_none(), true);
        debug_assert_eq!(array.decor().suffix().is_none(), true);
        let _rug_ed_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_array = 0;
    }
    #[test]
    fn test_visit_item_mut_array_of_tables() {
        let _rug_st_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_array_of_tables = 0;
        let rug_fuzz_0 = "should be an array of tables after visit";
        let mut node = Item::ArrayOfTables(ArrayOfTables::new());
        let mut visitor = MutVisitor;
        visit_item_mut(&mut visitor, &mut node);
        let array_of_tables = node.as_array_of_tables().expect(rug_fuzz_0);
        for table in array_of_tables.iter() {
            debug_assert_eq!(table.decor().prefix().is_none(), true);
            debug_assert_eq!(table.decor().suffix().is_none(), true);
        }
        let _rug_ed_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_array_of_tables = 0;
    }
    #[test]
    fn test_visit_item_mut_value() {
        let _rug_st_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_value = 0;
        let rug_fuzz_0 = "value";
        let mut node = Item::Value(
            Value::String(Formatted::new(rug_fuzz_0.to_string())),
        );
        let mut visitor = MutVisitor;
        visit_item_mut(&mut visitor, &mut node);
        if let Item::Value(Value::String(formatted)) = &node {
            debug_assert!(formatted.as_repr().is_some());
        } else {
            panic!("should be a formatted string after visit")
        }
        let _rug_ed_tests_llm_16_594_rrrruuuugggg_test_visit_item_mut_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_596_llm_16_596 {
    use crate::{visit_mut::VisitMut, Item, Key, visit_mut::visit_table_like_kv_mut};
    struct TestVisitor;
    impl VisitMut for TestVisitor {
        fn visit_item_mut(&mut self, node: &mut Item) {
            if node.is_value() {
                if let Some(value) = node.as_value_mut() {
                    *value = crate::Value::from("visited");
                }
            }
        }
    }
    #[test]
    fn test_visit_table_like_kv_mut() {
        let _rug_st_tests_llm_16_596_llm_16_596_rrrruuuugggg_test_visit_table_like_kv_mut = 0;
        let rug_fuzz_0 = "initial";
        let rug_fuzz_1 = "key";
        let mut item = Item::Value(crate::Value::from(rug_fuzz_0));
        let mut visitor = TestVisitor {};
        let mut key = Key::new(rug_fuzz_1);
        visit_table_like_kv_mut(&mut visitor, key.as_mut(), &mut item);
        debug_assert!(item.is_value());
        debug_assert_eq!(item.as_value().unwrap().as_str(), Some("visited"));
        let _rug_ed_tests_llm_16_596_llm_16_596_rrrruuuugggg_test_visit_table_like_kv_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_598 {
    use crate::{Table, Document, Item, visit_mut::{VisitMut, visit_table_mut}};
    struct TestVisitor {
        visited: bool,
    }
    impl VisitMut for TestVisitor {
        fn visit_table_like_mut(&mut self, _node: &mut dyn crate::TableLike) {
            self.visited = true;
        }
    }
    #[test]
    fn visit_table_mut_test() {
        let _rug_st_tests_llm_16_598_rrrruuuugggg_visit_table_mut_test = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = false;
        let mut table = Table::new();
        table.insert(rug_fuzz_0, Item::Value(rug_fuzz_1.parse().unwrap()));
        let mut visitor = TestVisitor { visited: rug_fuzz_2 };
        visit_table_mut(&mut visitor, &mut table);
        debug_assert!(visitor.visited);
        let _rug_ed_tests_llm_16_598_rrrruuuugggg_visit_table_mut_test = 0;
    }
}
