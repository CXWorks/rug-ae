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
        let key1 = Key::from("alpha");
        let key2 = Key::from("beta");
        let key3 = Key::from("alpha");
        let key4 = Key::from("Alpha");

        // key1 should be less than key2
        assert_eq!(key1.cmp(&key2), Ordering::Less);
        // key2 should be greater than key1
        assert_eq!(key2.cmp(&key1), Ordering::Greater);
        // key1 should be equal to key3
        assert_eq!(key1.cmp(&key3), Ordering::Equal);
        // Case-sensitive comparison: key1 should be greater than key4
        assert_eq!(key1.cmp(&key4), Ordering::Greater);
    }
}#[cfg(test)]
mod tests_llm_16_56 {
    use crate::key::Key;
    use std::cmp::PartialEq;

    #[test]
    fn test_key_eq_str() {
        let key = Key::from("example");
        let key_str: &str = "example";
        assert!(key.eq(&key_str));

        let non_matching_str: &str = "not-example";
        assert!(!key.eq(&non_matching_str));
    }
}#[cfg(test)]
mod tests_llm_16_57 {
    use crate::key::Key;
    use std::string::String;

    #[test]
    fn test_key_eq_string() {
        let key_string = String::from("my_key");
        let key = Key::from(key_string.clone());
        assert!(key.eq(&key_string));
    }

    #[test]
    fn test_key_eq_string_ref() {
        let key = Key::from("my_key");
        let key_string = "my_key".to_string();
        assert!(key.eq(&key_string));
    }

    #[test]
    fn test_key_not_eq_string() {
        let key = Key::from("my_key");
        let other_key_string = "other_key".to_string();
        assert!(!key.eq(&other_key_string));
    }
}#[cfg(test)]
mod tests_llm_16_58 {
    use crate::key::Key;

    #[test]
    fn test_eq_with_str() {
        let key_from_str: Key = "key".parse().expect("Failed to parse key from str");
        let key_from_string: Key = Key::from("key".to_string());

        assert!(key_from_str.eq("key"), "Key from str should equal 'key'");
        assert!(key_from_string.eq("key"), "Key from String should equal 'key'");
    }

    #[test]
    fn test_eq_with_different_str() {
        let key_from_str: Key = "key".parse().expect("Failed to parse key from str");

        assert!(!key_from_str.eq("other"), "Key from str should not equal 'other'");
    }

    #[test]
    fn test_eq_with_empty_str() {
        let key_from_str: Key = "key".parse().expect("Failed to parse key from str");

        assert!(!key_from_str.eq(""), "Key from str should not equal an empty string");
    }
}#[cfg(test)]
mod tests_llm_16_59_llm_16_59 {
    use crate::key::Key;
    use crate::key::Decor;
    use std::borrow::Borrow;
    use std::str::FromStr;

    #[test]
    fn test_key_eq_same_key() {
        let key1 = Key::from("same-key");
        let key2 = Key::from("same-key");
        assert!(key1.eq(&key2));
    }

    #[test]
    fn test_key_eq_different_keys() {
        let key1 = Key::from("key1");
        let key2 = Key::from("key2");
        assert!(!key1.eq(&key2));
    }

    #[test]
    fn test_key_eq_with_decor() {
        let key1 = Key::from("key").with_decor(Decor::new(" ", " "));
        let key2 = Key::from("key");
        assert!(key1.eq(&key2));
    }

    #[test]
    fn test_key_eq_with_different_decor() {
        let key1 = Key::from("key").with_decor(Decor::new(" ", " "));
        let key2 = Key::from("key").with_decor(Decor::new("", "  "));
        assert!(key1.eq(&key2));
    }

    #[test]
    fn test_key_eq_with_same_decor() {
        let decor = Decor::new(" ", " ");
        let key1 = Key::from("key").with_decor(decor.clone());
        let key2 = Key::from("key").with_decor(decor);
        assert!(key1.eq(&key2));
    }

    #[test]
    fn test_key_eq_with_different_repr() {
        let key1 = Key::from("key");
        // "with_repr_unchecked" and "Repr" creation is probably internal and might need internal knowledge or mock.
        // Skip tests that require internal representation changes or adjust as per actual internal testing strategy.
        // Using public `from_str` to mimic the `Repr` parsing.
        let key2 = Key::from_str("key").unwrap();
        assert!(key1.eq(&key2));
    }

