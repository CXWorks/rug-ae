use std::borrow::Cow;

use crate::RawString;

/// A value together with its `to_string` representation,
/// including surrounding it whitespaces and comments.
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Formatted<T> {
    value: T,
    repr: Option<Repr>,
    decor: Decor,
}

impl<T> Formatted<T>
where
    T: ValueRepr,
{
    /// Default-formatted value
    pub fn new(value: T) -> Self {
        Self {
            value,
            repr: None,
            decor: Default::default(),
        }
    }

    pub(crate) fn set_repr_unchecked(&mut self, repr: Repr) {
        self.repr = Some(repr);
    }

    /// The wrapped value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// The wrapped value
    pub fn into_value(self) -> T {
        self.value
    }

    /// Returns the raw representation, if available.
    pub fn as_repr(&self) -> Option<&Repr> {
        self.repr.as_ref()
    }

    /// Returns the default raw representation.
    pub fn default_repr(&self) -> Repr {
        self.value.to_repr()
    }

    /// Returns a raw representation.
    pub fn display_repr(&self) -> Cow<str> {
        self.as_repr()
            .and_then(|r| r.as_raw().as_str())
            .map(Cow::Borrowed)
            .unwrap_or_else(|| {
                Cow::Owned(self.default_repr().as_raw().as_str().unwrap().to_owned())
            })
    }

    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.repr.as_ref().and_then(|r| r.span())
    }

    pub(crate) fn despan(&mut self, input: &str) {
        self.decor.despan(input);
        if let Some(repr) = &mut self.repr {
            repr.despan(input);
        }
    }

    /// Returns the surrounding whitespace
    pub fn decor_mut(&mut self) -> &mut Decor {
        &mut self.decor
    }

    /// Returns the surrounding whitespace
    pub fn decor(&self) -> &Decor {
        &self.decor
    }

    /// Auto formats the value.
    pub fn fmt(&mut self) {
        self.repr = Some(self.value.to_repr());
    }
}

impl<T> std::fmt::Debug for Formatted<T>
where
    T: std::fmt::Debug,
{
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut d = formatter.debug_struct("Formatted");
        d.field("value", &self.value);
        match &self.repr {
            Some(r) => d.field("repr", r),
            None => d.field("repr", &"default"),
        };
        d.field("decor", &self.decor);
        d.finish()
    }
}

impl<T> std::fmt::Display for Formatted<T>
where
    T: ValueRepr,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::encode::Encode::encode(self, f, None, ("", ""))
    }
}

pub trait ValueRepr: crate::private::Sealed {
    /// The TOML representation of the value
    fn to_repr(&self) -> Repr;
}

/// TOML-encoded value
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Repr {
    raw_value: RawString,
}

impl Repr {
    pub(crate) fn new_unchecked(raw: impl Into<RawString>) -> Self {
        Repr {
            raw_value: raw.into(),
        }
    }

    /// Access the underlying value
    pub fn as_raw(&self) -> &RawString {
        &self.raw_value
    }

    /// Returns the location within the original document
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.raw_value.span()
    }

    pub(crate) fn despan(&mut self, input: &str) {
        self.raw_value.despan(input)
    }

    pub(crate) fn encode(&self, buf: &mut dyn std::fmt::Write, input: &str) -> std::fmt::Result {
        self.as_raw().encode(buf, input)
    }
}

impl std::fmt::Debug for Repr {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.raw_value.fmt(formatter)
    }
}

/// A prefix and suffix,
///
/// Including comments, whitespaces and newlines.
#[derive(Eq, PartialEq, Clone, Default, Hash)]
pub struct Decor {
    prefix: Option<RawString>,
    suffix: Option<RawString>,
}

impl Decor {
    /// Creates a new decor from the given prefix and suffix.
    pub fn new(prefix: impl Into<RawString>, suffix: impl Into<RawString>) -> Self {
        Self {
            prefix: Some(prefix.into()),
            suffix: Some(suffix.into()),
        }
    }

