use super::Value;
use crate::map::Map;
use crate::number::Number;
use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::iter::FromIterator;
macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(impl From <$ty > for Value { fn from(n : $ty) -> Self { Value::Number(n.into())
        } })*
    };
}
from_integer! {
    i8 i16 i32 i64 isize u8 u16 u32 u64 usize
}
#[cfg(feature = "arbitrary_precision")]
from_integer! {
    i128 u128
}
impl From<f32> for Value {
    /// Convert 32-bit floating point number to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f32 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f32) -> Self {
        Number::from_f32(f).map_or(Value::Null, Value::Number)
    }
}
impl From<f64> for Value {
    /// Convert 64-bit floating point number to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f64 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(Value::Null, Value::Number)
    }
}
impl From<bool> for Value {
    /// Convert boolean to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let b = false;
    /// let x: Value = b.into();
    /// ```
    fn from(f: bool) -> Self {
        Value::Bool(f)
    }
}
impl From<String> for Value {
    /// Convert `String` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: Value = s.into();
    /// ```
    fn from(f: String) -> Self {
        Value::String(f)
    }
}
impl<'a> From<&'a str> for Value {
    /// Convert string slice to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: &str = "lorem";
    /// let x: Value = s.into();
    /// ```
    fn from(f: &str) -> Self {
        Value::String(f.to_string())
    }
}
impl<'a> From<Cow<'a, str>> for Value {
    /// Convert copy-on-write string to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("lorem");
    /// let x: Value = s.into();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Owned("lorem".to_string());
    /// let x: Value = s.into();
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.into_owned())
    }
}
impl From<Number> for Value {
    /// Convert `Number` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Number, Value};
    ///
    /// let n = Number::from(7);
    /// let x: Value = n.into();
    /// ```
    fn from(f: Number) -> Self {
        Value::Number(f)
    }
}
impl From<Map<String, Value>> for Value {
    /// Convert map (with string keys) to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Map, Value};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_string(), "ipsum".into());
    /// let x: Value = m.into();
    /// ```
    fn from(f: Map<String, Value>) -> Self {
        Value::Object(f)
    }
}
impl<T: Into<Value>> From<Vec<T>> for Value {
    /// Convert a `Vec` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: Vec<T>) -> Self {
        Value::Array(f.into_iter().map(Into::into).collect())
    }
}
impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    /// Convert a slice to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: &'a [T]) -> Self {
        Value::Array(f.iter().cloned().map(Into::into).collect())
    }
}
impl<T: Into<Value>> FromIterator<T> for Value {
    /// Convert an iteratable type to a `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: Value = v.collect();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into_iter().collect();
    /// ```
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use serde_json::Value;
    ///
    /// let x: Value = Value::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Array(iter.into_iter().map(Into::into).collect())
    }
}
impl<K: Into<String>, V: Into<Value>> FromIterator<(K, V)> for Value {
    /// Convert an iteratable type to a `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec![("lorem", 40), ("ipsum", 2)];
    /// let x: Value = v.into_iter().collect();
    /// ```
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Object(iter.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}
impl From<()> for Value {
    /// Convert `()` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let u = ();
    /// let x: Value = u.into();
    /// ```
    fn from((): ()) -> Self {
        Value::Null
    }
}
impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Value::Null,
            Some(value) => Into::into(value),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_704_llm_16_704 {
    use crate::Value;
    #[test]
    fn it_converts_empty_slices_to_empty_arrays() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_empty_slices_to_empty_arrays = 0;
        let empty_slice: &[&str] = &[];
        let empty_array = Value::from(empty_slice);
        debug_assert_eq!(empty_array, Value::Array(vec![]));
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_empty_slices_to_empty_arrays = 0;
    }
    #[test]
    fn it_converts_slices_of_values_to_arrays() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_values_to_arrays = 0;
        let rug_fuzz_0 = "true";
        let rug_fuzz_1 = "null";
        let rug_fuzz_2 = "42";
        let slice: &[&str] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let array = Value::from(slice);
        debug_assert_eq!(
            array, Value::Array(vec![Value::from("true"), Value::from("null"),
            Value::from("42"),])
        );
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_values_to_arrays = 0;
    }
    #[test]
    fn it_converts_slices_of_strings_to_arrays() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_strings_to_arrays = 0;
        let rug_fuzz_0 = "lorem";
        let rug_fuzz_1 = "ipsum";
        let rug_fuzz_2 = "dolor";
        let slice: &[&str] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let array = Value::from(slice);
        debug_assert_eq!(
            array, Value::Array(vec![Value::from("lorem"), Value::from("ipsum"),
            Value::from("dolor"),])
        );
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_strings_to_arrays = 0;
    }
    #[test]
    fn it_converts_slices_of_ints_to_arrays() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_ints_to_arrays = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 20;
        let rug_fuzz_2 = 30;
        let slice: &[i32] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let array = Value::from(slice);
        debug_assert_eq!(
            array, Value::Array(vec![Value::from(10), Value::from(20), Value::from(30),])
        );
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_ints_to_arrays = 0;
    }
    #[test]
    fn it_converts_slices_of_floats_to_arrays() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_floats_to_arrays = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 20.1;
        let rug_fuzz_2 = 30.2;
        let slice: &[f64] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let array = Value::from(slice);
        debug_assert_eq!(
            array, Value::Array(vec![Value::from(10.0), Value::from(20.1),
            Value::from(30.2),])
        );
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_floats_to_arrays = 0;
    }
    #[test]
    fn it_converts_slices_of_bools_to_arrays() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_bools_to_arrays = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = true;
        let slice: &[bool] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let array = Value::from(slice);
        debug_assert_eq!(
            array, Value::Array(vec![Value::from(true), Value::from(false),
            Value::from(true),])
        );
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_it_converts_slices_of_bools_to_arrays = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_705 {
    use crate::{value::Value, from_str};
    #[test]
    fn from_str_literal() {
        let _rug_st_tests_llm_16_705_rrrruuuugggg_from_str_literal = 0;
        let rug_fuzz_0 = "lorem";
        let s: &str = rug_fuzz_0;
        let x: Value = s.into();
        debug_assert_eq!(x, Value::String("lorem".to_owned()));
        let _rug_ed_tests_llm_16_705_rrrruuuugggg_from_str_literal = 0;
    }
    #[test]
    fn from_str_empty() {
        let _rug_st_tests_llm_16_705_rrrruuuugggg_from_str_empty = 0;
        let rug_fuzz_0 = "";
        let s: &str = rug_fuzz_0;
        let x: Value = s.into();
        debug_assert_eq!(x, Value::String("".to_owned()));
        let _rug_ed_tests_llm_16_705_rrrruuuugggg_from_str_empty = 0;
    }
    #[test]
    fn from_str_json_encoded() {
        let _rug_st_tests_llm_16_705_rrrruuuugggg_from_str_json_encoded = 0;
        let rug_fuzz_0 = "\"json string\"";
        let s: &str = rug_fuzz_0;
        let x: Value = s.into();
        debug_assert_eq!(x, Value::String("\"json string\"".to_owned()));
        let _rug_ed_tests_llm_16_705_rrrruuuugggg_from_str_json_encoded = 0;
    }
    #[test]
    fn from_str_deserialize() {
        let _rug_st_tests_llm_16_705_rrrruuuugggg_from_str_deserialize = 0;
        let rug_fuzz_0 = "\"lorem\"";
        let s: &str = rug_fuzz_0;
        let x: Value = from_str(s).unwrap();
        debug_assert_eq!(x, Value::String("lorem".to_owned()));
        let _rug_ed_tests_llm_16_705_rrrruuuugggg_from_str_deserialize = 0;
    }
    #[test]
    #[should_panic]
    fn from_str_deserialize_invalid_json() {
        let _rug_st_tests_llm_16_705_rrrruuuugggg_from_str_deserialize_invalid_json = 0;
        let rug_fuzz_0 = "lorem";
        let s: &str = rug_fuzz_0;
        let _: Value = from_str(s).unwrap();
        let _rug_ed_tests_llm_16_705_rrrruuuugggg_from_str_deserialize_invalid_json = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_706 {
    use crate::value::Value;
    #[test]
    fn test_from_unit_to_null_value() {
        let _rug_st_tests_llm_16_706_rrrruuuugggg_test_from_unit_to_null_value = 0;
        let unit = ();
        let value: Value = Value::from(unit);
        debug_assert_eq!(value, Value::Null);
        let _rug_ed_tests_llm_16_706_rrrruuuugggg_test_from_unit_to_null_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_707 {
    use crate::Value;
    #[test]
    fn test_from_bool_to_value() {
        let _rug_st_tests_llm_16_707_rrrruuuugggg_test_from_bool_to_value = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert_eq!(Value::from(rug_fuzz_0), Value::Bool(true));
        debug_assert_eq!(Value::from(rug_fuzz_1), Value::Bool(false));
        let _rug_ed_tests_llm_16_707_rrrruuuugggg_test_from_bool_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_708 {
    use crate::value::{Number, Value};
    #[test]
    fn from_f32_non_finite() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_non_finite = 0;
        debug_assert_eq!(Value::from(f32::NAN), Value::Null);
        debug_assert_eq!(Value::from(f32::INFINITY), Value::Null);
        debug_assert_eq!(Value::from(f32::NEG_INFINITY), Value::Null);
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_non_finite = 0;
    }
    #[test]
    fn from_f32_finite() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_finite = 0;
        let rug_fuzz_0 = 13.37f32;
        let value = Value::from(rug_fuzz_0);
        let number = match value {
            Value::Number(num) => num,
            _ => panic!("Value is not a number"),
        };
        debug_assert!(number.is_f64());
        debug_assert_eq!(number.as_f64(), Some(13.37f64));
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_finite = 0;
    }
    #[test]
    fn from_f32_zero() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let value = Value::from(rug_fuzz_0);
        let number = match value {
            Value::Number(num) => num,
            _ => panic!("Value is not a number"),
        };
        debug_assert_eq!(number.as_f64(), Some(0.0f64));
        debug_assert_eq!(number.as_f32(), Some(0.0f32));
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_zero = 0;
    }
    #[test]
    fn from_f32_max() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_max = 0;
        let value = Value::from(f32::MAX);
        let number = match value {
            Value::Number(num) => num,
            _ => panic!("Value is not a number"),
        };
        debug_assert!(number.as_f64().is_some());
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_max = 0;
    }
    #[test]
    fn from_f32_min() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_min = 0;
        let value = Value::from(f32::MIN);
        let number = match value {
            Value::Number(num) => num,
            _ => panic!("Value is not a number"),
        };
        debug_assert!(number.as_f64().is_some());
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_min = 0;
    }
    #[test]
    fn from_f32_min_positive() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_min_positive = 0;
        let value = Value::from(f32::MIN_POSITIVE);
        let number = match value {
            Value::Number(num) => num,
            _ => panic!("Value is not a number"),
        };
        debug_assert_eq!(number.as_f64(), Some(f32::MIN_POSITIVE as f64));
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_min_positive = 0;
    }
    #[test]
    fn from_f32_eps() {
        let _rug_st_tests_llm_16_708_rrrruuuugggg_from_f32_eps = 0;
        let value = Value::from(f32::EPSILON);
        let number = match value {
            Value::Number(num) => num,
            _ => panic!("Value is not a number"),
        };
        debug_assert_eq!(number.as_f64(), Some(f32::EPSILON as f64));
        let _rug_ed_tests_llm_16_708_rrrruuuugggg_from_f32_eps = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_709_llm_16_709 {
    use crate::value::{Value, Number};
    #[test]
    fn test_from_f64_null() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_null = 0;
        debug_assert_eq!(Value::from(f64::NAN), Value::Null);
        debug_assert_eq!(Value::from(f64::INFINITY), Value::Null);
        debug_assert_eq!(Value::from(f64::NEG_INFINITY), Value::Null);
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_null = 0;
    }
    #[test]
    fn test_from_f64_number() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_number = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 2.71;
        let rug_fuzz_2 = 0.0;
        debug_assert_eq!(
            Value::from(rug_fuzz_0), Value::Number(Number::from_f64(3.14).unwrap())
        );
        debug_assert_eq!(
            Value::from(- rug_fuzz_1), Value::Number(Number::from_f64(- 2.71).unwrap())
        );
        debug_assert_eq!(
            Value::from(rug_fuzz_2), Value::Number(Number::from_f64(0.0).unwrap())
        );
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_number = 0;
    }
    #[test]
    fn test_from_f64_edge_cases() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_edge_cases = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        debug_assert_eq!(
            Value::from(rug_fuzz_0), Value::Number(Number::from_f64(0.0).unwrap())
        );
        debug_assert_eq!(
            Value::from(- rug_fuzz_1), Value::Number(Number::from_f64(0.0).unwrap())
        );
        debug_assert_eq!(
            Value::from(f64::MIN), Value::Number(Number::from_f64(f64::MIN).unwrap())
        );
        debug_assert_eq!(
            Value::from(f64::MAX), Value::Number(Number::from_f64(f64::MAX).unwrap())
        );
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_edge_cases = 0;
    }
    #[test]
    fn test_from_f64_integer_values() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_integer_values = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        debug_assert_eq!(
            Value::from(rug_fuzz_0), Value::Number(Number::from_f64(42.0).unwrap())
        );
        debug_assert_eq!(
            Value::from(- rug_fuzz_1), Value::Number(Number::from_f64(- 42.0).unwrap())
        );
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_f64_integer_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_710 {
    use crate::value::Value;
    use std::convert::From;
    #[test]
    fn test_from_i16_to_value() {
        let _rug_st_tests_llm_16_710_rrrruuuugggg_test_from_i16_to_value = 0;
        let rug_fuzz_0 = 42;
        let n: i16 = rug_fuzz_0;
        let value: Value = Value::from(n);
        let expected = Value::Number(n.into());
        debug_assert_eq!(value, expected);
        let _rug_ed_tests_llm_16_710_rrrruuuugggg_test_from_i16_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_711 {
    use crate::value::Value;
    #[test]
    fn test_from_i32_for_value() {
        let _rug_st_tests_llm_16_711_rrrruuuugggg_test_from_i32_for_value = 0;
        let rug_fuzz_0 = 123;
        let num: i32 = rug_fuzz_0;
        let json_value: Value = Value::from(num);
        debug_assert_eq!(json_value, Value::Number(num.into()));
        let _rug_ed_tests_llm_16_711_rrrruuuugggg_test_from_i32_for_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_712 {
    use crate::value::Value;
    use std::convert::From;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_712_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let num = rug_fuzz_0;
        let value = Value::from(num);
        debug_assert!(value.is_number());
        debug_assert_eq!(value.as_i64().unwrap(), num);
        let _rug_ed_tests_llm_16_712_rrrruuuugggg_test_from_i64 = 0;
    }
    #[test]
    fn test_from_i64_negative() {
        let _rug_st_tests_llm_16_712_rrrruuuugggg_test_from_i64_negative = 0;
        let rug_fuzz_0 = 42i64;
        let num = -rug_fuzz_0;
        let value = Value::from(num);
        debug_assert!(value.is_number());
        debug_assert_eq!(value.as_i64().unwrap(), num);
        let _rug_ed_tests_llm_16_712_rrrruuuugggg_test_from_i64_negative = 0;
    }
    #[test]
    fn test_from_i64_min() {
        let _rug_st_tests_llm_16_712_rrrruuuugggg_test_from_i64_min = 0;
        let num = i64::MIN;
        let value = Value::from(num);
        debug_assert!(value.is_number());
        debug_assert_eq!(value.as_i64().unwrap(), num);
        let _rug_ed_tests_llm_16_712_rrrruuuugggg_test_from_i64_min = 0;
    }
    #[test]
    fn test_from_i64_max() {
        let _rug_st_tests_llm_16_712_rrrruuuugggg_test_from_i64_max = 0;
        let num = i64::MAX;
        let value = Value::from(num);
        debug_assert!(value.is_number());
        debug_assert_eq!(value.as_i64().unwrap(), num);
        let _rug_ed_tests_llm_16_712_rrrruuuugggg_test_from_i64_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_713 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_llm_16_713_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 42;
        let num: i8 = -rug_fuzz_0;
        let json_value = Value::from(num);
        debug_assert_eq!(json_value, Value::Number(num.into()));
        debug_assert!(json_value.is_i64());
        debug_assert_eq!(json_value.as_i64(), Some(num as i64));
        debug_assert!(json_value.is_number());
        let _rug_ed_tests_llm_16_713_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_714_from_impl_for_value {
    use crate::value::Value;
    use std::convert::From;
    #[test]
    fn from_isize_for_value() {
        let _rug_st_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_for_value = 0;
        let rug_fuzz_0 = 42;
        let i: isize = -rug_fuzz_0;
        let v: Value = Value::from(i);
        debug_assert!(v.is_number());
        debug_assert_eq!(v, Value::Number(i.into()));
        let _rug_ed_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_for_value = 0;
    }
    #[test]
    fn from_isize_zero_for_value() {
        let _rug_st_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_zero_for_value = 0;
        let rug_fuzz_0 = 0;
        let i: isize = rug_fuzz_0;
        let v: Value = Value::from(i);
        debug_assert!(v.is_number());
        debug_assert_eq!(v, Value::Number(i.into()));
        let _rug_ed_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_zero_for_value = 0;
    }
    #[test]
    fn from_isize_positive_for_value() {
        let _rug_st_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_positive_for_value = 0;
        let rug_fuzz_0 = 42;
        let i: isize = rug_fuzz_0;
        let v: Value = Value::from(i);
        debug_assert!(v.is_number());
        debug_assert_eq!(v, Value::Number(i.into()));
        let _rug_ed_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_positive_for_value = 0;
    }
    #[test]
    fn from_isize_max_for_value() {
        let _rug_st_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_max_for_value = 0;
        let i: isize = isize::MAX;
        let v: Value = Value::from(i);
        debug_assert!(v.is_number());
        debug_assert_eq!(v, Value::Number(i.into()));
        let _rug_ed_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_max_for_value = 0;
    }
    #[test]
    fn from_isize_min_for_value() {
        let _rug_st_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_min_for_value = 0;
        let i: isize = isize::MIN;
        let v: Value = Value::from(i);
        debug_assert!(v.is_number());
        debug_assert_eq!(v, Value::Number(i.into()));
        let _rug_ed_tests_llm_16_714_from_impl_for_value_rrrruuuugggg_from_isize_min_for_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_716_llm_16_716 {
    use crate::Number;
    use crate::value::Value;
    #[test]
    fn test_from_number_to_value() {
        let _rug_st_tests_llm_16_716_llm_16_716_rrrruuuugggg_test_from_number_to_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 42u64;
        let rug_fuzz_3 = 42;
        let num_i64 = Number::from(rug_fuzz_0);
        let num_f64 = Number::from_f64(rug_fuzz_1).unwrap();
        let num_u64 = Number::from(rug_fuzz_2);
        let num_neg_i64 = Number::from(-rug_fuzz_3);
        debug_assert_eq!(Value::from(num_i64.clone()), Value::Number(num_i64));
        debug_assert_eq!(Value::from(num_f64.clone()), Value::Number(num_f64));
        debug_assert_eq!(Value::from(num_u64.clone()), Value::Number(num_u64));
        debug_assert_eq!(Value::from(num_neg_i64.clone()), Value::Number(num_neg_i64));
        let _rug_ed_tests_llm_16_716_llm_16_716_rrrruuuugggg_test_from_number_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_717 {
    use std::borrow::Cow;
    use crate::Value;
    #[test]
    fn from_cow_borrowed_str() {
        let _rug_st_tests_llm_16_717_rrrruuuugggg_from_cow_borrowed_str = 0;
        let rug_fuzz_0 = "borrowed";
        let cow: Cow<'_, str> = Cow::Borrowed(rug_fuzz_0);
        let value: Value = cow.into();
        debug_assert_eq!(value, Value::String("borrowed".to_owned()));
        let _rug_ed_tests_llm_16_717_rrrruuuugggg_from_cow_borrowed_str = 0;
    }
    #[test]
    fn from_cow_owned_str() {
        let _rug_st_tests_llm_16_717_rrrruuuugggg_from_cow_owned_str = 0;
        let rug_fuzz_0 = "owned";
        let cow: Cow<'_, str> = Cow::Owned(rug_fuzz_0.to_owned());
        let value: Value = cow.into();
        debug_assert_eq!(value, Value::String("owned".to_owned()));
        let _rug_ed_tests_llm_16_717_rrrruuuugggg_from_cow_owned_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_718 {
    use serde::{Deserialize, Serialize};
    use crate::{json, Value};
    use std::string::ToString;
    use crate::value::from_value;
    #[test]
    fn test_from_none() {
        let _rug_st_tests_llm_16_718_rrrruuuugggg_test_from_none = 0;
        let none_val: Option<i32> = None;
        let value: Value = from_value(none_val.into()).unwrap();
        debug_assert_eq!(value, Value::Null);
        let _rug_ed_tests_llm_16_718_rrrruuuugggg_test_from_none = 0;
    }
    #[test]
    fn test_from_some() {
        let _rug_st_tests_llm_16_718_rrrruuuugggg_test_from_some = 0;
        let rug_fuzz_0 = 123;
        let some_val = Some(rug_fuzz_0);
        let value: Value = from_value(some_val.into()).unwrap();
        debug_assert_eq!(value, Value::Number(123.into()));
        let _rug_ed_tests_llm_16_718_rrrruuuugggg_test_from_some = 0;
    }
    #[test]
    fn test_from_some_string() {
        let _rug_st_tests_llm_16_718_rrrruuuugggg_test_from_some_string = 0;
        let rug_fuzz_0 = "Hello, World!";
        let some_val = Some(rug_fuzz_0.to_string());
        let value: Value = from_value(some_val.into()).unwrap();
        debug_assert_eq!(value, Value::String("Hello, World!".to_string()));
        let _rug_ed_tests_llm_16_718_rrrruuuugggg_test_from_some_string = 0;
    }
    #[test]
    fn test_from_some_struct() {
        let _rug_st_tests_llm_16_718_rrrruuuugggg_test_from_some_struct = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "TestStruct";
        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            id: u32,
            name: String,
        }
        let my_struct = MyStruct {
            id: rug_fuzz_0,
            name: rug_fuzz_1.to_string(),
        };
        let some_val: Option<MyStruct> = Some(my_struct);
        let value: Value = from_value(crate::to_value(some_val).unwrap()).unwrap();
        let expected = json!({ "id" : 1, "name" : "TestStruct", });
        debug_assert_eq!(value, expected);
        let _rug_ed_tests_llm_16_718_rrrruuuugggg_test_from_some_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_719 {
    use crate::Value;
    #[test]
    fn string_into_value() {
        let _rug_st_tests_llm_16_719_rrrruuuugggg_string_into_value = 0;
        let rug_fuzz_0 = "test string";
        let string_value = rug_fuzz_0.to_owned();
        let json_value: Value = string_value.clone().into();
        debug_assert_eq!(json_value, Value::String(string_value));
        let _rug_ed_tests_llm_16_719_rrrruuuugggg_string_into_value = 0;
    }
    #[test]
    fn string_from_into_value() {
        let _rug_st_tests_llm_16_719_rrrruuuugggg_string_from_into_value = 0;
        let rug_fuzz_0 = "another test";
        let string_value = rug_fuzz_0.to_owned();
        let json_value: Value = Value::from(string_value.clone());
        debug_assert_eq!(json_value, Value::String(string_value));
        let _rug_ed_tests_llm_16_719_rrrruuuugggg_string_from_into_value = 0;
    }
    #[test]
    fn string_into_value_explicit() {
        let _rug_st_tests_llm_16_719_rrrruuuugggg_string_into_value_explicit = 0;
        let rug_fuzz_0 = "explicit conversion test";
        let string_value = rug_fuzz_0.to_owned();
        let json_value = Value::from(string_value.clone());
        debug_assert_eq!(json_value, Value::String(string_value));
        let _rug_ed_tests_llm_16_719_rrrruuuugggg_string_into_value_explicit = 0;
    }
    #[test]
    fn string_from_value_explicit_call() {
        let _rug_st_tests_llm_16_719_rrrruuuugggg_string_from_value_explicit_call = 0;
        let rug_fuzz_0 = "from specific type";
        let string_value = rug_fuzz_0.to_owned();
        let json_value = <Value as From<String>>::from(string_value.clone());
        debug_assert_eq!(json_value, Value::String(string_value));
        let _rug_ed_tests_llm_16_719_rrrruuuugggg_string_from_value_explicit_call = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_720_llm_16_720 {
    use crate::{Value, Number};
    #[test]
    fn test_from_empty_vec() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_empty_vec = 0;
        let v: Vec<Value> = Vec::new();
        let expected = Value::Array(vec![]);
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_empty_vec = 0;
    }
    #[test]
    fn test_from_vec_of_numbers() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_numbers = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let v = vec![
            Value::Number(Number::from(rug_fuzz_0)), Value::Number(Number::from_f64(3.14)
            .unwrap()), Value::Number(Number::from(- 7))
        ];
        let expected = Value::Array(
            vec![
                Value::Number(Number::from(rug_fuzz_1)),
                Value::Number(Number::from_f64(3.14).unwrap()),
                Value::Number(Number::from(- 7))
            ],
        );
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_numbers = 0;
    }
    #[test]
    fn test_from_vec_of_strings() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_strings = 0;
        let rug_fuzz_0 = "foo";
        let rug_fuzz_1 = "foo";
        let v = vec![rug_fuzz_0.to_string(), "bar".to_string(), "baz".to_string()];
        let expected = Value::Array(
            vec![
                Value::String(rug_fuzz_1.to_string()), Value::String("bar".to_string()),
                Value::String("baz".to_string())
            ],
        );
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_strings = 0;
    }
    #[test]
    fn test_from_vec_of_bools() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_bools = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = true;
        let v = vec![rug_fuzz_0, false, true];
        let expected = Value::Array(
            vec![Value::Bool(rug_fuzz_1), Value::Bool(false), Value::Bool(true)],
        );
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_bools = 0;
    }
    #[test]
    fn test_from_vec_of_mixed_types() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_mixed_types = 0;
        let rug_fuzz_0 = "foo";
        let rug_fuzz_1 = "foo";
        let v = vec![
            Value::String(rug_fuzz_0.to_string()), Value::Number(Number::from(42)),
            Value::Bool(true), Value::Null
        ];
        let expected = Value::Array(
            vec![
                Value::String(rug_fuzz_1.to_string()), Value::Number(Number::from(42)),
                Value::Bool(true), Value::Null
            ],
        );
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_mixed_types = 0;
    }
    #[test]
    fn test_from_vec_of_vecs() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_vecs = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let v = vec![
            vec![Value::Number(Number::from(rug_fuzz_0))],
            vec![Value::Number(Number::from(2)), Value::Number(Number::from(3))]
        ];
        let expected = Value::Array(
            vec![
                Value::Array(vec![Value::Number(Number::from(rug_fuzz_1))]),
                Value::Array(vec![Value::Number(Number::from(2)),
                Value::Number(Number::from(3))])
            ],
        );
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_vecs = 0;
    }
    #[test]
    fn test_from_vec_of_objects() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_objects = 0;
        let v = vec![
            crate ::json!({ "key1" : "value1" }) .as_object().unwrap().clone(), crate
            ::json!({ "key2" : "value2" }) .as_object().unwrap().clone()
        ];
        let expected = Value::Array(
            vec![
                crate ::json!({ "key1" : "value1" }), crate ::json!({ "key2" : "value2"
                })
            ],
        );
        debug_assert_eq!(Value::from(v), expected);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_test_from_vec_of_objects = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_721 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_from_u16_to_value() {
        let _rug_st_tests_llm_16_721_rrrruuuugggg_test_from_u16_to_value = 0;
        let rug_fuzz_0 = 256;
        let num: u16 = rug_fuzz_0;
        let expected_value = Value::Number(Number::from(num));
        debug_assert_eq!(Value::from(num), expected_value);
        let _rug_ed_tests_llm_16_721_rrrruuuugggg_test_from_u16_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_723 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_723_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 42;
        let num: u64 = rug_fuzz_0;
        let value: Value = Value::from(num);
        debug_assert!(value.is_number());
        debug_assert_eq!(value, Value::Number(Number::from(num)));
        debug_assert_eq!(value.as_u64(), Some(num));
        let _rug_ed_tests_llm_16_723_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_724 {
    use crate::Value;
    #[test]
    fn test_value_from_u8() {
        let _rug_st_tests_llm_16_724_rrrruuuugggg_test_value_from_u8 = 0;
        let rug_fuzz_0 = 42u8;
        let value = Value::from(rug_fuzz_0);
        debug_assert!(value.is_number());
        debug_assert_eq!(value, Value::Number(42u8.into()));
        let _rug_ed_tests_llm_16_724_rrrruuuugggg_test_value_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_725 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::convert::TryFrom;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_725_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 42usize;
        let num = rug_fuzz_0;
        let json_val = Value::from(num);
        debug_assert_eq!(json_val, Value::Number(Number::from(num as u64)));
        let _rug_ed_tests_llm_16_725_rrrruuuugggg_test_from_usize = 0;
    }
    #[test]
    fn test_from_usize_max() {
        let _rug_st_tests_llm_16_725_rrrruuuugggg_test_from_usize_max = 0;
        let num = usize::MAX;
        let json_val = Value::from(num);
        debug_assert_eq!(json_val, Value::Number(Number::from(num as u64)));
        let _rug_ed_tests_llm_16_725_rrrruuuugggg_test_from_usize_max = 0;
    }
    #[test]
    fn test_from_usize_zero() {
        let _rug_st_tests_llm_16_725_rrrruuuugggg_test_from_usize_zero = 0;
        let rug_fuzz_0 = 0usize;
        let num = rug_fuzz_0;
        let json_val = Value::from(num);
        debug_assert_eq!(json_val, Value::Number(Number::from(num as u64)));
        let _rug_ed_tests_llm_16_725_rrrruuuugggg_test_from_usize_zero = 0;
    }
    #[test]
    fn test_from_usize_overflow() {
        let _rug_st_tests_llm_16_725_rrrruuuugggg_test_from_usize_overflow = 0;
        let rug_fuzz_0 = 1;
        let num = usize::try_from(i64::MAX).unwrap() + rug_fuzz_0;
        let json_val = Value::from(num);
        debug_assert!(matches!(json_val, Value::Number(_)));
        let _rug_ed_tests_llm_16_725_rrrruuuugggg_test_from_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_726 {
    use crate::{Map, Value, Number};
    use std::iter::FromIterator;
    #[test]
    fn test_from_iter_with_empty_vec() {
        let _rug_st_tests_llm_16_726_rrrruuuugggg_test_from_iter_with_empty_vec = 0;
        let v: Vec<(String, Value)> = Vec::new();
        let val = Value::from_iter(v);
        let expected = Value::Object(Map::new());
        debug_assert_eq!(val, expected);
        let _rug_ed_tests_llm_16_726_rrrruuuugggg_test_from_iter_with_empty_vec = 0;
    }
    #[test]
    fn test_from_iter_with_non_empty_vec() {
        let _rug_st_tests_llm_16_726_rrrruuuugggg_test_from_iter_with_non_empty_vec = 0;
        let rug_fuzz_0 = "one";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "one";
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = "two";
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = "three";
        let rug_fuzz_7 = 3;
        let v = vec![
            (String::from(rug_fuzz_0), Value::from(rug_fuzz_1)), (String::from("two"),
            Value::from(2)), (String::from("three"), Value::from(3))
        ];
        let val = Value::from_iter(v);
        let mut map = Map::new();
        map.insert(String::from(rug_fuzz_2), Value::from(rug_fuzz_3));
        map.insert(String::from(rug_fuzz_4), Value::from(rug_fuzz_5));
        map.insert(String::from(rug_fuzz_6), Value::from(rug_fuzz_7));
        let expected = Value::Object(map);
        debug_assert_eq!(val, expected);
        let _rug_ed_tests_llm_16_726_rrrruuuugggg_test_from_iter_with_non_empty_vec = 0;
    }
    #[test]
    fn test_from_iter_with_complex_types() {
        let _rug_st_tests_llm_16_726_rrrruuuugggg_test_from_iter_with_complex_types = 0;
        let rug_fuzz_0 = "array";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "array";
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = "nested";
        let rug_fuzz_5 = "value";
        let rug_fuzz_6 = "another";
        let rug_fuzz_7 = "one";
        let rug_fuzz_8 = "object";
        let v = vec![
            (String::from(rug_fuzz_0), Value::Array(vec![Value::from(rug_fuzz_1),
            Value::from(2), Value::from(3)])), (String::from("object"),
            Value::Object(vec![(String::from("nested"),
            Value::String(String::from("value"))), (String::from("another"),
            Value::String(String::from("one")))] .into_iter().collect()))
        ];
        let val = Value::from_iter(v);
        let mut map = Map::new();
        map.insert(
            String::from(rug_fuzz_2),
            Value::Array(vec![Value::from(rug_fuzz_3), Value::from(2), Value::from(3)]),
        );
        let mut sub_map = Map::new();
        sub_map
            .insert(String::from(rug_fuzz_4), Value::String(String::from(rug_fuzz_5)));
        sub_map
            .insert(String::from(rug_fuzz_6), Value::String(String::from(rug_fuzz_7)));
        map.insert(String::from(rug_fuzz_8), Value::Object(sub_map));
        let expected = Value::Object(map);
        debug_assert_eq!(val, expected);
        let _rug_ed_tests_llm_16_726_rrrruuuugggg_test_from_iter_with_complex_types = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_727_llm_16_727 {
    use crate::Value;
    use crate::value::Value::Array;
    use crate::Map;
    use std::iter::FromIterator;
    #[test]
    fn test_from_iter_with_empty_vec() {
        let _rug_st_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_empty_vec = 0;
        let v: Vec<Value> = Vec::new();
        let result: Value = Array(v.into_iter().collect());
        debug_assert!(result.is_array());
        debug_assert!(result.as_array().unwrap().is_empty());
        let _rug_ed_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_empty_vec = 0;
    }
    #[test]
    fn test_from_iter_with_non_empty_vec() {
        let _rug_st_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_non_empty_vec = 0;
        let rug_fuzz_0 = 1;
        let v = vec![rug_fuzz_0, 2, 3];
        let result: Value = Array(v.into_iter().map(Value::from).collect());
        debug_assert_eq!(result, Value::from_iter(vec![1, 2, 3]));
        let _rug_ed_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_non_empty_vec = 0;
    }
    #[test]
    fn test_from_iter_with_mixed_types() {
        let _rug_st_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_mixed_types = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let v = vec![Value::from(rug_fuzz_0), Value::from(1.5), Value::from("string")];
        let result: Value = Array(v.into_iter().collect());
        let expected = Value::from_iter(
            vec![Value::from(rug_fuzz_1), Value::from(1.5), Value::from("string")],
        );
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_mixed_types = 0;
    }
    #[test]
    fn test_from_iter_with_nested_array() {
        let _rug_st_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_nested_array = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let v = vec![vec![rug_fuzz_0, 2], vec![3, 4]];
        let result: Value = Array(
            v
                .into_iter()
                .map(|inner| Array(inner.into_iter().map(Value::from).collect()))
                .collect(),
        );
        let expected = Value::from_iter(
            vec![Value::from_iter(vec![rug_fuzz_1, 2]), Value::from_iter(vec![3, 4])],
        );
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_nested_array = 0;
    }
    #[test]
    fn test_from_iter_with_nested_object() {
        let _rug_st_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_nested_object = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "key2";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "key1";
        let rug_fuzz_5 = 1;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_owned(), Value::from(rug_fuzz_3));
        let v = vec![map];
        let result: Value = Array(v.into_iter().map(Value::from).collect());
        let expected = Value::from_iter(
            vec![
                Value::from_iter(vec![(rug_fuzz_4.to_owned(), Value::from(rug_fuzz_5)),
                ("key2".to_owned(), Value::from(2))])
            ],
        );
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_727_llm_16_727_rrrruuuugggg_test_from_iter_with_nested_object = 0;
    }
}
