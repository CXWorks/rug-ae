use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::repr::Decor;
use crate::table::TableKeyValue;
use crate::{ArrayOfTables, Document, InternalString, Item, RawString, Table};
pub(crate) struct ParseState {
    document: Document,
    trailing: Option<std::ops::Range<usize>>,
    current_table_position: usize,
    current_table: Table,
    current_is_array: bool,
    current_table_path: Vec<Key>,
}
impl ParseState {
    pub(crate) fn into_document(mut self) -> Result<Document, CustomError> {
        self.finalize_table()?;
        let trailing = self.trailing.map(RawString::with_span);
        self.document.trailing = trailing.unwrap_or_default();
        Ok(self.document)
    }
    pub(crate) fn on_ws(&mut self, span: std::ops::Range<usize>) {
        if let Some(old) = self.trailing.take() {
            self.trailing = Some(old.start..span.end);
        } else {
            self.trailing = Some(span);
        }
    }
    pub(crate) fn on_comment(&mut self, span: std::ops::Range<usize>) {
        if let Some(old) = self.trailing.take() {
            self.trailing = Some(old.start..span.end);
        } else {
            self.trailing = Some(span);
        }
    }
    pub(crate) fn on_keyval(
        &mut self,
        mut path: Vec<Key>,
        mut kv: TableKeyValue,
    ) -> Result<(), CustomError> {
        {
            let mut prefix = self.trailing.take();
            let first_key = if path.is_empty() { &mut kv.key } else { &mut path[0] };
            let prefix = match (
                prefix.take(),
                first_key.decor.prefix().and_then(|d| d.span()),
            ) {
                (Some(p), Some(k)) => Some(p.start..k.end),
                (Some(p), None) | (None, Some(p)) => Some(p),
                (None, None) => None,
            };
            first_key
                .decor
                .set_prefix(prefix.map(RawString::with_span).unwrap_or_default());
        }
        if let (Some(existing), Some(value))
            = (self.current_table.span(), kv.value.span()) {
            self.current_table.span = Some((existing.start)..(value.end));
        }
        let table = &mut self.current_table;
        let table = Self::descend_path(table, &path, true)?;
        let mixed_table_types = table.is_dotted() == path.is_empty();
        if mixed_table_types {
            return Err(CustomError::DuplicateKey {
                key: kv.key.get().into(),
                table: None,
            });
        }
        let key: InternalString = kv.key.get_internal().into();
        match table.items.entry(key) {
            indexmap::map::Entry::Vacant(o) => {
                o.insert(kv);
            }
            indexmap::map::Entry::Occupied(o) => {
                return Err(CustomError::DuplicateKey {
                    key: o.key().as_str().into(),
                    table: Some(self.current_table_path.clone()),
                });
            }
        }
        Ok(())
    }
    pub(crate) fn start_aray_table(
        &mut self,
        path: Vec<Key>,
        decor: Decor,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(! path.is_empty());
        debug_assert!(self.current_table.is_empty());
        debug_assert!(self.current_table_path.is_empty());
        let root = self.document.as_table_mut();
        let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
        let key = &path[path.len() - 1];
        let entry = parent_table
            .entry_format(key)
            .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
        entry
            .as_array_of_tables()
            .ok_or_else(|| CustomError::duplicate_key(&path, path.len() - 1))?;
        self.current_table_position += 1;
        self.current_table.decor = decor;
        self.current_table.set_implicit(false);
        self.current_table.set_dotted(false);
        self.current_table.set_position(self.current_table_position);
        self.current_table.span = Some(span);
        self.current_is_array = true;
        self.current_table_path = path;
        Ok(())
    }
    pub(crate) fn start_table(
        &mut self,
        path: Vec<Key>,
        decor: Decor,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(! path.is_empty());
        debug_assert!(self.current_table.is_empty());
        debug_assert!(self.current_table_path.is_empty());
        let root = self.document.as_table_mut();
        let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
        let key = &path[path.len() - 1];
        if let Some(entry) = parent_table.remove(key.get()) {
            match entry {
                Item::Table(t) if t.implicit && !t.is_dotted() => {
                    self.current_table = t;
                }
                _ => return Err(CustomError::duplicate_key(&path, path.len() - 1)),
            }
        }
        self.current_table_position += 1;
        self.current_table.decor = decor;
        self.current_table.set_implicit(false);
        self.current_table.set_dotted(false);
        self.current_table.set_position(self.current_table_position);
        self.current_table.span = Some(span);
        self.current_is_array = false;
        self.current_table_path = path;
        Ok(())
    }
    pub(crate) fn finalize_table(&mut self) -> Result<(), CustomError> {
        let mut table = std::mem::take(&mut self.current_table);
        let path = std::mem::take(&mut self.current_table_path);
        let root = self.document.as_table_mut();
        if path.is_empty() {
            assert!(root.is_empty());
            std::mem::swap(&mut table, root);
        } else if self.current_is_array {
            let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
            let key = &path[path.len() - 1];
            let entry = parent_table
                .entry_format(key)
                .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
            let array = entry
                .as_array_of_tables_mut()
                .ok_or_else(|| CustomError::duplicate_key(&path, path.len() - 1))?;
            array.push(table);
            let span = if let (Some(first), Some(last))
                = (
                    array.values.first().and_then(|t| t.span()),
                    array.values.last().and_then(|t| t.span()),
                ) {
                Some((first.start)..(last.end))
            } else {
                None
            };
            array.span = span;
        } else {
            let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
            let key = &path[path.len() - 1];
            let entry = parent_table.entry_format(key);
            match entry {
                crate::Entry::Occupied(entry) => {
                    match entry.into_mut() {
                        Item::Table(ref mut t) if t.implicit => {
                            std::mem::swap(t, &mut table);
                        }
                        _ => {
                            return Err(
                                CustomError::duplicate_key(&path, path.len() - 1),
                            );
                        }
                    }
                }
                crate::Entry::Vacant(entry) => {
                    let item = Item::Table(table);
                    entry.insert(item);
                }
            }
        }
        Ok(())
    }
    pub(crate) fn descend_path<'t, 'k>(
        mut table: &'t mut Table,
        path: &'k [Key],
        dotted: bool,
    ) -> Result<&'t mut Table, CustomError> {
        for (i, key) in path.iter().enumerate() {
            let entry = table
                .entry_format(key)
                .or_insert_with(|| {
                    let mut new_table = Table::new();
                    new_table.set_implicit(true);
                    new_table.set_dotted(dotted);
                    Item::Table(new_table)
                });
            match *entry {
                Item::Value(ref v) => {
                    return Err(CustomError::extend_wrong_type(path, i, v.type_name()));
                }
                Item::ArrayOfTables(ref mut array) => {
                    debug_assert!(! array.is_empty());
                    let index = array.len() - 1;
                    let last_child = array.get_mut(index).unwrap();
                    table = last_child;
                }
                Item::Table(ref mut sweet_child_of_mine) => {
                    if dotted && !sweet_child_of_mine.is_implicit() {
                        return Err(CustomError::DuplicateKey {
                            key: key.get().into(),
                            table: None,
                        });
                    }
                    table = sweet_child_of_mine;
                }
                _ => unreachable!(),
            }
        }
        Ok(table)
    }
    pub(crate) fn on_std_header(
        &mut self,
        path: Vec<Key>,
        trailing: std::ops::Range<usize>,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(! path.is_empty());
        self.finalize_table()?;
        let leading = self.trailing.take().map(RawString::with_span).unwrap_or_default();
        self.start_table(
            path,
            Decor::new(leading, RawString::with_span(trailing)),
            span,
        )?;
        Ok(())
    }
    pub(crate) fn on_array_header(
        &mut self,
        path: Vec<Key>,
        trailing: std::ops::Range<usize>,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(! path.is_empty());
        self.finalize_table()?;
        let leading = self.trailing.take().map(RawString::with_span).unwrap_or_default();
        self.start_aray_table(
            path,
            Decor::new(leading, RawString::with_span(trailing)),
            span,
        )?;
        Ok(())
    }
}
impl Default for ParseState {
    fn default() -> Self {
        let mut root = Table::new();
        root.span = Some(0..0);
        Self {
            document: Document::new(),
            trailing: None,
            current_table_position: 0,
            current_table: root,
            current_is_array: false,
            current_table_path: Vec::new(),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_84 {
    use super::*;
    use crate::*;
    #[test]
    fn test_parse_state_default() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_test_parse_state_default = 0;
        let state = ParseState::default();
        debug_assert!(state.document.as_table().is_empty());
        debug_assert!(state.trailing.is_none());
        debug_assert_eq!(state.current_table_position, 0);
        debug_assert!(state.current_table.is_empty());
        debug_assert!(! state.current_is_array);
        debug_assert!(state.current_table_path.is_empty());
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_test_parse_state_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_392 {
    use super::*;
    use crate::*;
    use crate::Document;
    #[test]
    fn test_finalize_table_empty_root() {
        let _rug_st_tests_llm_16_392_rrrruuuugggg_test_finalize_table_empty_root = 0;
        let mut parse_state = ParseState::default();
        debug_assert!(parse_state.finalize_table().is_ok());
        debug_assert!(parse_state.document.as_table().is_empty());
        let _rug_ed_tests_llm_16_392_rrrruuuugggg_test_finalize_table_empty_root = 0;
    }
    #[test]
    fn test_finalize_table_with_existing_root() {
        let _rug_st_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_existing_root = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let mut parse_state = ParseState::default();
        parse_state
            .document[rug_fuzz_0] = Item::Value(
            Value::String(Formatted::new(rug_fuzz_1.to_string())),
        );
        debug_assert!(parse_state.finalize_table().is_ok());
        debug_assert_eq!(
            parse_state.document[rug_fuzz_2].as_value().unwrap().as_str().unwrap(),
            "value"
        );
        let _rug_ed_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_existing_root = 0;
    }
    #[test]
    fn test_finalize_table_with_nested_table() {
        let _rug_st_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_nested_table = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = "b";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = "a";
        let rug_fuzz_5 = "b";
        let rug_fuzz_6 = "key";
        let mut parse_state = ParseState::default();
        parse_state
            .document[rug_fuzz_0][rug_fuzz_1][rug_fuzz_2] = Item::Value(
            Value::Integer(Formatted::new(rug_fuzz_3)),
        );
        parse_state.finalize_table().unwrap();
        debug_assert_eq!(
            parse_state.document[rug_fuzz_4] [rug_fuzz_5] [rug_fuzz_6].as_value()
            .unwrap().as_integer().unwrap(), 42
        );
        let _rug_ed_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_nested_table = 0;
    }
    #[test]
    fn test_finalize_table_with_current_table() {
        let _rug_st_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_current_table = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = "key";
        let mut parse_state = ParseState::default();
        parse_state
            .current_table[rug_fuzz_0] = Item::Value(
            Value::Integer(Formatted::new(rug_fuzz_1)),
        );
        parse_state.finalize_table().unwrap();
        debug_assert_eq!(
            parse_state.document.as_table() [rug_fuzz_2].as_value().unwrap().as_integer()
            .unwrap(), 100
        );
        let _rug_ed_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_current_table = 0;
    }
    #[test]
    fn test_finalize_table_with_current_array() {
        let _rug_st_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_current_array = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "a";
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = "a";
        let rug_fuzz_5 = "b";
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = "key";
        let mut parse_state = ParseState::default();
        parse_state
            .current_table[rug_fuzz_0] = Item::Value(
            Value::String(Formatted::new(rug_fuzz_1.to_string())),
        );
        parse_state
            .current_table_path = vec![
            rug_fuzz_2.parse().unwrap(), "b".parse().unwrap()
        ];
        parse_state.current_is_array = rug_fuzz_3;
        parse_state.finalize_table().unwrap();
        debug_assert_eq!(
            parse_state.document[rug_fuzz_4] [rug_fuzz_5] [rug_fuzz_6] [rug_fuzz_7]
            .as_value().unwrap().as_str().unwrap(), "value"
        );
        let _rug_ed_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_current_array = 0;
    }
    #[test]
    fn test_finalize_table_with_duplicate_key() {
        let _rug_st_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_duplicate_key = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "value";
        let mut parse_state = ParseState::default();
        parse_state
            .document[rug_fuzz_0] = Item::Value(
            Value::Integer(Formatted::new(rug_fuzz_1)),
        );
        parse_state
            .current_table[rug_fuzz_2] = Item::Value(
            Value::String(Formatted::new(rug_fuzz_3.to_string())),
        );
        debug_assert!(parse_state.finalize_table().is_err());
        let _rug_ed_tests_llm_16_392_rrrruuuugggg_test_finalize_table_with_duplicate_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_395 {
    use super::*;
    use crate::*;
    use crate::parser::state::ParseState;
    #[test]
    fn test_on_comment_no_prev_trailing() {
        let _rug_st_tests_llm_16_395_rrrruuuugggg_test_on_comment_no_prev_trailing = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let mut state = ParseState::default();
        debug_assert!(state.trailing.is_none());
        state.on_comment(rug_fuzz_0..rug_fuzz_1);
        debug_assert_eq!(state.trailing, Some(0..10));
        let _rug_ed_tests_llm_16_395_rrrruuuugggg_test_on_comment_no_prev_trailing = 0;
    }
    #[test]
    fn test_on_comment_with_prev_trailing() {
        let _rug_st_tests_llm_16_395_rrrruuuugggg_test_on_comment_with_prev_trailing = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 15;
        let rug_fuzz_2 = 20;
        let rug_fuzz_3 = 30;
        let mut state = ParseState::default();
        state.trailing = Some(rug_fuzz_0..rug_fuzz_1);
        state.on_comment(rug_fuzz_2..rug_fuzz_3);
        debug_assert_eq!(state.trailing, Some(5..30));
        let _rug_ed_tests_llm_16_395_rrrruuuugggg_test_on_comment_with_prev_trailing = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_398 {
    use super::*;
    use crate::*;
    #[test]
    fn on_ws_empty_state_empty_span() {
        let _rug_st_tests_llm_16_398_rrrruuuugggg_on_ws_empty_state_empty_span = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let mut state = ParseState::default();
        state.on_ws(rug_fuzz_0..rug_fuzz_1);
        debug_assert_eq!(state.trailing, Some(0..0));
        let _rug_ed_tests_llm_16_398_rrrruuuugggg_on_ws_empty_state_empty_span = 0;
    }
    #[test]
    fn on_ws_empty_state_non_empty_span() {
        let _rug_st_tests_llm_16_398_rrrruuuugggg_on_ws_empty_state_non_empty_span = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let mut state = ParseState::default();
        state.on_ws(rug_fuzz_0..rug_fuzz_1);
        debug_assert_eq!(state.trailing, Some(5..10));
        let _rug_ed_tests_llm_16_398_rrrruuuugggg_on_ws_empty_state_non_empty_span = 0;
    }
    #[test]
    fn on_ws_non_empty_state_extend_span() {
        let _rug_st_tests_llm_16_398_rrrruuuugggg_on_ws_non_empty_state_extend_span = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let mut state = ParseState::default();
        state.trailing = Some(rug_fuzz_0..rug_fuzz_1);
        state.on_ws(rug_fuzz_2..rug_fuzz_3);
        debug_assert_eq!(state.trailing, Some(3..10));
        let _rug_ed_tests_llm_16_398_rrrruuuugggg_on_ws_non_empty_state_extend_span = 0;
    }
    #[test]
    fn on_ws_non_empty_state_replace_span() {
        let _rug_st_tests_llm_16_398_rrrruuuugggg_on_ws_non_empty_state_replace_span = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let mut state = ParseState::default();
        state.trailing = Some(rug_fuzz_0..rug_fuzz_1);
        state.on_ws(rug_fuzz_2..rug_fuzz_3);
        debug_assert_eq!(state.trailing, Some(0..10));
        let _rug_ed_tests_llm_16_398_rrrruuuugggg_on_ws_non_empty_state_replace_span = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_400 {
    use super::*;
    use crate::*;
    use crate::{array::Array, item::Item, key::Key, repr::Decor, table::Table};
    #[test]
    fn test_start_table_empty_path() {
        let _rug_st_tests_llm_16_400_rrrruuuugggg_test_start_table_empty_path = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let mut state = ParseState::default();
        let decor = Decor::default();
        let span = rug_fuzz_0..rug_fuzz_1;
        let result = state.start_table(Vec::new(), decor, span);
        debug_assert!(result.is_err(), "Empty path should not be allowed");
        let _rug_ed_tests_llm_16_400_rrrruuuugggg_test_start_table_empty_path = 0;
    }
    #[test]
    fn test_start_table_non_empty_current_table() {
        let _rug_st_tests_llm_16_400_rrrruuuugggg_test_start_table_non_empty_current_table = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "non_empty";
        let rug_fuzz_3 = "key";
        let rug_fuzz_4 = "value";
        let mut state = ParseState::default();
        let decor = Decor::default();
        let span = rug_fuzz_0..rug_fuzz_1;
        let path = vec![Key::new(rug_fuzz_2)];
        state.current_table = Table::new();
        state.current_table.insert(rug_fuzz_3, Item::Value(rug_fuzz_4.parse().unwrap()));
        let result = state.start_table(path.clone(), decor, span);
        debug_assert!(result.is_err(), "Non-empty current table should not be allowed");
        let _rug_ed_tests_llm_16_400_rrrruuuugggg_test_start_table_non_empty_current_table = 0;
    }
    #[test]
    fn test_start_table_non_empty_current_table_path() {
        let _rug_st_tests_llm_16_400_rrrruuuugggg_test_start_table_non_empty_current_table_path = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "non_empty_path";
        let rug_fuzz_3 = "non_empty";
        let mut state = ParseState::default();
        let decor = Decor::default();
        let span = rug_fuzz_0..rug_fuzz_1;
        let path = vec![Key::new(rug_fuzz_2)];
        state.current_table_path = vec![Key::new(rug_fuzz_3)];
        let result = state.start_table(path.clone(), decor, span);
        debug_assert!(
            result.is_err(), "Non-empty current table path should not be allowed"
        );
        let _rug_ed_tests_llm_16_400_rrrruuuugggg_test_start_table_non_empty_current_table_path = 0;
    }
    #[test]
    fn test_start_table_duplicate() {
        let _rug_st_tests_llm_16_400_rrrruuuugggg_test_start_table_duplicate = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "duplicate";
        let rug_fuzz_3 = "value";
        let rug_fuzz_4 = "  ";
        let rug_fuzz_5 = "  ";
        let mut state = ParseState::default();
        let mut decor = Decor::default();
        let span = rug_fuzz_0..rug_fuzz_1;
        let key = Key::new(rug_fuzz_2);
        let path = vec![key.clone()];
        state
            .document
            .root = Item::Table({
            let mut table = Table::new();
            table.insert(key.get(), Item::Value(rug_fuzz_3.parse().unwrap()));
            table
        });
        decor.set_prefix(rug_fuzz_4);
        decor.set_suffix(rug_fuzz_5);
        let result = state.start_table(path.clone(), decor, span.clone());
        debug_assert!(result.is_err(), "Duplicate table should not be allowed");
        let _rug_ed_tests_llm_16_400_rrrruuuugggg_test_start_table_duplicate = 0;
    }
    #[test]
    fn test_start_table_success() {
        let _rug_st_tests_llm_16_400_rrrruuuugggg_test_start_table_success = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "new_table";
        let rug_fuzz_3 = "  ";
        let rug_fuzz_4 = "  ";
        let mut state = ParseState::default();
        let mut decor = Decor::default();
        let span = rug_fuzz_0..rug_fuzz_1;
        let path = vec![Key::new(rug_fuzz_2)];
        decor.set_prefix(rug_fuzz_3);
        decor.set_suffix(rug_fuzz_4);
        let result = state.start_table(path, decor, span);
        debug_assert!(result.is_ok(), "Should be able to start a new table");
        let _rug_ed_tests_llm_16_400_rrrruuuugggg_test_start_table_success = 0;
    }
}
