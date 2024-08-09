use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result, Write};
use toml_datetime::*;
use crate::document::Document;
use crate::inline_table::DEFAULT_INLINE_KEY_DECOR;
use crate::key::Key;
use crate::repr::{Formatted, Repr, ValueRepr};
use crate::table::{DEFAULT_KEY_DECOR, DEFAULT_KEY_PATH_DECOR, DEFAULT_TABLE_DECOR};
use crate::value::{
    DEFAULT_LEADING_VALUE_DECOR, DEFAULT_TRAILING_VALUE_DECOR, DEFAULT_VALUE_DECOR,
};
use crate::{Array, InlineTable, Item, Table, Value};
pub(crate) trait Encode {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result;
}
impl Encode for Key {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        let decor = self.decor();
        decor.prefix_encode(buf, input, default_decor.0)?;
        if let Some(input) = input {
            let repr = self
                .as_repr()
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(self.default_repr()));
            repr.encode(buf, input)?;
        } else {
            let repr = self.display_repr();
            write!(buf, "{}", repr)?;
        };
        decor.suffix_encode(buf, input, default_decor.1)?;
        Ok(())
    }
}
impl<'k> Encode for &'k [Key] {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        for (i, key) in self.iter().enumerate() {
            let first = i == 0;
            let last = i + 1 == self.len();
            let prefix = if first { default_decor.0 } else { DEFAULT_KEY_PATH_DECOR.0 };
            let suffix = if last { default_decor.1 } else { DEFAULT_KEY_PATH_DECOR.1 };
            if !first {
                write!(buf, ".")?;
            }
            key.encode(buf, input, (prefix, suffix))?;
        }
        Ok(())
    }
}
impl<'k> Encode for &'k [&'k Key] {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        for (i, key) in self.iter().enumerate() {
            let first = i == 0;
            let last = i + 1 == self.len();
            let prefix = if first { default_decor.0 } else { DEFAULT_KEY_PATH_DECOR.0 };
            let suffix = if last { default_decor.1 } else { DEFAULT_KEY_PATH_DECOR.1 };
            if !first {
                write!(buf, ".")?;
            }
            key.encode(buf, input, (prefix, suffix))?;
        }
        Ok(())
    }
}
impl<T> Encode for Formatted<T>
where
    T: ValueRepr,
{
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        let decor = self.decor();
        decor.prefix_encode(buf, input, default_decor.0)?;
        if let Some(input) = input {
            let repr = self
                .as_repr()
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(self.default_repr()));
            repr.encode(buf, input)?;
        } else {
            let repr = self.display_repr();
            write!(buf, "{}", repr)?;
        };
        decor.suffix_encode(buf, input, default_decor.1)?;
        Ok(())
    }
}
impl Encode for Array {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        let decor = self.decor();
        decor.prefix_encode(buf, input, default_decor.0)?;
        write!(buf, "[")?;
        for (i, elem) in self.iter().enumerate() {
            let inner_decor;
            if i == 0 {
                inner_decor = DEFAULT_LEADING_VALUE_DECOR;
            } else {
                inner_decor = DEFAULT_VALUE_DECOR;
                write!(buf, ",")?;
            }
            elem.encode(buf, input, inner_decor)?;
        }
        if self.trailing_comma() && !self.is_empty() {
            write!(buf, ",")?;
        }
        self.trailing().encode_with_default(buf, input, "")?;
        write!(buf, "]")?;
        decor.suffix_encode(buf, input, default_decor.1)?;
        Ok(())
    }
}
impl Encode for InlineTable {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        let decor = self.decor();
        decor.prefix_encode(buf, input, default_decor.0)?;
        write!(buf, "{{")?;
        self.preamble().encode_with_default(buf, input, "")?;
        let children = self.get_values();
        let len = children.len();
        for (i, (key_path, value)) in children.into_iter().enumerate() {
            if i != 0 {
                write!(buf, ",")?;
            }
            let inner_decor = if i == len - 1 {
                DEFAULT_TRAILING_VALUE_DECOR
            } else {
                DEFAULT_VALUE_DECOR
            };
            key_path.as_slice().encode(buf, input, DEFAULT_INLINE_KEY_DECOR)?;
            write!(buf, "=")?;
            value.encode(buf, input, inner_decor)?;
        }
        write!(buf, "}}")?;
        decor.suffix_encode(buf, input, default_decor.1)?;
        Ok(())
    }
}
impl Encode for Value {
    fn encode(
        &self,
        buf: &mut dyn Write,
        input: Option<&str>,
        default_decor: (&str, &str),
    ) -> Result {
        match self {
            Value::String(repr) => repr.encode(buf, input, default_decor),
            Value::Integer(repr) => repr.encode(buf, input, default_decor),
            Value::Float(repr) => repr.encode(buf, input, default_decor),
            Value::Boolean(repr) => repr.encode(buf, input, default_decor),
            Value::Datetime(repr) => repr.encode(buf, input, default_decor),
            Value::Array(array) => array.encode(buf, input, default_decor),
            Value::InlineTable(table) => table.encode(buf, input, default_decor),
        }
    }
}
impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut path = Vec::new();
        let mut last_position = 0;
        let mut tables = Vec::new();
        visit_nested_tables(
                self.as_table(),
                &mut path,
                false,
                &mut |t, p, is_array| {
                    if let Some(pos) = t.position() {
                        last_position = pos;
                    }
                    tables.push((last_position, t, p.clone(), is_array));
                    Ok(())
                },
            )
            .unwrap();
        tables.sort_by_key(|&(id, _, _, _)| id);
        let mut first_table = true;
        for (_, table, path, is_array) in tables {
            visit_table(
                f,
                self.original.as_deref(),
                table,
                &path,
                is_array,
                &mut first_table,
            )?;
        }
        self.trailing().encode_with_default(f, self.original.as_deref(), "")
    }
}
fn visit_nested_tables<'t, F>(
    table: &'t Table,
    path: &mut Vec<Key>,
    is_array_of_tables: bool,
    callback: &mut F,
) -> Result
where
    F: FnMut(&'t Table, &Vec<Key>, bool) -> Result,
{
    if !table.is_dotted() {
        callback(table, path, is_array_of_tables)?;
    }
    for kv in table.items.values() {
        match kv.value {
            Item::Table(ref t) => {
                let mut key = kv.key.clone();
                if t.is_dotted() {
                    key.decor_mut().clear();
                }
                path.push(key);
                visit_nested_tables(t, path, false, callback)?;
                path.pop();
            }
            Item::ArrayOfTables(ref a) => {
                for t in a.iter() {
                    let key = kv.key.clone();
                    path.push(key);
                    visit_nested_tables(t, path, true, callback)?;
                    path.pop();
                }
            }
            _ => {}
        }
    }
    Ok(())
}
fn visit_table(
    buf: &mut dyn Write,
    input: Option<&str>,
    table: &Table,
    path: &[Key],
    is_array_of_tables: bool,
    first_table: &mut bool,
) -> Result {
    let children = table.get_values();
    let is_visible_std_table = !(table.implicit && children.is_empty());
    if path.is_empty() {
        if !children.is_empty() {
            *first_table = false;
        }
    } else if is_array_of_tables {
        let default_decor = if *first_table {
            *first_table = false;
            ("", DEFAULT_TABLE_DECOR.1)
        } else {
            DEFAULT_TABLE_DECOR
        };
        table.decor.prefix_encode(buf, input, default_decor.0)?;
        write!(buf, "[[")?;
        path.encode(buf, input, DEFAULT_KEY_PATH_DECOR)?;
        write!(buf, "]]")?;
        table.decor.suffix_encode(buf, input, default_decor.1)?;
        writeln!(buf)?;
    } else if is_visible_std_table {
        let default_decor = if *first_table {
            *first_table = false;
            ("", DEFAULT_TABLE_DECOR.1)
        } else {
            DEFAULT_TABLE_DECOR
        };
        table.decor.prefix_encode(buf, input, default_decor.0)?;
        write!(buf, "[")?;
        path.encode(buf, input, DEFAULT_KEY_PATH_DECOR)?;
        write!(buf, "]")?;
        table.decor.suffix_encode(buf, input, default_decor.1)?;
        writeln!(buf)?;
    }
    for (key_path, value) in children {
        key_path.as_slice().encode(buf, input, DEFAULT_KEY_DECOR)?;
        write!(buf, "=")?;
        value.encode(buf, input, DEFAULT_VALUE_DECOR)?;
        writeln!(buf)?;
    }
    Ok(())
}
impl ValueRepr for String {
    fn to_repr(&self) -> Repr {
        to_string_repr(self, None, None)
    }
}
pub(crate) fn to_string_repr(
    value: &str,
    style: Option<StringStyle>,
    literal: Option<bool>,
) -> Repr {
    let (style, literal) = match (style, literal) {
        (Some(style), Some(literal)) => (style, literal),
        (_, Some(literal)) => (infer_style(value).0, literal),
        (Some(style), _) => (style, infer_style(value).1),
        (_, _) => infer_style(value),
    };
    let mut output = String::with_capacity(value.len() * 2);
    if literal {
        output.push_str(style.literal_start());
        output.push_str(value);
        output.push_str(style.literal_end());
    } else {
        output.push_str(style.standard_start());
        for ch in value.chars() {
            match ch {
                '\u{8}' => output.push_str("\\b"),
                '\u{9}' => output.push_str("\\t"),
                '\u{a}' => {
                    match style {
                        StringStyle::NewlineTripple => output.push('\n'),
                        StringStyle::OnelineSingle => output.push_str("\\n"),
                        _ => unreachable!(),
                    }
                }
                '\u{c}' => output.push_str("\\f"),
                '\u{d}' => output.push_str("\\r"),
                '\u{22}' => output.push_str("\\\""),
                '\u{5c}' => output.push_str("\\\\"),
                c if c <= '\u{1f}' || c == '\u{7f}' => {
                    write!(output, "\\u{:04X}", ch as u32).unwrap();
                }
                ch => output.push(ch),
            }
        }
        output.push_str(style.standard_end());
    }
    Repr::new_unchecked(output)
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum StringStyle {
    NewlineTripple,
    OnelineTripple,
    OnelineSingle,
}
impl StringStyle {
    fn literal_start(self) -> &'static str {
        match self {
            Self::NewlineTripple => "'''\n",
            Self::OnelineTripple => "'''",
            Self::OnelineSingle => "'",
        }
    }
    fn literal_end(self) -> &'static str {
        match self {
            Self::NewlineTripple => "'''",
            Self::OnelineTripple => "'''",
            Self::OnelineSingle => "'",
        }
    }
    fn standard_start(self) -> &'static str {
        match self {
            Self::NewlineTripple => "\"\"\"\n",
            Self::OnelineTripple | Self::OnelineSingle => "\"",
        }
    }
    fn standard_end(self) -> &'static str {
        match self {
            Self::NewlineTripple => "\"\"\"",
            Self::OnelineTripple | Self::OnelineSingle => "\"",
        }
    }
}
fn infer_style(value: &str) -> (StringStyle, bool) {
    let mut out = String::with_capacity(value.len() * 2);
    let mut ty = StringStyle::OnelineSingle;
    let mut max_found_singles = 0;
    let mut found_singles = 0;
    let mut prefer_literal = false;
    let mut can_be_pretty = true;
    for ch in value.chars() {
        if can_be_pretty {
            if ch == '\'' {
                found_singles += 1;
                if found_singles >= 3 {
                    can_be_pretty = false;
                }
            } else {
                if found_singles > max_found_singles {
                    max_found_singles = found_singles;
                }
                found_singles = 0;
            }
            match ch {
                '\t' => {}
                '\\' => {
                    prefer_literal = true;
                }
                '\n' => ty = StringStyle::NewlineTripple,
                c if c <= '\u{1f}' || c == '\u{7f}' => can_be_pretty = false,
                _ => {}
            }
            out.push(ch);
        } else {
            if ch == '\n' {
                ty = StringStyle::NewlineTripple;
            }
        }
    }
    if found_singles > 0 && value.ends_with('\'') {
        can_be_pretty = false;
    }
    if !prefer_literal {
        can_be_pretty = false;
    }
    if !can_be_pretty {
        debug_assert!(ty != StringStyle::OnelineTripple);
        return (ty, false);
    }
    if found_singles > max_found_singles {
        max_found_singles = found_singles;
    }
    debug_assert!(max_found_singles < 3);
    if ty == StringStyle::OnelineSingle && max_found_singles >= 1 {
        ty = StringStyle::OnelineTripple;
    }
    (ty, true)
}
impl ValueRepr for i64 {
    fn to_repr(&self) -> Repr {
        Repr::new_unchecked(self.to_string())
    }
}
impl ValueRepr for f64 {
    fn to_repr(&self) -> Repr {
        to_f64_repr(*self)
    }
}
fn to_f64_repr(f: f64) -> Repr {
    let repr = match (f.is_sign_negative(), f.is_nan(), f == 0.0) {
        (true, true, _) => "-nan".to_owned(),
        (false, true, _) => "nan".to_owned(),
        (true, false, true) => "-0.0".to_owned(),
        (false, false, true) => "0.0".to_owned(),
        (_, false, false) => {
            if f % 1.0 == 0.0 { format!("{}.0", f) } else { format!("{}", f) }
        }
    };
    Repr::new_unchecked(repr)
}
impl ValueRepr for bool {
    fn to_repr(&self) -> Repr {
        Repr::new_unchecked(self.to_string())
    }
}
impl ValueRepr for Datetime {
    fn to_repr(&self) -> Repr {
        Repr::new_unchecked(self.to_string())
    }
}
#[cfg(test)]
mod tests_llm_16_54_llm_16_54 {
    use crate::encode::Encode;
    use crate::key::Key;
    use crate::repr::Decor;
    use std::fmt::Write;
    use std::str::FromStr;
    #[test]
    fn test_encode_key_without_input_and_default_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let key = Key::from_str(rug_fuzz_0).unwrap();
        let result = key.encode(&mut output, None, (rug_fuzz_1, rug_fuzz_2));
        debug_assert!(result.is_ok());
        debug_assert_eq!(output, "key");
             }
}
}
}    }
    #[test]
    fn test_encode_key_with_input_and_custom_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let mut key = Key::from_str(rug_fuzz_0).unwrap();
        key = key.with_decor(Decor::new(rug_fuzz_1, rug_fuzz_2));
        let result = key.encode(&mut output, Some(rug_fuzz_3), (rug_fuzz_4, rug_fuzz_5));
        debug_assert!(result.is_ok());
        debug_assert_eq!(output, " keyvalue #comment");
             }
}
}
}    }
    #[test]
    fn test_encode_key_with_input_and_default_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let key = Key::from_str(rug_fuzz_0).unwrap();
        let result = key.encode(&mut output, Some(rug_fuzz_1), (rug_fuzz_2, rug_fuzz_3));
        debug_assert!(result.is_ok());
        debug_assert_eq!(output, " keyvalue #default");
             }
}
}
}    }
    #[test]
    fn test_encode_key_with_default_decor_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let key = Key::from_str(rug_fuzz_0).unwrap();
        let result = key.encode(&mut output, Some(rug_fuzz_1), (rug_fuzz_2, rug_fuzz_3));
        debug_assert!(result.is_ok());
        debug_assert_eq!(output, "keyvalue");
             }
}
}
}    }
    #[test]
    fn test_encode_key_with_alternate_decor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut output = String::new();
        let mut key = Key::from_str(rug_fuzz_0).unwrap();
        key = key.with_decor(Decor::new(rug_fuzz_1, rug_fuzz_2));
        let result = key.encode(&mut output, None, (rug_fuzz_3, rug_fuzz_4));
        debug_assert!(result.is_ok());
        debug_assert_eq!(output, "<<key>>");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_187 {
    use super::*;
    use crate::*;
    #[test]
    fn to_repr_bool_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let repr = <bool as repr::ValueRepr>::to_repr(&value);
        debug_assert_eq!(repr.as_raw().as_str(), Some("true"));
             }
}
}
}    }
    #[test]
    fn to_repr_bool_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let repr = <bool as repr::ValueRepr>::to_repr(&value);
        debug_assert_eq!(repr.as_raw().as_str(), Some("false"));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_189 {
    use super::*;
    use crate::*;
    use crate::repr::ValueRepr;
    #[test]
    fn test_to_repr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: i64 = rug_fuzz_0;
        let repr = <i64 as repr::ValueRepr>::to_repr(&value);
        let raw_str = repr.as_raw().as_str();
        debug_assert_eq!(Some(rug_fuzz_1), raw_str);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_190_llm_16_190 {
    use crate::encode::{self, Repr, ValueRepr};
    use crate::RawString;
    #[test]
    fn string_to_repr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = String::from(rug_fuzz_0);
        let repr = s.to_repr();
        let expected_raw = RawString::from(rug_fuzz_1);
        debug_assert_eq!(repr.as_raw(), & expected_raw);
             }
}
}
}    }
    #[test]
    fn string_empty_to_repr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = String::from(rug_fuzz_0);
        let repr = s.to_repr();
        let expected_raw = RawString::from(rug_fuzz_1);
        debug_assert_eq!(repr.as_raw(), & expected_raw);
             }
}
}
}    }
    #[test]
    fn string_with_newline_to_repr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = String::from(rug_fuzz_0);
        let repr = s.to_repr();
        let expected_raw = RawString::from(rug_fuzz_1);
        debug_assert_eq!(repr.as_raw(), & expected_raw);
             }
}
}
}    }
    #[test]
    fn string_with_special_chars_to_repr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = String::from(rug_fuzz_0);
        let repr = s.to_repr();
        let expected_raw = RawString::from(rug_fuzz_1);
        debug_assert_eq!(repr.as_raw(), & expected_raw);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_192_llm_16_192 {
    use crate::encode::StringStyle;
    #[test]
    fn test_literal_end() {
        let _rug_st_tests_llm_16_192_llm_16_192_rrrruuuugggg_test_literal_end = 0;
        debug_assert_eq!(StringStyle::NewlineTripple.literal_end(), "'''");
        debug_assert_eq!(StringStyle::OnelineTripple.literal_end(), "'''");
        debug_assert_eq!(StringStyle::OnelineSingle.literal_end(), "'");
        let _rug_ed_tests_llm_16_192_llm_16_192_rrrruuuugggg_test_literal_end = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_193 {
    use crate::encode::StringStyle;
    #[test]
    fn test_literal_start() {
        let _rug_st_tests_llm_16_193_rrrruuuugggg_test_literal_start = 0;
        debug_assert_eq!(StringStyle::NewlineTripple.literal_start(), "'''\n");
        debug_assert_eq!(StringStyle::OnelineTripple.literal_start(), "'''");
        debug_assert_eq!(StringStyle::OnelineSingle.literal_start(), "'");
        let _rug_ed_tests_llm_16_193_rrrruuuugggg_test_literal_start = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_194 {
    use crate::encode::StringStyle;
    #[test]
    fn test_standard_end() {
        let _rug_st_tests_llm_16_194_rrrruuuugggg_test_standard_end = 0;
        debug_assert_eq!(StringStyle::NewlineTripple.standard_end(), "\"\"\"");
        debug_assert_eq!(StringStyle::OnelineTripple.standard_end(), "\"");
        debug_assert_eq!(StringStyle::OnelineSingle.standard_end(), "\"");
        let _rug_ed_tests_llm_16_194_rrrruuuugggg_test_standard_end = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_195 {
    use crate::encode::StringStyle;
    #[test]
    fn test_standard_start() {
        let _rug_st_tests_llm_16_195_rrrruuuugggg_test_standard_start = 0;
        debug_assert_eq!(StringStyle::NewlineTripple.standard_start(), "\"\"\"\n");
        debug_assert_eq!(StringStyle::OnelineTripple.standard_start(), "\"");
        debug_assert_eq!(StringStyle::OnelineSingle.standard_start(), "\"");
        let _rug_ed_tests_llm_16_195_rrrruuuugggg_test_standard_start = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_197 {
    use super::*;
    use crate::*;
    use crate::encode::to_f64_repr;
    use crate::repr::Repr;
    use crate::raw_string::RawString;
    #[test]
    fn test_to_f64_repr_positive_int() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_1, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_negative_int() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = -rug_fuzz_0;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_1, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_positive_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_1, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_negative_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = -rug_fuzz_0;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_1, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_1, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_negative_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = -rug_fuzz_0;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_1, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_nan() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = f64::NAN;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_0, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
    #[test]
    fn test_to_f64_repr_negative_nan() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = -f64::NAN;
        let repr = to_f64_repr(number);
        debug_assert_eq!(rug_fuzz_0, repr.as_raw().as_str().unwrap());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_198 {
    use crate::encode::{to_string_repr, StringStyle};
    use crate::repr::Repr;
    #[test]
    fn test_to_string_repr_literal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let style = Some(StringStyle::OnelineSingle);
        let literal = Some(rug_fuzz_1);
        let repr = to_string_repr(value, style, literal);
        debug_assert_eq!(repr.as_raw().as_str(), Some("'test_value'"));
             }
}
}
}    }
    #[test]
    fn test_to_string_repr_standard() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let style = Some(StringStyle::OnelineSingle);
        let literal = Some(rug_fuzz_1);
        let repr = to_string_repr(value, style, literal);
        debug_assert_eq!(repr.as_raw().as_str(), Some("\"test\\nvalue\""));
             }
}
}
}    }
    #[test]
    fn test_string_repr_escapes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let repr = to_string_repr(value, None, None);
        debug_assert_eq!(
            repr.as_raw().as_str(),
            Some("\"tab:\\t newline:\\n backslash:\\\\ quote:\\\"\"")
        );
             }
}
}
}    }
    #[test]
    fn test_to_string_repr_with_inferred_styles() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let repr_literal = to_string_repr(value, None, Some(rug_fuzz_1));
        let repr_standard = to_string_repr(value, None, Some(rug_fuzz_2));
        debug_assert_ne!(
            repr_literal.as_raw().as_str(), repr_standard.as_raw().as_str()
        );
             }
}
}
}    }
    #[test]
    fn test_to_string_repr_with_inferred_literal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let repr_literal = to_string_repr(value, None, None);
        let repr_standard = to_string_repr(
            value,
            Some(StringStyle::OnelineSingle),
            None,
        );
        debug_assert_eq!(
            repr_literal.as_raw().as_str(), repr_standard.as_raw().as_str()
        );
             }
}
}
}    }
    #[test]
    fn test_to_string_repr_with_special_chars() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let repr = to_string_repr(value, None, None);
        let expected = rug_fuzz_1;
        debug_assert_eq!(repr.as_raw().as_str(), Some(expected));
             }
}
}
}    }
}
