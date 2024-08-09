#[derive(Copy, Clone, Default)]
pub(crate) struct DocumentFormatter {
    pub(crate) multiline_array: bool,
}
impl toml_edit::visit_mut::VisitMut for DocumentFormatter {
    fn visit_document_mut(&mut self, node: &mut toml_edit::Document) {
        toml_edit::visit_mut::visit_document_mut(self, node);
    }
    fn visit_item_mut(&mut self, node: &mut toml_edit::Item) {
        let other = std::mem::take(node);
        let other = match other.into_table().map(toml_edit::Item::Table) {
            Ok(i) => i,
            Err(i) => i,
        };
        let other = match other
            .into_array_of_tables()
            .map(toml_edit::Item::ArrayOfTables)
        {
            Ok(i) => i,
            Err(i) => i,
        };
        *node = other;
        toml_edit::visit_mut::visit_item_mut(self, node);
    }
    fn visit_table_mut(&mut self, node: &mut toml_edit::Table) {
        node.decor_mut().clear();
        if !node.is_empty() {
            node.set_implicit(true);
        }
        toml_edit::visit_mut::visit_table_mut(self, node);
    }
    fn visit_value_mut(&mut self, node: &mut toml_edit::Value) {
        node.decor_mut().clear();
        toml_edit::visit_mut::visit_value_mut(self, node);
    }
    fn visit_array_mut(&mut self, node: &mut toml_edit::Array) {
        toml_edit::visit_mut::visit_array_mut(self, node);
        if !self.multiline_array || (0..=1).contains(&node.len()) {
            node.set_trailing("");
            node.set_trailing_comma(false);
        } else {
            for item in node.iter_mut() {
                item.decor_mut().set_prefix("\n    ");
            }
            node.set_trailing("\n");
            node.set_trailing_comma(true);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use toml_edit::{Document, Item, Table, ArrayOfTables, Value};
    use toml_edit::visit_mut::VisitMut;
    use crate::fmt::DocumentFormatter;
    #[test]
    fn test_visit_item_mut_table_conversion() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_table_conversion = 0;
        let mut formatter = DocumentFormatter::default();
        let mut item = Item::Table(Table::new());
        formatter.visit_item_mut(&mut item);
        debug_assert!(matches!(item, Item::Table(_)));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_table_conversion = 0;
    }
    #[test]
    fn test_visit_item_mut_array_of_tables_conversion() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_array_of_tables_conversion = 0;
        let mut formatter = DocumentFormatter::default();
        let mut item = Item::ArrayOfTables(ArrayOfTables::new());
        formatter.visit_item_mut(&mut item);
        debug_assert!(matches!(item, Item::ArrayOfTables(_)));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_array_of_tables_conversion = 0;
    }
    #[test]
    fn test_visit_item_mut_no_conversion() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_no_conversion = 0;
        let rug_fuzz_0 = 42;
        let mut formatter = DocumentFormatter::default();
        let mut doc = Document::new();
        let mut item = Item::Value(Value::from(rug_fuzz_0));
        formatter.visit_item_mut(&mut item);
        debug_assert!(matches!(item, Item::Value(_)));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_no_conversion = 0;
    }
    #[test]
    fn test_visit_item_mut_through_document() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_through_document = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "key";
        let mut formatter = DocumentFormatter::default();
        let mut doc = Document::new();
        let item = Item::Value(Value::from(rug_fuzz_0));
        doc.as_table_mut().insert(rug_fuzz_1, item);
        formatter.visit_document_mut(&mut doc);
        let retrieved_item = doc.get_mut(rug_fuzz_2).unwrap();
        debug_assert!(matches!(retrieved_item, Item::Value(_)));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_visit_item_mut_through_document = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use toml_edit::{Table, Item, visit_mut::VisitMut};
    use crate::fmt::DocumentFormatter;
    #[test]
    fn visit_table_mut_clears_decor_and_sets_implicit_if_non_empty() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_visit_table_mut_clears_decor_and_sets_implicit_if_non_empty = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = true;
        let mut table = Table::new();
        debug_assert!(table.is_empty());
        debug_assert!(table.is_implicit());
        table[rug_fuzz_0] = Item::Value(rug_fuzz_1.into());
        debug_assert!(! table.is_empty());
        let mut formatter = DocumentFormatter {
            multiline_array: rug_fuzz_2,
        };
        formatter.visit_table_mut(&mut table);
        debug_assert!(table.decor().prefix().is_none());
        debug_assert!(table.decor().suffix().is_none());
        debug_assert!(! table.is_implicit());
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_visit_table_mut_clears_decor_and_sets_implicit_if_non_empty = 0;
    }
    #[test]
    fn visit_table_mut_keeps_empty_tables_implicit() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_visit_table_mut_keeps_empty_tables_implicit = 0;
        let rug_fuzz_0 = true;
        let mut table = Table::new();
        let mut formatter = DocumentFormatter {
            multiline_array: rug_fuzz_0,
        };
        formatter.visit_table_mut(&mut table);
        debug_assert!(table.decor().prefix().is_none());
        debug_assert!(table.decor().suffix().is_none());
        debug_assert!(table.is_implicit());
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_visit_table_mut_keeps_empty_tables_implicit = 0;
    }
}
#[cfg(test)]
mod tests_rug_175 {
    use super::*;
    use crate::fmt::DocumentFormatter;
    use toml_edit::Document;
    use toml_edit::visit_mut::VisitMut;
    #[test]
    fn test_visit_document_mut() {
        let _rug_st_tests_rug_175_rrrruuuugggg_test_visit_document_mut = 0;
        let rug_fuzz_0 = true;
        let mut formatter = DocumentFormatter {
            multiline_array: rug_fuzz_0,
        };
        let mut document = Document::new();
        formatter.visit_document_mut(&mut document);
        let _rug_ed_tests_rug_175_rrrruuuugggg_test_visit_document_mut = 0;
    }
}
#[cfg(test)]
mod tests_rug_176 {
    use crate::fmt::DocumentFormatter;
    use toml_edit::{Value, visit_mut::VisitMut};
    #[test]
    fn test_visit_value_mut() {
        let _rug_st_tests_rug_176_rrrruuuugggg_test_visit_value_mut = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 42;
        let mut p0 = DocumentFormatter {
            multiline_array: rug_fuzz_0,
        };
        let mut p1 = Value::from(rug_fuzz_1);
        p0.visit_value_mut(&mut p1);
        let _rug_ed_tests_rug_176_rrrruuuugggg_test_visit_value_mut = 0;
    }
}
#[cfg(test)]
mod tests_rug_177 {
    use crate::fmt::DocumentFormatter;
    use toml_edit::{Array, Document, InlineTable, Item, Value};
    use toml_edit::visit_mut::VisitMut;
    #[test]
    fn test_visit_array_mut() {
        let _rug_st_tests_rug_177_rrrruuuugggg_test_visit_array_mut = 0;
        let rug_fuzz_0 = true;
        let mut p0 = DocumentFormatter {
            multiline_array: rug_fuzz_0,
        };
        let mut p1 = Array::default();
        p0.visit_array_mut(&mut p1);
        let _rug_ed_tests_rug_177_rrrruuuugggg_test_visit_array_mut = 0;
    }
}
