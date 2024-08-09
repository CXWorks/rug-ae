use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use crate::hir;
/// An inclusive range of codepoints from a generated file (hence the static
/// lifetime).
type Range = &'static [(char, char)];
/// An error that occurs when dealing with Unicode.
///
/// We don't impl the Error trait here because these always get converted
/// into other public errors. (This error type isn't exported.)
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    #[allow(dead_code)]
    PerlClassNotFound,
}
/// An error that occurs when Unicode-aware simple case folding fails.
///
/// This error can occur when the case mapping tables necessary for Unicode
/// aware case folding are unavailable. This only occurs when the
/// `unicode-case` feature is disabled. (The feature is enabled by default.)
#[derive(Debug)]
pub struct CaseFoldError(());
#[cfg(feature = "std")]
impl std::error::Error for CaseFoldError {}
impl core::fmt::Display for CaseFoldError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Unicode-aware case folding is not available \
             (probably because the unicode-case feature is not enabled)"
        )
    }
}
/// An error that occurs when the Unicode-aware `\w` class is unavailable.
///
/// This error can occur when the data tables necessary for the Unicode aware
/// Perl character class `\w` are unavailable. This only occurs when the
/// `unicode-perl` feature is disabled. (The feature is enabled by default.)
#[derive(Debug)]
pub struct UnicodeWordError(());
#[cfg(feature = "std")]
impl std::error::Error for UnicodeWordError {}
impl core::fmt::Display for UnicodeWordError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Unicode-aware \\w class is not available \
             (probably because the unicode-perl feature is not enabled)"
        )
    }
}
/// A state oriented traverser of the simple case folding table.
///
/// A case folder can be constructed via `SimpleCaseFolder::new()`, which will
/// return an error if the underlying case folding table is unavailable.
///
/// After construction, it is expected that callers will use
/// `SimpleCaseFolder::mapping` by calling it with codepoints in strictly
/// increasing order. For example, calling it on `b` and then on `a` is illegal
/// and will result in a panic.
///
/// The main idea of this type is that it tries hard to make mapping lookups
/// fast by exploiting the structure of the underlying table, and the ordering
/// assumption enables this.
#[derive(Debug)]
pub struct SimpleCaseFolder {
    /// The simple case fold table. It's a sorted association list, where the
    /// keys are Unicode scalar values and the values are the corresponding
    /// equivalence class (not including the key) of the "simple" case folded
    /// Unicode scalar values.
    table: &'static [(char, &'static [char])],
    /// The last codepoint that was used for a lookup.
    last: Option<char>,
    /// The index to the entry in `table` corresponding to the smallest key `k`
    /// such that `k > k0`, where `k0` is the most recent key lookup. Note that
    /// in particular, `k0` may not be in the table!
    next: usize,
}
impl SimpleCaseFolder {
    /// Create a new simple case folder, returning an error if the underlying
    /// case folding table is unavailable.
    pub fn new() -> Result<SimpleCaseFolder, CaseFoldError> {
        #[cfg(not(feature = "unicode-case"))] { Err(CaseFoldError(())) }
        #[cfg(feature = "unicode-case")]
        {
            Ok(SimpleCaseFolder {
                table: crate::unicode_tables::case_folding_simple::CASE_FOLDING_SIMPLE,
                last: None,
                next: 0,
            })
        }
    }
    /// Return the equivalence class of case folded codepoints for the given
    /// codepoint. The equivalence class returned never includes the codepoint
    /// given. If the given codepoint has no case folded codepoints (i.e.,
    /// no entry in the underlying case folding table), then this returns an
    /// empty slice.
    ///
    /// # Panics
    ///
    /// This panics when called with a `c` that is less than or equal to the
    /// previous call. In other words, callers need to use this method with
    /// strictly increasing values of `c`.
    pub fn mapping(&mut self, c: char) -> &'static [char] {
        if let Some(last) = self.last {
            assert!(
                last < c,
                "got codepoint U+{:X} which occurs before \
                 last codepoint U+{:X}",
                u32::from(c), u32::from(last),
            );
        }
        self.last = Some(c);
        if self.next >= self.table.len() {
            return &[];
        }
        let (k, v) = self.table[self.next];
        if k == c {
            self.next += 1;
            return v;
        }
        match self.get(c) {
            Err(i) => {
                self.next = i;
                &[]
            }
            Ok(i) => {
                assert!(i > self.next);
                self.next = i + 1;
                self.table[i].1
            }
        }
    }
    /// Returns true if and only if the given range overlaps with any region
    /// of the underlying case folding table. That is, when true, there exists
    /// at least one codepoint in the inclusive range `[start, end]` that has
    /// a non-trivial equivalence class of case folded codepoints. Conversely,
    /// when this returns false, all codepoints in the range `[start, end]`
    /// correspond to the trivial equivalence class of case folded codepoints,
    /// i.e., itself.
    ///
    /// This is useful to call before iterating over the codepoints in the
    /// range and looking up the mapping for each. If you know none of the
    /// mappings will return anything, then you might be able to skip doing it
    /// altogether.
    ///
    /// # Panics
    ///
    /// This panics when `end < start`.
    pub fn overlaps(&self, start: char, end: char) -> bool {
        use core::cmp::Ordering;
        assert!(start <= end);
        self.table
            .binary_search_by(|&(c, _)| {
                if start <= c && c <= end {
                    Ordering::Equal
                } else if c > end {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .is_ok()
    }
    /// Returns the index at which `c` occurs in the simple case fold table. If
    /// `c` does not occur, then this returns an `i` such that `table[i-1].0 <
    /// c` and `table[i].0 > c`.
    fn get(&self, c: char) -> Result<usize, usize> {
        self.table.binary_search_by_key(&c, |&(c1, _)| c1)
    }
}
/// A query for finding a character class defined by Unicode. This supports
/// either use of a property name directly, or lookup by property value. The
/// former generally refers to Binary properties (see UTS#44, Table 8), but
/// as a special exception (see UTS#18, Section 1.2) both general categories
/// (an enumeration) and scripts (a catalog) are supported as if each of their
/// possible values were a binary property.
///
/// In all circumstances, property names and values are normalized and
/// canonicalized. That is, `GC == gc == GeneralCategory == general_category`.
///
/// The lifetime `'a` refers to the shorter of the lifetimes of property name
/// and property value.
#[derive(Debug)]
pub enum ClassQuery<'a> {
    /// Return a class corresponding to a Unicode binary property, named by
    /// a single letter.
    OneLetter(char),
    /// Return a class corresponding to a Unicode binary property.
    ///
    /// Note that, by special exception (see UTS#18, Section 1.2), both
    /// general category values and script values are permitted here as if
    /// they were a binary property.
    Binary(&'a str),
    /// Return a class corresponding to all codepoints whose property
    /// (identified by `property_name`) corresponds to the given value
    /// (identified by `property_value`).
    ByValue {
        /// A property name.
        property_name: &'a str,
        /// A property value.
        property_value: &'a str,
    },
}
impl<'a> ClassQuery<'a> {
    fn canonicalize(&self) -> Result<CanonicalClassQuery, Error> {
        match *self {
            ClassQuery::OneLetter(c) => self.canonical_binary(&c.to_string()),
            ClassQuery::Binary(name) => self.canonical_binary(name),
            ClassQuery::ByValue { property_name, property_value } => {
                let property_name = symbolic_name_normalize(property_name);
                let property_value = symbolic_name_normalize(property_value);
                let canon_name = match canonical_prop(&property_name)? {
                    None => return Err(Error::PropertyNotFound),
                    Some(canon_name) => canon_name,
                };
                Ok(
                    match canon_name {
                        "General_Category" => {
                            let canon = match canonical_gencat(&property_value)? {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon) => canon,
                            };
                            CanonicalClassQuery::GeneralCategory(canon)
                        }
                        "Script" => {
                            let canon = match canonical_script(&property_value)? {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon) => canon,
                            };
                            CanonicalClassQuery::Script(canon)
                        }
                        _ => {
                            let vals = match property_values(canon_name)? {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(vals) => vals,
                            };
                            let canon_val = match canonical_value(
                                vals,
                                &property_value,
                            ) {
                                None => return Err(Error::PropertyValueNotFound),
                                Some(canon_val) => canon_val,
                            };
                            CanonicalClassQuery::ByValue {
                                property_name: canon_name,
                                property_value: canon_val,
                            }
                        }
                    },
                )
            }
        }
    }
    fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error> {
        let norm = symbolic_name_normalize(name);
        if norm != "cf" && norm != "sc" && norm != "lc" {
            if let Some(canon) = canonical_prop(&norm)? {
                return Ok(CanonicalClassQuery::Binary(canon));
            }
        }
        if let Some(canon) = canonical_gencat(&norm)? {
            return Ok(CanonicalClassQuery::GeneralCategory(canon));
        }
        if let Some(canon) = canonical_script(&norm)? {
            return Ok(CanonicalClassQuery::Script(canon));
        }
        Err(Error::PropertyNotFound)
    }
}
/// Like ClassQuery, but its parameters have been canonicalized. This also
/// differentiates binary properties from flattened general categories and
/// scripts.
#[derive(Debug, Eq, PartialEq)]
enum CanonicalClassQuery {
    /// The canonical binary property name.
    Binary(&'static str),
    /// The canonical general category name.
    GeneralCategory(&'static str),
    /// The canonical script name.
    Script(&'static str),
    /// An arbitrary association between property and value, both of which
    /// have been canonicalized.
    ///
    /// Note that by construction, the property name of ByValue will never
    /// be General_Category or Script. Those two cases are subsumed by the
    /// eponymous variants.
    ByValue {
        /// The canonical property name.
        property_name: &'static str,
        /// The canonical property value.
        property_value: &'static str,
    },
}
/// Looks up a Unicode class given a query. If one doesn't exist, then
/// `None` is returned.
pub fn class(query: ClassQuery<'_>) -> Result<hir::ClassUnicode, Error> {
    use self::CanonicalClassQuery::*;
    match query.canonicalize()? {
        Binary(name) => bool_property(name),
        GeneralCategory(name) => gencat(name),
        Script(name) => script(name),
        ByValue { property_name: "Age", property_value } => {
            let mut class = hir::ClassUnicode::empty();
            for set in ages(property_value)? {
                class.union(&hir_class(set));
            }
            Ok(class)
        }
        ByValue { property_name: "Script_Extensions", property_value } => {
            script_extension(property_value)
        }
        ByValue { property_name: "Grapheme_Cluster_Break", property_value } => {
            gcb(property_value)
        }
        ByValue { property_name: "Sentence_Break", property_value } => sb(property_value),
        ByValue { property_name: "Word_Break", property_value } => wb(property_value),
        _ => Err(Error::PropertyNotFound),
    }
}
/// Returns a Unicode aware class for \w.
///
/// This returns an error if the data is not available for \w.
pub fn perl_word() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-perl"))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(feature = "unicode-perl")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_word::PERL_WORD;
        Ok(hir_class(PERL_WORD))
    }
    imp()
}
/// Returns a Unicode aware class for \s.
///
/// This returns an error if the data is not available for \s.
pub fn perl_space() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(any(feature = "unicode-perl", feature = "unicode-bool")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(all(feature = "unicode-perl", not(feature = "unicode-bool")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_space::WHITE_SPACE;
        Ok(hir_class(WHITE_SPACE))
    }
    #[cfg(feature = "unicode-bool")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::property_bool::WHITE_SPACE;
        Ok(hir_class(WHITE_SPACE))
    }
    imp()
}
/// Returns a Unicode aware class for \d.
///
/// This returns an error if the data is not available for \d.
pub fn perl_digit() -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(any(feature = "unicode-perl", feature = "unicode-gencat")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        Err(Error::PerlClassNotFound)
    }
    #[cfg(all(feature = "unicode-perl", not(feature = "unicode-gencat")))]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::perl_decimal::DECIMAL_NUMBER;
        Ok(hir_class(DECIMAL_NUMBER))
    }
    #[cfg(feature = "unicode-gencat")]
    fn imp() -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::general_category::DECIMAL_NUMBER;
        Ok(hir_class(DECIMAL_NUMBER))
    }
    imp()
}
/// Build a Unicode HIR class from a sequence of Unicode scalar value ranges.
pub fn hir_class(ranges: &[(char, char)]) -> hir::ClassUnicode {
    let hir_ranges: Vec<hir::ClassUnicodeRange> = ranges
        .iter()
        .map(|&(s, e)| hir::ClassUnicodeRange::new(s, e))
        .collect();
    hir::ClassUnicode::new(hir_ranges)
}
/// Returns true only if the given codepoint is in the `\w` character class.
///
/// If the `unicode-perl` feature is not enabled, then this returns an error.
pub fn is_word_character(c: char) -> Result<bool, UnicodeWordError> {
    #[cfg(not(feature = "unicode-perl"))]
    fn imp(_: char) -> Result<bool, UnicodeWordError> {
        Err(UnicodeWordError(()))
    }
    #[cfg(feature = "unicode-perl")]
    fn imp(c: char) -> Result<bool, UnicodeWordError> {
        use crate::{is_word_byte, unicode_tables::perl_word::PERL_WORD};
        if u8::try_from(c).map_or(false, is_word_byte) {
            return Ok(true);
        }
        Ok(
            PERL_WORD
                .binary_search_by(|&(start, end)| {
                    use core::cmp::Ordering;
                    if start <= c && c <= end {
                        Ordering::Equal
                    } else if start > c {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
                .is_ok(),
        )
    }
    imp(c)
}
/// A mapping of property values for a specific property.
///
/// The first element of each tuple is a normalized property value while the
/// second element of each tuple is the corresponding canonical property
/// value.
type PropertyValues = &'static [(&'static str, &'static str)];
fn canonical_gencat(normalized_value: &str) -> Result<Option<&'static str>, Error> {
    Ok(
        match normalized_value {
            "any" => Some("Any"),
            "assigned" => Some("Assigned"),
            "ascii" => Some("ASCII"),
            _ => {
                let gencats = property_values("General_Category")?.unwrap();
                canonical_value(gencats, normalized_value)
            }
        },
    )
}
fn canonical_script(normalized_value: &str) -> Result<Option<&'static str>, Error> {
    let scripts = property_values("Script")?.unwrap();
    Ok(canonical_value(scripts, normalized_value))
}
/// Find the canonical property name for the given normalized property name.
///
/// If no such property exists, then `None` is returned.
///
/// The normalized property name must have been normalized according to
/// UAX44 LM3, which can be done using `symbolic_name_normalize`.
///
/// If the property names data is not available, then an error is returned.
fn canonical_prop(normalized_name: &str) -> Result<Option<&'static str>, Error> {
    #[cfg(
        not(
            any(
                feature = "unicode-age",
                feature = "unicode-bool",
                feature = "unicode-gencat",
                feature = "unicode-perl",
                feature = "unicode-script",
                feature = "unicode-segment",
            )
        )
    )]
    fn imp(_: &str) -> Result<Option<&'static str>, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(
        any(
            feature = "unicode-age",
            feature = "unicode-bool",
            feature = "unicode-gencat",
            feature = "unicode-perl",
            feature = "unicode-script",
            feature = "unicode-segment",
        )
    )]
    fn imp(name: &str) -> Result<Option<&'static str>, Error> {
        use crate::unicode_tables::property_names::PROPERTY_NAMES;
        Ok(
            PROPERTY_NAMES
                .binary_search_by_key(&name, |&(n, _)| n)
                .ok()
                .map(|i| PROPERTY_NAMES[i].1),
        )
    }
    imp(normalized_name)
}
/// Find the canonical property value for the given normalized property
/// value.
///
/// The given property values should correspond to the values for the property
/// under question, which can be found using `property_values`.
///
/// If no such property value exists, then `None` is returned.
///
/// The normalized property value must have been normalized according to
/// UAX44 LM3, which can be done using `symbolic_name_normalize`.
fn canonical_value(
    vals: PropertyValues,
    normalized_value: &str,
) -> Option<&'static str> {
    vals.binary_search_by_key(&normalized_value, |&(n, _)| n).ok().map(|i| vals[i].1)
}
/// Return the table of property values for the given property name.
///
/// If the property values data is not available, then an error is returned.
fn property_values(
    canonical_property_name: &'static str,
) -> Result<Option<PropertyValues>, Error> {
    #[cfg(
        not(
            any(
                feature = "unicode-age",
                feature = "unicode-bool",
                feature = "unicode-gencat",
                feature = "unicode-perl",
                feature = "unicode-script",
                feature = "unicode-segment",
            )
        )
    )]
    fn imp(_: &'static str) -> Result<Option<PropertyValues>, Error> {
        Err(Error::PropertyValueNotFound)
    }
    #[cfg(
        any(
            feature = "unicode-age",
            feature = "unicode-bool",
            feature = "unicode-gencat",
            feature = "unicode-perl",
            feature = "unicode-script",
            feature = "unicode-segment",
        )
    )]
    fn imp(name: &'static str) -> Result<Option<PropertyValues>, Error> {
        use crate::unicode_tables::property_values::PROPERTY_VALUES;
        Ok(
            PROPERTY_VALUES
                .binary_search_by_key(&name, |&(n, _)| n)
                .ok()
                .map(|i| PROPERTY_VALUES[i].1),
        )
    }
    imp(canonical_property_name)
}
#[allow(dead_code)]
fn property_set(
    name_map: &'static [(&'static str, Range)],
    canonical: &'static str,
) -> Option<Range> {
    name_map.binary_search_by_key(&canonical, |x| x.0).ok().map(|i| name_map[i].1)
}
/// Returns an iterator over Unicode Age sets. Each item corresponds to a set
/// of codepoints that were added in a particular revision of Unicode. The
/// iterator yields items in chronological order.
///
/// If the given age value isn't valid or if the data isn't available, then an
/// error is returned instead.
fn ages(canonical_age: &str) -> Result<impl Iterator<Item = Range>, Error> {
    #[cfg(not(feature = "unicode-age"))]
    fn imp(_: &str) -> Result<impl Iterator<Item = Range>, Error> {
        use core::option::IntoIter;
        Err::<IntoIter<Range>, _>(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-age")]
    fn imp(canonical_age: &str) -> Result<impl Iterator<Item = Range>, Error> {
        use crate::unicode_tables::age;
        const AGES: &[(&str, Range)] = &[
            ("V1_1", age::V1_1),
            ("V2_0", age::V2_0),
            ("V2_1", age::V2_1),
            ("V3_0", age::V3_0),
            ("V3_1", age::V3_1),
            ("V3_2", age::V3_2),
            ("V4_0", age::V4_0),
            ("V4_1", age::V4_1),
            ("V5_0", age::V5_0),
            ("V5_1", age::V5_1),
            ("V5_2", age::V5_2),
            ("V6_0", age::V6_0),
            ("V6_1", age::V6_1),
            ("V6_2", age::V6_2),
            ("V6_3", age::V6_3),
            ("V7_0", age::V7_0),
            ("V8_0", age::V8_0),
            ("V9_0", age::V9_0),
            ("V10_0", age::V10_0),
            ("V11_0", age::V11_0),
            ("V12_0", age::V12_0),
            ("V12_1", age::V12_1),
            ("V13_0", age::V13_0),
            ("V14_0", age::V14_0),
            ("V15_0", age::V15_0),
        ];
        assert_eq!(AGES.len(), age::BY_NAME.len(), "ages are out of sync");
        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AGES[..=i].iter().map(|&(_, classes)| classes)),
        }
    }
    imp(canonical_age)
}
/// Returns the Unicode HIR class corresponding to the given general category.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given general category could not be found, or if the general
/// category data is not available, then an error is returned.
fn gencat(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-gencat"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-gencat")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::general_category::BY_NAME;
        match name {
            "ASCII" => Ok(hir_class(&[('\0', '\x7F')])),
            "Any" => Ok(hir_class(&[('\0', '\u{10FFFF}')])),
            "Assigned" => {
                let mut cls = gencat("Unassigned")?;
                cls.negate();
                Ok(cls)
            }
            name => {
                property_set(BY_NAME, name)
                    .map(hir_class)
                    .ok_or(Error::PropertyValueNotFound)
            }
        }
    }
    match canonical_name {
        "Decimal_Number" => perl_digit(),
        name => imp(name),
    }
}
/// Returns the Unicode HIR class corresponding to the given script.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given script could not be found, or if the script data is not
/// available, then an error is returned.
fn script(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-script"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-script")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::script::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
    }
    imp(canonical_name)
}
/// Returns the Unicode HIR class corresponding to the given script extension.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given script extension could not be found, or if the script data is
/// not available, then an error is returned.
fn script_extension(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-script"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-script")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::script_extension::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
    }
    imp(canonical_name)
}
/// Returns the Unicode HIR class corresponding to the given Unicode boolean
/// property.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given boolean property could not be found, or if the boolean
/// property data is not available, then an error is returned.
fn bool_property(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-bool"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-bool")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::property_bool::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyNotFound)
    }
    match canonical_name {
        "Decimal_Number" => perl_digit(),
        "White_Space" => perl_space(),
        name => imp(name),
    }
}
/// Returns the Unicode HIR class corresponding to the given grapheme cluster
/// break property.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given property could not be found, or if the corresponding data is
/// not available, then an error is returned.
fn gcb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-segment"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-segment")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::grapheme_cluster_break::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
    }
    imp(canonical_name)
}
/// Returns the Unicode HIR class corresponding to the given word break
/// property.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given property could not be found, or if the corresponding data is
/// not available, then an error is returned.
fn wb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-segment"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-segment")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::word_break::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
    }
    imp(canonical_name)
}
/// Returns the Unicode HIR class corresponding to the given sentence
/// break property.
///
/// Name canonicalization is assumed to be performed by the caller.
///
/// If the given property could not be found, or if the corresponding data is
/// not available, then an error is returned.
fn sb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error> {
    #[cfg(not(feature = "unicode-segment"))]
    fn imp(_: &'static str) -> Result<hir::ClassUnicode, Error> {
        Err(Error::PropertyNotFound)
    }
    #[cfg(feature = "unicode-segment")]
    fn imp(name: &'static str) -> Result<hir::ClassUnicode, Error> {
        use crate::unicode_tables::sentence_break::BY_NAME;
        property_set(BY_NAME, name).map(hir_class).ok_or(Error::PropertyValueNotFound)
    }
    imp(canonical_name)
}
/// Like symbolic_name_normalize_bytes, but operates on a string.
fn symbolic_name_normalize(x: &str) -> String {
    let mut tmp = x.as_bytes().to_vec();
    let len = symbolic_name_normalize_bytes(&mut tmp).len();
    tmp.truncate(len);
    String::from_utf8(tmp).unwrap()
}
/// Normalize the given symbolic name in place according to UAX44-LM3.
///
/// A "symbolic name" typically corresponds to property names and property
/// value aliases. Note, though, that it should not be applied to property
/// string values.
///
/// The slice returned is guaranteed to be valid UTF-8 for all possible values
/// of `slice`.
///
/// See: https://unicode.org/reports/tr44/#UAX44-LM3
fn symbolic_name_normalize_bytes(slice: &mut [u8]) -> &mut [u8] {
    let mut start = 0;
    let mut starts_with_is = false;
    if slice.len() >= 2 {
        starts_with_is = slice[0..2] == b"is"[..] || slice[0..2] == b"IS"[..]
            || slice[0..2] == b"iS"[..] || slice[0..2] == b"Is"[..];
        if starts_with_is {
            start = 2;
        }
    }
    let mut next_write = 0;
    for i in start..slice.len() {
        let b = slice[i];
        if b == b' ' || b == b'_' || b == b'-' {
            continue;
        } else if b'A' <= b && b <= b'Z' {
            slice[next_write] = b + (b'a' - b'A');
            next_write += 1;
        } else if b <= 0x7F {
            slice[next_write] = b;
            next_write += 1;
        }
    }
    if starts_with_is && next_write == 1 && slice[0] == b'c' {
        slice[0] = b'i';
        slice[1] = b's';
        slice[2] = b'c';
        next_write = 3;
    }
    &mut slice[..next_write]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "unicode-case")]
    fn simple_fold_ok(c: char) -> impl Iterator<Item = char> {
        SimpleCaseFolder::new().unwrap().mapping(c).iter().copied()
    }
    #[cfg(feature = "unicode-case")]
    fn contains_case_map(start: char, end: char) -> bool {
        SimpleCaseFolder::new().unwrap().overlaps(start, end)
    }
    #[test]
    #[cfg(feature = "unicode-case")]
    fn simple_fold_k() {
        let xs: Vec<char> = simple_fold_ok('k').collect();
        assert_eq!(xs, alloc::vec!['K', 'K']);
        let xs: Vec<char> = simple_fold_ok('K').collect();
        assert_eq!(xs, alloc::vec!['k', 'K']);
        let xs: Vec<char> = simple_fold_ok('K').collect();
        assert_eq!(xs, alloc::vec!['K', 'k']);
    }
    #[test]
    #[cfg(feature = "unicode-case")]
    fn simple_fold_a() {
        let xs: Vec<char> = simple_fold_ok('a').collect();
        assert_eq!(xs, alloc::vec!['A']);
        let xs: Vec<char> = simple_fold_ok('A').collect();
        assert_eq!(xs, alloc::vec!['a']);
    }
    #[test]
    #[cfg(not(feature = "unicode-case"))]
    fn simple_fold_disabled() {
        assert!(SimpleCaseFolder::new().is_err());
    }
    #[test]
    #[cfg(feature = "unicode-case")]
    fn range_contains() {
        assert!(contains_case_map('A', 'A'));
        assert!(contains_case_map('Z', 'Z'));
        assert!(contains_case_map('A', 'Z'));
        assert!(contains_case_map('@', 'A'));
        assert!(contains_case_map('Z', '['));
        assert!(contains_case_map('☃', 'Ⰰ'));
        assert!(! contains_case_map('[', '['));
        assert!(! contains_case_map('[', '`'));
        assert!(! contains_case_map('☃', '☃'));
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn regression_466() {
        use super::{CanonicalClassQuery, ClassQuery};
        let q = ClassQuery::OneLetter('C');
        assert_eq!(
            q.canonicalize().unwrap(), CanonicalClassQuery::GeneralCategory("Other")
        );
    }
    #[test]
    fn sym_normalize() {
        let sym_norm = symbolic_name_normalize;
        assert_eq!(sym_norm("Line_Break"), "linebreak");
        assert_eq!(sym_norm("Line-break"), "linebreak");
        assert_eq!(sym_norm("linebreak"), "linebreak");
        assert_eq!(sym_norm("BA"), "ba");
        assert_eq!(sym_norm("ba"), "ba");
        assert_eq!(sym_norm("Greek"), "greek");
        assert_eq!(sym_norm("isGreek"), "greek");
        assert_eq!(sym_norm("IS_Greek"), "greek");
        assert_eq!(sym_norm("isc"), "isc");
        assert_eq!(sym_norm("is c"), "isc");
        assert_eq!(sym_norm("is_c"), "isc");
    }
    #[test]
    fn valid_utf8_symbolic() {
        let mut x = b"abc\xFFxyz".to_vec();
        let y = symbolic_name_normalize_bytes(&mut x);
        assert_eq!(y, b"abcxyz");
    }
}
#[cfg(test)]
mod tests_llm_16_97 {
    use super::*;
    use crate::*;
    use regex_syntax::unicode;
    #[test]
    fn test_case_fold_error_fmt() {
        let case_fold_error = unicode::CaseFoldError(());
        let mut formatted = String::new();
        let result = case_fold_error.fmt(&mut formatted);
        debug_assert_eq!(result, Ok(()));
        debug_assert_eq!(
            formatted,
            "Unicode-aware case folding is not available \
             (probably because the unicode-case feature is not enabled)"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_98 {
    use super::*;
    use crate::*;
    use core::fmt::Write;
    use regex_syntax::unicode::UnicodeWordError;
    #[test]
    fn test_unicode_word_error_fmt() {
        let err = UnicodeWordError(());
        let mut output = String::new();
        debug_assert!(err.fmt(& mut output).is_ok());
        debug_assert_eq!(
            output,
            "Unicode-aware \\w class is not available (probably because the unicode-perl feature is not enabled)"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_574 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_f32_into_value() {
        let _rug_st_tests_llm_16_574_rug_test_from_f32_into_value = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 3.14;
        let rug_fuzz_6 = "Expected Float variant";
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let _rug_st_tests_llm_16_574_rug_test_from_f32_into_value = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let rug_fuzz_2 = rug_fuzz_3;
        let rug_fuzz_3 = rug_fuzz_4;
        let rug_fuzz_4 = rug_fuzz_5;
        let rug_fuzz_5 = rug_fuzz_6;
        let rug_fuzz_6 = rug_fuzz_7;
        let rug_fuzz_7 = rug_fuzz_8;
        let rug_fuzz_8 = rug_fuzz_9;
        let rug_fuzz_9 = rug_fuzz_10;
        let _rug_st_tests_llm_16_574_rug_test_from_f32_into_value = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let rug_fuzz_2 = rug_fuzz_3;
        let rug_fuzz_3 = rug_fuzz_4;
        let rug_fuzz_4 = rug_fuzz_5;
        let rug_fuzz_5 = rug_fuzz_6;
        let rug_fuzz_6 = rug_fuzz_7;
        let rug_fuzz_7 = rug_fuzz_8;
        let _rug_st_tests_llm_16_574 = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let rug_fuzz_2 = rug_fuzz_3;
        let rug_fuzz_3 = rug_fuzz_4;
        let rug_fuzz_4 = rug_fuzz_5;
        let rug_fuzz_5 = rug_fuzz_6;
        let _rug_st = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let rug_fuzz_2 = rug_fuzz_3;
        let rug_fuzz_3 = rug_fuzz_4;
        let _rug_st = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let val: f32 = rug_fuzz_0;
        let value: Value = From::from(val);
        match value {
            Value::Float(f) => debug_assert_eq!(f, 3.14),
            _ => panic!(rug_fuzz_1),
        }
        let _rug_ed = rug_fuzz_3;
        let _rug_ed = rug_fuzz_5;
        let _rug_ed_tests_llm_16_574 = rug_fuzz_7;
        let _rug_ed_tests_llm_16_574_test_from_f32_into_value = rug_fuzz_9;
        let _rug_ed_tests_llm_16_574_rug_test_from_f32_into_value = rug_fuzz_11;
        let _rug_ed_tests_llm_16_574_rug_test_from_f32_into_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_575 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonicalize_one_letter() {
        let rug_fuzz_0 = 'c';
        let rug_fuzz_0 = rug_fuzz_0;
        let query = ClassQuery::OneLetter(rug_fuzz_0);
        debug_assert_eq!(
            query.canonicalize(), Ok(CanonicalClassQuery::GeneralCategory("c"
            .to_string()))
        );
    }
    #[test]
    fn test_canonicalize_binary() {
        let rug_fuzz_0 = "gc";
        let rug_fuzz_0 = rug_fuzz_0;
        let query = ClassQuery::Binary(rug_fuzz_0);
        debug_assert_eq!(
            query.canonicalize(), Ok(CanonicalClassQuery::GeneralCategory("gc"
            .to_string()))
        );
    }
    #[test]
    fn test_canonicalize_by_value_general_category() {
        let rug_fuzz_0 = "General_Category";
        let rug_fuzz_1 = "Letter";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(
            query.canonicalize(), Ok(CanonicalClassQuery::GeneralCategory("Letter"
            .to_string()))
        );
    }
    #[test]
    fn test_canonicalize_by_value_script() {
        let rug_fuzz_0 = "Script";
        let rug_fuzz_1 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(
            query.canonicalize(), Ok(CanonicalClassQuery::Script("Latin".to_string()))
        );
    }
    #[test]
    fn test_canonicalize_by_value_custom() {
        let rug_fuzz_0 = "Custom";
        let rug_fuzz_1 = "Value";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(
            query.canonicalize(), Ok(CanonicalClassQuery::ByValue { property_name :
            "Normalized_Custom", property_value : "Normalized_Value", })
        );
    }
}
mod tests_llm_16_576 {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_found() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'A';
        let rug_fuzz_2 = 'b';
        let rug_fuzz_3 = 'B';
        let rug_fuzz_4 = 'c';
        let rug_fuzz_5 = 'C';
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 'b';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let rug_fuzz_7 = rug_fuzz_7;
        let folder = SimpleCaseFolder {
            table: &[
                (rug_fuzz_0, &[rug_fuzz_1]),
                (rug_fuzz_2, &[rug_fuzz_3]),
                (rug_fuzz_4, &[rug_fuzz_5]),
            ],
            last: None,
            next: rug_fuzz_6,
        };
        debug_assert_eq!(folder.get(rug_fuzz_7), Ok(1));
    }
    #[test]
    fn test_get_not_found() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'A';
        let rug_fuzz_2 = 'b';
        let rug_fuzz_3 = 'B';
        let rug_fuzz_4 = 'c';
        let rug_fuzz_5 = 'C';
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 'x';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let rug_fuzz_7 = rug_fuzz_7;
        let folder = SimpleCaseFolder {
            table: &[
                (rug_fuzz_0, &[rug_fuzz_1]),
                (rug_fuzz_2, &[rug_fuzz_3]),
                (rug_fuzz_4, &[rug_fuzz_5]),
            ],
            last: None,
            next: rug_fuzz_6,
        };
        debug_assert_eq!(folder.get(rug_fuzz_7), Err(3));
    }
}
#[cfg(test)]
mod tests_llm_16_577 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mapping() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'A';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let mut folder = SimpleCaseFolder::new().unwrap();
        debug_assert_eq!(folder.mapping(rug_fuzz_0), & ['a']);
        debug_assert_eq!(folder.mapping(rug_fuzz_1), & ['a']);
    }
}
#[cfg(test)]
mod tests_llm_16_578 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_simple_case_folder() {
        #[cfg(not(feature = "unicode-case"))]
        {
            let result = new();
            debug_assert!(result.is_err());
        }
        #[cfg(feature = "unicode-case")]
        {
            let result = new();
            debug_assert!(result.is_ok());
            let case_folder = result.unwrap();
        }
    }
}
mod tests_llm_16_579 {
    use super::*;
    use crate::*;
    const CASE_FOLDING_TABLE: &[(char, &[char])] = &[
        ('A', &['a']),
        ('B', &['b']),
        ('C', &['c']),
    ];
    #[test]
    fn test_overlaps_true() {
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 'A';
        let rug_fuzz_2 = 'B';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let case_folder = SimpleCaseFolder {
            table: CASE_FOLDING_TABLE,
            last: None,
            next: rug_fuzz_0,
        };
        debug_assert!(case_folder.overlaps(rug_fuzz_1, rug_fuzz_2));
    }
    #[test]
    #[should_panic(expected = "assertion failed: start <= end")]
    fn test_overlaps_panic() {
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 'C';
        let rug_fuzz_2 = 'B';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let case_folder = SimpleCaseFolder {
            table: CASE_FOLDING_TABLE,
            last: None,
            next: rug_fuzz_0,
        };
        case_folder.overlaps(rug_fuzz_1, rug_fuzz_2);
    }
    #[test]
    fn test_overlaps_false() {
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 'D';
        let rug_fuzz_2 = 'Z';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let case_folder = SimpleCaseFolder {
            table: CASE_FOLDING_TABLE,
            last: None,
            next: rug_fuzz_0,
        };
        debug_assert!(! case_folder.overlaps(rug_fuzz_1, rug_fuzz_2));
    }
}
#[cfg(test)]
mod tests_llm_16_580 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::age;
    use std::ops::Range;
    #[test]
    fn test_ages_valid_age() {
        let rug_fuzz_0 = "V12_0";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = ages(rug_fuzz_0);
        debug_assert!(result.is_ok());
        let iterator = result.unwrap();
        debug_assert_eq!(iterator.next(), Some(age::V1_1));
        debug_assert_eq!(iterator.next(), Some(age::V2_0));
        debug_assert_eq!(iterator.next(), Some(age::V2_1));
        debug_assert_eq!(iterator.next(), Some(age::V3_0));
        debug_assert_eq!(iterator.next(), Some(age::V3_1));
        debug_assert_eq!(iterator.next(), Some(age::V3_2));
        debug_assert_eq!(iterator.next(), Some(age::V4_0));
        debug_assert_eq!(iterator.next(), Some(age::V4_1));
        debug_assert_eq!(iterator.next(), Some(age::V5_0));
        debug_assert_eq!(iterator.next(), Some(age::V5_1));
        debug_assert_eq!(iterator.next(), Some(age::V5_2));
        debug_assert_eq!(iterator.next(), Some(age::V6_0));
        debug_assert_eq!(iterator.next(), Some(age::V6_1));
        debug_assert_eq!(iterator.next(), Some(age::V6_2));
        debug_assert_eq!(iterator.next(), Some(age::V6_3));
        debug_assert_eq!(iterator.next(), Some(age::V7_0));
        debug_assert_eq!(iterator.next(), Some(age::V8_0));
        debug_assert_eq!(iterator.next(), Some(age::V9_0));
        debug_assert_eq!(iterator.next(), Some(age::V10_0));
        debug_assert_eq!(iterator.next(), Some(age::V11_0));
        debug_assert_eq!(iterator.next(), Some(age::V12_0));
        debug_assert_eq!(iterator.next(), None);
    }
    #[test]
    fn test_ages_invalid_age() {
        let rug_fuzz_0 = "V16_0";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = ages(rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(result.err(), Some(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_581 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::age;
    #[test]
    fn test_imp_with_valid_age() {
        let rug_fuzz_0 = "V12_0";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = imp(rug_fuzz_0).unwrap().collect::<Vec<_>>();
        debug_assert_eq!(
            result, vec![age::V1_1, age::V2_0, age::V2_1, age::V3_0, age::V3_1,
            age::V3_2, age::V4_0, age::V4_1, age::V5_0, age::V5_1, age::V5_2, age::V6_0,
            age::V6_1, age::V6_2, age::V6_3, age::V7_0, age::V8_0, age::V9_0, age::V10_0,
            age::V11_0, age::V12_0]
        );
    }
    #[test]
    fn test_imp_with_invalid_age() {
        let rug_fuzz_0 = "InvalidAge";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = imp(rug_fuzz_0);
        debug_assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_582 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::property_bool::PropertyBool;
    #[test]
    fn test_bool_property_decimal_number() {
        let rug_fuzz_0 = "Decimal_Number";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(bool_property(rug_fuzz_0), Ok(perl_digit()));
    }
    #[test]
    fn test_bool_property_white_space() {
        let rug_fuzz_0 = "White_Space";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(bool_property(rug_fuzz_0), Ok(perl_space()));
    }
    #[test]
    fn test_bool_property_invalid_name() {
        let rug_fuzz_0 = "Invalid_Property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(bool_property(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_583 {
    use super::*;
    use crate::*;
    use regex_syntax::unicode::hir;
    #[test]
    fn test_imp_with_valid_name() {
        let rug_fuzz_0 = "some_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = imp(rug_fuzz_0);
        debug_assert!(result.is_ok());
    }
    #[test]
    fn test_imp_with_invalid_name() {
        let rug_fuzz_0 = "invalid_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = imp(rug_fuzz_0);
        debug_assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_584 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonical_gencat() {
        let rug_fuzz_0 = "any";
        let rug_fuzz_1 = "assigned";
        let rug_fuzz_2 = "ascii";
        let rug_fuzz_3 = "Lu";
        let rug_fuzz_4 = "foo";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        debug_assert_eq!(canonical_gencat(rug_fuzz_0), Ok(Some("Any")));
        debug_assert_eq!(canonical_gencat(rug_fuzz_1), Ok(Some("Assigned")));
        debug_assert_eq!(canonical_gencat(rug_fuzz_2), Ok(Some("ASCII")));
        debug_assert_eq!(canonical_gencat(rug_fuzz_3), Ok(Some("Uppercase_Letter")));
        debug_assert_eq!(canonical_gencat(rug_fuzz_4), Ok(None));
    }
}
#[cfg(test)]
mod tests_llm_16_585 {
    use super::*;
    use crate::*;
    use crate::Error;
    #[test]
    fn test_canonical_prop() {
        let rug_fuzz_0 = "age";
        let rug_fuzz_1 = "unknown_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(canonical_prop(rug_fuzz_0), Ok(Some("Age")));
        debug_assert_eq!(canonical_prop(rug_fuzz_1), Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_586 {
    use crate::imp;
    #[test]
    #[cfg(
        any(
            feature = "unicode-age",
            feature = "unicode-bool",
            feature = "unicode-gencat",
            feature = "unicode-perl",
            feature = "unicode-script",
            feature = "unicode-segment",
        )
    )]
    fn test_imp() {
        let rug_fuzz_0 = "some_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(None));
    }
}
#[cfg(test)]
mod tests_llm_16_587 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonical_script() {
        let rug_fuzz_0 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = canonical_script(rug_fuzz_0);
        debug_assert_eq!(result, Ok(Some("Latn")));
    }
}
#[cfg(test)]
mod tests_llm_16_588 {
    use super::*;
    use crate::*;
    use crate::unicode::{PropertyValues, symbolic_name_normalize};
    #[test]
    fn test_canonical_value() {
        let rug_fuzz_0 = "value1";
        let rug_fuzz_1 = "canonical_value1";
        let rug_fuzz_2 = "value1";
        let rug_fuzz_3 = "value2";
        let rug_fuzz_4 = "value3";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let vals: PropertyValues = vec![
            (rug_fuzz_0, rug_fuzz_1), ("value2", "canonical_value2")
        ];
        debug_assert_eq!(
            canonical_value(& vals, & symbolic_name_normalize(rug_fuzz_2)),
            Some("canonical_value1")
        );
        debug_assert_eq!(
            canonical_value(& vals, & symbolic_name_normalize(rug_fuzz_3)),
            Some("canonical_value2")
        );
        debug_assert_eq!(
            canonical_value(& vals, & symbolic_name_normalize(rug_fuzz_4)), None
        );
    }
}
#[cfg(test)]
mod tests_llm_16_589 {
    use super::*;
    use crate::*;
    use crate::unicode;
    #[test]
    fn test_class_query_binary() {
        let rug_fuzz_0 = "L";
        let rug_fuzz_0 = rug_fuzz_0;
        let input = unicode::ClassQuery::Binary(rug_fuzz_0);
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_general_category() {
        let rug_fuzz_0 = 'L';
        let rug_fuzz_0 = rug_fuzz_0;
        let input = unicode::ClassQuery::OneLetter(rug_fuzz_0);
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_by_value_age() {
        let rug_fuzz_0 = "Age";
        let rug_fuzz_1 = "V1_1";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let input = unicode::ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_by_value_script_extensions() {
        let rug_fuzz_0 = "Script_Extensions";
        let rug_fuzz_1 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let input = unicode::ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_by_value_grapheme_cluster_break() {
        let rug_fuzz_0 = "Grapheme_Cluster_Break";
        let rug_fuzz_1 = "CR";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let input = unicode::ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_by_value_sentence_break() {
        let rug_fuzz_0 = "Sentence_Break";
        let rug_fuzz_1 = "Lower";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let input = unicode::ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_by_value_word_break() {
        let rug_fuzz_0 = "Word_Break";
        let rug_fuzz_1 = "CR";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let input = unicode::ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(unicode::class(input), Ok(hir::ClassUnicode::empty()));
    }
    #[test]
    fn test_class_query_invalid_property() {
        let rug_fuzz_0 = 'X';
        let rug_fuzz_0 = rug_fuzz_0;
        let input = unicode::ClassQuery::OneLetter(rug_fuzz_0);
        debug_assert_eq!(unicode::class(input), Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_590 {
    use super::*;
    use crate::*;
    use regex_syntax::hir;
    #[test]
    fn test_gcb_with_known_property() {
        let rug_fuzz_0 = "Extend";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = gcb(rug_fuzz_0);
        debug_assert_eq!(result, Ok(hir::ClassUnicode::extend()));
    }
    #[test]
    fn test_gcb_with_unknown_property() {
        let rug_fuzz_0 = "UnknownProperty";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = gcb(rug_fuzz_0);
        debug_assert_eq!(result, Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_591 {
    use super::*;
    use crate::*;
    use crate::unicode::gcb::imp;
    use crate::unicode_tables::grapheme_cluster_break::BY_NAME;
    use crate::hir::{ClassUnicode, Error};
    #[test]
    fn test_imp_with_existing_name() {
        let rug_fuzz_0 = "some_existing_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(ClassUnicode::SomeExpectedValue));
    }
    #[test]
    fn test_imp_with_non_existing_name() {
        let rug_fuzz_0 = "some_non_existing_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_592 {
    use super::*;
    use crate::*;
    use crate::hir;
    #[test]
    fn test_gencat_decimal_number() {
        let rug_fuzz_0 = "Decimal_Number";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(gencat(rug_fuzz_0).unwrap(), perl_digit());
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_ascii() {
        let rug_fuzz_0 = "ASCII";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(gencat(rug_fuzz_0).unwrap(), hir_class(& [('\0', '\x7F')]));
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_any() {
        let rug_fuzz_0 = "Any";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            gencat(rug_fuzz_0).unwrap(), hir_class(& [('\0', '\u{10FFFF}')])
        );
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_assigned() {
        let rug_fuzz_0 = '\0';
        let rug_fuzz_1 = '\u{10FFFF}';
        let rug_fuzz_2 = "Assigned";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let mut cls = hir_class(&[(rug_fuzz_0, rug_fuzz_1)]);
        cls.negate();
        debug_assert_eq!(gencat(rug_fuzz_2).unwrap(), cls);
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_unassigned() {
        let rug_fuzz_0 = "Unassigned";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(gencat(rug_fuzz_0).unwrap_err(), Error::PropertyNotFound);
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_invalid_name() {
        let rug_fuzz_0 = "Invalid_Category";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(gencat(rug_fuzz_0).unwrap_err(), Error::PropertyValueNotFound);
    }
}
#[cfg(test)]
mod tests_llm_16_594 {
    use super::*;
    use crate::*;
    use crate::unicode;
    #[test]
    fn test_hir_class() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'z';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let ranges: Vec<(char, char)> = vec![(rug_fuzz_0, rug_fuzz_1), ('A', 'Z')];
        let class = hir_class(&ranges);
        debug_assert_eq!(class.ranges().len(), 2);
    }
}
#[cfg(test)]
mod tests_llm_16_595 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_word_character() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = ',';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(is_word_character(rug_fuzz_0).unwrap(), true);
        debug_assert_eq!(is_word_character(rug_fuzz_1).unwrap(), false);
    }
}
#[cfg(test)]
mod tests_llm_16_596 {
    use crate::imp;
    use crate::unicode::UnicodeWordError;
    #[test]
    fn test_imp_word_character() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = '7';
        let rug_fuzz_2 = '!';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        debug_assert_eq!(imp(rug_fuzz_0).unwrap(), true);
        debug_assert_eq!(imp(rug_fuzz_1).unwrap(), true);
        debug_assert_eq!(imp(rug_fuzz_2).unwrap(), false);
    }
}
#[cfg(test)]
mod tests_llm_16_597 {
    use super::*;
    use crate::*;
    use crate::unicode_tables;
    #[test]
    fn test_perl_digit_unicode_perl() {
        #[cfg(feature = "unicode-perl")]
        debug_assert!(
            matches!(perl_digit(),
            Ok(hir::ClassUnicode::new(unicode_tables::perl_decimal::DECIMAL_NUMBER)))
        );
    }
    #[test]
    fn test_perl_digit_unicode_gencat() {
        #[cfg(feature = "unicode-gencat")]
        debug_assert!(
            matches!(perl_digit(),
            Ok(hir::ClassUnicode::new(unicode_tables::general_category::DECIMAL_NUMBER)))
        );
    }
    #[test]
    fn test_perl_digit_not_found() {
        #[cfg(not(any(feature = "unicode-perl", feature = "unicode-gencat")))]
        debug_assert!(matches!(perl_digit(), Err(Error::PerlClassNotFound)));
    }
}
#[cfg(test)]
mod tests_llm_16_598 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::general_category::DECIMAL_NUMBER;
    #[test]
    fn test_imp() {
        debug_assert!(imp().is_ok());
        debug_assert_eq!(imp().unwrap(), hir_class(DECIMAL_NUMBER));
    }
}
#[cfg(test)]
mod tests_llm_16_599 {
    use super::*;
    use crate::*;
    use crate::hir;
    #[test]
    fn test_perl_space() {
        #[cfg(all(feature = "unicode-perl", not(feature = "unicode-bool")))]
        {
            debug_assert_eq!(
                perl_space().unwrap(),
                hir::ClassUnicode::new(hir::ClassUnicodeInner::Unicode { ranges : & crate
                ::unicode_tables::perl_space::WHITE_SPACE })
            );
        }
        #[cfg(feature = "unicode-bool")]
        {
            debug_assert_eq!(
                perl_space().unwrap(),
                hir::ClassUnicode::new(hir::ClassUnicodeInner::Unicode { ranges : & crate
                ::unicode_tables::property_bool::WHITE_SPACE })
            );
        }
        #[cfg(not(any(feature = "unicode-perl", feature = "unicode-bool")))]
        {
            debug_assert_eq!(perl_space(), Err(Error::PerlClassNotFound));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_600 {
    use crate::imp;
    use regex_syntax::hir::{Class, ClassUnicode};
    #[test]
    fn test_imp() {
        debug_assert_eq!(imp().unwrap(), Class::Unicode(ClassUnicode::new(WHITE_SPACE)));
    }
}
#[cfg(test)]
mod tests_llm_16_601 {
    use crate::perl_word;
    use crate::hir::ClassUnicode;
    use crate::Error;
    #[test]
    fn test_perl_word_unicode_not_available() {
        #[cfg(not(feature = "unicode-perl"))]
        debug_assert_eq!(perl_word(), Err(Error::PerlClassNotFound));
    }
    #[test]
    #[cfg(feature = "unicode-perl")]
    fn test_perl_word_unicode_available() {
        use crate::unicode_tables::perl_word::PERL_WORD;
        debug_assert_eq!(perl_word(), Ok(ClassUnicode::from(PERL_WORD)));
    }
}
#[cfg(test)]
mod tests_llm_16_602 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::perl_word::PERL_WORD;
    use crate::hir::ClassUnicode;
    #[test]
    fn test_imp() {
        let result = imp();
        debug_assert_eq!(result, Ok(ClassUnicode(PERL_WORD)));
    }
}
#[cfg(test)]
mod tests_llm_16_603 {
    use super::*;
    use crate::*;
    use regex_syntax::unicode::Range;
    #[test]
    fn test_property_set_existing() {
        let rug_fuzz_0 = "prop1";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = "prop2";
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 20;
        let rug_fuzz_6 = "prop1";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let name_map: &[(&'static str, Range)] = &[
            (
                rug_fuzz_0,
                Range {
                    start: rug_fuzz_1,
                    end: rug_fuzz_2,
                },
            ),
            (
                rug_fuzz_3,
                Range {
                    start: rug_fuzz_4,
                    end: rug_fuzz_5,
                },
            ),
        ];
        let canonical = rug_fuzz_6;
        debug_assert_eq!(
            property_set(name_map, canonical), Some(Range { start : 0, end : 10 })
        );
    }
    #[test]
    fn test_property_set_non_existing() {
        let rug_fuzz_0 = "prop1";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = "prop2";
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 20;
        let rug_fuzz_6 = "prop3";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let name_map: &[(&'static str, Range)] = &[
            (
                rug_fuzz_0,
                Range {
                    start: rug_fuzz_1,
                    end: rug_fuzz_2,
                },
            ),
            (
                rug_fuzz_3,
                Range {
                    start: rug_fuzz_4,
                    end: rug_fuzz_5,
                },
            ),
        ];
        let canonical = rug_fuzz_6;
        debug_assert_eq!(property_set(name_map, canonical), None);
    }
}
#[cfg(test)]
mod tests_llm_16_605 {
    use crate::imp;
    #[test]
    fn test_imp() {
        let rug_fuzz_0 = "some_property";
        let rug_fuzz_1 = "some_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert!(imp(rug_fuzz_0).is_ok());
        debug_assert_eq!(imp(rug_fuzz_1).unwrap(), None);
    }
}
#[cfg(test)]
mod tests_llm_16_606 {
    use super::*;
    use crate::*;
    use regex_syntax::hir::ClassUnicode;
    #[test]
    fn test_sb_not_found() {
        let rug_fuzz_0 = "unknown_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(sb(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
    #[cfg(feature = "unicode-segment")]
    #[test]
    fn test_sb_found() {
        let rug_fuzz_0 = "sentence_break_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(sb(rug_fuzz_0), Ok(ClassUnicode::SomeValue));
    }
}
#[cfg(test)]
mod tests_llm_16_607 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::sentence_break::BY_NAME;
    use crate::hir::ClassUnicode;
    #[test]
    fn test_imp() {
        let rug_fuzz_0 = "some_name";
        let rug_fuzz_1 = "non_existent_name";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(hir_class(BY_NAME, "some_name")));
        debug_assert_eq!(imp(rug_fuzz_1), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_608 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::script::BY_NAME;
    #[test]
    fn test_script_found() {
        let rug_fuzz_0 = "SomeScript";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(script(rug_fuzz_0), Ok(hir::ClassUnicode::SomeScript));
    }
    #[test]
    fn test_script_not_found() {
        let rug_fuzz_0 = "NonExistentScript";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(script(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
    #[test]
    fn test_script_table_not_available() {
        let rug_fuzz_0 = "SomeScript";
        let rug_fuzz_0 = rug_fuzz_0;
        let original_script_table = BY_NAME;
        crate::unicode_tables::script::BY_NAME = None;
        debug_assert_eq!(script(rug_fuzz_0), Err(Error::PropertyValueNotFound));
        crate::unicode_tables::script::BY_NAME = original_script_table;
    }
}
#[cfg(test)]
mod tests_llm_16_609 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::script::BY_NAME;
    #[test]
    fn test_imp_with_existing_script() {
        let rug_fuzz_0 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(imp(rug_fuzz_0).is_ok());
    }
    #[test]
    fn test_imp_with_non_existing_script() {
        let rug_fuzz_0 = "NonExistingScript";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_610 {
    use super::*;
    use crate::*;
    use regex_syntax::hir::ClassUnicode;
    use regex_syntax::Error;
    #[test]
    fn test_script_extension_feature_unicode_script() {
        let rug_fuzz_0 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            script_extension(rug_fuzz_0), Ok(ClassUnicode::ScriptExtension(crate
            ::unicode_tables::script_extension::HIR_LATIN))
        );
    }
    #[test]
    fn test_script_extension_no_feature_unicode_script() {
        let rug_fuzz_0 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(script_extension(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_611 {
    use super::*;
    use crate::*;
    use regex_syntax::unicode::script_extension::{imp, hir::ClassUnicode, Error};
    #[test]
    fn test_imp_valid_name() {
        let rug_fuzz_0 = "some_valid_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(ClassUnicode::new()));
    }
    #[test]
    fn test_imp_invalid_name() {
        let rug_fuzz_0 = "invalid_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_612 {
    use crate::symbolic_name_normalize;
    #[test]
    fn test_symbolic_name_normalize() {
        let rug_fuzz_0 = "abc";
        let rug_fuzz_1 = "a\u{00A0}b";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(symbolic_name_normalize(rug_fuzz_0), "abc".to_string());
        debug_assert_eq!(symbolic_name_normalize(rug_fuzz_1), "a b".to_string());
    }
}
#[cfg(test)]
mod tests_llm_16_613 {
    use super::*;
    use crate::*;
    #[test]
    fn test_symbolic_name_normalize_bytes() {
        let rug_fuzz_0 = b"IS_Latin";
        let rug_fuzz_1 = b"latin";
        let rug_fuzz_2 = b"isCyrillic";
        let rug_fuzz_3 = b"cyrillic";
        let rug_fuzz_4 = b"Some_Unicode-Name";
        let rug_fuzz_5 = b"someunicodename";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let mut input = rug_fuzz_0.to_vec();
        let expected_output = rug_fuzz_1.to_vec();
        debug_assert_eq!(symbolic_name_normalize_bytes(& mut input), & expected_output);
        let mut input = rug_fuzz_2.to_vec();
        let expected_output = rug_fuzz_3.to_vec();
        debug_assert_eq!(symbolic_name_normalize_bytes(& mut input), & expected_output);
        let mut input = rug_fuzz_4.to_vec();
        let expected_output = rug_fuzz_5.to_vec();
        debug_assert_eq!(symbolic_name_normalize_bytes(& mut input), & expected_output);
    }
}
#[cfg(test)]
mod tests_llm_16_615 {
    use crate::imp;
    use crate::hir::ClassUnicode;
    use crate::Error;
    #[test]
    fn test_imp_with_existing_property() {
        let rug_fuzz_0 = "some_valid_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(ClassUnicode()));
    }
    #[test]
    fn test_imp_with_non_existing_property() {
        let rug_fuzz_0 = "non_existing_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_728 {
    use super::*;
    use crate::*;
    use core::fmt::Write;
    #[test]
    fn test_case_fold_error_display() {
        let error = unicode::CaseFoldError(());
        let mut output = String::new();
        error.fmt(&mut output).unwrap();
        debug_assert_eq!(
            output,
            "Unicode-aware case folding is not available (probably because the unicode-case feature is not enabled)"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_729 {
    use super::*;
    use crate::*;
    use core::fmt::Write;
    #[test]
    fn test_fmt() {
        let error = unicode::UnicodeWordError(());
        let mut output = String::new();
        let result = error.fmt(&mut output);
        debug_assert!(result.is_ok());
        debug_assert_eq!(
            output,
            "Unicode-aware \\w class is not available (probably because the unicode-perl feature is not enabled)"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1205 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonical_binary() {
        let rug_fuzz_0 = "Letter";
        let rug_fuzz_1 = "Letter";
        let rug_fuzz_2 = "cf";
        let rug_fuzz_3 = "cf";
        let rug_fuzz_4 = "Invalid";
        let rug_fuzz_5 = "Invalid";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        debug_assert_eq!(
            unicode::ClassQuery::Binary(rug_fuzz_0).canonical_binary(rug_fuzz_1),
            Ok(CanonicalClassQuery::GeneralCategory("L"))
        );
        debug_assert_eq!(
            unicode::ClassQuery::Binary(rug_fuzz_2).canonical_binary(rug_fuzz_3),
            Ok(CanonicalClassQuery::GeneralCategory("Format"))
        );
        debug_assert_eq!(
            unicode::ClassQuery::Binary(rug_fuzz_4).canonical_binary(rug_fuzz_5),
            Err(Error::PropertyNotFound)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1206 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonicalize_with_one_letter() {
        let rug_fuzz_0 = 'C';
        let rug_fuzz_0 = rug_fuzz_0;
        let class_query = ClassQuery::OneLetter(rug_fuzz_0);
        debug_assert_eq!(
            class_query.canonicalize(), Ok(CanonicalClassQuery::GeneralCategory("C"
            .to_string()))
        );
    }
    #[test]
    fn test_canonicalize_with_binary() {
        let rug_fuzz_0 = "Alphabetic";
        let rug_fuzz_0 = rug_fuzz_0;
        let class_query = ClassQuery::Binary(rug_fuzz_0);
        debug_assert_eq!(
            class_query.canonicalize(),
            Ok(CanonicalClassQuery::GeneralCategory("Alphabetic".to_string()))
        );
    }
    #[test]
    fn test_canonicalize_with_by_value() {
        let rug_fuzz_0 = "General_Category";
        let rug_fuzz_1 = "Letter";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let class_query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(
            class_query.canonicalize(), Ok(CanonicalClassQuery::GeneralCategory("Letter"
            .to_string()))
        );
    }
    #[test]
    fn test_canonicalize_with_invalid_property_name() {
        let rug_fuzz_0 = "Invalid_Property";
        let rug_fuzz_1 = "Value";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let class_query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(class_query.canonicalize(), Err(Error::PropertyNotFound));
    }
    #[test]
    fn test_canonicalize_with_invalid_property_value() {
        let rug_fuzz_0 = "General_Category";
        let rug_fuzz_1 = "InvalidValue";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let class_query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(class_query.canonicalize(), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1207 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::case_folding_simple::CASE_FOLDING_SIMPLE;
    #[test]
    fn test_simple_case_folder_get() {
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 'a';
        let rug_fuzz_2 = 'b';
        let rug_fuzz_3 = 'z';
        let rug_fuzz_4 = 'A';
        let rug_fuzz_5 = 'Z';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let case_folder = SimpleCaseFolder {
            table: CASE_FOLDING_SIMPLE,
            last: None,
            next: rug_fuzz_0,
        };
        debug_assert_eq!(case_folder.get(rug_fuzz_1), Ok(0));
        debug_assert_eq!(case_folder.get(rug_fuzz_2), Ok(1));
        debug_assert_eq!(case_folder.get(rug_fuzz_3), Ok(26));
        debug_assert_eq!(case_folder.get(rug_fuzz_4), Err(0));
        debug_assert_eq!(case_folder.get(rug_fuzz_5), Err(0));
    }
}
#[cfg(test)]
mod tests_llm_16_1208 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mapping() {
        let rug_fuzz_0 = 'A';
        let rug_fuzz_1 = 'B';
        let rug_fuzz_2 = 'C';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let mut folder = SimpleCaseFolder::new().unwrap();
        debug_assert_eq!(folder.mapping(rug_fuzz_0), & ['a']);
        debug_assert_eq!(folder.mapping(rug_fuzz_1), & ['b']);
        debug_assert_eq!(folder.mapping(rug_fuzz_2), & ['c']);
    }
    #[test]
    #[should_panic(
        expected = "got codepoint U+42 which occurs before last codepoint U+41"
    )]
    fn test_mapping_panic() {
        let rug_fuzz_0 = 'b';
        let rug_fuzz_1 = 'a';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let mut folder = SimpleCaseFolder::new().unwrap();
        folder.mapping(rug_fuzz_0);
        folder.mapping(rug_fuzz_1);
    }
    #[test]
    fn test_overlaps() {
        let rug_fuzz_0 = 'A';
        let rug_fuzz_1 = 'Z';
        let rug_fuzz_2 = '!';
        let rug_fuzz_3 = '/';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let folder = SimpleCaseFolder::new().unwrap();
        debug_assert!(folder.overlaps(rug_fuzz_0, rug_fuzz_1));
        debug_assert!(! folder.overlaps(rug_fuzz_2, rug_fuzz_3));
    }
}
#[cfg(test)]
mod tests_llm_16_1209 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_simple_case_folder() {
        let result = unicode::SimpleCaseFolder::new();
        #[cfg(feature = "unicode-case")]
        {
            debug_assert!(result.is_ok());
        }
        #[cfg(not(feature = "unicode-case"))]
        {
            debug_assert!(result.is_err());
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1210 {
    use super::*;
    use crate::*;
    #[test]
    fn test_overlaps() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'A';
        let rug_fuzz_2 = 'b';
        let rug_fuzz_3 = 'B';
        let rug_fuzz_4 = 'c';
        let rug_fuzz_5 = 'C';
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 'a';
        let rug_fuzz_8 = 'c';
        let rug_fuzz_9 = 'a';
        let rug_fuzz_10 = 'b';
        let rug_fuzz_11 = 'b';
        let rug_fuzz_12 = 'c';
        let rug_fuzz_13 = 'd';
        let rug_fuzz_14 = 'f';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let rug_fuzz_7 = rug_fuzz_7;
        let rug_fuzz_8 = rug_fuzz_8;
        let rug_fuzz_9 = rug_fuzz_9;
        let rug_fuzz_10 = rug_fuzz_10;
        let rug_fuzz_11 = rug_fuzz_11;
        let rug_fuzz_12 = rug_fuzz_12;
        let rug_fuzz_13 = rug_fuzz_13;
        let rug_fuzz_14 = rug_fuzz_14;
        let case_folder = SimpleCaseFolder {
            table: &[
                (rug_fuzz_0, &[rug_fuzz_1]),
                (rug_fuzz_2, &[rug_fuzz_3]),
                (rug_fuzz_4, &[rug_fuzz_5]),
            ],
            last: None,
            next: rug_fuzz_6,
        };
        debug_assert_eq!(case_folder.overlaps(rug_fuzz_7, rug_fuzz_8), true);
        debug_assert_eq!(case_folder.overlaps(rug_fuzz_9, rug_fuzz_10), true);
        debug_assert_eq!(case_folder.overlaps(rug_fuzz_11, rug_fuzz_12), true);
        debug_assert_eq!(case_folder.overlaps(rug_fuzz_13, rug_fuzz_14), false);
    }
}
#[cfg(test)]
mod tests_llm_16_1211 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::age;
    #[test]
    fn test_ages_valid() {
        let rug_fuzz_0 = "V12_0";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            ages(rug_fuzz_0), Ok(vec![age::V1_1, age::V2_0, age::V2_1, age::V3_0,
            age::V3_1, age::V3_2, age::V4_0, age::V4_1, age::V5_0, age::V5_1, age::V5_2,
            age::V6_0, age::V6_1, age::V6_2, age::V6_3, age::V7_0, age::V8_0, age::V9_0,
            age::V10_0, age::V11_0, age::V12_0])
        );
    }
    #[test]
    fn test_ages_invalid() {
        let rug_fuzz_0 = "V16_0";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(ages(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1212 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::age;
    #[test]
    fn test_imp() {
        let rug_fuzz_0 = "V1_1";
        let rug_fuzz_1 = "V2_0";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(
            imp(rug_fuzz_0).unwrap().collect:: < Vec < _ > > (), age::V1_1.collect:: <
            Vec < _ > > ()
        );
        debug_assert_eq!(
            imp(rug_fuzz_1).unwrap().collect:: < Vec < _ > > (), age::V2_0.collect:: <
            Vec < _ > > ()
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1213 {
    use super::*;
    use crate::*;
    use crate::hir;
    #[test]
    fn test_bool_property_decimal_number() {
        let rug_fuzz_0 = "Decimal_Number";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(bool_property(rug_fuzz_0), Ok(hir::ClassUnicode::PerlDigit));
    }
    #[test]
    fn test_bool_property_white_space() {
        let rug_fuzz_0 = "White_Space";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(bool_property(rug_fuzz_0), Ok(hir::ClassUnicode::PerlSpace));
    }
    #[test]
    fn test_bool_property_unknown_property() {
        let rug_fuzz_0 = "Unknown_Property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(bool_property(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1214 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::property_bool;
    #[test]
    fn test_imp_existing_property() {
        let rug_fuzz_0 = "propertyName";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(imp(rug_fuzz_0).is_ok());
    }
    #[test]
    fn test_imp_non_existing_property() {
        let rug_fuzz_0 = "nonExistingProperty";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(imp(rug_fuzz_0).is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_1215 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonical_gencat_any() {
        let rug_fuzz_0 = "any";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(canonical_gencat(rug_fuzz_0), Ok(Some("Any")));
    }
    #[test]
    fn test_canonical_gencat_assigned() {
        let rug_fuzz_0 = "assigned";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(canonical_gencat(rug_fuzz_0), Ok(Some("Assigned")));
    }
    #[test]
    fn test_canonical_gencat_ascii() {
        let rug_fuzz_0 = "ascii";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(canonical_gencat(rug_fuzz_0), Ok(Some("ASCII")));
    }
}
#[cfg(test)]
mod tests_llm_16_1216 {
    use super::*;
    use crate::*;
    #[test]
    fn test_canonical_prop_found() {
        let rug_fuzz_0 = "age";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(canonical_prop(rug_fuzz_0), Ok(Some("Age")));
    }
    #[test]
    fn test_canonical_prop_not_found() {
        let rug_fuzz_0 = "invalid_prop";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(canonical_prop(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1217 {
    use super::*;
    use crate::*;
    #[test]
    #[cfg(
        any(
            feature = "unicode-age",
            feature = "unicode-bool",
            feature = "unicode-gencat",
            feature = "unicode-perl",
            feature = "unicode-script",
            feature = "unicode-segment",
        )
    )]
    fn test_imp() {
        let rug_fuzz_0 = "SomePropertyName";
        let rug_fuzz_1 = "NonExistentProperty";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(Some("SomePropertyValue")));
        debug_assert_eq!(imp(rug_fuzz_1), Ok(None));
    }
}
#[cfg(test)]
mod tests_llm_16_1218 {
    use crate::canonical_script;
    #[test]
    fn test_canonical_script() {
        let rug_fuzz_0 = "Latn";
        let rug_fuzz_1 = "Cyrl";
        let rug_fuzz_2 = "Invalid";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        debug_assert_eq!(canonical_script(rug_fuzz_0), Ok(Some("Latin")));
        debug_assert_eq!(canonical_script(rug_fuzz_1), Ok(Some("Cyrillic")));
        debug_assert_eq!(canonical_script(rug_fuzz_2), Ok(None));
    }
}
#[cfg(test)]
mod tests_llm_16_1219 {
    use super::*;
    use crate::*;
    use crate::unicode::{PropertyValues, symbolic_name_normalize};
    #[test]
    fn test_canonical_value() {
        let rug_fuzz_0 = "value1";
        let rug_fuzz_1 = "canonical_value1";
        let rug_fuzz_2 = "value2";
        let rug_fuzz_3 = "value4";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let property_values: PropertyValues = vec![
            (rug_fuzz_0, rug_fuzz_1), ("value2", "canonical_value2"), ("value3",
            "canonical_value3")
        ];
        debug_assert_eq!(
            canonical_value(property_values, rug_fuzz_2), Some("canonical_value2")
        );
        debug_assert_eq!(canonical_value(property_values, rug_fuzz_3), None);
    }
    #[test]
    fn test_canonical_value_normalized() {
        let rug_fuzz_0 = "value1";
        let rug_fuzz_1 = "canonical_value1";
        let rug_fuzz_2 = "  Value2  ";
        let rug_fuzz_3 = "value4";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let property_values: PropertyValues = vec![
            (rug_fuzz_0, rug_fuzz_1), ("  value2  ", "canonical_value2"), ("VALUE3",
            "canonical_value3")
        ];
        debug_assert_eq!(
            canonical_value(property_values, symbolic_name_normalize(rug_fuzz_2)),
            Some("canonical_value2")
        );
        debug_assert_eq!(
            canonical_value(property_values, symbolic_name_normalize(rug_fuzz_3)), None
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1220 {
    use super::*;
    use crate::*;
    #[test]
    fn test_class_query_one_letter() {
        let rug_fuzz_0 = 'A';
        let rug_fuzz_0 = rug_fuzz_0;
        let query = ClassQuery::OneLetter(rug_fuzz_0);
        debug_assert_eq!(class(query).unwrap(), hir::ClassUnicode::empty());
    }
    #[test]
    fn test_class_query_binary() {
        let rug_fuzz_0 = "Alphabetic";
        let rug_fuzz_0 = rug_fuzz_0;
        let query = ClassQuery::Binary(rug_fuzz_0);
        debug_assert_eq!(class(query).unwrap(), hir::ClassUnicode::empty());
    }
    #[test]
    fn test_class_query_by_value() {
        let rug_fuzz_0 = "Age";
        let rug_fuzz_1 = "V1_1";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let query = ClassQuery::ByValue {
            property_name: rug_fuzz_0,
            property_value: rug_fuzz_1,
        };
        debug_assert_eq!(class(query).unwrap(), hir::ClassUnicode::empty());
    }
}
#[cfg(test)]
mod tests_llm_16_1221 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::grapheme_cluster_break::HIR_CLASS_UNICODE;
    #[test]
    fn test_gcb() {
        let rug_fuzz_0 = "some_property_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(gcb(rug_fuzz_0).unwrap(), HIR_CLASS_UNICODE);
    }
}
#[cfg(test)]
mod tests_llm_16_1222 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::grapheme_cluster_break::BY_NAME;
    #[test]
    fn test_imp_with_valid_name() {
        let rug_fuzz_0 = "valid_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(imp(rug_fuzz_0).is_ok());
    }
    #[test]
    fn test_imp_with_invalid_name() {
        let rug_fuzz_0 = "invalid_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0).unwrap_err(), Error::PropertyValueNotFound);
    }
}
#[cfg(test)]
mod tests_llm_16_1223 {
    use super::*;
    use crate::*;
    use crate::hir;
    #[test]
    fn test_gencat_decimal_number() {
        let rug_fuzz_0 = "Decimal_Number";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(gencat(rug_fuzz_0), Ok(hir::ClassUnicode::PerlDigit));
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_ascii() {
        let rug_fuzz_0 = "ASCII";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            gencat(rug_fuzz_0), Ok(hir::ClassUnicode::new(vec![('\0', '\x7F')]))
        );
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_any() {
        let rug_fuzz_0 = "Any";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            gencat(rug_fuzz_0), Ok(hir::ClassUnicode::new(vec![('\0', '\u{10FFFF}')]))
        );
    }
    #[test]
    #[cfg(feature = "unicode-gencat")]
    fn test_gencat_assigned() {
        let rug_fuzz_0 = "Assigned";
        let rug_fuzz_0 = rug_fuzz_0;
        let mut cls = hir::ClassUnicode::new(vec![]);
        cls.negate();
        debug_assert_eq!(gencat(rug_fuzz_0), Ok(cls));
    }
}
#[cfg(test)]
mod tests_llm_16_1224 {
    use crate::imp;
    use crate::unicode::gencat::Error;
    use crate::regex_syntax::hir::ClassUnicode;
    #[test]
    fn test_imp_ascii() {
        let rug_fuzz_0 = "ASCII";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(ClassUnicode::new(vec![('\0', '\x7F')])));
    }
    #[test]
    fn test_imp_any() {
        let rug_fuzz_0 = "Any";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            imp(rug_fuzz_0), Ok(ClassUnicode::new(vec![('\0', '\u{10FFFF}')]))
        );
    }
    #[test]
    fn test_imp_assigned() {
        let rug_fuzz_0 = "Assigned";
        let rug_fuzz_0 = rug_fuzz_0;
        let mut cls = ClassUnicode::new(vec![]);
        cls.negate();
        debug_assert_eq!(imp(rug_fuzz_0), Ok(cls));
    }
    #[test]
    fn test_imp_invalid_name() {
        let rug_fuzz_0 = "InvalidName";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1225 {
    use super::*;
    use crate::*;
    use hir::ClassUnicodeRange;
    #[test]
    fn test_hir_class() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'z';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let ranges: Vec<ClassUnicodeRange> = vec![
            ClassUnicodeRange::new(rug_fuzz_0, rug_fuzz_1), ClassUnicodeRange::new('A',
            'Z')
        ];
        let class = hir_class(&ranges);
        debug_assert_eq!(class.ranges(), ranges.as_slice());
    }
}
#[cfg(test)]
mod tests_llm_16_1226 {
    use super::*;
    use crate::*;
    use crate::UnicodeWordError;
    #[test]
    fn test_is_word_character_unicode_perl_enabled() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = '3';
        let rug_fuzz_2 = '_';
        let rug_fuzz_3 = '@';
        let rug_fuzz_4 = ' ';
        let rug_fuzz_5 = '\t';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        debug_assert_eq!(is_word_character(rug_fuzz_0), Ok(true));
        debug_assert_eq!(is_word_character(rug_fuzz_1), Ok(true));
        debug_assert_eq!(is_word_character(rug_fuzz_2), Ok(true));
        debug_assert_eq!(is_word_character(rug_fuzz_3), Ok(false));
        debug_assert_eq!(is_word_character(rug_fuzz_4), Ok(false));
        debug_assert_eq!(is_word_character(rug_fuzz_5), Ok(false));
    }
    #[test]
    fn test_is_word_character_unicode_perl_disabled() {
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = '3';
        let rug_fuzz_2 = '_';
        let rug_fuzz_3 = '@';
        let rug_fuzz_4 = ' ';
        let rug_fuzz_5 = '\t';
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        debug_assert_eq!(is_word_character(rug_fuzz_0), Err(UnicodeWordError(())));
        debug_assert_eq!(is_word_character(rug_fuzz_1), Err(UnicodeWordError(())));
        debug_assert_eq!(is_word_character(rug_fuzz_2), Err(UnicodeWordError(())));
        debug_assert_eq!(is_word_character(rug_fuzz_3), Err(UnicodeWordError(())));
        debug_assert_eq!(is_word_character(rug_fuzz_4), Err(UnicodeWordError(())));
        debug_assert_eq!(is_word_character(rug_fuzz_5), Err(UnicodeWordError(())));
    }
}
#[cfg(test)]
mod tests_llm_16_1228 {
    use super::*;
    use crate::*;
    use crate::hir;
    #[test]
    fn test_perl_digit_unicode_perl() {
        let rug_fuzz_0 = "Feature unicode-perl is required for this test";
        let rug_fuzz_0 = rug_fuzz_0;
        #[cfg(not(feature = "unicode-perl"))] compile_error!(rug_fuzz_0);
        #[cfg(feature = "unicode-perl")]
        {
            let expected_class = hir_class();
            debug_assert_eq!(perl_digit().unwrap(), expected_class);
        }
    }
    #[test]
    fn test_perl_digit_unicode_gencat() {
        let rug_fuzz_0 = "Feature unicode-gencat is required for this test";
        let rug_fuzz_0 = rug_fuzz_0;
        #[cfg(not(feature = "unicode-gencat"))] compile_error!(rug_fuzz_0);
        #[cfg(feature = "unicode-gencat")]
        {
            let expected_class = hir_class();
            debug_assert_eq!(perl_digit().unwrap(), expected_class);
        }
    }
    #[test]
    fn test_perl_digit_no_features() {
        #[cfg(not(all(feature = "unicode-perl", feature = "unicode-gencat")))]
        {
            let result = perl_digit();
            debug_assert!(result.is_err());
            debug_assert_eq!(result.err().unwrap(), Error::PerlClassNotFound);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1229 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::general_category;
    #[test]
    fn test_imp() {
        debug_assert_eq!(imp().unwrap(), hir_class(general_category::DECIMAL_NUMBER));
    }
}
#[cfg(test)]
mod tests_llm_16_1230 {
    use super::*;
    use crate::*;
    use crate::unicode_tables;
    #[test]
    fn test_perl_space() {
        #[cfg(feature = "unicode-perl")]
        {
            debug_assert!(perl_space().is_ok());
            let result = perl_space().unwrap();
            debug_assert_eq!(result, hir_class(unicode_tables::perl_space::WHITE_SPACE));
        }
        #[cfg(feature = "unicode-bool")]
        {
            debug_assert!(perl_space().is_ok());
            let result = perl_space().unwrap();
            debug_assert_eq!(
                result, hir_class(unicode_tables::property_bool::WHITE_SPACE)
            );
        }
        #[cfg(not(any(feature = "unicode-perl", feature = "unicode-bool")))]
        {
            debug_assert!(perl_space().is_err());
            let error = perl_space().unwrap_err();
            debug_assert_eq!(error, Error::PerlClassNotFound);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1231 {
    use super::*;
    use crate::*;
    use crate::unicode::perl_space::imp;
    use crate::hir::ClassUnicode;
    use crate::Error;
    #[test]
    fn test_imp() {
        debug_assert!(imp().is_ok());
        debug_assert_eq!(imp().unwrap(), ClassUnicode::new(WHITE_SPACE));
    }
}
#[cfg(test)]
mod tests_llm_16_1232 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::perl_word;
    #[test]
    fn test_perl_word_with_unicode_perl_feature() {
        let rug_fuzz_0 = "unicode-perl";
        let rug_fuzz_0 = rug_fuzz_0;
        feature::set(rug_fuzz_0);
        debug_assert!(perl_word().is_ok());
    }
    #[test]
    fn test_perl_word_without_unicode_perl_feature() {
        let rug_fuzz_0 = "unicode-perl";
        let rug_fuzz_0 = rug_fuzz_0;
        feature::clear(rug_fuzz_0);
        debug_assert!(perl_word().is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_1233 {
    use crate::unicode::perl_word::imp;
    use crate::hir::ClassUnicode;
    use crate::Error;
    #[test]
    fn test_imp() {
        let result = imp();
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), ClassUnicode);
    }
}
#[cfg(test)]
mod tests_llm_16_1234 {
    use super::*;
    use crate::*;
    #[test]
    fn test_property_set_found() {
        let rug_fuzz_0 = "property1";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = "property2";
        let rug_fuzz_4 = 6;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = "property2";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let name_map: &'static [(&'static str, Range)] = &[
            (rug_fuzz_0, rug_fuzz_1..rug_fuzz_2),
            (rug_fuzz_3, rug_fuzz_4..rug_fuzz_5),
        ];
        let canonical = rug_fuzz_6;
        debug_assert_eq!(property_set(name_map, canonical), Some(6..10));
    }
    #[test]
    fn test_property_set_not_found() {
        let rug_fuzz_0 = "property1";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = "property2";
        let rug_fuzz_4 = 6;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = "property3";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        let rug_fuzz_3 = rug_fuzz_3;
        let rug_fuzz_4 = rug_fuzz_4;
        let rug_fuzz_5 = rug_fuzz_5;
        let rug_fuzz_6 = rug_fuzz_6;
        let name_map: &'static [(&'static str, Range)] = &[
            (rug_fuzz_0, rug_fuzz_1..rug_fuzz_2),
            (rug_fuzz_3, rug_fuzz_4..rug_fuzz_5),
        ];
        let canonical = rug_fuzz_6;
        debug_assert_eq!(property_set(name_map, canonical), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1235 {
    use super::*;
    use crate::*;
    #[test]
    fn test_property_values_not_found() {
        let rug_fuzz_0 = "unknown_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = property_values(rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(result.err(), Some(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1236 {
    use super::*;
    use crate::*;
    #[test]
    fn test_imp() {
        let rug_fuzz_0 = "property_name";
        let rug_fuzz_1 = "non_existent_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        debug_assert_eq!(imp(rug_fuzz_0).unwrap().unwrap(), expected_property_value);
        debug_assert_eq!(imp(rug_fuzz_1).unwrap(), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1237 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::sentence_break::BY_NAME;
    use crate::hir::ClassUnicode;
    #[test]
    fn test_sb_not_found() {
        let rug_fuzz_0 = "unknown_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(sb(rug_fuzz_0), Err(Error::PropertyNotFound));
    }
    #[cfg(feature = "unicode-segment")]
    #[test]
    fn test_sb_found() {
        let rug_fuzz_0 = "some_property";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(sb(rug_fuzz_0), Ok(ClassUnicode()));
    }
}
#[cfg(test)]
mod tests_llm_16_1238 {
    use crate::imp;
    use crate::unicode::hir::{self, ClassUnicode};
    use crate::unicode_tables::sentence_break::BY_NAME;
    use crate::Error;
    #[test]
    fn test_imp_existing_property() {
        let rug_fuzz_0 = "SomeExistingProperty";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(hir::ClassUnicode::SomeExpectedValue));
    }
    #[test]
    fn test_imp_non_existing_property() {
        let rug_fuzz_0 = "NonExistingProperty";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1239 {
    use super::*;
    use crate::*;
    use crate::hir;
    use crate::Error;
    #[test]
    fn test_script() {
        let rug_fuzz_0 = "Latin";
        let rug_fuzz_1 = "Greek";
        let rug_fuzz_2 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        #[cfg(feature = "unicode-script")]
        {
            debug_assert_eq!(
                script(rug_fuzz_0).unwrap(), hir::ClassUnicode::ScriptLatin
            );
            debug_assert_eq!(
                script(rug_fuzz_1).unwrap(), hir::ClassUnicode::ScriptGreek
            );
        }
        #[cfg(not(feature = "unicode-script"))]
        {
            debug_assert_eq!(script(rug_fuzz_2), Err(Error::PropertyNotFound));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1240 {
    use crate::imp;
    use crate::unicode::hir::ClassUnicode;
    use crate::unicode::Error;
    #[test]
    fn test_imp_valid_name() {
        let rug_fuzz_0 = "Latin";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Ok(ClassUnicode(0x00000200..0x00000300)));
    }
    #[test]
    fn test_imp_invalid_name() {
        let rug_fuzz_0 = "InvalidScript";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound));
    }
}
#[cfg(test)]
mod tests_llm_16_1241 {
    use super::*;
    use crate::*;
    use crate::hir::{ClassUnicode, Error};
    #[test]
    fn test_script_extension() {
        let rug_fuzz_0 = "CanonicalName";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert_eq!(
            script_extension(rug_fuzz_0), Ok(ClassUnicode::SomeUnicodeClass)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1242 {
    use crate::imp;
    #[test]
    fn test_imp_valid_name() {
        let rug_fuzz_0 = "ValidName";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(imp(rug_fuzz_0).is_ok());
    }
    #[test]
    fn test_imp_invalid_name() {
        let rug_fuzz_0 = "InvalidName";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(imp(rug_fuzz_0).is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_1243 {
    use super::*;
    use crate::*;
    #[test]
    fn test_symbolic_name_normalize() {
        let rug_fuzz_0 = "ValidString";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = "Caf\u{e9}";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        let rug_fuzz_2 = rug_fuzz_2;
        debug_assert_eq!(symbolic_name_normalize(rug_fuzz_0), "ValidString");
        debug_assert_eq!(symbolic_name_normalize(rug_fuzz_1), "");
        debug_assert_eq!(symbolic_name_normalize(rug_fuzz_2), "Caf\u{e9}");
    }
}
#[cfg(test)]
mod tests_llm_16_1244 {
    use crate::symbolic_name_normalize_bytes;
    #[test]
    fn test_symbolic_name_normalize_bytes() {
        let rug_fuzz_0 = b"is_Lowercase";
        let rug_fuzz_0 = rug_fuzz_0;
        let mut input = rug_fuzz_0.to_vec();
        let result = symbolic_name_normalize_bytes(&mut input);
        debug_assert_eq!(result, b"lowercase");
    }
}
#[cfg(test)]
mod tests_llm_16_1245 {
    use super::*;
    use crate::*;
    use crate::unicode_tables::word_break::BY_NAME;
    use crate::hir::ClassUnicode;
    use crate::error::Error;
    #[test]
    fn test_wb_success() {
        let rug_fuzz_0 = "property_name";
        let rug_fuzz_0 = rug_fuzz_0;
        let result = wb(rug_fuzz_0);
        debug_assert_eq!(result, Ok(ClassUnicode::SomeUnicodeClass));
    }
    #[test]
    fn test_wb_property_not_found() {
        let rug_fuzz_0 = "nonexistent_property";
        let rug_fuzz_1 = "nonexistent_property";
        let rug_fuzz_0 = rug_fuzz_0;
        let rug_fuzz_1 = rug_fuzz_1;
        #[cfg(feature = "unicode-segment")]
        {
            let result = wb(rug_fuzz_0);
            debug_assert_eq!(result, Err(Error::PropertyValueNotFound));
        }
        #[cfg(not(feature = "unicode-segment"))]
        {
            let result = wb(rug_fuzz_1);
            debug_assert_eq!(result, Err(Error::PropertyNotFound));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1246 {
    use crate::imp;
    use crate::regex_syntax::unicode::hir::{ClassUnicode, Error};
    #[test]
    fn test_imp_valid_name() {
        let rug_fuzz_0 = "valid_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(matches!(imp(rug_fuzz_0), Ok(ClassUnicode)));
    }
    #[test]
    fn test_imp_invalid_name() {
        let rug_fuzz_0 = "invalid_name";
        let rug_fuzz_0 = rug_fuzz_0;
        debug_assert!(matches!(imp(rug_fuzz_0), Err(Error::PropertyValueNotFound)));
    }
}