    /// Go back to default decor
    pub fn clear(&mut self) {
        self.prefix = None;
        self.suffix = None;
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&RawString> {
        self.prefix.as_ref()
    }

    pub(crate) fn prefix_encode(
        &self,
        buf: &mut dyn std::fmt::Write,
        input: Option<&str>,
        default: &str,
    ) -> std::fmt::Result {
        if let Some(prefix) = self.prefix() {
            prefix.encode_with_default(buf, input, default)
        } else {
            write!(buf, "{}", default)
        }
    }

    /// Set the prefix.
    pub fn set_prefix(&mut self, prefix: impl Into<RawString>) {
        self.prefix = Some(prefix.into());
    }

    /// Get the suffix.
    pub fn suffix(&self) -> Option<&RawString> {
        self.suffix.as_ref()
    }

    pub(crate) fn suffix_encode(
        &self,
        buf: &mut dyn std::fmt::Write,
        input: Option<&str>,
        default: &str,
    ) -> std::fmt::Result {
        if let Some(suffix) = self.suffix() {
            suffix.encode_with_default(buf, input, default)
        } else {
            write!(buf, "{}", default)
        }
    }

    /// Set the suffix.
    pub fn set_suffix(&mut self, suffix: impl Into<RawString>) {
        self.suffix = Some(suffix.into());
    }

    pub(crate) fn despan(&mut self, input: &str) {
        if let Some(prefix) = &mut self.prefix {
            prefix.despan(input);
        }
        if let Some(suffix) = &mut self.suffix {
            suffix.despan(input);
        }
    }
}

impl std::fmt::Debug for Decor {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut d = formatter.debug_struct("Decor");
        match &self.prefix {
            Some(r) => d.field("prefix", r),
            None => d.field("prefix", &"default"),
        };
        match &self.suffix {
            Some(r) => d.field("suffix", r),
            None => d.field("suffix", &"default"),
        };
        d.finish()
    }
}
#[cfg(test)]
mod tests_llm_16_439 {
    use crate::repr::{Decor, RawString};

    #[test]
    fn clear_resets_decor_to_default() {
        let prefix = RawString::from("prefix");
        let suffix = RawString::from("suffix");
        let mut decor = Decor::new(prefix, suffix);

        decor.clear();

        assert_eq!(decor.prefix(), None);
        assert_eq!(decor.suffix(), None);
    }
}#[cfg(test)]
mod tests_llm_16_440_llm_16_440 {
    use crate::Decor;
    use crate::repr::RawString;

    #[test]
    fn test_despan() {
        let mut decor = Decor::new(RawString::from("/* My Prefix */"), RawString::from("/* My Suffix */"));
        let input = "/* My Prefix */some_value/* My Suffix */";
        decor.despan(input);

        assert!(decor.prefix().is_none());
        assert!(decor.suffix().is_none());
    }
}#[cfg(test)]
mod tests_llm_16_441 {
    use super::*;

use crate::*;
    use crate::repr::RawString;

    #[test]
    fn test_decor_new() {
        let prefix = "prefix";
        let suffix = "suffix";
        let decor = Decor::new(prefix, suffix);

        assert_eq!(
            decor.prefix(),
            Some(&RawString::from(prefix))
        );
        assert_eq!(
            decor.suffix(),
            Some(&RawString::from(suffix))
        );
    }

    #[test]
    fn test_decor_new_empty() {
        let prefix = "";
        let suffix = "";
        let decor = Decor::new(prefix, suffix);

        assert_eq!(
            decor.prefix(),
            Some(&RawString::from(prefix))
        );
        assert_eq!(
            decor.suffix(),
            Some(&RawString::from(suffix))
        );
    }

    #[test]
    fn test_decor_new_with_spaces() {
        let prefix = "    ";
        let suffix = "  ";
        let decor = Decor::new(prefix, suffix);

        assert_eq!(
            decor.prefix(),
            Some(&RawString::from(prefix))
        );
        assert_eq!(
            decor.suffix(),
            Some(&RawString::from(suffix))
        );
    }

