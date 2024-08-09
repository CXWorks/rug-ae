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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2863() {
//    rusty_monitor::set_test_id(2863);
    let mut i32_0: i32 = 365i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = -81i32;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_4);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 2u8;
    let mut i128_1: i128 = 9223372036854775807i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_4_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_4;
    std::ops::AddAssign::add_assign(offsetdatetime_4_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_319() {
//    rusty_monitor::set_test_id(319);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_0: i64 = -38i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 67i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_1: f64 = 4652007308841189376.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_1: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i64_2: i64 = 2440588i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_3: i8 = 4i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 59i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i8_9: i8 = 3i8;
    let mut i8_10: i8 = 60i8;
    let mut i8_11: i8 = 2i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 29u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_12: i8 = 3i8;
    let mut i8_13: i8 = 6i8;
    let mut i8_14: i8 = 5i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_2: u32 = 10000000u32;
    let mut u8_6: u8 = 23u8;
    let mut u8_7: u8 = 19u8;
    let mut u8_8: u8 = 4u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i128_0: i128 = 1000i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = 296i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_10);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut f64_2: f64 = 4794699203894837248.000000f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut u32_3: u32 = 100000u32;
    let mut f32_0: f32 = -84.219210f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_3: i64 = 604800i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 999999u32;
    let mut u8_9: u8 = 31u8;
    let mut u8_10: u8 = 7u8;
    let mut u8_11: u8 = 65u8;
    let mut i64_4: i64 = 253402300799i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut i32_2: i32 = 178i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    std::ops::AddAssign::add_assign(offsetdatetime_1_ref_0, duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4859() {
//    rusty_monitor::set_test_id(4859);
    let mut u32_0: u32 = 100u32;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 16i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i64_3: i64 = 0i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_5_ref_0, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_6, i32_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_198() {
//    rusty_monitor::set_test_id(198);
    let mut i8_0: i8 = 0i8;
    let mut i64_0: i64 = -113i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i8_1: i8 = 2i8;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i8_2: i8 = 0i8;
    let mut i64_2: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i8_3: i8 = 24i8;
    let mut i64_3: i64 = 1000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i8_4: i8 = -15i8;
    let mut i32_0: i32 = 73i32;
    let mut i64_4: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i8_5: i8 = -19i8;
    let mut u8_0: u8 = 12u8;
    let mut i32_1: i32 = 3600i32;
    let mut i64_5: i64 = 1i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_1);
    std::ops::MulAssign::mul_assign(duration_5_ref_0, i8_4);
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i8_3);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i8_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i8_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_209() {
//    rusty_monitor::set_test_id(209);
    let mut i8_0: i8 = 24i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i8_1: i8 = 127i8;
    let mut i64_0: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i8_2: i8 = 6i8;
    let mut i64_1: i64 = 3600i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i8_3: i8 = 60i8;
    let mut i64_2: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i8_4: i8 = 6i8;
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i8_5: i8 = -56i8;
    let mut i64_4: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i8_6: i8 = 3i8;
    let mut i16_0: i16 = 6i16;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    std::ops::DivAssign::div_assign(duration_5_ref_0, i8_5);
    std::ops::DivAssign::div_assign(duration_4_ref_0, i8_4);
    std::ops::DivAssign::div_assign(duration_3_ref_0, i8_3);
    std::ops::DivAssign::div_assign(duration_2_ref_0, i8_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, i8_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, i8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_561() {
//    rusty_monitor::set_test_id(561);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut f64_1: f64 = 4652007308841189376.000000f64;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f64_2: f64 = 4815374002031689728.000000f64;
    let mut i64_2: i64 = 269i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut f64_3: f64 = 4828193600913801216.000000f64;
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut f64_4: f64 = 79.950940f64;
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut f64_5: f64 = -67.747699f64;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::MulAssign::mul_assign(duration_8_ref_0, f64_5);
    std::ops::MulAssign::mul_assign(duration_7_ref_0, f64_4);
    std::ops::MulAssign::mul_assign(duration_6_ref_0, f64_3);
    std::ops::MulAssign::mul_assign(duration_4_ref_0, f64_2);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, f64_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_174() {
//    rusty_monitor::set_test_id(174);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut i64_1: i64 = 224i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
    let mut i64_2: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2_ref_0: &mut crate::instant::Instant = &mut instant_2;
    let mut i32_0: i32 = 133i32;
    let mut i64_3: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &mut crate::instant::Instant = &mut instant_3;
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5_ref_0: &mut crate::instant::Instant = &mut instant_5;
    let mut u32_0: u32 = 1000u32;
    let mut f64_0: f64 = 17.989145f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6_ref_0: &mut crate::instant::Instant = &mut instant_6;
    std::ops::SubAssign::sub_assign(instant_5_ref_0, duration_9);
    std::ops::SubAssign::sub_assign(instant_3_ref_0, duration_7);
    std::ops::SubAssign::sub_assign(instant_2_ref_0, duration_5);
    std::ops::SubAssign::sub_assign(instant_1_ref_0, duration_3);
    std::ops::SubAssign::sub_assign(instant_0_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6581() {
//    rusty_monitor::set_test_id(6581);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 16i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_5, i32_1);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
    let mut i64_3: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6051() {
//    rusty_monitor::set_test_id(6051);
    let mut i32_0: i32 = 376i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut i32_1: i32 = 100i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i32_2: i32 = 348i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_0: i64 = -5i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = 2147483647i32;
    let mut i64_1: i64 = -46i64;
    let mut u16_0: u16 = 55u16;
    let mut i32_4: i32 = -190i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut u32_0: u32 = 10000000u32;
    let mut i64_2: i64 = 3600i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_3: i64 = -102i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8714() {
//    rusty_monitor::set_test_id(8714);
    let mut i16_0: i16 = 8i16;
    let mut i64_0: i64 = 604800i64;
    let mut i64_1: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 604800i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 392i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 392i32;
    let mut i64_3: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut u16_1: u16 = 366u16;
    let mut i32_2: i32 = 336i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
    let mut month_0: month::Month = crate::date::Date::month(date_3);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    std::ops::DivAssign::div_assign(duration_7_ref_0, i16_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_150() {
//    rusty_monitor::set_test_id(150);
    let mut f64_0: f64 = 4815374002031689728.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 314i32;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i32_1: i32 = 308i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = 320i32;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut f64_1: f64 = 4652007308841189376.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_3: i64 = 60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut i64_4: i64 = 36i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut i16_0: i16 = 4i16;
    let mut i64_5: i64 = 60i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i64_6: i64 = 1000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut u8_0: u8 = 0u8;
    let mut i64_7: i64 = 604800i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    std::ops::SubAssign::sub_assign(duration_9_ref_0, duration_8);
    std::ops::SubAssign::sub_assign(duration_5_ref_0, duration_2);
    std::ops::SubAssign::sub_assign(duration_1_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4367() {
//    rusty_monitor::set_test_id(4367);
    let mut f64_0: f64 = 4815374002031689728.000000f64;
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i64_0: i64 = -299i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 195i32;
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_0_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut f64_2: f64 = 0.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_5);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 268i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_1);
    let mut i32_2: i32 = 16i32;
    let mut i64_2: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_3: i64 = 3600i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut i64_4: i64 = 0i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_9_ref_0, duration_8);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_10, i32_2);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_2);
    std::ops::SubAssign::sub_assign(primitivedatetime_0_ref_0, duration_3);
    std::ops::DivAssign::div_assign(duration_0_ref_0, f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_254() {
//    rusty_monitor::set_test_id(254);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 82i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_1: i32 = -17i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_2: i64 = 60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_7);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8803() {
//    rusty_monitor::set_test_id(8803);
    let mut u16_0: u16 = 7u16;
    let mut i32_0: i32 = 1000i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = 82i32;
    let mut i64_1: i64 = 1i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 20u8;
    let mut i32_2: i32 = 257i32;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut i64_2: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -81i32;
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_8);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_7);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 2u8;
    let mut i128_1: i128 = 9223372036854775807i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_2: u32 = 10000000u32;
    let mut u8_6: u8 = 59u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_355() {
//    rusty_monitor::set_test_id(355);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut i32_0: i32 = 252i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut f32_0: f32 = -120.586510f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f64_2: f64 = 4768169126130614272.000000f64;
    let mut i64_1: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut f64_3: f64 = 4607182418800017408.000000f64;
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut f64_4: f64 = 4607182418800017408.000000f64;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut f64_5: f64 = 0.000000f64;
    let mut i64_3: i64 = 1000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut f64_6: f64 = 4828193600913801216.000000f64;
    let mut f32_1: f32 = 126.742660f32;
    let mut i64_4: i64 = 86400i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    std::ops::DivAssign::div_assign(duration_7_ref_0, f64_5);
    std::ops::DivAssign::div_assign(duration_6_ref_0, f64_4);
    std::ops::DivAssign::div_assign(duration_5_ref_0, f64_3);
    std::ops::DivAssign::div_assign(duration_4_ref_0, f64_2);
    std::ops::DivAssign::div_assign(duration_2_ref_0, f64_1);
    std::ops::DivAssign::div_assign(duration_1_ref_0, f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3706() {
//    rusty_monitor::set_test_id(3706);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_0: i32 = 9i32;
    let mut i32_1: i32 = 100i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut i32_2: i32 = 348i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_1: i64 = -5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = 2147483647i32;
    let mut i64_2: i64 = -46i64;
    let mut u16_0: u16 = 55u16;
    let mut i32_4: i32 = -190i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut u32_0: u32 = 10000000u32;
    let mut i64_3: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = -102i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i64_5: i64 = crate::duration::Duration::whole_weeks(duration_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::October;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    std::ops::AddAssign::add_assign(time_0_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_758() {
//    rusty_monitor::set_test_id(758);
    let mut i32_0: i32 = 348i32;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 16i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_5, i32_1);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7569() {
//    rusty_monitor::set_test_id(7569);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 93i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
    let mut i128_0: i128 = 1000000i128;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 268i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_2: i32 = 16i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_2: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i64_3: i64 = 0i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_6_ref_0, duration_5);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_7, i32_2);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
    std::ops::AddAssign::add_assign(date_0_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6103() {
//    rusty_monitor::set_test_id(6103);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 32i32;
    let mut i64_1: i64 = 1i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut f32_0: f32 = -49.468023f32;
    let mut i32_1: i32 = 9i32;
    let mut i32_2: i32 = 100i32;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_2);
    let mut i32_3: i32 = 348i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_2: i64 = -5i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut u16_0: u16 = 55u16;
    let mut i32_4: i32 = -190i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut f32_2: f32 = 1065353216.000000f32;
    let mut i64_3: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = -102i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i64_5: i64 = crate::duration::Duration::whole_weeks(duration_7);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::October;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_4, i32_1);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::DivAssign::div_assign(duration_8_ref_0, f32_0);
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    std::ops::SubAssign::sub_assign(offsetdatetime_0_ref_0, duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5885() {
//    rusty_monitor::set_test_id(5885);
    let mut u16_0: u16 = 5u16;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 2u8;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_0: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 16i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_5, i32_1);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7433() {
//    rusty_monitor::set_test_id(7433);
    let mut u8_0: u8 = 9u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 10u8;
    let mut u8_3: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut f64_1: f64 = 0.000000f64;
    let mut i128_0: i128 = 1i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_3: i8 = 2i8;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut u32_1: u32 = 100u32;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 12u8;
    let mut u8_6: u8 = 53u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut u32_2: u32 = 100000u32;
    let mut i64_1: i64 = 186i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u16_0: u16 = 39u16;
    let mut i32_0: i32 = -34i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::MulAssign::mul_assign(duration_2_ref_0, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4287() {
//    rusty_monitor::set_test_id(4287);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7398() {
//    rusty_monitor::set_test_id(7398);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 16i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_5, i32_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_504() {
//    rusty_monitor::set_test_id(504);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = 114i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 95u8;
    let mut u8_4: u8 = 14u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i64_0: i64 = -109i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i16_0: i16 = 5i16;
    let mut i32_1: i32 = 280i32;
    let mut i64_2: i64 = 1i64;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    std::ops::SubAssign::sub_assign(offsetdatetime_2_ref_0, duration_6);
    std::ops::SubAssign::sub_assign(offsetdatetime_1_ref_0, duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_270() {
//    rusty_monitor::set_test_id(270);
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_0: i32 = 381i32;
    let mut i64_3: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_1: i32 = 195i32;
    let mut i64_5: i64 = -91i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_1);
    let mut i64_6: i64 = 143i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut i64_7: i64 = 253402300799i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut i32_2: i32 = 370i32;
    let mut i64_8: i64 = -71i64;
    std::ops::AddAssign::add_assign(duration_10_ref_0, duration_7);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_4);
    std::ops::AddAssign::add_assign(duration_2_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6677() {
//    rusty_monitor::set_test_id(6677);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 212i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_0_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 268i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_1);
    let mut i32_2: i32 = 16i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_1: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i64_2: i64 = 0i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_6_ref_0, duration_5);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_7, i32_2);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_2);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    std::ops::SubAssign::sub_assign(primitivedatetime_0_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_232() {
//    rusty_monitor::set_test_id(232);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 15i32;
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 111i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    let mut i64_2: i64 = 2147483647i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_2: i32 = 100i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2_ref_0: &mut crate::date::Date = &mut date_2;
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_3: i32 = 76i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3_ref_0: &mut crate::date::Date = &mut date_3;
    let mut i64_4: i64 = 253402300799i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_4: i32 = 331i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4_ref_0: &mut crate::date::Date = &mut date_4;
    let mut i64_5: i64 = 2440588i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i32_5: i32 = 252i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_5_ref_0: &mut crate::date::Date = &mut date_5;
    std::ops::SubAssign::sub_assign(date_5_ref_0, duration_11);
    std::ops::SubAssign::sub_assign(date_4_ref_0, duration_9);
    std::ops::SubAssign::sub_assign(date_3_ref_0, duration_7);
    std::ops::SubAssign::sub_assign(date_2_ref_0, duration_5);
    std::ops::SubAssign::sub_assign(date_1_ref_0, duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7463() {
//    rusty_monitor::set_test_id(7463);
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 43i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 268i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_1);
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6753() {
//    rusty_monitor::set_test_id(6753);
    let mut i32_0: i32 = 18i32;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -63i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_1: i32 = 303i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_5);
    let mut u16_0: u16 = 60u16;
    let mut i32_2: i32 = 2i32;
    let mut i64_3: i64 = 86400i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 604800i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_11);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_2, u16_0);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    std::ops::SubAssign::sub_assign(time_1_ref_0, duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5433() {
//    rusty_monitor::set_test_id(5433);
    let mut i32_0: i32 = 376i32;
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = 511i32;
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i64_2: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_3: i32 = 16i32;
    let mut i64_4: i64 = 12i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_5: i64 = 3600i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut i64_6: i64 = 0i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_10_ref_0, duration_9);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_11, i32_3);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_1);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_302() {
//    rusty_monitor::set_test_id(302);
    let mut i16_0: i16 = 35i16;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i16_1: i16 = 4i16;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i16_2: i16 = 10i16;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i16_3: i16 = -9i16;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i16_4: i16 = 0i16;
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut i16_5: i16 = 227i16;
    let mut u32_0: u32 = 10000000u32;
    let mut i32_0: i32 = 5119853i32;
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_0);
    std::ops::MulAssign::mul_assign(duration_7_ref_0, i16_4);
    std::ops::MulAssign::mul_assign(duration_6_ref_0, i16_3);
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i16_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i16_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_435() {
//    rusty_monitor::set_test_id(435);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut i64_0: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut f32_2: f32 = -128.297976f32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f32_3: f32 = 1315859240.000000f32;
    let mut i32_0: i32 = 2i32;
    let mut i64_2: i64 = 1000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut f32_4: f32 = 1065353216.000000f32;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut f32_5: f32 = 16.596741f32;
    let mut i128_0: i128 = 1000000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::DivAssign::div_assign(duration_5_ref_0, f32_5);
    std::ops::DivAssign::div_assign(duration_4_ref_0, f32_4);
    std::ops::DivAssign::div_assign(duration_3_ref_0, f32_3);
    std::ops::DivAssign::div_assign(duration_2_ref_0, f32_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, f32_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, f32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4024() {
//    rusty_monitor::set_test_id(4024);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 9u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = -62i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 268i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut i32_2: i32 = 16i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i128_0: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_5, i32_2);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_3);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_216() {
//    rusty_monitor::set_test_id(216);
    let mut u16_0: u16 = 1u16;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u16_1: u16 = 10u16;
    let mut i32_0: i32 = -53i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u16_2: u16 = 7u16;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut u16_3: u16 = 60u16;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 178i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    std::ops::DivAssign::div_assign(duration_2_ref_0, u16_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, u16_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_522() {
//    rusty_monitor::set_test_id(522);
    let mut u8_0: u8 = 59u8;
    let mut i32_0: i32 = 263i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u8_1: u8 = 70u8;
    let mut i64_1: i64 = 1i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u8_2: u8 = 23u8;
    let mut i64_2: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut u8_3: u8 = 63u8;
    let mut i64_3: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut u8_4: u8 = 31u8;
    let mut i64_4: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i32_1: i32 = 16i32;
    let mut i64_5: i64 = 1i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut u8_5: u8 = 59u8;
    let mut i64_6: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    std::ops::DivAssign::div_assign(duration_7_ref_0, u8_5);
    std::ops::DivAssign::div_assign(duration_6_ref_0, u8_4);
    std::ops::DivAssign::div_assign(duration_3_ref_0, u8_3);
    std::ops::DivAssign::div_assign(duration_2_ref_0, u8_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, u8_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_196() {
//    rusty_monitor::set_test_id(196);
    let mut i32_0: i32 = 100i32;
    let mut i64_0: i64 = -26i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i32_1: i32 = 184i32;
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i32_2: i32 = -204i32;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i32_3: i32 = 5119853i32;
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_4: i32 = 111i32;
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = -94i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i32_5: i32 = -84i32;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut i64_5: i64 = 21i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    std::ops::MulAssign::mul_assign(duration_6_ref_0, i32_4);
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i32_3);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i32_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i32_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_349() {
//    rusty_monitor::set_test_id(349);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut i32_0: i32 = 285i32;
    let mut i64_1: i64 = -16i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f32_2: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut f32_3: f32 = -10.157748f32;
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 184i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    std::ops::MulAssign::mul_assign(duration_3_ref_0, f32_2);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, f32_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7080() {
//    rusty_monitor::set_test_id(7080);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = 86400i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = -52i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 263i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i64_1: i64 = -126i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &mut crate::instant::Instant = &mut instant_3;
    std::ops::SubAssign::sub_assign(instant_3_ref_0, duration_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_467() {
//    rusty_monitor::set_test_id(467);
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i64_0: i64 = 76i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 105i32;
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_2: i32 = 303i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i8_0: i8 = 93i8;
    let mut u32_0: u32 = 36u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 0u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_1: i8 = -72i8;
    let mut i8_2: i8 = 3i8;
    let mut i8_3: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_0);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 0i8;
    let mut i8_6: i8 = -69i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_6, utcoffset_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_2);
    let mut primitivedatetime_3_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}
}