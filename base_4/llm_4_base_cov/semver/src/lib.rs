//! [![github]](https://github.com/dtolnay/semver)&ensp;[![crates-io]](https://crates.io/crates/semver)&ensp;[![docs-rs]](https://docs.rs/semver)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! A parser and evaluator for Cargo's flavor of Semantic Versioning.
//!
//! Semantic Versioning (see <https://semver.org>) is a guideline for how
//! version numbers are assigned and incremented. It is widely followed within
//! the Cargo/crates.io ecosystem for Rust.
//!
//! <br>
//!
//! # Example
//!
//! ```
//! use semver::{BuildMetadata, Prerelease, Version, VersionReq};
//!
//! fn main() {
//!     let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();
//!
//!     // Check whether this requirement matches version 1.2.3-alpha.1 (no)
//!     let version = Version {
//!         major: 1,
//!         minor: 2,
//!         patch: 3,
//!         pre: Prerelease::new("alpha.1").unwrap(),
//!         build: BuildMetadata::EMPTY,
//!     };
//!     assert!(!req.matches(&version));
//!
//!     // Check whether it matches 1.3.0 (yes it does)
//!     let version = Version::parse("1.3.0").unwrap();
//!     assert!(req.matches(&version));
//! }
//! ```
//!
//! <br><br>
//!
//! # Scope of this crate
//!
//! Besides Cargo, several other package ecosystems and package managers for
//! other languages also use SemVer:&ensp;RubyGems/Bundler for Ruby, npm for
//! JavaScript, Composer for PHP, CocoaPods for Objective-C...
//!
//! The `semver` crate is specifically intended to implement Cargo's
//! interpretation of Semantic Versioning.
//!
//! Where the various tools differ in their interpretation or implementation of
//! the spec, this crate follows the implementation choices made by Cargo. If
//! you are operating on version numbers from some other package ecosystem, you
//! will want to use a different semver library which is appropriate to that
//! ecosystem.
//!
//! The extent of Cargo's SemVer support is documented in the *[Specifying
//! Dependencies]* chapter of the Cargo reference.
//!
//! [Specifying Dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
#![doc(html_root_url = "https://docs.rs/semver/1.0.17")]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(no_unsafe_op_in_unsafe_fn_lint, allow(unused_unsafe))]
#![cfg_attr(no_str_strip_prefix, allow(unstable_name_collisions))]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::items_after_statements,
    clippy::manual_map,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::option_if_let_else,
    clippy::ptr_as_ptr,
    clippy::redundant_else,
    clippy::semicolon_if_nothing_returned,
    clippy::similar_names,
    clippy::unnested_or_patterns,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]
