use std::iter::FromIterator;

use crate::{Array, Item, Table};

/// Type representing a TOML array of tables
#[derive(Clone, Debug, Default)]
pub struct ArrayOfTables {
    // Always Vec<Item::Table>, just `Item` to make `Index` work
    pub(crate) span: Option<std::ops::Range<usize>>,
    pub(crate) values: Vec<Item>,
}

/// Constructors
///
/// See also `FromIterator`
impl ArrayOfTables {
    /// Creates an empty array of tables.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Formatting
impl ArrayOfTables {
    /// Convert to an inline array
    pub fn into_array(mut self) -> Array {
        for value in self.values.iter_mut() {
            value.make_value();
        }
        let mut a = Array::with_vec(self.values);
        a.fmt();
        a
    }

    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }

    pub(crate) fn despan(&mut self, input: &str) {
        self.span = None;
        for value in &mut self.values {
            value.despan(input);
        }
    }
}

impl ArrayOfTables {
    /// Returns an iterator over tables.
    pub fn iter(&self) -> ArrayOfTablesIter<'_> {
        Box::new(self.values.iter().filter_map(Item::as_table))
    }

    /// Returns an iterator over tables.
    pub fn iter_mut(&mut self) -> ArrayOfTablesIterMut<'_> {
        Box::new(self.values.iter_mut().filter_map(Item::as_table_mut))
    }

    /// Returns the length of the underlying Vec.
    /// To get the actual number of items use `a.iter().count()`.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true iff `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes all the tables.
    pub fn clear(&mut self) {
        self.values.clear()
    }

    /// Returns an optional reference to the table.
    pub fn get(&self, index: usize) -> Option<&Table> {
        self.values.get(index).and_then(Item::as_table)
    }

    /// Returns an optional mutable reference to the table.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.values.get_mut(index).and_then(Item::as_table_mut)
    }

    /// Appends a table to the array.
    pub fn push(&mut self, table: Table) {
        self.values.push(Item::Table(table));
    }

    /// Removes a table with the given index.
    pub fn remove(&mut self, index: usize) {
        self.values.remove(index);
    }
}

/// An iterator type over `ArrayOfTables`'s values.
pub type ArrayOfTablesIter<'a> = Box<dyn Iterator<Item = &'a Table> + 'a>;
/// An iterator type over `ArrayOfTables`'s values.
pub type ArrayOfTablesIterMut<'a> = Box<dyn Iterator<Item = &'a mut Table> + 'a>;
/// An iterator type over `ArrayOfTables`'s values.
pub type ArrayOfTablesIntoIter = Box<dyn Iterator<Item = Table>>;

impl Extend<Table> for ArrayOfTables {
    fn extend<T: IntoIterator<Item = Table>>(&mut self, iter: T) {
        for value in iter {
            self.push(value);
        }
    }
}

impl FromIterator<Table> for ArrayOfTables {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Table>,
    {
        let v = iter.into_iter().map(Item::Table);
        ArrayOfTables {
            values: v.collect(),
            span: None,
        }
    }
}

impl IntoIterator for ArrayOfTables {
    type Item = Table;
    type IntoIter = ArrayOfTablesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.values
                .into_iter()
                .filter(|v| v.is_table())
                .map(|v| v.into_table().unwrap()),
        )
    }
}

impl<'s> IntoIterator for &'s ArrayOfTables {
    type Item = &'s Table;
    type IntoIter = ArrayOfTablesIter<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl std::fmt::Display for ArrayOfTables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // HACK: Without the header, we don't really have a proper way of printing this
        self.clone().into_array().fmt(f)
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Table;
    
