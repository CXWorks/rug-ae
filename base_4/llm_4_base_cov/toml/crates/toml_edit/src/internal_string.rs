use std::borrow::Borrow;
use std::str::FromStr;
/// Opaque string storage internal to `toml_edit`
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalString(Inner);
#[cfg(feature = "kstring")]
type Inner = kstring::KString;
#[cfg(not(feature = "kstring"))]
type Inner = String;
impl InternalString {
    /// Create an empty string
    pub fn new() -> Self {
        InternalString(Inner::new())
    }
    /// Access the underlying string
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
impl std::fmt::Debug for InternalString {
    #[inline]
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        self.0.fmt(formatter)
    }
}
impl std::ops::Deref for InternalString {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}
impl Borrow<str> for InternalString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl AsRef<str> for InternalString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for InternalString {
    #[inline]
    fn from(s: &str) -> Self {
        #[cfg(feature = "kstring")]
        let inner = kstring::KString::from_ref(s);
        #[cfg(not(feature = "kstring"))]
        let inner = String::from(s);
        InternalString(inner)
    }
}
impl From<String> for InternalString {
    #[inline]
    fn from(s: String) -> Self {
        #[allow(clippy::useless_conversion)] InternalString(s.into())
    }
}
impl From<&String> for InternalString {
    #[inline]
    fn from(s: &String) -> Self {
        InternalString(s.into())
    }
}
impl From<&InternalString> for InternalString {
    #[inline]
    fn from(s: &InternalString) -> Self {
        s.clone()
    }
}
impl From<Box<str>> for InternalString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        InternalString(s.into())
    }
}
impl FromStr for InternalString {
    type Err = core::convert::Infallible;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}
