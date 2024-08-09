//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents
//! into Rust structures. Note that some top-level functions here are also
//! provided at the top of the crate.
/// Deserializes a string into a type.
///
/// This function will attempt to interpret `s` as a TOML document and
/// deserialize `T` from the document.
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     title: String,
///     owner: Owner,
/// }
///
/// #[derive(Deserialize)]
/// struct Owner {
///     name: String,
/// }
///
/// let config: Config = toml::from_str(r#"
///     title = 'TOML Example'
///
///     [owner]
///     name = 'Lisa'
/// "#).unwrap();
///
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// ```
#[cfg(feature = "parse")]
pub fn from_str<T>(s: &'_ str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(Deserializer::new(s))
}
/// Errors that can occur when deserializing a type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    inner: crate::edit::de::Error,
}
impl Error {
    fn new(inner: crate::edit::de::Error) -> Self {
        Self { inner }
    }
    pub(crate) fn add_key(&mut self, key: String) {
        self.inner.add_key(key)
    }
    /// What went wrong
    pub fn message(&self) -> &str {
        self.inner.message()
    }
    /// The start/end index into the original document where the error occurred
    #[cfg(feature = "parse")]
    pub fn span(&self) -> Option<std::ops::Range<usize>> {
        self.inner.span()
    }
}
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::new(crate::edit::de::Error::custom(msg))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
impl std::error::Error for Error {}
/// Deserialization TOML document
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
#[cfg(feature = "parse")]
pub struct Deserializer<'a> {
    input: &'a str,
}
#[cfg(feature = "parse")]
impl<'a> Deserializer<'a> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}
#[cfg(feature = "parse")]
impl<'de, 'a> serde::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::Deserializer>()
            .map_err(Error::new)?;
        inner.deserialize_any(visitor).map_err(Error::new)
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::Deserializer>()
            .map_err(Error::new)?;
        inner.deserialize_option(visitor).map_err(Error::new)
    }
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::Deserializer>()
            .map_err(Error::new)?;
        inner.deserialize_newtype_struct(name, visitor).map_err(Error::new)
    }
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::Deserializer>()
            .map_err(Error::new)?;
        inner.deserialize_struct(name, fields, visitor).map_err(Error::new)
    }
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::Deserializer>()
            .map_err(Error::new)?;
        inner.deserialize_enum(name, variants, visitor).map_err(Error::new)
    }
    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq bytes byte_buf map
        unit ignored_any unit_struct tuple_struct tuple identifier
    }
}
/// Deserialization TOML [value][crate::Value]
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     title: String,
///     owner: Owner,
/// }
///
/// #[derive(Deserialize)]
/// struct Owner {
///     name: String,
/// }
///
/// let config = Config::deserialize(toml::de::ValueDeserializer::new(
///     r#"{ title = 'TOML Example', owner = { name = 'Lisa' } }"#
/// )).unwrap();
///
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// ```
#[cfg(feature = "parse")]
pub struct ValueDeserializer<'a> {
    input: &'a str,
}
#[cfg(feature = "parse")]
impl<'a> ValueDeserializer<'a> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}
#[cfg(feature = "parse")]
impl<'de, 'a> serde::Deserializer<'de> for ValueDeserializer<'a> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_any(visitor).map_err(Error::new)
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_option(visitor).map_err(Error::new)
    }
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_newtype_struct(name, visitor).map_err(Error::new)
    }
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_struct(name, fields, visitor).map_err(Error::new)
    }
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_enum(name, variants, visitor).map_err(Error::new)
    }
    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq bytes byte_buf map
        unit ignored_any unit_struct tuple_struct tuple identifier
    }
}
#[cfg(test)]
mod tests_llm_16_21_llm_16_21 {
    use super::*;
    use crate::*;
    use crate::de::Deserializer;
    use crate::de::Error;
    use serde::{Deserialize, Deserializer as SerdeDeserializer};
    use serde::de::{self, Visitor, EnumAccess};
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(crate = "serde")]
    enum TestEnum {
        A,
        B,
    }
    #[test]
    fn test_deserialize_enum() {
        let input = "type = \"A\"";
        let mut deserializer = Deserializer::new(input);
        let result = deserializer
            .deserialize_enum("TestEnum", &["A", "B"], TestEnumVisitor);
        assert_eq!(result, Ok(TestEnum::A));
    }
    struct TestEnumVisitor;
    impl<'de> Visitor<'de> for TestEnumVisitor {
        type Value = TestEnum;
        fn expecting(
            &self,
            formatter: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(formatter, "a TestEnum variant")
        }
        fn visit_enum<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: EnumAccess<'de>,
        {
            let (variant, _variant_access) = access.variant()?;
            match variant {
                TestEnum::A => Ok(TestEnum::A),
                TestEnum::B => Ok(TestEnum::B),
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_22_llm_16_22 {
    use super::*;
    use crate::*;
    use serde::de::{Deserializer as _, Error, Visitor};
    use crate::de::Deserializer;
    use crate::map::Map;
    use crate::value::Value;
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Map<String, Value>;
        fn expecting(
            &self,
            formatter: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Map::new())
        }
        fn visit_map<M>(self, mut visitor: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_newtype_struct() {
        let _rug_st_tests_llm_16_22_llm_16_22_rrrruuuugggg_test_deserialize_newtype_struct = 0;
        let rug_fuzz_0 = "key = 'value'";
        let rug_fuzz_1 = "Test";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "key";
        let toml_str = rug_fuzz_0;
        let mut deserializer = Deserializer::new(toml_str);
        let visitor = TestVisitor;
        let result: Result<Map<String, Value>, crate::de::Error> = deserializer
            .deserialize_newtype_struct(rug_fuzz_1, visitor);
        debug_assert!(result.is_ok());
        let map = result.unwrap();
        debug_assert!(map.contains_key(rug_fuzz_2));
        debug_assert_eq!(
            map.get(rug_fuzz_3), Some(& Value::String("value".to_string()))
        );
        let _rug_ed_tests_llm_16_22_llm_16_22_rrrruuuugggg_test_deserialize_newtype_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_25_llm_16_25 {
    use super::*;
    use crate::*;
    use serde::de::Error as SerdeError;
    struct Displayable;
    impl std::fmt::Display for Displayable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "custom error message")
        }
    }
    #[test]
    fn test_custom_error() {
        let _rug_st_tests_llm_16_25_llm_16_25_rrrruuuugggg_test_custom_error = 0;
        let displayable = Displayable;
        let error: de::Error = SerdeError::custom(displayable);
        debug_assert_eq!(error.message(), "custom error message");
        let _rug_ed_tests_llm_16_25_llm_16_25_rrrruuuugggg_test_custom_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_26_llm_16_26 {
    use super::*;
    use crate::*;
    use crate::value::Value;
    use serde::de::{self, Deserialize};
    use std::collections::BTreeMap as Map;
    use std::fmt;
    use crate::de::{Error, ValueDeserializer};
    use serde::Deserializer;
    struct ValueVisitor;
    impl<'de> serde::de::Visitor<'de> for ValueVisitor {
        type Value = Map<String, Value>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a map")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Map::new())
        }
        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            let mut values = Map::new();
            while let Some((key, value)) = visitor.next_entry()? {
                values.insert(key, value);
            }
            Ok(values)
        }
    }
    #[test]
    fn test_deserialize_any() {
        let _rug_st_tests_llm_16_26_llm_16_26_rrrruuuugggg_test_deserialize_any = 0;
        let rug_fuzz_0 = r#"{ "key1": "value1", "key2": 2 }"#;
        let rug_fuzz_1 = "key1";
        let rug_fuzz_2 = "key2";
        let toml_str = rug_fuzz_0;
        let deserializer = ValueDeserializer::new(toml_str);
        let visitor = ValueVisitor;
        let result: Result<Map<String, Value>, crate::de::Error> = deserializer
            .deserialize_any(visitor);
        debug_assert!(result.is_ok());
        let map = result.unwrap();
        debug_assert_eq!(map.len(), 2);
        debug_assert_eq!(map[rug_fuzz_1], Value::String("value1".to_owned()));
        debug_assert_eq!(map[rug_fuzz_2], Value::Integer(2));
        let _rug_ed_tests_llm_16_26_llm_16_26_rrrruuuugggg_test_deserialize_any = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_268 {
    use super::*;
    use crate::*;
    use serde::Deserialize;
    use std::marker::PhantomData;
    #[derive(Debug, Deserialize)]
    struct Dummy<'a> {
        #[serde(skip)]
        _marker: PhantomData<&'a ()>,
    }
    #[test]
    fn test_new_deserializer() {
        let _rug_st_tests_llm_16_268_rrrruuuugggg_test_new_deserializer = 0;
        let rug_fuzz_0 = "key = \"value\"";
        let input = rug_fuzz_0;
        let deserializer = Deserializer::new(input);
        let dummy: Result<Dummy, _> = Deserialize::deserialize(deserializer);
        debug_assert!(dummy.is_ok());
        let _rug_ed_tests_llm_16_268_rrrruuuugggg_test_new_deserializer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_269_llm_16_269 {
    use super::*;
    use crate::*;
    use serde::de::Error as SerdeError;
    #[test]
    fn test_add_key() {
        let _rug_st_tests_llm_16_269_llm_16_269_rrrruuuugggg_test_add_key = 0;
        let rug_fuzz_0 = "initial error";
        let rug_fuzz_1 = "extra_info";
        let mut error = Error::custom(rug_fuzz_0);
        let initial_message = error.message().to_string();
        let key = rug_fuzz_1.to_string();
        error.add_key(key.clone());
        let updated_message = error.message();
        debug_assert!(updated_message.contains(& key));
        debug_assert_ne!(initial_message, updated_message);
        let _rug_ed_tests_llm_16_269_llm_16_269_rrrruuuugggg_test_add_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_270 {
    use super::*;
    use crate::*;
    use serde::de::value::Error as ValueError;
    use serde::de::Error as SerdeError;
    use std::fmt;
    #[test]
    fn test_message() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_test_message = 0;
        struct CustomError;
        impl fmt::Display for CustomError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "custom error message")
            }
        }
        let custom_error = CustomError;
        let error = Error::custom(custom_error);
        debug_assert_eq!(error.message(), "custom error message");
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_test_message = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_273_llm_16_273 {
    use crate::de::ValueDeserializer;
    use serde::Deserialize;
    #[derive(Deserialize, PartialEq, Debug)]
    struct Config {
        title: String,
        owner: Owner,
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct Owner {
        name: String,
    }
    #[test]
    fn test_value_deserializer_new() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_test_value_deserializer_new = 0;
        let rug_fuzz_0 = r#"
            title = 'TOML Example'
            [owner]
            name = 'Lisa'
        "#;
        let rug_fuzz_1 = "TOML Example";
        let rug_fuzz_2 = "Lisa";
        let toml_str = rug_fuzz_0;
        let deserializer = ValueDeserializer::new(toml_str);
        let config: Config = serde::Deserialize::deserialize(deserializer).unwrap();
        let expected = Config {
            title: String::from(rug_fuzz_1),
            owner: Owner {
                name: String::from(rug_fuzz_2),
            },
        };
        debug_assert_eq!(config, expected);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_test_value_deserializer_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_274 {
    use crate::from_str;
    use serde::Deserialize;
    use std::result::Result;
    #[derive(Deserialize, PartialEq, Debug)]
    struct TestConfig {
        key: String,
        value: i32,
    }
    #[test]
    fn test_from_str_valid_toml() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_from_str_valid_toml = 0;
        let rug_fuzz_0 = r#"
            key = "example"
            value = 42
        "#;
        let toml_str = rug_fuzz_0;
        let parsed: Result<TestConfig, _> = from_str(toml_str);
        debug_assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        debug_assert_eq!(parsed, TestConfig { key : "example".to_owned(), value : 42, });
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_from_str_valid_toml = 0;
    }
    #[test]
    fn test_from_str_invalid_toml() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_from_str_invalid_toml = 0;
        let rug_fuzz_0 = r#"
            key = "example"
            value = "not a number"
        "#;
        let toml_str = rug_fuzz_0;
        let parsed: Result<TestConfig, _> = from_str(toml_str);
        debug_assert!(parsed.is_err());
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_from_str_invalid_toml = 0;
    }
    #[test]
    fn test_from_str_missing_keys() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_from_str_missing_keys = 0;
        let rug_fuzz_0 = r#"
            value = 42
        "#;
        let toml_str = rug_fuzz_0;
        let parsed: Result<TestConfig, _> = from_str(toml_str);
        debug_assert!(parsed.is_err());
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_from_str_missing_keys = 0;
    }
}
