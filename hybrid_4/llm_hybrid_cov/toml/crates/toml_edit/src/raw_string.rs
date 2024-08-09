use crate::InternalString;

/// Opaque string storage for raw TOML; internal to `toml_edit`
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct RawString(RawStringInner);

#[derive(PartialEq, Eq, Clone, Hash)]
enum RawStringInner {
    Empty,
    Explicit(InternalString),
    Spanned(std::ops::Range<usize>),
}

impl RawString {
    pub(crate) fn with_span(span: std::ops::Range<usize>) -> Self {
        if span.start == span.end {
            RawString(RawStringInner::Empty)
        } else {
            RawString(RawStringInner::Spanned(span))
        }
    }

    /// Access the underlying string
    pub fn as_str(&self) -> Option<&str> {
        match &self.0 {
            RawStringInner::Empty => Some(""),
            RawStringInner::Explicit(s) => Some(s.as_str()),
            RawStringInner::Spanned(_) => None,
        }
    }

    pub(crate) fn to_str<'s>(&'s self, input: &'s str) -> &'s str {
        match &self.0 {
            RawStringInner::Empty => "",
            RawStringInner::Explicit(s) => s.as_str(),
            RawStringInner::Spanned(span) => input.get(span.clone()).unwrap_or_else(|| {
                panic!("span {:?} should be in input:\n```\n{}\n```", span, input)
            }),
        }
    }

    pub(crate) fn to_str_with_default<'s>(
        &'s self,
        input: Option<&'s str>,
        default: &'s str,
    ) -> &'s str {
        match &self.0 {
            RawStringInner::Empty => "",
            RawStringInner::Explicit(s) => s.as_str(),
            RawStringInner::Spanned(span) => {
                if let Some(input) = input {
                    input.get(span.clone()).unwrap_or_else(|| {
                        panic!("span {:?} should be in input:\n```\n{}\n```", span, input)
                    })
                } else {
                    default
                }
            }
        }
    }

    /// Access the underlying span
    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        match &self.0 {
            RawStringInner::Empty => None,
            RawStringInner::Explicit(_) => None,
            RawStringInner::Spanned(span) => Some(span.clone()),
        }
    }

    pub(crate) fn despan(&mut self, input: &str) {
        match &self.0 {
            RawStringInner::Empty => {}
            RawStringInner::Explicit(_) => {}
            RawStringInner::Spanned(span) => {
                *self = Self::from(input.get(span.clone()).unwrap_or_else(|| {
                    panic!("span {:?} should be in input:\n```\n{}\n```", span, input)
                }))
            }
        }
    }

    pub(crate) fn encode(&self, buf: &mut dyn std::fmt::Write, input: &str) -> std::fmt::Result {
        let raw = self.to_str(input);
        for part in raw.split('\r') {
            write!(buf, "{}", part)?;
        }
        Ok(())
    }

    pub(crate) fn encode_with_default(
        &self,
        buf: &mut dyn std::fmt::Write,
        input: Option<&str>,
        default: &str,
    ) -> std::fmt::Result {
        let raw = self.to_str_with_default(input, default);
        for part in raw.split('\r') {
            write!(buf, "{}", part)?;
        }
        Ok(())
    }
}

impl Default for RawString {
    fn default() -> Self {
        Self(RawStringInner::Empty)
    }
}

impl std::fmt::Debug for RawString {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.0 {
            RawStringInner::Empty => write!(formatter, "empty"),
            RawStringInner::Explicit(s) => write!(formatter, "{:?}", s),
            RawStringInner::Spanned(s) => write!(formatter, "{:?}", s),
        }
    }
}

impl From<&str> for RawString {
    #[inline]
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<String> for RawString {
    #[inline]
    fn from(s: String) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<&String> for RawString {
    #[inline]
    fn from(s: &String) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<InternalString> for RawString {
    #[inline]
    fn from(inner: InternalString) -> Self {
        Self(RawStringInner::Explicit(inner))
    }
}

impl From<&InternalString> for RawString {
    #[inline]
    fn from(s: &InternalString) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}

impl From<Box<str>> for RawString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        if s.is_empty() {
            Self(RawStringInner::Empty)
        } else {
            InternalString::from(s).into()
        }
    }
}
#[cfg(test)]
mod tests_llm_16_85 {
    use super::*;

use crate::*;
    use crate::raw_string::RawString;
    use crate::internal_string::InternalString;

