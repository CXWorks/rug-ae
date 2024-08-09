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
//!   If you are using this feature, **please leave a comment**
//!   [on this discussion](https://github.com/time-rs/time/discussions/306) with your use case. If
//!   there is not sufficient demand for this feature, it will be dropped in a future release.
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
//! - `serde-well-known` (_implicitly enables `serde/alloc`, `formatting`, and `parsing`_)
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
//! One pseudo-feature flag that is only available to end users is the `unsound_local_offset` cfg.
//! As the name indicates, using the feature is unsound, and [may cause unexpected segmentation
//! faults](https://github.com/time-rs/time/issues/293). Unlike other flags, this is deliberately
//! only available to end users; this is to ensure that a user doesn't have unsound behavior without
//! knowing it. To enable this behavior, you must use `RUSTFLAGS="--cfg unsound_local_offset" cargo
//! build` or similar. Note: This flag is _not tested anywhere_, including in the regular test of
//! the powerset of all feature flags. Use at your own risk. Without this flag, any method that
//! requires the local offset will return the `Err` variant.
#![feature(no_coverage)]

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![cfg_attr(__time_03_docs, feature(doc_cfg, doc_auto_cfg, doc_notable_trait))]
#![cfg_attr(
    __time_03_docs,
    deny(rustdoc::broken_intra_doc_links, rustdoc::private_intra_doc_links)
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
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
#![allow(clippy::redundant_pub_crate)]
#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(test(attr(deny(warnings))))]

#[allow(unused_extern_crates)]
#[cfg(feature = "alloc")]
extern crate alloc;
pub mod rusty_monitor;
use ntest::timeout;
// region: macros
/// Helper macro for easily implementing `OpAssign`.
macro_rules! __impl_assign {
    ($sym:tt $op:ident $fn:ident $target:ty : $($(#[$attr:meta])* $t:ty),+) => {$(
        #[allow(unused_qualifications)]
        $(#[$attr])*
        impl core::ops::$op<$t> for $target {
            fn $fn(&mut self, rhs: $t) {
                *self = *self $sym rhs;
            }
        }
    )+};
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
    ($a:expr, $b:expr) => {{
        let _a = $a;
        let _b = $b;

        let (_quotient, _remainder) = (_a / _b, _a % _b);

        if (_remainder > 0 && _b < 0) || (_remainder < 0 && _b > 0) {
            _quotient - 1
        } else {
            _quotient
        }
    }};
}

/// Cascade an out-of-bounds value.
macro_rules! cascade {
    (@ordinal ordinal) => {};
    (@year year) => {};

    // Cascade an out-of-bounds value from "from" to "to".
    ($from:ident in $min:literal.. $max:literal => $to:tt) => {
        #[allow(unused_comparisons, unused_assignments)]
        if $from >= $max {
            $from -= $max - $min;
            $to += 1;
        } else if $from < $min {
            $from += $max - $min;
            $to -= 1;
        }
    };

    // Special case the ordinal-to-year cascade, as it has different behavior.
    ($ordinal:ident => $year:ident) => {
        // We need to actually capture the idents. Without this, macro hygiene causes errors.
        cascade!(@ordinal $ordinal);
        cascade!(@year $year);
        #[allow(unused_assignments)]
        if $ordinal > crate::util::days_in_year($year) {
            $year += 1;
            $ordinal = 1;
        } else if $ordinal == 0 {
            $year -= 1;
            $ordinal = crate::util::days_in_year($year);
        }
    };
}

/// Returns `Err(error::ComponentRange)` if the value is not in range.
macro_rules! ensure_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {{
        let _start = $start;
        let _end = $end;
        #[allow(trivial_numeric_casts, unused_comparisons)]
        if $value < _start || $value > _end {
            return Err(crate::error::ComponentRange {
                name: stringify!($value),
                minimum: _start as _,
                maximum: _end as _,
                value: $value as _,
                conditional_range: false,
            });
        }
    }};

    ($value:ident conditionally in $start:expr => $end:expr) => {{
        let _start = $start;
        let _end = $end;
        #[allow(trivial_numeric_casts, unused_comparisons)]
        if $value < _start || $value > _end {
            return Err(crate::error::ComponentRange {
                name: stringify!($value),
                minimum: _start as _,
                maximum: _end as _,
                value: $value as _,
                conditional_range: true,
            });
        }
    }};
}

/// Try to unwrap an expression, returning if not possible.
///
/// This is similar to the `?` operator, but does not perform `.into()`. Because of this, it is
/// usable in `const` contexts.
macro_rules! const_try {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(error) => return Err(error),
        }
    };
}