impl std::fmt::Display for InternalString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InternalString {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InternalString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor)
    }
}
#[cfg(feature = "serde")]
struct StringVisitor;
#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = InternalString;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(InternalString::from(v))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(InternalString::from(v))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => Ok(InternalString::from(s)),
            Err(_) => {
                Err(
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Bytes(v),
                        &self,
                    ),
                )
            }
        }
    }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(InternalString::from(s)),
            Err(e) => {
                Err(
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Bytes(&e.into_bytes()),
                        &self,
                    ),
                )
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_43 {
    use crate::InternalString;
    use std::borrow::Borrow;
    #[test]
    fn test_borrow() {
        let _rug_st_tests_llm_16_43_rrrruuuugggg_test_borrow = 0;
        let rug_fuzz_0 = "Hello, world!";
        let original = rug_fuzz_0;
        let internal_string = InternalString::from(original);
        let borrowed: &str = internal_string.borrow();
        debug_assert_eq!(borrowed, original);
        let _rug_ed_tests_llm_16_43_rrrruuuugggg_test_borrow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use crate::InternalString;
    use std::convert::AsRef;
    #[test]
    fn test_internal_string_as_ref() {
        let _rug_st_tests_llm_16_44_rrrruuuugggg_test_internal_string_as_ref = 0;
        let rug_fuzz_0 = "example";
        let intern_string = InternalString::from(rug_fuzz_0);
        let as_ref_str: &str = intern_string.as_ref();
        debug_assert_eq!(as_ref_str, "example");
        let _rug_ed_tests_llm_16_44_rrrruuuugggg_test_internal_string_as_ref = 0;
    }
    #[test]
    fn test_internal_string_as_ref_empty() {
        let _rug_st_tests_llm_16_44_rrrruuuugggg_test_internal_string_as_ref_empty = 0;
        let rug_fuzz_0 = "";
        let intern_string = InternalString::from(rug_fuzz_0);
        let as_ref_str: &str = intern_string.as_ref();
        debug_assert_eq!(as_ref_str, "");
        let _rug_ed_tests_llm_16_44_rrrruuuugggg_test_internal_string_as_ref_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use crate::InternalString;
    use std::convert::From;
    #[test]
    fn test_internal_string_from() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_internal_string_from = 0;
        let rug_fuzz_0 = "test value";
        let orig = InternalString::from(rug_fuzz_0);
        let from_orig = <InternalString as From<&InternalString>>::from(&orig);
        debug_assert_eq!(from_orig.as_str(), orig.as_str());
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_internal_string_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use crate::InternalString;
    use std::string::String;
    #[test]
    fn test_internal_string_from_string_ref() {
        let _rug_st_tests_llm_16_46_rrrruuuugggg_test_internal_string_from_string_ref = 0;
        let rug_fuzz_0 = "Hello, TOML!";
        let original = String::from(rug_fuzz_0);
        let internal_str = InternalString::from(&original);
        debug_assert_eq!(internal_str.as_str(), "Hello, TOML!");
        let _rug_ed_tests_llm_16_46_rrrruuuugggg_test_internal_string_from_string_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use crate::InternalString;
    #[test]
    fn test_internal_string_from_str() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_internal_string_from_str = 0;
        let rug_fuzz_0 = "test string";
        let test_str = rug_fuzz_0;
        let internal_string = InternalString::from(test_str);
        debug_assert_eq!(test_str, internal_string.as_str());
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_internal_string_from_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_48 {
    use crate::InternalString;
    #[test]
    fn from_boxed_str() {
        let _rug_st_tests_llm_16_48_rrrruuuugggg_from_boxed_str = 0;
        let rug_fuzz_0 = "hello world";
        let boxed_str = rug_fuzz_0.to_string().into_boxed_str();
        let internal_string = InternalString::from(boxed_str.clone());
        debug_assert_eq!(internal_string.as_str(), & * boxed_str);
        let _rug_ed_tests_llm_16_48_rrrruuuugggg_from_boxed_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use crate::InternalString;
    use std::string::String;
    #[test]
    fn test_from_string_to_internal_string() {
        let _rug_st_tests_llm_16_49_rrrruuuugggg_test_from_string_to_internal_string = 0;
        let rug_fuzz_0 = "test content";
        let test_string = String::from(rug_fuzz_0);
        let internal_string: InternalString = InternalString::from(test_string.clone());
        debug_assert_eq!(internal_string.as_str(), test_string.as_str());
        let _rug_ed_tests_llm_16_49_rrrruuuugggg_test_from_string_to_internal_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_50_llm_16_50 {
    use crate::internal_string::InternalString;
    use std::ops::Deref;
    #[test]
    fn deref_returns_correct_str() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_deref_returns_correct_str = 0;
        let rug_fuzz_0 = "Test string";
        let original_str = rug_fuzz_0;
        let internal_str = InternalString::from(original_str);
        debug_assert_eq!(& * internal_str, original_str);
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_deref_returns_correct_str = 0;
    }
    #[test]
    fn deref_maintains_equality() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_deref_maintains_equality = 0;
        let rug_fuzz_0 = "Another test";
        let original_str = rug_fuzz_0;
        let internal_str = InternalString::from(original_str);
        let deref_str: &str = internal_str.deref();
        debug_assert_eq!(deref_str, original_str);
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_deref_maintains_equality = 0;
    }
    #[test]
    fn deref_with_different_strings() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_deref_with_different_strings = 0;
        let rug_fuzz_0 = "String one";
        let rug_fuzz_1 = "String two";
        let original_str1 = rug_fuzz_0;
        let original_str2 = rug_fuzz_1;
        let internal_str1 = InternalString::from(original_str1);
        let internal_str2 = InternalString::from(original_str2);
        debug_assert_ne!(& * internal_str1, & * internal_str2);
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_deref_with_different_strings = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_51 {
    use crate::InternalString;
    use std::str::FromStr;
    #[test]
    fn test_from_str() {
        let _rug_st_tests_llm_16_51_rrrruuuugggg_test_from_str = 0;
        let rug_fuzz_0 = "A test string";
        let test_str = rug_fuzz_0;
        let internal_str = InternalString::from_str(test_str).unwrap();
        debug_assert_eq!(internal_str.as_str(), test_str);
        let _rug_ed_tests_llm_16_51_rrrruuuugggg_test_from_str = 0;
    }
    #[test]
    fn test_from_str_empty() {
        let _rug_st_tests_llm_16_51_rrrruuuugggg_test_from_str_empty = 0;
        let rug_fuzz_0 = "";
        let test_str = rug_fuzz_0;
        let internal_str = InternalString::from_str(test_str).unwrap();
        debug_assert_eq!(internal_str.as_str(), test_str);
        let _rug_ed_tests_llm_16_51_rrrruuuugggg_test_from_str_empty = 0;
    }
    #[test]
    fn test_from_str_special_chars() {
        let _rug_st_tests_llm_16_51_rrrruuuugggg_test_from_str_special_chars = 0;
        let rug_fuzz_0 = "特殊字符@!#&*()";
        let test_str = rug_fuzz_0;
        let internal_str = InternalString::from_str(test_str).unwrap();
        debug_assert_eq!(internal_str.as_str(), test_str);
        let _rug_ed_tests_llm_16_51_rrrruuuugggg_test_from_str_special_chars = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_258 {
    use crate::InternalString;
    use std::convert::From;
    #[test]
    fn test_as_str_empty() {
        let _rug_st_tests_llm_16_258_rrrruuuugggg_test_as_str_empty = 0;
        let internal_str = InternalString::new();
        debug_assert_eq!(internal_str.as_str(), "");
        let _rug_ed_tests_llm_16_258_rrrruuuugggg_test_as_str_empty = 0;
    }
    #[test]
    fn test_as_str_from_str() {
        let _rug_st_tests_llm_16_258_rrrruuuugggg_test_as_str_from_str = 0;
        let rug_fuzz_0 = "test_str";
        let internal_str = InternalString::from(rug_fuzz_0);
        debug_assert_eq!(internal_str.as_str(), "test_str");
        let _rug_ed_tests_llm_16_258_rrrruuuugggg_test_as_str_from_str = 0;
    }
    #[test]
    fn test_as_str_from_string() {
        let _rug_st_tests_llm_16_258_rrrruuuugggg_test_as_str_from_string = 0;
        let rug_fuzz_0 = "test_string";
        let s = String::from(rug_fuzz_0);
        let internal_str = InternalString::from(s);
        debug_assert_eq!(internal_str.as_str(), "test_string");
        let _rug_ed_tests_llm_16_258_rrrruuuugggg_test_as_str_from_string = 0;
    }
    #[test]
    fn test_as_str_clone() {
        let _rug_st_tests_llm_16_258_rrrruuuugggg_test_as_str_clone = 0;
        let rug_fuzz_0 = "test_clone";
        let internal_str = InternalString::from(rug_fuzz_0);
        let internal_str_clone = internal_str.clone();
        debug_assert_eq!(internal_str_clone.as_str(), "test_clone");
        let _rug_ed_tests_llm_16_258_rrrruuuugggg_test_as_str_clone = 0;
    }
    #[test]
    fn test_as_str_deref() {
        let _rug_st_tests_llm_16_258_rrrruuuugggg_test_as_str_deref = 0;
        let rug_fuzz_0 = "test_deref";
        let internal_str = InternalString::from(rug_fuzz_0);
        debug_assert_eq!(internal_str.as_str(), & * internal_str);
        let _rug_ed_tests_llm_16_258_rrrruuuugggg_test_as_str_deref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_259_llm_16_259 {
    use crate::InternalString;
    use std::str::FromStr;
    #[test]
    fn test_internal_string_new() {
        let _rug_st_tests_llm_16_259_llm_16_259_rrrruuuugggg_test_internal_string_new = 0;
        let empty: InternalString = InternalString::new();
        debug_assert_eq!(empty.as_str(), "");
        debug_assert_eq!(empty, InternalString::from(""));
        debug_assert_eq!(empty, InternalString::from(String::new()));
        debug_assert_eq!(empty, InternalString::from(""));
        debug_assert_eq!(empty, InternalString::from(String::new()));
        debug_assert_eq!(empty, InternalString::from_str("").unwrap());
        let _rug_ed_tests_llm_16_259_llm_16_259_rrrruuuugggg_test_internal_string_new = 0;
    }
}
