use std::fmt;
use serde::de;
use serde::ser;
use crate::map::Map;
use crate::Value;
/// Type representing a TOML table, payload of the `Value::Table` variant.
/// By default it is backed by a BTreeMap, enable the `preserve_order` feature
/// to use a LinkedHashMap instead.
pub type Table = Map<String, Value>;
impl Table {
    /// Convert a `T` into `toml::Table`.
    ///
    /// This conversion can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    pub fn try_from<T>(value: T) -> Result<Self, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(crate::value::TableSerializer)
    }
    /// Interpret a `toml::Table` as an instance of type `T`.
    ///
    /// This conversion can fail if the structure of the `Table` does not match the structure
    /// expected by `T`, for example if `T` is a bool which can't be mapped to a `Table`. It can
    /// also fail if the structure is correct but `T`'s implementation of `Deserialize` decides
    /// that something is wrong with the data, for example required struct fields are missing from
    /// the TOML map or some number is too big to fit in the expected primitive type.
    pub fn try_into<'de, T>(self) -> Result<T, crate::de::Error>
    where
        T: de::Deserialize<'de>,
    {
        de::Deserialize::deserialize(self)
    }
}
#[cfg(feature = "display")]
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::ser::to_string(self).expect("Unable to represent value as string").fmt(f)
    }
}
#[cfg(feature = "parse")]
impl std::str::FromStr for Table {
    type Err = crate::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::from_str(s)
    }
}
impl<'de> de::Deserializer<'de> for Table {
    type Error = crate::de::Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_any(visitor)
    }
    #[inline]
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_enum(name, variants, visitor)
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_option(visitor)
    }
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_newtype_struct(name, visitor)
    }
    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq bytes
        byte_buf map unit_struct tuple_struct struct tuple ignored_any identifier
    }
}
impl<'de> de::IntoDeserializer<'de, crate::de::Error> for Table {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}
#[cfg(test)]
mod tests_llm_16_321 {
    use super::*;
    use crate::*;
    use serde::Deserialize;
    use std::string::String;
    use crate::value::Value;
    #[derive(Deserialize, PartialEq, Debug)]
    struct TestStruct {
        key1: String,
        key2: u64,
    }
    #[test]
    fn test_try_into_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Map::new();
        table.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        table.insert(rug_fuzz_2.to_string(), Value::Integer(rug_fuzz_3));
        let result: Result<TestStruct, crate::de::Error> = table.try_into();
        debug_assert!(result.is_ok());
        let test_struct = result.unwrap();
        debug_assert_eq!(
            test_struct, TestStruct { key1 : "value1".to_string(), key2 : 42, }
        );
             }
}
}
}    }
    #[test]
    fn test_try_into_failure_missing_fields() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Map::new();
        table.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        let result: Result<TestStruct, crate::de::Error> = table.try_into();
        debug_assert!(result.is_err());
             }
}
}
}    }
    #[test]
    fn test_try_into_failure_invalid_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut table = Map::new();
        table.insert(rug_fuzz_0.to_string(), Value::String(rug_fuzz_1.to_string()));
        table.insert(rug_fuzz_2.to_string(), Value::String(rug_fuzz_3.to_string()));
        let result: Result<TestStruct, crate::de::Error> = table.try_into();
        debug_assert!(result.is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_326_llm_16_326 {
    use serde::de::IntoDeserializer;
    use crate::map::Map;
    use crate::value::Value;
    #[test]
    fn test_into_deserializer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = Map::new();
        map.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        map.insert(rug_fuzz_2.to_string(), Value::from(rug_fuzz_3));
        let deserializer = map.clone().into_deserializer();
        debug_assert_eq!(map, deserializer);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_327 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use std::str::FromStr;
    use crate::map::Map;
    #[test]
    fn test_from_str_valid() {
        let _rug_st_tests_llm_16_327_rrrruuuugggg_test_from_str_valid = 0;
        let rug_fuzz_0 = r#"
            [section]
            key = "value"
        "#;
        let rug_fuzz_1 = "section";
        let rug_fuzz_2 = "section";
        let rug_fuzz_3 = "key";
        let toml_str = rug_fuzz_0;
        let map = Map::<String, Value>::from_str(toml_str);
        debug_assert!(map.is_ok());
        let map = map.unwrap();
        debug_assert!(map.contains_key(rug_fuzz_1));
        if let Some(&Value::Table(ref section)) = map.get(rug_fuzz_2) {
            debug_assert_eq!(
                section.get(rug_fuzz_3), Some(& Value::String("value".to_string()))
            );
        } else {
            panic!("section key is not a table")
        }
        let _rug_ed_tests_llm_16_327_rrrruuuugggg_test_from_str_valid = 0;
    }
    #[test]
    fn test_from_str_invalid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toml_str = rug_fuzz_0;
        let map = Map::<String, Value>::from_str(toml_str);
        debug_assert!(map.is_err());
             }
}
}
}    }
}