    #[test]
    fn test_decor_new_with_newlines() {
        let prefix = "\n\n\n";
        let suffix = "\n\n";
        let decor = Decor::new(prefix, suffix);

        assert_eq!(
            decor.prefix(),
            Some(&RawString::from(prefix))
        );
        assert_eq!(
            decor.suffix(),
            Some(&RawString::from(suffix))
        );
    }
}#[cfg(test)]
mod tests_llm_16_442 {
    use crate::repr::{Decor, RawString};

    #[test]
    fn test_prefix_with_no_prefix_set() {
        let decor = Decor::default();
        assert_eq!(decor.prefix(), None);
    }

    #[test]
    fn test_prefix_with_prefix_set() {
        let prefix = "## ";
        let mut decor = Decor::default();
        decor.set_prefix(prefix);
        assert_eq!(decor.prefix(), Some(&RawString::from(prefix)));
    }
}#[cfg(test)]
mod tests_llm_16_443 {
    use crate::Decor; // Note: Adjust the path according to the actual module structure
    use std::fmt::Write;

    #[test]
    fn test_prefix_encode_with_prefix() {
        let mut decor = Decor::new("prefix_", "");
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "prefix_default");

        buffer.clear();
        decor.prefix_encode(&mut buffer, Some("input"), "default").unwrap();
        assert_eq!(buffer, "prefix_input");
    }

    #[test]
    fn test_prefix_encode_without_prefix() {
        let mut decor = Decor::default();
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "default");

        buffer.clear();
        decor.prefix_encode(&mut buffer, Some("input"), "default").unwrap();
        assert_eq!(buffer, "default");
    }

    #[test]
    fn test_prefix_encode_with_cleared_prefix() {
        let mut decor = Decor::new("prefix_", "");
        decor.clear();
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "default");

        buffer.clear();
        decor.prefix_encode(&mut buffer, Some("input"), "default").unwrap();
        assert_eq!(buffer, "default");
    }

    #[test]
    fn test_prefix_encode_set_prefix() {
        let mut decor = Decor::default();
        decor.set_prefix("new_prefix_");
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "new_prefix_default");

        buffer.clear();
        decor.prefix_encode(&mut buffer, Some("input"), "default").unwrap();
        assert_eq!(buffer, "new_prefix_input");
    }
}#[cfg(test)]
mod tests_llm_16_444 {
    use super::*;

use crate::*;
    use crate::repr::Decor;

    #[test]
    fn test_set_prefix() {
        let mut decor = Decor::default();
        let new_prefix = "## ";
        decor.set_prefix(new_prefix.to_string());
        assert_eq!(decor.prefix(), Some(&new_prefix.into()));
    }

    #[test]
    fn test_set_prefix_clear_and_set() {
        let mut decor = Decor::default();
        decor.set_prefix("123");
        decor.clear();
        assert_eq!(decor.prefix(), None);
        let new_prefix = "## ";
        decor.set_prefix(new_prefix.to_string());
        assert_eq!(decor.prefix(), Some(&new_prefix.into()));
    }

    #[test]
    fn test_set_prefix_overwrite_existing() {
        let mut decor = Decor::default();
        decor.set_prefix("123");
        let new_prefix = "## ";
        decor.set_prefix(new_prefix.to_string());
        assert_eq!(decor.prefix(), Some(&new_prefix.into()));
    }
}#[cfg(test)]
mod tests_llm_16_445 {
    use super::*;

use crate::*;
    use crate::repr::Decor;
    use crate::repr::RawString;

    #[test]
    fn test_set_suffix() {
        let mut decor = Decor::default();
        assert_eq!(decor.suffix(), None);

        let new_suffix = " # This is a suffix";
        decor.set_suffix(new_suffix);
        assert_eq!(
            decor.suffix(),
            Some(&RawString::from(new_suffix.to_string()))
        );
    }
}#[cfg(test)]
mod tests_llm_16_446 {
    use crate::{repr::{Decor, RawString}};

    #[test]
    fn suffix_none_when_decor_is_default() {
        let decor = Decor::default();
        assert_eq!(decor.suffix(), None);
    }