    #[test]
    fn test_from_empty_internal_string() {
        let internal_str = InternalString::new();
        let raw_string = RawString::from(&internal_str);
        assert_eq!(raw_string.as_str(), Some(""));
    }

    #[test]
    fn test_from_non_empty_internal_string() {
        let internal_str = InternalString::from("test");
        let raw_string = RawString::from(&internal_str);
        assert_eq!(raw_string.as_str(), Some("test"));
    }
}#[cfg(test)]
mod tests_llm_16_86 {
    use super::*;

use crate::*;
    use std::string::String;

    #[test]
    fn test_from_empty_string_creates_empty_raw_string() {
        let empty_string = String::new();
        let raw_string = RawString::from(&empty_string);
        assert_eq!(RawString(RawStringInner::Empty), raw_string);
    }

    #[test]
    fn test_from_non_empty_string_creates_explicit_raw_string() {
        let non_empty_string = String::from("Hello, world!");
        let raw_string = RawString::from(&non_empty_string);
        assert!(matches!(raw_string, RawString(RawStringInner::Explicit(_))));
    }

    #[test]
    fn test_from_non_empty_string_content() {
        let non_empty_string = String::from("Hello, world!");
        let raw_string = RawString::from(&non_empty_string);
        if let RawString(RawStringInner::Explicit(internal_string)) = raw_string {
            assert_eq!(non_empty_string, internal_string.as_ref());
        } else {
            panic!("RawString was not explicit as expected");
        }
    }
}#[cfg(test)]
mod tests_llm_16_87 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;
    use std::convert::From;

    #[test]
    fn test_from_empty_str() {
        let empty_string = "";
        let raw_string = RawString::from(empty_string);
        matches!(raw_string.0, RawStringInner::Empty);
    }

