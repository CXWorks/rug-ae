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
	use std::ops::MulAssign;
	use std::ops::AddAssign;
	use std::ops::DivAssign;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4524() {
    rusty_monitor::set_test_id(4524);
    let mut u8_0: u8 = 24u8;
    let mut i64_0: i64 = -24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut f32_0: f32 = 110.433329f32;
    let mut i128_0: i128 = 34i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_1: f32 = -93.280033f32;
    let mut i16_0: i16 = -73i16;
    let mut i64_1: i64 = -40i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut f64_0: f64 = -78.848019f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::February;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i8_0: i8 = 38i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = 39i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = 10i8;
    let mut i8_5: i8 = 61i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 78u16;
    let mut i64_2: i64 = -27i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = 66i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 75u8;
    let mut u8_3: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u16_1: u16 = 67u16;
    let mut i32_1: i32 = -61i32;
    let mut i64_3: i64 = -43i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = -96i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i32_2: i32 = crate::duration::Duration::subsec_microseconds(duration_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    std::ops::DivAssign::div_assign(duration_0_ref_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_187() {
    rusty_monitor::set_test_id(187);
    let mut i8_0: i8 = -15i8;
    let mut i8_1: i8 = -36i8;
    let mut i8_2: i8 = -96i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -148i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 27u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 79u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 117.129169f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u16_0: u16 = 13u16;
    let mut i32_1: i32 = -113i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut f64_0: f64 = 142.872874f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    let mut i8_3: i8 = -123i8;
    let mut i8_4: i8 = -46i8;
    let mut i8_5: i8 = -11i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 99i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = 11i32;
    let mut i64_2: i64 = 13i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut u32_1: u32 = 40u32;
    let mut u8_3: u8 = 91u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 28u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = 70i8;
    let mut i8_7: i8 = -62i8;
    let mut i8_8: i8 = -70i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_2);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_2};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_3, duration_8);
    let mut u16_1: u16 = 88u16;
    let mut u16_2: u16 = 0u16;
    let mut u8_6: u8 = 48u8;
    let mut u8_7: u8 = 39u8;
    let mut u8_8: u8 = 46u8;
    let mut i64_3: i64 = 155i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_8, u8_7, u8_6, u16_2);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    std::ops::DivAssign::div_assign(duration_9_ref_0, u16_1);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_4, utcoffset_1);
    std::ops::AddAssign::add_assign(time_1_ref_0, duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_629() {
    rusty_monitor::set_test_id(629);
    let mut i8_0: i8 = -123i8;
    let mut i8_1: i8 = -46i8;
    let mut i8_2: i8 = -11i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 99i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = 11i32;
    let mut i64_1: i64 = 13i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut u32_0: u32 = 40u32;
    let mut u8_0: u8 = 91u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 70i8;
    let mut i8_4: i8 = -62i8;
    let mut i8_5: i8 = -70i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut u16_0: u16 = 88u16;
    let mut u16_1: u16 = 0u16;
    let mut u8_3: u8 = 48u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 46u8;
    let mut i64_2: i64 = 155i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_5, u8_4, u8_3, u16_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_146() {
    rusty_monitor::set_test_id(146);
    let mut i64_0: i64 = -60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 14i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_0: i8 = 80i8;
    let mut i8_1: i8 = 27i8;
    let mut i8_2: i8 = -8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    let mut i8_3: i8 = 113i8;
    let mut i8_4: i8 = -76i8;
    let mut i8_5: i8 = -25i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -80i8;
    let mut i8_7: i8 = -55i8;
    let mut i8_8: i8 = -19i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = -2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 46i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_0: i32 = 4i32;
    let mut i64_4: i64 = -75i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i8_9: i8 = -112i8;
    let mut i8_10: i8 = 71i8;
    let mut i8_11: i8 = 52i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 36u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 37u8;
    let mut u8_4: u8 = 43u8;
    let mut u8_5: u8 = 18u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_12: i8 = -111i8;
    let mut i8_13: i8 = -67i8;
    let mut i8_14: i8 = 87i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 77i8;
    let mut i8_16: i8 = 119i8;
    let mut i8_17: i8 = 30i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i128_0: i128 = 198i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_18: i8 = -52i8;
    let mut i8_19: i8 = -63i8;
    let mut i8_20: i8 = -48i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 8u32;
    let mut u8_6: u8 = 48u8;
    let mut u8_7: u8 = 2u8;
    let mut u8_8: u8 = 1u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_21: i8 = 47i8;
    let mut i8_22: i8 = 23i8;
    let mut i8_23: i8 = -43i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 28i8;
    let mut i8_25: i8 = 18i8;
    let mut i8_26: i8 = 9i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i64_5: i64 = 46i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i64_6: i64 = 77i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i8_27: i8 = 47i8;
    let mut i8_28: i8 = 92i8;
    let mut i8_29: i8 = -10i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 99u32;
    let mut u8_9: u8 = 43u8;
    let mut u8_10: u8 = 42u8;
    let mut u8_11: u8 = 9u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_7: i64 = 62i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i8_30: i8 = -52i8;
    let mut i8_31: i8 = 16i8;
    let mut i8_32: i8 = -10i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut f32_0: f32 = 59.878166f32;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_8: i64 = -90i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut i64_9: i64 = -93i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_14_ref_0: &mut crate::duration::Duration = &mut duration_14;
    let mut i8_33: i8 = -123i8;
    let mut i8_34: i8 = -46i8;
    let mut i8_35: i8 = -11i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i64_10: i64 = 99i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut i32_1: i32 = 11i32;
    let mut i64_11: i64 = 13i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new(i64_11, i32_1);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_16, duration_15);
    let mut u32_4: u32 = 40u32;
    let mut u8_12: u8 = 91u8;
    let mut u8_13: u8 = 27u8;
    let mut u8_14: u8 = 28u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_36: i8 = 70i8;
    let mut i8_37: i8 = -62i8;
    let mut i8_38: i8 = -70i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut u16_0: u16 = 88u16;
    let mut u16_1: u16 = 0u16;
    let mut u8_15: u8 = 48u8;
    let mut u8_16: u8 = 39u8;
    let mut u8_17: u8 = 46u8;
    let mut i64_12: i64 = 155i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds(i64_12);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_17, u8_16, u8_15, u16_1);
    let mut duration_18_ref_0: &mut crate::duration::Duration = &mut duration_18;
    std::ops::DivAssign::div_assign(duration_18_ref_0, u16_0);
    std::ops::AddAssign::add_assign(duration_14_ref_0, duration_13);
    std::ops::AddAssign::add_assign(date_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2366() {
    rusty_monitor::set_test_id(2366);
    let mut i32_0: i32 = -34i32;
    let mut i32_1: i32 = -19i32;
    let mut i64_0: i64 = 72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i32_2: i32 = -27i32;
    let mut i64_1: i64 = 87i64;
    let mut i32_3: i32 = 19i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = 54i32;
    let mut i64_2: i64 = 76i64;
    let mut f32_0: f32 = -95.947368f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_3: i64 = -156i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut bool_0: bool = crate::util::is_leap_year(i32_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2262() {
    rusty_monitor::set_test_id(2262);
    let mut i64_0: i64 = -35i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f32_0: f32 = -34.929034f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_0: i8 = 35i8;
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u8_0: u8 = 92u8;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i32_0: i32 = 16i32;
    let mut i32_1: i32 = 104i32;
    let mut i64_1: i64 = -71i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut u16_0: u16 = 83u16;
    let mut i32_2: i32 = -17i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut i128_0: i128 = -131i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -16i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 1u16;
    let mut i32_3: i32 = 179i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut f64_0: f64 = 56.382990f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    std::ops::AddAssign::add_assign(time_1_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4726() {
    rusty_monitor::set_test_id(4726);
    let mut i32_0: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_1: i32 = 8i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_2: i32 = -6i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i32_3: i32 = -189i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i32_4: i32 = -50i32;
    let mut i64_1: i64 = 42i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u32_1: u32 = 70u32;
    let mut u8_3: u8 = 16u8;
    let mut u8_4: u8 = 45u8;
    let mut u8_5: u8 = 63u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_5: i32 = 40i32;
    let mut i64_2: i64 = -198i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_5);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i32_1);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1624() {
    rusty_monitor::set_test_id(1624);
    let mut u32_0: u32 = 2u32;
    let mut i8_0: i8 = -39i8;
    let mut i8_1: i8 = -120i8;
    let mut i8_2: i8 = 19i8;
    let mut i128_0: i128 = -98i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = 34i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_0: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i32_1: i32 = -73i32;
    let mut i64_1: i64 = 22i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f32_0: f32 = -101.553629f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = 87i32;
    let mut i64_2: i64 = -36i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut u32_1: u32 = 2u32;
    let mut u8_0: u8 = 81u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 14u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i8_3: i8 = -30i8;
    let mut i8_4: i8 = 70i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -45i8;
    let mut i8_7: i8 = -42i8;
    let mut i8_8: i8 = 27i8;
    let mut u8_3: u8 = crate::date::Date::sunday_based_week(date_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2921() {
    rusty_monitor::set_test_id(2921);
    let mut i8_0: i8 = -4i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = -30i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -95i32;
    let mut i64_0: i64 = 11i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_0: u8 = 79u8;
    let mut i32_1: i32 = -35i32;
    let mut i32_2: i32 = 67i32;
    let mut i32_3: i32 = -32i32;
    let mut i64_1: i64 = -148i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_4: i32 = -125i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i32_5: i32 = -78i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_5};
    let mut i32_6: i32 = -136i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_6};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_1);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut u32_0: u32 = 33u32;
    let mut u8_1: u8 = 67u8;
    let mut u8_2: u8 = 19u8;
    let mut u8_3: u8 = 89u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_7: i32 = -45i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_2);
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    std::ops::DivAssign::div_assign(duration_0_ref_0, i32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2103() {
    rusty_monitor::set_test_id(2103);
    let mut i16_0: i16 = -42i16;
    let mut i32_0: i32 = 55i32;
    let mut i64_0: i64 = 137i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 17i32;
    let mut i64_1: i64 = 43i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut u16_0: u16 = 88u16;
    let mut i32_2: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_2: i64 = 2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut u16_1: u16 = 33u16;
    let mut i32_3: i32 = -114i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_3);
    let mut i8_0: i8 = -22i8;
    let mut i8_1: i8 = -97i8;
    let mut i8_2: i8 = 113i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 63u8;
    let mut u8_1: u8 = 89u8;
    let mut u8_2: u8 = 74u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut i64_4: i64 = 24i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut f64_0: f64 = -59.755607f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f64_1: f64 = 104.765623f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 92u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 77u8;
    let mut u8_5: u8 = 56u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_2: u16 = 54u16;
    let mut i32_4: i32 = 82i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_2};
    let mut i32_5: i32 = crate::primitive_date_time::PrimitiveDateTime::to_julian_day(primitivedatetime_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_209() {
    rusty_monitor::set_test_id(209);
    let mut u8_0: u8 = 90u8;
    let mut i32_0: i32 = 63i32;
    let mut u16_0: u16 = 91u16;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 35u8;
    let mut u8_3: u8 = 4u8;
    let mut i64_0: i64 = -339i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i128_0: i128 = 58i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_1: i64 = -20i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut f64_0: f64 = 189.511375f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = 16i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i64_3: i64 = 33i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_4: i64 = -195i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i32_1: i32 = -132i32;
    let mut i128_1: i128 = -230i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_1);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_11);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut i8_0: i8 = 10i8;
    let mut i8_1: i8 = -28i8;
    let mut i8_2: i8 = 100i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut u32_0: u32 = 91u32;
    let mut u8_4: u8 = 96u8;
    let mut u8_5: u8 = 76u8;
    let mut u8_6: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_0);
    let mut i32_2: i32 = 69i32;
    let mut i64_5: i64 = 21i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_2);
    let mut i32_3: i32 = 115i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_14);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_3, u8_2, u8_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    std::ops::MulAssign::mul_assign(duration_9_ref_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7612() {
    rusty_monitor::set_test_id(7612);
    let mut f32_0: f32 = 61.957035f32;
    let mut u16_0: u16 = 64u16;
    let mut i32_0: i32 = 38i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i64_0: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_0: i8 = -80i8;
    let mut i8_1: i8 = 49i8;
    let mut i8_2: i8 = -69i8;
    let mut u16_1: u16 = 67u16;
    let mut i32_1: i32 = 94i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i8_3: i8 = -73i8;
    let mut i8_4: i8 = -31i8;
    let mut i8_5: i8 = 48i8;
    let mut i128_0: i128 = 13i128;
    let mut i128_1: i128 = -98i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_1: i64 = 34i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i32_2: i32 = 56i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut i32_3: i32 = -73i32;
    let mut i64_2: i64 = 22i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f32_1: f32 = -101.553629f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_4: i32 = 87i32;
    let mut i64_3: i64 = -36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 14u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_6: i8 = -30i8;
    let mut i8_7: i8 = 70i8;
    let mut i8_8: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -45i8;
    let mut i8_10: i8 = -42i8;
    let mut i8_11: i8 = 27i8;
    let mut u8_3: u8 = crate::date::Date::sunday_based_week(date_3);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut u8_4: u8 = crate::time::Time::minute(time_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5679() {
    rusty_monitor::set_test_id(5679);
    let mut i8_0: i8 = 87i8;
    let mut i64_0: i64 = 22i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_1: i8 = -73i8;
    let mut i8_2: i8 = -31i8;
    let mut i8_3: i8 = 48i8;
    let mut i128_0: i128 = 13i128;
    let mut i128_1: i128 = -98i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_1: i64 = 34i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_0, duration_2);
    let mut i32_0: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i32_1: i32 = -73i32;
    let mut i64_2: i64 = 22i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f32_0: f32 = -101.553629f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 81u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 14u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_4: i8 = -30i8;
    let mut i8_5: i8 = 70i8;
    let mut i8_6: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i8_7: i8 = -45i8;
    let mut i8_8: i8 = -42i8;
    let mut i8_9: i8 = 27i8;
    let mut u8_3: u8 = crate::date::Date::sunday_based_week(date_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_3, i8_2, i8_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::DivAssign::div_assign(duration_5_ref_0, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1270() {
    rusty_monitor::set_test_id(1270);
    let mut i8_0: i8 = 34i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_0: i64 = 166i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = 32i32;
    let mut i64_2: i64 = -50i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_0);
    let mut f32_0: f32 = 16.282288f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_1: i8 = -8i8;
    let mut i8_2: i8 = -6i8;
    let mut i8_3: i8 = -38i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i8_4: i8 = 32i8;
    let mut i8_5: i8 = 16i8;
    let mut i8_6: i8 = -8i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 68u8;
    let mut u8_2: u8 = 99u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 98u16;
    let mut i32_1: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut i64_3: i64 = -111i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_sub(offsetdatetime_1, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, i8_0);
    panic!("From RustyUnit with love");
}
}