#[cfg(not(no_alloc_crate))]
extern crate alloc;
mod backport;
mod display;
mod error;
mod eval;
mod identifier;
mod impls;
mod parse;
#[cfg(feature = "serde")]
mod serde;
use crate::alloc::vec::Vec;
use crate::identifier::Identifier;
use core::str::FromStr;
#[allow(unused_imports)]
use crate::backport::*;
pub use crate::parse::Error;
/// **SemVer version** as defined by <https://semver.org>.
///
/// # Syntax
///
/// - The major, minor, and patch numbers may be any integer 0 through u64::MAX.
///   When representing a SemVer version as a string, each number is written as
///   a base 10 integer. For example, `1.0.119`.
///
/// - Leading zeros are forbidden in those positions. For example `1.01.00` is
///   invalid as a SemVer version.
///
/// - The pre-release identifier, if present, must conform to the syntax
///   documented for [`Prerelease`].
///
/// - The build metadata, if present, must conform to the syntax documented for
///   [`BuildMetadata`].
///
/// - Whitespace is not allowed anywhere in the version.
///
/// # Total ordering
///
/// Given any two SemVer versions, one is less than, greater than, or equal to
/// the other. Versions may be compared against one another using Rust's usual
/// comparison operators.
///
/// - The major, minor, and patch number are compared numerically from left to
/// right, lexicographically ordered as a 3-tuple of integers. So for example
/// version `1.5.0` is less than version `1.19.0`, despite the fact that
/// "1.19.0" &lt; "1.5.0" as ASCIIbetically compared strings and 1.19 &lt; 1.5
/// as real numbers.
///
/// - When major, minor, and patch are equal, a pre-release version is
///   considered less than the ordinary release:&ensp;version `1.0.0-alpha.1` is
///   less than version `1.0.0`.
///
/// - Two pre-releases of the same major, minor, patch are compared by
///   lexicographic ordering of dot-separated components of the pre-release
///   string.
///
///   - Identifiers consisting of only digits are compared
///     numerically:&ensp;`1.0.0-pre.8` is less than `1.0.0-pre.12`.
///
///   - Identifiers that contain a letter or hyphen are compared in ASCII sort
///     order:&ensp;`1.0.0-pre12` is less than `1.0.0-pre8`.
///
///   - Any numeric identifier is always less than any non-numeric
///     identifier:&ensp;`1.0.0-pre.1` is less than `1.0.0-pre.x`.
///
/// Example:&ensp;`1.0.0-alpha`&ensp;&lt;&ensp;`1.0.0-alpha.1`&ensp;&lt;&ensp;`1.0.0-alpha.beta`&ensp;&lt;&ensp;`1.0.0-beta`&ensp;&lt;&ensp;`1.0.0-beta.2`&ensp;&lt;&ensp;`1.0.0-beta.11`&ensp;&lt;&ensp;`1.0.0-rc.1`&ensp;&lt;&ensp;`1.0.0`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Prerelease,
    pub build: BuildMetadata,
}
/// **SemVer version requirement** describing the intersection of some version
/// comparators, such as `>=1.2.3, <1.8`.
///
/// # Syntax
///
/// - Either `*` (meaning "any"), or one or more comma-separated comparators.
///
/// - A [`Comparator`] is an operator ([`Op`]) and a partial version, separated
///   by optional whitespace. For example `>=1.0.0` or `>=1.0`.
///
/// - Build metadata is syntactically permitted on the partial versions, but is
///   completely ignored, as it's never relevant to whether any comparator
///   matches a particular version.
///
/// - Whitespace is permitted around commas and around operators. Whitespace is
///   not permitted within a partial version, i.e. anywhere between the major
///   version number and its minor, patch, pre-release, or build metadata.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(no_const_vec_new, derive(Default))]
pub struct VersionReq {
    pub comparators: Vec<Comparator>,
}
/// A pair of comparison operator and partial version, such as `>=1.2`. Forms
/// one piece of a VersionReq.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Comparator {
    pub op: Op,
    pub major: u64,
    pub minor: Option<u64>,
    /// Patch is only allowed if minor is Some.
    pub patch: Option<u64>,
    /// Non-empty pre-release is only allowed if patch is Some.
    pub pre: Prerelease,
}
/// SemVer comparison operator: `=`, `>`, `>=`, `<`, `<=`, `~`, `^`, `*`.
///
/// # Op::Exact
/// - &ensp;**`=I.J.K`**&emsp;&mdash;&emsp;exactly the version I.J.K
/// - &ensp;**`=I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.J.0, <I.(J+1).0`
/// - &ensp;**`=I`**&emsp;&mdash;&emsp;equivalent to `>=I.0.0, <(I+1).0.0`
///
/// # Op::Greater
/// - &ensp;**`>I.J.K`**
/// - &ensp;**`>I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.(J+1).0`
/// - &ensp;**`>I`**&emsp;&mdash;&emsp;equivalent to `>=(I+1).0.0`
///
/// # Op::GreaterEq
/// - &ensp;**`>=I.J.K`**
/// - &ensp;**`>=I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.J.0`
/// - &ensp;**`>=I`**&emsp;&mdash;&emsp;equivalent to `>=I.0.0`
///
/// # Op::Less
/// - &ensp;**`<I.J.K`**
/// - &ensp;**`<I.J`**&emsp;&mdash;&emsp;equivalent to `<I.J.0`
/// - &ensp;**`<I`**&emsp;&mdash;&emsp;equivalent to `<I.0.0`
///
/// # Op::LessEq
/// - &ensp;**`<=I.J.K`**
/// - &ensp;**`<=I.J`**&emsp;&mdash;&emsp;equivalent to `<I.(J+1).0`
/// - &ensp;**`<=I`**&emsp;&mdash;&emsp;equivalent to `<(I+1).0.0`
///
/// # Op::Tilde&emsp;("patch" updates)
/// *Tilde requirements allow the **patch** part of the semver version (the third number) to increase.*
/// - &ensp;**`~I.J.K`**&emsp;&mdash;&emsp;equivalent to `>=I.J.K, <I.(J+1).0`
/// - &ensp;**`~I.J`**&emsp;&mdash;&emsp;equivalent to `=I.J`
/// - &ensp;**`~I`**&emsp;&mdash;&emsp;equivalent to `=I`
///
/// # Op::Caret&emsp;("compatible" updates)
/// *Caret requirements allow parts that are **right of the first nonzero** part of the semver version to increase.*
/// - &ensp;**`^I.J.K`**&ensp;(for I\>0)&emsp;&mdash;&emsp;equivalent to `>=I.J.K, <(I+1).0.0`
/// - &ensp;**`^0.J.K`**&ensp;(for J\>0)&emsp;&mdash;&emsp;equivalent to `>=0.J.K, <0.(J+1).0`
/// - &ensp;**`^0.0.K`**&emsp;&mdash;&emsp;equivalent to `=0.0.K`
/// - &ensp;**`^I.J`**&ensp;(for I\>0 or J\>0)&emsp;&mdash;&emsp;equivalent to `^I.J.0`
/// - &ensp;**`^0.0`**&emsp;&mdash;&emsp;equivalent to `=0.0`
/// - &ensp;**`^I`**&emsp;&mdash;&emsp;equivalent to `=I`
///
/// # Op::Wildcard
/// - &ensp;**`I.J.*`**&emsp;&mdash;&emsp;equivalent to `=I.J`
/// - &ensp;**`I.*`**&ensp;or&ensp;**`I.*.*`**&emsp;&mdash;&emsp;equivalent to `=I`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(not(no_non_exhaustive), non_exhaustive)]
pub enum Op {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Tilde,
    Caret,
    Wildcard,
    #[cfg(no_non_exhaustive)]
    #[doc(hidden)]
    __NonExhaustive,
}
/// Optional pre-release identifier on a version string. This comes after `-` in
/// a SemVer version, like `1.0.0-alpha.1`
///
/// # Examples
///
/// Some real world pre-release idioms drawn from crates.io:
///
/// - **[mio]** <code>0.7.0-<b>alpha.1</b></code> &mdash; the most common style
///   for numbering pre-releases.
///
/// - **[pest]** <code>1.0.0-<b>beta.8</b></code>,&ensp;<code>1.0.0-<b>rc.0</b></code>
///   &mdash; this crate makes a distinction between betas and release
///   candidates.
///
/// - **[sassers]** <code>0.11.0-<b>shitshow</b></code> &mdash; ???.
///
/// - **[atomic-utils]** <code>0.0.0-<b>reserved</b></code> &mdash; a squatted
///   crate name.
///
/// [mio]: https://crates.io/crates/mio
/// [pest]: https://crates.io/crates/pest
/// [atomic-utils]: https://crates.io/crates/atomic-utils
/// [sassers]: https://crates.io/crates/sassers
///
/// *Tip:* Be aware that if you are planning to number your own pre-releases,
/// you should prefer to separate the numeric part from any non-numeric
/// identifiers by using a dot in between. That is, prefer pre-releases
/// `alpha.1`, `alpha.2`, etc rather than `alpha1`, `alpha2` etc. The SemVer
/// spec's rule for pre-release precedence has special treatment of numeric
/// components in the pre-release string, but only if there are no non-digit
/// characters in the same dot-separated component. So you'd have `alpha.2` &lt;
/// `alpha.11` as intended, but `alpha11` &lt; `alpha2`.
///
/// # Syntax
///
/// Pre-release strings are a series of dot separated identifiers immediately
/// following the patch version. Identifiers must comprise only ASCII
/// alphanumerics and hyphens: `0-9`, `A-Z`, `a-z`, `-`. Identifiers must not be
/// empty. Numeric identifiers must not include leading zeros.
///
/// # Total ordering
///
/// Pre-releases have a total order defined by the SemVer spec. It uses
/// lexicographic ordering of dot-separated components. Identifiers consisting
/// of only digits are compared numerically. Otherwise, identifiers are compared
/// in ASCII sort order. Any numeric identifier is always less than any
/// non-numeric identifier.
///
/// Example:&ensp;`alpha`&ensp;&lt;&ensp;`alpha.85`&ensp;&lt;&ensp;`alpha.90`&ensp;&lt;&ensp;`alpha.200`&ensp;&lt;&ensp;`alpha.0a`&ensp;&lt;&ensp;`alpha.1a0`&ensp;&lt;&ensp;`alpha.a`&ensp;&lt;&ensp;`beta`
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Prerelease {
    identifier: Identifier,
}
/// Optional build metadata identifier. This comes after `+` in a SemVer
/// version, as in `0.8.1+zstd.1.5.0`.
///
/// # Examples
///
/// Some real world build metadata idioms drawn from crates.io:
///
/// - **[libgit2-sys]** <code>0.12.20+<b>1.1.0</b></code> &mdash; for this
///   crate, the build metadata indicates the version of the C libgit2 library
///   that the Rust crate is built against.
///
/// - **[mashup]** <code>0.1.13+<b>deprecated</b></code> &mdash; just the word
///   "deprecated" for a crate that has been superseded by another. Eventually
///   people will take notice of this in Cargo's build output where it lists the
///   crates being compiled.
///
/// - **[google-bigquery2]** <code>2.0.4+<b>20210327</b></code> &mdash; this
///   library is automatically generated from an official API schema, and the
///   build metadata indicates the date on which that schema was last captured.
///
/// - **[fbthrift-git]** <code>0.0.6+<b>c7fcc0e</b></code> &mdash; this crate is
///   published from snapshots of a big company monorepo. In monorepo
///   development, there is no concept of versions, and all downstream code is
///   just updated atomically in the same commit that breaking changes to a
///   library are landed. Therefore for crates.io purposes, every published
///   version must be assumed to be incompatible with the previous. The build
///   metadata provides the source control hash of the snapshotted code.
///
/// [libgit2-sys]: https://crates.io/crates/libgit2-sys
/// [mashup]: https://crates.io/crates/mashup
/// [google-bigquery2]: https://crates.io/crates/google-bigquery2
/// [fbthrift-git]: https://crates.io/crates/fbthrift-git
///
/// # Syntax
///
/// Build metadata is a series of dot separated identifiers immediately
/// following the patch or pre-release version. Identifiers must comprise only
/// ASCII alphanumerics and hyphens: `0-9`, `A-Z`, `a-z`, `-`. Identifiers must
/// not be empty. Leading zeros *are* allowed, unlike any other place in the
/// SemVer grammar.
///
/// # Total ordering
///
/// Build metadata is ignored in evaluating `VersionReq`; it plays no role in
/// whether a `Version` matches any one of the comparison operators.
///
/// However for comparing build metadatas among one another, they do have a
/// total order which is determined by lexicographic ordering of dot-separated
/// components. Identifiers consisting of only digits are compared numerically.
/// Otherwise, identifiers are compared in ASCII sort order. Any numeric
/// identifier is always less than any non-numeric identifier.
///
/// Example:&ensp;`demo`&ensp;&lt;&ensp;`demo.85`&ensp;&lt;&ensp;`demo.90`&ensp;&lt;&ensp;`demo.090`&ensp;&lt;&ensp;`demo.200`&ensp;&lt;&ensp;`demo.1a0`&ensp;&lt;&ensp;`demo.a`&ensp;&lt;&ensp;`memo`
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BuildMetadata {
    identifier: Identifier,
}
impl Version {
    /// Create `Version` with an empty pre-release and build metadata.
    ///
    /// Equivalent to:
    ///
    /// ```
    /// # use semver::{BuildMetadata, Prerelease, Version};
    /// #
    /// # const fn new(major: u64, minor: u64, patch: u64) -> Version {
    /// Version {
    ///     major,
    ///     minor,
    ///     patch,
    ///     pre: Prerelease::EMPTY,
    ///     build: BuildMetadata::EMPTY,
    /// }
    /// # }
    /// ```
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version {
            major,
            minor,
            patch,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
    }
    /// Create `Version` by parsing from string representation.
    ///
    /// # Errors
    ///
    /// Possible reasons for the parse to fail include:
    ///
    /// - `1.0` &mdash; too few numeric components. A SemVer version must have
    ///   exactly three. If you are looking at something that has fewer than
    ///   three numbers in it, it's possible it is a `VersionReq` instead (with
    ///   an implicit default `^` comparison operator).
    ///
    /// - `1.0.01` &mdash; a numeric component has a leading zero.
    ///
    /// - `1.0.unknown` &mdash; unexpected character in one of the components.
    ///
    /// - `1.0.0-` or `1.0.0+` &mdash; the pre-release or build metadata are
    ///   indicated present but empty.
    ///
    /// - `1.0.0-alpha_123` &mdash; pre-release or build metadata have something
    ///   outside the allowed characters, which are `0-9`, `A-Z`, `a-z`, `-`,
    ///   and `.` (dot).
    ///
    /// - `23456789999999999999.0.0` &mdash; overflow of a u64.
    pub fn parse(text: &str) -> Result<Self, Error> {
        Version::from_str(text)
    }
}
impl VersionReq {
    /// A `VersionReq` with no constraint on the version numbers it matches.
    /// Equivalent to `VersionReq::parse("*").unwrap()`.
    ///
    /// In terms of comparators this is equivalent to `>=0.0.0`.
    ///
    /// Counterintuitively a `*` VersionReq does not match every possible
    /// version number. In particular, in order for *any* `VersionReq` to match
    /// a pre-release version, the `VersionReq` must contain at least one
    /// `Comparator` that has an explicit major, minor, and patch version
    /// identical to the pre-release being matched, and that has a nonempty
    /// pre-release component. Since `*` is not written with an explicit major,
    /// minor, and patch version, and does not contain a nonempty pre-release
    /// component, it does not match any pre-release versions.
    #[cfg(not(no_const_vec_new))]
    pub const STAR: Self = VersionReq {
        comparators: Vec::new(),
    };
    /// Create `VersionReq` by parsing from string representation.
    ///
    /// # Errors
    ///
    /// Possible reasons for the parse to fail include:
    ///
    /// - `>a.b` &mdash; unexpected characters in the partial version.
    ///
    /// - `@1.0.0` &mdash; unrecognized comparison operator.
    ///
    /// - `^1.0.0, ` &mdash; unexpected end of input.
    ///
    /// - `>=1.0 <2.0` &mdash; missing comma between comparators.
    ///
    /// - `*.*` &mdash; unsupported wildcard syntax.
    pub fn parse(text: &str) -> Result<Self, Error> {
        VersionReq::from_str(text)
    }
    /// Evaluate whether the given `Version` satisfies the version requirement
    /// described by `self`.
    pub fn matches(&self, version: &Version) -> bool {
        eval::matches_req(self, version)
    }
}
/// The default VersionReq is the same as [`VersionReq::STAR`].
#[cfg(not(no_const_vec_new))]
impl Default for VersionReq {
    fn default() -> Self {
        VersionReq::STAR
    }
}
impl Comparator {
    pub fn parse(text: &str) -> Result<Self, Error> {
        Comparator::from_str(text)
    }
    pub fn matches(&self, version: &Version) -> bool {
        eval::matches_comparator(self, version)
    }
}
impl Prerelease {
    pub const EMPTY: Self = Prerelease {
        identifier: Identifier::empty(),
    };
    pub fn new(text: &str) -> Result<Self, Error> {
        Prerelease::from_str(text)
    }
    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }
    pub fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
}
impl BuildMetadata {
    pub const EMPTY: Self = BuildMetadata {
        identifier: Identifier::empty(),
    };
    pub fn new(text: &str) -> Result<Self, Error> {
        BuildMetadata::from_str(text)
    }
    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }
    pub fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    #[test]
    fn test_versionreq_default() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_versionreq_default = 0;
        let rug_fuzz_0 = "1.0.0";
        let rug_fuzz_1 = "0.0.1";
        let rug_fuzz_2 = "9999.9999.9999";
        let rug_fuzz_3 = "1.0.0-alpha";
        let rug_fuzz_4 = "1.0.0-beta";
        let default_versionreq = VersionReq::default();
        debug_assert_eq!(default_versionreq.comparators.len(), 0);
        debug_assert!(default_versionreq.matches(& Version::parse(rug_fuzz_0).unwrap()));
        debug_assert!(default_versionreq.matches(& Version::parse(rug_fuzz_1).unwrap()));
        debug_assert!(default_versionreq.matches(& Version::parse(rug_fuzz_2).unwrap()));
        debug_assert!(
            ! default_versionreq.matches(& Version::parse(rug_fuzz_3).unwrap())
        );
        debug_assert!(
            ! default_versionreq.matches(& Version::parse(rug_fuzz_4).unwrap())
        );
        debug_assert_eq!(format!("{}", default_versionreq), "*");
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_versionreq_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_build_metadata_as_str() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_build_metadata_as_str = 0;
        let rug_fuzz_0 = "12345";
        let rug_fuzz_1 = "";
        let metadata_str = rug_fuzz_0;
        let metadata = BuildMetadata::from_str(metadata_str).unwrap();
        debug_assert_eq!(metadata.as_str(), metadata_str);
        let empty_metadata = BuildMetadata::from_str(rug_fuzz_1).unwrap();
        debug_assert!(empty_metadata.as_str().is_empty());
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_build_metadata_as_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use crate::BuildMetadata;
    use std::str::FromStr;
    #[test]
    fn test_is_empty_with_empty_metadata() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_empty_metadata = 0;
        let metadata = BuildMetadata::EMPTY;
        debug_assert!(metadata.is_empty());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_empty_metadata = 0;
    }
    #[test]
    fn test_is_empty_with_non_empty_metadata() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_non_empty_metadata = 0;
        let rug_fuzz_0 = "1.0.0";
        let metadata = BuildMetadata::from_str(rug_fuzz_0).unwrap();
        debug_assert!(! metadata.is_empty());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_non_empty_metadata = 0;
    }
    #[test]
    fn test_is_empty_with_newly_created_empty_metadata() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_newly_created_empty_metadata = 0;
        let rug_fuzz_0 = "";
        let metadata = BuildMetadata::new(rug_fuzz_0).unwrap();
        debug_assert!(metadata.is_empty());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_newly_created_empty_metadata = 0;
    }
    #[test]
    fn test_is_empty_with_newly_created_non_empty_metadata() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_newly_created_non_empty_metadata = 0;
        let rug_fuzz_0 = "build.123";
        let metadata = BuildMetadata::new(rug_fuzz_0).unwrap();
        debug_assert!(! metadata.is_empty());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_is_empty_with_newly_created_non_empty_metadata = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use crate::BuildMetadata;
    use std::str::FromStr;
    #[test]
    fn test_build_metadata_new_valid() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_build_metadata_new_valid = 0;
        let rug_fuzz_0 = "001";
        let text = rug_fuzz_0;
        debug_assert!(BuildMetadata::new(text).is_ok());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_build_metadata_new_valid = 0;
    }
    #[test]
    fn test_build_metadata_new_empty() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_build_metadata_new_empty = 0;
        let rug_fuzz_0 = "";
        let text = rug_fuzz_0;
        debug_assert!(BuildMetadata::new(text).is_ok());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_build_metadata_new_empty = 0;
    }
    #[test]
    fn test_build_metadata_new_invalid() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_build_metadata_new_invalid = 0;
        let rug_fuzz_0 = "!InvalidMetadata";
        let text = rug_fuzz_0;
        debug_assert!(BuildMetadata::new(text).is_err());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_build_metadata_new_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    #[test]
    fn comparator_matches_version_exact() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_exact = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let comparator = Comparator {
            op: Op::Exact,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert!(comparator.matches(& version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_exact = 0;
    }
    #[test]
    fn comparator_matches_version_greater() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_greater = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 9;
        let rug_fuzz_5 = 9;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let comparator = Comparator {
            op: Op::Greater,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let lower_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let higher_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(! comparator.matches(& lower_version));
        debug_assert!(comparator.matches(& higher_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_greater = 0;
    }
    #[test]
    fn comparator_matches_version_greater_eq() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_greater_eq = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let comparator = Comparator {
            op: Op::GreaterEq,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let equal_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let higher_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(comparator.matches(& equal_version));
        debug_assert!(comparator.matches(& higher_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_greater_eq = 0;
    }
    #[test]
    fn comparator_matches_version_less() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_less = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 9;
        let rug_fuzz_5 = 9;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let comparator = Comparator {
            op: Op::Less,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let lower_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let equal_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(comparator.matches(& lower_version));
        debug_assert!(! comparator.matches(& equal_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_less = 0;
    }
    #[test]
    fn comparator_matches_version_less_eq() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_less_eq = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 9;
        let rug_fuzz_5 = 9;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let comparator = Comparator {
            op: Op::LessEq,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let lower_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let equal_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(comparator.matches(& lower_version));
        debug_assert!(comparator.matches(& equal_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_less_eq = 0;
    }
    #[test]
    fn comparator_matches_version_tilde() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_tilde = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 0;
        let comparator = Comparator {
            op: Op::Tilde,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let patch_update_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let minor_update_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(comparator.matches(& patch_update_version));
        debug_assert!(! comparator.matches(& minor_update_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_tilde = 0;
    }
    #[test]
    fn comparator_matches_version_caret() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_caret = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let comparator = Comparator {
            op: Op::Caret,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let minor_update_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let major_update_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(comparator.matches(& minor_update_version));
        debug_assert!(! comparator.matches(& major_update_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_caret = 0;
    }
    #[test]
    fn comparator_matches_version_wildcard() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_wildcard = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let comparator = Comparator {
            op: Op::Wildcard,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let minor_update_version = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let major_update_version = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(comparator.matches(& minor_update_version));
        debug_assert!(comparator.matches(& major_update_version));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_wildcard = 0;
    }
    #[test]
    fn comparator_matches_version_with_pre_release() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_with_pre_release = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "alpha.1";
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = "alpha.1";
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let comparator = Comparator {
            op: Op::Exact,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::new(rug_fuzz_3).unwrap(),
        };
        let version_with_pre = Version {
            major: rug_fuzz_4,
            minor: rug_fuzz_5,
            patch: rug_fuzz_6,
            pre: Prerelease::new(rug_fuzz_7).unwrap(),
            build: BuildMetadata::EMPTY,
        };
        let version_without_pre = Version::new(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10);
        debug_assert!(comparator.matches(& version_with_pre));
        debug_assert!(! comparator.matches(& version_without_pre));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_with_pre_release = 0;
    }
    #[test]
    fn comparator_matches_version_with_build_metadata() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_with_build_metadata = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "alpha.1";
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = "alpha.1";
        let rug_fuzz_8 = "20210327";
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = "alpha.1";
        let comparator = Comparator {
            op: Op::Exact,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::new(rug_fuzz_3).unwrap(),
        };
        let version_with_build = Version {
            major: rug_fuzz_4,
            minor: rug_fuzz_5,
            patch: rug_fuzz_6,
            pre: Prerelease::new(rug_fuzz_7).unwrap(),
            build: BuildMetadata::new(rug_fuzz_8).unwrap(),
        };
        let version_without_build = Version {
            major: rug_fuzz_9,
            minor: rug_fuzz_10,
            patch: rug_fuzz_11,
            pre: Prerelease::new(rug_fuzz_12).unwrap(),
            build: BuildMetadata::EMPTY,
        };
        debug_assert!(comparator.matches(& version_with_build));
        debug_assert!(comparator.matches(& version_without_build));
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_comparator_matches_version_with_build_metadata = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    #[test]
    fn parse_valid_version() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_parse_valid_version = 0;
        let rug_fuzz_0 = "1.0.0";
        let rug_fuzz_1 = "1.0";
        let rug_fuzz_2 = "^1.2.3";
        let rug_fuzz_3 = "~1";
        let rug_fuzz_4 = ">=1.0.0";
        let rug_fuzz_5 = "<1.2.3";
        let rug_fuzz_6 = "=1.0.0";
        let test_cases = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
        ];
        for &version in &test_cases {
            debug_assert!(Comparator::parse(version).is_ok());
        }
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_parse_valid_version = 0;
    }
    #[test]
    fn parse_invalid_version() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_parse_invalid_version = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "not a version";
        let rug_fuzz_2 = "1.2.3.4";
        let rug_fuzz_3 = "01.0.0";
        let rug_fuzz_4 = "1.0.0-pre+build";
        let test_cases = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        for &version in &test_cases {
            debug_assert!(Comparator::parse(version).is_err());
        }
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_parse_invalid_version = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_prerelease_as_str() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_test_prerelease_as_str = 0;
        let rug_fuzz_0 = "alpha.1";
        let rug_fuzz_1 = "beta";
        let rug_fuzz_2 = "rc.0";
        let rug_fuzz_3 = "rc.1";
        let rug_fuzz_4 = "rc.1.2";
        let rug_fuzz_5 = "1.2.3";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(prerelease.as_str(), "alpha.1");
        let prerelease = Prerelease::from_str(rug_fuzz_1).unwrap();
        debug_assert_eq!(prerelease.as_str(), "beta");
        let prerelease = Prerelease::from_str(rug_fuzz_2).unwrap();
        debug_assert_eq!(prerelease.as_str(), "rc.0");
        let prerelease = Prerelease::from_str(rug_fuzz_3).unwrap();
        debug_assert_eq!(prerelease.as_str(), "rc.1");
        let prerelease = Prerelease::from_str(rug_fuzz_4).unwrap();
        debug_assert_eq!(prerelease.as_str(), "rc.1.2");
        let prerelease = Prerelease::from_str(rug_fuzz_5).unwrap();
        debug_assert_eq!(prerelease.as_str(), "1.2.3");
        let empty_prerelease = Prerelease::EMPTY;
        debug_assert!(empty_prerelease.as_str().is_empty());
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_test_prerelease_as_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use crate::Prerelease;
    use std::str::FromStr;
    #[test]
    fn test_prerelease_is_empty_with_empty_string() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_empty_string = 0;
        let rug_fuzz_0 = "";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert!(prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_empty_string = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_non_empty_string() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_non_empty_string = 0;
        let rug_fuzz_0 = "alpha";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert!(! prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_non_empty_string = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_numeric() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_numeric = 0;
        let rug_fuzz_0 = "123";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert!(! prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_numeric = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_mixed() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_mixed = 0;
        let rug_fuzz_0 = "alpha.1";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert!(! prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_mixed = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_hyphens() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_hyphens = 0;
        let rug_fuzz_0 = "alpha-beta";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert!(! prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_hyphens = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_empty_prerelease() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_empty_prerelease = 0;
        let rug_fuzz_0 = "0";
        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert!(! prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_empty_prerelease = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_inline() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_inline = 0;
        let rug_fuzz_0 = "0";
        let prerelease = Prerelease::new(rug_fuzz_0).unwrap();
        debug_assert!(! prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_inline = 0;
    }
    #[test]
    fn test_prerelease_is_empty_with_empty_struct() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_empty_struct = 0;
        let prerelease = Prerelease::EMPTY;
        debug_assert!(prerelease.is_empty());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_prerelease_is_empty_with_empty_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use crate::{Prerelease, Error};
    #[test]
    fn test_prerelease_new_valid() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_valid = 0;
        let rug_fuzz_0 = "alpha.1";
        let valid_prerelease = rug_fuzz_0;
        debug_assert!(Prerelease::new(valid_prerelease).is_ok());
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_valid = 0;
    }
    #[test]
    fn test_prerelease_new_empty() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_empty = 0;
        let rug_fuzz_0 = "";
        let empty_prerelease = rug_fuzz_0;
        debug_assert!(Prerelease::new(empty_prerelease).is_ok());
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_empty = 0;
    }
    #[test]
    fn test_prerelease_new_invalid() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_invalid = 0;
        let rug_fuzz_0 = "!!invalid!!";
        let invalid_prerelease = rug_fuzz_0;
        debug_assert!(Prerelease::new(invalid_prerelease).is_err());
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_invalid = 0;
    }
    #[test]
    fn test_prerelease_new_invalid_empty_numeric() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_invalid_empty_numeric = 0;
        let rug_fuzz_0 = "1.";
        let invalid_empty_numeric = rug_fuzz_0;
        debug_assert!(Prerelease::new(invalid_empty_numeric).is_err());
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_invalid_empty_numeric = 0;
    }
    #[test]
    fn test_prerelease_new_invalid_leading_zero() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_invalid_leading_zero = 0;
        let rug_fuzz_0 = "01";
        let invalid_leading_zero = rug_fuzz_0;
        debug_assert!(Prerelease::new(invalid_leading_zero).is_err());
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_prerelease_new_invalid_leading_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_13_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3;
        let version = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(rug_fuzz_3, version.major);
        debug_assert_eq!(rug_fuzz_4, version.minor);
        debug_assert_eq!(rug_fuzz_5, version.patch);
        debug_assert!(version.pre.is_empty());
        debug_assert!(version.build.is_empty());
        let _rug_ed_tests_llm_16_13_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_14 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_parse_valid_versions() {
        let _rug_st_tests_llm_16_14_rrrruuuugggg_test_parse_valid_versions = 0;
        let rug_fuzz_0 = "0.0.0";
        let valid_versions = vec![
            rug_fuzz_0, "1.0.0", "1.2.3", "10.20.30", "1.2.3-alpha", "1.2.3-beta.1",
            "1.2.3-x.7.z.92", "1.2.3+20130313144700", "1.2.3-beta+exp.sha.5114f85",
            "1.0.0-alpha.beta", "1.0.0+0.build.1-rc.10000aaa-kk-0.1"
        ];
        for ver in valid_versions {
            debug_assert!(Version::parse(ver).is_ok());
        }
        let _rug_ed_tests_llm_16_14_rrrruuuugggg_test_parse_valid_versions = 0;
    }
    #[test]
    fn test_parse_invalid_versions() {
        let _rug_st_tests_llm_16_14_rrrruuuugggg_test_parse_invalid_versions = 0;
        let rug_fuzz_0 = "";
        let invalid_versions = vec![
            rug_fuzz_0, "1", "1.0", "01.0.0", "1.0.01", "1.0.unknown", "1.0.0-",
            "1.0.0+", "1.0.0-alpha_123", "1.0.0+*", "1.0.0+!", "23456789999999999999.0.0"
        ];
        for ver in invalid_versions {
            debug_assert!(Version::parse(ver).is_err());
        }
        let _rug_ed_tests_llm_16_14_rrrruuuugggg_test_parse_invalid_versions = 0;
    }
    #[test]
    fn test_parse_edge_cases() {
        let _rug_st_tests_llm_16_14_rrrruuuugggg_test_parse_edge_cases = 0;
        let rug_fuzz_0 = "0.0.0-0";
        let rug_fuzz_1 = "0.0.0+0";
        let rug_fuzz_2 = "1.0.0-";
        let rug_fuzz_3 = "1.0.0+";
        debug_assert!(Version::parse(rug_fuzz_0).is_ok());
        debug_assert!(Version::parse(rug_fuzz_1).is_ok());
        debug_assert!(Version::parse(rug_fuzz_2).is_err());
        debug_assert!(Version::parse(rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_14_rrrruuuugggg_test_parse_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;
    use crate::*;
    use crate::{Version, VersionReq};
    #[test]
    fn test_matches_exact() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_exact = 0;
        let rug_fuzz_0 = "=1.2.3";
        let rug_fuzz_1 = "1.2.3";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_exact = 0;
    }
    #[test]
    fn test_matches_wildcard() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_wildcard = 0;
        let rug_fuzz_0 = "*";
        let rug_fuzz_1 = "1.2.3";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_wildcard = 0;
    }
    #[test]
    fn test_matches_major_wildcard() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_major_wildcard = 0;
        let rug_fuzz_0 = "1.*";
        let rug_fuzz_1 = "1.2.3";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_major_wildcard = 0;
    }
    #[test]
    fn test_matches_minor_wildcard() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_minor_wildcard = 0;
        let rug_fuzz_0 = "1.2.*";
        let rug_fuzz_1 = "1.2.3";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_minor_wildcard = 0;
    }
    #[test]
    fn test_does_not_match() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_does_not_match = 0;
        let rug_fuzz_0 = "=1.2.3";
        let rug_fuzz_1 = "2.0.0";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(! req.matches(& ver));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_does_not_match = 0;
    }
    #[test]
    fn test_matches_range() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_range = 0;
        let rug_fuzz_0 = ">=1.2.3, <2.0.0";
        let rug_fuzz_1 = "1.2.3";
        let rug_fuzz_2 = "2.0.0";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver_in_range = Version::parse(rug_fuzz_1).unwrap();
        let ver_out_of_range = Version::parse(rug_fuzz_2).unwrap();
        debug_assert!(req.matches(& ver_in_range));
        debug_assert!(! req.matches(& ver_out_of_range));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_range = 0;
    }
    #[test]
    fn test_matches_prerelease() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_prerelease = 0;
        let rug_fuzz_0 = ">=1.2.3-rc1";
        let rug_fuzz_1 = "1.2.3-rc1";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver_prerelease = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver_prerelease));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_prerelease = 0;
    }
    #[test]
    fn test_does_not_match_prerelease() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_does_not_match_prerelease = 0;
        let rug_fuzz_0 = "=1.2.3";
        let rug_fuzz_1 = "1.2.3-rc1";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver_prerelease = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(! req.matches(& ver_prerelease));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_does_not_match_prerelease = 0;
    }
    #[test]
    fn test_matches_build() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_build = 0;
        let rug_fuzz_0 = ">=1.2.3+build201";
        let rug_fuzz_1 = "1.2.3+build201";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver_build = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver_build));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_build = 0;
    }
    #[test]
    fn test_matches_ignore_build() {
        let _rug_st_tests_llm_16_15_rrrruuuugggg_test_matches_ignore_build = 0;
        let rug_fuzz_0 = ">=1.2.3";
        let rug_fuzz_1 = "1.2.3+build201";
        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver_build = Version::parse(rug_fuzz_1).unwrap();
        debug_assert!(req.matches(& ver_build));
        let _rug_ed_tests_llm_16_15_rrrruuuugggg_test_matches_ignore_build = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_16 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_parse_valid_version_req() {
        let _rug_st_tests_llm_16_16_rrrruuuugggg_test_parse_valid_version_req = 0;
        let rug_fuzz_0 = "1.0.0";
        let inputs = vec![
            rug_fuzz_0, ">= 1.0.0, < 2.0.0", ">=1", "<2.1.1", "=2.1.1", "^0.1.2", "~1",
            "~1.2.3-beta"
        ];
        for input in inputs {
            debug_assert!(VersionReq::parse(input).is_ok());
        }
        let _rug_ed_tests_llm_16_16_rrrruuuugggg_test_parse_valid_version_req = 0;
    }
    #[test]
    fn test_parse_invalid_version_req() {
        let _rug_st_tests_llm_16_16_rrrruuuugggg_test_parse_invalid_version_req = 0;
        let rug_fuzz_0 = ">= 1.0.0, <2.0";
        let inputs = vec![
            rug_fuzz_0, ">= 1.0.0 <2.0.0", "1.0", ">a.b", "@1.0.0", "^1.0.0, ",
            ">=1.0 <2.0", "*.*"
        ];
        for input in inputs {
            debug_assert!(VersionReq::parse(input).is_err());
        }
        let _rug_ed_tests_llm_16_16_rrrruuuugggg_test_parse_invalid_version_req = 0;
    }
}
