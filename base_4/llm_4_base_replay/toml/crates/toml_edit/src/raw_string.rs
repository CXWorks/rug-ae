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
            RawStringInner::Spanned(span) => {
                input
                    .get(span.clone())
                    .unwrap_or_else(|| {
                        panic!(
                            "span {:?} should be in input:\n```\n{}\n```", span, input
                        )
                    })
            }
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
                    input
                        .get(span.clone())
                        .unwrap_or_else(|| {
                            panic!(
                                "span {:?} should be in input:\n```\n{}\n```", span, input
                            )
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
                *self = Self::from(
                    input
                        .get(span.clone())
                        .unwrap_or_else(|| {
                            panic!(
                                "span {:?} should be in input:\n```\n{}\n```", span, input
                            )
                        }),
                );
            }
        }
    }
    pub(crate) fn encode(
        &self,
        buf: &mut dyn std::fmt::Write,
        input: &str,
    ) -> std::fmt::Result {
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
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
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
        let _rug_st_tests_llm_16_85_rrrruuuugggg_test_from_empty_internal_string = 0;
        let internal_str = InternalString::new();
        let raw_string = RawString::from(&internal_str);
        debug_assert_eq!(raw_string.as_str(), Some(""));
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_test_from_empty_internal_string = 0;
    }
    #[test]
    fn test_from_non_empty_internal_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let internal_str = InternalString::from(rug_fuzz_0);
        let raw_string = RawString::from(&internal_str);
        debug_assert_eq!(raw_string.as_str(), Some("test"));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_86 {
    use super::*;
    use crate::*;
    use std::string::String;
    #[test]
    fn test_from_empty_string_creates_empty_raw_string() {
        let _rug_st_tests_llm_16_86_rrrruuuugggg_test_from_empty_string_creates_empty_raw_string = 0;
        let empty_string = String::new();
        let raw_string = RawString::from(&empty_string);
        debug_assert_eq!(RawString(RawStringInner::Empty), raw_string);
        let _rug_ed_tests_llm_16_86_rrrruuuugggg_test_from_empty_string_creates_empty_raw_string = 0;
    }
    #[test]
    fn test_from_non_empty_string_creates_explicit_raw_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_empty_string = String::from(rug_fuzz_0);
        let raw_string = RawString::from(&non_empty_string);
        debug_assert!(matches!(raw_string, RawString(RawStringInner::Explicit(_))));
             }
}
}
}    }
    #[test]
    fn test_from_non_empty_string_content() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_empty_string = String::from(rug_fuzz_0);
        let raw_string = RawString::from(&non_empty_string);
        if let RawString(RawStringInner::Explicit(internal_string)) = raw_string {
            debug_assert_eq!(non_empty_string, internal_string.as_ref());
        } else {
            panic!("RawString was not explicit as expected");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_87 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;
    use std::convert::From;
    #[test]
    fn test_from_empty_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let empty_string = rug_fuzz_0;
        let raw_string = RawString::from(empty_string);
        matches!(raw_string.0, RawStringInner::Empty);
             }
}
}
}    }
    #[test]
    fn test_from_non_empty_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_empty_string = rug_fuzz_0;
        let raw_string = RawString::from(non_empty_string);
        match raw_string.0 {
            RawStringInner::Explicit(internal_string) => {
                debug_assert_eq!(internal_string.as_str(), non_empty_string);
            }
            _ => panic!("Expected RawStringInner::Explicit"),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_88 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;
    #[test]
    fn from_internal_string_empty() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_from_internal_string_empty = 0;
        let empty_internal_string = InternalString::new();
        let raw_string = RawString::from(empty_internal_string);
        debug_assert_eq!(
            raw_string, RawString(RawStringInner::Explicit(InternalString::new()))
        );
        debug_assert_eq!(raw_string.as_str(), Some(""));
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_from_internal_string_empty = 0;
    }
    #[test]
    fn from_internal_string_non_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_empty_internal_string = InternalString::from(rug_fuzz_0);
        let raw_string = RawString::from(non_empty_internal_string.clone());
        debug_assert_eq!(
            raw_string, RawString(RawStringInner::Explicit(non_empty_internal_string))
        );
        debug_assert_eq!(raw_string.as_str(), Some("non-empty"));
             }
}
}
}    }
    #[test]
    fn internal_string_debug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let internal_string = InternalString::from(rug_fuzz_0);
        let debug_string = format!("{:?}", internal_string);
        debug_assert_eq!(debug_string, "\"test\"");
             }
}
}
}    }
    #[test]
    fn raw_string_debug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string_explicit = RawString::from(InternalString::from(rug_fuzz_0));
        let debug_string = format!("{:?}", raw_string_explicit);
        debug_assert_eq!(debug_string.contains(rug_fuzz_1), true);
             }
}
}
}    }
    #[test]
    fn raw_string_empty_debug() {
        let _rug_st_tests_llm_16_88_rrrruuuugggg_raw_string_empty_debug = 0;
        let raw_string_empty = RawString::from(InternalString::new());
        let debug_string = format!("{:?}", raw_string_empty);
        debug_assert_eq!(debug_string, "empty");
        let _rug_ed_tests_llm_16_88_rrrruuuugggg_raw_string_empty_debug = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use super::*;
    use crate::*;
    #[test]
    fn from_empty_box_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s: Box<str> = rug_fuzz_0.into();
        let raw_string = RawString::from(s);
        debug_assert!(matches!(raw_string, RawString(RawStringInner::Empty)));
        debug_assert_eq!(raw_string.as_str(), Some(""));
             }
}
}
}    }
    #[test]
    fn from_non_empty_box_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let s: Box<str> = rug_fuzz_0.into();
        let raw_string = RawString::from(s);
        debug_assert!(matches!(raw_string, RawString(RawStringInner::Explicit(_))));
        debug_assert_eq!(raw_string.as_str(), Some("hello"));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_90_llm_16_90 {
    use super::*;
    use crate::*;
    use crate::raw_string::RawStringInner;
    #[test]
    fn from_empty_string() {
        let _rug_st_tests_llm_16_90_llm_16_90_rrrruuuugggg_from_empty_string = 0;
        let empty_string = String::new();
        let raw_string = RawString::from(empty_string);
        debug_assert!(matches!(raw_string.0, RawStringInner::Empty));
        let _rug_ed_tests_llm_16_90_llm_16_90_rrrruuuugggg_from_empty_string = 0;
    }
    #[test]
    fn from_non_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_empty_string = String::from(rug_fuzz_0);
        let raw_string = RawString::from(non_empty_string);
        debug_assert!(matches!(raw_string.0, RawStringInner::Explicit(_)));
             }
}
}
}    }
    #[test]
    fn from_string_preserves_content() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let content = rug_fuzz_0;
        let string = String::from(content);
        let raw_string = RawString::from(string);
        match raw_string.0 {
            RawStringInner::Explicit(internal_string) => {
                debug_assert_eq!(internal_string.as_str(), content)
            }
            _ => panic!("Expected RawStringInner::Explicit, found other variant"),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_91 {
    use crate::raw_string::RawString;
    #[test]
    fn test_raw_string_default() {
        let _rug_st_tests_llm_16_91_rrrruuuugggg_test_raw_string_default = 0;
        let raw_string = RawString::default();
        debug_assert!(raw_string.as_str().unwrap().is_empty());
        let _rug_ed_tests_llm_16_91_rrrruuuugggg_test_raw_string_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_431_llm_16_431 {
    use crate::{InternalString, RawString};
    #[test]
    fn test_as_str_for_empty_raw_string() {
        let _rug_st_tests_llm_16_431_llm_16_431_rrrruuuugggg_test_as_str_for_empty_raw_string = 0;
        let raw_string = RawString::default();
        debug_assert_eq!(raw_string.as_str(), Some(""));
        let _rug_ed_tests_llm_16_431_llm_16_431_rrrruuuugggg_test_as_str_for_empty_raw_string = 0;
    }
    #[test]
    fn test_as_str_for_explicit_raw_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let intern_str = InternalString::from(rug_fuzz_0);
        let raw_string = RawString::from(intern_str);
        debug_assert_eq!(raw_string.as_str(), Some("test"));
             }
}
}
}    }
    #[test]
    fn test_as_str_for_spanned_raw_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let span = rug_fuzz_0..rug_fuzz_1;
        let raw_string = RawString::with_span(span);
        debug_assert_eq!(raw_string.as_str(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_432 {
    use crate::raw_string::{RawString, RawStringInner};
    #[test]
    fn despan_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut raw_string = RawString::default();
        let input = rug_fuzz_0;
        raw_string.despan(input);
        debug_assert_eq!(raw_string, RawString::default());
             }
}
}
}    }
    #[test]
    fn despan_explicit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut raw_string = RawString::from(rug_fuzz_0);
        let input = rug_fuzz_1;
        raw_string.despan(input);
        debug_assert_eq!(raw_string, RawString::from("explicit"));
             }
}
}
}    }
    #[test]
    fn despan_spanned_in_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut raw_string = RawString::with_span(rug_fuzz_0..rug_fuzz_1);
        let input = rug_fuzz_2;
        raw_string.despan(input);
        debug_assert_eq!(raw_string, RawString::from("test"));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "span 0..14 should be in input")]
    fn despan_spanned_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut raw_string = RawString::with_span(rug_fuzz_0..rug_fuzz_1);
        let input = rug_fuzz_2;
        raw_string.despan(input);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "span 5..10 should be in input")]
    fn despan_spanned_empty_in_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut raw_string = RawString::with_span(rug_fuzz_0..rug_fuzz_1);
        let input = rug_fuzz_2;
        raw_string.despan(input);
             }
}
}
}    }
}
#[cfg(test)]
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
        let span = 10..17;
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
}
#[cfg(test)]
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
}
#[cfg(test)]
mod tests_llm_16_435 {
    use crate::raw_string::{RawString, RawStringInner};
    use crate::internal_string::InternalString;
    use std::ops::Range;
    #[test]
    fn span_empty() {
        let _rug_st_tests_llm_16_435_rrrruuuugggg_span_empty = 0;
        let rs = RawString(RawStringInner::Empty);
        debug_assert_eq!(rs.span(), None);
        let _rug_ed_tests_llm_16_435_rrrruuuugggg_span_empty = 0;
    }
    #[test]
    fn span_explicit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let is = InternalString::from(rug_fuzz_0);
        let rs = RawString(RawStringInner::Explicit(is));
        debug_assert_eq!(rs.span(), None);
             }
}
}
}    }
    #[test]
    fn span_spanned() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let span = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let rs = RawString(RawStringInner::Spanned(span.clone()));
        debug_assert_eq!(rs.span(), Some(span));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_436 {
    use crate::raw_string::{RawString, RawStringInner};
    use std::ops::Range;
    #[test]
    fn to_str_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string = RawString(RawStringInner::Empty);
        let input = rug_fuzz_0;
        let result = raw_string.to_str(input);
        debug_assert_eq!(result, "");
             }
}
}
}    }
    #[test]
    fn to_str_explicit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string = RawString(RawStringInner::Explicit(rug_fuzz_0.into()));
        let input = rug_fuzz_1;
        let result = raw_string.to_str(input);
        debug_assert_eq!(result, "explicit_string");
             }
}
}
}    }
    #[test]
    fn to_str_spanned_within_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string = RawString(
            RawStringInner::Spanned(Range {
                start: rug_fuzz_0,
                end: rug_fuzz_1,
            }),
        );
        let input = rug_fuzz_2;
        let result = raw_string.to_str(input);
        debug_assert_eq!(result, "input");
             }
}
}
}    }
    #[test]
    #[should_panic(
        expected = "span StartEnd { start: 0, end: 100 } should be in input:"
    )]
    fn to_str_spanned_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string = RawString(
            RawStringInner::Spanned(Range {
                start: rug_fuzz_0,
                end: rug_fuzz_1,
            }),
        );
        let input = rug_fuzz_2;
        let _result = raw_string.to_str(input);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_437 {
    use crate::RawString;
    use std::ops::Range;
    #[test]
    fn to_str_with_default_empty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string = RawString::default();
        debug_assert_eq!(
            raw_string.to_str_with_default(Some(rug_fuzz_0), rug_fuzz_1), ""
        );
        debug_assert_eq!(raw_string.to_str_with_default(None, rug_fuzz_2), "");
             }
}
}
}    }
    #[test]
    fn to_str_with_default_explicit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_string = RawString::from(rug_fuzz_0);
        debug_assert_eq!(
            raw_string.to_str_with_default(Some(rug_fuzz_1), rug_fuzz_2), "explicit"
        );
        debug_assert_eq!(raw_string.to_str_with_default(None, rug_fuzz_3), "explicit");
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn to_str_with_default_span_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let span = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let raw_string = RawString::with_span(span);
        raw_string.to_str_with_default(Some(rug_fuzz_2), rug_fuzz_3);
             }
}
}
}    }
    #[test]
    fn to_str_with_default_span_within_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let span = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let input = rug_fuzz_2;
        let raw_string = RawString::with_span(span);
        debug_assert_eq!(
            raw_string.to_str_with_default(Some(input), rug_fuzz_3), "67890"
        );
             }
}
}
}    }
    #[test]
    fn to_str_with_default_span_with_default() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let span = Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let raw_string = RawString::with_span(span);
        debug_assert_eq!(raw_string.to_str_with_default(None, rug_fuzz_2), "default");
             }
}
}
}    }
}