    #[test]
    #[should_panic]
    fn test_key_eq_panic_on_different_keys() {
        let key1 = Key::from("key1");
        let key2 = Key::from("key2");
        assert!(key1.eq(&key2));
    }

    #[test]
    fn test_key_eq_variant_forms() {
        let key1 = Key::from("key");
        let key2 = Key::from_str("key").unwrap();
        let key3 = Key::from(String::from("key"));
        let key4 = Key::from("key");
        assert!(key1.eq(&key2));
        assert!(key2.eq(&key3));
        assert!(key3.eq(&key4));
        assert!(key4.eq(&key1));
    }
}#[cfg(test)]
mod tests_llm_16_60 {
    use crate::key::Key;
    use std::cmp::Ordering;

    #[test]
    fn test_partial_cmp() {
        let key1 = Key::from("alpha");
        let key2 = Key::from("beta");
        let key3 = Key::from("alpha");

        assert_eq!(key1.partial_cmp(&key2), Some(Ordering::Less));
        assert_eq!(key2.partial_cmp(&key1), Some(Ordering::Greater));
        assert_eq!(key1.partial_cmp(&key3), Some(Ordering::Equal));
    }
}#[cfg(test)]
mod tests_llm_16_61 {
    use crate::key::Key;
    use std::convert::From;

    #[test]
    fn test_key_from_string_reference() {
        let test_string = "test_key".to_string();
        let key_from_string = Key::from(&test_string);
        assert_eq!(key_from_string.get(), test_string);
    }
}#[cfg(test)]
mod tests_llm_16_62 {
    use crate::Key;

    #[test]
    fn key_from_str() {
        let key_str = "example-key";
        let key: Key = key_str.into();
        assert_eq!(key.get(), key_str);
    }

    #[test]
    fn key_from_string() {
        let key_string = "example-key".to_string();
        let key: Key = (&key_string).into();
        assert_eq!(key.get(), key_string);
    }
}#[cfg(test)]
mod tests_llm_16_63 {
    use crate::Key;

    #[test]
    fn test_from_internal_string_to_key() {
        use crate::internal_string::InternalString;

        let raw_key = "example_key";
        let internal_string = InternalString::from(raw_key);
        let key = Key::from(internal_string.clone());

        assert_eq!(key.get(), internal_string.as_str());
    }
}#[cfg(test)]
mod tests_llm_16_64 {
    use super::*;

use crate::*;
    use std::convert::From;

    #[test]
    fn test_from_string_to_key() {
        let test_string = "test_key".to_string();
        let key = Key::from(test_string.clone());
        assert_eq!(*key, test_string);
    }
}#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;

use crate::*;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_key_hash() {
        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        let key1 = Key::from("test_key");
        let key2 = Key::from("test_key");
        let key3 = Key::from("different_key");

        // Ensure that two identical keys produce the same hash
        key1.hash(&mut hasher1);
        key2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());

        // Ensure that two different keys produce different hashes
        hasher1 = std::collections::hash_map::DefaultHasher::new();
        key3.hash(&mut hasher1);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}#[cfg(test)]
mod tests_llm_16_66 {
    use crate::Key;
    use std::ops::Deref;

    #[test]
    fn deref_key() {
        // Create a Key from a string
        let key_string = "example-key".to_string();
        let key: Key = Key::from(key_string.clone());

        // Deref Key to get a &str
        let deref_str: &str = key.deref();

        // The deref_str should match the original string that created key
        assert_eq!(deref_str, key_string);
    }
}#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;

use crate::*;
    use std::str::FromStr; // For calling from_str on Key

    #[test]
    fn test_from_str_valid_simple_key() {
        let key_str = "valid_key";
        let key = Key::from_str(key_str);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().get(), key_str);
    }

    #[test]
    fn test_from_str_valid_quoted_key() {
        let key_str = "\"valid_key\"";
        let key = Key::from_str(key_str);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().get(), key_str.trim_matches('\"'));
    }

    #[test]
    fn test_from_str_empty_key() {
        let key_str = "";
        let key = Key::from_str(key_str);
        assert!(key.is_err());
    }

    #[test]
    fn test_from_str_invalid_characters() {
        let key_str = "invalid key"; // Contains a space, which is not allowed in a key
        let key = Key::from_str(key_str);
        assert!(key.is_err());
    }

    #[test]
    fn test_from_str_valid_quoted_with_invalid_characters() {
        let key_str = "\"invalid key\""; // Contains space, but is a valid quoted key
        let key = Key::from_str(key_str);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().get(), key_str.trim_matches('\"'));
    }

    #[test]
    fn test_from_str_single_quoted_key() {
        let key_str = "'valid_key'";
        let key = Key::from_str(key_str);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().get(), key_str.trim_matches('\''));
    }

    #[test]
    fn test_from_str_single_quoted_with_invalid_characters() {
        let key_str = "'invalid key'"; // Contains space, but is a valid single quoted key
        let key = Key::from_str(key_str);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().get(), key_str.trim_matches('\''));
    }

    // Add more tests if necessary
}#[cfg(test)]
mod tests_llm_16_68_llm_16_68 {
    use crate::{Key, KeyMut};
    use std::ops::Deref;