    #[test]
    fn into_iter_empty() {
        let array_of_tables = ArrayOfTables::new();
        let mut iter = array_of_tables.into_iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn into_iter_non_empty() {
        let mut array_of_tables = ArrayOfTables::new();
        
        let table1 = Table::new();
        let table2 = Table::new();
        array_of_tables.push(table1);
        array_of_tables.push(table2);
        
        let mut iter = array_of_tables.into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn into_iter_counts() {
        let mut array_of_tables = ArrayOfTables::new();
        let num_tables = 5;
        
        for _ in 0..num_tables {
            array_of_tables.push(Table::new());
        }
        
        assert_eq!(array_of_tables.into_iter().count(), num_tables);
    }
}#[cfg(test)]
mod tests_llm_16_13_llm_16_13 {
    use crate::{array_of_tables::ArrayOfTables, table::Table, Item};

    #[test]
    fn test_extend_with_empty_iter() {
        let mut array_of_tables = ArrayOfTables::new();
        let empty: Vec<Table> = Vec::new();
        array_of_tables.extend(empty);
        assert!(array_of_tables.is_empty());
    }

    #[test]
    fn test_extend_with_non_empty_iter() {
        let mut array_of_tables = ArrayOfTables::new();
        let tables: Vec<Table> = vec![Table::new(), Table::new()];
        let initial_len = tables.len();
        array_of_tables.extend(tables);
        assert_eq!(array_of_tables.len(), initial_len);
    }

    #[test]
    fn test_extend_with_tables_iter() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.extend(std::iter::once(table.clone()));
        assert_eq!(array_of_tables.len(), 1);
        array_of_tables.extend(std::iter::once(table));
        assert_eq!(array_of_tables.len(), 2);
    }

    #[test]
    fn test_extend_with_tables_collected() {
        let mut array_of_tables = ArrayOfTables::new();
        let tables: Vec<Table> = (0..3).map(|_| Table::new()).collect();
        array_of_tables.extend(tables.clone().into_iter());
        assert_eq!(array_of_tables.len(), 3);
        let array_of_tables_collected = ArrayOfTables::from_iter(tables);
        assert_eq!(array_of_tables_collected.len(), 3);
    }

    #[test]
    fn test_extend_with_into_iter() {
        let mut array_of_tables = ArrayOfTables::new();
        let tables: Vec<Table> = vec![Table::new(), Table::new()];
        let tables_item: Vec<Item> = tables.into_iter().map(Item::Table).collect();
        array_of_tables.extend(tables_item.into_iter().filter_map(|item| {
            if let Item::Table(table) = item {
                Some(table)
            } else {
                None
            }
        }));
        assert_eq!(array_of_tables.len(), 2);
    }
}#[cfg(test)]
mod tests_llm_16_15_llm_16_15 {
    use crate::{ArrayOfTables, Item, Table, Value};

