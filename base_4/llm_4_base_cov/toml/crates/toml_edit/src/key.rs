use std::borrow::Cow;
use std::str::FromStr;
use crate::encode::{to_string_repr, StringStyle};
use crate::parser;
use crate::parser::key::is_unquoted_char;
use crate::repr::{Decor, Repr};
use crate::InternalString;
/// Key as part of a Key/Value Pair or a table header.
///
/// # Examples
///
/// ```notrust
/// [dependencies."nom"]
/// version = "5.0"
/// 'literal key' = "nonsense"
/// "basic string key" = 42
/// ```
///
/// There are 3 types of keys:
///
/// 1. Bare keys (`version` and `dependencies`)
///
/// 2. Basic quoted keys (`"basic string key"` and `"nom"`)
///
/// 3. Literal quoted keys (`'literal key'`)
///
/// For details see [toml spec](https://github.com/toml-lang/toml/#keyvalue-pair).
///
/// To parse a key use `FromStr` trait implementation: `"string".parse::<Key>()`.
#[derive(Debug, Clone)]
pub struct Key {
    key: InternalString,
    pub(crate) repr: Option<Repr>,
    pub(crate) decor: Decor,
}
impl Key {
    /// Create a new table key
    pub fn new(key: impl Into<InternalString>) -> Self {
        Self {
            key: key.into(),
            repr: None,
            decor: Default::default(),
        }
    }
    /// Parse a TOML key expression
    ///
    /// Unlike `"".parse<Key>()`, this supports dotted keys.
    pub fn parse(repr: &str) -> Result<Vec<Self>, crate::TomlError> {
        Self::try_parse_path(repr)
    }
    pub(crate) fn with_repr_unchecked(mut self, repr: Repr) -> Self {
        self.repr = Some(repr);
        self
    }
    /// While creating the `Key`, add `Decor` to it
    pub fn with_decor(mut self, decor: Decor) -> Self {
        self.decor = decor;
        self
    }
    /// Access a mutable proxy for the `Key`.
    pub fn as_mut(&mut self) -> KeyMut<'_> {
        KeyMut { key: self }
    }
    /// Returns the parsed key value.
    pub fn get(&self) -> &str {
        &self.key
    }
    pub(crate) fn get_internal(&self) -> &InternalString {
        &self.key
    }
    /// Returns key raw representation, if available.
    pub fn as_repr(&self) -> Option<&Repr> {
        self.repr.as_ref()
    }
    /// Returns the default raw representation.
    pub fn default_repr(&self) -> Repr {
        to_key_repr(&self.key)
    }
    /// Returns a raw representation.
    pub fn display_repr(&self) -> Cow<'_, str> {
        self.as_repr()
            .and_then(|r| r.as_raw().as_str())
            .map(Cow::Borrowed)
            .unwrap_or_else(|| {
                Cow::Owned(self.default_repr().as_raw().as_str().unwrap().to_owned())
            })
    }
    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }
    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        &self.decor
    }
    /// Returns the location within the original document
    #[cfg(feature = "serde")]
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.repr.as_ref().and_then(|r| r.span())
    }
    pub(crate) fn despan(&mut self, input: &str) {
        self.decor.despan(input);
        if let Some(repr) = &mut self.repr {
            repr.despan(input)
        }
    }
    /// Auto formats the key.
    pub fn fmt(&mut self) {
        self.repr = Some(to_key_repr(&self.key));
        self.decor.clear();
    }
    fn try_parse_simple(s: &str) -> Result<Key, crate::TomlError> {
        let mut key = parser::parse_key(s)?;
        key.despan(s);
        Ok(key)
    }
    fn try_parse_path(s: &str) -> Result<Vec<Key>, crate::TomlError> {
        let mut keys = parser::parse_key_path(s)?;
        for key in &mut keys {
            key.despan(s);
        }
        Ok(keys)
    }
}
impl std::ops::Deref for Key {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(other.get())
    }
}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Key {}
impl PartialEq for Key {
    #[inline]
    fn eq(&self, other: &Key) -> bool {
        PartialEq::eq(self.get(), other.get())
    }
}
impl PartialEq<str> for Key {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.get(), other)
    }
}
impl<'s> PartialEq<&'s str> for Key {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.get(), *other)
    }
}
impl PartialEq<String> for Key {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.get(), other.as_str())
    }
}
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, None, ("", ""))
    }
}
impl FromStr for Key {
    type Err = crate::TomlError;
    /// Tries to parse a key from a &str,
    /// if fails, tries as basic quoted key (surrounds with "")
    /// and then literal quoted key (surrounds with '')
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Key::try_parse_simple(s)
    }
}
fn to_key_repr(key: &str) -> Repr {
    if key.as_bytes().iter().copied().all(is_unquoted_char) && !key.is_empty() {
        Repr::new_unchecked(key)
    } else {
        to_string_repr(key, Some(StringStyle::OnelineSingle), Some(false))
    }
}
impl<'b> From<&'b str> for Key {
    fn from(s: &'b str) -> Self {
        Key::new(s)
    }
}
impl<'b> From<&'b String> for Key {
    fn from(s: &'b String) -> Self {
        Key::new(s)
    }
}
impl From<String> for Key {
    fn from(s: String) -> Self {
        Key::new(s)
    }
}
impl From<InternalString> for Key {
    fn from(s: InternalString) -> Self {
        Key::new(s)
    }
}
#[doc(hidden)]
impl From<Key> for InternalString {
    fn from(key: Key) -> InternalString {
        key.key
    }
}
/// A mutable reference to a `Key`
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct KeyMut<'k> {
    key: &'k mut Key,
}
impl<'k> KeyMut<'k> {
    /// Returns the parsed key value.
    pub fn get(&self) -> &str {
        self.key.get()
    }
    /// Returns the raw representation, if available.
    pub fn as_repr(&self) -> Option<&Repr> {
        self.key.as_repr()
    }
    /// Returns the default raw representation.
    pub fn default_repr(&self) -> Repr {
        self.key.default_repr()
    }
    /// Returns a raw representation.
    pub fn display_repr(&self) -> Cow<str> {
        self.key.display_repr()
    }
    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        self.key.decor_mut()
    }
    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        self.key.decor()
    }
    /// Auto formats the key.
    pub fn fmt(&mut self) {
        self.key.fmt()
    }
}
impl<'k> std::ops::Deref for KeyMut<'k> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
impl<'s> PartialEq<str> for KeyMut<'s> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.get(), other)
    }
}
impl<'s> PartialEq<&'s str> for KeyMut<'s> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.get(), *other)
    }
}
impl<'s> PartialEq<String> for KeyMut<'s> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.get(), other.as_str())
    }
}
impl<'k> std::fmt::Display for KeyMut<'k> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.key, f)
    }
}
#[cfg(test)]
mod tests_llm_16_55 {
    use crate::key::Key;
    use std::cmp::Ordering;
    #[test]
    fn test_key_cmp() {
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_key_cmp = 0;
        let rug_fuzz_0 = "alpha";
        let rug_fuzz_1 = "beta";
        let rug_fuzz_2 = "alpha";
        let rug_fuzz_3 = "Alpha";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from(rug_fuzz_1);
        let key3 = Key::from(rug_fuzz_2);
        let key4 = Key::from(rug_fuzz_3);
        debug_assert_eq!(key1.cmp(& key2), Ordering::Less);
        debug_assert_eq!(key2.cmp(& key1), Ordering::Greater);
        debug_assert_eq!(key1.cmp(& key3), Ordering::Equal);
        debug_assert_eq!(key1.cmp(& key4), Ordering::Greater);
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_key_cmp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_56 {
    use crate::key::Key;
    use std::cmp::PartialEq;
    #[test]
    fn test_key_eq_str() {
        let _rug_st_tests_llm_16_56_rrrruuuugggg_test_key_eq_str = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = "example";
        let rug_fuzz_2 = "not-example";
        let key = Key::from(rug_fuzz_0);
        let key_str: &str = rug_fuzz_1;
        debug_assert!(key.eq(& key_str));
        let non_matching_str: &str = rug_fuzz_2;
        debug_assert!(! key.eq(& non_matching_str));
        let _rug_ed_tests_llm_16_56_rrrruuuugggg_test_key_eq_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_57 {
    use crate::key::Key;
    use std::string::String;
    #[test]
    fn test_key_eq_string() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_key_eq_string = 0;
        let rug_fuzz_0 = "my_key";
        let key_string = String::from(rug_fuzz_0);
        let key = Key::from(key_string.clone());
        debug_assert!(key.eq(& key_string));
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_key_eq_string = 0;
    }
    #[test]
    fn test_key_eq_string_ref() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_key_eq_string_ref = 0;
        let rug_fuzz_0 = "my_key";
        let rug_fuzz_1 = "my_key";
        let key = Key::from(rug_fuzz_0);
        let key_string = rug_fuzz_1.to_string();
        debug_assert!(key.eq(& key_string));
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_key_eq_string_ref = 0;
    }
    #[test]
    fn test_key_not_eq_string() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_key_not_eq_string = 0;
        let rug_fuzz_0 = "my_key";
        let rug_fuzz_1 = "other_key";
        let key = Key::from(rug_fuzz_0);
        let other_key_string = rug_fuzz_1.to_string();
        debug_assert!(! key.eq(& other_key_string));
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_key_not_eq_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_58 {
    use crate::key::Key;
    #[test]
    fn test_eq_with_str() {
        let _rug_st_tests_llm_16_58_rrrruuuugggg_test_eq_with_str = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "Failed to parse key from str";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "key";
        let rug_fuzz_4 = "key";
        let key_from_str: Key = rug_fuzz_0.parse().expect(rug_fuzz_1);
        let key_from_string: Key = Key::from(rug_fuzz_2.to_string());
        debug_assert!(key_from_str.eq(rug_fuzz_3), "Key from str should equal 'key'");
        debug_assert!(
            key_from_string.eq(rug_fuzz_4), "Key from String should equal 'key'"
        );
        let _rug_ed_tests_llm_16_58_rrrruuuugggg_test_eq_with_str = 0;
    }
    #[test]
    fn test_eq_with_different_str() {
        let _rug_st_tests_llm_16_58_rrrruuuugggg_test_eq_with_different_str = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "Failed to parse key from str";
        let rug_fuzz_2 = "other";
        let key_from_str: Key = rug_fuzz_0.parse().expect(rug_fuzz_1);
        debug_assert!(
            ! key_from_str.eq(rug_fuzz_2), "Key from str should not equal 'other'"
        );
        let _rug_ed_tests_llm_16_58_rrrruuuugggg_test_eq_with_different_str = 0;
    }
    #[test]
    fn test_eq_with_empty_str() {
        let _rug_st_tests_llm_16_58_rrrruuuugggg_test_eq_with_empty_str = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "Failed to parse key from str";
        let rug_fuzz_2 = "";
        let key_from_str: Key = rug_fuzz_0.parse().expect(rug_fuzz_1);
        debug_assert!(
            ! key_from_str.eq(rug_fuzz_2),
            "Key from str should not equal an empty string"
        );
        let _rug_ed_tests_llm_16_58_rrrruuuugggg_test_eq_with_empty_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_59_llm_16_59 {
    use crate::key::Key;
    use crate::key::Decor;
    use std::borrow::Borrow;
    use std::str::FromStr;
    #[test]
    fn test_key_eq_same_key() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_same_key = 0;
        let rug_fuzz_0 = "same-key";
        let rug_fuzz_1 = "same-key";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from(rug_fuzz_1);
        debug_assert!(key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_same_key = 0;
    }
    #[test]
    fn test_key_eq_different_keys() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_different_keys = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "key2";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from(rug_fuzz_1);
        debug_assert!(! key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_different_keys = 0;
    }
    #[test]
    fn test_key_eq_with_decor() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_decor = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = " ";
        let rug_fuzz_2 = " ";
        let rug_fuzz_3 = "key";
        let key1 = Key::from(rug_fuzz_0).with_decor(Decor::new(rug_fuzz_1, rug_fuzz_2));
        let key2 = Key::from(rug_fuzz_3);
        debug_assert!(key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_decor = 0;
    }
    #[test]
    fn test_key_eq_with_different_decor() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_different_decor = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = " ";
        let rug_fuzz_2 = " ";
        let rug_fuzz_3 = "key";
        let rug_fuzz_4 = "";
        let rug_fuzz_5 = "  ";
        let key1 = Key::from(rug_fuzz_0).with_decor(Decor::new(rug_fuzz_1, rug_fuzz_2));
        let key2 = Key::from(rug_fuzz_3).with_decor(Decor::new(rug_fuzz_4, rug_fuzz_5));
        debug_assert!(key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_different_decor = 0;
    }
    #[test]
    fn test_key_eq_with_same_decor() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_same_decor = 0;
        let rug_fuzz_0 = " ";
        let rug_fuzz_1 = " ";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "key";
        let decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        let key1 = Key::from(rug_fuzz_2).with_decor(decor.clone());
        let key2 = Key::from(rug_fuzz_3).with_decor(decor);
        debug_assert!(key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_same_decor = 0;
    }
    #[test]
    fn test_key_eq_with_different_repr() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_different_repr = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "key";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from_str(rug_fuzz_1).unwrap();
        debug_assert!(key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_with_different_repr = 0;
    }
    #[test]
    #[should_panic]
    fn test_key_eq_panic_on_different_keys() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_panic_on_different_keys = 0;
        let rug_fuzz_0 = "key1";
        let rug_fuzz_1 = "key2";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from(rug_fuzz_1);
        debug_assert!(key1.eq(& key2));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_panic_on_different_keys = 0;
    }
    #[test]
    fn test_key_eq_variant_forms() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_variant_forms = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "key";
        let rug_fuzz_3 = "key";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from_str(rug_fuzz_1).unwrap();
        let key3 = Key::from(String::from(rug_fuzz_2));
        let key4 = Key::from(rug_fuzz_3);
        debug_assert!(key1.eq(& key2));
        debug_assert!(key2.eq(& key3));
        debug_assert!(key3.eq(& key4));
        debug_assert!(key4.eq(& key1));
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_key_eq_variant_forms = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_60 {
    use crate::key::Key;
    use std::cmp::Ordering;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_llm_16_60_rrrruuuugggg_test_partial_cmp = 0;
        let rug_fuzz_0 = "alpha";
        let rug_fuzz_1 = "beta";
        let rug_fuzz_2 = "alpha";
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from(rug_fuzz_1);
        let key3 = Key::from(rug_fuzz_2);
        debug_assert_eq!(key1.partial_cmp(& key2), Some(Ordering::Less));
        debug_assert_eq!(key2.partial_cmp(& key1), Some(Ordering::Greater));
        debug_assert_eq!(key1.partial_cmp(& key3), Some(Ordering::Equal));
        let _rug_ed_tests_llm_16_60_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_61 {
    use crate::key::Key;
    use std::convert::From;
    #[test]
    fn test_key_from_string_reference() {
        let _rug_st_tests_llm_16_61_rrrruuuugggg_test_key_from_string_reference = 0;
        let rug_fuzz_0 = "test_key";
        let test_string = rug_fuzz_0.to_string();
        let key_from_string = Key::from(&test_string);
        debug_assert_eq!(key_from_string.get(), test_string);
        let _rug_ed_tests_llm_16_61_rrrruuuugggg_test_key_from_string_reference = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_62 {
    use crate::Key;
    #[test]
    fn key_from_str() {
        let _rug_st_tests_llm_16_62_rrrruuuugggg_key_from_str = 0;
        let rug_fuzz_0 = "example-key";
        let key_str = rug_fuzz_0;
        let key: Key = key_str.into();
        debug_assert_eq!(key.get(), key_str);
        let _rug_ed_tests_llm_16_62_rrrruuuugggg_key_from_str = 0;
    }
    #[test]
    fn key_from_string() {
        let _rug_st_tests_llm_16_62_rrrruuuugggg_key_from_string = 0;
        let rug_fuzz_0 = "example-key";
        let key_string = rug_fuzz_0.to_string();
        let key: Key = (&key_string).into();
        debug_assert_eq!(key.get(), key_string);
        let _rug_ed_tests_llm_16_62_rrrruuuugggg_key_from_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_63 {
    use crate::Key;
    #[test]
    fn test_from_internal_string_to_key() {
        let _rug_st_tests_llm_16_63_rrrruuuugggg_test_from_internal_string_to_key = 0;
        let rug_fuzz_0 = "example_key";
        use crate::internal_string::InternalString;
        let raw_key = rug_fuzz_0;
        let internal_string = InternalString::from(raw_key);
        let key = Key::from(internal_string.clone());
        debug_assert_eq!(key.get(), internal_string.as_str());
        let _rug_ed_tests_llm_16_63_rrrruuuugggg_test_from_internal_string_to_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_64 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn test_from_string_to_key() {
        let _rug_st_tests_llm_16_64_rrrruuuugggg_test_from_string_to_key = 0;
        let rug_fuzz_0 = "test_key";
        let test_string = rug_fuzz_0.to_string();
        let key = Key::from(test_string.clone());
        debug_assert_eq!(* key, test_string);
        let _rug_ed_tests_llm_16_64_rrrruuuugggg_test_from_string_to_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;
    use crate::*;
    use std::hash::{Hash, Hasher};
    #[test]
    fn test_key_hash() {
        let _rug_st_tests_llm_16_65_rrrruuuugggg_test_key_hash = 0;
        let rug_fuzz_0 = "test_key";
        let rug_fuzz_1 = "test_key";
        let rug_fuzz_2 = "different_key";
        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        let key1 = Key::from(rug_fuzz_0);
        let key2 = Key::from(rug_fuzz_1);
        let key3 = Key::from(rug_fuzz_2);
        key1.hash(&mut hasher1);
        key2.hash(&mut hasher2);
        debug_assert_eq!(hasher1.finish(), hasher2.finish());
        hasher1 = std::collections::hash_map::DefaultHasher::new();
        key3.hash(&mut hasher1);
        debug_assert_ne!(hasher1.finish(), hasher2.finish());
        let _rug_ed_tests_llm_16_65_rrrruuuugggg_test_key_hash = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_66 {
    use crate::Key;
    use std::ops::Deref;
    #[test]
    fn deref_key() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_deref_key = 0;
        let rug_fuzz_0 = "example-key";
        let key_string = rug_fuzz_0.to_string();
        let key: Key = Key::from(key_string.clone());
        let deref_str: &str = key.deref();
        debug_assert_eq!(deref_str, key_string);
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_deref_key = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid_simple_key() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_valid_simple_key = 0;
        let rug_fuzz_0 = "valid_key";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_ok());
        debug_assert_eq!(key.unwrap().get(), key_str);
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_valid_simple_key = 0;
    }
    #[test]
    fn test_from_str_valid_quoted_key() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_valid_quoted_key = 0;
        let rug_fuzz_0 = "\"valid_key\"";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_ok());
        debug_assert_eq!(key.unwrap().get(), key_str.trim_matches('\"'));
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_valid_quoted_key = 0;
    }
    #[test]
    fn test_from_str_empty_key() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_empty_key = 0;
        let rug_fuzz_0 = "";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_err());
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_empty_key = 0;
    }
    #[test]
    fn test_from_str_invalid_characters() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_invalid_characters = 0;
        let rug_fuzz_0 = "invalid key";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_err());
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_invalid_characters = 0;
    }
    #[test]
    fn test_from_str_valid_quoted_with_invalid_characters() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_valid_quoted_with_invalid_characters = 0;
        let rug_fuzz_0 = "\"invalid key\"";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_ok());
        debug_assert_eq!(key.unwrap().get(), key_str.trim_matches('\"'));
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_valid_quoted_with_invalid_characters = 0;
    }
    #[test]
    fn test_from_str_single_quoted_key() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_single_quoted_key = 0;
        let rug_fuzz_0 = "'valid_key'";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_ok());
        debug_assert_eq!(key.unwrap().get(), key_str.trim_matches('\''));
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_single_quoted_key = 0;
    }
    #[test]
    fn test_from_str_single_quoted_with_invalid_characters() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_from_str_single_quoted_with_invalid_characters = 0;
        let rug_fuzz_0 = "'invalid key'";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str);
        debug_assert!(key.is_ok());
        debug_assert_eq!(key.unwrap().get(), key_str.trim_matches('\''));
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_from_str_single_quoted_with_invalid_characters = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_68_llm_16_68 {
    use crate::{Key, KeyMut};
    use std::ops::Deref;
    #[test]
    fn deref_for_key_mut() {
        let _rug_st_tests_llm_16_68_llm_16_68_rrrruuuugggg_deref_for_key_mut = 0;
        let rug_fuzz_0 = "some_key";
        let mut key = Key::new(rug_fuzz_0);
        let key_mut = key.as_mut();
        let deref_value: &str = key_mut.deref();
        debug_assert_eq!(deref_value, "some_key");
        let _rug_ed_tests_llm_16_68_llm_16_68_rrrruuuugggg_deref_for_key_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;
    use crate::*;
    use std::borrow::Borrow;
    #[test]
    fn key_mut_eq_str() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_key_mut_eq_str = 0;
        let rug_fuzz_0 = "example";
        let mut key = Key::from(rug_fuzz_0);
        let key_mut = key.as_mut();
        let key_str: &str = key_mut.borrow();
        debug_assert!(key_mut.eq(& key_str));
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_key_mut_eq_str = 0;
    }
    #[test]
    fn key_mut_neq_str() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_key_mut_neq_str = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = "other";
        let mut key = Key::from(rug_fuzz_0);
        let key_mut = key.as_mut();
        debug_assert!(! key_mut.eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_key_mut_neq_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_70 {
    use super::*;
    use crate::*;
    #[test]
    fn eq_with_string() {
        let _rug_st_tests_llm_16_70_rrrruuuugggg_eq_with_string = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = "example";
        let rug_fuzz_2 = "different";
        let mut key = Key::new(rug_fuzz_0);
        let key_mut = key.as_mut();
        let string = rug_fuzz_1.to_string();
        debug_assert!(key_mut.eq(& string));
        let string_different = rug_fuzz_2.to_string();
        debug_assert!(! key_mut.eq(& string_different));
        let _rug_ed_tests_llm_16_70_rrrruuuugggg_eq_with_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;
    use crate::*;
    use crate::key::{Key, KeyMut};
    #[test]
    fn test_key_mut_eq_str() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_test_key_mut_eq_str = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "other_key";
        let mut key = Key::new(rug_fuzz_0);
        let mut key_mut = key.as_mut();
        debug_assert!(key_mut.eq(rug_fuzz_1));
        debug_assert!(! key_mut.eq(rug_fuzz_2));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_test_key_mut_eq_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_303 {
    use crate::{InternalString, Key};
    #[test]
    fn test_from_key_for_internal_string() {
        let _rug_st_tests_llm_16_303_rrrruuuugggg_test_from_key_for_internal_string = 0;
        let rug_fuzz_0 = "test_key";
        let key_str = rug_fuzz_0;
        let key = Key::from(key_str);
        let internal_string: InternalString = InternalString::from(key);
        debug_assert_eq!(key_str, internal_string.as_str());
        let _rug_ed_tests_llm_16_303_rrrruuuugggg_test_from_key_for_internal_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_304 {
    use crate::{Key, KeyMut};
    #[test]
    fn test_key_as_mut() {
        let _rug_st_tests_llm_16_304_rrrruuuugggg_test_key_as_mut = 0;
        let rug_fuzz_0 = "key_name";
        let mut key = Key::new(rug_fuzz_0);
        let mut key_mut = key.as_mut();
        key_mut.fmt();
        debug_assert_eq!(key_mut.get(), "key_name");
        debug_assert_eq!(& * key_mut, "key_name");
        let _rug_ed_tests_llm_16_304_rrrruuuugggg_test_key_as_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_306 {
    use crate::{Decor, Key};
    #[test]
    fn key_decor() {
        let _rug_st_tests_llm_16_306_rrrruuuugggg_key_decor = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "  ";
        let rug_fuzz_2 = "  ";
        let rug_fuzz_3 = "test";
        let key_without_decor = Key::new(rug_fuzz_0);
        debug_assert_eq!(key_without_decor.decor(), & Decor::default());
        let decor = Decor::new(rug_fuzz_1, rug_fuzz_2);
        let key_with_decor = Key::new(rug_fuzz_3).with_decor(decor.clone());
        debug_assert_eq!(key_with_decor.decor(), & decor);
        let key_with_changed_decor = key_with_decor.clone().with_decor(Decor::default());
        debug_assert_eq!(key_with_changed_decor.decor(), & Decor::default());
        debug_assert_ne!(key_with_changed_decor.decor(), key_with_decor.decor());
        let _rug_ed_tests_llm_16_306_rrrruuuugggg_key_decor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_308 {
    use crate::key::Key;
    use crate::repr::Repr;
    use crate::internal_string::InternalString;
    use crate::raw_string::RawString;
    use std::str::FromStr;
    #[test]
    fn test_key_default_repr() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_test_key_default_repr = 0;
        let rug_fuzz_0 = "test_key";
        let rug_fuzz_1 = "test_key";
        let raw_string = RawString::from(rug_fuzz_0);
        let key_repr = Repr::new_unchecked(raw_string);
        let key = Key::new(rug_fuzz_1);
        debug_assert_eq!(key.default_repr(), key_repr);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_test_key_default_repr = 0;
    }
    #[test]
    fn test_key_default_repr_empty() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_empty = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        let raw_string = RawString::from(rug_fuzz_0);
        let key_repr = Repr::new_unchecked(raw_string);
        let key = Key::new(rug_fuzz_1);
        debug_assert_eq!(key.default_repr(), key_repr);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_empty = 0;
    }
    #[test]
    fn test_key_default_repr_quoted() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_quoted = 0;
        let rug_fuzz_0 = "\"quoted_key\"";
        let rug_fuzz_1 = "\"quoted_key\"";
        let raw_string = RawString::from(rug_fuzz_0);
        let key_repr = Repr::new_unchecked(raw_string);
        let key = Key::from_str(rug_fuzz_1).unwrap();
        debug_assert_eq!(key.default_repr(), key_repr);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_quoted = 0;
    }
    #[test]
    fn test_key_default_repr_literal() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_literal = 0;
        let rug_fuzz_0 = "'literal_key'";
        let rug_fuzz_1 = "'literal_key'";
        let raw_string = RawString::from(rug_fuzz_0);
        let key_repr = Repr::new_unchecked(raw_string);
        let key = Key::from_str(rug_fuzz_1).unwrap();
        debug_assert_eq!(key.default_repr(), key_repr);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_literal = 0;
    }
    #[test]
    fn test_key_default_repr_special_chars() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_special_chars = 0;
        let rug_fuzz_0 = "key_with_special_chars_!@#$%^&*()_+";
        let rug_fuzz_1 = "key_with_special_chars_!@#$%^&*()_+";
        let raw_string = RawString::from(rug_fuzz_0);
        let key_repr = Repr::new_unchecked(raw_string);
        let key = Key::new(rug_fuzz_1);
        debug_assert_eq!(key.default_repr(), key_repr);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_test_key_default_repr_special_chars = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_309 {
    use crate::key::Key;
    use crate::repr::Decor;
    #[test]
    fn test_despan() {
        let _rug_st_tests_llm_16_309_rrrruuuugggg_test_despan = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "   ";
        let rug_fuzz_2 = "   ";
        let input = rug_fuzz_0;
        let mut key = Key::new(input).with_decor(Decor::new(rug_fuzz_1, rug_fuzz_2));
        debug_assert!(key.decor().prefix().is_some());
        debug_assert!(key.decor().suffix().is_some());
        key.despan(input);
        debug_assert!(key.decor().prefix().is_none());
        debug_assert!(key.decor().suffix().is_none());
        let _rug_ed_tests_llm_16_309_rrrruuuugggg_test_despan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_311 {
    use crate::key::Key;
    use crate::repr::Decor;
    #[test]
    fn test_key_fmt() {
        let _rug_st_tests_llm_16_311_rrrruuuugggg_test_key_fmt = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "    ";
        let rug_fuzz_2 = "  ";
        let mut key = Key::new(rug_fuzz_0);
        key.decor_mut().set_prefix(rug_fuzz_1);
        key.decor_mut().set_suffix(rug_fuzz_2);
        key.fmt();
        debug_assert_eq!(key.get(), "key");
        debug_assert!(key.decor().prefix().is_none());
        debug_assert!(key.decor().suffix().is_none());
        let _rug_ed_tests_llm_16_311_rrrruuuugggg_test_key_fmt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_312 {
    use crate::key::Key;
    use std::str::FromStr;
    #[test]
    fn test_key_get() {
        let _rug_st_tests_llm_16_312_rrrruuuugggg_test_key_get = 0;
        let rug_fuzz_0 = "test_key";
        let key = Key::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(key.get(), "test_key");
        let _rug_ed_tests_llm_16_312_rrrruuuugggg_test_key_get = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_313 {
    use crate::key::Key;
    use crate::internal_string::InternalString;
    use std::str::FromStr;
    #[test]
    fn test_get_internal() {
        let _rug_st_tests_llm_16_313_rrrruuuugggg_test_get_internal = 0;
        let rug_fuzz_0 = "example_key";
        let rug_fuzz_1 = "Failed to create Key from str";
        let key_str = rug_fuzz_0;
        let key = Key::from_str(key_str).expect(rug_fuzz_1);
        let internal = key.get_internal();
        debug_assert_eq!(internal.as_str(), key_str);
        debug_assert_eq!(internal, & InternalString::from(key_str));
        let _rug_ed_tests_llm_16_313_rrrruuuugggg_test_get_internal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_314 {
    use crate::Key;
    use crate::InternalString;
    #[test]
    fn test_key_new_with_str() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_new_with_str = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0;
        let key = Key::new(key_str);
        debug_assert_eq!(key.get(), key_str);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_new_with_str = 0;
    }
    #[test]
    fn test_key_new_with_string() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_new_with_string = 0;
        let rug_fuzz_0 = "example";
        let key_string = rug_fuzz_0.to_string();
        let key = Key::new(key_string.clone());
        debug_assert_eq!(
            * key.get_internal(), InternalString::from(key_string.as_str())
        );
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_new_with_string = 0;
    }
    #[test]
    fn test_key_new_with_internal_string() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_new_with_internal_string = 0;
        let rug_fuzz_0 = "example";
        let internal_string = InternalString::from(rug_fuzz_0);
        let key = Key::new(internal_string.clone());
        debug_assert_eq!(* key.get_internal(), internal_string);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_new_with_internal_string = 0;
    }
    #[test]
    fn test_key_new_with_key() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_new_with_key = 0;
        let rug_fuzz_0 = "example";
        let key = Key::new(rug_fuzz_0);
        let key_cloned = Key::new(key.clone());
        debug_assert_eq!(key_cloned.get(), key.get());
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_new_with_key = 0;
    }
    #[test]
    fn test_key_from_string() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_from_string = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0;
        let key = Key::from(key_str.to_string());
        debug_assert_eq!(key.get(), key_str);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_from_string = 0;
    }
    #[test]
    fn test_key_from_str() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_from_str = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0;
        let key = Key::from(key_str);
        debug_assert_eq!(key.get(), key_str);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_from_str = 0;
    }
    #[test]
    fn test_key_from_internal_string() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_from_internal_string = 0;
        let rug_fuzz_0 = "example";
        let internal_string = InternalString::from(rug_fuzz_0);
        let key = Key::from(internal_string.clone());
        debug_assert_eq!(* key.get_internal(), internal_string);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_from_internal_string = 0;
    }
    #[test]
    fn test_key_display() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_display = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0;
        let key = Key::new(key_str);
        debug_assert_eq!(key.to_string(), key_str);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_display = 0;
    }
    #[test]
    fn test_key_debug() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_debug = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0;
        let key = Key::new(key_str);
        debug_assert_eq!(format!("{:?}", key), format!("\"{}\"", key_str));
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_debug = 0;
    }
    #[test]
    fn test_key_partial_eq_str() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_partial_eq_str = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0;
        let key = Key::new(key_str);
        debug_assert_eq!(key, key_str);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_partial_eq_str = 0;
    }
    #[test]
    fn test_key_partial_eq_string() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_partial_eq_string = 0;
        let rug_fuzz_0 = "example";
        let key_str = rug_fuzz_0.to_string();
        let key = Key::new(&key_str);
        debug_assert_eq!(key, key_str);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_partial_eq_string = 0;
    }
    #[test]
    fn test_key_ord() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_ord = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = "b";
        let key_a = Key::new(rug_fuzz_0);
        let key_b = Key::new(rug_fuzz_1);
        debug_assert!(key_a < key_b);
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_ord = 0;
    }
    #[test]
    fn test_key_partial_ord() {
        let _rug_st_tests_llm_16_314_rrrruuuugggg_test_key_partial_ord = 0;
        let rug_fuzz_0 = "a";
        let rug_fuzz_1 = "b";
        let key_a = Key::new(rug_fuzz_0);
        let key_b = Key::new(rug_fuzz_1);
        debug_assert!(key_a.partial_cmp(& key_b) == Some(std::cmp::Ordering::Less));
        let _rug_ed_tests_llm_16_314_rrrruuuugggg_test_key_partial_ord = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_315 {
    use crate::{Key, TomlError, Document};
    #[test]
    fn test_parse_valid_key() -> Result<(), TomlError> {
        let key_str = "parent.child";
        let parsed = Key::parse(key_str)?;
        assert_eq!(parsed, vec![Key::new("parent"), Key::new("child")]);
        Ok(())
    }
    #[test]
    fn test_parse_empty_key() {
        let key_str = "";
        assert!(Key::parse(key_str).is_err());
    }
    #[test]
    fn test_parse_single_key() -> Result<(), TomlError> {
        let key_str = "single";
        let parsed = Key::parse(key_str)?;
        assert_eq!(parsed, vec![Key::new("single")]);
        Ok(())
    }
    #[test]
    fn test_parse_invalid_key() {
        let key_str = "invalid key";
        assert!(Key::parse(key_str).is_err());
    }
    #[test]
    fn test_parse_quoted_key() -> Result<(), TomlError> {
        let key_str = r#""part.one"."part.two""#;
        let parsed = Key::parse(key_str)?;
        assert_eq!(parsed, vec![Key::new("part.one"), Key::new("part.two")]);
        Ok(())
    }
    #[test]
    fn test_parse_key_with_special_chars() -> Result<(), TomlError> {
        let key_str = r#"this."is".a."key""#;
        let parsed = Key::parse(key_str)?;
        assert_eq!(
            parsed, vec![Key::new("this"), Key::new("is"), Key::new("a"),
            Key::new("key")]
        );
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_318 {
    use crate::key::Key;
    use crate::repr::Decor;
    #[test]
    fn test_key_with_decor() {
        let _rug_st_tests_llm_16_318_rrrruuuugggg_test_key_with_decor = 0;
        let rug_fuzz_0 = "test_key";
        let rug_fuzz_1 = "/* Prefix */";
        let rug_fuzz_2 = "/* Suffix */";
        let key = Key::new(rug_fuzz_0);
        let decor = Decor::new(rug_fuzz_1, rug_fuzz_2);
        let decorated_key = key.with_decor(decor.clone());
        debug_assert_eq!(decor, * decorated_key.decor());
        let _rug_ed_tests_llm_16_318_rrrruuuugggg_test_key_with_decor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_319 {
    use crate::key::Key;
    use crate::repr::Repr;
    use crate::raw_string::RawString;
    #[test]
    fn test_with_repr_unchecked() {
        let _rug_st_tests_llm_16_319_rrrruuuugggg_test_with_repr_unchecked = 0;
        let rug_fuzz_0 = "test_key";
        let key_string = rug_fuzz_0;
        let raw_string: RawString = key_string.into();
        let initial_key = Key::new(key_string);
        let repr = Repr::new_unchecked(raw_string);
        let key_with_repr = initial_key.clone().with_repr_unchecked(repr.clone());
        debug_assert_eq!(key_with_repr.as_repr(), Some(& repr));
        debug_assert_eq!(key_with_repr, initial_key);
        let _rug_ed_tests_llm_16_319_rrrruuuugggg_test_with_repr_unchecked = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_323_llm_16_323 {
    use super::*;
    use crate::*;
    #[test]
    fn test_key_mut_default_repr() {
        let mut key = Key::new("example");
        let key_mut = key.as_mut();
        let expected_repr = to_key_repr("example");
        assert_eq!(
            key_mut.default_repr().as_raw().as_str(), expected_repr.as_raw().as_str()
        );
        fn to_key_repr(example: &str) -> Repr {
            let key = Key::new(example);
            key.default_repr()
        }
    }
}
#[cfg(test)]
mod tests_llm_16_325_llm_16_325 {
    use crate::Key;
    use std::str::FromStr;
    #[test]
    fn get_returns_key_value() {
        let _rug_st_tests_llm_16_325_llm_16_325_rrrruuuugggg_get_returns_key_value = 0;
        let rug_fuzz_0 = "test_key";
        let key_str = rug_fuzz_0;
        let mut key = Key::from_str(key_str).unwrap();
        let key_mut = key.as_mut();
        debug_assert_eq!(key_mut.get(), key_str);
        let _rug_ed_tests_llm_16_325_llm_16_325_rrrruuuugggg_get_returns_key_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_326 {
    use super::*;
    use crate::*;
    use crate::key::to_key_repr;
    use crate::repr::Repr;
    #[test]
    fn test_to_key_repr_unquoted() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_unquoted = 0;
        let rug_fuzz_0 = "key";
        let unquoted_key = rug_fuzz_0;
        let repr = to_key_repr(unquoted_key);
        debug_assert_eq!(repr.as_raw().as_str(), Some(unquoted_key));
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_unquoted = 0;
    }
    #[test]
    fn test_to_key_repr_single_quoted() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_single_quoted = 0;
        let rug_fuzz_0 = "'single_quoted'";
        let single_quoted_key = rug_fuzz_0;
        let repr = to_key_repr(single_quoted_key);
        debug_assert_eq!(repr.as_raw().as_str(), Some(single_quoted_key));
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_single_quoted = 0;
    }
    #[test]
    fn test_to_key_repr_double_quoted() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_double_quoted = 0;
        let rug_fuzz_0 = "\"double_quoted\"";
        let double_quoted_key = rug_fuzz_0;
        let repr = to_key_repr(double_quoted_key);
        debug_assert_eq!(repr.as_raw().as_str(), Some(double_quoted_key));
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_double_quoted = 0;
    }
    #[test]
    fn test_to_key_repr_empty() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_empty = 0;
        let rug_fuzz_0 = "";
        let empty_key = rug_fuzz_0;
        let repr = to_key_repr(empty_key);
        debug_assert_eq!(repr.as_raw().as_str(), Some("\"\""));
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_empty = 0;
    }
    #[test]
    fn test_to_key_repr_special_characters() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_special_characters = 0;
        let rug_fuzz_0 = "key with space";
        let special_chars_key = rug_fuzz_0;
        let repr = to_key_repr(special_chars_key);
        debug_assert_eq!(repr.as_raw().as_str(), Some("\"key with space\""));
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_special_characters = 0;
    }
    #[test]
    fn test_to_key_repr_newline() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_newline = 0;
        let rug_fuzz_0 = "key\nwith\nnewlines";
        let newline_key = rug_fuzz_0;
        let repr = to_key_repr(newline_key);
        debug_assert_eq!(repr.as_raw().as_str(), Some("\"key\\nwith\\nnewlines\""));
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_to_key_repr_newline = 0;
    }
}