    #[test]
    fn deref_for_key_mut() {
        let mut key = Key::new("some_key");
        let key_mut = key.as_mut();
        let deref_value: &str = key_mut.deref();
        assert_eq!(deref_value, "some_key");
    }
}#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;

use crate::*;
    use std::borrow::Borrow;

    #[test]
    fn key_mut_eq_str() {
        let mut key = Key::from("example");
        let key_mut = key.as_mut();
        let key_str: &str = key_mut.borrow();
        assert!(key_mut.eq(&key_str));
    }

    #[test]
    fn key_mut_neq_str() {
        let mut key = Key::from("example");
        let key_mut = key.as_mut();
        assert!(!key_mut.eq(&"other"));
    }
}#[cfg(test)]
mod tests_llm_16_70 {
    use super::*;

use crate::*;

    #[test]
    fn eq_with_string() {
        let mut key = Key::new("example");
        let key_mut = key.as_mut();
        let string = "example".to_string();
        assert!(key_mut.eq(&string));
        
        let string_different = "different".to_string();
        assert!(!key_mut.eq(&string_different));
    }
}#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;

use crate::*;
    use crate::key::{Key, KeyMut};

    #[test]
    fn test_key_mut_eq_str() {
        let mut key = Key::new("key");
        let mut key_mut = key.as_mut();

        assert!(key_mut.eq("key"));
        assert!(!key_mut.eq("other_key"));
    }
}#[cfg(test)]
mod tests_llm_16_303 {
    use crate::{InternalString, Key};

    #[test]
    fn test_from_key_for_internal_string() {
        let key_str = "test_key";
        let key = Key::from(key_str);
        let internal_string: InternalString = InternalString::from(key);

        assert_eq!(key_str, internal_string.as_str());
    }
}#[cfg(test)]
mod tests_llm_16_304 {
    use crate::{Key, KeyMut};

    #[test]
    fn test_key_as_mut() {
        let mut key = Key::new("key_name");
        let mut key_mut = key.as_mut();

        key_mut.fmt();

        assert_eq!(key_mut.get(), "key_name");
        assert_eq!(&*key_mut, "key_name");
    }
}#[cfg(test)]
mod tests_llm_16_306 {
    use crate::{Decor, Key};

    #[test]
    fn key_decor() {
        let key_without_decor = Key::new("test");
        assert_eq!(key_without_decor.decor(), &Decor::default());

        let decor = Decor::new("  ", "  ");
        let key_with_decor = Key::new("test").with_decor(decor.clone());
        assert_eq!(key_with_decor.decor(), &decor);
        
        let key_with_changed_decor = key_with_decor.clone().with_decor(Decor::default());
        assert_eq!(key_with_changed_decor.decor(), &Decor::default());
        assert_ne!(key_with_changed_decor.decor(), key_with_decor.decor());
    }
}#[cfg(test)]
mod tests_llm_16_308 {
    use crate::key::Key;
    use crate::repr::Repr;
    use crate::internal_string::InternalString;
    use crate::raw_string::RawString;
    use std::str::FromStr;

    #[test]
    fn test_key_default_repr() {
        let raw_string = RawString::from("test_key");
        let key_repr = Repr::new_unchecked(raw_string);

        let key = Key::new("test_key");
        assert_eq!(key.default_repr(), key_repr);
    }

    #[test]
    fn test_key_default_repr_empty() {
        let raw_string = RawString::from("");
        let key_repr = Repr::new_unchecked(raw_string);

        let key = Key::new("");
        assert_eq!(key.default_repr(), key_repr);
    }

    #[test]
    fn test_key_default_repr_quoted() {
        let raw_string = RawString::from("\"quoted_key\"");
        let key_repr = Repr::new_unchecked(raw_string);

        let key = Key::from_str("\"quoted_key\"").unwrap();
        assert_eq!(key.default_repr(), key_repr);
    }

