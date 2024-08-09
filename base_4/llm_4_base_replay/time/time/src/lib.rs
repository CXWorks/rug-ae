//! # Feature flags
//!
//! This crate exposes a number of features. These can be enabled or disabled as shown
//! [in Cargo's documentation](https://doc.rust-lang.org/cargo/reference/features.html). Features
//! are _disabled_ by default unless otherwise noted.
//!
//! Reliance on a given feature is always indicated alongside the item definition.
//!
//! - `std` (_enabled by default, implicitly enables `alloc`_)
//!
//!   This enables a number of features that depend on the standard library.
//!
//! - `alloc` (_enabled by default via `std`_)
//!
//!   Enables a number of features that require the ability to dynamically allocate memory.
//!
//! - `macros`
//!
//!   Enables macros that provide compile-time verification of values and intuitive syntax.
//!
//! - `formatting` (_implicitly enables `std`_)
//!
//!   Enables formatting of most structs.
//!
//! - `parsing`
//!
//!   Enables parsing of most structs.
//!
//! - `local-offset` (_implicitly enables `std`_)
//!
//!   This feature enables a number of methods that allow obtaining the system's UTC offset.
//!
//! - `large-dates`
//!
//!   By default, only years within the ±9999 range (inclusive) are supported. If you need support
//!   for years outside this range, consider enabling this feature; the supported range will be
//!   increased to ±999,999.
//!
//!   Note that enabling this feature has some costs, as it means forgoing some optimizations.
//!   Ambiguities may be introduced when parsing that would not otherwise exist.
//!
//! - `serde`
//!
//!   Enables [serde](https://docs.rs/serde) support for all types except [`Instant`].
//!
//! - `serde-human-readable` (_implicitly enables `serde`, `formatting`, and `parsing`_)
//!
//!   Allows serde representations to use a human-readable format. This is determined by the
//!   serializer, not the user. If this feature is not enabled or if the serializer requests a
//!   non-human-readable format, a format optimized for binary representation will be used.
//!
//!   Libraries should never enable this feature, as the decision of what format to use should be up
//!   to the user.
//!
//! - `serde-well-known` (_implicitly enables `serde-human-readable`_)
//!
//!   _This feature flag is deprecated and will be removed in a future breaking release. Use the
//!   `serde-human-readable` feature instead._
//!
//!   Enables support for serializing and deserializing well-known formats using serde's
//!   [`#[with]` attribute](https://serde.rs/field-attrs.html#with).
//!
//! - `rand`
//!
//!   Enables [rand](https://docs.rs/rand) support for all types.
//!
//! - `quickcheck` (_implicitly enables `alloc`_)
//!
//!   Enables [quickcheck](https://docs.rs/quickcheck) support for all types except [`Instant`].
//!
//! - `wasm-bindgen`
//!
//!   Enables [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) support for converting
//!   [JavaScript dates](https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Date.html), as
//!   well as obtaining the UTC offset from JavaScript.
#![doc(html_playground_url = "https://play.rust-lang.org")]
#![cfg_attr(__time_03_docs, feature(doc_auto_cfg, doc_notable_trait))]
#![cfg_attr(coverage_nightly, feature(no_coverage))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::uninlined_format_args,
    clippy::unnested_or_patterns,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::redundant_pub_crate,
    clippy::option_if_let_else,
    clippy::unused_peekable,
    clippy::std_instead_of_core,
)]
#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55999857")]
#[allow(unused_extern_crates)]
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(unsound_local_offset)]
compile_error!(
    "The `unsound_local_offset` flag was removed in time 0.3.18. If you need this functionality, \
     see the `time::util::local_offset::set_soundness` function."
);
/// Helper macro for easily implementing `OpAssign`.
macro_rules! __impl_assign {
    ($sym:tt $op:ident $fn:ident $target:ty : $($(#[$attr:meta])* $t:ty),+) => {
        $(#[allow(unused_qualifications)] $(#[$attr])* impl core::ops::$op <$t > for
        $target { fn $fn (& mut self, rhs : $t) { * self = * self $sym rhs; } })+
    };
}
/// Implement `AddAssign` for the provided types.
macro_rules! impl_add_assign {
    ($target:ty : $($(#[$attr:meta])* $t:ty),+ $(,)?) => {
        __impl_assign!(+ AddAssign add_assign $target : $($(#[$attr])* $t),+);
    };
}
/// Implement `SubAssign` for the provided types.
macro_rules! impl_sub_assign {
    ($target:ty : $($(#[$attr:meta])* $t:ty),+ $(,)?) => {
        __impl_assign!(- SubAssign sub_assign $target : $($(#[$attr])* $t),+);
    };
}
/// Implement `MulAssign` for the provided types.
macro_rules! impl_mul_assign {
    ($target:ty : $($(#[$attr:meta])* $t:ty),+ $(,)?) => {
        __impl_assign!(* MulAssign mul_assign $target : $($(#[$attr])* $t),+);
    };
}
/// Implement `DivAssign` for the provided types.
macro_rules! impl_div_assign {
    ($target:ty : $($(#[$attr:meta])* $t:ty),+ $(,)?) => {
        __impl_assign!(/ DivAssign div_assign $target : $($(#[$attr])* $t),+);
    };
}
/// Division of integers, rounding the resulting value towards negative infinity.
macro_rules! div_floor {
    ($a:expr, $b:expr) => {
        { let _a = $a; let _b = $b; let (_quotient, _remainder) = (_a / _b, _a % _b); if
        (_remainder > 0 && _b < 0) || (_remainder < 0 && _b > 0) { _quotient - 1 } else {
        _quotient } }
    };
}
/// Cascade an out-of-bounds value.
macro_rules! cascade {
    (@ ordinal ordinal) => {};
    (@ year year) => {};
    ($from:ident in $min:literal .. $max:expr => $to:tt) => {
        #[allow(unused_comparisons, unused_assignments)] let min = $min; let max = $max;
        if $from >= max { $from -= max - min; $to += 1; } else if $from < min { $from +=
        max - min; $to -= 1; }
    };
    ($ordinal:ident => $year:ident) => {
        cascade!(@ ordinal $ordinal); cascade!(@ year $year);
        #[allow(unused_assignments)] if $ordinal > crate ::util::days_in_year($year) as
        i16 { $ordinal -= crate ::util::days_in_year($year) as i16; $year += 1; } else if
        $ordinal < 1 { $year -= 1; $ordinal += crate ::util::days_in_year($year) as i16;
        }
    };
}
/// Returns `Err(error::ComponentRange)` if the value is not in range.
macro_rules! ensure_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        { let _start = $start; let _end = $end; #[allow(trivial_numeric_casts,
        unused_comparisons)] if $value < _start || $value > _end { return Err(crate
        ::error::ComponentRange { name : stringify!($value), minimum : _start as _,
        maximum : _end as _, value : $value as _, conditional_range : false, }); } }
    };
    ($value:ident conditionally in $start:expr => $end:expr) => {
        { let _start = $start; let _end = $end; #[allow(trivial_numeric_casts,
        unused_comparisons)] if $value < _start || $value > _end { return Err(crate
        ::error::ComponentRange { name : stringify!($value), minimum : _start as _,
        maximum : _end as _, value : $value as _, conditional_range : true, }); } }
    };
}
/// Try to unwrap an expression, returning if not possible.
///
/// This is similar to the `?` operator, but does not perform `.into()`. Because of this, it is
/// usable in `const` contexts.
macro_rules! const_try {
    ($e:expr) => {
        match $e { Ok(value) => value, Err(error) => return Err(error), }
    };
}
/// Try to unwrap an expression, returning if not possible.
///
/// This is similar to the `?` operator, but is usable in `const` contexts.
macro_rules! const_try_opt {
    ($e:expr) => {
        match $e { Some(value) => value, None => return None, }
    };
}
/// Try to unwrap an expression, panicking if not possible.
///
/// This is similar to `$e.expect($message)`, but is usable in `const` contexts.
macro_rules! expect_opt {
    ($e:expr, $message:literal) => {
        match $e { Some(value) => value, None => crate ::expect_failed($message), }
    };
}
/// `unreachable!()`, but better.
macro_rules! bug {
    () => {
        compile_error!("provide an error message to help fix a possible bug")
    };
    ($descr:literal $($rest:tt)?) => {
        panic!(concat!("internal error: ", $descr) $($rest)?)
    };
}
mod date;
mod date_time;
mod duration;
pub mod error;
pub mod ext;
#[cfg(any(feature = "formatting", feature = "parsing"))]
pub mod format_description;
#[cfg(feature = "formatting")]
pub mod formatting;
#[cfg(feature = "std")]
mod instant;
#[cfg(feature = "macros")]
pub mod macros;
mod month;
mod offset_date_time;
#[cfg(feature = "parsing")]
pub mod parsing;
mod primitive_date_time;
#[cfg(feature = "quickcheck")]
mod quickcheck;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "serde")]
#[allow(missing_copy_implementations, missing_debug_implementations)]
pub mod serde;
mod sys;
#[cfg(test)]
mod tests;
mod time;
mod utc_offset;
pub mod util;
mod weekday;
use time_core::convert;
pub use crate::date::Date;
use crate::date_time::DateTime;
pub use crate::duration::Duration;
pub use crate::error::Error;
#[cfg(feature = "std")]
pub use crate::instant::Instant;
pub use crate::month::Month;
pub use crate::offset_date_time::OffsetDateTime;
pub use crate::primitive_date_time::PrimitiveDateTime;
pub use crate::time::Time;
pub use crate::utc_offset::UtcOffset;
pub use crate::weekday::Weekday;
/// An alias for [`std::result::Result`] with a generic error from the time crate.
pub type Result<T> = core::result::Result<T, Error>;
/// This is a separate function to reduce the code size of `expect_opt!`.
#[inline(never)]
#[cold]
#[track_caller]
const fn expect_failed(message: &str) -> ! {
    panic!("{}", message)
}
#[cfg(test)]
mod tests_llm_16_3 {
    use crate::date::Date;
    use crate::duration::Duration;
    use crate::ext::NumericalDuration;
    use std::ops::AddAssign;
    #[test]
    fn add_assign_duration_to_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u8, i64, i32, u8, i64, i32, u8, i64, i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::January,
                rug_fuzz_1,
            )
            .unwrap();
        let duration = rug_fuzz_2.days();
        date.add_assign(duration);
        debug_assert_eq!(
            date, Date::from_calendar_date(2020, crate ::Month::January, 2).unwrap()
        );
        let mut date = Date::from_calendar_date(
                rug_fuzz_3,
                crate::Month::December,
                rug_fuzz_4,
            )
            .unwrap();
        let duration = rug_fuzz_5.days();
        date.add_assign(duration);
        debug_assert_eq!(
            date, Date::from_calendar_date(2021, crate ::Month::January, 1).unwrap()
        );
        let mut date = Date::from_calendar_date(
                rug_fuzz_6,
                crate::Month::February,
                rug_fuzz_7,
            )
            .unwrap();
        let duration = rug_fuzz_8.days();
        date.add_assign(duration);
        debug_assert_eq!(
            date, Date::from_calendar_date(2020, crate ::Month::February, 29).unwrap()
        );
        let mut date = Date::from_calendar_date(
                rug_fuzz_9,
                crate::Month::February,
                rug_fuzz_10,
            )
            .unwrap();
        let duration = rug_fuzz_11.days();
        date.add_assign(duration);
        debug_assert_eq!(
            date, Date::from_calendar_date(2021, crate ::Month::March, 1).unwrap()
        );
        let mut date = Date::MIN;
        let duration = Duration::MIN;
        date.add_assign(duration);
        debug_assert_eq!(date, Date::MIN);
        let mut date = Date::MAX;
        let duration = Duration::MAX;
        date.add_assign(duration);
        debug_assert_eq!(date, Date::MAX);
        let mut date = Date::MIN;
        let duration = Duration::MAX;
        date.add_assign(duration);
        debug_assert_eq!(
            date, Date::from_calendar_date(2022, crate ::Month::September, 0).unwrap()
        );
        let mut date = Date::MAX;
        let duration = Duration::MIN;
        date.add_assign(duration);
        debug_assert_eq!(
            date, Date::from_calendar_date(- 1958, crate ::Month::May, 11).unwrap()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use crate::{Date, Duration};
    #[test]
    fn sub_assign_positive_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u16, i64, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = Date::from_ordinal_date(rug_fuzz_0, rug_fuzz_1).unwrap();
        let duration = Duration::days(rug_fuzz_2);
        date -= duration;
        debug_assert_eq!(Date::from_ordinal_date(rug_fuzz_3, rug_fuzz_4).unwrap(), date);
             }
}
}
}    }
    #[test]
    fn sub_assign_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u16, i64, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = Date::from_ordinal_date(rug_fuzz_0, rug_fuzz_1).unwrap();
        let duration = Duration::days(-rug_fuzz_2);
        date -= duration;
        debug_assert_eq!(Date::from_ordinal_date(rug_fuzz_3, rug_fuzz_4).unwrap(), date);
             }
}
}
}    }
    #[test]
    fn sub_assign_zero_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u16, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = Date::from_ordinal_date(rug_fuzz_0, rug_fuzz_1).unwrap();
        let duration = Duration::ZERO;
        date -= duration;
        debug_assert_eq!(Date::from_ordinal_date(rug_fuzz_2, rug_fuzz_3).unwrap(), date);
             }
}
}
}    }
    #[test]
    fn sub_assign_to_min_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = Date::MIN;
        let duration = Duration::days(rug_fuzz_0);
        date -= duration;
        debug_assert_eq!(Date::MIN, date);
             }
}
}
}    }
    #[test]
    fn sub_assign_to_max_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = Date::MAX;
        let duration = Duration::days(-rug_fuzz_0);
        date -= duration;
        debug_assert_eq!(Date::MAX, date);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use crate::duration::Duration;
    use std::ops::DivAssign;
    #[test]
    fn div_assign_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= -rug_fuzz_1;
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert!(duration.is_zero());
             }
}
}
}    }
    #[test]
    fn div_assign_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_assign_by_zero_panic() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
             }
}
}
}    }
    #[test]
    fn div_assign_fractional() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::milliseconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, Duration::milliseconds(1000));
             }
}
}
}    }
    #[test]
    fn div_assign_large() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX);
        duration /= rug_fuzz_0;
        debug_assert_eq!(duration, Duration::seconds(i64::MAX / 2));
             }
}
}
}    }
    #[test]
    fn div_assign_small() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::milliseconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, Duration::nanoseconds(1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_48 {
    use super::*;
    use crate::*;
    use std::ops::DivAssign;
    #[test]
    fn test_div_assign_zero_by_nonzero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn test_div_assign_integer_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn test_div_assign_fractional_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds_f64(2.5));
             }
}
}
}    }
    #[test]
    fn test_div_assign_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(-rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_assign_divide_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_div_assign_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0)
            + Duration::nanoseconds(rug_fuzz_1);
        duration.div_assign(rug_fuzz_2);
        debug_assert_eq!(
            duration, Duration::seconds(5) + Duration::nanoseconds(250_000_000)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use crate::Duration;
    use std::ops::DivAssign;
    #[test]
    fn div_assign_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(-rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                duration.div_assign(rug_fuzz_1);
            }),
        );
        debug_assert!(result.is_err());
             }
}
}
}    }
    #[test]
    fn div_assign_to_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn div_assign_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn div_assign_large_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(i16::MAX);
        debug_assert_eq!(duration, Duration::new(0, 305));
             }
}
}
}    }
    #[test]
    fn div_assign_small_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(i16::MIN);
        debug_assert_eq!(duration, Duration::new(0, - 305));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_50 {
    use super::*;
    use crate::*;
    use std::ops::DivAssign;
    #[test]
    fn div_assign_with_positive_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_assign_with_negative_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= -rug_fuzz_1;
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_assign_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let original_duration = duration;
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, original_duration);
             }
}
}
}    }
    #[test]
    fn div_assign_with_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration /= rug_fuzz_1;
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn div_assign_with_max_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX);
        duration /= rug_fuzz_0;
        debug_assert_eq!(duration, Duration::seconds(i64::MAX));
             }
}
}
}    }
    #[test]
    fn div_assign_with_min_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MIN);
        duration /= -rug_fuzz_0;
        debug_assert_eq!(duration, Duration::seconds(i64::MAX));
             }
}
}
}    }
    #[test]
    fn div_assign_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX);
        duration /= rug_fuzz_0;
        debug_assert_eq!(duration, Duration::seconds(i64::MAX / 2));
             }
}
}
}    }
    #[test]
    fn div_assign_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MIN);
        duration /= rug_fuzz_0;
        debug_assert_eq!(duration, Duration::seconds(i64::MIN / 2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_51 {
    use super::*;
    use crate::*;
    use std::ops::DivAssign;
    #[test]
    fn div_assign_with_positive_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_assign_with_negative_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(-rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_assign_with_zero_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                duration.div_assign(rug_fuzz_1);
            }),
        );
        debug_assert!(result.is_err());
             }
}
}
}    }
    #[test]
    fn div_assign_with_one_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn div_assign_with_max_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(i8::MAX);
        debug_assert_eq!(duration, Duration::seconds(10 / i8::MAX as i64));
             }
}
}
}    }
    #[test]
    fn div_assign_with_min_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(i8::MIN);
        debug_assert_eq!(duration, Duration::seconds(10 / i8::MIN as i64));
             }
}
}
}    }
    #[test]
    fn div_assign_with_i8_and_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        duration.div_assign(rug_fuzz_2);
        debug_assert_eq!(duration, Duration::new(0, 1_000_000));
             }
}
}
}    }
    #[test]
    fn div_assign_with_i8_and_negative_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::new(rug_fuzz_0, -rug_fuzz_1);
        duration.div_assign(rug_fuzz_2);
        debug_assert_eq!(duration, Duration::new(0, - 1_000_000));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_52 {
    use super::*;
    use crate::*;
    use std::ops::DivAssign;
    #[test]
    fn div_assign_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn div_assign_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn div_assign_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(-rug_fuzz_0);
        duration.div_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_assign_max_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::MAX;
        duration.div_assign(rug_fuzz_0);
        debug_assert_ne!(duration, Duration::MAX);
             }
}
}
}    }
    #[test]
    fn div_assign_min_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::MIN;
        duration.div_assign(rug_fuzz_0);
        debug_assert_ne!(duration, Duration::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_54 {
    use crate::Duration;
    use std::ops::DivAssign;
    #[test]
    fn div_assign_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let rhs: u8 = rug_fuzz_1;
        let result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                duration /= rhs;
            }),
        );
        debug_assert!(result.is_err());
             }
}
}
}    }
    #[test]
    fn div_assign_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let rhs: u8 = rug_fuzz_1;
        duration /= rhs;
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let rhs: u8 = rug_fuzz_1;
        duration /= rhs;
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn div_assign_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(-rug_fuzz_0);
        let rhs: u8 = rug_fuzz_1;
        duration /= rhs;
        debug_assert_eq!(duration, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_assign_by_max() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        let rhs: u8 = u8::MAX;
        duration /= rhs;
        debug_assert!(duration.is_negative());
             }
}
}
}    }
    #[test]
    fn div_assign_max_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::MAX;
        let rhs: u8 = rug_fuzz_0;
        duration /= rhs;
        debug_assert_eq!(duration, Duration::new(i64::MAX / 2, 499_999_999));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_64 {
    use super::*;
    use crate::*;
    use std::ops::MulAssign;
    #[test]
    fn duration_mul_assign_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn duration_mul_assign_with_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(7) + Duration::milliseconds(500));
             }
}
}
}    }
    #[test]
    fn duration_mul_assign_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(-rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn duration_mul_assign_with_infinity() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(f64::INFINITY);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn duration_mul_assign_with_nan() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(f64::NAN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;
    use crate::*;
    use std::ops::MulAssign;
    #[test]
    fn mul_assign_with_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = duration::Duration::seconds(rug_fuzz_0);
        let multiplier: i16 = rug_fuzz_1;
        duration.mul_assign(multiplier);
        debug_assert_eq!(duration, duration::Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn mul_assign_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = duration::Duration::seconds(rug_fuzz_0);
        let multiplier: i16 = -rug_fuzz_1;
        duration.mul_assign(multiplier);
        debug_assert_eq!(duration, duration::Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn mul_assign_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = duration::Duration::seconds(rug_fuzz_0);
        let multiplier: i16 = rug_fuzz_1;
        duration.mul_assign(multiplier);
        debug_assert_eq!(duration, duration::Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_assign_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = duration::Duration::seconds(i64::MAX);
        let multiplier: i16 = rug_fuzz_0;
        duration.mul_assign(multiplier);
        debug_assert_eq!(duration, duration::Duration::MAX);
             }
}
}
}    }
    #[test]
    fn mul_assign_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = duration::Duration::seconds(i64::MIN);
        let multiplier: i16 = rug_fuzz_0;
        duration.mul_assign(multiplier);
        debug_assert_eq!(duration, duration::Duration::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_66 {
    use super::*;
    use crate::*;
    use std::ops::MulAssign;
    #[test]
    fn mul_assign_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn mul_assign_by_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(-rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn mul_assign_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_assign_large_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(i32::MAX);
        debug_assert_eq!(duration, Duration::seconds(5i64 * i32::MAX as i64));
             }
}
}
}    }
    #[test]
    fn mul_assign_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
        duration.mul_assign(rug_fuzz_2);
        debug_assert!(duration.is_positive());
        debug_assert_eq!(duration.whole_seconds(), i64::MAX);
             }
}
}
}    }
    #[test]
    fn mul_assign_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MIN / rug_fuzz_0 - rug_fuzz_1);
        duration.mul_assign(rug_fuzz_2);
        debug_assert!(duration.is_negative());
        debug_assert_eq!(duration.whole_seconds(), i64::MIN);
             }
}
}
}    }
    #[test]
    fn mul_assign_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        duration.mul_assign(rug_fuzz_2);
        debug_assert_eq!(duration, Duration::new(3, 0));
             }
}
}
}    }
    #[test]
    fn mul_assign_with_negative_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::new(-rug_fuzz_0, -rug_fuzz_1);
        duration.mul_assign(rug_fuzz_2);
        debug_assert_eq!(duration, Duration::new(- 3, 0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use crate::Duration;
    use std::ops::MulAssign;
    #[test]
    fn mul_assign_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn mul_assign_by_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(-rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn mul_assign_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_assign_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn mul_assign_to_max() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX / rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(i64::MAX));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when multiplying duration")]
    fn mul_assign_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX);
        duration.mul_assign(rug_fuzz_0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;
    use crate::*;
    use std::ops::MulAssign;
    #[test]
    fn mul_assign_zero_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::ZERO;
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(duration, Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn mul_assign_second_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::SECOND;
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(duration, Duration::SECOND);
             }
}
}
}    }
    #[test]
    fn mul_assign_second_by_two() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::SECOND;
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(duration, Duration::seconds(2));
             }
}
}
}    }
    #[test]
    fn mul_assign_max_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::MAX;
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(duration, Duration::MAX);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when adding durations")]
    fn mul_assign_max_by_two() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::MAX;
        duration.mul_assign(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    fn mul_assign_negative_by_two() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(-rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(duration, Duration::seconds(- 2));
             }
}
}
}    }
    #[test]
    fn mul_assign_millisecond_by_thousand() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::MILLISECOND;
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(duration, Duration::SECOND);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when adding durations")]
    fn mul_assign_second_by_max() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_mul_assign_second_by_max = 0;
        let mut duration = Duration::SECOND;
        duration.mul_assign(u32::MAX);
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_mul_assign_second_by_max = 0;
    }
    #[test]
    fn mul_assign_subsecond_by_ten() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        duration.mul_assign(rug_fuzz_2);
        debug_assert_eq!(duration, Duration::milliseconds(10));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_70 {
    use crate::Duration;
    use std::ops::MulAssign;
    #[test]
    fn mul_assign_by_zero_should_yield_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(Duration::seconds(rug_fuzz_2), duration);
             }
}
}
}    }
    #[test]
    fn mul_assign_by_one_should_yield_same_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(Duration::seconds(rug_fuzz_2), duration);
             }
}
}
}    }
    #[test]
    fn mul_assign_by_two_should_double_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(rug_fuzz_1);
        debug_assert_eq!(Duration::seconds(rug_fuzz_2), duration);
             }
}
}
}    }
    #[test]
    fn mul_assign_by_negative_one_should_negate_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(rug_fuzz_0);
        duration.mul_assign(-rug_fuzz_1 as u8);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_2), duration);
             }
}
}
}    }
    #[test]
    fn mul_assign_by_255_should_result_in_overflow_for_positive_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MAX);
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(Duration::seconds(i64::MIN), duration);
             }
}
}
}    }
    #[test]
    fn mul_assign_by_255_should_result_in_overflow_for_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration = Duration::seconds(i64::MIN);
        duration.mul_assign(rug_fuzz_0);
        debug_assert_eq!(Duration::seconds(i64::MIN), duration);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_75 {
    use crate::Duration;
    use std::ops::SubAssign;
    #[test]
    fn sub_assign_positive_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::seconds(rug_fuzz_0);
        let duration2 = Duration::seconds(rug_fuzz_1);
        duration1.sub_assign(duration2);
        debug_assert_eq!(duration1, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn sub_assign_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::seconds(rug_fuzz_0);
        let duration2 = Duration::seconds(rug_fuzz_1);
        duration1.sub_assign(duration2);
        debug_assert_eq!(duration1, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn sub_assign_mixed_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::seconds(-rug_fuzz_0);
        let duration2 = Duration::seconds(rug_fuzz_1);
        duration1.sub_assign(duration2);
        debug_assert_eq!(duration1, Duration::seconds(- 8));
             }
}
}
}    }
    #[test]
    fn sub_assign_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let duration2 = Duration::new(rug_fuzz_2, rug_fuzz_3);
        duration1.sub_assign(duration2);
        debug_assert_eq!(duration1, Duration::new(0, 750_000_000));
             }
}
}
}    }
    #[test]
    fn sub_assign_to_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::seconds(rug_fuzz_0);
        let duration2 = Duration::seconds(rug_fuzz_1);
        duration1.sub_assign(duration2);
        debug_assert!(duration1.is_zero());
             }
}
}
}    }
    #[test]
    fn sub_assign_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::new(i64::MIN, rug_fuzz_0);
        let duration2 = Duration::seconds(rug_fuzz_1);
        duration1.sub_assign(duration2);
        debug_assert_eq!(duration1, Duration::new(i64::MIN, 0));
             }
}
}
}    }
    #[test]
    fn sub_assign_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut duration1 = Duration::new(i64::MAX, rug_fuzz_0);
        let duration2 = Duration::seconds(-rug_fuzz_1);
        duration1.sub_assign(duration2);
        debug_assert_eq!(duration1, Duration::new(i64::MAX, 999_999_999));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_332 {
    use crate::expect_failed;
    #[test]
    #[should_panic(expected = "Expected panic message")]
    fn test_expect_failed_with_specific_message() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        expect_failed(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_expect_failed_with_any_message() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        expect_failed(rug_fuzz_0);
             }
}
}
}    }
}