/// Try to unwrap an expression, returning if not possible.
///
/// This is similar to the `?` operator, but is usable in `const` contexts.
macro_rules! const_try_opt {
    ($e:expr) => {
        match $e {
            Some(value) => value,
            None => return None,
        }
    };
}
// endregion macros

pub mod date;
pub mod duration;
pub mod error;
pub mod ext;
#[cfg(any(feature = "formatting", feature = "parsing"))]
pub mod format_description;
#[cfg(feature = "formatting")]
pub mod formatting;
#[cfg(feature = "std")]
pub mod instant;
#[cfg(feature = "macros")]
pub mod macros;
pub mod month;
pub mod offset_date_time;
#[cfg(feature = "parsing")]
pub mod parsing;
pub mod primitive_date_time;
#[cfg(feature = "quickcheck")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
pub mod quickcheck;
#[cfg(feature = "rand")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
pub mod rand;
#[cfg(feature = "serde")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "serde")))]
#[allow(missing_copy_implementations, missing_debug_implementations)]
pub mod serde;
pub mod sys;
//#[cfg(test)]
//pub mod tests;
pub mod time;
pub mod utc_offset;
pub mod util;
pub mod weekday;

pub use crate::date::Date;
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

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::ops::SubAssign;
	use std::ops::MulAssign;
	use std::ops::AddAssign;
	use std::ops::DivAssign;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3920() {
    rusty_monitor::set_test_id(3920);
    let mut i32_0: i32 = -168i32;
    let mut i64_0: i64 = 69i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = -94i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 44u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 29i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_2: i32 = 108i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i128_0: i128 = -77i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 92u16;
    let mut i32_3: i32 = 14i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut i32_4: i32 = -37i32;
    let mut i64_2: i64 = 129i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, i32_4);
    let mut date_4: crate::date::Date = crate::primitive_date_time::PrimitiveDateTime::date(primitivedatetime_2);
    std::ops::SubAssign::sub_assign(primitivedatetime_1_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1570() {
    rusty_monitor::set_test_id(1570);
    let mut i32_0: i32 = 95i32;
    let mut i64_0: i64 = 89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut f32_0: f32 = 74.213342f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i32_1: i32 = -161i32;
    let mut i64_1: i64 = -128i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_2: i32 = -119i32;
    let mut i64_2: i64 = 259i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = -97i32;
    let mut i64_3: i64 = -81i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_6);
    std::ops::AddAssign::add_assign(duration_2_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3745() {
    rusty_monitor::set_test_id(3745);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 56u8;
    let mut u8_2: u8 = 81u8;
    let mut i32_0: i32 = 45i32;
    let mut i32_1: i32 = 27i32;
    let mut i64_0: i64 = 42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -192i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut f64_0: f64 = -100.556972f64;
    let mut f32_0: f32 = 29.380039f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, f64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, i32_0);
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2242() {
    rusty_monitor::set_test_id(2242);
    let mut i128_0: i128 = -67i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i128_1: i128 = -72i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut i32_0: i32 = 137i32;
    let mut i64_0: i64 = 122i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_1: i64 = -71i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut f32_0: f32 = -72.689593f32;
    let mut i64_2: i64 = 125i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut f32_1: f32 = -112.393008f32;
    let mut i64_3: i64 = 136i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::MulAssign::mul_assign(duration_8_ref_0, f32_1);
    std::ops::DivAssign::div_assign(duration_7_ref_0, f32_0);
    std::ops::SubAssign::sub_assign(duration_6_ref_0, duration_5);
    let mut i64_4: i64 = crate::duration::Duration::whole_minutes(duration_3);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    std::ops::AddAssign::add_assign(instant_0_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_990() {
    rusty_monitor::set_test_id(990);
    let mut i8_0: i8 = 49i8;
    let mut i8_1: i8 = 54i8;
    let mut i8_2: i8 = 10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 64i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i8_3: i8 = -14i8;
    let mut i8_4: i8 = -33i8;
    let mut i8_5: i8 = -60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut u32_0: u32 = 11u32;
    let mut i64_0: i64 = 96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 58i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = -153i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u32_1: u32 = 14u32;
    let mut u8_0: u8 = 21u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 40u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    std::ops::SubAssign::sub_assign(time_1_ref_0, duration_3);
    std::ops::DivAssign::div_assign(duration_2_ref_0, u32_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3653() {
    rusty_monitor::set_test_id(3653);
    let mut i64_0: i64 = 45i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 94i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u16_0: u16 = 6u16;
    let mut i32_0: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i32_1: i32 = -11i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut u8_0: u8 = 58u8;
    let mut i64_2: i64 = -53i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u8_0);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3104() {
    rusty_monitor::set_test_id(3104);
    let mut u32_0: u32 = 77u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 76u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 8i32;
    let mut i64_1: i64 = 94i64;
    let mut u32_1: u32 = 26u32;
    let mut f64_0: f64 = -24.640160f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, u32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_2: i64 = crate::duration::Duration::whole_weeks(duration_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut u16_0: u16 = crate::time::Time::millisecond(time_0);
    let mut month_1: month::Month = crate::month::Month::August;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2519() {
    rusty_monitor::set_test_id(2519);
    let mut i64_0: i64 = -12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 55u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_1: u32 = 31u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 91u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    let mut i128_0: i128 = -68i128;
    let mut f32_0: f32 = -78.166791f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_2: u32 = 64u32;
    let mut i64_2: i64 = 122i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i16_0: i16 = 48i16;
    let mut i64_3: i64 = 246i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut f64_0: f64 = -52.408008f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_0: i32 = -9i32;
    let mut i64_4: i64 = 79i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_0);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    std::ops::SubAssign::sub_assign(duration_10_ref_0, duration_9);
    std::ops::MulAssign::mul_assign(duration_7_ref_0, i16_0);
    std::ops::DivAssign::div_assign(duration_6_ref_0, u32_2);
    let mut i64_5: i64 = crate::duration::Duration::whole_seconds(duration_5);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::AddAssign::add_assign(time_1_ref_0, duration_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3109() {
    rusty_monitor::set_test_id(3109);
    let mut i32_0: i32 = -89i32;
    let mut i64_0: i64 = -95i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 51i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i16_0: i16 = 28i16;
    let mut i32_1: i32 = 153i32;
    let mut i64_2: i64 = 155i64;
    let mut u16_0: u16 = 18u16;
    let mut i32_2: i32 = 45i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i8_0: i8 = -114i8;
    let mut i8_1: i8 = 27i8;
    let mut i8_2: i8 = -26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i16_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    std::ops::AddAssign::add_assign(date_1_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4663() {
    rusty_monitor::set_test_id(4663);
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 52u16;
    let mut i32_0: i32 = 158i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_1: i32 = -105i32;
    let mut i64_0: i64 = -89i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    std::ops::AddAssign::add_assign(instant_4_ref_0, duration_1);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4061() {
    rusty_monitor::set_test_id(4061);
    let mut i8_0: i8 = -84i8;
    let mut i8_1: i8 = 77i8;
    let mut i8_2: i8 = -16i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 85i8;
    let mut i8_4: i8 = 72i8;
    let mut i8_5: i8 = -28i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 83i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 34u16;
    let mut i32_0: i32 = 135i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_1};
    let mut i64_1: i64 = -64i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_2: i64 = 21i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_2);
    let mut i64_3: i64 = -84i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut i64_4: i64 = -183i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u8_0: u8 = 21u8;
    let mut i64_5: i64 = 31i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i32_1: i32 = -115i32;
    let mut i64_6: i64 = -89i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    std::ops::MulAssign::mul_assign(duration_7_ref_0, u8_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_1, duration_4);
    std::ops::SubAssign::sub_assign(instant_0_ref_0, duration_3);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_2);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4496() {
    rusty_monitor::set_test_id(4496);
    let mut i64_0: i64 = -46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 61i8;
    let mut i8_2: i8 = -110i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f64_0: f64 = -67.894670f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut i32_0: i32 = -21i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u16_0: u16 = 23u16;
    let mut i32_1: i32 = 17i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i64_1: i64 = 19i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = -92i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut u32_0: u32 = 58u32;
    let mut i32_2: i32 = 261i32;
    let mut i64_3: i64 = -155i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    std::ops::MulAssign::mul_assign(duration_7_ref_0, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_6, duration_4);
    let mut duration_8: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1517() {
    rusty_monitor::set_test_id(1517);
    let mut i32_0: i32 = -26i32;
    let mut f32_0: f32 = -53.261669f32;
    let mut i64_0: i64 = -81i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u16_0: u16 = 70u16;
    let mut i32_1: i32 = -175i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut f32_1: f32 = 19.088103f32;
    let mut i16_0: i16 = 80i16;
    let mut f64_0: f64 = -105.724374f64;
    let mut f32_2: f32 = -178.597493f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, u16_0);
    std::ops::DivAssign::div_assign(duration_0_ref_0, f32_0);
    let mut u8_0: u8 = crate::util::weeks_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2144() {
    rusty_monitor::set_test_id(2144);
    let mut i64_0: i64 = 49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -59i64;
    let mut f32_0: f32 = 218.827556f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
    let mut i8_0: i8 = -33i8;
    let mut i32_0: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i16_0: i16 = -1i16;
    let mut u8_0: u8 = 32u8;
    let mut i64_2: i64 = -25i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u8_1: u8 = crate::date::Date::sunday_based_week(date_0);
    std::ops::SubAssign::sub_assign(instant_1_ref_0, duration_4);
    let mut i64_3: i64 = crate::duration::Duration::whole_hours(duration_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::SubAssign::sub_assign(duration_6_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4223() {
    rusty_monitor::set_test_id(4223);
    let mut i64_0: i64 = 151i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -112i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i128_0: i128 = -109i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = -11i8;
    let mut i8_1: i8 = 83i8;
    let mut i8_2: i8 = 73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -53i32;
    let mut i128_1: i128 = -257i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_0);
    let mut u32_0: u32 = 17u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_1: i32 = -114i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    std::ops::AddAssign::add_assign(offsetdatetime_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1025() {
    rusty_monitor::set_test_id(1025);
    let mut i16_0: i16 = -48i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 114i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -110i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_0: i32 = -178i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = -109i8;
    let mut i8_1: i8 = 49i8;
    let mut i8_2: i8 = -108i8;
    let mut i8_3: i8 = -41i8;
    let mut i8_4: i8 = -127i8;
    let mut i8_5: i8 = -52i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_0, duration_2);
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2323() {
    rusty_monitor::set_test_id(2323);
    let mut i32_0: i32 = -22i32;
    let mut i64_0: i64 = 54i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 97i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_0: u8 = 39u8;
    let mut i32_2: i32 = -137i32;
    let mut i32_3: i32 = -184i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_0: u32 = 35u32;
    let mut i64_1: i64 = 120i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u32_0);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_0, weekday_0);
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::nanosecond(offsetdatetime_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3959() {
    rusty_monitor::set_test_id(3959);
    let mut f64_0: f64 = 71.939085f64;
    let mut u32_0: u32 = 42u32;
    let mut u8_0: u8 = 44u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 4i128;
    let mut i128_1: i128 = 179i128;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 93u16;
    let mut i32_0: i32 = -116i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_1);
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_255() {
    rusty_monitor::set_test_id(255);
    let mut i32_0: i32 = 99i32;
    let mut i64_0: i64 = 178i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 178i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u16_0: u16 = 57u16;
    let mut i32_1: i32 = -21i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut i32_2: i32 = -31i32;
    let mut u8_0: u8 = 97u8;
    let mut f32_0: f32 = 16.270706f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i32_3: i32 = -111i32;
    let mut i32_4: i32 = -50i32;
    let mut i64_2: i64 = 207i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_5: i32 = -8i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    std::ops::DivAssign::div_assign(duration_2_ref_0, u8_0);
    let mut u16_1: u16 = crate::util::days_in_year(i32_2);
    let mut u8_1: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3562() {
    rusty_monitor::set_test_id(3562);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 65u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 153i32;
    let mut i32_1: i32 = 149i32;
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i16_0: i16 = -146i16;
    let mut i32_2: i32 = -33i32;
    let mut i64_1: i64 = -89i64;
    let mut i64_2: i64 = 102i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f32_0: f32 = 7.862801f32;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, i16_0);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_1);
    let mut u8_3: u8 = crate::time::Time::minute(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1076() {
    rusty_monitor::set_test_id(1076);
    let mut i32_0: i32 = -8i32;
    let mut i64_0: i64 = 68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = -107i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i32_2: i32 = -6i32;
    let mut i64_1: i64 = 65i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut u16_0: u16 = 45u16;
    let mut i32_3: i32 = -46i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i64_2: i64 = -108i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut u8_0: u8 = 3u8;
    let mut i64_3: i64 = 63i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut f64_0: f64 = -2.535705f64;
    let mut i64_4: i64 = 22i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::DivAssign::div_assign(duration_6_ref_0, f64_0);
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u8_0);
    std::ops::AddAssign::add_assign(offsetdatetime_2_ref_0, duration_3);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_306() {
    rusty_monitor::set_test_id(306);
    let mut i8_0: i8 = 22i8;
    let mut i8_1: i8 = -99i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 2i32;
    let mut i64_0: i64 = 18i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i128_0: i128 = 126i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = 76i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i8_3: i8 = 7i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 114i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_1: i32 = -57i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = -17i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_1);
    let mut i128_1: i128 = crate::duration::Duration::whole_milliseconds(duration_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_802() {
    rusty_monitor::set_test_id(802);
    let mut u8_0: u8 = 93u8;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_0: i32 = -33i32;
    let mut i32_1: i32 = -11i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_2: i32 = -44i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -114i32;
    let mut i64_1: i64 = 160i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, i32_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3558() {
    rusty_monitor::set_test_id(3558);
    let mut i64_0: i64 = -46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_0: i8 = -103i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 87i8;
    let mut i8_4: i8 = -6i8;
    let mut i8_5: i8 = -27i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 64i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_0: u32 = 1u32;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 89u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 29u16;
    let mut i32_0: i32 = -33i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i32_1: i32 = -157i32;
    let mut f32_0: f32 = -13.630986f32;
    let mut i32_2: i32 = -72i32;
    let mut i64_2: i64 = -33i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut i32_3: i32 = 111i32;
    let mut i64_3: i64 = 137i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i64_4: i64 = 123i64;
    let mut i32_4: i32 = 40i32;
    let mut i8_6: i8 = -65i8;
    let mut i8_7: i8 = 68i8;
    let mut i8_8: i8 = -90i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i32_5: i32 = crate::duration::Duration::subsec_microseconds(duration_4);
    std::ops::SubAssign::sub_assign(offsetdatetime_1_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4084() {
    rusty_monitor::set_test_id(4084);
    let mut u16_0: u16 = 39u16;
    let mut i32_0: i32 = 84i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut f32_0: f32 = -39.091047f32;
    let mut i32_1: i32 = -177i32;
    let mut i64_0: i64 = 47i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u16_1: u16 = 41u16;
    let mut i32_2: i32 = 22i32;
    let mut i64_1: i64 = 96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f64_0: f64 = 21.720380f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_3: i32 = -42i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_2: i64 = 181i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u16_2: u16 = 40u16;
    let mut i32_4: i32 = 80i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut date_3_ref_0: &mut crate::date::Date = &mut date_3;
    let mut i64_3: i64 = -98i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = -113i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::SubAssign::sub_assign(duration_6_ref_0, duration_5);
    std::ops::SubAssign::sub_assign(date_3_ref_0, duration_4);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f32_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1681() {
    rusty_monitor::set_test_id(1681);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 99u8;
    let mut i32_0: i32 = 68i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = -17i32;
    let mut i64_0: i64 = -77i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut u32_1: u32 = 88u32;
    let mut u8_3: u8 = 36u8;
    let mut u8_4: u8 = 19u8;
    let mut u8_5: u8 = 8u8;
    let mut i32_2: i32 = 219i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i16_0: i16 = 1i16;
    let mut u32_2: u32 = 17u32;
    let mut i32_3: i32 = -47i32;
    let mut i64_1: i64 = 26i64;
    let mut i64_2: i64 = -63i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut f64_0: f64 = 85.190406f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_3: i64 = 170i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_4: i32 = 8i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_2);
    let mut date_5_ref_0: &mut crate::date::Date = &mut date_5;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_6: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, i16_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_902() {
    rusty_monitor::set_test_id(902);
    let mut i64_0: i64 = -10i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = 271i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i16_0: i16 = -67i16;
    let mut i64_2: i64 = 74i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_0: i32 = 109i32;
    let mut i64_3: i64 = 43i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut u16_0: u16 = 33u16;
    let mut i32_1: i32 = -60i32;
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 54u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    std::ops::DivAssign::div_assign(duration_6_ref_0, i16_0);
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4461() {
    rusty_monitor::set_test_id(4461);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 293i32;
    let mut i64_0: i64 = 8i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 21u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 32u32;
    let mut u8_3: u8 = 49u8;
    let mut u8_4: u8 = 92u8;
    let mut u8_5: u8 = 33u8;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = crate::duration::Duration::subsec_microseconds(duration_2);
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
    std::ops::AddAssign::add_assign(instant_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_858() {
    rusty_monitor::set_test_id(858);
    let mut u16_0: u16 = 83u16;
    let mut i32_0: i32 = 117i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u8_0: u8 = 96u8;
    let mut i32_1: i32 = 20i32;
    let mut f32_0: f32 = 179.294201f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut f64_0: f64 = 44.367442f64;
    let mut f32_1: f32 = 53.505780f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_2: i32 = -64i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u16_1: u16 = 99u16;
    let mut i32_3: i32 = -75i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u8_0);
    let mut u8_1: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2104() {
    rusty_monitor::set_test_id(2104);
    let mut i64_0: i64 = -51i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 35u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut u16_0: u16 = 30u16;
    let mut i64_1: i64 = -4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f64_0: f64 = 177.845318f64;
    let mut i32_0: i32 = 107i32;
    let mut i64_2: i64 = 25i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, f64_0);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, u16_0);
    std::ops::AddAssign::add_assign(time_0_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3993() {
    rusty_monitor::set_test_id(3993);
    let mut i64_0: i64 = 153i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = -105i32;
    let mut i64_2: i64 = -43i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u16_0: u16 = 64u16;
    let mut i32_1: i32 = -24i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    std::ops::SubAssign::sub_assign(offsetdatetime_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4019() {
    rusty_monitor::set_test_id(4019);
    let mut i64_0: i64 = -112i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 28u32;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut i64_2: i64 = 14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_0: i32 = 5i32;
    let mut i64_3: i64 = 47i64;
    let mut i32_1: i32 = 47i32;
    let mut i64_4: i64 = -73i64;
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_3);
    std::ops::SubAssign::sub_assign(time_0_ref_0, duration_2);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_576() {
    rusty_monitor::set_test_id(576);
    let mut i32_0: i32 = -27i32;
    let mut f32_0: f32 = 40.000643f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 34i64;
    let mut f32_1: f32 = 40.275679f32;
    let mut i64_1: i64 = 102i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 64u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 34u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 44u16;
    let mut i32_1: i32 = -70i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_0_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut u8_3: u8 = 0u8;
    let mut i8_0: i8 = -30i8;
    let mut i64_2: i64 = -124i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    std::ops::SubAssign::sub_assign(primitivedatetime_0_ref_0, duration_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3554() {
    rusty_monitor::set_test_id(3554);
    let mut i8_0: i8 = -4i8;
    let mut i64_0: i64 = 192i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_0: i32 = -107i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i64_1: i64 = -26i64;
    let mut i8_1: i8 = -93i8;
    let mut i8_2: i8 = -33i8;
    let mut i8_3: i8 = -6i8;
    let mut i8_4: i8 = 104i8;
    let mut i8_5: i8 = -62i8;
    let mut i8_6: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_2: i64 = -10i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_7: i8 = 24i8;
    let mut i8_8: i8 = -30i8;
    let mut i8_9: i8 = 79i8;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2669() {
    rusty_monitor::set_test_id(2669);
    let mut f32_0: f32 = 161.909215f32;
    let mut i128_0: i128 = -72i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = -66i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i128_1: i128 = -242i128;
    let mut f32_1: f32 = 49.455844f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_0: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
    std::ops::AddAssign::add_assign(date_0_ref_0, duration_4);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_1);
    std::ops::DivAssign::div_assign(duration_2_ref_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4935() {
    rusty_monitor::set_test_id(4935);
    let mut i64_0: i64 = 83i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 93i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = 33i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut u32_0: u32 = 52u32;
    let mut u8_0: u8 = 14u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -109i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut i8_0: i8 = 17i8;
    let mut i32_1: i32 = -8i32;
    let mut i64_3: i64 = -102i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::DivAssign::div_assign(duration_5_ref_0, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    std::ops::SubAssign::sub_assign(duration_2_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4472() {
    rusty_monitor::set_test_id(4472);
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 29u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 49u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 17i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_0: i32 = -172i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = -154i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 206i32;
    let mut i128_0: i128 = 15i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_3: i32 = 82i32;
    let mut i64_2: i64 = 142i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut i64_3: i64 = -78i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut i32_4: i32 = 93i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_5);
    let mut date_4_ref_0: &mut crate::date::Date = &mut date_4;
    std::ops::SubAssign::sub_assign(date_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_2);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut duration_6: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3872() {
    rusty_monitor::set_test_id(3872);
    let mut u32_0: u32 = 1u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 70u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i64_0: i64 = -3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -45i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i64_2: i64 = -97i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_1: i32 = 61i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_3: i64 = -173i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_2: i32 = 245i32;
    let mut i64_4: i64 = -35i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut u32_1: u32 = 42u32;
    let mut u8_3: u8 = 73u8;
    let mut u8_4: u8 = 48u8;
    let mut u8_5: u8 = 95u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = 189i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_7);
    let mut primitivedatetime_2_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut u8_6: u8 = 4u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    std::ops::MulAssign::mul_assign(duration_8_ref_0, u8_6);
    std::ops::AddAssign::add_assign(primitivedatetime_2_ref_0, duration_5);
    let mut u8_7: u8 = crate::date::Date::iso_week(date_1);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut u8_8: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2052() {
    rusty_monitor::set_test_id(2052);
    let mut f32_0: f32 = 179.757508f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 77u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 59u16;
    let mut i32_0: i32 = -31i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i64_0: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = -79i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = 93i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2_ref_0: &mut crate::date::Date = &mut date_2;
    std::ops::SubAssign::sub_assign(date_2_ref_0, duration_5);
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1656() {
    rusty_monitor::set_test_id(1656);
    let mut i8_0: i8 = 17i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 24i32;
    let mut i64_0: i64 = -105i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 135i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = 64i8;
    let mut i8_4: i8 = 85i8;
    let mut i8_5: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 80i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_0: u16 = 35u16;
    let mut i32_2: i32 = 83i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_1};
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_3: i32 = 32i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_2: i64 = -5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 57i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut u32_1: u32 = 34u32;
    let mut u8_3: u8 = 64u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = 3i8;
    let mut i8_7: i8 = 50i8;
    let mut i8_8: i8 = 63i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut i32_4: i32 = -22i32;
    let mut u32_2: u32 = 99u32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    std::ops::DivAssign::div_assign(duration_6_ref_0, u32_2);
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_4, duration_5);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::next_day(date_2);
    let mut u32_3: u32 = crate::time::Time::microsecond(time_1);
    let mut i32_5: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    panic!("From RustyUnit with love");
}
}