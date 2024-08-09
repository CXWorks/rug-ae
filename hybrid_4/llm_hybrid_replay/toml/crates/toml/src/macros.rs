pub use serde::de::{Deserialize, IntoDeserializer};
use crate::value::{Array, Table, Value};
/// Construct a [`Table`] from TOML syntax.
///
/// ```rust
/// let cargo_toml = toml::toml! {
///     [package]
///     name = "toml"
///     version = "0.4.5"
///     authors = ["Alex Crichton <alex@alexcrichton.com>"]
///
///     [badges]
///     travis-ci = { repository = "alexcrichton/toml-rs" }
///
///     [dependencies]
///     serde = "1.0"
///
///     [dev-dependencies]
///     serde_derive = "1.0"
///     serde_json = "1.0"
/// };
///
/// println!("{:#?}", cargo_toml);
/// ```
#[macro_export]
macro_rules! toml {
    ($($toml:tt)+) => {
        { let table = $crate ::value::Table::new(); let mut root = $crate
        ::Value::Table(table); $crate ::toml_internal!(@ toplevel root[] $($toml)+);
        match root { $crate ::Value::Table(table) => table, _ => unreachable!(), } }
    };
}
#[macro_export]
#[doc(hidden)]
macro_rules! toml_internal {
    (@ toplevel $root:ident [$($path:tt)*]) => {};
    (@ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = - $v:tt $($rest:tt)*) => {
        $crate ::toml_internal!(@ toplevel $root [$($path)*] $($($k)-+).+ = (-$v)
        $($rest)*);
    };
    (@ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = + $v:tt $($rest:tt)*) => {
        $crate ::toml_internal!(@ toplevel $root [$($path)*] $($($k)-+).+ = ($v)
        $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt
        : $min:tt : $sec:tt . $frac:tt - $tzh:tt : $tzm:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $dhr : $min : $sec . $frac - $tzh : $tzm) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt
        $hr:tt : $min:tt : $sec:tt . $frac:tt - $tzh:tt : $tzm:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $day T $hr : $min : $sec . $frac - $tzh : $tzm) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt
        : $min:tt : $sec:tt - $tzh:tt : $tzm:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $dhr : $min : $sec - $tzh : $tzm) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt
        $hr:tt : $min:tt : $sec:tt - $tzh:tt : $tzm:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $day T $hr : $min : $sec - $tzh : $tzm) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt
        : $min:tt : $sec:tt . $frac:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $dhr : $min : $sec . $frac) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt
        $hr:tt : $min:tt : $sec:tt . $frac:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $day T $hr : $min : $sec . $frac) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt
        : $min:tt : $sec:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $dhr : $min : $sec) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt
        $hr:tt : $min:tt : $sec:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $day T $hr : $min : $sec) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt
        $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($yr
        - $mo - $day) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $hr:tt : $min:tt :
        $sec:tt . $frac:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($hr
        : $min : $sec . $frac) $($rest)*);
    };
    (
        @ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $hr:tt : $min:tt :
        $sec:tt $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ topleveldatetime $root [$($path)*] $($($k)-+).+ = ($hr
        : $min : $sec) $($rest)*);
    };
    (@ toplevel $root:ident [$($path:tt)*] $($($k:tt)-+).+ = $v:tt $($rest:tt)*) => {
        { $crate ::macros::insert_toml(& mut $root, & [$($path)* $(& concat!($("-",
        $crate ::toml_internal!(@ path $k),)+) [1..],)+], $crate ::toml_internal!(@ value
        $v)); $crate ::toml_internal!(@ toplevel $root [$($path)*] $($rest)*); }
    };
    (@ toplevel $root:ident $oldpath:tt [[$($($path:tt)-+).+]] $($rest:tt)*) => {
        $crate ::macros::push_toml(& mut $root, & [$(& concat!($("-", $crate
        ::toml_internal!(@ path $path),)+) [1..],)+]); $crate ::toml_internal!(@ toplevel
        $root [$(& concat!($("-", $crate ::toml_internal!(@ path $path),)+) [1..],)+]
        $($rest)*);
    };
    (@ toplevel $root:ident $oldpath:tt [$($($path:tt)-+).+] $($rest:tt)*) => {
        $crate ::macros::insert_toml(& mut $root, & [$(& concat!($("-", $crate
        ::toml_internal!(@ path $path),)+) [1..],)+], $crate ::Value::Table($crate
        ::value::Table::new())); $crate ::toml_internal!(@ toplevel $root [$(&
        concat!($("-", $crate ::toml_internal!(@ path $path),)+) [1..],)+] $($rest)*);
    };
    (
        @ topleveldatetime $root:ident [$($path:tt)*] $($($k:tt)-+).+ =
        ($($datetime:tt)+) $($rest:tt)*
    ) => {
        $crate ::macros::insert_toml(& mut $root, & [$($path)* $(& concat!($("-", $crate
        ::toml_internal!(@ path $k),)+) [1..],)+], $crate
        ::Value::Datetime(concat!($(stringify!($datetime)),+) .parse().unwrap())); $crate
        ::toml_internal!(@ toplevel $root [$($path)*] $($rest)*);
    };
    (@ path $ident:ident) => {
        stringify!($ident)
    };
    (@ path $quoted:tt) => {
        $quoted
    };
    (@ value { $($inline:tt)* }) => {
        { let mut table = $crate ::Value::Table($crate ::value::Table::new()); $crate
        ::toml_internal!(@ trailingcomma(@ table table) $($inline)*); table }
    };
    (@ value[$($inline:tt)*]) => {
        { let mut array = $crate ::value::Array::new(); $crate ::toml_internal!(@
        trailingcomma(@ array array) $($inline)*); $crate ::Value::Array(array) }
    };
    (@ value(- nan)) => {
        $crate ::Value::Float(-::std::f64::NAN)
    };
    (@ value(nan)) => {
        $crate ::Value::Float(::std::f64::NAN)
    };
    (@ value nan) => {
        $crate ::Value::Float(::std::f64::NAN)
    };
    (@ value(- inf)) => {
        $crate ::Value::Float(::std::f64::NEG_INFINITY)
    };
    (@ value(inf)) => {
        $crate ::Value::Float(::std::f64::INFINITY)
    };
    (@ value inf) => {
        $crate ::Value::Float(::std::f64::INFINITY)
    };
    (@ value $v:tt) => {
        { let de = $crate ::macros::IntoDeserializer::<$crate ::de::Error
        >::into_deserializer($v); <$crate ::Value as $crate ::macros::Deserialize
        >::deserialize(de).unwrap() }
    };
    (@ table $root:ident) => {};
    (@ table $root:ident $($($k:tt)-+).+ = - $v:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ table $root $($($k)-+).+ = (-$v), $($rest)*);
    };
    (@ table $root:ident $($($k:tt)-+).+ = + $v:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ table $root $($($k)-+).+ = ($v), $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt : $min:tt :
        $sec:tt . $frac:tt - $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $dhr :
        $min : $sec . $frac - $tzh : $tzm) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt
        : $sec:tt . $frac:tt - $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $day T
        $hr : $min : $sec . $frac - $tzh : $tzm) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt : $min:tt :
        $sec:tt - $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $dhr :
        $min : $sec - $tzh : $tzm) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt
        : $sec:tt - $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $day T
        $hr : $min : $sec - $tzh : $tzm) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt : $min:tt :
        $sec:tt . $frac:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $dhr :
        $min : $sec . $frac) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt
        : $sec:tt . $frac:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $day T
        $hr : $min : $sec . $frac) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $dhr:tt : $min:tt :
        $sec:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $dhr :
        $min : $sec) $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt
        : $sec:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $day T
        $hr : $min : $sec) $($rest)*);
    };
    (@ table $root:ident $($($k:tt)-+).+ = $yr:tt - $mo:tt - $day:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($yr - $mo - $day)
        $($rest)*);
    };
    (
        @ table $root:ident $($($k:tt)-+).+ = $hr:tt : $min:tt : $sec:tt . $frac:tt,
        $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($hr : $min : $sec .
        $frac) $($rest)*);
    };
    (@ table $root:ident $($($k:tt)-+).+ = $hr:tt : $min:tt : $sec:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ tabledatetime $root $($($k)-+).+ = ($hr : $min : $sec)
        $($rest)*);
    };
    (@ table $root:ident $($($k:tt)-+).+ = $v:tt, $($rest:tt)*) => {
        $crate ::macros::insert_toml(& mut $root, & [$(& concat!($("-", $crate
        ::toml_internal!(@ path $k),)+) [1..],)+], $crate ::toml_internal!(@ value $v));
        $crate ::toml_internal!(@ table $root $($rest)*);
    };
    (@ tabledatetime $root:ident $($($k:tt)-+).+ = ($($datetime:tt)*) $($rest:tt)*) => {
        $crate ::macros::insert_toml(& mut $root, & [$(& concat!($("-", $crate
        ::toml_internal!(@ path $k),)+) [1..],)+], $crate
        ::Value::Datetime(concat!($(stringify!($datetime)),+) .parse().unwrap())); $crate
        ::toml_internal!(@ table $root $($rest)*);
    };
    (@ array $root:ident) => {};
    (@ array $root:ident - $v:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ array $root (-$v), $($rest)*);
    };
    (@ array $root:ident + $v:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ array $root ($v), $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $dhr:tt : $min:tt : $sec:tt . $frac:tt -
        $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $dhr : $min : $sec .
        $frac - $tzh : $tzm) $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt : $sec:tt .
        $frac:tt - $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $day T $hr : $min :
        $sec . $frac - $tzh : $tzm) $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $dhr:tt : $min:tt : $sec:tt - $tzh:tt :
        $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $dhr : $min : $sec -
        $tzh : $tzm) $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt : $sec:tt -
        $tzh:tt : $tzm:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $day T $hr : $min :
        $sec - $tzh : $tzm) $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $dhr:tt : $min:tt : $sec:tt . $frac:tt,
        $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $dhr : $min : $sec .
        $frac) $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt : $sec:tt .
        $frac:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $day T $hr : $min :
        $sec . $frac) $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $dhr:tt : $min:tt : $sec:tt, $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $dhr : $min : $sec)
        $($rest)*);
    };
    (
        @ array $root:ident $yr:tt - $mo:tt - $day:tt $hr:tt : $min:tt : $sec:tt,
        $($rest:tt)*
    ) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $day T $hr : $min :
        $sec) $($rest)*);
    };
    (@ array $root:ident $yr:tt - $mo:tt - $day:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ arraydatetime $root ($yr - $mo - $day) $($rest)*);
    };
    (@ array $root:ident $hr:tt : $min:tt : $sec:tt . $frac:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ arraydatetime $root ($hr : $min : $sec . $frac)
        $($rest)*);
    };
    (@ array $root:ident $hr:tt : $min:tt : $sec:tt, $($rest:tt)*) => {
        $crate ::toml_internal!(@ arraydatetime $root ($hr : $min : $sec) $($rest)*);
    };
    (@ array $root:ident $v:tt, $($rest:tt)*) => {
        $root .push($crate ::toml_internal!(@ value $v)); $crate ::toml_internal!(@ array
        $root $($rest)*);
    };
    (@ arraydatetime $root:ident ($($datetime:tt)*) $($rest:tt)*) => {
        $root .push($crate ::Value::Datetime(concat!($(stringify!($datetime)),+) .parse()
        .unwrap())); $crate ::toml_internal!(@ array $root $($rest)*);
    };
    (@ trailingcomma($($args:tt)*)) => {
        $crate ::toml_internal!($($args)*);
    };
    (@ trailingcomma($($args:tt)*),) => {
        $crate ::toml_internal!($($args)*,);
    };
    (@ trailingcomma($($args:tt)*) $last:tt) => {
        $crate ::toml_internal!($($args)* $last,);
    };
    (@ trailingcomma($($args:tt)*) $first:tt $($rest:tt)+) => {
        $crate ::toml_internal!(@ trailingcomma($($args)* $first) $($rest)+);
    };
}
pub fn insert_toml(root: &mut Value, path: &[&str], value: Value) {
    *traverse(root, path) = value;
}
pub fn push_toml(root: &mut Value, path: &[&str]) {
    let target = traverse(root, path);
    if !target.is_array() {
        *target = Value::Array(Array::new());
    }
    target.as_array_mut().unwrap().push(Value::Table(Table::new()));
}
fn traverse<'a>(root: &'a mut Value, path: &[&str]) -> &'a mut Value {
    let mut cur = root;
    for &key in path {
        let cur1 = cur;
        let cur2 = if cur1.is_array() {
            cur1.as_array_mut().unwrap().last_mut().unwrap()
        } else {
            cur1
        };
        if !cur2.is_table() {
            *cur2 = Value::Table(Table::new());
        }
        if !cur2.as_table().unwrap().contains_key(key) {
            let empty = Value::Table(Table::new());
            cur2.as_table_mut().unwrap().insert(key.to_owned(), empty);
        }
        cur = cur2.as_table_mut().unwrap().get_mut(key).unwrap();
    }
    cur
}
#[cfg(test)]
mod tests_llm_16_275 {
    use super::*;
    use crate::*;
    use crate::Value;
    #[test]
    fn insert_string_into_root() {
        let _rug_st_tests_llm_16_275_rrrruuuugggg_insert_string_into_root = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = "key";
        let mut root = Value::Table(crate::map::Map::new());
        let path = vec![rug_fuzz_0];
        let value = Value::String(rug_fuzz_1.to_string());
        macros::insert_toml(&mut root, &path, value.clone());
        debug_assert_eq!(root.get(rug_fuzz_2), Some(& value));
        let _rug_ed_tests_llm_16_275_rrrruuuugggg_insert_string_into_root = 0;
    }
    #[test]
    fn insert_integer_into_root() {
        let _rug_st_tests_llm_16_275_rrrruuuugggg_insert_integer_into_root = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = "key";
        let mut root = Value::Table(crate::map::Map::new());
        let path = vec![rug_fuzz_0];
        let value = Value::Integer(rug_fuzz_1);
        macros::insert_toml(&mut root, &path, value.clone());
        debug_assert_eq!(root.get(rug_fuzz_2), Some(& value));
        let _rug_ed_tests_llm_16_275_rrrruuuugggg_insert_integer_into_root = 0;
    }
    #[test]
    fn insert_nested_string() {
        let _rug_st_tests_llm_16_275_rrrruuuugggg_insert_nested_string = 0;
        let rug_fuzz_0 = "nested";
        let rug_fuzz_1 = "nested_value";
        let rug_fuzz_2 = "nested";
        let rug_fuzz_3 = "key";
        let mut root = Value::Table(crate::map::Map::new());
        let path = vec![rug_fuzz_0, "key"];
        let value = Value::String(rug_fuzz_1.to_string());
        macros::insert_toml(&mut root, &path, value.clone());
        let nested = root.get(rug_fuzz_2).unwrap().get(rug_fuzz_3);
        debug_assert_eq!(nested, Some(& value));
        let _rug_ed_tests_llm_16_275_rrrruuuugggg_insert_nested_string = 0;
    }
    #[test]
    fn insert_into_non_table() {
        let _rug_st_tests_llm_16_275_rrrruuuugggg_insert_into_non_table = 0;
        let rug_fuzz_0 = "I am not a table";
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "value";
        let rug_fuzz_3 = "key";
        let mut root = Value::String(rug_fuzz_0.to_string());
        let path = vec![rug_fuzz_1];
        let value = Value::String(rug_fuzz_2.to_string());
        macros::insert_toml(&mut root, &path, value.clone());
        debug_assert_eq!(root.get(rug_fuzz_3), None);
        let _rug_ed_tests_llm_16_275_rrrruuuugggg_insert_into_non_table = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_276 {
    use super::*;
    use crate::*;
    use crate::value::{Table, Value};
    fn setup_empty_table() -> Value {
        Value::Table(Table::new())
    }
    fn setup_table_with_array() -> Value {
        let mut table = Table::new();
        table.insert("test".to_string(), Value::Array(vec![Value::Table(Table::new())]));
        Value::Table(table)
    }
    fn setup_table_with_non_array() -> Value {
        let mut table = Table::new();
        table.insert("test".to_string(), Value::Integer(42));
        Value::Table(table)
    }
    #[test]
    fn push_toml_creates_array_if_none_exists() {
        let mut root = setup_empty_table();
        let path = ["new_array"];
        push_toml(&mut root, &path);
        assert!(root.get("new_array").unwrap().is_array());
    }
    #[test]
    fn push_toml_pushes_table_to_existing_array() {
        let mut root = setup_table_with_array();
        let path = ["test"];
        let initial_length = root.get("test").unwrap().as_array().unwrap().len();
        push_toml(&mut root, &path);
        let array = root.get("test").unwrap().as_array().unwrap();
        assert!(array.len() == initial_length + 1);
        assert!(array.last().unwrap().is_table());
    }
    #[test]
    fn push_toml_converts_non_array_to_array() {
        let mut root = setup_table_with_non_array();
        let path = ["test"];
        push_toml(&mut root, &path);
        let value = root.get("test").unwrap();
        assert!(value.is_array());
        assert!(value.as_array().unwrap().len() == 1);
        assert!(value.as_array().unwrap().last().unwrap().is_table());
    }
}
#[cfg(test)]
mod tests_rug_79 {
    use crate::Value;
    use crate::map::Map;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_79_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Sample string value";
        let rug_fuzz_1 = "a";
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = "c";
        let mut p0 = Value::from(rug_fuzz_0);
        let mut p1 = [rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        crate::macros::traverse(&mut p0, &p1);
        let _rug_ed_tests_rug_79_rrrruuuugggg_test_rug = 0;
    }
}
