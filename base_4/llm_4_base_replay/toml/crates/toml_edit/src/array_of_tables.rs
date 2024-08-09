use std::iter::FromIterator;
use crate::{Array, Item, Table};
/// Type representing a TOML array of tables
#[derive(Clone, Debug, Default)]
pub struct ArrayOfTables {
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
            self
                .values
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
        let _rug_st_tests_llm_16_6_rrrruuuugggg_into_iter_empty = 0;
        let array_of_tables = ArrayOfTables::new();
        let mut iter = array_of_tables.into_iter();
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_into_iter_empty = 0;
    }
    #[test]
    fn into_iter_non_empty() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_into_iter_non_empty = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table1 = Table::new();
        let table2 = Table::new();
        array_of_tables.push(table1);
        array_of_tables.push(table2);
        let mut iter = array_of_tables.into_iter();
        debug_assert!(iter.next().is_some());
        debug_assert!(iter.next().is_some());
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_into_iter_non_empty = 0;
    }
    #[test]
    fn into_iter_counts() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array_of_tables = ArrayOfTables::new();
        let num_tables = rug_fuzz_0;
        for _ in rug_fuzz_1..num_tables {
            array_of_tables.push(Table::new());
        }
        debug_assert_eq!(array_of_tables.into_iter().count(), num_tables);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_13_llm_16_13 {
    use crate::{array_of_tables::ArrayOfTables, table::Table, Item};
    #[test]
    fn test_extend_with_empty_iter() {
        let _rug_st_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_empty_iter = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let empty: Vec<Table> = Vec::new();
        array_of_tables.extend(empty);
        debug_assert!(array_of_tables.is_empty());
        let _rug_ed_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_empty_iter = 0;
    }
    #[test]
    fn test_extend_with_non_empty_iter() {
        let _rug_st_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_non_empty_iter = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let tables: Vec<Table> = vec![Table::new(), Table::new()];
        let initial_len = tables.len();
        array_of_tables.extend(tables);
        debug_assert_eq!(array_of_tables.len(), initial_len);
        let _rug_ed_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_non_empty_iter = 0;
    }
    #[test]
    fn test_extend_with_tables_iter() {
        let _rug_st_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_tables_iter = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.extend(std::iter::once(table.clone()));
        debug_assert_eq!(array_of_tables.len(), 1);
        array_of_tables.extend(std::iter::once(table));
        debug_assert_eq!(array_of_tables.len(), 2);
        let _rug_ed_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_tables_iter = 0;
    }
    #[test]
    fn test_extend_with_tables_collected() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array_of_tables = ArrayOfTables::new();
        let tables: Vec<Table> = (rug_fuzz_0..rug_fuzz_1)
            .map(|_| Table::new())
            .collect();
        array_of_tables.extend(tables.clone().into_iter());
        debug_assert_eq!(array_of_tables.len(), 3);
        let array_of_tables_collected = ArrayOfTables::from_iter(tables);
        debug_assert_eq!(array_of_tables_collected.len(), 3);
             }
}
}
}    }
    #[test]
    fn test_extend_with_into_iter() {
        let _rug_st_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_into_iter = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let tables: Vec<Table> = vec![Table::new(), Table::new()];
        let tables_item: Vec<Item> = tables.into_iter().map(Item::Table).collect();
        array_of_tables
            .extend(
                tables_item
                    .into_iter()
                    .filter_map(|item| {
                        if let Item::Table(table) = item { Some(table) } else { None }
                    }),
            );
        debug_assert_eq!(array_of_tables.len(), 2);
        let _rug_ed_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_extend_with_into_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_15_llm_16_15 {
    use crate::{ArrayOfTables, Item, Table, Value};
    #[test]
    fn test_into_iter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array_of_tables = ArrayOfTables::new();
        let mut table = Table::new();
        table[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        array_of_tables.push(table.clone());
        array_of_tables.values.push(Item::None);
        let mut iter = array_of_tables.into_iter();
        if let Some(t) = iter.next() {
            debug_assert_eq!(t[rug_fuzz_2].as_str(), Some("value"));
        } else {
            debug_assert!(rug_fuzz_3, "Expected a table but got None");
        }
        debug_assert!(iter.next().is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_165 {
    use super::*;
    use crate::*;
    use crate::Item;
    use crate::Table;
    #[test]
    fn array_of_tables_clear() {
        let _rug_st_tests_llm_16_165_rrrruuuugggg_array_of_tables_clear = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table.clone());
        array_of_tables.push(table.clone());
        array_of_tables.push(table);
        debug_assert!(! array_of_tables.is_empty());
        array_of_tables.clear();
        debug_assert!(array_of_tables.is_empty());
        let _rug_ed_tests_llm_16_165_rrrruuuugggg_array_of_tables_clear = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_166_llm_16_166 {
    use super::*;
    use crate::*;
    use crate::array::Array;
    use crate::table::Table;
    use crate::Item;
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
}
#[cfg(test)]
mod tests_llm_16_168 {
    use crate::ArrayOfTables;
    use crate::{Item, Table};
    #[test]
    fn test_get_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(&str, &str, usize, usize, &str, usize, usize, &str, &str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut tables = ArrayOfTables::new();
        let mut table = Table::new();
        table[rug_fuzz_0] = crate::value(rug_fuzz_1);
        tables.push(table);
        debug_assert!(tables.get_mut(rug_fuzz_2).is_some());
        debug_assert_eq!(
            tables.get_mut(rug_fuzz_3).unwrap() [rug_fuzz_4].as_str(), Some("value")
        );
        debug_assert!(tables.get_mut(rug_fuzz_5).is_none());
        if let Some(table) = tables.get_mut(rug_fuzz_6) {
            table[rug_fuzz_7] = crate::value(rug_fuzz_8);
        }
        debug_assert_eq!(
            tables.get(rug_fuzz_9).unwrap() [rug_fuzz_10].as_str(), Some("new_value")
        );
             }
}
}
}    }
    #[test]
    fn test_get_mut_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut tables = ArrayOfTables::new();
        debug_assert!(tables.get_mut(rug_fuzz_0).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_170 {
    use super::*;
    use crate::*;
    use crate::Item;
    use crate::Table;
    #[test]
    fn is_empty_empty_array_of_tables() {
        let _rug_st_tests_llm_16_170_rrrruuuugggg_is_empty_empty_array_of_tables = 0;
        let array_of_tables = ArrayOfTables::new();
        debug_assert!(array_of_tables.is_empty());
        let _rug_ed_tests_llm_16_170_rrrruuuugggg_is_empty_empty_array_of_tables = 0;
    }
    #[test]
    fn is_empty_non_empty_array_of_tables() {
        let _rug_st_tests_llm_16_170_rrrruuuugggg_is_empty_non_empty_array_of_tables = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);
        debug_assert!(! array_of_tables.is_empty());
        let _rug_ed_tests_llm_16_170_rrrruuuugggg_is_empty_non_empty_array_of_tables = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_171 {
    use super::*;
    use crate::*;
    use crate::table::Table;
    use crate::Item;
    #[test]
    fn test_iter_empty() {
        let _rug_st_tests_llm_16_171_rrrruuuugggg_test_iter_empty = 0;
        let array_of_tables = ArrayOfTables::new();
        debug_assert_eq!(array_of_tables.iter().count(), 0);
        let _rug_ed_tests_llm_16_171_rrrruuuugggg_test_iter_empty = 0;
    }
    #[test]
    fn test_iter_single_table() {
        let _rug_st_tests_llm_16_171_rrrruuuugggg_test_iter_single_table = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);
        debug_assert_eq!(array_of_tables.iter().count(), 1);
        let _rug_ed_tests_llm_16_171_rrrruuuugggg_test_iter_single_table = 0;
    }
    #[test]
    fn test_iter_multiple_tables() {
        let _rug_st_tests_llm_16_171_rrrruuuugggg_test_iter_multiple_tables = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table_1 = Table::new();
        let table_2 = Table::new();
        array_of_tables.push(table_1);
        array_of_tables.push(table_2);
        debug_assert_eq!(array_of_tables.iter().count(), 2);
        let _rug_ed_tests_llm_16_171_rrrruuuugggg_test_iter_multiple_tables = 0;
    }
    #[test]
    fn test_iter_non_table_items() {
        let _rug_st_tests_llm_16_171_rrrruuuugggg_test_iter_non_table_items = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        let non_table = Item::None;
        array_of_tables.values.push(Item::Table(table.clone()));
        array_of_tables.values.push(non_table);
        let mut iter = array_of_tables.iter();
        debug_assert!(iter.next().is_some());
        debug_assert!(iter.next().is_none());
        let _rug_ed_tests_llm_16_171_rrrruuuugggg_test_iter_non_table_items = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_172 {
    use crate::{ArrayOfTables, Table, Item, Value};
    #[test]
    fn test_iter_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(&str, &str, &str, &str, &str, &str, usize, &str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array_of_tables = ArrayOfTables::new();
        let mut table1 = Table::new();
        table1[rug_fuzz_0] = Item::Value(Value::from(rug_fuzz_1));
        array_of_tables.push(table1);
        let mut table2 = Table::new();
        table2[rug_fuzz_2] = Item::Value(Value::from(rug_fuzz_3));
        array_of_tables.push(table2);
        for table in array_of_tables.iter_mut() {
            table[rug_fuzz_4] = Item::Value(Value::from(rug_fuzz_5));
        }
        let tables: Vec<_> = array_of_tables.iter().collect();
        debug_assert_eq!(tables.len(), 2);
        debug_assert_eq!(
            tables[rug_fuzz_6] [rug_fuzz_7].as_value().unwrap().as_str(),
            Some("new_value")
        );
        debug_assert_eq!(
            tables[rug_fuzz_8] [rug_fuzz_9].as_value().unwrap().as_str(),
            Some("new_value")
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_173 {
    use crate::{ArrayOfTables, Item, Table};
    #[test]
    fn empty_array_of_tables_should_have_length_zero() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_empty_array_of_tables_should_have_length_zero = 0;
        let array_of_tables = ArrayOfTables::new();
        debug_assert_eq!(array_of_tables.len(), 0);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_empty_array_of_tables_should_have_length_zero = 0;
    }
    #[test]
    fn array_of_tables_with_single_table_should_have_length_one() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_array_of_tables_with_single_table_should_have_length_one = 0;
        let mut array_of_tables = ArrayOfTables::new();
        let table = Table::new();
        array_of_tables.push(table);
        debug_assert_eq!(array_of_tables.len(), 1);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_array_of_tables_with_single_table_should_have_length_one = 0;
    }
    #[test]
    fn array_of_tables_with_multiple_tables_should_reflect_correct_length() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_array_of_tables_with_multiple_tables_should_reflect_correct_length = 0;
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        debug_assert_eq!(array_of_tables.len(), 2);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_array_of_tables_with_multiple_tables_should_reflect_correct_length = 0;
    }
    #[test]
    fn array_of_tables_length_should_update_after_adding_tables() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_array_of_tables_length_should_update_after_adding_tables = 0;
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        debug_assert_eq!(array_of_tables.len(), 1);
        array_of_tables.push(Table::new());
        debug_assert_eq!(array_of_tables.len(), 2);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_array_of_tables_length_should_update_after_adding_tables = 0;
    }
    #[test]
    fn array_of_tables_length_should_update_after_removing_tables() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        array_of_tables.remove(rug_fuzz_0);
        debug_assert_eq!(array_of_tables.len(), 1);
        array_of_tables.remove(rug_fuzz_1);
        debug_assert_eq!(array_of_tables.len(), 0);
             }
}
}
}    }
    #[test]
    fn array_of_tables_after_clear_should_have_length_zero() {
        let _rug_st_tests_llm_16_173_rrrruuuugggg_array_of_tables_after_clear_should_have_length_zero = 0;
        let mut array_of_tables = ArrayOfTables::new();
        array_of_tables.push(Table::new());
        array_of_tables.push(Table::new());
        array_of_tables.clear();
        debug_assert_eq!(array_of_tables.len(), 0);
        let _rug_ed_tests_llm_16_173_rrrruuuugggg_array_of_tables_after_clear_should_have_length_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_174 {
    use crate::ArrayOfTables;
    #[test]
    fn test_new_creates_empty_array_of_tables() {
        let _rug_st_tests_llm_16_174_rrrruuuugggg_test_new_creates_empty_array_of_tables = 0;
        let array_of_tables = ArrayOfTables::new();
        debug_assert!(array_of_tables.is_empty());
        debug_assert_eq!(array_of_tables.len(), 0);
        let _rug_ed_tests_llm_16_174_rrrruuuugggg_test_new_creates_empty_array_of_tables = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_177_llm_16_177 {
    use crate::array_of_tables::ArrayOfTables;
    use crate::Item;
    use crate::Table;
    use std::ops::Range;
    #[test]
    fn test_span_returns_none_when_not_set() {
        let _rug_st_tests_llm_16_177_llm_16_177_rrrruuuugggg_test_span_returns_none_when_not_set = 0;
        let arr_of_tables = ArrayOfTables::new();
        debug_assert_eq!(arr_of_tables.span(), None);
        let _rug_ed_tests_llm_16_177_llm_16_177_rrrruuuugggg_test_span_returns_none_when_not_set = 0;
    }
    #[test]
    fn test_span_returns_some_when_set() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut arr_of_tables = ArrayOfTables::new();
        let span = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        arr_of_tables.span = Some(span.clone());
        debug_assert_eq!(arr_of_tables.span(), Some(span));
             }
}
}
}    }
    #[test]
    fn test_span_is_clone_of_original() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut arr_of_tables = ArrayOfTables::new();
        let span = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        arr_of_tables.span = Some(span.clone());
        let retrieved_span = arr_of_tables.span();
        debug_assert_eq!(retrieved_span, Some(span.clone()));
        arr_of_tables.span = None;
        debug_assert_eq!(retrieved_span, Some(span));
             }
}
}
}    }
}