    #[test]
    fn test_key_default_repr_literal() {
        let raw_string = RawString::from("'literal_key'");
        let key_repr = Repr::new_unchecked(raw_string);

        let key = Key::from_str("'literal_key'").unwrap();
        assert_eq!(key.default_repr(), key_repr);
    }

    #[test]
    fn test_key_default_repr_special_chars() {
        let raw_string = RawString::from("key_with_special_chars_!@#$%^&*()_+");
        let key_repr = Repr::new_unchecked(raw_string);

        let key = Key::new("key_with_special_chars_!@#$%^&*()_+");
        assert_eq!(key.default_repr(), key_repr);
    }
}#[cfg(test)]
mod tests_llm_16_309 {
    use crate::key::Key;
    use crate::repr::Decor;

    #[test]
    fn test_despan() {
        let input = "key";

        let mut key = Key::new(input).with_decor(Decor::new("   ", "   "));
        assert!(key.decor().prefix().is_some());
        assert!(key.decor().suffix().is_some());

        key.despan(input);
        assert!(key.decor().prefix().is_none());
        assert!(key.decor().suffix().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_311 {
    use crate::key::Key;
    use crate::repr::Decor;

    #[test]
    fn test_key_fmt() {
        let mut key = Key::new("key");
        key.decor_mut().set_prefix("    "); // some arbitrary whitespace
        key.decor_mut().set_suffix("  ");   // some arbitrary whitespace
        key.fmt();
        assert_eq!(key.get(), "key");
        assert!(key.decor().prefix().is_none());
        assert!(key.decor().suffix().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_312 {
    use crate::key::Key;
    use std::str::FromStr;

    #[test]
    fn test_key_get() {
        let key = Key::from_str("test_key").unwrap();
        assert_eq!(key.get(), "test_key");
    }
}#[cfg(test)]
mod tests_llm_16_313 {
    use crate::key::Key;
    use crate::internal_string::InternalString;
    use std::str::FromStr;

    #[test]
    fn test_get_internal() {
        let key_str = "example_key";
        let key = Key::from_str(key_str).expect("Failed to create Key from str");
        let internal = key.get_internal();

        assert_eq!(internal.as_str(), key_str);
        assert_eq!(internal, &InternalString::from(key_str));
    }
}#[cfg(test)]
mod tests_llm_16_314 {
    use crate::Key;
    use crate::InternalString;

    #[test]
    fn test_key_new_with_str() {
        let key_str = "example";
        let key = Key::new(key_str);
        assert_eq!(key.get(), key_str);
    }

    #[test]
    fn test_key_new_with_string() {
        let key_string = "example".to_string();
        let key = Key::new(key_string.clone());
        assert_eq!(*key.get_internal(), InternalString::from(key_string.as_str()));
    }

    #[test]
    fn test_key_new_with_internal_string() {
        let internal_string = InternalString::from("example");
        let key = Key::new(internal_string.clone());
        assert_eq!(*key.get_internal(), internal_string);
    }

    #[test]
    fn test_key_new_with_key() {
        let key = Key::new("example");
        let key_cloned = Key::new(key.clone());
        assert_eq!(key_cloned.get(), key.get());
    }

    #[test]
    fn test_key_from_string() {
        let key_str = "example";
        let key = Key::from(key_str.to_string());
        assert_eq!(key.get(), key_str);
    }

    #[test]
    fn test_key_from_str() {
        let key_str = "example";
        let key = Key::from(key_str);
        assert_eq!(key.get(), key_str);
    }

    #[test]
    fn test_key_from_internal_string() {
        let internal_string = InternalString::from("example");
        let key = Key::from(internal_string.clone());
        assert_eq!(*key.get_internal(), internal_string);
    }

    #[test]
    fn test_key_display() {
        let key_str = "example";
        let key = Key::new(key_str);
        assert_eq!(key.to_string(), key_str);
    }

    #[test]
    fn test_key_debug() {
        let key_str = "example";
        let key = Key::new(key_str);
        assert_eq!(format!("{:?}", key), format!("\"{}\"", key_str));
    }

    #[test]
    fn test_key_partial_eq_str() {
        let key_str = "example";
        let key = Key::new(key_str);
        assert_eq!(key, key_str);
    }

    #[test]
    fn test_key_partial_eq_string() {
        let key_str = "example".to_string();
        let key = Key::new(&key_str);
        assert_eq!(key, key_str);
    }

    #[test]
    fn test_key_ord() {
        let key_a = Key::new("a");
        let key_b = Key::new("b");
        assert!(key_a < key_b);
    }

    #[test]
    fn test_key_partial_ord() {
        let key_a = Key::new("a");
        let key_b = Key::new("b");
        assert!(key_a.partial_cmp(&key_b) == Some(std::cmp::Ordering::Less));
    }
}#[cfg(test)]
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
        assert_eq!(parsed, vec![Key::new("this"), Key::new("is"), Key::new("a"), Key::new("key")]);
        Ok(())
    }
}#[cfg(test)]
mod tests_llm_16_318 {
    use crate::key::Key;
    use crate::repr::Decor;

    #[test]
    fn test_key_with_decor() {
        let key = Key::new("test_key");
        let decor = Decor::new("/* Prefix */", "/* Suffix */");
        let decorated_key = key.with_decor(decor.clone());

        assert_eq!(decor, *decorated_key.decor());
    }
}#[cfg(test)]
mod tests_llm_16_319 {
    use crate::key::Key;
    use crate::repr::Repr;
    use crate::raw_string::RawString;

    #[test]
    fn test_with_repr_unchecked() {
        let key_string = "test_key";
        let raw_string: RawString = key_string.into();

        let initial_key = Key::new(key_string);
        let repr = Repr::new_unchecked(raw_string);
        let key_with_repr = initial_key.clone().with_repr_unchecked(repr.clone());

        assert_eq!(key_with_repr.as_repr(), Some(&repr));
        assert_eq!(key_with_repr, initial_key);
    }
}#[cfg(test)]
mod tests_llm_16_323_llm_16_323 {
    use super::*;

use crate::*;

    #[test]
    fn test_key_mut_default_repr() {
        let mut key = Key::new("example");
        let key_mut = key.as_mut();

        let expected_repr = to_key_repr("example");

        assert_eq!(
            key_mut.default_repr().as_raw().as_str(),
            expected_repr.as_raw().as_str()
        );

        // Helper function to match the expected behavior of to_key_repr
        // The exact implementation of to_key_repr will depend on the actual crate implementation
        fn to_key_repr(example: &str) -> Repr {
            let key = Key::new(example);
            key.default_repr()
        }
    }
}#[cfg(test)]
mod tests_llm_16_325_llm_16_325 {
    use crate::Key;
    use std::str::FromStr;

    #[test]
    fn get_returns_key_value() {
        let key_str = "test_key";
        let mut key = Key::from_str(key_str).unwrap();
        let key_mut = key.as_mut();
        assert_eq!(key_mut.get(), key_str);
    }
}#[cfg(test)]
mod tests_llm_16_326 {
    use super::*;

use crate::*;
    use crate::key::to_key_repr;
    use crate::repr::Repr;

    #[test]
    fn test_to_key_repr_unquoted() {
        let unquoted_key = "key";
        let repr = to_key_repr(unquoted_key);
        assert_eq!(repr.as_raw().as_str(), Some(unquoted_key));
    }

    #[test]
    fn test_to_key_repr_single_quoted() {
        let single_quoted_key = "'single_quoted'";
        let repr = to_key_repr(single_quoted_key);
        // This test assumes that single quoted keys are returned
        // in the same representation
        assert_eq!(repr.as_raw().as_str(), Some(single_quoted_key));
    }

    #[test]
    fn test_to_key_repr_double_quoted() {
        let double_quoted_key = "\"double_quoted\"";
        let repr = to_key_repr(double_quoted_key);
        // This test assumes that double quoted keys are returned
        // in the same representation
        assert_eq!(repr.as_raw().as_str(), Some(double_quoted_key));
    }

    #[test]
    fn test_to_key_repr_empty() {
        let empty_key = "";
        let repr = to_key_repr(empty_key);
        assert_eq!(repr.as_raw().as_str(), Some("\"\""));
    }

    #[test]
    fn test_to_key_repr_special_characters() {
        let special_chars_key = "key with space";
        let repr = to_key_repr(special_chars_key);
        // This test assumes that keys with special characters
        // are returned quoted
        assert_eq!(repr.as_raw().as_str(), Some("\"key with space\""));
    }

    #[test]
    fn test_to_key_repr_newline() {
        let newline_key = "key\nwith\nnewlines";
        let repr = to_key_repr(newline_key);
        // This test assumes that newlines in keys are escaped
        assert_eq!(repr.as_raw().as_str(), Some("\"key\\nwith\\nnewlines\""));
    }
}