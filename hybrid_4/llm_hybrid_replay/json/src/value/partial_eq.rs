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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(bool, bool, bool, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val_true = Value::Bool(rug_fuzz_0);
        let val_false = Value::Bool(rug_fuzz_1);
        debug_assert!(Value::eq(& val_true, & rug_fuzz_2));
        debug_assert!(! Value::eq(& val_true, & rug_fuzz_3));
        debug_assert!(! Value::eq(& val_false, & rug_fuzz_4));
             }
}
}
}    }
    #[test]
    fn test_eq_bool_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(bool, bool, bool, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val_true = Value::Bool(rug_fuzz_0);
        let val_false = Value::Bool(rug_fuzz_1);
        debug_assert!(! Value::eq(& val_false, & rug_fuzz_2));
        debug_assert!(Value::eq(& val_false, & rug_fuzz_3));
        debug_assert!(! Value::eq(& val_true, & rug_fuzz_4));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_735 {
    use crate::Number;
    use crate::Value;
    #[test]
    fn value_partial_eq_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, i32, bool, i32, bool, f64, bool, i32, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_736 {
    use crate::value::{Value, Number};
    #[test]
    fn test_value_eq_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(f32, f32, f32, f32, f32, f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_737 {
    use crate::value::Value;
    #[test]
    fn eq_with_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_738 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn test_value_eq_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21)) = <(&str, f32, &str, f32, &str, f32, &str, f32, &str, f32, &str, f32, &str, f32, &str, f32, &str, f32, &str, f32, &str, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_742 {
    use crate::{Number, Value};
    #[test]
    fn eq_with_different_types() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value_number = Value::Number(Number::from(rug_fuzz_0));
        let value_null = Value::Null;
        debug_assert_ne!(value_number, value_null);
             }
}
}
}    }
    #[test]
    fn eq_with_number_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert_eq!(value, Value::Number(Number::from(10_i16)));
        debug_assert_ne!(value, Value::Number(Number::from(11_i16)));
             }
}
}
}    }
    #[test]
    fn eq_with_bool_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    fn eq_with_null_type() {
        let _rug_st_tests_llm_16_742_rrrruuuugggg_eq_with_null_type = 0;
        let value = Value::Null;
        let _rug_ed_tests_llm_16_742_rrrruuuugggg_eq_with_null_type = 0;
    }
    #[test]
    fn eq_with_string_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
             }
}
}
}    }
    #[test]
    fn eq_with_array_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
             }
}
}
}    }
    #[test]
    fn eq_with_object_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut object = crate::Map::new();
        object.insert(rug_fuzz_0.to_owned(), Value::Number(Number::from(rug_fuzz_1)));
        let value = Value::Object(object);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_743 {
    use crate::value::Value;
    use std::i16;
    #[test]
    fn test_eq_with_i16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_744_llm_16_744 {
    use crate::value::Value;
    use crate::Number;
    use std::i16;
    #[test]
    fn test_eq_i16_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i16, i32, i16, i32, i16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
    #[test]
    fn test_eq_i16_with_non_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i16, &str, i16, bool, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let null = Value::Null;
        debug_assert!(! Value::eq(& null, & rug_fuzz_0));
        let string = Value::String(rug_fuzz_1.to_string());
        debug_assert!(! Value::eq(& string, & rug_fuzz_2));
        let boolean = Value::Bool(rug_fuzz_3);
        debug_assert!(! Value::eq(& boolean, & rug_fuzz_4));
             }
}
}
}    }
    #[test]
    fn test_eq_i16_with_different_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i16, i16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let different = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(! Value::eq(& different, & rug_fuzz_1));
        let out_of_range = Value::Number(Number::from(rug_fuzz_2));
        debug_assert!(! Value::eq(& out_of_range, & i16::MAX));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_745 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_eq_with_i32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_num = Value::Number(Number::from(rug_fuzz_0));
        let other: i32 = rug_fuzz_1;
        debug_assert_eq!(Value::eq(& value_num, & other), true);
        let value_num_negative = Value::Number(Number::from(-rug_fuzz_2));
        let other_negative: i32 = -rug_fuzz_3;
        debug_assert_eq!(Value::eq(& value_num_negative, & other_negative), true);
        let other_different: i32 = rug_fuzz_4;
        debug_assert_eq!(Value::eq(& value_num, & other_different), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_746 {
    use crate::value::Value;
    #[test]
    fn value_eq_integer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, i32, i32, i32, i32, i32, &str, i32, i32, i32, i32, bool, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_747_llm_16_747 {
    use crate::{Number, Value};
    #[test]
    fn test_eq_with_integers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(v.eq(& rug_fuzz_1));
        debug_assert!(! v.eq(& rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_eq_with_floating_point() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(v.eq(& rug_fuzz_1));
        debug_assert!(! v.eq(& rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_eq_with_non_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::String(rug_fuzz_0.into());
        debug_assert!(! v.eq(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! v.eq(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_owned(), Value::Number(Number::from(rug_fuzz_1)));
        let v = Value::Object(map);
        debug_assert!(! v.eq(& rug_fuzz_2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_748 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_value_eq_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_749 {
    use crate::value::Value;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v1 = Value::Number(rug_fuzz_0.into());
        let v2 = Value::Number(rug_fuzz_1.into());
        debug_assert_eq!(& v1, & 42);
        let v3 = Value::Number(rug_fuzz_2.into());
        debug_assert_ne!(& v2, & 100);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_750 {
    use crate::Value;
    #[test]
    fn test_eq_number_and_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = Value::Number(rug_fuzz_0.into());
        debug_assert!(num == rug_fuzz_1);
        debug_assert!(num != rug_fuzz_2);
             }
}
}
}    }
    #[test]
    fn test_eq_string_and_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string = Value::String(rug_fuzz_0.to_string());
        debug_assert!(string != rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_eq_array_and_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = Value::Array(vec![Value::Number(rug_fuzz_0.into())]);
        debug_assert!(array != rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_eq_object_and_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut obj = crate::Map::new();
        obj.insert(rug_fuzz_0.to_string(), Value::Number(rug_fuzz_1.into()));
        let object = Value::Object(obj);
        debug_assert!(object != rug_fuzz_2);
             }
}
}
}    }
    #[test]
    fn test_eq_bool_and_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let boolean = Value::Bool(rug_fuzz_0);
        debug_assert!(boolean != rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_eq_null_and_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let null = Value::Null;
        debug_assert!(null != rug_fuzz_0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_752 {
    use crate::{json, Value};
    #[test]
    fn eq_i8_with_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i8, i32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_num = Value::Number(rug_fuzz_0.into());
        let num_i8: i8 = rug_fuzz_1;
        let non_matching_value_num = Value::Number(rug_fuzz_2.into());
        let non_matching_num_i8: i8 = rug_fuzz_3;
        debug_assert!(value_num == num_i8);
        debug_assert!(non_matching_value_num != num_i8);
        debug_assert!(value_num != non_matching_num_i8);
             }
}
}
}    }
    #[test]
    fn eq_str_with_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_str = Value::String(String::from(rug_fuzz_0));
        let str_slice: &str = rug_fuzz_1;
        let non_matching_value_str = Value::String(String::from(rug_fuzz_2));
        let non_matching_str_slice: &str = rug_fuzz_3;
        debug_assert!(value_str == str_slice);
        debug_assert!(non_matching_value_str != str_slice);
        debug_assert!(value_str != non_matching_str_slice);
             }
}
}
}    }
    #[test]
    fn eq_value_with_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i8, i32, i8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num_i8: i8 = rug_fuzz_0;
        let value_num = Value::Number(rug_fuzz_1.into());
        let non_matching_num_i8: i8 = rug_fuzz_2;
        let non_matching_value_num = Value::Number(rug_fuzz_3.into());
        debug_assert!(num_i8 == value_num);
        debug_assert!(non_matching_num_i8 != value_num);
        debug_assert!(num_i8 != non_matching_value_num);
             }
}
}
}    }
    #[test]
    fn eq_value_with_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let str_slice: &str = rug_fuzz_0;
        let value_str = Value::String(String::from(rug_fuzz_1));
        let non_matching_str_slice: &str = rug_fuzz_2;
        let non_matching_value_str = Value::String(String::from(rug_fuzz_3));
        debug_assert!(str_slice == value_str);
        debug_assert!(non_matching_str_slice != value_str);
        debug_assert!(str_slice != non_matching_value_str);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_753 {
    use crate::Value;
    #[test]
    fn test_eq_i8_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let num = Value::Number(i8_val.into());
        debug_assert!(num.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_negative_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = -rug_fuzz_0;
        let num = Value::Number(i8_val.into());
        debug_assert!(num.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_incorrect_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let str_val = Value::String(rug_fuzz_1.to_string());
        debug_assert!(! str_val.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_number_out_of_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let i64_val = Value::Number((i8_val as i64 + i8::max_value() as i64).into());
        debug_assert!(! i64_val.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let null = Value::Null;
        debug_assert!(! null.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let tru = Value::Bool(rug_fuzz_1);
        debug_assert!(! tru.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let arr = Value::Array(vec![Value::Number(i8_val.into())]);
        debug_assert!(! arr.eq(& i8_val));
             }
}
}
}    }
    #[test]
    fn test_eq_i8_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i8_val: i8 = rug_fuzz_0;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_1.to_string(), Value::Number(i8_val.into()));
        let obj = Value::Object(map);
        debug_assert!(! obj.eq(& i8_val));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_755 {
    use crate::value::Value;
    use std::str::FromStr;
    #[test]
    fn test_eq_number_and_isize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = Value::Number(crate::Number::from_str(rug_fuzz_0).unwrap());
        debug_assert_eq!(num, 42isize);
        debug_assert_ne!(num, - 42isize);
             }
}
}
}    }
    #[test]
    fn test_eq_string_and_isize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s = Value::String(rug_fuzz_0.to_string());
        debug_assert_ne!(s, 42isize);
             }
}
}
}    }
    #[test]
    fn test_eq_bool_and_isize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let b = Value::Bool(rug_fuzz_0);
        debug_assert_ne!(b, 42isize);
             }
}
}
}    }
    #[test]
    fn test_eq_array_and_isize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let arr = Value::Array(vec![Value::Number(crate ::Number::from(rug_fuzz_0))]);
        debug_assert_ne!(arr, 42isize);
             }
}
}
}    }
    #[test]
    fn test_eq_object_and_isize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut m = crate::Map::new();
        m.insert(rug_fuzz_0.to_string(), Value::Number(crate::Number::from(rug_fuzz_1)));
        let obj = Value::Object(m);
        debug_assert_ne!(obj, 42isize);
             }
}
}
}    }
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u64, f64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_760 {
    use super::*;
    use crate::*;
    use crate::{json, Value};
    #[test]
    fn eq_u16_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_number = Value::Number(num.into());
        debug_assert!(value_number.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_string = Value::String(num.to_string());
        debug_assert!(! value_string.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_bool = Value::Bool(rug_fuzz_1);
        debug_assert!(! value_bool.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_null = Value::Null;
        debug_assert!(! value_null.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_array = json!([]);
        debug_assert!(! value_array.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_object = json!({});
        debug_assert!(! value_object.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_same_number_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_number = json!(rug_fuzz_1);
        debug_assert!(value_number.eq(& num));
             }
}
}
}    }
    #[test]
    fn eq_u16_with_different_number_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u16 = rug_fuzz_0;
        let value_number = json!(rug_fuzz_1);
        debug_assert!(! value_number.eq(& num));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_761 {
    use crate::{Number, Value};
    #[test]
    fn value_eq_u16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u16, u16, bool, &str, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_762 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn test_value_eq_uint() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u64, i64, f64, bool, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val_number = Value::Number(Number::from(rug_fuzz_0));
        let val_number_neg = Value::Number(Number::from(-rug_fuzz_1));
        let val_number_float = Value::Number(Number::from_f64(rug_fuzz_2).unwrap());
        let val_not_number = Value::Bool(rug_fuzz_3);
        let uint = rug_fuzz_4;
        debug_assert_eq!(val_number.eq(& uint), true);
        debug_assert_eq!(val_number_neg.eq(& uint), false);
        debug_assert_eq!(val_number_float.eq(& uint), false);
        debug_assert_eq!(val_not_number.eq(& uint), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_763 {
    use crate::{json, Value};
    #[test]
    fn test_eq_u32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_765 {
    use crate::value::{Value, from_value};
    #[test]
    fn eq_u64_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_766 {
    use crate::Value;
    #[test]
    fn test_eq_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_u64 = rug_fuzz_0;
        let value_json_number = Value::Number(value_u64.into());
        debug_assert_eq!(& value_json_number, & value_u64);
        debug_assert_eq!(& Value::Null, & 0u64);
             }
}
}
}    }
    #[test]
    fn test_not_eq_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_u64 = rug_fuzz_0;
        let value_json_string = Value::String(rug_fuzz_1.to_string());
        debug_assert_ne!(& value_json_string, & value_u64);
             }
}
}
}    }
    #[test]
    fn test_eq_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_i64 = -rug_fuzz_0;
        let value_json_number = Value::Number(value_i64.into());
        debug_assert_eq!(& value_json_number, & value_i64);
             }
}
}
}    }
    #[test]
    fn test_not_eq_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_i64 = -rug_fuzz_0;
        let value_json_string = Value::String(rug_fuzz_1.to_string());
        debug_assert_ne!(& value_json_string, & value_i64);
             }
}
}
}    }
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, &str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let u8_val = rug_fuzz_0;
        let number_val = Value::Number(Number::from(u8_val));
        let other_val = Value::String(String::from(rug_fuzz_1));
        debug_assert!(< Value as PartialEq < u8 > > ::eq(& number_val, & u8_val));
        debug_assert!(
            ! < Value as PartialEq < u8 > > ::eq(& number_val, & (u8_val + rug_fuzz_2))
        );
        debug_assert!(! < Value as PartialEq < u8 > > ::eq(& other_val, & u8_val));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_770 {
    use crate::value::Value;
    #[test]
    fn test_eq_null_and_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let null_value = Value::Null;
        debug_assert_eq!(null_value == Value::from(rug_fuzz_0), false);
             }
}
}
}    }
    #[test]
    fn test_eq_bool_and_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bool_value = Value::Bool(rug_fuzz_0);
        debug_assert_eq!(bool_value == Value::from(rug_fuzz_1), false);
             }
}
}
}    }
    #[test]
    fn test_eq_number_and_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number_value = Value::Number(rug_fuzz_0.into());
        debug_assert_eq!(number_value == Value::from(rug_fuzz_1), true);
             }
}
}
}    }
    #[test]
    fn test_eq_string_and_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = Value::String(String::from(rug_fuzz_0));
        debug_assert_eq!(string_value == Value::from(rug_fuzz_1), false);
             }
}
}
}    }
    #[test]
    fn test_eq_array_and_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array_value = Value::Array(vec![Value::from(rug_fuzz_0)]);
        debug_assert_eq!(array_value == Value::from(rug_fuzz_1), false);
             }
}
}
}    }
    #[test]
    fn test_eq_object_and_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::Map::new();
        map.insert(String::from(rug_fuzz_0), Value::from(rug_fuzz_1));
        let object_value = Value::Object(map);
        debug_assert_eq!(object_value == Value::from(rug_fuzz_2), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_771 {
    use crate::Value;
    #[test]
    fn value_partial_eq_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_772 {
    use crate::{json, Value};
    #[test]
    fn ensure_partial_eq_for_usize_with_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n: usize = rug_fuzz_0;
        let v: Value = json!(rug_fuzz_1);
        debug_assert_eq!(& v, & n);
             }
}
}
}    }
    #[test]
    fn ensure_value_not_partial_eq_with_different_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n: usize = rug_fuzz_0;
        let v: Value = json!(rug_fuzz_1);
        debug_assert_ne!(& v, & n);
             }
}
}
}    }
    #[test]
    fn ensure_value_partial_eq_with_usize_in_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n: usize = rug_fuzz_0;
        let v: Value = json!([rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let array = v.as_array().unwrap();
        debug_assert!(array.iter().any(| value | value == & n));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_773_llm_16_773 {
    use crate::value::{Number, Value};
    #[test]
    fn value_eq_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, usize, usize, &str, &str, &str, bool, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_num = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(Value::eq(& value_num, & rug_fuzz_1));
        debug_assert!(! Value::eq(& value_num, & rug_fuzz_2));
        let value_str = Value::String(rug_fuzz_3.to_string());
        debug_assert!(Value::eq(& value_str, & rug_fuzz_4.to_string()));
        debug_assert!(! Value::eq(& value_str, & rug_fuzz_5.to_string()));
        let value_bool = Value::Bool(rug_fuzz_6);
        debug_assert!(Value::eq(& value_bool, & rug_fuzz_7));
        debug_assert!(! Value::eq(& value_bool, & rug_fuzz_8));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_775 {
    use crate::Value;
    #[test]
    fn eq_bool_true_with_json_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0, Value::Bool(true));
             }
}
}
}    }
    #[test]
    fn eq_bool_true_with_json_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_ne!(rug_fuzz_0, Value::Bool(false));
             }
}
}
}    }
    #[test]
    fn eq_bool_false_with_json_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_ne!(rug_fuzz_0, Value::Bool(true));
             }
}
}
}    }
    #[test]
    fn eq_bool_false_with_json_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0, Value::Bool(false));
             }
}
}
}    }
    #[test]
    fn eq_bool_with_json_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_ne!(rug_fuzz_0, Value::Null);
             }
}
}
}    }
    #[test]
    fn eq_bool_with_json_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_ne!(rug_fuzz_0, Value::Number(1.into()));
             }
}
}
}    }
    #[test]
    fn eq_bool_with_json_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_ne!(rug_fuzz_0, Value::String("false".to_string()));
             }
}
}
}    }
    #[test]
    fn eq_bool_with_json_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_ne!(rug_fuzz_0, Value::Array(vec![Value::Bool(true)]));
             }
}
}
}    }
    #[test]
    fn eq_bool_with_json_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::map::Map;
        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Bool(rug_fuzz_1));
        debug_assert_ne!(rug_fuzz_2, Value::Object(map));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_776_llm_16_776 {
    use crate::value::{Value, Number};
    #[test]
    fn f32_eq_with_json_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f: f32 = rug_fuzz_0;
        let n = Number::from_f32(f).unwrap();
        let v = Value::Number(n);
        debug_assert!(f.eq(& v));
             }
}
}
}    }
    #[test]
    fn f32_eq_with_json_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f: f32 = rug_fuzz_0;
        let v = Value::Null;
        debug_assert!(! f.eq(& v));
             }
}
}
}    }
    #[test]
    fn f32_eq_with_json_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f: f32 = rug_fuzz_0;
        let v = Value::Bool(rug_fuzz_1);
        debug_assert!(f.eq(& v));
             }
}
}
}    }
    #[test]
    fn f32_eq_with_json_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f: f32 = rug_fuzz_0;
        let v = Value::String(rug_fuzz_1.to_owned());
        debug_assert!(! f.eq(& v));
             }
}
}
}    }
    #[test]
    fn f32_eq_with_json_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f: f32 = rug_fuzz_0;
        let v = Value::Array(
            vec![
                Value::Number(Number::from_f32(rug_fuzz_1).unwrap()),
                Value::Number(Number::from_f32(56.78).unwrap())
            ],
        );
        debug_assert!(! f.eq(& v));
             }
}
}
}    }
    #[test]
    fn f32_eq_with_json_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, &str, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f: f32 = rug_fuzz_0;
        let mut map = crate::map::Map::new();
        map.insert(
            rug_fuzz_1.to_string(),
            Value::Number(Number::from_f32(rug_fuzz_2).unwrap()),
        );
        let v = Value::Object(map);
        debug_assert!(! f.eq(& v));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_777 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn eq_with_value_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Null));
             }
}
}
}    }
    #[test]
    fn eq_with_value_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_1)));
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_2)));
             }
}
}
}    }
    #[test]
    fn eq_with_value_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        debug_assert!(num.eq(& Value::Number(Number::from_f64(rug_fuzz_1).unwrap())));
        debug_assert!(! num.eq(& Value::Number(Number::from_f64(rug_fuzz_2).unwrap())));
             }
}
}
}    }
    #[test]
    fn eq_with_value_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::String(rug_fuzz_1.to_string())));
             }
}
}
}    }
    #[test]
    fn eq_with_value_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        let array = vec![Value::Number(Number::from_f64(rug_fuzz_1).unwrap())];
        debug_assert!(! num.eq(& Value::Array(array)));
             }
}
}
}    }
    #[test]
    fn eq_with_value_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, &str, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        let mut map = crate::Map::new();
        map.insert(
            rug_fuzz_1.to_string(),
            Value::Number(Number::from_f64(rug_fuzz_2).unwrap()),
        );
        debug_assert!(! num.eq(& Value::Object(map)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_778 {
    use crate::value::Value;
    #[test]
    fn i16_equal_to_i16_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(rug_fuzz_0.into());
        debug_assert_eq!(rug_fuzz_1.eq(& value), true);
             }
}
}
}    }
    #[test]
    fn i16_not_equal_to_other_value_types() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i16, i16, bool, i16, &str, i16, i32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.eq(& Value::Null), false);
        debug_assert_eq!(rug_fuzz_1.eq(& Value::Bool(rug_fuzz_2)), false);
        debug_assert_eq!(
            rug_fuzz_3.eq(& Value::String(String::from(rug_fuzz_4))), false
        );
        debug_assert_eq!(
            rug_fuzz_5.eq(& Value::Array(vec![Value::Number(rug_fuzz_6.into())])), false
        );
        debug_assert_eq!(rug_fuzz_7.eq(& Value::Object(crate ::Map::new())), false);
             }
}
}
}    }
    #[test]
    fn i16_equal_to_other_numbers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.eq(& Value::Number(rug_fuzz_1.into())), true);
             }
}
}
}    }
    #[test]
    fn i16_not_equal_to_different_numbers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.eq(& Value::Number(rug_fuzz_1.into())), false);
             }
}
}
}    }
    #[test]
    fn i16_not_equal_to_floating_numbers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let float_number = crate::Number::from_f64(rug_fuzz_0).unwrap();
        debug_assert_eq!(rug_fuzz_1.eq(& Value::Number(float_number)), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_779 {
    use crate::value::Value;
    use crate::Number;
    #[test]
    fn eq_with_json_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! i32::eq(& rug_fuzz_0, & Value::Null));
             }
}
}
}    }
    #[test]
    fn eq_with_json_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! i32::eq(& rug_fuzz_0, & Value::Bool(rug_fuzz_1)));
             }
}
}
}    }
    #[test]
    fn eq_with_json_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, i32, i32, f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
    #[test]
    fn eq_with_json_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! i32::eq(& rug_fuzz_0, & Value::String(rug_fuzz_1.to_owned())));
             }
}
}
}    }
    #[test]
    fn eq_with_json_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            ! i32::eq(& rug_fuzz_0, &
            Value::Array(vec![Value::Number(Number::from(rug_fuzz_1))]))
        );
             }
}
}
}    }
    #[test]
    fn eq_with_json_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut object = crate::Map::new();
        object.insert(rug_fuzz_0.to_owned(), Value::Number(Number::from(rug_fuzz_1)));
        debug_assert!(! i32::eq(& rug_fuzz_2, & Value::Object(object)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_780 {
    use crate::Value;
    #[test]
    fn test_eq_with_i64_and_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Value::from(rug_fuzz_0), 42_i64);
        debug_assert_eq!(Value::from(- rug_fuzz_1), - 42_i64);
        debug_assert_ne!(Value::from(rug_fuzz_2), 43_i64);
        debug_assert_ne!(Value::from(- rug_fuzz_3), - 41_i64);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_782_llm_16_782 {
    use crate::{json, Number, Value};
    #[test]
    fn eq_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(value.eq(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_with_positive_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(-rug_fuzz_0));
        debug_assert!(value.eq(& (- rug_fuzz_1)));
             }
}
}
}    }
    #[test]
    fn eq_with_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        debug_assert!(value.eq(& rug_fuzz_1.to_string()));
             }
}
}
}    }
    #[test]
    fn eq_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(value.eq(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Null;
        debug_assert!(! value.eq(& rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn eq_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![json!(rug_fuzz_0), json!(2), json!(3)]);
        debug_assert!(! value.eq(& rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::map::Map::new();
        map.insert(rug_fuzz_0.to_owned(), json!(rug_fuzz_1));
        let value = Value::Object(map);
        debug_assert!(! value.eq(& rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn eq_with_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(value.eq(& rug_fuzz_1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_783 {
    use crate::Value;
    use std::string::String;
    #[test]
    fn test_string_eq_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = String::from(rug_fuzz_0);
        let value_string = Value::String(rug_fuzz_1.to_owned());
        let value_number = Value::Number(rug_fuzz_2.into());
        let value_null = Value::Null;
        debug_assert!(string_value.eq(& value_string));
        debug_assert!(! string_value.eq(& value_number));
        debug_assert!(! string_value.eq(& value_null));
             }
}
}
}    }
    #[test]
    fn test_string_eq_value_with_escape_chars() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = String::from(rug_fuzz_0);
        let value_string = Value::String(rug_fuzz_1.to_owned());
        let value_string_unesc = Value::String(rug_fuzz_2.to_owned());
        debug_assert!(string_value.eq(& value_string));
        debug_assert!(! string_value.eq(& value_string_unesc));
             }
}
}
}    }
    #[test]
    fn test_string_eq_value_with_different_case() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = String::from(rug_fuzz_0);
        let value_string_same_case = Value::String(rug_fuzz_1.to_owned());
        let value_string_diff_case = Value::String(rug_fuzz_2.to_owned());
        debug_assert!(string_value.eq(& value_string_same_case));
        debug_assert!(! string_value.eq(& value_string_diff_case));
             }
}
}
}    }
    #[test]
    fn test_string_eq_value_with_non_string_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = String::from(rug_fuzz_0);
        let value_bool = Value::Bool(rug_fuzz_1);
        let value_object = Value::Object(crate::Map::new());
        debug_assert!(! string_value.eq(& value_bool));
        debug_assert!(! string_value.eq(& value_object));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_784 {
    use crate::Value;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(&str, i32, &str, &str, &str, i32, i32, i32, &str, &str, &str, &str, i32, &str, usize, i32, &str, usize, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_785 {
    use crate::Value;
    use std::u16;
    #[test]
    fn test_eq_with_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u16, u16, &str, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value_number = Value::Number(rug_fuzz_0.into());
        debug_assert_eq!(u16::eq(& rug_fuzz_1, & value_number), true);
        debug_assert_eq!(u16::eq(& u16::MAX, & value_number), false);
        debug_assert_eq!(u16::eq(& rug_fuzz_2, & value_number), false);
        let value_string = Value::String(rug_fuzz_3.to_string());
        debug_assert_eq!(u16::eq(& rug_fuzz_4, & value_string), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_786 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Null));
             }
}
}
}    }
    #[test]
    fn test_eq_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_1)));
        debug_assert!(! num.eq(& Value::Bool(rug_fuzz_2)));
             }
}
}
}    }
    #[test]
    fn test_eq_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        debug_assert!(num.eq(& Value::Number(rug_fuzz_1.into())));
        debug_assert!(! num.eq(& Value::Number(rug_fuzz_2.into())));
        debug_assert!(! num.eq(& Value::Number((- rug_fuzz_3).into())));
             }
}
}
}    }
    #[test]
    fn test_eq_with_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::String(rug_fuzz_1.to_owned())));
             }
}
}
}    }
    #[test]
    fn test_eq_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        debug_assert!(! num.eq(& Value::Array(vec![Value::Number(rug_fuzz_1.into())])));
             }
}
}
}    }
    #[test]
    fn test_eq_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: u32 = rug_fuzz_0;
        let mut map = crate::Map::new();
        map.insert(rug_fuzz_1.to_string(), Value::Number(rug_fuzz_2.into()));
        debug_assert!(! num.eq(& Value::Object(map)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_787_llm_16_787 {
    use crate::Value;
    use crate::Number;
    #[test]
    fn u64_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, &str, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = rug_fuzz_0;
        let value_num = Value::Number(Number::from(num));
        let value_str = Value::String(rug_fuzz_1.to_owned());
        let value_f64 = Value::Number(Number::from_f64(rug_fuzz_2).unwrap());
        let value_null = Value::Null;
        debug_assert_eq!(num.eq(& value_num), true);
        debug_assert_eq!(num.eq(& value_str), false);
        debug_assert_eq!(num.eq(& value_f64), true);
        debug_assert_eq!(num.eq(& value_null), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_788_llm_16_788 {
    use crate::value::Value;
    use crate::number::Number;
    #[test]
    fn test_u8_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_789 {
    use crate::value::{Value, Number};
    use crate::map::Map;
    #[test]
    fn test_eq_usize_with_json_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        debug_assert_eq!(number.eq(& Value::Null), false);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        debug_assert_eq!(number.eq(& Value::Bool(rug_fuzz_1)), false);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_number_positive_int() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let value_number = Number::from(rug_fuzz_1);
        debug_assert_eq!(number.eq(& Value::Number(value_number)), true);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_number_negative_int() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let value_number = Number::from(-rug_fuzz_1);
        debug_assert_eq!(number.eq(& Value::Number(value_number)), false);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_number_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let value_number = Number::from_f64(rug_fuzz_1).unwrap();
        debug_assert_eq!(number.eq(& Value::Number(value_number)), true);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let value_string = Value::String(rug_fuzz_1.to_string());
        debug_assert_eq!(number.eq(& value_string), false);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let value_array = Value::Array(vec![Value::Number(rug_fuzz_1.into())]);
        debug_assert_eq!(number.eq(& value_array), false);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_json_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let mut value_map = Map::new();
        value_map.insert(rug_fuzz_1.to_string(), Value::Number(rug_fuzz_2.into()));
        let value_object = Value::Object(value_map);
        debug_assert_eq!(number.eq(& value_object), false);
             }
}
}
}    }
    #[test]
    fn test_eq_usize_with_disparate_json_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let value_number = Number::from(rug_fuzz_1);
        debug_assert_eq!(number.eq(& Value::Number(value_number)), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_790 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    #[test]
    fn test_eq_bool_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_bool_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_bool_true_with_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_bool_false_with_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! eq_bool(& Value::Bool(rug_fuzz_0), rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_bool_with_non_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(bool, bool, i32, bool, &str, bool, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! eq_bool(& Value::Null, rug_fuzz_0));
        debug_assert!(! eq_bool(& Value::Null, rug_fuzz_1));
        debug_assert!(
            ! eq_bool(& Value::Number(crate ::Number::from(rug_fuzz_2)), rug_fuzz_3)
        );
        debug_assert!(! eq_bool(& Value::String(rug_fuzz_4.to_owned()), rug_fuzz_5));
        debug_assert!(! eq_bool(& Value::Array(Vec::new()), rug_fuzz_6));
        debug_assert!(! eq_bool(& Value::Object(crate ::Map::new()), rug_fuzz_7));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_791 {
    use crate::{
        value::{self, Value},
        number::Number,
    };
    #[test]
    fn test_eq_f32_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n = Number::from_f32(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        debug_assert!(value::partial_eq::eq_f32(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_different_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n = Number::from_f32(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_integer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(rug_fuzz_0.into());
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_non_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Null;
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![]);
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Object(crate::map::Map::new());
        debug_assert!(! value::partial_eq::eq_f32(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_eq_f32_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n = Number::from_f32(rug_fuzz_0).unwrap();
        let value = Value::Number(n);
        debug_assert!(value::partial_eq::eq_f32(& value, rug_fuzz_1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_792 {
    use crate::{value::Value, Number, value::partial_eq::eq_f64};
    #[test]
    fn eq_f64_null_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Null;
        debug_assert!(! eq_f64(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn eq_f64_bool_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_number_integer_returns_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_number_integer_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_number_float_returns_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_number_float_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_string_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_string());
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_empty_array_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![]);
        debug_assert!(! eq_f64(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn eq_f64_non_empty_array_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! eq_f64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn eq_f64_empty_object_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Object(crate::Map::new());
        debug_assert!(! eq_f64(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn eq_f64_non_empty_object_returns_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::Number(Number::from(rug_fuzz_1)));
        let value = Value::Object(map);
        debug_assert!(! eq_f64(& value, rug_fuzz_2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_793_llm_16_793 {
    use crate::{value::Value, value::partial_eq::eq_i64, number::Number};
    #[test]
    fn test_eq_i64_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Null;
        debug_assert!(! eq_i64(& v, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Bool(rug_fuzz_0);
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_number_pos_int() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(eq_i64(& v, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_number_neg_int() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Number(Number::from(-rug_fuzz_0));
        debug_assert!(eq_i64(& v, - rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_number_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let n = Number::from_f64(rug_fuzz_0).unwrap();
        let v = Value::Number(n);
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::String(rug_fuzz_0.into());
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! eq_i64(& v, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_i64_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.into(), Value::Number(Number::from(rug_fuzz_1)));
        let v = Value::Object(map);
        debug_assert!(! eq_i64(& v, rug_fuzz_2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_794_llm_16_794 {
    use crate::value::partial_eq::eq_str;
    use crate::Value;
    #[test]
    fn test_eq_str_with_string_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_non_string_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_null_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Null;
        let other = rug_fuzz_0;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_different_string_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_non_empty_string_and_empty_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.to_owned());
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_number_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(crate::Number::from(rug_fuzz_0));
        let other = rug_fuzz_1;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_array_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![]);
        let other = rug_fuzz_0;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
    #[test]
    fn test_eq_str_with_object_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Object(crate::map::Map::new());
        let other = rug_fuzz_0;
        debug_assert!(! eq_str(& value, other));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_795_llm_16_795 {
    use crate::value::partial_eq::eq_u64;
    use crate::value::Value;
    use crate::number::Number;
    #[test]
    fn test_eq_u64_with_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(eq_u64(& value, rug_fuzz_1));
        debug_assert!(! eq_u64(& value, rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Number(Number::from(-rug_fuzz_0));
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::String(rug_fuzz_0.into());
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Bool(rug_fuzz_0);
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Null;
        debug_assert!(! eq_u64(& value, rug_fuzz_0));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Array(vec![Value::Number(Number::from(rug_fuzz_0))]);
        debug_assert!(! eq_u64(& value, rug_fuzz_1));
             }
}
}
}    }
    #[test]
    fn test_eq_u64_with_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Value::Object(crate::Map::new());
        debug_assert!(! eq_u64(& value, rug_fuzz_0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_258 {
    use super::*;
    use crate::value::Value;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let mut p1: &&str = &&rug_fuzz_1;
        debug_assert!(Value::eq(& p0, p1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_259 {
    use super::*;
    use crate::{value::Value, Value as OtherValue};
    #[test]
    fn test_eq() {
        let _rug_st_tests_rug_259_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = "true";
        let rug_fuzz_1 = true;
        let p0: &'static str = rug_fuzz_0;
        let mut p1: Value = OtherValue::Bool(rug_fuzz_1);
        debug_assert!(p0.eq(& p1));
        let _rug_ed_tests_rug_259_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_rug_260 {
    use crate::value::Value;
    use crate::value::partial_eq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i8 = rug_fuzz_0;
        let p1: Value = Value::Bool(rug_fuzz_1);
        debug_assert!(< i8 as PartialEq < Value > > ::eq(& p0, & p1) == p1.eq(& p0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_261 {
    use crate::Value;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Value::Bool(rug_fuzz_0);
        let p1: i8 = rug_fuzz_1;
        debug_assert!(p0.eq(& p1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_262 {
    use crate::value::Value;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1: isize = rug_fuzz_1;
        debug_assert_eq!(Value::eq(& p0, & p1), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_263 {
    use super::*;
    use crate::value::{self, Value};
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let p1: isize = rug_fuzz_1;
        debug_assert_eq!(p0.eq(& p1), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_264 {
    use crate::value::Value;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut Value = &mut Value::Bool(rug_fuzz_0);
        let p1: &u8 = &rug_fuzz_1;
        debug_assert_eq!(< Value as PartialEq < u8 > > ::eq(p0, p1), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_265 {
    use crate::value::Value;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(Value::eq(& p0, & p1), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_266 {
    use crate::value;
    use crate::Value;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1: u64 = rug_fuzz_1;
        debug_assert_eq!(
            < value::Value as std::cmp::PartialEq < u64 > > ::eq(& p0, & p1), false
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_267 {
    use super::*;
    use crate::value::{self, Value};
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1: f64 = rug_fuzz_1;
        debug_assert_eq!(< Value as PartialEq < f64 > > ::eq(& p0, & p1), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_268 {
    use crate::value::Value;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1: f64 = rug_fuzz_1;
        debug_assert_eq!(p0.eq(& p1), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_269 {
    use crate::Value;
    use crate::value::partial_eq;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Value::Bool(rug_fuzz_0);
        let mut p1: f64 = rug_fuzz_1;
        debug_assert_eq!(p0.eq(& p1), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_270 {
    use crate::{value::Value, Value as OtherValue};
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = OtherValue::Bool(rug_fuzz_0);
        let p1 = rug_fuzz_1;
        debug_assert!(p0.eq(& p1));
             }
}
}
}    }
}