    #[test]
    fn suffix_returns_suffix_when_set() {
        let mut decor = Decor::default();
        let suffix = RawString::from(" # suffix");
        decor.set_suffix(suffix.clone());
        assert_eq!(decor.suffix(), Some(&suffix));
    }

    #[test]
    fn suffix_none_after_clear() {
        let mut decor = Decor::new(" # prefix", " # suffix");
        decor.clear();
        assert_eq!(decor.suffix(), None);
    }
}#[cfg(test)]
mod tests_llm_16_447 {
    use crate::Decor;
    use crate::repr::RawString;
    use std::fmt::Write;

    #[test]
    fn test_suffix_encode_with_suffix() {
        let mut decor = Decor::new("prefix", "suffix");
        let mut output = String::new();
        decor.suffix_encode(&mut output, None, "default_suffix").unwrap();
        assert_eq!(output, "suffix");
    }

    #[test]
    fn test_suffix_encode_without_suffix() {
        let mut decor = Decor::default();
        let mut output = String::new();
        decor.suffix_encode(&mut output, None, "default_suffix").unwrap();
        assert_eq!(output, "default_suffix");
    }

    #[test]
    fn test_suffix_encode_with_input() {
        let mut decor = Decor::new("prefix", "suffix");
        let mut output = String::new();
        decor.suffix_encode(&mut output, Some("input_suffix"), "default_suffix").unwrap();
        assert_eq!(output, "input_suffix");
    }

    #[test]
    fn test_suffix_encode_without_suffix_with_input() {
        let mut decor = Decor::default();
        let mut output = String::new();
        decor.suffix_encode(&mut output, Some("input_suffix"), "default_suffix").unwrap();
        assert_eq!(output, "input_suffix");
    }

    #[test]
    fn test_suffix_encode_with_empty_suffix() {
        let mut decor = Decor::new("prefix", "");
        let mut output = String::new();
        decor.suffix_encode(&mut output, None, "default_suffix").unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_suffix_encode_with_empty_suffix_and_input() {
        let mut decor = Decor::new("prefix", "");
        let mut output = String::new();
        decor.suffix_encode(&mut output, Some("input_suffix"), "default_suffix").unwrap();
        assert_eq!(output, "input_suffix");
    }
}#[cfg(test)]
mod tests_llm_16_451 {
    use super::*;

use crate::*;
    use crate::repr::{Decor, Formatted, Repr, ValueRepr};
    use crate::raw_string::RawString;
    use std::borrow::Cow;
    use std::fmt::Write;

    #[test]
    fn test_default_repr() {
        let value = "test_value".to_string();
        let formatted_value = Formatted::new(value.clone());
        let default_repr = formatted_value.default_repr();

        let mut buf = String::new();
        default_repr.encode(&mut buf, &value).unwrap();
        let expected_repr = value.clone();

        assert_eq!(Cow::Borrowed(expected_repr.as_str()), formatted_value.display_repr());
        assert_eq!(expected_repr, buf, "Encoded representation should match the expected value");
    }
}#[cfg(test)]
mod tests_llm_16_455 {
    use crate::Formatted;
    use crate::repr::Decor;

    #[test]
    fn test_new_formatted() {
        let value = 42;
        let formatted = Formatted::new(value);

        assert_eq!(formatted.value(), &value);
        assert!(formatted.as_repr().is_none());
        assert_eq!(formatted.decor(), &Decor::default());
    }
}#[cfg(test)]
mod tests_llm_16_458 {
    use super::*;

use crate::*;
    use crate::repr::{Decor, Formatted, RawString};
    use std::default::Default;

    #[test]
    fn test_value() {
        // Create a Formatted instance with a default decor and a simple integer value
        let formatted = Formatted {
            value: 42,
            repr: None,
            decor: Decor::default(),
        };

        // Test that `value` method returns a reference to the value
        assert_eq!(*formatted.value(), 42);
    }
}#[cfg(test)]
mod tests_llm_16_459 {
    use super::*;

use crate::*;
    use crate::repr::Repr;
    use crate::raw_string::RawString;
    use crate::internal_string::InternalString;
    
