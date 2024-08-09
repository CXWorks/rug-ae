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
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
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
        Repr { raw_value: raw.into() }
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
    pub(crate) fn encode(
        &self,
        buf: &mut dyn std::fmt::Write,
        input: &str,
    ) -> std::fmt::Result {
        self.as_raw().encode(buf, input)
    }
}
impl std::fmt::Debug for Repr {
    #[inline]
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
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
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
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
        let _rug_st_tests_llm_16_439_rrrruuuugggg_clear_resets_decor_to_default = 0;
        let rug_fuzz_0 = "prefix";
        let rug_fuzz_1 = "suffix";
        let prefix = RawString::from(rug_fuzz_0);
        let suffix = RawString::from(rug_fuzz_1);
        let mut decor = Decor::new(prefix, suffix);
        decor.clear();
        debug_assert_eq!(decor.prefix(), None);
        debug_assert_eq!(decor.suffix(), None);
        let _rug_ed_tests_llm_16_439_rrrruuuugggg_clear_resets_decor_to_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_440_llm_16_440 {
    use crate::Decor;
    use crate::repr::RawString;
    #[test]
    fn test_despan() {
        let _rug_st_tests_llm_16_440_llm_16_440_rrrruuuugggg_test_despan = 0;
        let rug_fuzz_0 = "/* My Prefix */";
        let rug_fuzz_1 = "/* My Suffix */";
        let rug_fuzz_2 = "/* My Prefix */some_value/* My Suffix */";
        let mut decor = Decor::new(
            RawString::from(rug_fuzz_0),
            RawString::from(rug_fuzz_1),
        );
        let input = rug_fuzz_2;
        decor.despan(input);
        debug_assert!(decor.prefix().is_none());
        debug_assert!(decor.suffix().is_none());
        let _rug_ed_tests_llm_16_440_llm_16_440_rrrruuuugggg_test_despan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_441 {
    use super::*;
    use crate::*;
    use crate::repr::RawString;
    #[test]
    fn test_decor_new() {
        let _rug_st_tests_llm_16_441_rrrruuuugggg_test_decor_new = 0;
        let rug_fuzz_0 = "prefix";
        let rug_fuzz_1 = "suffix";
        let prefix = rug_fuzz_0;
        let suffix = rug_fuzz_1;
        let decor = Decor::new(prefix, suffix);
        debug_assert_eq!(decor.prefix(), Some(& RawString::from(prefix)));
        debug_assert_eq!(decor.suffix(), Some(& RawString::from(suffix)));
        let _rug_ed_tests_llm_16_441_rrrruuuugggg_test_decor_new = 0;
    }
    #[test]
    fn test_decor_new_empty() {
        let _rug_st_tests_llm_16_441_rrrruuuugggg_test_decor_new_empty = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        let prefix = rug_fuzz_0;
        let suffix = rug_fuzz_1;
        let decor = Decor::new(prefix, suffix);
        debug_assert_eq!(decor.prefix(), Some(& RawString::from(prefix)));
        debug_assert_eq!(decor.suffix(), Some(& RawString::from(suffix)));
        let _rug_ed_tests_llm_16_441_rrrruuuugggg_test_decor_new_empty = 0;
    }
    #[test]
    fn test_decor_new_with_spaces() {
        let _rug_st_tests_llm_16_441_rrrruuuugggg_test_decor_new_with_spaces = 0;
        let rug_fuzz_0 = "    ";
        let rug_fuzz_1 = "  ";
        let prefix = rug_fuzz_0;
        let suffix = rug_fuzz_1;
        let decor = Decor::new(prefix, suffix);
        debug_assert_eq!(decor.prefix(), Some(& RawString::from(prefix)));
        debug_assert_eq!(decor.suffix(), Some(& RawString::from(suffix)));
        let _rug_ed_tests_llm_16_441_rrrruuuugggg_test_decor_new_with_spaces = 0;
    }
    #[test]
    fn test_decor_new_with_newlines() {
        let _rug_st_tests_llm_16_441_rrrruuuugggg_test_decor_new_with_newlines = 0;
        let rug_fuzz_0 = "\n\n\n";
        let rug_fuzz_1 = "\n\n";
        let prefix = rug_fuzz_0;
        let suffix = rug_fuzz_1;
        let decor = Decor::new(prefix, suffix);
        debug_assert_eq!(decor.prefix(), Some(& RawString::from(prefix)));
        debug_assert_eq!(decor.suffix(), Some(& RawString::from(suffix)));
        let _rug_ed_tests_llm_16_441_rrrruuuugggg_test_decor_new_with_newlines = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_442 {
    use crate::repr::{Decor, RawString};
    #[test]
    fn test_prefix_with_no_prefix_set() {
        let _rug_st_tests_llm_16_442_rrrruuuugggg_test_prefix_with_no_prefix_set = 0;
        let decor = Decor::default();
        debug_assert_eq!(decor.prefix(), None);
        let _rug_ed_tests_llm_16_442_rrrruuuugggg_test_prefix_with_no_prefix_set = 0;
    }
    #[test]
    fn test_prefix_with_prefix_set() {
        let _rug_st_tests_llm_16_442_rrrruuuugggg_test_prefix_with_prefix_set = 0;
        let rug_fuzz_0 = "## ";
        let prefix = rug_fuzz_0;
        let mut decor = Decor::default();
        decor.set_prefix(prefix);
        debug_assert_eq!(decor.prefix(), Some(& RawString::from(prefix)));
        let _rug_ed_tests_llm_16_442_rrrruuuugggg_test_prefix_with_prefix_set = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_443 {
    use crate::Decor;
    use std::fmt::Write;
    #[test]
    fn test_prefix_encode_with_prefix() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_with_prefix = 0;
        let rug_fuzz_0 = "prefix_";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = "default";
        let rug_fuzz_3 = "input";
        let rug_fuzz_4 = "default";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, rug_fuzz_2).unwrap();
        debug_assert_eq!(buffer, "prefix_default");
        buffer.clear();
        decor.prefix_encode(&mut buffer, Some(rug_fuzz_3), rug_fuzz_4).unwrap();
        debug_assert_eq!(buffer, "prefix_input");
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_with_prefix = 0;
    }
    #[test]
    fn test_prefix_encode_without_prefix() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_without_prefix = 0;
        let rug_fuzz_0 = "default";
        let rug_fuzz_1 = "input";
        let rug_fuzz_2 = "default";
        let mut decor = Decor::default();
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, rug_fuzz_0).unwrap();
        debug_assert_eq!(buffer, "default");
        buffer.clear();
        decor.prefix_encode(&mut buffer, Some(rug_fuzz_1), rug_fuzz_2).unwrap();
        debug_assert_eq!(buffer, "default");
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_without_prefix = 0;
    }
    #[test]
    fn test_prefix_encode_with_cleared_prefix() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_with_cleared_prefix = 0;
        let rug_fuzz_0 = "prefix_";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = "default";
        let rug_fuzz_3 = "input";
        let rug_fuzz_4 = "default";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        decor.clear();
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, rug_fuzz_2).unwrap();
        debug_assert_eq!(buffer, "default");
        buffer.clear();
        decor.prefix_encode(&mut buffer, Some(rug_fuzz_3), rug_fuzz_4).unwrap();
        debug_assert_eq!(buffer, "default");
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_with_cleared_prefix = 0;
    }
    #[test]
    fn test_prefix_encode_set_prefix() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_set_prefix = 0;
        let rug_fuzz_0 = "new_prefix_";
        let rug_fuzz_1 = "default";
        let rug_fuzz_2 = "input";
        let rug_fuzz_3 = "default";
        let mut decor = Decor::default();
        decor.set_prefix(rug_fuzz_0);
        let mut buffer = String::new();
        decor.prefix_encode(&mut buffer, None, rug_fuzz_1).unwrap();
        debug_assert_eq!(buffer, "new_prefix_default");
        buffer.clear();
        decor.prefix_encode(&mut buffer, Some(rug_fuzz_2), rug_fuzz_3).unwrap();
        debug_assert_eq!(buffer, "new_prefix_input");
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_prefix_encode_set_prefix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_444 {
    use super::*;
    use crate::*;
    use crate::repr::Decor;
    #[test]
    fn test_set_prefix() {
        let _rug_st_tests_llm_16_444_rrrruuuugggg_test_set_prefix = 0;
        let rug_fuzz_0 = "## ";
        let mut decor = Decor::default();
        let new_prefix = rug_fuzz_0;
        decor.set_prefix(new_prefix.to_string());
        debug_assert_eq!(decor.prefix(), Some(& new_prefix.into()));
        let _rug_ed_tests_llm_16_444_rrrruuuugggg_test_set_prefix = 0;
    }
    #[test]
    fn test_set_prefix_clear_and_set() {
        let _rug_st_tests_llm_16_444_rrrruuuugggg_test_set_prefix_clear_and_set = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = "## ";
        let mut decor = Decor::default();
        decor.set_prefix(rug_fuzz_0);
        decor.clear();
        debug_assert_eq!(decor.prefix(), None);
        let new_prefix = rug_fuzz_1;
        decor.set_prefix(new_prefix.to_string());
        debug_assert_eq!(decor.prefix(), Some(& new_prefix.into()));
        let _rug_ed_tests_llm_16_444_rrrruuuugggg_test_set_prefix_clear_and_set = 0;
    }
    #[test]
    fn test_set_prefix_overwrite_existing() {
        let _rug_st_tests_llm_16_444_rrrruuuugggg_test_set_prefix_overwrite_existing = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = "## ";
        let mut decor = Decor::default();
        decor.set_prefix(rug_fuzz_0);
        let new_prefix = rug_fuzz_1;
        decor.set_prefix(new_prefix.to_string());
        debug_assert_eq!(decor.prefix(), Some(& new_prefix.into()));
        let _rug_ed_tests_llm_16_444_rrrruuuugggg_test_set_prefix_overwrite_existing = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_445 {
    use super::*;
    use crate::*;
    use crate::repr::Decor;
    use crate::repr::RawString;
    #[test]
    fn test_set_suffix() {
        let _rug_st_tests_llm_16_445_rrrruuuugggg_test_set_suffix = 0;
        let rug_fuzz_0 = " # This is a suffix";
        let mut decor = Decor::default();
        debug_assert_eq!(decor.suffix(), None);
        let new_suffix = rug_fuzz_0;
        decor.set_suffix(new_suffix);
        debug_assert_eq!(
            decor.suffix(), Some(& RawString::from(new_suffix.to_string()))
        );
        let _rug_ed_tests_llm_16_445_rrrruuuugggg_test_set_suffix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_446 {
    use crate::repr::{Decor, RawString};
    #[test]
    fn suffix_none_when_decor_is_default() {
        let _rug_st_tests_llm_16_446_rrrruuuugggg_suffix_none_when_decor_is_default = 0;
        let decor = Decor::default();
        debug_assert_eq!(decor.suffix(), None);
        let _rug_ed_tests_llm_16_446_rrrruuuugggg_suffix_none_when_decor_is_default = 0;
    }
    #[test]
    fn suffix_returns_suffix_when_set() {
        let _rug_st_tests_llm_16_446_rrrruuuugggg_suffix_returns_suffix_when_set = 0;
        let rug_fuzz_0 = " # suffix";
        let mut decor = Decor::default();
        let suffix = RawString::from(rug_fuzz_0);
        decor.set_suffix(suffix.clone());
        debug_assert_eq!(decor.suffix(), Some(& suffix));
        let _rug_ed_tests_llm_16_446_rrrruuuugggg_suffix_returns_suffix_when_set = 0;
    }
    #[test]
    fn suffix_none_after_clear() {
        let _rug_st_tests_llm_16_446_rrrruuuugggg_suffix_none_after_clear = 0;
        let rug_fuzz_0 = " # prefix";
        let rug_fuzz_1 = " # suffix";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        decor.clear();
        debug_assert_eq!(decor.suffix(), None);
        let _rug_ed_tests_llm_16_446_rrrruuuugggg_suffix_none_after_clear = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_447 {
    use crate::Decor;
    use crate::repr::RawString;
    use std::fmt::Write;
    #[test]
    fn test_suffix_encode_with_suffix() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_suffix = 0;
        let rug_fuzz_0 = "prefix";
        let rug_fuzz_1 = "suffix";
        let rug_fuzz_2 = "default_suffix";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        let mut output = String::new();
        decor.suffix_encode(&mut output, None, rug_fuzz_2).unwrap();
        debug_assert_eq!(output, "suffix");
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_suffix = 0;
    }
    #[test]
    fn test_suffix_encode_without_suffix() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_without_suffix = 0;
        let rug_fuzz_0 = "default_suffix";
        let mut decor = Decor::default();
        let mut output = String::new();
        decor.suffix_encode(&mut output, None, rug_fuzz_0).unwrap();
        debug_assert_eq!(output, "default_suffix");
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_without_suffix = 0;
    }
    #[test]
    fn test_suffix_encode_with_input() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_input = 0;
        let rug_fuzz_0 = "prefix";
        let rug_fuzz_1 = "suffix";
        let rug_fuzz_2 = "input_suffix";
        let rug_fuzz_3 = "default_suffix";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        let mut output = String::new();
        decor.suffix_encode(&mut output, Some(rug_fuzz_2), rug_fuzz_3).unwrap();
        debug_assert_eq!(output, "input_suffix");
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_input = 0;
    }
    #[test]
    fn test_suffix_encode_without_suffix_with_input() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_without_suffix_with_input = 0;
        let rug_fuzz_0 = "input_suffix";
        let rug_fuzz_1 = "default_suffix";
        let mut decor = Decor::default();
        let mut output = String::new();
        decor.suffix_encode(&mut output, Some(rug_fuzz_0), rug_fuzz_1).unwrap();
        debug_assert_eq!(output, "input_suffix");
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_without_suffix_with_input = 0;
    }
    #[test]
    fn test_suffix_encode_with_empty_suffix() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_empty_suffix = 0;
        let rug_fuzz_0 = "prefix";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = "default_suffix";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        let mut output = String::new();
        decor.suffix_encode(&mut output, None, rug_fuzz_2).unwrap();
        debug_assert_eq!(output, "");
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_empty_suffix = 0;
    }
    #[test]
    fn test_suffix_encode_with_empty_suffix_and_input() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_empty_suffix_and_input = 0;
        let rug_fuzz_0 = "prefix";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = "input_suffix";
        let rug_fuzz_3 = "default_suffix";
        let mut decor = Decor::new(rug_fuzz_0, rug_fuzz_1);
        let mut output = String::new();
        decor.suffix_encode(&mut output, Some(rug_fuzz_2), rug_fuzz_3).unwrap();
        debug_assert_eq!(output, "input_suffix");
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_suffix_encode_with_empty_suffix_and_input = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_451 {
    use super::*;
    use crate::*;
    use crate::repr::{Decor, Formatted, Repr, ValueRepr};
    use crate::raw_string::RawString;
    use std::borrow::Cow;
    use std::fmt::Write;
    #[test]
    fn test_default_repr() {
        let _rug_st_tests_llm_16_451_rrrruuuugggg_test_default_repr = 0;
        let rug_fuzz_0 = "test_value";
        let value = rug_fuzz_0.to_string();
        let formatted_value = Formatted::new(value.clone());
        let default_repr = formatted_value.default_repr();
        let mut buf = String::new();
        default_repr.encode(&mut buf, &value).unwrap();
        let expected_repr = value.clone();
        debug_assert_eq!(
            Cow::Borrowed(expected_repr.as_str()), formatted_value.display_repr()
        );
        debug_assert_eq!(
            expected_repr, buf, "Encoded representation should match the expected value"
        );
        let _rug_ed_tests_llm_16_451_rrrruuuugggg_test_default_repr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_455 {
    use crate::Formatted;
    use crate::repr::Decor;
    #[test]
    fn test_new_formatted() {
        let _rug_st_tests_llm_16_455_rrrruuuugggg_test_new_formatted = 0;
        let rug_fuzz_0 = 42;
        let value = rug_fuzz_0;
        let formatted = Formatted::new(value);
        debug_assert_eq!(formatted.value(), & value);
        debug_assert!(formatted.as_repr().is_none());
        debug_assert_eq!(formatted.decor(), & Decor::default());
        let _rug_ed_tests_llm_16_455_rrrruuuugggg_test_new_formatted = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_458 {
    use super::*;
    use crate::*;
    use crate::repr::{Decor, Formatted, RawString};
    use std::default::Default;
    #[test]
    fn test_value() {
        let _rug_st_tests_llm_16_458_rrrruuuugggg_test_value = 0;
        let rug_fuzz_0 = 42;
        let formatted = Formatted {
            value: rug_fuzz_0,
            repr: None,
            decor: Decor::default(),
        };
        debug_assert_eq!(* formatted.value(), 42);
        let _rug_ed_tests_llm_16_458_rrrruuuugggg_test_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_459 {
    use super::*;
    use crate::*;
    use crate::repr::Repr;
    use crate::raw_string::RawString;
    use crate::internal_string::InternalString;
    #[test]
    fn test_as_raw_empty() {
        let _rug_st_tests_llm_16_459_rrrruuuugggg_test_as_raw_empty = 0;
        let rug_fuzz_0 = "";
        let empty_internal = InternalString::from(rug_fuzz_0);
        let empty_raw = RawString::from(&empty_internal);
        let repr = Repr::new_unchecked(empty_raw.clone());
        debug_assert_eq!(repr.as_raw(), & empty_raw);
        let _rug_ed_tests_llm_16_459_rrrruuuugggg_test_as_raw_empty = 0;
    }
    #[test]
    fn test_as_raw_explicit() {
        let _rug_st_tests_llm_16_459_rrrruuuugggg_test_as_raw_explicit = 0;
        let rug_fuzz_0 = "value";
        let explicit_internal = InternalString::from(rug_fuzz_0);
        let explicit_raw = RawString::from(explicit_internal);
        let repr = Repr::new_unchecked(explicit_raw.clone());
        debug_assert_eq!(repr.as_raw(), & explicit_raw);
        let _rug_ed_tests_llm_16_459_rrrruuuugggg_test_as_raw_explicit = 0;
    }
    #[test]
    fn test_as_raw_spanned() {
        let _rug_st_tests_llm_16_459_rrrruuuugggg_test_as_raw_spanned = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let span = std::ops::Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let spanned_raw = RawString::with_span(span);
        let repr = Repr::new_unchecked(spanned_raw.clone());
        debug_assert_eq!(repr.as_raw(), & spanned_raw);
        let _rug_ed_tests_llm_16_459_rrrruuuugggg_test_as_raw_spanned = 0;
    }
}
#[cfg(test)]
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
}
#[cfg(test)]
mod tests_llm_16_461 {
    use super::*;
    use crate::*;
    #[test]
    fn test_encode() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = "example input";
        let input_str = rug_fuzz_0;
        let mut buffer = String::new();
        let repr = Repr::new_unchecked(input_str);
        let encode_result = repr.encode(&mut buffer, input_str);
        debug_assert!(encode_result.is_ok());
        debug_assert_eq!(buffer, "example input");
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_462 {
    use super::*;
    use crate::*;
    #[test]
    fn new_unchecked_empty_string() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_new_unchecked_empty_string = 0;
        let rug_fuzz_0 = "";
        let raw = RawString::from(rug_fuzz_0);
        let repr = Repr::new_unchecked(raw);
        debug_assert!(repr.as_raw().as_str().is_some());
        debug_assert_eq!(repr.as_raw().as_str().unwrap(), "");
        debug_assert_eq!(repr.as_raw().span(), None);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_new_unchecked_empty_string = 0;
    }
    #[test]
    fn new_unchecked_non_empty_string() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_new_unchecked_non_empty_string = 0;
        let rug_fuzz_0 = "value";
        let raw = RawString::from(rug_fuzz_0);
        let repr = Repr::new_unchecked(raw);
        debug_assert!(repr.as_raw().as_str().is_some());
        debug_assert_eq!(repr.as_raw().as_str().unwrap(), "value");
        debug_assert_eq!(repr.as_raw().span(), None);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_new_unchecked_non_empty_string = 0;
    }
    #[test]
    fn new_unchecked_internal_string() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_new_unchecked_internal_string = 0;
        let rug_fuzz_0 = "value";
        let internal = InternalString::from(rug_fuzz_0);
        let raw = RawString::from(internal);
        let repr = Repr::new_unchecked(raw);
        debug_assert!(repr.as_raw().as_str().is_some());
        debug_assert_eq!(repr.as_raw().as_str().unwrap(), "value");
        debug_assert_eq!(repr.as_raw().span(), None);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_new_unchecked_internal_string = 0;
    }
    #[test]
    fn new_unchecked_span_range() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_new_unchecked_span_range = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let raw = RawString::with_span(rug_fuzz_0..rug_fuzz_1);
        let repr = Repr::new_unchecked(raw);
        debug_assert!(repr.as_raw().as_str().is_none());
        debug_assert_eq!(repr.as_raw().span(), Some(5..10));
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_new_unchecked_span_range = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_463 {
    use super::*;
    use crate::*;
    #[test]
    fn test_span_with_explicit_string() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_span_with_explicit_string = 0;
        let rug_fuzz_0 = "value";
        let raw_string = RawString::from(InternalString::from(rug_fuzz_0));
        let repr = Repr::new_unchecked(raw_string);
        debug_assert_eq!(repr.span(), None);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_span_with_explicit_string = 0;
    }
    #[test]
    fn test_span_with_spanned_string() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_span_with_spanned_string = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let raw_string = RawString::with_span(rug_fuzz_0..rug_fuzz_1);
        let repr = Repr::new_unchecked(raw_string);
        debug_assert_eq!(repr.span(), Some(5..10));
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_span_with_spanned_string = 0;
    }
    #[test]
    fn test_span_with_empty_string() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_span_with_empty_string = 0;
        let rug_fuzz_0 = "";
        let raw_string = RawString::from(InternalString::from(rug_fuzz_0));
        let repr = Repr::new_unchecked(raw_string);
        debug_assert_eq!(repr.span(), None);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_span_with_empty_string = 0;
    }
}