    #[test]
    fn test_from_non_empty_str() {
        let non_empty_string = "Hello, World!";
        let raw_string = RawString::from(non_empty_string);
        match raw_string.0 {
            RawStringInner::Explicit(internal_string) => {
                assert_eq!(internal_string.as_str(), non_empty_string);
            }
            _ => panic!("Expected RawStringInner::Explicit"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_88 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;

    #[test]
    fn from_internal_string_empty() {
        let empty_internal_string = InternalString::new();
        let raw_string = RawString::from(empty_internal_string);
        assert_eq!(raw_string, RawString(RawStringInner::Explicit(InternalString::new())));
        assert_eq!(raw_string.as_str(), Some(""));
    }

    #[test]
    fn from_internal_string_non_empty() {
        let non_empty_internal_string = InternalString::from("non-empty");
        let raw_string = RawString::from(non_empty_internal_string.clone());
        assert_eq!(raw_string, RawString(RawStringInner::Explicit(non_empty_internal_string)));
        assert_eq!(raw_string.as_str(), Some("non-empty"));
    }

    #[test]
    fn internal_string_debug() {
        let internal_string = InternalString::from("test");
        let debug_string = format!("{:?}", internal_string);
        assert_eq!(debug_string, "\"test\"");
    }

    #[test]
    fn raw_string_debug() {
        let raw_string_explicit = RawString::from(InternalString::from("test"));
        let debug_string = format!("{:?}", raw_string_explicit);
        assert_eq!(debug_string.contains("test"), true);
    }

    #[test]
    fn raw_string_empty_debug() {
        let raw_string_empty = RawString::from(InternalString::new());
        let debug_string = format!("{:?}", raw_string_empty);
        assert_eq!(debug_string, "empty");
    }
}#[cfg(test)]
mod tests_llm_16_89 {
    use super::*;

use crate::*;

    #[test]
    fn from_empty_box_str() {
        let s: Box<str> = "".into();
        let raw_string = RawString::from(s);
        assert!(matches!(raw_string, RawString(RawStringInner::Empty)));
        assert_eq!(raw_string.as_str(), Some(""));
    }

    #[test]
    fn from_non_empty_box_str() {
        let s: Box<str> = "hello".into();
        let raw_string = RawString::from(s);
        assert!(matches!(raw_string, RawString(RawStringInner::Explicit(_))));
        assert_eq!(raw_string.as_str(), Some("hello"));
    }
}#[cfg(test)]
mod tests_llm_16_90_llm_16_90 {
    use super::*;

use crate::*;
    use crate::raw_string::RawStringInner;

    #[test]
    fn from_empty_string() {
        let empty_string = String::new();
        let raw_string = RawString::from(empty_string);
        assert!(matches!(raw_string.0, RawStringInner::Empty));
    }

    #[test]
    fn from_non_empty_string() {
        let non_empty_string = String::from("Test");
        let raw_string = RawString::from(non_empty_string);
        assert!(matches!(raw_string.0, RawStringInner::Explicit(_)));
    }

    #[test]
    fn from_string_preserves_content() {
        let content = "Test content";
        let string = String::from(content);
        let raw_string = RawString::from(string);
        match raw_string.0 {
            RawStringInner::Explicit(internal_string) => assert_eq!(internal_string.as_str(), content),
            _ => panic!("Expected RawStringInner::Explicit, found other variant"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_91 {
    use crate::raw_string::RawString;

    #[test]
    fn test_raw_string_default() {
        let raw_string = RawString::default();
        assert!(raw_string.as_str().unwrap().is_empty());
    }
}#[cfg(test)]
mod tests_llm_16_431_llm_16_431 {
    use crate::{InternalString, RawString};

    #[test]
    fn test_as_str_for_empty_raw_string() {
        let raw_string = RawString::default();
        assert_eq!(raw_string.as_str(), Some(""));
    } 

    #[test]
    fn test_as_str_for_explicit_raw_string() {
        let intern_str = InternalString::from("test");
        let raw_string = RawString::from(intern_str);
        assert_eq!(raw_string.as_str(), Some("test"));
    }

    #[test]
    fn test_as_str_for_spanned_raw_string() {
        let span = 3..8;
        let raw_string = RawString::with_span(span);
        assert_eq!(raw_string.as_str(), None);
    }
}#[cfg(test)]
mod tests_llm_16_432 {
    use crate::raw_string::{RawString, RawStringInner};

    #[test]
    fn despan_empty() {
        let mut raw_string = RawString::default();
        let input = "test input";
        raw_string.despan(input);
        assert_eq!(raw_string, RawString::default());
    }

    #[test]
    fn despan_explicit() {
        let mut raw_string = RawString::from("explicit");
        let input = "test input";
        raw_string.despan(input);
        assert_eq!(raw_string, RawString::from("explicit"));
    }

    #[test]
    fn despan_spanned_in_bounds() {
        let mut raw_string = RawString::with_span(0..4);
        let input = "test input";
        raw_string.despan(input);
        assert_eq!(raw_string, RawString::from("test"));
    }

    #[test]
    #[should_panic(expected = "span 0..14 should be in input")]
    fn despan_spanned_out_of_bounds() {
        let mut raw_string = RawString::with_span(0..14);
        let input = "test input";
        raw_string.despan(input);
    }

    #[test]
    #[should_panic(expected = "span 5..10 should be in input")]
    fn despan_spanned_empty_in_bounds() {
        let mut raw_string = RawString::with_span(5..10);
        let input = "";
        raw_string.despan(input);
    }
}#[cfg(test)]
mod tests_llm_16_433 {
    use super::*;

use crate::*;

    #[test]
    fn test_encode_empty_raw_string() -> std::fmt::Result {
        let raw_string = RawString::default();
        let mut buf = String::new();
        raw_string.encode(&mut buf, "")?;
        assert_eq!(buf, "");
        Ok(())
    }

    #[test]
    fn test_encode_explicit_raw_string() -> std::fmt::Result {
        let raw_string = RawString::from("example");
        let mut buf = String::new();
        raw_string.encode(&mut buf, "example")?;
        assert_eq!(buf, "example");
        Ok(())
    }

    #[test]
    fn test_encode_spanned_raw_string() -> std::fmt::Result {
        let input = "text with example span";
        let span = 10..17; // "example"
        let raw_string = RawString::with_span(span);
        let mut buf = String::new();
        raw_string.encode(&mut buf, input)?;
        assert_eq!(buf, "example");
        Ok(())
    }

    #[test]
    fn test_encode_raw_string_with_carriage_return() -> std::fmt::Result {
        let raw_string = RawString::from("line\r\nbreak");
        let mut buf = String::new();
        raw_string.encode(&mut buf, "line\r\nbreak")?;
        assert_eq!(buf, "line\nbreak");
        Ok(())
    }
}#[cfg(test)]
mod tests_llm_16_434_llm_16_434 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;
    use std::fmt::Write;

    #[test]
    fn test_encode_with_default_empty() {
        let raw_string = RawString(RawStringInner::Empty);
        let mut buffer = String::new();
        raw_string.encode_with_default(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "");
    }

    #[test]
    fn test_encode_with_default_explicit() {
        let internal_string = InternalString::from("explicit");
        let raw_string = RawString(RawStringInner::Explicit(internal_string));
        let mut buffer = String::new();
        raw_string.encode_with_default(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "explicit");
    }

    #[test]
    fn test_encode_with_default_span_with_input() {
        let span = 5..10;
        let raw_string = RawString(RawStringInner::Spanned(span));
        let mut buffer = String::new();
        let input = "Some input string that contains spanned";
        raw_string.encode_with_default(&mut buffer, Some(input), "default").unwrap();
        assert_eq!(buffer, "input")
    }

    #[test]
    fn test_encode_with_default_span_with_default() {
        let span = 5..10;
        let raw_string = RawString(RawStringInner::Spanned(span));
        let mut buffer = String::new();
        raw_string.encode_with_default(&mut buffer, None, "default").unwrap();
        assert_eq!(buffer, "default")
    }

    #[test]
    fn test_encode_with_default_span_with_carriage_return() {
        let span = 0..14;
        let raw_string = RawString(RawStringInner::Spanned(span));
        let mut buffer = String::new();
        let input = "Line1\rLine2\rLine3";
        raw_string.encode_with_default(&mut buffer, Some(input), "default").unwrap();
        assert_eq!(buffer, "Line1Line2Line3")
    }
}#[cfg(test)]
mod tests_llm_16_435 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;
    use std::ops::Range;

    #[test]
    fn span_empty() {
        let rs = RawString(RawStringInner::Empty);
        assert_eq!(rs.span(), None);
    }

    #[test]
    fn span_explicit() {
        let is = InternalString::from("explicit");
        let rs = RawString(RawStringInner::Explicit(is));
        assert_eq!(rs.span(), None);
    }

    #[test]
    fn span_spanned() {
        let span = Range { start: 1, end: 5 };
        let rs = RawString(RawStringInner::Spanned(span.clone()));
        assert_eq!(rs.span(), Some(span));
    }
}#[cfg(test)]
mod tests_llm_16_436 {
    use crate::raw_string::{RawString, RawStringInner};
    use std::ops::Range;

    #[test]
    fn to_str_empty() {
        let raw_string = RawString(RawStringInner::Empty);
        let input = "";
        let result = raw_string.to_str(input);
        assert_eq!(result, "");
    }

    #[test]
    fn to_str_explicit() {
        let raw_string = RawString(RawStringInner::Explicit("explicit_string".into()));
        let input = "irrelevant input";
        let result = raw_string.to_str(input);
        assert_eq!(result, "explicit_string");
    }

    #[test]
    fn to_str_spanned_within_bounds() {
        let raw_string = RawString(RawStringInner::Spanned(Range { start: 5, end: 11 }));
        let input = "Sample input string";
        let result = raw_string.to_str(input);
        assert_eq!(result, "input");
    }

    #[test]
    #[should_panic(expected = "span StartEnd { start: 0, end: 100 } should be in input:")]
    fn to_str_spanned_out_of_bounds() {
        let raw_string = RawString(RawStringInner::Spanned(Range { start: 0, end: 100 }));
        let input = "Short input";
        let _result = raw_string.to_str(input);
    }
}#[cfg(test)]
mod tests_llm_16_437 {
    use crate::RawString;
    use std::ops::Range;

    #[test]
    fn to_str_with_default_empty() {
        let raw_string = RawString::default();
        assert_eq!(raw_string.to_str_with_default(Some("unused"), "default"), "");
        assert_eq!(raw_string.to_str_with_default(None, "default"), "");
    }

    #[test]
    fn to_str_with_default_explicit() {
        let raw_string = RawString::from("explicit");
        assert_eq!(raw_string.to_str_with_default(Some("unused"), "default"), "explicit");
        assert_eq!(raw_string.to_str_with_default(None, "default"), "explicit");
    }

    #[test]
    #[should_panic]
    fn to_str_with_default_span_out_of_bounds() {
        let span = Range { start: 5, end: 10 };
        let raw_string = RawString::with_span(span);
        raw_string.to_str_with_default(Some("short"), "default");
    }

    #[test]
    fn to_str_with_default_span_within_bounds() {
        let span = Range { start: 5, end: 10 };
        let input = "1234567890abcdefghij";
        let raw_string = RawString::with_span(span);
        assert_eq!(raw_string.to_str_with_default(Some(input), "default"), "67890");
    }

    #[test]
    fn to_str_with_default_span_with_default() {
        let span = Range { start: 5, end: 10 };
        let raw_string = RawString::with_span(span);
        assert_eq!(raw_string.to_str_with_default(None, "default"), "default");
    }
}