    #[test]
    fn test_as_raw_empty() {
        let empty_internal = InternalString::from("");
        let empty_raw = RawString::from(&empty_internal);
        let repr = Repr::new_unchecked(empty_raw.clone());
        assert_eq!(repr.as_raw(), &empty_raw);
    }

    #[test]
    fn test_as_raw_explicit() {
        let explicit_internal = InternalString::from("value");
        let explicit_raw = RawString::from(explicit_internal);
        let repr = Repr::new_unchecked(explicit_raw.clone());
        assert_eq!(repr.as_raw(), &explicit_raw);
    }

    #[test]
    fn test_as_raw_spanned() {
        let span = std::ops::Range { start: 0, end: 5 };
        let spanned_raw = RawString::with_span(span);
        let repr = Repr::new_unchecked(spanned_raw.clone());
        assert_eq!(repr.as_raw(), &spanned_raw);
    }
}#[cfg(test)]
mod tests_llm_16_460 {
    use super::*;

use crate::*;
    use crate::repr::Repr;

    #[test]
    fn test_despan() {
        let mut repr = Repr::new_unchecked("initial value");
        let input = "updated value";
        repr.despan(input);
        assert_eq!(repr.as_raw().as_str(), Some(input))
    }
}#[cfg(test)]
mod tests_llm_16_461 {
    use super::*;

use crate::*;

    #[test]
    fn test_encode() {
        // Prepare the Repr object
        let input_str = "example input";
        let mut buffer = String::new();
        let repr = Repr::new_unchecked(input_str);

        // Perform the encode operation
        let encode_result = repr.encode(&mut buffer, input_str);

        // Check the encode operation was successful
        assert!(encode_result.is_ok());
        // Check the buffer now contains the input_str without carriage returns
        assert_eq!(buffer, "example input");
    }
}#[cfg(test)]
mod tests_llm_16_462 {
    use super::*;

use crate::*;

    #[test]
    fn new_unchecked_empty_string() {
        let raw = RawString::from("");
        let repr = Repr::new_unchecked(raw);
        assert!(repr.as_raw().as_str().is_some());
        assert_eq!(repr.as_raw().as_str().unwrap(), "");
        assert_eq!(repr.as_raw().span(), None);
    }

    #[test]
    fn new_unchecked_non_empty_string() {
        let raw = RawString::from("value");
        let repr = Repr::new_unchecked(raw);
        assert!(repr.as_raw().as_str().is_some());
        assert_eq!(repr.as_raw().as_str().unwrap(), "value");
        assert_eq!(repr.as_raw().span(), None);
    }

    #[test]
    fn new_unchecked_internal_string() {
        let internal = InternalString::from("value");
        let raw = RawString::from(internal);
        let repr = Repr::new_unchecked(raw);
        assert!(repr.as_raw().as_str().is_some());
        assert_eq!(repr.as_raw().as_str().unwrap(), "value");
        assert_eq!(repr.as_raw().span(), None);
    }

    #[test]
    fn new_unchecked_span_range() {
        let raw = RawString::with_span(5..10);
        let repr = Repr::new_unchecked(raw);
        assert!(repr.as_raw().as_str().is_none());
        assert_eq!(repr.as_raw().span(), Some(5..10));
    }
}#[cfg(test)]
mod tests_llm_16_463 {
    use super::*;

use crate::*;

    #[test]
    fn test_span_with_explicit_string() {
        let raw_string = RawString::from(InternalString::from("value"));
        let repr = Repr::new_unchecked(raw_string);
        assert_eq!(repr.span(), None);
    }

    #[test]
    fn test_span_with_spanned_string() {
        let raw_string = RawString::with_span(5..10);
        let repr = Repr::new_unchecked(raw_string);
        assert_eq!(repr.span(), Some(5..10));
    }

    #[test]
    fn test_span_with_empty_string() {
        let raw_string = RawString::from(InternalString::from(""));
        let repr = Repr::new_unchecked(raw_string);
        assert_eq!(repr.span(), None);
    }
}