    #[test]
    fn test_into_iter() {
        let mut array_of_tables = ArrayOfTables::new();
        // Add a proper Table
        let mut table = Table::new();
        table["key"] = Item::Value(Value::from("value"));
        array_of_tables.push(table.clone());

        // Add a non-Table Item to test if it gets filtered out
        array_of_tables.values.push(Item::None);

        let mut iter = array_of_tables.into_iter();
        if let Some(t) = iter.next() {
            assert_eq!(t["key"].as_str(), Some("value"));
        } else {
            assert!(false, "Expected a table but got None");
        }
        assert!(iter.next().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_165 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Table;

    #[test]
    fn array_of_tables_clear() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();

        array_of_tables.push(table.clone());
        array_of_tables.push(table.clone());
        array_of_tables.push(table);

        assert!(!array_of_tables.is_empty());
        array_of_tables.clear();
        assert!(array_of_tables.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_166_llm_16_166 {
    use super::*;

use crate::*;
    use crate::array::Array;
    use crate::table::Table;
    use crate::Item;
    
    // Assuming the existence of the `Table::with_span` method to create a table with `span`
    // as the method `set_span` is not found.
    // You might need to adjust the method name or logic according to the actual `Table` implementation.
    fn table_with_span(span: Option<std::ops::Range<usize>>) -> Table {
        let mut table = Table::new();
        table.span = span;
        table
    }

    #[test]
    fn test_despan() {
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.span = Some(5..10);
        let table1 = table_with_span(Some(1..4));
        let table2 = table_with_span(Some(6..9));
        array_of_tables.push(table1);
        array_of_tables.push(table2);
        
        array_of_tables.despan("Some input");
        
        assert_eq!(array_of_tables.span(), None);
        for table in array_of_tables.values.iter() {
            if let Item::Table(ref t) = table {
                assert_eq!(t.span, None);
            } else {
                panic!("Expected a table");
            }
        }
    }
}#[cfg(test)]
mod tests_llm_16_168 {
    use crate::ArrayOfTables;
    use crate::{Item, Table};
    
    #[test]
    fn test_get_mut() {
        let mut tables = ArrayOfTables::new();
        let mut table = Table::new();
        table["key"] = crate::value("value");
        tables.push(table);

        assert!(tables.get_mut(0).is_some());
        assert_eq!(tables.get_mut(0).unwrap()["key"].as_str(), Some("value"));
        assert!(tables.get_mut(1).is_none());
        
        // Test mutation
        if let Some(table) = tables.get_mut(0) {
            table["key"] = crate::value("new_value");
        }
        
        assert_eq!(tables.get(0).unwrap()["key"].as_str(), Some("new_value"));
    }
    
    #[test]
    fn test_get_mut_empty() {
        let mut tables = ArrayOfTables::new();
        assert!(tables.get_mut(0).is_none());
    }
}#[cfg(test)]
mod tests_llm_16_170 {
    use super::*;

use crate::*;
    use crate::Item;
    use crate::Table;
    
    #[test]
    fn is_empty_empty_array_of_tables() {
        let array_of_tables = ArrayOfTables::new();
        assert!(array_of_tables.is_empty());
    }

    #[test]
    fn is_empty_non_empty_array_of_tables() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);
        assert!(!array_of_tables.is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_171 {
    use super::*;

use crate::*;
    use crate::table::Table;
    use crate::Item;

    #[test]
    fn test_iter_empty() {
        let array_of_tables = ArrayOfTables::new();
        assert_eq!(array_of_tables.iter().count(), 0);
    }

    #[test]
    fn test_iter_single_table() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);
        assert_eq!(array_of_tables.iter().count(), 1);
    }

    #[test]
    fn test_iter_multiple_tables() {
        let mut array_of_tables = ArrayOfTables::new();
        let table_1 = Table::new();
        let table_2 = Table::new();
        array_of_tables.push(table_1);
        array_of_tables.push(table_2);
        assert_eq!(array_of_tables.iter().count(), 2);
    }

    #[test]
    fn test_iter_non_table_items() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        let non_table = Item::None;
        array_of_tables.values.push(Item::Table(table.clone()));
        array_of_tables.values.push(non_table);
        let mut iter = array_of_tables.iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_172 {
    use crate::{ArrayOfTables, Table, Item, Value};

    #[test]
    fn test_iter_mut() {
        let mut array_of_tables = ArrayOfTables::new();
        
        let mut table1 = Table::new();
        table1["key1"] = Item::Value(Value::from("value1"));
        array_of_tables.push(table1);

        let mut table2 = Table::new();
        table2["key2"] = Item::Value(Value::from("value2"));
        array_of_tables.push(table2);

        // Modify elements using iter_mut
        for table in array_of_tables.iter_mut() {
            table["new_key"] = Item::Value(Value::from("new_value"));
        }

        // Check if the tables have been modified
        let tables: Vec<_> = array_of_tables.iter().collect();
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0]["new_key"].as_value().unwrap().as_str(), Some("new_value"));
        assert_eq!(tables[1]["new_key"].as_value().unwrap().as_str(), Some("new_value"));
    }
}#[cfg(test)]
mod tests_llm_16_173 {
    use crate::{ArrayOfTables, Item, Table};

    #[test]
    fn empty_array_of_tables_should_have_length_zero() {
        let array_of_tables = ArrayOfTables::new();
        assert_eq!(array_of_tables.len(), 0);
    }
    
    #[test]
    fn array_of_tables_with_single_table_should_have_length_one() {
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);
        assert_eq!(array_of_tables.len(), 1);
    }
    
    #[test]
    fn array_of_tables_with_multiple_tables_should_reflect_correct_length() {
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        assert_eq!(array_of_tables.len(), 2);
    }
    
    #[test]
    fn array_of_tables_length_should_update_after_adding_tables() {
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        assert_eq!(array_of_tables.len(), 1);
        array_of_tables.push(Table::new());
        assert_eq!(array_of_tables.len(), 2);
    }
    
    #[test]
    fn array_of_tables_length_should_update_after_removing_tables() {
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        array_of_tables.remove(0);
        assert_eq!(array_of_tables.len(), 1);
        array_of_tables.remove(0);
        assert_eq!(array_of_tables.len(), 0);
    }
    
    #[test]
    fn array_of_tables_after_clear_should_have_length_zero() {
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        array_of_tables.clear();
        assert_eq!(array_of_tables.len(), 0);
    }
}#[cfg(test)]
mod tests_llm_16_174 {
    use crate::ArrayOfTables;

    #[test]
    fn test_new_creates_empty_array_of_tables() {
        let array_of_tables = ArrayOfTables::new();

        assert!(array_of_tables.is_empty());
        assert_eq!(array_of_tables.len(), 0);
    }
}#[cfg(test)]
mod tests_llm_16_177_llm_16_177 {
    use crate::array_of_tables::ArrayOfTables;
    use crate::Item;
    use crate::Table;
    use std::ops::Range;

    #[test]
    fn test_span_returns_none_when_not_set() {
        let arr_of_tables = ArrayOfTables::new();
        assert_eq!(arr_of_tables.span(), None);
    }

    #[test]
    fn test_span_returns_some_when_set() {
        let mut arr_of_tables = ArrayOfTables::new();
        let span = Range { start: 10, end: 20 };
        arr_of_tables.span = Some(span.clone());
        assert_eq!(arr_of_tables.span(), Some(span));
    }

    #[test]
    fn test_span_is_clone_of_original() {
        let mut arr_of_tables = ArrayOfTables::new();
        let span = Range { start: 30, end: 40 };
        arr_of_tables.span = Some(span.clone());
        let retrieved_span = arr_of_tables.span();
        assert_eq!(retrieved_span, Some(span.clone())); // Clone span before assert
        // Now test that the retrieved span is independent (cloned)
        arr_of_tables.span = None;
        assert_eq!(retrieved_span, Some(span));
    }
}