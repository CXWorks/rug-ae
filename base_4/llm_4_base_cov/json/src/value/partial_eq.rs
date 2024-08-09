use super::Value;
use alloc::string::String;
fn eq_i64(value: &Value, other: i64) -> bool {
    value.as_i64().map_or(false, |i| i == other)
}
fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64().map_or(false, |i| i == other)
}
fn eq_f32(value: &Value, other: f32) -> bool {
    match value {
        Value::Number(n) => n.as_f32().map_or(false, |i| i == other),
        _ => false,
    }
}
fn eq_f64(value: &Value, other: f64) -> bool {
    value.as_f64().map_or(false, |i| i == other)
}
fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool().map_or(false, |i| i == other)
}
fn eq_str(value: &Value, other: &str) -> bool {
    value.as_str().map_or(false, |i| i == other)
}
impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}
impl<'a> PartialEq<&'a str> for Value {
    fn eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}
impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self)
    }
}
impl<'a> PartialEq<Value> for &'a str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, *self)
    }
}
impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}
impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
    }
}
macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(impl PartialEq <$ty > for Value { fn eq(& self, other : &$ty) -> bool { $eq
        (self, * other as _) } } impl PartialEq < Value > for $ty { fn eq(& self, other :
        & Value) -> bool { $eq (other, * self as _) } } impl <'a > PartialEq <$ty > for
        &'a Value { fn eq(& self, other : &$ty) -> bool { $eq (* self, * other as _) } }
        impl <'a > PartialEq <$ty > for &'a mut Value { fn eq(& self, other : &$ty) ->
        bool { $eq (* self, * other as _) } })*)*
    };
}
partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize] eq_u64[u8 u16 u32 u64 usize] eq_f32[f32] eq_f64[f64]
    eq_bool[bool]
}
#[cfg(test)]
mod tests_llm_16_734 {
    use crate::value::Value;
    #[test]
    fn test_eq_bool_true() {
        let _rug_st_tests_llm_16_734_rrrruuuugggg_test_eq_bool_true = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = true;
        let val_true = Value::Bool(rug_fuzz_0);
        let val_false = Value::Bool(rug_fuzz_1);
        debug_assert!(Value::eq(& val_true, & rug_fuzz_2));
        debug_assert!(! Value::eq(& val_true, & rug_fuzz_3));
        debug_assert!(! Value::eq(& val_false, & rug_fuzz_4));
        let _rug_ed_tests_llm_16_734_rrrruuuugggg_test_eq_bool_true = 0;
    }
    #[test]
    fn test_eq_bool_false() {
        let _rug_st_tests_llm_16_734_rrrruuuugggg_test_eq_bool_false = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = false;
        let val_true = Value::Bool(rug_fuzz_0);
        let val_false = Value::Bool(rug_fuzz_1);
        debug_assert!(! Value::eq(& val_false, & rug_fuzz_2));
        debug_assert!(Value::eq(& val_false, & rug_fuzz_3));
        debug_assert!(! Value::eq(& val_true, & rug_fuzz_4));
        let _rug_ed_tests_llm_16_734_rrrruuuugggg_test_eq_bool_false = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_735 {
    use crate::Number;
    use crate::Value;
    #[test]
    fn value_partial_eq_with_bool() {
        let _rug_st_tests_llm_16_735_rrrruuuugggg_value_partial_eq_with_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = false;
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = false;
        let rug_fuzz_5 = false;
        let rug_fuzz_6 = true;
        let rug_fuzz_7 = false;
        let rug_fuzz_8 = true;
        let rug_fuzz_9 = false;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = true;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = false;
        let rug_fuzz_14 = 1.0;
        let rug_fuzz_15 = true;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = false;
        debug_assert!(Value::Bool(rug_fuzz_0) == Value::Bool(rug_fuzz_1));
        debug_assert!(Value::Bool(rug_fuzz_2) != Value::Bool(rug_fuzz_3));
        debug_assert!(Value::Bool(rug_fuzz_4) == Value::Bool(rug_fuzz_5));
        debug_assert!(Value::Bool(rug_fuzz_6) != Value::Bool(rug_fuzz_7));
        debug_assert!(Value::Null != Value::Bool(rug_fuzz_8));
        debug_assert!(Value::Null != Value::Bool(rug_fuzz_9));
        debug_assert!(
            Value::Number(Number::from(rug_fuzz_10)) != Value::Bool(rug_fuzz_11)
        );
        debug_assert!(
            Value::Number(Number::from(rug_fuzz_12)) != Value::Bool(rug_fuzz_13)
        );
        debug_assert!(
            Value::Number(Number::from_f64(rug_fuzz_14).unwrap()) !=
            Value::Bool(rug_fuzz_15)
        );
        debug_assert!(
            Value::Number(Number::from(rug_fuzz_16)) == Value::Bool(rug_fuzz_17)
        );
        let _rug_ed_tests_llm_16_735_rrrruuuugggg_value_partial_eq_with_bool = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_736 {
    use crate::value::{Value, Number};
    #[test]
    fn test_value_eq_f32() {
        let _rug_st_tests_llm_16_736_rrrruuuugggg_test_value_eq_f32 = 0;
        let rug_fuzz_0 = 12.5;
        let rug_fuzz_1 = 12.5f32;
        let rug_fuzz_2 = 12.0f32;
        let rug_fuzz_3 = 12.5f32;
        let rug_fuzz_4 = 10.0;
        let rug_fuzz_5 = 10.0f32;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0f32;
        let pos_inf_value = Value::Number(Number::from_f32(f32::INFINITY).unwrap());
        debug_assert!(! pos_inf_value.eq(& f32::INFINITY));
        let neg_inf_value = Value::Number(Number::from_f32(f32::NEG_INFINITY).unwrap());
        debug_assert!(! neg_inf_value.eq(& f32::NEG_INFINITY));
        let nan_value = Value::Number(Number::from_f32(f32::NAN).unwrap());
        debug_assert!(! nan_value.eq(& f32::NAN));
        let number_value = Value::Number(Number::from_f32(rug_fuzz_0).unwrap());
        debug_assert!(number_value.eq(& rug_fuzz_1));
        debug_assert!(! number_value.eq(& rug_fuzz_2));
        let null_value = Value::Null;
        debug_assert!(! null_value.eq(& rug_fuzz_3));
        let int_value = Value::Number(Number::from_f32(rug_fuzz_4).unwrap());
        debug_assert!(int_value.eq(& rug_fuzz_5));
        let zero_value = Value::Number(Number::from_f32(rug_fuzz_6).unwrap());
        debug_assert!(zero_value.eq(& rug_fuzz_7));
        let _rug_ed_tests_llm_16_736_rrrruuuugggg_test_value_eq_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_737 {
    use crate::value::Value;
    #[test]
    fn eq_with_f32() {
        let _rug_st_tests_llm_16_737_rrrruuuugggg_eq_with_f32 = 0;
        let rug_fuzz_0 = 123.456_f32;
        let rug_fuzz_1 = 0.0_f32;
        let value_number = |n: f64| {
            Value::Number(crate::Number::from_f64(n).unwrap())
        };
        let value_null = Value::Null;
        let f: f32 = rug_fuzz_0;
        let f_value = value_number(f64::from(f));
        debug_assert!(
            ! value_null.eq(& Value::Number(crate ::Number::from_f32(f).unwrap()))
        );
        debug_assert!(f_value.eq(& Value::Number(crate ::Number::from_f32(f).unwrap())));
        debug_assert!(
            Value::from(f).eq(& Value::Number(crate ::Number::from_f32(f).unwrap()))
        );
        debug_assert!(
            ! Value::from(rug_fuzz_1).eq(& Value::Number(crate ::Number::from_f32(f)
            .unwrap()))
        );
        debug_assert!(
            ! Value::from(f64::INFINITY).eq(& Value::Number(crate ::Number::from_f32(f)
            .unwrap()))
        );
        debug_assert!(
            ! Value::from(f64::NAN).eq(& Value::Number(crate ::Number::from_f32(f)
            .unwrap()))
        );
        let _rug_ed_tests_llm_16_737_rrrruuuugggg_eq_with_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_738 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn test_value_eq_f32() {
        let _rug_st_tests_llm_16_738_rrrruuuugggg_test_value_eq_f32 = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1.0_f32;
        let rug_fuzz_2 = "b";
        let rug_fuzz_3 = 1.0_f32;
        let rug_fuzz_4 = "c";
        let rug_fuzz_5 = 1.0_f32;
        let rug_fuzz_6 = "d";
        let rug_fuzz_7 = 1.0_f32;
        let rug_fuzz_8 = "e";
        let rug_fuzz_9 = 1.0_f32;
        let rug_fuzz_10 = "f";
        let rug_fuzz_11 = 1.0_f32;
        let rug_fuzz_12 = "g";
        let rug_fuzz_13 = 1.0_f32;
        let rug_fuzz_14 = "h";
        let rug_fuzz_15 = 1.0_f32;
        let rug_fuzz_16 = "i";
        let rug_fuzz_17 = 1.0_f32;
        let rug_fuzz_18 = "j";
        let rug_fuzz_19 = 1.0_f32;
        let rug_fuzz_20 = "k";
        let rug_fuzz_21 = 1.0_f32;
        let data = json!(
            { "a" : 1.0, "b" : 1.0_f32, "c" : 1.0001_f32, "d" : - 1.0, "e" : "1.0", "f" :
            1, "g" : 1.5, "h" : null, "i" : true, "j" : [1.0], "k" : { "key" : 1.0 }, }
        );
        debug_assert!(data[rug_fuzz_0] == rug_fuzz_1);
        debug_assert!(data[rug_fuzz_2] == rug_fuzz_3);
        debug_assert!(! (data[rug_fuzz_4] == rug_fuzz_5));
        debug_assert!(! (data[rug_fuzz_6] == rug_fuzz_7));
        debug_assert!(! (data[rug_fuzz_8] == rug_fuzz_9));
        debug_assert!(! (data[rug_fuzz_10] == rug_fuzz_11));
        debug_assert!(! (data[rug_fuzz_12] == rug_fuzz_13));
        debug_assert!(! (data[rug_fuzz_14] == rug_fuzz_15));
        debug_assert!(! (data[rug_fuzz_16] == rug_fuzz_17));
        debug_assert!(! (data[rug_fuzz_18] == rug_fuzz_19));
        debug_assert!(! (data[rug_fuzz_20] == rug_fuzz_21));
        let _rug_ed_tests_llm_16_738_rrrruuuugggg_test_value_eq_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_742 {
    use crate::{Number, Value};
    #[test]
    fn eq_with_different_types() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_different_types = 0;
        let rug_fuzz_0 = 5_i16;
        let mut value_number = Value::Number(Number::from(rug_fuzz_0));
        let value_null = Value::Null;
        debug_assert_ne!(value_number, value_null);
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_different_types = 0;
    }
    #[test]
    fn eq_with_number_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_number_type = 0;
        let rug_fuzz_0 = 10_i16;
        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert_eq!(value, Value::Number(Number::from(10_i16)));
        debug_assert_ne!(value, Value::Number(Number::from(11_i16)));
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_number_type = 0;
    }
    #[test]
    fn eq_with_bool_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_bool_type = 0;
        let rug_fuzz_0 = true;
        let value = Value::Bool(rug_fuzz_0);
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_bool_type = 0;
    }
    #[test]
    fn eq_with_null_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_null_type = 0;
        let value = Value::Null;
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_null_type = 0;
    }
    #[test]
    fn eq_with_string_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_string_type = 0;
        let rug_fuzz_0 = "10";
        let value = Value::String(rug_fuzz_0.to_owned());
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_string_type = 0;
    }
    #[test]
    fn eq_with_array_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_array_type = 0;
        let rug_fuzz_0 = 10_i16;
        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_array_type = 0;
    }
    #[test]
    fn eq_with_object_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_object_type = 0;
        let rug_fuzz_0 = "number";
        let rug_fuzz_1 = 10_i16;
        let mut object = crate::Map::new();
        object.insert(rug_fuzz_0.to_owned(), Value::Number(Number::from(rug_fuzz_1)));
        let value = Value::Object(object);
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_object_type = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_743 {
    use crate::value::Value;
    use std::i16;
    #[test]
    fn test_eq_with_i16() {
        let _rug_st_tests_llm_16_743_rrrruuuugggg_test_eq_with_i16 = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 42i16;
        let rug_fuzz_2 = 42i16;
        let rug_fuzz_3 = 0i16;
        let rug_fuzz_4 = 0i16;
        let rug_fuzz_5 = 42i16;
        let rug_fuzz_6 = 42i16;
        let rug_fuzz_7 = 42i16;
        let rug_fuzz_8 = 42i16;
        let rug_fuzz_9 = 0i16;
        let rug_fuzz_10 = 1i16;
        let rug_fuzz_11 = 1i16;
        let rug_fuzz_12 = 1i16;
        let rug_fuzz_13 = 1i16;
        let rug_fuzz_14 = 42i16;
        let rug_fuzz_15 = 42i16;
        let max_i16_value = Value::from(i16::MAX);
        let min_i16_value = Value::from(i16::MIN);
        let zero_value = Value::from(rug_fuzz_0);
        let pos_value = Value::from(rug_fuzz_1);
        let neg_value = Value::from(-rug_fuzz_2);
        let null_value = Value::Null;
        debug_assert!(Value::Number(crate ::Number::from(i16::MAX)).eq(& i16::MAX));
        debug_assert!(Value::Number(crate ::Number::from(i16::MIN)).eq(& i16::MIN));
        debug_assert!(Value::Number(crate ::Number::from(rug_fuzz_3)).eq(& rug_fuzz_4));
        debug_assert!(Value::Number(crate ::Number::from(rug_fuzz_5)).eq(& rug_fuzz_6));
        debug_assert!(
            Value::Number(crate ::Number::from(- rug_fuzz_7)).eq(& - rug_fuzz_8)
        );
        debug_assert!(! null_value.eq(& rug_fuzz_9));
        debug_assert!(! null_value.eq(& - rug_fuzz_10));
        debug_assert!(! null_value.eq(& rug_fuzz_11));
        debug_assert!(! max_i16_value.eq(& - rug_fuzz_12));
        debug_assert!(! min_i16_value.eq(& rug_fuzz_13));
        debug_assert!(! pos_value.eq(& - rug_fuzz_14));
        debug_assert!(! neg_value.eq(& rug_fuzz_15));
        let _rug_ed_tests_llm_16_743_rrrruuuugggg_test_eq_with_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_744_llm_16_744 {
    use crate::value::Value;
    use crate::Number;
    use std::i16;
    #[test]
    fn test_eq_i16_with_number() {
        let _rug_st_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_eq_i16_with_number = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 123i16;
        let rug_fuzz_3 = 123;
        let rug_fuzz_4 = 123i16;
        let rug_fuzz_5 = 123;
        let min_value = Value::Number(Number::from(i16::MIN));
        debug_assert!(Value::eq(& min_value, & i16::MIN));
        let max_value = Value::Number(Number::from(i16::MAX));
        debug_assert!(Value::eq(& max_value, & i16::MAX));
        let zero = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(Value::eq(& zero, & rug_fuzz_1));
        let positive = Value::Number(Number::from(rug_fuzz_2));
        debug_assert!(Value::eq(& positive, & rug_fuzz_3));
        let negative = Value::Number(Number::from(-rug_fuzz_4));
        debug_assert!(Value::eq(& negative, & - rug_fuzz_5));
        let _rug_ed_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_eq_i16_with_number = 0;
    }
    #[test]
    fn test_eq_i16_with_non_number() {
        let _rug_st_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_eq_i16_with_non_number = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = "123";
        let rug_fuzz_2 = 123i16;
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = 0i16;
        let null = Value::Null;
        debug_assert!(! Value::eq(& null, & rug_fuzz_0));
        let string = Value::String(rug_fuzz_1.to_string());
        debug_assert!(! Value::eq(& string, & rug_fuzz_2));
        let boolean = Value::Bool(rug_fuzz_3);
        debug_assert!(! Value::eq(& boolean, & rug_fuzz_4));
        let _rug_ed_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_eq_i16_with_non_number = 0;
    }
    #[test]
    fn test_eq_i16_with_different_number() {
        let _rug_st_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_eq_i16_with_different_number = 0;
        let rug_fuzz_0 = 42i16;
        let rug_fuzz_1 = 123i16;
        let rug_fuzz_2 = 32768i32;
        let different = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(! Value::eq(& different, & rug_fuzz_1));
        let out_of_range = Value::Number(Number::from(rug_fuzz_2));
        debug_assert!(! Value::eq(& out_of_range, & i16::MAX));
        let _rug_ed_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_eq_i16_with_different_number = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_745 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_eq_with_i32() {
        let _rug_st_tests_llm_16_745_rrrruuuugggg_test_eq_with_i32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = 100;
        let value_num = Value::Number(Number::from(rug_fuzz_0));
        let other: i32 = rug_fuzz_1;
        debug_assert_eq!(Value::eq(& value_num, & other), true);
        let value_num_negative = Value::Number(Number::from(-rug_fuzz_2));
        let other_negative: i32 = -rug_fuzz_3;
        debug_assert_eq!(Value::eq(& value_num_negative, & other_negative), true);
        let other_different: i32 = rug_fuzz_4;
        debug_assert_eq!(Value::eq(& value_num, & other_different), false);
        let _rug_ed_tests_llm_16_745_rrrruuuugggg_test_eq_with_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_746 {
    use crate::value::Value;
    #[test]
    fn value_eq_integer() {
        let _rug_st_tests_llm_16_746_rrrruuuugggg_value_eq_integer = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 43;
        let rug_fuzz_4 = 42;
        let rug_fuzz_5 = 42;
        let rug_fuzz_6 = "42";
        let rug_fuzz_7 = 42;
        let rug_fuzz_8 = 42;
        let rug_fuzz_9 = 42;
        let rug_fuzz_10 = 42;
        let rug_fuzz_11 = true;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 0;
        let value_num = Value::Number(rug_fuzz_0.into());
        let other_num: i32 = rug_fuzz_1;
        debug_assert!(value_num.eq(& other_num));
        let value_num = Value::Number(rug_fuzz_2.into());
        let other_num: i32 = rug_fuzz_3;
        debug_assert!(! value_num.eq(& other_num));
        let value_negative_num = Value::Number((-rug_fuzz_4).into());
        let other_num: i32 = -rug_fuzz_5;
        debug_assert!(value_negative_num.eq(& other_num));
        let value_str = Value::String(String::from(rug_fuzz_6));
        let other_num: i32 = rug_fuzz_7;
        debug_assert!(! value_str.eq(& other_num));
        let value_array = Value::Array(vec![Value::Number(rug_fuzz_8.into())]);
        let other_num: i32 = rug_fuzz_9;
        debug_assert!(! value_array.eq(& other_num));
        let value_obj = Value::Object(crate::Map::new());
        let other_num: i32 = rug_fuzz_10;
        debug_assert!(! value_obj.eq(& other_num));
        let value_bool = Value::Bool(rug_fuzz_11);
        let other_num: i32 = rug_fuzz_12;
        debug_assert!(! value_bool.eq(& other_num));
        let value_null = Value::Null;
        let other_num: i32 = rug_fuzz_13;
        debug_assert!(! value_null.eq(& other_num));
        let _rug_ed_tests_llm_16_746_rrrruuuugggg_value_eq_integer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_747_llm_16_747 {
    use crate::{Number, Value};
    #[test]
    fn test_eq_with_integers() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_integers = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let v = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(v.eq(& rug_fuzz_1));
        debug_assert!(! v.eq(& rug_fuzz_2));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_integers = 0;
    }
    #[test]
    fn test_eq_with_floating_point() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_floating_point = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let v = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(v.eq(& rug_fuzz_1));
        debug_assert!(! v.eq(& rug_fuzz_2));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_floating_point = 0;
    }
    #[test]
    fn test_eq_with_non_number() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_non_number = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = 42;
        let v = Value::String(rug_fuzz_0.into());
        debug_assert!(! v.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_non_number = 0;
    }
    #[test]
    fn test_eq_with_array() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_array = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let v = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! v.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_array = 0;
    }
    #[test]
    fn test_eq_with_object() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_object = 0;
        let rug_fuzz_0 = "number";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Number(Number::from(rug_fuzz_1)));
        let v = Value::Object(map);
        debug_assert!(! v.eq(& rug_fuzz_2));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_eq_with_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_748 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_value_eq_i64() {
        let _rug_st_tests_llm_16_748_rrrruuuugggg_test_value_eq_i64 = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42_i64;
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = 0_i64;
        let num = Number::from(rug_fuzz_0);
        let val_num = Value::Number(num);
        let int_value: i64 = rug_fuzz_1;
        debug_assert_eq!(& val_num, & int_value);
        let num_neg = Number::from(-rug_fuzz_2);
        let val_num_neg = Value::Number(num_neg);
        let int_neg_value: i64 = -rug_fuzz_3;
        debug_assert_eq!(& val_num_neg, & int_neg_value);
        let num_big = Number::from(i64::max_value());
        let val_num_big = Value::Number(num_big);
        let int_big_value: i64 = i64::max_value();
        debug_assert_eq!(& val_num_big, & int_big_value);
        let num_small = Number::from(i64::min_value());
        let val_num_small = Value::Number(num_small);
        let int_small_value: i64 = i64::min_value();
        debug_assert_eq!(& val_num_small, & int_small_value);
        let non_matching_num = Number::from(rug_fuzz_4);
        let val_non_matching_num = Value::Number(non_matching_num);
        debug_assert_ne!(& val_non_matching_num, & int_value);
        let _rug_ed_tests_llm_16_748_rrrruuuugggg_test_value_eq_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_749 {
    use crate::value::Value;
    #[test]
    fn test_eq() {
        let _rug_st_tests_llm_16_749_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 100;
        let v1 = Value::Number(rug_fuzz_0.into());
        let v2 = Value::Number(rug_fuzz_1.into());
        debug_assert_eq!(& v1, & 42);
        let v3 = Value::Number(rug_fuzz_2.into());
        debug_assert_ne!(& v2, & 100);
        let _rug_ed_tests_llm_16_749_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_750 {
    use crate::Value;
    #[test]
    fn test_eq_number_and_i64() {
        let _rug_st_tests_llm_16_750_rrrruuuugggg_test_eq_number_and_i64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42i64;
        let rug_fuzz_2 = 100i64;
        let num = Value::Number(rug_fuzz_0.into());
        debug_assert!(num == rug_fuzz_1);
        debug_assert!(num != rug_fuzz_2);
        let _rug_ed_tests_llm_16_750_rrrruuuugggg_test_eq_number_and_i64 = 0;
    }
    #[test]
    fn test_eq_string_and_i64() {
        let _rug_st_tests_llm_16_750_rrrruuuugggg_test_eq_string_and_i64 = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = 42i64;
        let string = Value::String(rug_fuzz_0.to_string());
        debug_assert!(string != rug_fuzz_1);
        let _rug_ed_tests_llm_16_750_rrrruuuugggg_test_eq_string_and_i64 = 0;
    }
    #[test]
    fn test_eq_array_and_i64() {
        let _rug_st_tests_llm_16_750_rrrruuuugggg_test_eq_array_and_i64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42i64;
        let array = Value::Array(vec![Value::Number(rug_fuzz_0.into())]);
        debug_assert!(array != rug_fuzz_1);
        let _rug_ed_tests_llm_16_750_rrrruuuugggg_test_eq_array_and_i64 = 0;
    }
    #[test]
    fn test_eq_object_and_i64() {
        let _rug_st_tests_llm_16_750_rrrruuuugggg_test_eq_object_and_i64 = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42i64;
        let mut obj = crate::Map::new();
        obj.insert(rug_fuzz_0.to_string(), Value::Number(rug_fuzz_1.into()));
        let object = Value::Object(obj);
        debug_assert!(object != rug_fuzz_2);
        let _rug_ed_tests_llm_16_750_rrrruuuugggg_test_eq_object_and_i64 = 0;
    }
    #[test]
    fn test_eq_bool_and_i64() {
        let _rug_st_tests_llm_16_750_rrrruuuugggg_test_eq_bool_and_i64 = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 42i64;
        let boolean = Value::Bool(rug_fuzz_0);
        debug_assert!(boolean != rug_fuzz_1);
        let _rug_ed_tests_llm_16_750_rrrruuuugggg_test_eq_bool_and_i64 = 0;
    }
    #[test]
    fn test_eq_null_and_i64() {
        let _rug_st_tests_llm_16_750_rrrruuuugggg_test_eq_null_and_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let null = Value::Null;
        debug_assert!(null != rug_fuzz_0);
        let _rug_ed_tests_llm_16_750_rrrruuuugggg_test_eq_null_and_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_752 {
    use crate::{json, Value};
    #[test]
    fn eq_i8_with_value() {
        let _rug_st_tests_llm_16_752_rrrruuuugggg_eq_i8_with_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 100;
        let value_num = Value::Number(rug_fuzz_0.into());
        let num_i8: i8 = rug_fuzz_1;
        let non_matching_value_num = Value::Number(rug_fuzz_2.into());
        let non_matching_num_i8: i8 = rug_fuzz_3;
        debug_assert!(value_num == num_i8);
        debug_assert!(non_matching_value_num != num_i8);
        debug_assert!(value_num != non_matching_num_i8);
        let _rug_ed_tests_llm_16_752_rrrruuuugggg_eq_i8_with_value = 0;
    }
    #[test]
    fn eq_str_with_value() {
        let _rug_st_tests_llm_16_752_rrrruuuugggg_eq_str_with_value = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = "fail";
        let rug_fuzz_3 = "fail";
        let value_str = Value::String(String::from(rug_fuzz_0));
        let str_slice: &str = rug_fuzz_1;
        let non_matching_value_str = Value::String(String::from(rug_fuzz_2));
        let non_matching_str_slice: &str = rug_fuzz_3;
        debug_assert!(value_str == str_slice);
        debug_assert!(non_matching_value_str != str_slice);
        debug_assert!(value_str != non_matching_str_slice);
        let _rug_ed_tests_llm_16_752_rrrruuuugggg_eq_str_with_value = 0;
    }
    #[test]
    fn eq_value_with_i8() {
        let _rug_st_tests_llm_16_752_rrrruuuugggg_eq_value_with_i8 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 100;
        let num_i8: i8 = rug_fuzz_0;
        let value_num = Value::Number(rug_fuzz_1.into());
        let non_matching_num_i8: i8 = rug_fuzz_2;
        let non_matching_value_num = Value::Number(rug_fuzz_3.into());
        debug_assert!(num_i8 == value_num);
        debug_assert!(non_matching_num_i8 != value_num);
        debug_assert!(num_i8 != non_matching_value_num);
        let _rug_ed_tests_llm_16_752_rrrruuuugggg_eq_value_with_i8 = 0;
    }
    #[test]
    fn eq_value_with_str() {
        let _rug_st_tests_llm_16_752_rrrruuuugggg_eq_value_with_str = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = "fail";
        let rug_fuzz_3 = "fail";
        let str_slice: &str = rug_fuzz_0;
        let value_str = Value::String(String::from(rug_fuzz_1));
        let non_matching_str_slice: &str = rug_fuzz_2;
        let non_matching_value_str = Value::String(String::from(rug_fuzz_3));
        debug_assert!(str_slice == value_str);
        debug_assert!(non_matching_str_slice != value_str);
        debug_assert!(str_slice != non_matching_value_str);
        let _rug_ed_tests_llm_16_752_rrrruuuugggg_eq_value_with_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_753 {
    use crate::Value;
    #[test]
    fn test_eq_i8_with_number() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_number = 0;
        let rug_fuzz_0 = 10;
        let i8_val: i8 = rug_fuzz_0;
        let num = Value::Number(i8_val.into());
        debug_assert!(num.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_number = 0;
    }
    #[test]
    fn test_eq_i8_with_negative_number() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_negative_number = 0;
        let rug_fuzz_0 = 10;
        let i8_val: i8 = -rug_fuzz_0;
        let num = Value::Number(i8_val.into());
        debug_assert!(num.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_negative_number = 0;
    }
    #[test]
    fn test_eq_i8_with_incorrect_type() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_incorrect_type = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = "10";
        let i8_val: i8 = rug_fuzz_0;
        let str_val = Value::String(rug_fuzz_1.to_string());
        debug_assert!(! str_val.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_incorrect_type = 0;
    }
    #[test]
    fn test_eq_i8_with_number_out_of_range() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_number_out_of_range = 0;
        let rug_fuzz_0 = 10;
        let i8_val: i8 = rug_fuzz_0;
        let i64_val = Value::Number((i8_val as i64 + i8::max_value() as i64).into());
        debug_assert!(! i64_val.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_number_out_of_range = 0;
    }
    #[test]
    fn test_eq_i8_with_null() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_null = 0;
        let rug_fuzz_0 = 10;
        let i8_val: i8 = rug_fuzz_0;
        let null = Value::Null;
        debug_assert!(! null.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_null = 0;
    }
    #[test]
    fn test_eq_i8_with_bool() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_bool = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = true;
        let i8_val: i8 = rug_fuzz_0;
        let tru = Value::Bool(rug_fuzz_1);
        debug_assert!(! tru.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_bool = 0;
    }
    #[test]
    fn test_eq_i8_with_array() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_array = 0;
        let rug_fuzz_0 = 10;
        let i8_val: i8 = rug_fuzz_0;
        let arr = Value::Array(vec![Value::Number(i8_val.into())]);
        debug_assert!(! arr.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_array = 0;
    }
    #[test]
    fn test_eq_i8_with_object() {
        let _rug_st_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_object = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = "key";
        let i8_val: i8 = rug_fuzz_0;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_1.to_string(), Value::Number(i8_val.into()));
        let obj = Value::Object(map);
        debug_assert!(! obj.eq(& i8_val));
        let _rug_ed_tests_llm_16_753_rrrruuuugggg_test_eq_i8_with_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_755 {
    use crate::value::Value;
    use std::str::FromStr;
    #[test]
    fn test_eq_number_and_isize() {
        let _rug_st_tests_llm_16_755_rrrruuuugggg_test_eq_number_and_isize = 0;
        let rug_fuzz_0 = "42";
        let num = Value::Number(crate::Number::from_str(rug_fuzz_0).unwrap());
        debug_assert_eq!(num, 42isize);
        debug_assert_ne!(num, - 42isize);
        let _rug_ed_tests_llm_16_755_rrrruuuugggg_test_eq_number_and_isize = 0;
    }
    #[test]
    fn test_eq_string_and_isize() {
        let _rug_st_tests_llm_16_755_rrrruuuugggg_test_eq_string_and_isize = 0;
        let rug_fuzz_0 = "42";
        let s = Value::String(rug_fuzz_0.to_string());
        debug_assert_ne!(s, 42isize);
        let _rug_ed_tests_llm_16_755_rrrruuuugggg_test_eq_string_and_isize = 0;
    }
    #[test]
    fn test_eq_bool_and_isize() {
        let _rug_st_tests_llm_16_755_rrrruuuugggg_test_eq_bool_and_isize = 0;
        let rug_fuzz_0 = false;
        let b = Value::Bool(rug_fuzz_0);
        debug_assert_ne!(b, 42isize);
        let _rug_ed_tests_llm_16_755_rrrruuuugggg_test_eq_bool_and_isize = 0;
    }
    #[test]
    fn test_eq_array_and_isize() {
        let _rug_st_tests_llm_16_755_rrrruuuugggg_test_eq_array_and_isize = 0;
        let rug_fuzz_0 = 42isize;
        let arr = Value::Array(vec![Value::Number(crate ::Number::from(rug_fuzz_0))]);
        debug_assert_ne!(arr, 42isize);
        let _rug_ed_tests_llm_16_755_rrrruuuugggg_test_eq_array_and_isize = 0;
    }
    #[test]
    fn test_eq_object_and_isize() {
        let _rug_st_tests_llm_16_755_rrrruuuugggg_test_eq_object_and_isize = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42isize;
        let mut m = crate::Map::new();
        m.insert(rug_fuzz_0.to_string(), Value::Number(crate::Number::from(rug_fuzz_1)));
        let obj = Value::Object(m);
        debug_assert_ne!(obj, 42isize);
        let _rug_ed_tests_llm_16_755_rrrruuuugggg_test_eq_object_and_isize = 0;
    }
    #[test]
    fn test_eq_null_and_isize() {
        let _rug_st_tests_llm_16_755_rrrruuuugggg_test_eq_null_and_isize = 0;
        let null = Value::Null;
        debug_assert_ne!(null, 42isize);
        let _rug_ed_tests_llm_16_755_rrrruuuugggg_test_eq_null_and_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_757_llm_16_757 {
    use super::*;
    use crate::*;
    use crate::*;
    use crate::value::Value;
    use std::string::String;
    fn eq(value: &Value, other: &String) -> bool {
        eq_str(value, other.as_str())
    }
    #[test]
    fn test_eq_null() {
        let value = Value::Null;
        let other = String::from("null");
        assert!(! eq(& value, & other));
    }
    #[test]
    fn test_eq_bool_true() {
        let value = Value::Bool(true);
        let other = String::from("true");
        assert!(eq(& value, & other));
    }
    #[test]
    fn test_eq_bool_false() {
        let value = Value::Bool(false);
        let other = String::from("false");
        assert!(eq(& value, & other));
    }
    #[test]
    fn test_eq_number() {
        let value = Value::Number(crate::Number::from(42));
        let other = String::from("42");
        assert!(eq(& value, & other));
    }
    #[test]
    fn test_eq_string() {
        let value = Value::String(String::from("hello"));
        let other = String::from("hello");
        assert!(eq(& value, & other));
    }
    #[test]
    fn test_eq_array() {
        let value = Value::Array(
            vec![
                Value::Number(crate ::Number::from(1)), Value::Number(crate
                ::Number::from(2))
            ],
        );
        let other = String::from("[1,2]");
        assert!(! eq(& value, & other));
    }
    #[test]
    fn test_eq_object() {
        let mut map = crate::Map::new();
        map.insert(String::from("key"), Value::Number(crate::Number::from(1)));
        let value = Value::Object(map);
        let other = String::from("{\"key\":1}");
        assert!(! eq(& value, & other));
    }
    #[test]
    fn test_eq_different_types() {
        let value = Value::Number(crate::Number::from(42));
        let other = String::from("42.0");
        assert!(! eq(& value, & other));
        let value = Value::Bool(true);
        let other = String::from("42");
        assert!(! eq(& value, & other));
        let value = Value::String(String::from("hello"));
        let other = String::from("42");
        assert!(! eq(& value, & other));
    }
}
#[cfg(test)]
mod tests_llm_16_758 {
    use crate::value::{Value, Number};
    use crate::map::Map;
    use std::iter::FromIterator;
    #[test]
    fn test_eq() {
        let _rug_st_tests_llm_16_758_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "hello";
        let rug_fuzz_2 = "hello";
        let rug_fuzz_3 = "world";
        let rug_fuzz_4 = "hello";
        let rug_fuzz_5 = 123;
        let rug_fuzz_6 = "123";
        let rug_fuzz_7 = 123;
        let rug_fuzz_8 = "456";
        let rug_fuzz_9 = 123;
        let rug_fuzz_10 = "123.0";
        let rug_fuzz_11 = "123";
        let rug_fuzz_12 = true;
        let rug_fuzz_13 = "true";
        let rug_fuzz_14 = false;
        let rug_fuzz_15 = "false";
        let rug_fuzz_16 = true;
        let rug_fuzz_17 = "false";
        let rug_fuzz_18 = false;
        let rug_fuzz_19 = "true";
        let rug_fuzz_20 = "true";
        let rug_fuzz_21 = "false";
        let rug_fuzz_22 = "null";
        let rug_fuzz_23 = "null";
        let rug_fuzz_24 = "null";
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = "null";
        let rug_fuzz_27 = false;
        let rug_fuzz_28 = "null";
        let rug_fuzz_29 = "a";
        let rug_fuzz_30 = r#"["a","b"]"#;
        let rug_fuzz_31 = "a";
        let rug_fuzz_32 = r#"["a","c"]"#;
        let rug_fuzz_33 = "a";
        let rug_fuzz_34 = r#"["a"]"#;
        let rug_fuzz_35 = r#"["a","b"]"#;
        let rug_fuzz_36 = "a";
        let rug_fuzz_37 = "1";
        let rug_fuzz_38 = r#"{"a":"1","b":"2"}"#;
        let rug_fuzz_39 = "a";
        let rug_fuzz_40 = "1";
        let rug_fuzz_41 = r#"{"a":"1","b":"3"}"#;
        let rug_fuzz_42 = "a";
        let rug_fuzz_43 = "1";
        let rug_fuzz_44 = r#"{"a":"1"}"#;
        let rug_fuzz_45 = r#"{"a":"1","b":"2"}"#;
        debug_assert!(Value::String(rug_fuzz_0.to_owned()).eq(rug_fuzz_1));
        debug_assert!(! Value::String(rug_fuzz_2.to_owned()).eq(rug_fuzz_3));
        debug_assert!(! Value::Null.eq(rug_fuzz_4));
        debug_assert!(Value::Number(Number::from(rug_fuzz_5)).eq(rug_fuzz_6));
        debug_assert!(! Value::Number(Number::from(rug_fuzz_7)).eq(rug_fuzz_8));
        debug_assert!(! Value::Number(Number::from(rug_fuzz_9)).eq(rug_fuzz_10));
        debug_assert!(! Value::Null.eq(rug_fuzz_11));
        debug_assert!(Value::Bool(rug_fuzz_12).eq(rug_fuzz_13));
        debug_assert!(Value::Bool(rug_fuzz_14).eq(rug_fuzz_15));
        debug_assert!(! Value::Bool(rug_fuzz_16).eq(rug_fuzz_17));
        debug_assert!(! Value::Bool(rug_fuzz_18).eq(rug_fuzz_19));
        debug_assert!(! Value::Null.eq(rug_fuzz_20));
        debug_assert!(! Value::Null.eq(rug_fuzz_21));
        debug_assert!(Value::Null.eq(rug_fuzz_22));
        debug_assert!(! Value::String(rug_fuzz_23.to_owned()).eq(rug_fuzz_24));
        debug_assert!(! Value::Number(Number::from(rug_fuzz_25)).eq(rug_fuzz_26));
        debug_assert!(! Value::Bool(rug_fuzz_27).eq(rug_fuzz_28));
        debug_assert!(
            Value::Array(vec![rug_fuzz_29.to_owned().into(), "b".to_owned().into()])
            .eq(rug_fuzz_30)
        );
        debug_assert!(
            ! Value::Array(vec![rug_fuzz_31.to_owned().into(), "b".to_owned().into()])
            .eq(rug_fuzz_32)
        );
        debug_assert!(
            ! Value::Array(vec![rug_fuzz_33.to_owned().into(), "b".to_owned().into()])
            .eq(rug_fuzz_34)
        );
        debug_assert!(! Value::Null.eq(rug_fuzz_35));
        debug_assert!(
            Value::Object(Map::from_iter(vec![(rug_fuzz_36.to_owned(),
            Value::from(rug_fuzz_37)), ("b".to_owned(), Value::from("2"))]))
            .eq(rug_fuzz_38)
        );
        debug_assert!(
            ! Value::Object(Map::from_iter(vec![(rug_fuzz_39.to_owned(),
            Value::from(rug_fuzz_40)), ("b".to_owned(), Value::from("2"))]))
            .eq(rug_fuzz_41)
        );
        debug_assert!(
            ! Value::Object(Map::from_iter(vec![(rug_fuzz_42.to_owned(),
            Value::from(rug_fuzz_43)), ("b".to_owned(), Value::from("2"))]))
            .eq(rug_fuzz_44)
        );
        debug_assert!(! Value::Null.eq(rug_fuzz_45));
        let _rug_ed_tests_llm_16_758_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_759_llm_16_759 {
    use crate::Value;
    #[test]
    fn eq_with_u16() {
        let _rug_st_tests_llm_16_759_llm_16_759_rrrruuuugggg_eq_with_u16 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42_u64;
        let rug_fuzz_2 = 42.0;
        let rug_fuzz_3 = 42_u16;
        let mut value_i64 = Value::Number(rug_fuzz_0.into());
        let mut value_u64 = Value::Number(rug_fuzz_1.into());
        let mut value_f64 = Value::Number(crate::Number::from_f64(rug_fuzz_2).unwrap());
        let other_u16 = rug_fuzz_3;
        debug_assert_eq!(
            & mut value_i64, & other_u16,
            "Value::Number(i64) should be equal to the same u16 number"
        );
        debug_assert_eq!(
            & mut value_u64, & other_u16,
            "Value::Number(u64) should be equal to the same u16 number"
        );
        debug_assert_ne!(
            & mut value_f64, & other_u16,
            "Value::Number(f64) should not be equal to u16 number when f64 represents a floating point number"
        );
        let _rug_ed_tests_llm_16_759_llm_16_759_rrrruuuugggg_eq_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_760 {
    use super::*;
    use crate::*;
    use crate::{json, Value};
    #[test]
    fn eq_u16_with_number() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_number = 0;
        let rug_fuzz_0 = 42;
        let num: u16 = rug_fuzz_0;
        let value_number = Value::Number(num.into());
        debug_assert!(value_number.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_number = 0;
    }
    #[test]
    fn eq_u16_with_string() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_string = 0;
        let rug_fuzz_0 = 42;
        let num: u16 = rug_fuzz_0;
        let value_string = Value::String(num.to_string());
        debug_assert!(! value_string.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_string = 0;
    }
    #[test]
    fn eq_u16_with_bool() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_bool = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = true;
        let num: u16 = rug_fuzz_0;
        let value_bool = Value::Bool(rug_fuzz_1);
        debug_assert!(! value_bool.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_bool = 0;
    }
    #[test]
    fn eq_u16_with_null() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_null = 0;
        let rug_fuzz_0 = 0;
        let num: u16 = rug_fuzz_0;
        let value_null = Value::Null;
        debug_assert!(! value_null.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_null = 0;
    }
    #[test]
    fn eq_u16_with_array() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_array = 0;
        let rug_fuzz_0 = 0;
        let num: u16 = rug_fuzz_0;
        let value_array = json!([]);
        debug_assert!(! value_array.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_array = 0;
    }
    #[test]
    fn eq_u16_with_object() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_object = 0;
        let rug_fuzz_0 = 0;
        let num: u16 = rug_fuzz_0;
        let value_object = json!({});
        debug_assert!(! value_object.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_object = 0;
    }
    #[test]
    fn eq_u16_with_same_number_value() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_same_number_value = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 123;
        let num: u16 = rug_fuzz_0;
        let value_number = json!(rug_fuzz_1);
        debug_assert!(value_number.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_same_number_value = 0;
    }
    #[test]
    fn eq_u16_with_different_number_value() {
        let _rug_st_tests_llm_16_760_rrrruuuugggg_eq_u16_with_different_number_value = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 321;
        let num: u16 = rug_fuzz_0;
        let value_number = json!(rug_fuzz_1);
        debug_assert!(! value_number.eq(& num));
        let _rug_ed_tests_llm_16_760_rrrruuuugggg_eq_u16_with_different_number_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_761 {
    use crate::{Number, Value};
    #[test]
    fn value_eq_u16() {
        let _rug_st_tests_llm_16_761_rrrruuuugggg_value_eq_u16 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42u16;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = "42";
        let rug_fuzz_4 = 42u16;
        let rug_fuzz_5 = 43u16;
        let num: u16 = rug_fuzz_0;
        let num_value: Value = Value::Number(Number::from(num));
        debug_assert_eq!(Value::Null, num);
        debug_assert_eq!(num_value, num);
        debug_assert_eq!(Value::Number(Number::from(rug_fuzz_1)), num);
        debug_assert!(Value::Bool(rug_fuzz_2) != num);
        debug_assert!(Value::String(rug_fuzz_3.to_string()) != num);
        debug_assert!(
            Value::Array(vec![Value::Number(Number::from(rug_fuzz_4))]) != num
        );
        debug_assert!(Value::Number(Number::from(rug_fuzz_5)) != num);
        let num: u16 = u16::MAX;
        debug_assert_eq!(Value::Number(Number::from(num)), num);
        let _rug_ed_tests_llm_16_761_rrrruuuugggg_value_eq_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_762 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_value_eq_uint() {
        let _rug_st_tests_llm_16_762_rrrruuuugggg_test_value_eq_uint = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = 42i64;
        let rug_fuzz_2 = 42.0;
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = 42u32;
        let val_number = Value::Number(Number::from(rug_fuzz_0));
        let val_number_neg = Value::Number(Number::from(-rug_fuzz_1));
        let val_number_float = Value::Number(Number::from_f64(rug_fuzz_2).unwrap());
        let val_not_number = Value::Bool(rug_fuzz_3);
        let uint = rug_fuzz_4;
        debug_assert_eq!(val_number.eq(& uint), true);
        debug_assert_eq!(val_number_neg.eq(& uint), false);
        debug_assert_eq!(val_number_float.eq(& uint), false);
        debug_assert_eq!(val_not_number.eq(& uint), false);
        let _rug_ed_tests_llm_16_762_rrrruuuugggg_test_value_eq_uint = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_763 {
    use crate::{json, Value};
    #[test]
    fn test_eq_u32() {
        let _rug_st_tests_llm_16_763_rrrruuuugggg_test_eq_u32 = 0;
        let rug_fuzz_0 = 42u32;
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = 999999u32;
        let rug_fuzz_3 = 42u32;
        let rug_fuzz_4 = 42u32;
        let rug_fuzz_5 = 42u32;
        let rug_fuzz_6 = 42u32;
        let rug_fuzz_7 = 42u32;
        debug_assert_eq!(Value::from(rug_fuzz_0), json!(42u32));
        debug_assert_eq!(Value::from(rug_fuzz_1), json!(0));
        debug_assert_eq!(Value::from(rug_fuzz_2), json!(999999));
        debug_assert_eq!(Value::from(u32::MAX), Value::from(u32::MAX as u64));
        debug_assert_ne!(Value::from(rug_fuzz_3), json!(43));
        debug_assert_ne!(Value::from(rug_fuzz_4), json!(42.0));
        debug_assert_ne!(Value::from(rug_fuzz_5), json!("42"));
        debug_assert_ne!(Value::from(rug_fuzz_6), json!([42]));
        debug_assert_ne!(Value::from(rug_fuzz_7), json!({ "42" : 42 }));
        debug_assert_eq!(Value::from(u32::MIN), json!(0));
        debug_assert_eq!(Value::from(u32::MAX), json!(u32::MAX));
        debug_assert_ne!(Value::from(u32::MAX), json!(- 1));
        let _rug_ed_tests_llm_16_763_rrrruuuugggg_test_eq_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_765 {
    use crate::value::{Value, from_value};
    #[test]
    fn eq_u64_value() {
        let _rug_st_tests_llm_16_765_rrrruuuugggg_eq_u64_value = 0;
        let rug_fuzz_0 = 123u64;
        let rug_fuzz_1 = 123i64;
        let rug_fuzz_2 = "123";
        let u64_val = rug_fuzz_0;
        let number_value: Value = Value::Number(u64::into(u64_val));
        let mut value = Value::Number(u64::into(u64_val));
        debug_assert!(< Value as PartialEq < u64 > > ::eq(& value, & u64_val));
        debug_assert!(< Value as PartialEq < u64 > > ::eq(& number_value, & u64_val));
        value = Value::Number(i64::into(-rug_fuzz_1));
        debug_assert!(! < Value as PartialEq < u64 > > ::eq(& value, & u64_val));
        value = Value::String(rug_fuzz_2.to_owned());
        debug_assert!(! < Value as PartialEq < u64 > > ::eq(& value, & u64_val));
        value = Value::Null;
        debug_assert!(! < Value as PartialEq < u64 > > ::eq(& value, & u64_val));
        value = from_value(crate::json!({ "key" : 123 })).unwrap();
        debug_assert!(! < Value as PartialEq < u64 > > ::eq(& value, & u64_val));
        let _rug_ed_tests_llm_16_765_rrrruuuugggg_eq_u64_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_766 {
    use crate::Value;
    #[test]
    fn test_eq_u64() {
        let _rug_st_tests_llm_16_766_rrrruuuugggg_test_eq_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let value_u64 = rug_fuzz_0;
        let value_json_number = Value::Number(value_u64.into());
        debug_assert_eq!(& value_json_number, & value_u64);
        debug_assert_eq!(& Value::Null, & 0u64);
        let _rug_ed_tests_llm_16_766_rrrruuuugggg_test_eq_u64 = 0;
    }
    #[test]
    fn test_not_eq_u64() {
        let _rug_st_tests_llm_16_766_rrrruuuugggg_test_not_eq_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = "42";
        let value_u64 = rug_fuzz_0;
        let value_json_string = Value::String(rug_fuzz_1.to_string());
        debug_assert_ne!(& value_json_string, & value_u64);
        let _rug_ed_tests_llm_16_766_rrrruuuugggg_test_not_eq_u64 = 0;
    }
    #[test]
    fn test_eq_i64() {
        let _rug_st_tests_llm_16_766_rrrruuuugggg_test_eq_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let value_i64 = -rug_fuzz_0;
        let value_json_number = Value::Number(value_i64.into());
        debug_assert_eq!(& value_json_number, & value_i64);
        let _rug_ed_tests_llm_16_766_rrrruuuugggg_test_eq_i64 = 0;
    }
    #[test]
    fn test_not_eq_i64() {
        let _rug_st_tests_llm_16_766_rrrruuuugggg_test_not_eq_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let rug_fuzz_1 = "-42";
        let value_i64 = -rug_fuzz_0;
        let value_json_string = Value::String(rug_fuzz_1.to_string());
        debug_assert_ne!(& value_json_string, & value_i64);
        let _rug_ed_tests_llm_16_766_rrrruuuugggg_test_not_eq_i64 = 0;
    }
    #[test]
    fn test_eq_null() {
        let _rug_st_tests_llm_16_766_rrrruuuugggg_test_eq_null = 0;
        debug_assert_eq!(& Value::Null, & 0u64);
        let _rug_ed_tests_llm_16_766_rrrruuuugggg_test_eq_null = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_769 {
    use super::*;
    use crate::*;
    use crate::value::{Number, Value};
    use crate::json;
    #[test]
    fn eq_u8_to_value() {
        let _rug_st_tests_llm_16_769_rrrruuuugggg_eq_u8_to_value = 0;
        let rug_fuzz_0 = 10u8;
        let rug_fuzz_1 = "10";
        let rug_fuzz_2 = 1;
        let u8_val = rug_fuzz_0;
        let number_val = Value::Number(Number::from(u8_val));
        let other_val = Value::String(String::from(rug_fuzz_1));
        debug_assert!(< Value as PartialEq < u8 > > ::eq(& number_val, & u8_val));
        debug_assert!(
            ! < Value as PartialEq < u8 > > ::eq(& number_val, & (u8_val + rug_fuzz_2))
        );
        debug_assert!(! < Value as PartialEq < u8 > > ::eq(& other_val, & u8_val));
        let _rug_ed_tests_llm_16_769_rrrruuuugggg_eq_u8_to_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_770 {
    use crate::value::Value;
    #[test]
    fn test_eq_null_and_u8() {
        let _rug_st_tests_llm_16_770_rrrruuuugggg_test_eq_null_and_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let null_value = Value::Null;
        debug_assert_eq!(null_value == Value::from(rug_fuzz_0), false);
        let _rug_ed_tests_llm_16_770_rrrruuuugggg_test_eq_null_and_u8 = 0;
    }
    #[test]
    fn test_eq_bool_and_u8() {
        let _rug_st_tests_llm_16_770_rrrruuuugggg_test_eq_bool_and_u8 = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 1u8;
        let bool_value = Value::Bool(rug_fuzz_0);
        debug_assert_eq!(bool_value == Value::from(rug_fuzz_1), false);
        let _rug_ed_tests_llm_16_770_rrrruuuugggg_test_eq_bool_and_u8 = 0;
    }
    #[test]
    fn test_eq_number_and_u8() {
        let _rug_st_tests_llm_16_770_rrrruuuugggg_test_eq_number_and_u8 = 0;
        let rug_fuzz_0 = 123u8;
        let rug_fuzz_1 = 123u8;
        let number_value = Value::Number(rug_fuzz_0.into());
        debug_assert_eq!(number_value == Value::from(rug_fuzz_1), true);
        let _rug_ed_tests_llm_16_770_rrrruuuugggg_test_eq_number_and_u8 = 0;
    }
    #[test]
    fn test_eq_string_and_u8() {
        let _rug_st_tests_llm_16_770_rrrruuuugggg_test_eq_string_and_u8 = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 123u8;
        let string_value = Value::String(String::from(rug_fuzz_0));
        debug_assert_eq!(string_value == Value::from(rug_fuzz_1), false);
        let _rug_ed_tests_llm_16_770_rrrruuuugggg_test_eq_string_and_u8 = 0;
    }
    #[test]
    fn test_eq_array_and_u8() {
        let _rug_st_tests_llm_16_770_rrrruuuugggg_test_eq_array_and_u8 = 0;
        let rug_fuzz_0 = 123u8;
        let rug_fuzz_1 = 123u8;
        let array_value = Value::Array(vec![Value::from(rug_fuzz_0)]);
        debug_assert_eq!(array_value == Value::from(rug_fuzz_1), false);
        let _rug_ed_tests_llm_16_770_rrrruuuugggg_test_eq_array_and_u8 = 0;
    }
    #[test]
    fn test_eq_object_and_u8() {
        let _rug_st_tests_llm_16_770_rrrruuuugggg_test_eq_object_and_u8 = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 123u8;
        let rug_fuzz_2 = 123u8;
        let mut map = crate::Map::new();
        map.insert(String::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        let object_value = Value::Object(map);
        debug_assert_eq!(object_value == Value::from(rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_770_rrrruuuugggg_test_eq_object_and_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_771 {
    use crate::Value;
    #[test]
    fn value_partial_eq_usize() {
        let _rug_st_tests_llm_16_771_rrrruuuugggg_value_partial_eq_usize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 3;
        let value = &mut Value::Array(
            vec![
                Value::Number(rug_fuzz_0.into()), Value::Number(1.into()),
                Value::Number(2.into())
            ],
        );
        debug_assert!(< Value as PartialEq < usize > > ::eq(value, & rug_fuzz_1));
        debug_assert!(< Value as PartialEq < usize > > ::eq(value, & rug_fuzz_2));
        debug_assert!(< Value as PartialEq < usize > > ::eq(value, & rug_fuzz_3));
        debug_assert!(! < Value as PartialEq < usize > > ::eq(value, & rug_fuzz_4));
        debug_assert!(! < Value as PartialEq < usize > > ::eq(value, & usize::MAX));
        let _rug_ed_tests_llm_16_771_rrrruuuugggg_value_partial_eq_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_772 {
    use crate::{json, Value};
    #[test]
    fn ensure_partial_eq_for_usize_with_value() {
        let _rug_st_tests_llm_16_772_rrrruuuugggg_ensure_partial_eq_for_usize_with_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let n: usize = rug_fuzz_0;
        let v: Value = json!(rug_fuzz_1);
        debug_assert_eq!(& v, & n);
        let _rug_ed_tests_llm_16_772_rrrruuuugggg_ensure_partial_eq_for_usize_with_value = 0;
    }
    #[test]
    fn ensure_value_not_partial_eq_with_different_usize() {
        let _rug_st_tests_llm_16_772_rrrruuuugggg_ensure_value_not_partial_eq_with_different_usize = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 42;
        let n: usize = rug_fuzz_0;
        let v: Value = json!(rug_fuzz_1);
        debug_assert_ne!(& v, & n);
        let _rug_ed_tests_llm_16_772_rrrruuuugggg_ensure_value_not_partial_eq_with_different_usize = 0;
    }
    #[test]
    fn ensure_value_partial_eq_with_usize_in_array() {
        let _rug_st_tests_llm_16_772_rrrruuuugggg_ensure_value_partial_eq_with_usize_in_array = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 43;
        let rug_fuzz_3 = 44;
        let n: usize = rug_fuzz_0;
        let v: Value = json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let array = v.as_array().unwrap();
        debug_assert!(array.iter().any(| value | value == & n));
        let _rug_ed_tests_llm_16_772_rrrruuuugggg_ensure_value_partial_eq_with_usize_in_array = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_773_llm_16_773 {
    use crate::value::{Number, Value};
    #[test]
    fn value_eq_usize() {
        let _rug_st_tests_llm_16_773_llm_16_773_rrrruuuugggg_value_eq_usize = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42usize;
        let rug_fuzz_2 = 100usize;
        let rug_fuzz_3 = "hello";
        let rug_fuzz_4 = "hello";
        let rug_fuzz_5 = "world";
        let rug_fuzz_6 = true;
        let rug_fuzz_7 = 1usize;
        let rug_fuzz_8 = 0usize;
        let value_num = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(Value::eq(& value_num, & rug_fuzz_1));
        debug_assert!(! Value::eq(& value_num, & rug_fuzz_2));
        let value_str = Value::String(rug_fuzz_3.to_string());
        debug_assert!(Value::eq(& value_str, & rug_fuzz_4.to_string()));
        debug_assert!(! Value::eq(& value_str, & rug_fuzz_5.to_string()));
        let value_bool = Value::Bool(rug_fuzz_6);
        debug_assert!(Value::eq(& value_bool, & rug_fuzz_7));
        debug_assert!(! Value::eq(& value_bool, & rug_fuzz_8));
        let _rug_ed_tests_llm_16_773_llm_16_773_rrrruuuugggg_value_eq_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_775 {
    use crate::Value;
    #[test]
    fn eq_bool_true_with_json_true() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_true_with_json_true = 0;
        let rug_fuzz_0 = true;
        debug_assert_eq!(rug_fuzz_0, Value::Bool(true));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_true_with_json_true = 0;
    }
    #[test]
    fn eq_bool_true_with_json_false() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_true_with_json_false = 0;
        let rug_fuzz_0 = true;
        debug_assert_ne!(rug_fuzz_0, Value::Bool(false));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_true_with_json_false = 0;
    }
    #[test]
    fn eq_bool_false_with_json_true() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_false_with_json_true = 0;
        let rug_fuzz_0 = false;
        debug_assert_ne!(rug_fuzz_0, Value::Bool(true));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_false_with_json_true = 0;
    }
    #[test]
    fn eq_bool_false_with_json_false() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_false_with_json_false = 0;
        let rug_fuzz_0 = false;
        debug_assert_eq!(rug_fuzz_0, Value::Bool(false));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_false_with_json_false = 0;
    }
    #[test]
    fn eq_bool_with_json_null() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_null = 0;
        let rug_fuzz_0 = false;
        debug_assert_ne!(rug_fuzz_0, Value::Null);
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_null = 0;
    }
    #[test]
    fn eq_bool_with_json_number() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_number = 0;
        let rug_fuzz_0 = true;
        debug_assert_ne!(rug_fuzz_0, Value::Number(1.into()));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_number = 0;
    }
    #[test]
    fn eq_bool_with_json_string() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_string = 0;
        let rug_fuzz_0 = false;
        debug_assert_ne!(rug_fuzz_0, Value::String("false".to_string()));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_string = 0;
    }
    #[test]
    fn eq_bool_with_json_array() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_array = 0;
        let rug_fuzz_0 = true;
        debug_assert_ne!(rug_fuzz_0, Value::Array(vec![Value::Bool(true)]));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_array = 0;
    }
    #[test]
    fn eq_bool_with_json_object() {
        let _rug_st_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_object = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = false;
        use crate::map::Map;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Bool(rug_fuzz_1));
        debug_assert_ne!(rug_fuzz_2, Value::Object(map));
        let _rug_ed_tests_llm_16_775_rrrruuuugggg_eq_bool_with_json_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_776_llm_16_776 {
    use crate::value::{Value, Number};
    #[test]
    fn f32_eq_with_json_number() {
        let _rug_st_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_number = 0;
        let rug_fuzz_0 = 12.34;
        let f: f32 = rug_fuzz_0;
        let n = Number::from_f32(f).unwrap();
        let v = Value::Number(n);
        debug_assert!(f.eq(& v));
        let _rug_ed_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_number = 0;
    }
    #[test]
    fn f32_eq_with_json_null() {
        let _rug_st_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_null = 0;
        let rug_fuzz_0 = 12.34;
        let f: f32 = rug_fuzz_0;
        let v = Value::Null;
        debug_assert!(! f.eq(& v));
        let _rug_ed_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_null = 0;
    }
    #[test]
    fn f32_eq_with_json_bool() {
        let _rug_st_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_bool = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = true;
        let f: f32 = rug_fuzz_0;
        let v = Value::Bool(rug_fuzz_1);
        debug_assert!(f.eq(& v));
        let _rug_ed_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_bool = 0;
    }
    #[test]
    fn f32_eq_with_json_string() {
        let _rug_st_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_string = 0;
        let rug_fuzz_0 = 12.34;
        let rug_fuzz_1 = "12.34";
        let f: f32 = rug_fuzz_0;
        let v = Value::String(rug_fuzz_1.to_owned());
        debug_assert!(! f.eq(& v));
        let _rug_ed_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_string = 0;
    }
    #[test]
    fn f32_eq_with_json_array() {
        let _rug_st_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_array = 0;
        let rug_fuzz_0 = 12.34;
        let rug_fuzz_1 = 12.34;
        let f: f32 = rug_fuzz_0;
        let v = Value::Array(
            vec![
                Value::Number(Number::from_f32(rug_fuzz_1).unwrap()),
                Value::Number(Number::from_f32(56.78).unwrap())
            ],
        );
        debug_assert!(! f.eq(& v));
        let _rug_ed_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_array = 0;
    }
    #[test]
    fn f32_eq_with_json_object() {
        let _rug_st_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_object = 0;
        let rug_fuzz_0 = 12.34;
        let rug_fuzz_1 = "value";
        let rug_fuzz_2 = 12.34;
        let f: f32 = rug_fuzz_0;
        let mut map = crate::map::Map::new();
        map.insert(
            rug_fuzz_1.to_string(),
            Value::Number(Number::from_f32(rug_fuzz_2).unwrap()),
        );
        let v = Value::Object(map);
        debug_assert!(! f.eq(& v));
        let _rug_ed_tests_llm_16_776_llm_16_776_rrrruuuugggg_f32_eq_with_json_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_777 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn eq_with_value_null() {
        let _rug_st_tests_llm_16_777_rrrruuuugggg_eq_with_value_null = 0;
        let rug_fuzz_0 = 42.0;
        let num = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Null));
        let _rug_ed_tests_llm_16_777_rrrruuuugggg_eq_with_value_null = 0;
    }
    #[test]
    fn eq_with_value_bool() {
        let _rug_st_tests_llm_16_777_rrrruuuugggg_eq_with_value_bool = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = true;
        let num = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_1)));
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_2)));
        let _rug_ed_tests_llm_16_777_rrrruuuugggg_eq_with_value_bool = 0;
    }
    #[test]
    fn eq_with_value_number() {
        let _rug_st_tests_llm_16_777_rrrruuuugggg_eq_with_value_number = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 100.0;
        let num = rug_fuzz_0;
        debug_assert!(num.eq(& Value::Number(Number::from_f64(rug_fuzz_1).unwrap())));
        debug_assert!(! num.eq(& Value::Number(Number::from_f64(rug_fuzz_2).unwrap())));
        let _rug_ed_tests_llm_16_777_rrrruuuugggg_eq_with_value_number = 0;
    }
    #[test]
    fn eq_with_value_string() {
        let _rug_st_tests_llm_16_777_rrrruuuugggg_eq_with_value_string = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = "42";
        let num = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::String(rug_fuzz_1.to_string())));
        let _rug_ed_tests_llm_16_777_rrrruuuugggg_eq_with_value_string = 0;
    }
    #[test]
    fn eq_with_value_array() {
        let _rug_st_tests_llm_16_777_rrrruuuugggg_eq_with_value_array = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let num = rug_fuzz_0;
        let array = vec![Value::Number(Number::from_f64(rug_fuzz_1).unwrap())];
        debug_assert!(! num.eq(& Value::Array(array)));
        let _rug_ed_tests_llm_16_777_rrrruuuugggg_eq_with_value_array = 0;
    }
    #[test]
    fn eq_with_value_object() {
        let _rug_st_tests_llm_16_777_rrrruuuugggg_eq_with_value_object = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = "number";
        let rug_fuzz_2 = 42.0;
        let num = rug_fuzz_0;
        let mut map = crate::Map::new();
        map.insert(
            rug_fuzz_1.to_string(),
            Value::Number(Number::from_f64(rug_fuzz_2).unwrap()),
        );
        debug_assert!(! num.eq(& Value::Object(map)));
        let _rug_ed_tests_llm_16_777_rrrruuuugggg_eq_with_value_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_778 {
    use crate::value::Value;
    #[test]
    fn i16_equal_to_i16_value() {
        let _rug_st_tests_llm_16_778_rrrruuuugggg_i16_equal_to_i16_value = 0;
        let rug_fuzz_0 = 16;
        let rug_fuzz_1 = 16i16;
        let value = Value::Number(rug_fuzz_0.into());
        debug_assert_eq!(rug_fuzz_1.eq(& value), true);
        let _rug_ed_tests_llm_16_778_rrrruuuugggg_i16_equal_to_i16_value = 0;
    }
    #[test]
    fn i16_not_equal_to_other_value_types() {
        let _rug_st_tests_llm_16_778_rrrruuuugggg_i16_not_equal_to_other_value_types = 0;
        let rug_fuzz_0 = 16i16;
        let rug_fuzz_1 = 16i16;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = 16i16;
        let rug_fuzz_4 = "16";
        let rug_fuzz_5 = 16i16;
        let rug_fuzz_6 = 16;
        let rug_fuzz_7 = 16i16;
        debug_assert_eq!(rug_fuzz_0.eq(& Value::Null), false);
        debug_assert_eq!(rug_fuzz_1.eq(& Value::Bool(rug_fuzz_2)), false);
        debug_assert_eq!(
            rug_fuzz_3.eq(& Value::String(String::from(rug_fuzz_4))), false
        );
        debug_assert_eq!(
            rug_fuzz_5.eq(& Value::Array(vec![Value::Number(rug_fuzz_6.into())])), false
        );
        debug_assert_eq!(rug_fuzz_7.eq(& Value::Object(crate ::Map::new())), false);
        let _rug_ed_tests_llm_16_778_rrrruuuugggg_i16_not_equal_to_other_value_types = 0;
    }
    #[test]
    fn i16_equal_to_other_numbers() {
        let _rug_st_tests_llm_16_778_rrrruuuugggg_i16_equal_to_other_numbers = 0;
        let rug_fuzz_0 = 16i16;
        let rug_fuzz_1 = 16;
        debug_assert_eq!(rug_fuzz_0.eq(& Value::Number(rug_fuzz_1.into())), true);
        let _rug_ed_tests_llm_16_778_rrrruuuugggg_i16_equal_to_other_numbers = 0;
    }
    #[test]
    fn i16_not_equal_to_different_numbers() {
        let _rug_st_tests_llm_16_778_rrrruuuugggg_i16_not_equal_to_different_numbers = 0;
        let rug_fuzz_0 = 16i16;
        let rug_fuzz_1 = 17;
        debug_assert_eq!(rug_fuzz_0.eq(& Value::Number(rug_fuzz_1.into())), false);
        let _rug_ed_tests_llm_16_778_rrrruuuugggg_i16_not_equal_to_different_numbers = 0;
    }
    #[test]
    fn i16_not_equal_to_floating_numbers() {
        let _rug_st_tests_llm_16_778_rrrruuuugggg_i16_not_equal_to_floating_numbers = 0;
        let rug_fuzz_0 = 16.1;
        let rug_fuzz_1 = 16i16;
        let float_number = crate::Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert_eq!(rug_fuzz_1.eq(& Value::Number(float_number)), false);
        let _rug_ed_tests_llm_16_778_rrrruuuugggg_i16_not_equal_to_floating_numbers = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_779 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn eq_with_json_null() {
        let _rug_st_tests_llm_16_779_rrrruuuugggg_eq_with_json_null = 0;
        let rug_fuzz_0 = 1;
        debug_assert!(! i32::eq(& rug_fuzz_0, & Value::Null));
        let _rug_ed_tests_llm_16_779_rrrruuuugggg_eq_with_json_null = 0;
    }
    #[test]
    fn eq_with_json_bool() {
        let _rug_st_tests_llm_16_779_rrrruuuugggg_eq_with_json_bool = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = true;
        debug_assert!(! i32::eq(& rug_fuzz_0, & Value::Bool(rug_fuzz_1)));
        let _rug_ed_tests_llm_16_779_rrrruuuugggg_eq_with_json_bool = 0;
    }
    #[test]
    fn eq_with_json_number() {
        let _rug_st_tests_llm_16_779_rrrruuuugggg_eq_with_json_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 43;
        let rug_fuzz_4 = 42;
        let rug_fuzz_5 = 42.0;
        let rug_fuzz_6 = 42;
        debug_assert!(i32::eq(& rug_fuzz_0, & Value::Number(Number::from(rug_fuzz_1))));
        debug_assert!(
            ! i32::eq(& rug_fuzz_2, & Value::Number(Number::from(rug_fuzz_3)))
        );
        debug_assert!(
            ! i32::eq(& rug_fuzz_4, & Value::Number(Number::from_f64(rug_fuzz_5)
            .unwrap()))
        );
        debug_assert!(
            ! i32::eq(& rug_fuzz_6, & Value::Number(Number::from_f64(f64::NAN).unwrap()))
        );
        let _rug_ed_tests_llm_16_779_rrrruuuugggg_eq_with_json_number = 0;
    }
    #[test]
    fn eq_with_json_string() {
        let _rug_st_tests_llm_16_779_rrrruuuugggg_eq_with_json_string = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "42";
        debug_assert!(! i32::eq(& rug_fuzz_0, & Value::String(rug_fuzz_1.to_owned())));
        let _rug_ed_tests_llm_16_779_rrrruuuugggg_eq_with_json_string = 0;
    }
    #[test]
    fn eq_with_json_array() {
        let _rug_st_tests_llm_16_779_rrrruuuugggg_eq_with_json_array = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        debug_assert!(
            ! i32::eq(& rug_fuzz_0, &
            Value::Array(vec![Value::Number(Number::from(rug_fuzz_1))]))
        );
        let _rug_ed_tests_llm_16_779_rrrruuuugggg_eq_with_json_array = 0;
    }
    #[test]
    fn eq_with_json_object() {
        let _rug_st_tests_llm_16_779_rrrruuuugggg_eq_with_json_object = 0;
        let rug_fuzz_0 = "forty_two";
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42;
        let mut object = crate::Map::new();
        object.insert(rug_fuzz_0.to_owned(), Value::Number(Number::from(rug_fuzz_1)));
        debug_assert!(! i32::eq(& rug_fuzz_2, & Value::Object(object)));
        let _rug_ed_tests_llm_16_779_rrrruuuugggg_eq_with_json_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_780 {
    use crate::Value;
    #[test]
    fn test_eq_with_i64_and_value() {
        let _rug_st_tests_llm_16_780_rrrruuuugggg_test_eq_with_i64_and_value = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42_i64;
        let rug_fuzz_2 = 42_i64;
        let rug_fuzz_3 = 42_i64;
        debug_assert_eq!(Value::from(rug_fuzz_0), 42_i64);
        debug_assert_eq!(Value::from(- rug_fuzz_1), - 42_i64);
        debug_assert_ne!(Value::from(rug_fuzz_2), 43_i64);
        debug_assert_ne!(Value::from(- rug_fuzz_3), - 41_i64);
        let _rug_ed_tests_llm_16_780_rrrruuuugggg_test_eq_with_i64_and_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_782_llm_16_782 {
    use crate::{json, Number, Value};
    #[test]
    fn eq_with_number() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(value.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_number = 0;
    }
    #[test]
    fn eq_with_positive_number() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_positive_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let value = Value::Number(Number::from(-rug_fuzz_0));
        debug_assert!(value.eq(& (- rug_fuzz_1)));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_positive_number = 0;
    }
    #[test]
    fn eq_with_string() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_string = 0;
        let rug_fuzz_0 = "foo";
        let rug_fuzz_1 = "foo";
        let value = Value::String(rug_fuzz_0.to_owned());
        debug_assert!(value.eq(& rug_fuzz_1.to_string()));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_string = 0;
    }
    #[test]
    fn eq_with_bool() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 1;
        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(value.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_bool = 0;
    }
    #[test]
    fn eq_with_null() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_null = 0;
        let rug_fuzz_0 = 1;
        let value = Value::Null;
        debug_assert!(! value.eq(& rug_fuzz_0));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_null = 0;
    }
    #[test]
    fn eq_with_array() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_array = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let value = Value::Array(vec![json!(rug_fuzz_0), json!(2), json!(3)]);
        debug_assert!(! value.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_array = 0;
    }
    #[test]
    fn eq_with_object() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_object = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let mut map = crate::map::Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        let value = Value::Object(map);
        debug_assert!(! value.eq(& rug_fuzz_2));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_object = 0;
    }
    #[test]
    fn eq_with_float() {
        let _rug_st_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_float = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42;
        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(value.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_782_llm_16_782_rrrruuuugggg_eq_with_float = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_783 {
    use crate::Value;
    use std::string::String;
    #[test]
    fn test_string_eq_value() {
        let _rug_st_tests_llm_16_783_rrrruuuugggg_test_string_eq_value = 0;
        let rug_fuzz_0 = "example string";
        let rug_fuzz_1 = "example string";
        let rug_fuzz_2 = 123;
        let string_value = String::from(rug_fuzz_0);
        let value_string = Value::String(rug_fuzz_1.to_owned());
        let value_number = Value::Number(rug_fuzz_2.into());
        let value_null = Value::Null;
        debug_assert!(string_value.eq(& value_string));
        debug_assert!(! string_value.eq(& value_number));
        debug_assert!(! string_value.eq(& value_null));
        let _rug_ed_tests_llm_16_783_rrrruuuugggg_test_string_eq_value = 0;
    }
    #[test]
    fn test_string_eq_value_with_escape_chars() {
        let _rug_st_tests_llm_16_783_rrrruuuugggg_test_string_eq_value_with_escape_chars = 0;
        let rug_fuzz_0 = "esc\\ape\\d string";
        let rug_fuzz_1 = "esc\\ape\\d string";
        let rug_fuzz_2 = "escaped string";
        let string_value = String::from(rug_fuzz_0);
        let value_string = Value::String(rug_fuzz_1.to_owned());
        let value_string_unesc = Value::String(rug_fuzz_2.to_owned());
        debug_assert!(string_value.eq(& value_string));
        debug_assert!(! string_value.eq(& value_string_unesc));
        let _rug_ed_tests_llm_16_783_rrrruuuugggg_test_string_eq_value_with_escape_chars = 0;
    }
    #[test]
    fn test_string_eq_value_with_different_case() {
        let _rug_st_tests_llm_16_783_rrrruuuugggg_test_string_eq_value_with_different_case = 0;
        let rug_fuzz_0 = "CaseSensitive";
        let rug_fuzz_1 = "CaseSensitive";
        let rug_fuzz_2 = "casesensitive";
        let string_value = String::from(rug_fuzz_0);
        let value_string_same_case = Value::String(rug_fuzz_1.to_owned());
        let value_string_diff_case = Value::String(rug_fuzz_2.to_owned());
        debug_assert!(string_value.eq(& value_string_same_case));
        debug_assert!(! string_value.eq(& value_string_diff_case));
        let _rug_ed_tests_llm_16_783_rrrruuuugggg_test_string_eq_value_with_different_case = 0;
    }
    #[test]
    fn test_string_eq_value_with_non_string_value() {
        let _rug_st_tests_llm_16_783_rrrruuuugggg_test_string_eq_value_with_non_string_value = 0;
        let rug_fuzz_0 = "non-string value";
        let rug_fuzz_1 = true;
        let string_value = String::from(rug_fuzz_0);
        let value_bool = Value::Bool(rug_fuzz_1);
        let value_object = Value::Object(crate::Map::new());
        debug_assert!(! string_value.eq(& value_bool));
        debug_assert!(! string_value.eq(& value_object));
        let _rug_ed_tests_llm_16_783_rrrruuuugggg_test_string_eq_value_with_non_string_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_784 {
    use crate::Value;
    #[test]
    fn test_eq() {
        let _rug_st_tests_llm_16_784_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = "value";
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = "value";
        let rug_fuzz_3 = "Value";
        let rug_fuzz_4 = "other";
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = "value";
        let rug_fuzz_9 = "key";
        let rug_fuzz_10 = "other";
        let rug_fuzz_11 = "key";
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = "array";
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 4;
        let rug_fuzz_16 = "array";
        let rug_fuzz_17 = 2;
        let rug_fuzz_18 = "value";
        let rug_fuzz_19 = "array";
        let obj = crate::json!({ "key" : "value", "array" : [1, 2, 3] });
        let val_str = rug_fuzz_0;
        let val_num = rug_fuzz_1;
        debug_assert_eq!(val_str.eq(& Value::String(rug_fuzz_2.to_owned())), true);
        debug_assert_eq!(val_str.eq(& Value::String(rug_fuzz_3.to_owned())), false);
        debug_assert_eq!(val_str.eq(& Value::String(rug_fuzz_4.to_owned())), false);
        debug_assert_eq!(val_str.eq(& Value::Null), false);
        debug_assert_eq!(val_num.eq(& Value::Number(rug_fuzz_5.into())), true);
        debug_assert_eq!(val_num.eq(& Value::Number(rug_fuzz_6.into())), false);
        debug_assert_eq!(val_num.eq(& Value::Number((- rug_fuzz_7).into())), false);
        debug_assert_eq!(val_num.eq(& Value::Null), false);
        debug_assert_eq!(rug_fuzz_8.eq(& obj[rug_fuzz_9]), true);
        debug_assert_eq!(rug_fuzz_10.eq(& obj[rug_fuzz_11]), false);
        debug_assert_eq!(rug_fuzz_12.eq(& obj[rug_fuzz_13] [rug_fuzz_14]), true);
        debug_assert_eq!(rug_fuzz_15.eq(& obj[rug_fuzz_16] [rug_fuzz_17]), false);
        debug_assert_eq!(rug_fuzz_18.eq(& obj[rug_fuzz_19]), false);
        let _rug_ed_tests_llm_16_784_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_785 {
    use crate::Value;
    use std::u16;
    #[test]
    fn test_eq_with_value() {
        let _rug_st_tests_llm_16_785_rrrruuuugggg_test_eq_with_value = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "10";
        let rug_fuzz_4 = 10;
        let value_number = Value::Number(rug_fuzz_0.into());
        debug_assert_eq!(u16::eq(& rug_fuzz_1, & value_number), true);
        debug_assert_eq!(u16::eq(& u16::MAX, & value_number), false);
        debug_assert_eq!(u16::eq(& rug_fuzz_2, & value_number), false);
        let value_string = Value::String(rug_fuzz_3.to_string());
        debug_assert_eq!(u16::eq(& rug_fuzz_4, & value_string), false);
        let _rug_ed_tests_llm_16_785_rrrruuuugggg_test_eq_with_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_786 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq_with_null() {
        let _rug_st_tests_llm_16_786_rrrruuuugggg_test_eq_with_null = 0;
        let rug_fuzz_0 = 42;
        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Null));
        let _rug_ed_tests_llm_16_786_rrrruuuugggg_test_eq_with_null = 0;
    }
    #[test]
    fn test_eq_with_bool() {
        let _rug_st_tests_llm_16_786_rrrruuuugggg_test_eq_with_bool = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = true;
        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_1)));
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_2)));
        let _rug_ed_tests_llm_16_786_rrrruuuugggg_test_eq_with_bool = 0;
    }
    #[test]
    fn test_eq_with_number() {
        let _rug_st_tests_llm_16_786_rrrruuuugggg_test_eq_with_number = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 42;
        let num: u32 = rug_fuzz_0;
        debug_assert!(num.eq(& Value::Number(rug_fuzz_1.into())));
        debug_assert!(! num.eq(& Value::Number(rug_fuzz_2.into())));
        debug_assert!(! num.eq(& Value::Number((- rug_fuzz_3).into())));
        let _rug_ed_tests_llm_16_786_rrrruuuugggg_test_eq_with_number = 0;
    }
    #[test]
    fn test_eq_with_string() {
        let _rug_st_tests_llm_16_786_rrrruuuugggg_test_eq_with_string = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "42";
        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::String(rug_fuzz_1.to_owned())));
        let _rug_ed_tests_llm_16_786_rrrruuuugggg_test_eq_with_string = 0;
    }
    #[test]
    fn test_eq_with_array() {
        let _rug_st_tests_llm_16_786_rrrruuuugggg_test_eq_with_array = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Array(vec![Value::Number(rug_fuzz_1.into())])));
        let _rug_ed_tests_llm_16_786_rrrruuuugggg_test_eq_with_array = 0;
    }
    #[test]
    fn test_eq_with_object() {
        let _rug_st_tests_llm_16_786_rrrruuuugggg_test_eq_with_object = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = 42;
        let num: u32 = rug_fuzz_0;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_1.to_string(), Value::Number(rug_fuzz_2.into()));
        debug_assert!(! num.eq(& Value::Object(map)));
        let _rug_ed_tests_llm_16_786_rrrruuuugggg_test_eq_with_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_787_llm_16_787 {
    use crate::Value;
    use crate::Number;
    #[test]
    fn u64_eq() {
        let _rug_st_tests_llm_16_787_llm_16_787_rrrruuuugggg_u64_eq = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = "42";
        let rug_fuzz_2 = 42.0;
        let num = rug_fuzz_0;
        let value_num = Value::Number(Number::from(num));
        let value_str = Value::String(rug_fuzz_1.to_owned());
        let value_f64 = Value::Number(Number::from_f64(rug_fuzz_2).unwrap());
        let value_null = Value::Null;
        debug_assert_eq!(num.eq(& value_num), true);
        debug_assert_eq!(num.eq(& value_str), false);
        debug_assert_eq!(num.eq(& value_f64), true);
        debug_assert_eq!(num.eq(& value_null), false);
        let _rug_ed_tests_llm_16_787_llm_16_787_rrrruuuugggg_u64_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_788_llm_16_788 {
    use crate::value::Value;
    use crate::number::Number;
    #[test]
    fn test_u8_eq() {
        let _rug_st_tests_llm_16_788_llm_16_788_rrrruuuugggg_test_u8_eq = 0;
        let rug_fuzz_0 = 8u8;
        let rug_fuzz_1 = 8u8;
        let rug_fuzz_2 = true;
        let test_values = vec![
            (rug_fuzz_0, Value::Number(Number::from(rug_fuzz_1)), rug_fuzz_2),
            (u8::max_value(), Value::Number(Number::from(u8::max_value())), true), (0u8,
            Value::Number(Number::from(0u64)), true), (128u8,
            Value::Number(Number::from(127u8)), false), (255u8,
            Value::Number(Number::from(254u8)), false), (10u8, Value::String("10"
            .to_owned()), false), (42u8,
            Value::Array(vec![Value::Number(Number::from(42u8))]), false), (1u8,
            Value::Bool(true), false), (0u8, Value::Bool(false), false), (100u8,
            Value::Null, false), (200u8, Value::Object(crate ::map::Map::new()), false)
        ];
        for (u, value, expected) in test_values {
            debug_assert_eq!(u.eq(& value), expected);
        }
        let _rug_ed_tests_llm_16_788_llm_16_788_rrrruuuugggg_test_u8_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_789 {
    use crate::value::{Value, Number};
    use crate::map::Map;
    #[test]
    fn test_eq_usize_with_json_null() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_null = 0;
        let rug_fuzz_0 = 42_usize;
        let number = rug_fuzz_0;
        debug_assert_eq!(number.eq(& Value::Null), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_null = 0;
    }
    #[test]
    fn test_eq_usize_with_json_bool() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_bool = 0;
        let rug_fuzz_0 = 1_usize;
        let rug_fuzz_1 = true;
        let number = rug_fuzz_0;
        debug_assert_eq!(number.eq(& Value::Bool(rug_fuzz_1)), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_bool = 0;
    }
    #[test]
    fn test_eq_usize_with_json_number_positive_int() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_number_positive_int = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = 42;
        let number = rug_fuzz_0;
        let value_number = Number::from(rug_fuzz_1);
        debug_assert_eq!(number.eq(& Value::Number(value_number)), true);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_number_positive_int = 0;
    }
    #[test]
    fn test_eq_usize_with_json_number_negative_int() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_number_negative_int = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = 42;
        let number = rug_fuzz_0;
        let value_number = Number::from(-rug_fuzz_1);
        debug_assert_eq!(number.eq(& Value::Number(value_number)), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_number_negative_int = 0;
    }
    #[test]
    fn test_eq_usize_with_json_number_float() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_number_float = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = 42.0;
        let number = rug_fuzz_0;
        let value_number = Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert_eq!(number.eq(& Value::Number(value_number)), true);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_number_float = 0;
    }
    #[test]
    fn test_eq_usize_with_json_string() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_string = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = "42";
        let number = rug_fuzz_0;
        let value_string = Value::String(rug_fuzz_1.to_string());
        debug_assert_eq!(number.eq(& value_string), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_string = 0;
    }
    #[test]
    fn test_eq_usize_with_json_array() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_array = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = 42;
        let number = rug_fuzz_0;
        let value_array = Value::Array(vec![Value::Number(rug_fuzz_1.into())]);
        debug_assert_eq!(number.eq(& value_array), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_array = 0;
    }
    #[test]
    fn test_eq_usize_with_json_object() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_object = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = "number";
        let rug_fuzz_2 = 42;
        let number = rug_fuzz_0;
        let mut value_map = Map::new();
        value_map.insert(rug_fuzz_1.to_string(), Value::Number(rug_fuzz_2.into()));
        let value_object = Value::Object(value_map);
        debug_assert_eq!(number.eq(& value_object), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_json_object = 0;
    }
    #[test]
    fn test_eq_usize_with_disparate_json_number() {
        let _rug_st_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_disparate_json_number = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = 0;
        let number = rug_fuzz_0;
        let value_number = Number::from(rug_fuzz_1);
        debug_assert_eq!(number.eq(& Value::Number(value_number)), false);
        let _rug_ed_tests_llm_16_789_rrrruuuugggg_test_eq_usize_with_disparate_json_number = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_790 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_eq_bool_true() {
        let _rug_st_tests_llm_16_790_rrrruuuugggg_test_eq_bool_true = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = true;
        debug_assert!(eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
        let _rug_ed_tests_llm_16_790_rrrruuuugggg_test_eq_bool_true = 0;
    }
    #[test]
    fn test_eq_bool_false() {
        let _rug_st_tests_llm_16_790_rrrruuuugggg_test_eq_bool_false = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = false;
        debug_assert!(eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
        let _rug_ed_tests_llm_16_790_rrrruuuugggg_test_eq_bool_false = 0;
    }
    #[test]
    fn test_eq_bool_true_with_false() {
        let _rug_st_tests_llm_16_790_rrrruuuugggg_test_eq_bool_true_with_false = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert!(! eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
        let _rug_ed_tests_llm_16_790_rrrruuuugggg_test_eq_bool_true_with_false = 0;
    }
    #[test]
    fn test_eq_bool_false_with_true() {
        let _rug_st_tests_llm_16_790_rrrruuuugggg_test_eq_bool_false_with_true = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        debug_assert!(! eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
        let _rug_ed_tests_llm_16_790_rrrruuuugggg_test_eq_bool_false_with_true = 0;
    }
    #[test]
    fn test_eq_bool_with_non_bool() {
        let _rug_st_tests_llm_16_790_rrrruuuugggg_test_eq_bool_with_non_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = true;
        let rug_fuzz_4 = "true";
        let rug_fuzz_5 = true;
        let rug_fuzz_6 = false;
        let rug_fuzz_7 = true;
        debug_assert!(! eq_bool(& Value::Null, rug_fuzz_0));
        debug_assert!(! eq_bool(& Value::Null, rug_fuzz_1));
        debug_assert!(
            ! eq_bool(& Value::Number(crate ::Number::from(rug_fuzz_2)), rug_fuzz_3)
        );
        debug_assert!(! eq_bool(& Value::String(rug_fuzz_4.to_owned()), rug_fuzz_5));
        debug_assert!(! eq_bool(& Value::Array(Vec::new()), rug_fuzz_6));
        debug_assert!(! eq_bool(& Value::Object(crate ::Map::new()), rug_fuzz_7));
        let _rug_ed_tests_llm_16_790_rrrruuuugggg_test_eq_bool_with_non_bool = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_791 {
    use crate::{
        value::{self, Value},
        number::Number,
    };
    #[test]
    fn test_eq_f32_with_number() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_number = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 3.14_f32;
        let n = Number::from_f32(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        debug_assert!(value::partial_eq::eq_f32(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_number = 0;
    }
    #[test]
    fn test_eq_f32_with_different_number() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_different_number = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 1.59_f32;
        let n = Number::from_f32(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_different_number = 0;
    }
    #[test]
    fn test_eq_f32_with_integer() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_integer = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 10.0_f32;
        let value = Value::Number(rug_fuzz_0.into());
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_integer = 0;
    }
    #[test]
    fn test_eq_f32_with_non_number() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_non_number = 0;
        let rug_fuzz_0 = "3.14";
        let rug_fuzz_1 = 3.14_f32;
        let value = Value::String(rug_fuzz_0.to_owned());
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_non_number = 0;
    }
    #[test]
    fn test_eq_f32_with_null() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_null = 0;
        let rug_fuzz_0 = 3.14_f32;
        let value = Value::Null;
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_null = 0;
    }
    #[test]
    fn test_eq_f32_with_bool() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 1.0_f32;
        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_bool = 0;
    }
    #[test]
    fn test_eq_f32_with_array() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_array = 0;
        let rug_fuzz_0 = 3.14_f32;
        let value = Value::Array(vec![]);
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_array = 0;
    }
    #[test]
    fn test_eq_f32_with_object() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_object = 0;
        let rug_fuzz_0 = 3.14_f32;
        let value = Value::Object(crate::map::Map::new());
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_object = 0;
    }
    #[test]
    fn test_eq_f32_with_zero() {
        let _rug_st_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0_f32;
        let n = Number::from_f32(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        debug_assert!(value::partial_eq::eq_f32(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_791_rrrruuuugggg_test_eq_f32_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_792 {
    use crate::{value::Value, Number, value::partial_eq::eq_f64};
    #[test]
    fn eq_f64_null_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_null_returns_false = 0;
        let rug_fuzz_0 = 0.0;
        let value = Value::Null;
        debug_assert!(! eq_f64(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_null_returns_false = 0;
    }
    #[test]
    fn eq_f64_bool_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_bool_returns_false = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 0.0;
        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_bool_returns_false = 0;
    }
    #[test]
    fn eq_f64_number_integer_returns_true() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_number_integer_returns_true = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 5.0;
        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_number_integer_returns_true = 0;
    }
    #[test]
    fn eq_f64_number_integer_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_number_integer_returns_false = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 5.1;
        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_number_integer_returns_false = 0;
    }
    #[test]
    fn eq_f64_number_float_returns_true() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_number_float_returns_true = 0;
        let rug_fuzz_0 = 5.5;
        let rug_fuzz_1 = 5.5;
        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_number_float_returns_true = 0;
    }
    #[test]
    fn eq_f64_number_float_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_number_float_returns_false = 0;
        let rug_fuzz_0 = 5.5;
        let rug_fuzz_1 = 5.6;
        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_number_float_returns_false = 0;
    }
    #[test]
    fn eq_f64_string_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_string_returns_false = 0;
        let rug_fuzz_0 = "5.0";
        let rug_fuzz_1 = 5.0;
        let value = Value::String(rug_fuzz_0.to_string());
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_string_returns_false = 0;
    }
    #[test]
    fn eq_f64_empty_array_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_empty_array_returns_false = 0;
        let rug_fuzz_0 = 0.0;
        let value = Value::Array(vec![]);
        debug_assert!(! eq_f64(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_empty_array_returns_false = 0;
    }
    #[test]
    fn eq_f64_non_empty_array_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_non_empty_array_returns_false = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1.0;
        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_non_empty_array_returns_false = 0;
    }
    #[test]
    fn eq_f64_empty_object_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_empty_object_returns_false = 0;
        let rug_fuzz_0 = 0.0;
        let value = Value::Object(crate::Map::new());
        debug_assert!(! eq_f64(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_empty_object_returns_false = 0;
    }
    #[test]
    fn eq_f64_non_empty_object_returns_false() {
        let _rug_st_tests_llm_16_792_rrrruuuugggg_eq_f64_non_empty_object_returns_false = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 2.0;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(Number::from(rug_fuzz_1)));
        let value = Value::Object(map);
        debug_assert!(! eq_f64(& value, rug_fuzz_2));
        let _rug_ed_tests_llm_16_792_rrrruuuugggg_eq_f64_non_empty_object_returns_false = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_793_llm_16_793 {
    use crate::{value::Value, value::partial_eq::eq_i64, number::Number};
    #[test]
    fn test_eq_i64_null() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_null = 0;
        let rug_fuzz_0 = 42;
        let v = Value::Null;
        debug_assert!(! eq_i64(& v, rug_fuzz_0));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_null = 0;
    }
    #[test]
    fn test_eq_i64_bool() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 1;
        let v = Value::Bool(rug_fuzz_0);
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_bool = 0;
    }
    #[test]
    fn test_eq_i64_number_pos_int() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_number_pos_int = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42;
        let v = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(eq_i64(& v, rug_fuzz_1));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_number_pos_int = 0;
    }
    #[test]
    fn test_eq_i64_number_neg_int() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_number_neg_int = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42;
        let v = Value::Number(Number::from(-rug_fuzz_0));
        debug_assert!(eq_i64(& v, - rug_fuzz_1));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_number_neg_int = 0;
    }
    #[test]
    fn test_eq_i64_number_float() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_number_float = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42;
        let n = Number::from_f64(rug_fuzz_0).unwrap();
        let v = Value::Number(n);
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_number_float = 0;
    }
    #[test]
    fn test_eq_i64_string() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_string = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = 42;
        let v = Value::String(rug_fuzz_0.into());
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_string = 0;
    }
    #[test]
    fn test_eq_i64_array() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_array = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42;
        let v = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_array = 0;
    }
    #[test]
    fn test_eq_i64_object() {
        let _rug_st_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_object = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = 42_i64;
        let rug_fuzz_2 = 42;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.into(), Value::Number(Number::from(rug_fuzz_1)));
        let v = Value::Object(map);
        debug_assert!(! eq_i64(& v, rug_fuzz_2));
        let _rug_ed_tests_llm_16_793_llm_16_793_rrrruuuugggg_test_eq_i64_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_794_llm_16_794 {
    use crate::value::partial_eq::eq_str;
    use crate::Value;
    #[test]
    fn test_eq_str_with_string_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_string_value = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "test";
        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_string_value = 0;
    }
    #[test]
    fn test_eq_str_with_non_string_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_non_string_value = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = "true";
        let value = Value::Bool(rug_fuzz_0);
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_non_string_value = 0;
    }
    #[test]
    fn test_eq_str_with_null_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_null_value = 0;
        let rug_fuzz_0 = "null";
        let value = Value::Null;
        let other = rug_fuzz_0;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_null_value = 0;
    }
    #[test]
    fn test_eq_str_with_different_string_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_different_string_value = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "different";
        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_different_string_value = 0;
    }
    #[test]
    fn test_eq_str_with_empty_string() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_empty_string = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_empty_string = 0;
    }
    #[test]
    fn test_eq_str_with_non_empty_string_and_empty_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_non_empty_string_and_empty_value = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "non-empty";
        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_non_empty_string_and_empty_value = 0;
    }
    #[test]
    fn test_eq_str_with_number_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_number_value = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "42";
        let value = Value::Number(crate::Number::from(rug_fuzz_0));
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_number_value = 0;
    }
    #[test]
    fn test_eq_str_with_array_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_array_value = 0;
        let rug_fuzz_0 = "[]";
        let value = Value::Array(vec![]);
        let other = rug_fuzz_0;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_array_value = 0;
    }
    #[test]
    fn test_eq_str_with_object_value() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_object_value = 0;
        let rug_fuzz_0 = "{}";
        let value = Value::Object(crate::map::Map::new());
        let other = rug_fuzz_0;
        debug_assert!(! eq_str(& value, other));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_eq_str_with_object_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_795_llm_16_795 {
    use crate::value::partial_eq::eq_u64;
    use crate::value::Value;
    use crate::number::Number;
    #[test]
    fn test_eq_u64_with_u64() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_u64 = 0;
        let rug_fuzz_0 = 42_u64;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 43;
        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(eq_u64(& value, rug_fuzz_1));
        debug_assert!(! eq_u64(& value, rug_fuzz_2));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_u64 = 0;
    }
    #[test]
    fn test_eq_u64_with_f64() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42;
        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_f64 = 0;
    }
    #[test]
    fn test_eq_u64_with_negative() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_negative = 0;
        let rug_fuzz_0 = 42_i64;
        let rug_fuzz_1 = 42;
        let value = Value::Number(Number::from(-rug_fuzz_0));
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_negative = 0;
    }
    #[test]
    fn test_eq_u64_with_string() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_string = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = 42;
        let value = Value::String(rug_fuzz_0.into());
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_string = 0;
    }
    #[test]
    fn test_eq_u64_with_bool() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_bool = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 1;
        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_bool = 0;
    }
    #[test]
    fn test_eq_u64_with_null() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_null = 0;
        let rug_fuzz_0 = 0;
        let value = Value::Null;
        debug_assert!(! eq_u64(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_null = 0;
    }
    #[test]
    fn test_eq_u64_with_array() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_array = 0;
        let rug_fuzz_0 = 42_u64;
        let rug_fuzz_1 = 42;
        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_array = 0;
    }
    #[test]
    fn test_eq_u64_with_object() {
        let _rug_st_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_object = 0;
        let rug_fuzz_0 = 42;
        let value = Value::Object(crate::Map::new());
        debug_assert!(! eq_u64(& value, rug_fuzz_0));
        let _rug_ed_tests_llm_16_795_llm_16_795_rrrruuuugggg_test_eq_u64_with_object = 0;
    }
}
