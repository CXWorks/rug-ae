use std::str::FromStr;
use toml_datetime::*;
use crate::array_of_tables::ArrayOfTables;
use crate::table::TableLike;
use crate::{Array, InlineTable, Table, Value};
/// Type representing either a value, a table, an array of tables, or none.
#[derive(Debug, Clone)]
pub enum Item {
    /// Type representing none.
    None,
    /// Type representing value.
    Value(Value),
    /// Type representing table.
    Table(Table),
    /// Type representing array of tables.
    ArrayOfTables(ArrayOfTables),
}
impl Item {
    /// Sets `self` to the given item iff `self` is none and
    /// returns a mutable reference to `self`.
    pub fn or_insert(&mut self, item: Item) -> &mut Item {
        if self.is_none() {
            *self = item;
        }
        self
    }
}
/// Downcasting
impl Item {
    /// Text description of value type
    pub fn type_name(&self) -> &'static str {
        match self {
            Item::None => "none",
            Item::Value(v) => v.type_name(),
            Item::Table(..) => "table",
            Item::ArrayOfTables(..) => "array of tables",
        }
    }
    /// Index into a TOML array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if:
    /// - The type of `self` does not match the type of the
    ///   index, for example if the index is a string and `self` is an array or a
    ///   number.
    /// - The given key does not exist in the map
    ///   or the given index is not within the bounds of the array.
    pub fn get<I: crate::index::Index>(&self, index: I) -> Option<&Item> {
        index.index(self)
    }
    /// Mutably index into a TOML array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if:
    /// - The type of `self` does not match the type of the
    ///   index, for example if the index is a string and `self` is an array or a
    ///   number.
    /// - The given key does not exist in the map
    ///   or the given index is not within the bounds of the array.
    pub fn get_mut<I: crate::index::Index>(&mut self, index: I) -> Option<&mut Item> {
        index.index_mut(self)
    }
    /// Casts `self` to value.
    pub fn as_value(&self) -> Option<&Value> {
        match *self {
            Item::Value(ref v) => Some(v),
            _ => None,
        }
    }
    /// Casts `self` to table.
    pub fn as_table(&self) -> Option<&Table> {
        match *self {
            Item::Table(ref t) => Some(t),
            _ => None,
        }
    }
    /// Casts `self` to array of tables.
    pub fn as_array_of_tables(&self) -> Option<&ArrayOfTables> {
        match *self {
            Item::ArrayOfTables(ref a) => Some(a),
            _ => None,
        }
    }
    /// Casts `self` to mutable value.
    pub fn as_value_mut(&mut self) -> Option<&mut Value> {
        match *self {
            Item::Value(ref mut v) => Some(v),
            _ => None,
        }
    }
    /// Casts `self` to mutable table.
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match *self {
            Item::Table(ref mut t) => Some(t),
            _ => None,
        }
    }
    /// Casts `self` to mutable array of tables.
    pub fn as_array_of_tables_mut(&mut self) -> Option<&mut ArrayOfTables> {
        match *self {
            Item::ArrayOfTables(ref mut a) => Some(a),
            _ => None,
        }
    }
    /// Casts `self` to value.
    pub fn into_value(self) -> Result<Value, Self> {
        match self {
            Item::None => Err(self),
            Item::Value(v) => Ok(v),
            Item::Table(v) => {
                let v = v.into_inline_table();
                Ok(Value::InlineTable(v))
            }
            Item::ArrayOfTables(v) => {
                let v = v.into_array();
                Ok(Value::Array(v))
            }
        }
    }
    /// In-place convert to a value
    pub fn make_value(&mut self) {
        let other = std::mem::take(self);
        let other = other.into_value().map(Item::Value).unwrap_or(Item::None);
        *self = other;
    }
    /// Casts `self` to table.
    pub fn into_table(self) -> Result<Table, Self> {
        match self {
            Item::Table(t) => Ok(t),
            Item::Value(Value::InlineTable(t)) => Ok(t.into_table()),
            _ => Err(self),
        }
    }
    /// Casts `self` to array of tables.
    pub fn into_array_of_tables(self) -> Result<ArrayOfTables, Self> {
        match self {
            Item::ArrayOfTables(a) => Ok(a),
            Item::Value(Value::Array(a)) => {
                if a.is_empty() {
                    Err(Item::Value(Value::Array(a)))
                } else if a.iter().all(|v| v.is_inline_table()) {
                    let mut aot = ArrayOfTables::new();
                    aot.values = a.values;
                    for value in aot.values.iter_mut() {
                        value.make_item();
                    }
                    Ok(aot)
                } else {
                    Err(Item::Value(Value::Array(a)))
                }
            }
            _ => Err(self),
        }
    }
    pub(crate) fn make_item(&mut self) {
        let other = std::mem::take(self);
        let other = match other.into_table().map(crate::Item::Table) {
            Ok(i) => i,
            Err(i) => i,
        };
        let other = match other.into_array_of_tables().map(crate::Item::ArrayOfTables) {
            Ok(i) => i,
            Err(i) => i,
        };
        *self = other;
    }
    /// Returns true iff `self` is a value.
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }
    /// Returns true iff `self` is a table.
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }
    /// Returns true iff `self` is an array of tables.
    pub fn is_array_of_tables(&self) -> bool {
        self.as_array_of_tables().is_some()
    }
    /// Returns true iff `self` is `None`.
    pub fn is_none(&self) -> bool {
        matches!(* self, Item::None)
    }
    /// Casts `self` to integer.
    pub fn as_integer(&self) -> Option<i64> {
        self.as_value().and_then(Value::as_integer)
    }
    /// Returns true iff `self` is an integer.
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }
    /// Casts `self` to float.
    pub fn as_float(&self) -> Option<f64> {
        self.as_value().and_then(Value::as_float)
    }
    /// Returns true iff `self` is a float.
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }
    /// Casts `self` to boolean.
    pub fn as_bool(&self) -> Option<bool> {
        self.as_value().and_then(Value::as_bool)
    }
    /// Returns true iff `self` is a boolean.
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }
    /// Casts `self` to str.
    pub fn as_str(&self) -> Option<&str> {
        self.as_value().and_then(Value::as_str)
    }
    /// Returns true iff `self` is a string.
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }
    /// Casts `self` to date-time.
    pub fn as_datetime(&self) -> Option<&Datetime> {
        self.as_value().and_then(Value::as_datetime)
    }
    /// Returns true iff `self` is a date-time.
    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }
    /// Casts `self` to array.
    pub fn as_array(&self) -> Option<&Array> {
        self.as_value().and_then(Value::as_array)
    }
    /// Casts `self` to mutable array.
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        self.as_value_mut().and_then(Value::as_array_mut)
    }
    /// Returns true iff `self` is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    /// Casts `self` to inline table.
    pub fn as_inline_table(&self) -> Option<&InlineTable> {
        self.as_value().and_then(Value::as_inline_table)
    }
    /// Casts `self` to mutable inline table.
    pub fn as_inline_table_mut(&mut self) -> Option<&mut InlineTable> {
        self.as_value_mut().and_then(Value::as_inline_table_mut)
    }
    /// Returns true iff `self` is an inline table.
    pub fn is_inline_table(&self) -> bool {
        self.as_inline_table().is_some()
    }
    /// Casts `self` to either a table or an inline table.
    pub fn as_table_like(&self) -> Option<&dyn TableLike> {
        self.as_table()
            .map(|t| t as &dyn TableLike)
            .or_else(|| self.as_inline_table().map(|t| t as &dyn TableLike))
    }
    /// Casts `self` to either a table or an inline table.
    pub fn as_table_like_mut(&mut self) -> Option<&mut dyn TableLike> {
        match self {
            Item::Table(t) => Some(t as &mut dyn TableLike),
            Item::Value(Value::InlineTable(t)) => Some(t as &mut dyn TableLike),
            _ => None,
        }
    }
    /// Returns true iff `self` is either a table, or an inline table.
    pub fn is_table_like(&self) -> bool {
        self.as_table_like().is_some()
    }
    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        match self {
            Item::None => None,
            Item::Value(v) => v.span(),
            Item::Table(v) => v.span(),
            Item::ArrayOfTables(v) => v.span(),
        }
    }
    pub(crate) fn despan(&mut self, input: &str) {
        match self {
            Item::None => {}
            Item::Value(v) => v.despan(input),
            Item::Table(v) => v.despan(input),
            Item::ArrayOfTables(v) => v.despan(input),
        }
    }
}
impl Default for Item {
    fn default() -> Self {
        Item::None
    }
}
impl FromStr for Item {
    type Err = crate::TomlError;
    /// Parses a value from a &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<Value>()?;
        Ok(Item::Value(value))
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Item::None => Ok(()),
            Item::Value(v) => v.fmt(f),
            Item::Table(v) => v.fmt(f),
            Item::ArrayOfTables(v) => v.fmt(f),
        }
    }
}
/// Returns a formatted value.
///
/// Since formatting is part of a `Value`, the right hand side of the
/// assignment needs to be decorated with a space before the value.
/// The `value` function does just that.
///
/// # Examples
/// ```rust
/// # use snapbox::assert_eq;
/// # use toml_edit::*;
/// let mut table = Table::default();
/// let mut array = Array::default();
/// array.push("hello");
/// array.push("\\, world"); // \ is only allowed in a literal string
/// table["key1"] = value("value1");
/// table["key2"] = value(42);
/// table["key3"] = value(array);
/// assert_eq(table.to_string(),
/// r#"key1 = "value1"
/// key2 = 42
/// key3 = ["hello", '\, world']
/// "#);
/// ```
pub fn value<V: Into<Value>>(v: V) -> Item {
    Item::Value(v.into())
}
/// Returns an empty table.
pub fn table() -> Item {
    Item::Table(Table::new())
}
/// Returns an empty array of tables.
pub fn array() -> Item {
    Item::ArrayOfTables(ArrayOfTables::new())
}
#[cfg(test)]
mod tests_llm_16_52 {
    use crate::Item;
    #[test]
    fn test_item_default() {
        let _rug_st_tests_llm_16_52_rrrruuuugggg_test_item_default = 0;
        let default_item = Item::default();
        debug_assert!(
            matches!(default_item, Item::None),
            "Item::default() did not return Item::None"
        );
        let _rug_ed_tests_llm_16_52_rrrruuuugggg_test_item_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_53 {
    use crate::{Item, value::Value};
    #[test]
    fn test_from_str_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = rug_fuzz_0;
        let item_result = <Item as std::str::FromStr>::from_str(s);
        debug_assert!(item_result.is_ok());
        let item = item_result.unwrap();
        if let Item::Value(value) = item {
            debug_assert_eq!(value.as_str(), Some("value"));
        } else {
            panic!("Expected Item::Value but got {:?}", item);
        }
             }
});    }
    #[test]
    fn test_from_str_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = rug_fuzz_0;
        let item_result = <Item as std::str::FromStr>::from_str(s);
        debug_assert!(item_result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_260_llm_16_260 {
    use super::*;
    use crate::*;
    /// Test if `as_array` correctly returns `Some` when `Item` is an `Array`.
    #[test]
    fn item_as_array_some() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array = Array::new();
        array.push(rug_fuzz_0);
        let item = Item::Value(Value::Array(array));
        debug_assert!(item.as_array().is_some());
             }
});    }
    /// Test if `as_array` correctly returns `None` when `Item` is not an `Array`.
    #[test]
    fn item_as_array_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let item = Item::Value(Value::Integer(Formatted::new(rug_fuzz_0)));
        debug_assert!(item.as_array().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_261_llm_16_261 {
    use crate::{Item, Value, Array};
    #[test]
    fn test_as_array_mut() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut item = Item::Value(Value::Array(Array::default()));
        debug_assert!(item.as_array_mut().is_some());
        let mut item = Item::Value(Value::Array(Array::new()));
        debug_assert!(item.as_array_mut().is_some());
        let mut item = Item::None;
        debug_assert!(item.as_array_mut().is_none());
        let mut item = Item::Value(Value::from(rug_fuzz_0));
        debug_assert!(item.as_array_mut().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_262 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn test_as_array_of_tables_some() {
        let _rug_st_tests_llm_16_262_rrrruuuugggg_test_as_array_of_tables_some = 0;
        let mut aot = ArrayOfTables::new();
        aot.push(Table::new());
        let item = Item::ArrayOfTables(aot);
        debug_assert!(item.as_array_of_tables().is_some());
        let _rug_ed_tests_llm_16_262_rrrruuuugggg_test_as_array_of_tables_some = 0;
    }
    #[test]
    fn test_as_array_of_tables_none() {
        let _rug_st_tests_llm_16_262_rrrruuuugggg_test_as_array_of_tables_none = 0;
        let item = Item::Value(Value::String(Formatted::new(String::new())));
        debug_assert!(item.as_array_of_tables().is_none());
        let _rug_ed_tests_llm_16_262_rrrruuuugggg_test_as_array_of_tables_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_263 {
    use crate::{Item, ArrayOfTables, Table, Value};
    #[test]
    fn test_as_array_of_tables_mut_when_array_of_tables() {
        let _rug_st_tests_llm_16_263_rrrruuuugggg_test_as_array_of_tables_mut_when_array_of_tables = 0;
        let mut item = Item::ArrayOfTables(ArrayOfTables::new());
        let aot_mut = item.as_array_of_tables_mut();
        debug_assert!(aot_mut.is_some());
        let _rug_ed_tests_llm_16_263_rrrruuuugggg_test_as_array_of_tables_mut_when_array_of_tables = 0;
    }
    #[test]
    fn test_as_array_of_tables_mut_when_not_array_of_tables() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut item = Item::Table(Table::new());
        let aot_mut = item.as_array_of_tables_mut();
        debug_assert!(aot_mut.is_none());
        let mut item = Item::Value(Value::from(rug_fuzz_0));
        let aot_mut = item.as_array_of_tables_mut();
        debug_assert!(aot_mut.is_none());
        let mut item = Item::None;
        let aot_mut = item.as_array_of_tables_mut();
        debug_assert!(aot_mut.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_266 {
    use crate::{Formatted, Item, Value};
    #[test]
    fn as_float_from_float_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = Value::Float(Formatted::new(rug_fuzz_0));
        let item = Item::Value(val);
        debug_assert_eq!(item.as_float(), Some(42.0));
             }
});    }
    #[test]
    fn as_float_from_non_float_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = Value::Integer(Formatted::new(rug_fuzz_0));
        let item = Item::Value(val);
        debug_assert!(item.as_float().is_none());
             }
});    }
    #[test]
    fn as_float_from_none() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_as_float_from_none = 0;
        let item = Item::None;
        debug_assert!(item.as_float().is_none());
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_as_float_from_none = 0;
    }
    #[test]
    fn as_float_from_table() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_as_float_from_table = 0;
        let table = crate::table::Table::new();
        let item = Item::Table(table);
        debug_assert!(item.as_float().is_none());
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_as_float_from_table = 0;
    }
    #[test]
    fn as_float_from_array_of_tables() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_as_float_from_array_of_tables = 0;
        let array_of_tables = crate::array_of_tables::ArrayOfTables::new();
        let item = Item::ArrayOfTables(array_of_tables);
        debug_assert!(item.as_float().is_none());
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_as_float_from_array_of_tables = 0;
    }
    #[test]
    fn as_float_from_array() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_as_float_from_array = 0;
        let array = crate::array::Array::new();
        let item = Item::Value(Value::Array(array));
        debug_assert!(item.as_float().is_none());
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_as_float_from_array = 0;
    }
    #[test]
    fn as_float_from_inline_table() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_as_float_from_inline_table = 0;
        let inline_table = crate::inline_table::InlineTable::new();
        let item = Item::Value(Value::InlineTable(inline_table));
        debug_assert!(item.as_float().is_none());
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_as_float_from_inline_table = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_267 {
    use crate::item::Item;
    use crate::inline_table::InlineTable;
    use crate::value::Value;
    #[test]
    fn test_as_inline_table_on_inline_table_item() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        let key = rug_fuzz_0.to_string();
        let value = Value::from(rug_fuzz_1);
        inline_table.insert(key, value);
        let item = Item::Value(Value::InlineTable(inline_table));
        let result = item.as_inline_table();
        debug_assert!(result.is_some());
             }
});    }
    #[test]
    fn test_as_inline_table_on_non_inline_table_item() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let item = Item::Value(Value::from(rug_fuzz_0));
        let result = item.as_inline_table();
        debug_assert!(result.is_none());
             }
});    }
    #[test]
    fn test_as_inline_table_on_non_value_item() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_as_inline_table_on_non_value_item = 0;
        let item = Item::None;
        let result = item.as_inline_table();
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_as_inline_table_on_non_value_item = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_270 {
    use crate::{Item, Value};
    #[test]
    fn test_item_as_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val_str = rug_fuzz_0;
        let val_item = Item::Value(Value::from(val_str));
        debug_assert_eq!(val_item.as_str(), Some(val_str));
        let table_item = Item::Table(Default::default());
        debug_assert_eq!(table_item.as_str(), None);
        let aot_item = Item::ArrayOfTables(Default::default());
        debug_assert_eq!(aot_item.as_str(), None);
        let none_item = Item::None;
        debug_assert_eq!(none_item.as_str(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_271 {
    use crate::{Item, Table};
    #[test]
    fn as_table_from_table_item() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(rug_fuzz_1.into());
        let item = Item::Table(table);
        let table_ref = item.as_table();
        debug_assert!(table_ref.is_some());
        debug_assert_eq!(table_ref.unwrap() [rug_fuzz_2].as_integer(), Some(42));
             }
});    }
    #[test]
    fn as_table_from_non_table_item() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let item = Item::Value(rug_fuzz_0.into());
        let table_ref = item.as_table();
        debug_assert!(table_ref.is_none());
             }
});    }
    #[test]
    fn as_table_from_none_item() {
        let _rug_st_tests_llm_16_271_rrrruuuugggg_as_table_from_none_item = 0;
        let item = Item::None;
        let table_ref = item.as_table();
        debug_assert!(table_ref.is_none());
        let _rug_ed_tests_llm_16_271_rrrruuuugggg_as_table_from_none_item = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_272_llm_16_272 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_table_like_on_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        table
            .insert(
                rug_fuzz_0,
                Item::Value(Value::String(Formatted::new(rug_fuzz_1.to_owned()))),
            );
        let item = Item::Table(table);
        debug_assert!(item.as_table_like().is_some());
             }
});    }
    #[test]
    fn test_as_table_like_on_inline_table() {
        let _rug_st_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_table_like_on_inline_table = 0;
        let inline_table = InlineTable::new();
        let item = Item::Value(Value::InlineTable(inline_table));
        debug_assert!(item.as_table_like().is_some());
        let _rug_ed_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_table_like_on_inline_table = 0;
    }
    #[test]
    fn test_as_table_like_on_array() {
        let _rug_st_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_table_like_on_array = 0;
        let array = Array::new();
        let item = Item::Value(Value::Array(array));
        debug_assert!(item.as_table_like().is_none());
        let _rug_ed_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_table_like_on_array = 0;
    }
    #[test]
    fn test_as_table_like_on_none() {
        let _rug_st_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_table_like_on_none = 0;
        let item = Item::None;
        debug_assert!(item.as_table_like().is_none());
        let _rug_ed_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_table_like_on_none = 0;
    }
    #[test]
    fn test_as_table_like_on_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(Formatted::new(rug_fuzz_0.to_owned()));
        let item = Item::Value(value);
        debug_assert!(item.as_table_like().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_274 {
    use super::*;
    use crate::*;
    use crate::item::Item;
    use crate::table::Table;
    #[test]
    fn test_as_table_mut_none() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_as_table_mut_none = 0;
        let mut item = Item::None;
        debug_assert!(item.as_table_mut().is_none());
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_as_table_mut_none = 0;
    }
    #[test]
    fn test_as_table_mut_table() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_as_table_mut_table = 0;
        let mut table = Table::new();
        let mut item = Item::Table(table);
        debug_assert!(item.as_table_mut().is_some());
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_as_table_mut_table = 0;
    }
    #[test]
    fn test_as_table_mut_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut item = Item::Value(Value::String(Formatted::new(rug_fuzz_0.into())));
        debug_assert!(item.as_table_mut().is_none());
             }
});    }
    #[test]
    fn test_as_table_mut_array_of_tables() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_as_table_mut_array_of_tables = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let mut item = Item::ArrayOfTables(array_of_tables);
        debug_assert!(item.as_table_mut().is_none());
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_as_table_mut_array_of_tables = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_280 {
    use super::*;
    use crate::*;
    #[test]
    fn test_into_array_of_tables_ok_with_array_of_inline_tables() {
        let _rug_st_tests_llm_16_280_rrrruuuugggg_test_into_array_of_tables_ok_with_array_of_inline_tables = 0;
        use crate::{Item, Table, Value};
        let mut array = Array::new();
        array.push(InlineTable::new());
        let array_item = Item::Value(Value::Array(array));
        let array_of_tables_result = array_item.clone().into_array_of_tables();
        debug_assert!(array_of_tables_result.is_ok());
        let array_of_tables = array_of_tables_result.unwrap();
        debug_assert_eq!(array_of_tables.iter().count(), 1);
        let _rug_ed_tests_llm_16_280_rrrruuuugggg_test_into_array_of_tables_ok_with_array_of_inline_tables = 0;
    }
    #[test]
    fn test_into_array_of_tables_ok_with_item_array_of_tables() {
        let _rug_st_tests_llm_16_280_rrrruuuugggg_test_into_array_of_tables_ok_with_item_array_of_tables = 0;
        use crate::ArrayOfTables;
        let array_of_tables_item = Item::ArrayOfTables(ArrayOfTables::new());
        let array_of_tables_result = array_of_tables_item.clone().into_array_of_tables();
        debug_assert!(array_of_tables_result.is_ok());
        let _rug_ed_tests_llm_16_280_rrrruuuugggg_test_into_array_of_tables_ok_with_item_array_of_tables = 0;
    }
    #[test]
    fn test_into_array_of_tables_err_empty_array() {
        let _rug_st_tests_llm_16_280_rrrruuuugggg_test_into_array_of_tables_err_empty_array = 0;
        use crate::{Item, Value};
        let empty_array_item = Item::Value(Value::Array(Array::new()));
        let array_of_tables_result = empty_array_item.clone().into_array_of_tables();
        debug_assert!(array_of_tables_result.is_err());
        let _rug_ed_tests_llm_16_280_rrrruuuugggg_test_into_array_of_tables_err_empty_array = 0;
    }
    #[test]
    fn test_into_array_of_tables_err_with_mixed_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::{Array, InlineTable, Item, Value};
        let mut mixed_array = Array::new();
        mixed_array.push(InlineTable::new());
        mixed_array.push(rug_fuzz_0);
        let mixed_array_item = Item::Value(Value::Array(mixed_array));
        let array_of_tables_result = mixed_array_item.clone().into_array_of_tables();
        debug_assert!(array_of_tables_result.is_err());
             }
});    }
    #[test]
    fn test_into_array_of_tables_err_with_non_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_array_item = Item::Value(Value::Boolean(Formatted::new(rug_fuzz_0)));
        let array_of_tables_result = non_array_item.clone().into_array_of_tables();
        debug_assert!(array_of_tables_result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_281 {
    use crate::{Array, Formatted, InlineTable, InternalString, Item, Table, Value};
    #[test]
    fn test_into_table_from_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Table::new();
        let key = InternalString::from(rug_fuzz_0);
        let value = Value::String(Formatted::new(String::from(rug_fuzz_1)));
        table.insert(&key, Item::Value(value));
        let item = Item::Table(table);
        let table_result = item.into_table();
        debug_assert!(table_result.is_ok());
        debug_assert!(table_result.unwrap().contains_key(& key));
             }
});    }
    #[test]
    fn test_into_table_from_inline_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inline_table = InlineTable::new();
        inline_table
            .insert(
                InternalString::from(rug_fuzz_0),
                Value::String(Formatted::new(String::from(rug_fuzz_1))),
            );
        let item = Item::Value(Value::InlineTable(inline_table));
        let table_result = item.into_table();
        debug_assert!(table_result.is_ok());
        debug_assert!(
            table_result.unwrap().contains_key(& InternalString::from(rug_fuzz_2))
        );
             }
});    }
    #[test]
    fn test_into_table_from_none() {
        let _rug_st_tests_llm_16_281_rrrruuuugggg_test_into_table_from_none = 0;
        let item = Item::None;
        let table_result = item.into_table();
        debug_assert!(table_result.is_err());
        let _rug_ed_tests_llm_16_281_rrrruuuugggg_test_into_table_from_none = 0;
    }
    #[test]
    fn test_into_table_from_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let item = Item::Value(Value::String(Formatted::new(String::from(rug_fuzz_0))));
        let table_result = item.into_table();
        debug_assert!(table_result.is_err());
             }
});    }
    #[test]
    fn test_into_table_from_array_of_tables() {
        let _rug_st_tests_llm_16_281_rrrruuuugggg_test_into_table_from_array_of_tables = 0;
        let array_of_tables = Array::new();
        let item = Item::Value(Value::Array(array_of_tables));
        let table_result = item.into_table();
        debug_assert!(table_result.is_err());
        let _rug_ed_tests_llm_16_281_rrrruuuugggg_test_into_table_from_array_of_tables = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_283 {
    use crate::{Item, Value, Array, InlineTable, Table, ArrayOfTables};
    fn create_array() -> Item {
        let mut array = Array::new();
        array.push(1);
        array.push(2);
        Item::Value(Value::Array(array))
    }
    fn create_inline_table() -> Item {
        let mut table = InlineTable::new();
        table.insert("key", Value::from("value"));
        Item::Value(Value::InlineTable(table))
    }
    fn create_table() -> Item {
        let mut table = Table::new();
        table.insert("key", Item::Value(Value::from("value")));
        Item::Table(table)
    }
    fn create_array_of_tables() -> Item {
        let mut array = ArrayOfTables::new();
        let mut table = Table::new();
        table.insert("key", Item::Value(Value::from("value")));
        array.push(table);
        Item::ArrayOfTables(array)
    }
    #[test]
    fn test_is_array_with_array() {
        let item = create_array();
        assert!(item.is_array());
    }
    #[test]
    fn test_is_array_with_inline_table() {
        let item = create_inline_table();
        assert!(! item.is_array());
    }
    #[test]
    fn test_is_array_with_table() {
        let item = create_table();
        assert!(! item.is_array());
    }
    #[test]
    fn test_is_array_with_array_of_tables() {
        let item = create_array_of_tables();
        assert!(! item.is_array());
    }
    #[test]
    fn test_is_array_with_none() {
        let item = Item::None;
        assert!(! item.is_array());
    }
}
#[cfg(test)]
mod tests_llm_16_284 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Formatted;
    #[test]
    fn test_is_array_of_tables_for_none() {
        let _rug_st_tests_llm_16_284_rrrruuuugggg_test_is_array_of_tables_for_none = 0;
        let item_none = Item::None;
        debug_assert!(! item_none.is_array_of_tables());
        let _rug_ed_tests_llm_16_284_rrrruuuugggg_test_is_array_of_tables_for_none = 0;
    }
    #[test]
    fn test_is_array_of_tables_for_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let item_value = Item::Value(
            Value::String(Formatted::new(rug_fuzz_0.to_owned())),
        );
        debug_assert!(! item_value.is_array_of_tables());
             }
});    }
    #[test]
    fn test_is_array_of_tables_for_table() {
        let _rug_st_tests_llm_16_284_rrrruuuugggg_test_is_array_of_tables_for_table = 0;
        let item_table = Item::Table(Table::new());
        debug_assert!(! item_table.is_array_of_tables());
        let _rug_ed_tests_llm_16_284_rrrruuuugggg_test_is_array_of_tables_for_table = 0;
    }
    #[test]
    fn test_is_array_of_tables_for_array_of_tables() {
        let _rug_st_tests_llm_16_284_rrrruuuugggg_test_is_array_of_tables_for_array_of_tables = 0;
        let item_array_of_tables = Item::ArrayOfTables(ArrayOfTables::new());
        debug_assert!(item_array_of_tables.is_array_of_tables());
        let _rug_ed_tests_llm_16_284_rrrruuuugggg_test_is_array_of_tables_for_array_of_tables = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_285 {
    use crate::Item;
    use crate::Value;
    #[test]
    fn test_item_is_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(bool, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bool_item = Item::Value(
            Value::Boolean(crate::repr::Formatted::new(rug_fuzz_0)),
        );
        let int_item = Item::Value(
            Value::Integer(crate::repr::Formatted::new(rug_fuzz_1)),
        );
        let string_item = Item::Value(
            Value::String(crate::repr::Formatted::new(rug_fuzz_2.to_owned())),
        );
        let array_item = Item::Value(Value::Array(crate::Array::new()));
        let table_item = Item::Value(Value::InlineTable(crate::InlineTable::new()));
        let none_item = Item::None;
        debug_assert!(bool_item.is_bool());
        debug_assert!(! int_item.is_bool());
        debug_assert!(! string_item.is_bool());
        debug_assert!(! array_item.is_bool());
        debug_assert!(! table_item.is_bool());
        debug_assert!(! none_item.is_bool());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_286 {
    use super::*;
    use crate::*;
    use crate::{Item, Value};
    #[test]
    fn test_item_is_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, i64, f64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_string = rug_fuzz_0;
        let datetime_value: Value = datetime_string.parse().unwrap();
        let datetime_item = Item::Value(datetime_value);
        debug_assert!(datetime_item.is_datetime());
        let string_value = Value::from(rug_fuzz_1);
        let string_item = Item::Value(string_value);
        debug_assert!(! string_item.is_datetime());
        let integer_value = Value::from(rug_fuzz_2);
        let integer_item = Item::Value(integer_value);
        debug_assert!(! integer_item.is_datetime());
        let float_value = Value::from(rug_fuzz_3);
        let float_item = Item::Value(float_value);
        debug_assert!(! float_item.is_datetime());
        let boolean_value = Value::from(rug_fuzz_4);
        let boolean_item = Item::Value(boolean_value);
        debug_assert!(! boolean_item.is_datetime());
        let array_value = Value::Array(Array::default());
        let array_item = Item::Value(array_value);
        debug_assert!(! array_item.is_datetime());
        let table_value = Value::InlineTable(InlineTable::default());
        let table_item = Item::Value(table_value);
        debug_assert!(! table_item.is_datetime());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_287 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::repr::Formatted;
    use std::str::FromStr;
    #[test]
    fn test_is_float() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let float_value = Value::Float(Formatted::new(rug_fuzz_0));
        let non_float_values = vec![
            Value::Array(Array::new()), Value::Boolean(Formatted::new(true)),
            Value::Datetime(Formatted::new(Datetime::from_str("1979-05-27T07:32:00Z")
            .unwrap())), Value::InlineTable(InlineTable::new()),
            Value::Integer(Formatted::new(42)), Value::String(Formatted::new("test"
            .to_string()))
        ];
        debug_assert!(Item::Value(float_value).is_float());
        for non_float_value in non_float_values {
            debug_assert!(! Item::Value(non_float_value).is_float());
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_288 {
    use crate::item::Item;
    use crate::value::Value;
    #[test]
    fn test_is_inline_table_on_inline_table() {
        let _rug_st_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_inline_table = 0;
        let inline_table = Item::Value(Value::InlineTable(Default::default()));
        debug_assert!(inline_table.is_inline_table());
        let _rug_ed_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_inline_table = 0;
    }
    #[test]
    fn test_is_inline_table_on_array() {
        let _rug_st_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_array = 0;
        let array = Item::Value(Value::Array(Default::default()));
        debug_assert!(! array.is_inline_table());
        let _rug_ed_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_array = 0;
    }
    #[test]
    fn test_is_inline_table_on_array_of_tables() {
        let _rug_st_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_array_of_tables = 0;
        let array_of_tables = Item::ArrayOfTables(Default::default());
        debug_assert!(! array_of_tables.is_inline_table());
        let _rug_ed_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_array_of_tables = 0;
    }
    #[test]
    fn test_is_inline_table_on_table() {
        let _rug_st_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_table = 0;
        let table = Item::Table(Default::default());
        debug_assert!(! table.is_inline_table());
        let _rug_ed_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_table = 0;
    }
    #[test]
    fn test_is_inline_table_on_none() {
        let _rug_st_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_none = 0;
        let none = Item::None;
        debug_assert!(! none.is_inline_table());
        let _rug_ed_tests_llm_16_288_rrrruuuugggg_test_is_inline_table_on_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_289 {
    use super::*;
    use crate::*;
    use crate::Item;
    #[test]
    fn test_is_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, &str, f64, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let int_value = Item::Value(Value::Integer(Formatted::new(rug_fuzz_0)));
        let string_value = Item::Value(
            Value::String(Formatted::new(rug_fuzz_1.to_string())),
        );
        let float_value = Item::Value(Value::Float(Formatted::new(rug_fuzz_2)));
        let bool_value = Item::Value(Value::Boolean(Formatted::new(rug_fuzz_3)));
        let datetime_value = Item::Value(
            Value::Datetime(Formatted::new(rug_fuzz_4.parse().unwrap())),
        );
        debug_assert_eq!(int_value.is_integer(), true);
        debug_assert_eq!(string_value.is_integer(), false);
        debug_assert_eq!(float_value.is_integer(), false);
        debug_assert_eq!(bool_value.is_integer(), false);
        debug_assert_eq!(datetime_value.is_integer(), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_290 {
    use super::*;
    use crate::*;
    #[test]
    fn test_item_is_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Item::None.is_none());
        debug_assert!(! Item::Value(Value::from(rug_fuzz_0)).is_none());
        debug_assert!(! Item::Table(Table::new()).is_none());
        debug_assert!(! Item::ArrayOfTables(ArrayOfTables::new()).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_291_llm_16_291 {
    use crate::item::Item;
    use crate::repr::Formatted;
    use crate::value::Value;
    use crate::raw_string::RawString;
    #[test]
    fn test_item_is_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let str_value = Value::String(Formatted::new(rug_fuzz_0.to_owned()));
        let str_item = Item::Value(str_value);
        debug_assert!(str_item.is_str());
        let int_value = Value::Integer(Formatted::new(rug_fuzz_1));
        let int_item = Item::Value(int_value);
        debug_assert!(! int_item.is_str());
        let table_item = Item::Table(Default::default());
        debug_assert!(! table_item.is_str());
        let array_of_tables_item = Item::ArrayOfTables(Default::default());
        debug_assert!(! array_of_tables_item.is_str());
        let none_item = Item::None;
        debug_assert!(! none_item.is_str());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_292 {
    use super::*;
    use crate::*;
    use crate::Item;
    #[test]
    fn item_is_table_for_table_type() {
        let _rug_st_tests_llm_16_292_rrrruuuugggg_item_is_table_for_table_type = 0;
        let table = Item::Table(Table::new());
        debug_assert!(table.is_table());
        let _rug_ed_tests_llm_16_292_rrrruuuugggg_item_is_table_for_table_type = 0;
    }
    #[test]
    fn item_is_table_for_non_table_types() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = Item::Value(Value::Array(Array::new()));
        let array_of_tables = Item::ArrayOfTables(ArrayOfTables::new());
        let value = Item::Value(Value::String(Formatted::new(rug_fuzz_0.to_string())));
        debug_assert!(! array.is_table());
        debug_assert!(! array_of_tables.is_table());
        debug_assert!(! value.is_table());
             }
});    }
    #[test]
    fn item_is_table_for_none_type() {
        let _rug_st_tests_llm_16_292_rrrruuuugggg_item_is_table_for_none_type = 0;
        let none = Item::None;
        debug_assert!(! none.is_table());
        let _rug_ed_tests_llm_16_292_rrrruuuugggg_item_is_table_for_none_type = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_294 {
    use crate::item::Item;
    use crate::value::Value;
    use crate::table::Table;
    use crate::array::Array;
    use crate::array_of_tables::ArrayOfTables;
    use crate::inline_table::InlineTable;
    #[test]
    fn test_is_value_on_value_item() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::from(rug_fuzz_0);
        let item = Item::Value(value);
        debug_assert!(item.is_value());
             }
});    }
    #[test]
    fn test_is_value_on_table_item() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_is_value_on_table_item = 0;
        let table = Table::new();
        let item = Item::Table(table);
        debug_assert!(! item.is_value());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_is_value_on_table_item = 0;
    }
    #[test]
    fn test_is_value_on_array_of_tables_item() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_is_value_on_array_of_tables_item = 0;
        let array_of_tables = ArrayOfTables::new();
        let item = Item::ArrayOfTables(array_of_tables);
        debug_assert!(! item.is_value());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_is_value_on_array_of_tables_item = 0;
    }
    #[test]
    fn test_is_value_on_array_item() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_is_value_on_array_item = 0;
        let array = Array::new();
        let value = Value::Array(array);
        let item = Item::Value(value);
        debug_assert!(item.is_value());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_is_value_on_array_item = 0;
    }
    #[test]
    fn test_is_value_on_inline_table_item() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_is_value_on_inline_table_item = 0;
        let inline_table = InlineTable::new();
        let value = Value::InlineTable(inline_table);
        let item = Item::Value(value);
        debug_assert!(item.is_value());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_is_value_on_inline_table_item = 0;
    }
    #[test]
    fn test_is_value_on_none_item() {
        let _rug_st_tests_llm_16_294_rrrruuuugggg_test_is_value_on_none_item = 0;
        let item = Item::None;
        debug_assert!(! item.is_value());
        let _rug_ed_tests_llm_16_294_rrrruuuugggg_test_is_value_on_none_item = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_295 {
    use super::*;
    use crate::*;
    use crate::item::Item;
    use crate::value::Value;
    #[test]
    fn test_make_item_on_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut item = Item::Value(Value::from(rug_fuzz_0));
        item.make_item();
        debug_assert!(matches!(item, Item::Value(_)));
             }
});    }
    #[test]
    fn test_make_item_on_table() {
        let _rug_st_tests_llm_16_295_rrrruuuugggg_test_make_item_on_table = 0;
        let mut item = Item::Table(Table::new());
        item.make_item();
        debug_assert!(matches!(item, Item::Table(_)));
        let _rug_ed_tests_llm_16_295_rrrruuuugggg_test_make_item_on_table = 0;
    }
    #[test]
    fn test_make_item_on_array_of_tables() {
        let _rug_st_tests_llm_16_295_rrrruuuugggg_test_make_item_on_array_of_tables = 0;
        let mut item = Item::ArrayOfTables(ArrayOfTables::new());
        item.make_item();
        debug_assert!(matches!(item, Item::ArrayOfTables(_)));
        let _rug_ed_tests_llm_16_295_rrrruuuugggg_test_make_item_on_array_of_tables = 0;
    }
    #[test]
    fn test_make_item_on_array() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = Value::from_iter(vec![Value::from(rug_fuzz_0)]);
        let mut item = Item::Value(array);
        item.make_item();
        debug_assert!(matches!(item, Item::Value(Value::Array(_))));
             }
});    }
    #[test]
    fn test_make_item_on_inline_table() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let table = Value::from_iter(vec![(rug_fuzz_0, Value::from(rug_fuzz_1))]);
        let mut item = Item::Value(table);
        item.make_item();
        debug_assert!(matches!(item, Item::Value(Value::InlineTable(_))));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_296 {
    use crate::item::Item;
    #[test]
    fn test_make_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut item_none = Item::None;
        item_none.make_value();
        debug_assert!(matches!(item_none, Item::None));
        let mut item_value = Item::Value(rug_fuzz_0.into());
        item_value.make_value();
        debug_assert!(matches!(item_value, Item::Value(_)));
        let mut item_table = Item::Table(Default::default());
        item_table.make_value();
        debug_assert!(matches!(item_table, Item::Value(_)));
        let mut item_aot = Item::ArrayOfTables(Default::default());
        item_aot.make_value();
        debug_assert!(matches!(item_aot, Item::Value(_)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_299 {
    use crate::item::Item;
    use crate::value::Value;
    use crate::array::Array;
    use crate::array_of_tables::ArrayOfTables;
    use crate::table::Table;
    use crate::inline_table::InlineTable;
    use crate::repr::Formatted;
    #[test]
    fn test_type_name() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let none_item = Item::None;
        debug_assert_eq!(none_item.type_name(), "none");
        let value_item = Item::Value(
            Value::String(Formatted::new(rug_fuzz_0.to_string())),
        );
        debug_assert_eq!(value_item.type_name(), "string");
        let table_item = Item::Table(Table::new());
        debug_assert_eq!(table_item.type_name(), "table");
        let array_item = Item::Value(Value::Array(Array::new()));
        debug_assert_eq!(array_item.type_name(), "array");
        let array_of_tables_item = Item::ArrayOfTables(ArrayOfTables::new());
        debug_assert_eq!(array_of_tables_item.type_name(), "array of tables");
        let inline_table_item = Item::Value(Value::InlineTable(InlineTable::new()));
        debug_assert_eq!(inline_table_item.type_name(), "inline table");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_300_llm_16_300 {
    use crate::item::{array, Item};
    use crate::array::Array;
    use crate::array_of_tables::ArrayOfTables;
    use crate::table::Table;
    use crate::value::Value;
    #[test]
    fn test_array_creates_an_array_of_tables() {
        let _rug_st_tests_llm_16_300_llm_16_300_rrrruuuugggg_test_array_creates_an_array_of_tables = 0;
        let item = array();
        if let Item::ArrayOfTables(array_of_tables) = item {
            debug_assert!(array_of_tables.is_empty());
        } else {
            panic!("array() did not create an Item::ArrayOfTables");
        }
        let _rug_ed_tests_llm_16_300_llm_16_300_rrrruuuugggg_test_array_creates_an_array_of_tables = 0;
    }
    #[test]
    fn test_array_of_tables_traits() {
        let _rug_st_tests_llm_16_300_llm_16_300_rrrruuuugggg_test_array_of_tables_traits = 0;
        let array_of_tables = ArrayOfTables::new();
        let cloned = array_of_tables.clone();
        debug_assert_eq!(
            array_of_tables.len(), cloned.len(),
            "Cloning did not produce an equal ArrayOfTables with regards to length"
        );
        let default = ArrayOfTables::default();
        debug_assert_eq!(
            default.len(), 0, "Default should create an empty ArrayOfTables"
        );
        let mut extended = ArrayOfTables::new();
        extended.extend(vec![Table::new(), Table::new()]);
        debug_assert_eq!(extended.len(), 2, "Extending did not add two Tables");
        let array: ArrayOfTables = vec![Table::new(), Table::new()]
            .into_iter()
            .collect();
        debug_assert_eq!(
            array.len(), 2, "Collecting did not produce an ArrayOfTables with two Tables"
        );
        let _rug_ed_tests_llm_16_300_llm_16_300_rrrruuuugggg_test_array_of_tables_traits = 0;
    }
    #[test]
    fn test_array_of_tables_iter() {
        let _rug_st_tests_llm_16_300_llm_16_300_rrrruuuugggg_test_array_of_tables_iter = 0;
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.extend(vec![Table::new(), Table::new()]);
        debug_assert_eq!(
            array_of_tables.iter().count(), 2, "Iterator should yield two Tables"
        );
        debug_assert_eq!(
            array_of_tables.iter_mut().count(), 2,
            "Mutable Iterator should yield two Tables"
        );
        let _rug_ed_tests_llm_16_300_llm_16_300_rrrruuuugggg_test_array_of_tables_iter = 0;
    }
    #[test]
    fn test_array_of_tables_get_and_remove() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.extend(vec![Table::new(), Table::new()]);
        debug_assert!(
            array_of_tables.get(rug_fuzz_0).is_some(), "Should get a table at index 0"
        );
        debug_assert!(
            array_of_tables.get_mut(rug_fuzz_1).is_some(),
            "Should get a mutable table at index 0"
        );
        array_of_tables.remove(rug_fuzz_2);
        debug_assert_eq!(array_of_tables.len(), 1, "Remove should decrease length by 1");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_301 {
    use crate::table;
    use crate::item::Item;
    use crate::table::Table;
    #[test]
    fn test_table_creates_empty_table() {
        let _rug_st_tests_llm_16_301_rrrruuuugggg_test_table_creates_empty_table = 0;
        let result = table();
        debug_assert!(matches!(result, Item::Table(_)));
        if let Item::Table(t) = result {
            debug_assert!(t.is_empty());
        } else {
            panic!("table() did not create an Item::Table");
        }
        let _rug_ed_tests_llm_16_301_rrrruuuugggg_test_table_creates_empty_table = 0;
    }
    #[test]
    fn test_table_creates_table_with_no_span() {
        let _rug_st_tests_llm_16_301_rrrruuuugggg_test_table_creates_table_with_no_span = 0;
        let result = table();
        debug_assert!(matches!(result, Item::Table(_)));
        if let Item::Table(t) = result {
            debug_assert!(t.span().is_none());
        } else {
            panic!("table() did not create an Item::Table");
        }
        let _rug_ed_tests_llm_16_301_rrrruuuugggg_test_table_creates_table_with_no_span = 0;
    }
    #[test]
    fn test_table_creates_table_with_default_decor() {
        let _rug_st_tests_llm_16_301_rrrruuuugggg_test_table_creates_table_with_default_decor = 0;
        let result = table();
        debug_assert!(matches!(result, Item::Table(_)));
        if let Item::Table(t) = result {
            debug_assert!(t.decor().prefix().is_none());
            debug_assert!(t.decor().suffix().is_none());
        } else {
            panic!("table() did not create an Item::Table");
        }
        let _rug_ed_tests_llm_16_301_rrrruuuugggg_test_table_creates_table_with_default_decor = 0;
    }
}
