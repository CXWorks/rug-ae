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
	use std::ops::SubAssign;
	use std::ops::AddAssign;
	use std::ops::DivAssign;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_657() {
//    rusty_monitor::set_test_id(657);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = 87i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut i64_2: i64 = 3600i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 94u8;
    let mut u8_5: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    let mut i32_0: i32 = 3652425i32;
    let mut i64_3: i64 = 86400i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut f64_1: f64 = 19.748904f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = 82i32;
    let mut i32_2: i32 = 48i32;
    let mut i64_4: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_1);
    let mut u16_0: u16 = 60u16;
    let mut i32_3: i32 = 291i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    std::ops::AddAssign::add_assign(time_1_ref_0, duration_2);
    std::ops::AddAssign::add_assign(time_0_ref_0, duration_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_481() {
//    rusty_monitor::set_test_id(481);
    let mut u16_0: u16 = 365u16;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u16_1: u16 = 7u16;
    let mut i64_1: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u16_2: u16 = 366u16;
    let mut i64_2: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut u16_3: u16 = 43u16;
    let mut i64_3: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut u16_4: u16 = 60u16;
    let mut i64_4: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut u16_5: u16 = 0u16;
    let mut i128_0: i128 = 1i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut u16_6: u16 = 19u16;
    let mut i64_5: i64 = 86400i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::DivAssign::div_assign(duration_6_ref_0, u16_6);
    std::ops::DivAssign::div_assign(duration_5_ref_0, u16_5);
    std::ops::DivAssign::div_assign(duration_4_ref_0, u16_4);
    std::ops::DivAssign::div_assign(duration_3_ref_0, u16_3);
    std::ops::DivAssign::div_assign(duration_2_ref_0, u16_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, u16_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_514() {
//    rusty_monitor::set_test_id(514);
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = 604800i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2_ref_0: &mut crate::instant::Instant = &mut instant_2;
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = 24i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &mut crate::instant::Instant = &mut instant_3;
    let mut u8_0: u8 = 5u8;
    let mut i64_5: i64 = 1i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i64_6: i64 = 2440588i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    std::ops::SubAssign::sub_assign(instant_3_ref_0, duration_7);
    std::ops::SubAssign::sub_assign(instant_2_ref_0, duration_4);
    std::ops::SubAssign::sub_assign(instant_1_ref_0, duration_1);
    std::ops::SubAssign::sub_assign(instant_0_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_83() {
//    rusty_monitor::set_test_id(83);
    let mut i32_0: i32 = 99i32;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 1i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i32_1: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_2: i32 = 314i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_3: i32 = 359i32;
    let mut i64_4: i64 = 2147483647i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut u32_1: u32 = 100000000u32;
    let mut u16_1: u16 = 0u16;
    std::ops::DivAssign::div_assign(duration_5_ref_0, i32_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut bool_1: bool = crate::duration::Duration::is_negative(duration_3);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7076() {
//    rusty_monitor::set_test_id(7076);
    let mut i64_0: i64 = -54i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut i64_1: i64 = 604800i64;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 0u8;
    let mut i32_0: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::June;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 400i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_2: i32 = 156i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_3_ref_0, f32_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_hours(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_648() {
//    rusty_monitor::set_test_id(648);
    let mut u16_0: u16 = 60u16;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u16_1: u16 = 999u16;
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u16_2: u16 = 7u16;
    let mut i128_0: i128 = 1i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_0: i32 = -123i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 178i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_0);
    let mut i64_2: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, u16_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u16_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7994() {
//    rusty_monitor::set_test_id(7994);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 116i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_1: i32 = 201i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 11u8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i16_0: i16 = 5i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i64_0: i64 = 3600i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_2_ref_0, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_3, duration_1);
    let mut duration_4: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_550() {
//    rusty_monitor::set_test_id(550);
    let mut i32_0: i32 = 365i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = -10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u16_0: u16 = 999u16;
    let mut i32_1: i32 = 0i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_539() {
//    rusty_monitor::set_test_id(539);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 367u16;
    let mut i32_0: i32 = 36525i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 127i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 2i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 23i8;
    let mut i8_10: i8 = 2i8;
    let mut i8_11: i8 = 3i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 246i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_1);
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    std::ops::SubAssign::sub_assign(offsetdatetime_2_ref_0, duration_1);
    std::ops::SubAssign::sub_assign(offsetdatetime_0_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_434() {
//    rusty_monitor::set_test_id(434);
    let mut i32_0: i32 = 195i32;
    let mut i64_0: i64 = 1000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i8_6: i8 = 127i8;
    let mut i8_7: i8 = -104i8;
    let mut i8_8: i8 = 1i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -25i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i64_4: i64 = 1000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i64_5: i64 = 2147483647i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_6);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i16_0: i16 = 10i16;
    let mut i64_6: i64 = 2440588i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut u16_0: u16 = 60u16;
    let mut i32_1: i32 = 359i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2_ref_0: &mut crate::date::Date = &mut date_2;
    std::ops::SubAssign::sub_assign(date_1_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7314() {
//    rusty_monitor::set_test_id(7314);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 3u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 103i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 0u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i32_1: i32 = 178i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 60i64;
    let mut i64_2: i64 = 1000i64;
    let mut i64_3: i64 = 129i64;
    let mut str_0: &str = "DifferentVariant";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_4: i64 = 177i64;
    let mut i64_5: i64 = 1000i64;
    let mut i64_6: i64 = 29i64;
    let mut str_1: &str = "PrimitiveDateTime";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut i32_2: i32 = 240i32;
    let mut i32_3: i32 = 6i32;
    let mut i64_7: i64 = 9223372036854775807i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_3);
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_7, i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_2);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_471() {
//    rusty_monitor::set_test_id(471);
    let mut i32_0: i32 = 314i32;
    let mut i64_0: i64 = 168i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i32_1: i32 = 65i32;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i32_2: i32 = 99i32;
    let mut i64_2: i64 = -47i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i32_3: i32 = 2i32;
    let mut i64_3: i64 = 91i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_4: i32 = 212i32;
    let mut i64_4: i64 = 2147483647i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i32_5: i32 = 9i32;
    let mut i32_6: i32 = -28i32;
    let mut i64_5: i64 = 9223372036854775807i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_6);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, i32_5);
    std::ops::MulAssign::mul_assign(duration_5_ref_0, i32_4);
    std::ops::MulAssign::mul_assign(duration_4_ref_0, i32_3);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i32_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i32_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_766() {
//    rusty_monitor::set_test_id(766);
    let mut i16_0: i16 = 8i16;
    let mut i32_0: i32 = 16i32;
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u16_0: u16 = 23u16;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 5u8;
    let mut u32_0: u32 = 999999999u32;
    let mut u8_3: u8 = 28u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    std::ops::DivAssign::div_assign(duration_1_ref_0, i16_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4080() {
//    rusty_monitor::set_test_id(4080);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = 143i64;
    let mut i64_3: i64 = 1000i64;
    let mut i64_4: i64 = 604800i64;
    let mut str_0: &str = "date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_5: i64 = 63i64;
    let mut i64_6: i64 = 12i64;
    let mut i64_7: i64 = 2i64;
    let mut str_1: &str = "utc_datetime";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut u16_0: u16 = 999u16;
    let mut i32_1: i32 = 325i32;
    let mut i64_8: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 150i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_1, primitivedatetime_0);
    let mut date_2_ref_0: &mut crate::date::Date = &mut date_2;
    std::ops::SubAssign::sub_assign(date_2_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8719() {
//    rusty_monitor::set_test_id(8719);
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = 246i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_1: i32 = 348i32;
    let mut i64_3: i64 = -57i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut i32_2: i32 = 116i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_3: i32 = 201i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 11u8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i16_0: i16 = 5i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut i64_4: i64 = 3600i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_8_ref_0, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_9, duration_7);
    let mut duration_10: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_476() {
//    rusty_monitor::set_test_id(476);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut f64_1: f64 = 0.000000f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut f64_2: f64 = 4696837146684686336.000000f64;
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f64_3: f64 = 4815374002031689728.000000f64;
    let mut i32_0: i32 = 25i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut f64_4: f64 = 4828193600913801216.000000f64;
    let mut i128_0: i128 = 28i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut f64_5: f64 = 4794699203894837248.000000f64;
    let mut i64_3: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut f64_6: f64 = 4794699203894837248.000000f64;
    let mut i32_1: i32 = 303i32;
    let mut i64_4: i64 = 2147483647i64;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, f64_5);
    std::ops::MulAssign::mul_assign(duration_5_ref_0, f64_4);
    std::ops::MulAssign::mul_assign(duration_4_ref_0, f64_3);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, f64_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, f64_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6738() {
//    rusty_monitor::set_test_id(6738);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 400i32;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 20i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut u32_1: u32 = 16u32;
    let mut u8_3: u8 = 80u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 3u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = 246i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 0u32;
    let mut u8_6: u8 = 8u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 85u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_3: i32 = 33i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_2);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut i32_4: i32 = 348i32;
    let mut i64_2: i64 = -57i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_2: f64 = 4768169126130614272.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i32_5: i32 = 116i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_4, duration_7);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_5, utcoffset_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_5);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_6: i32 = 201i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut u32_3: u32 = 100000000u32;
    let mut u8_9: u8 = 1u8;
    let mut u8_10: u8 = 1u8;
    let mut u8_11: u8 = 11u8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_3: f64 = 4696837146684686336.000000f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut i16_0: i16 = 5i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut i64_3: i64 = 3600i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_9_ref_0, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_10, duration_8);
    let mut duration_11: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_11, u8_10, u8_9, u32_3);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_3);
    let mut duration_11_ref_0: &mut crate::duration::Duration = &mut duration_11;
    std::ops::MulAssign::mul_assign(duration_11_ref_0, f64_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_0};
    std::ops::SubAssign::sub_assign(primitivedatetime_1_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7955() {
//    rusty_monitor::set_test_id(7955);
    let mut i64_0: i64 = -89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u16_0: u16 = 367u16;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 8u8;
    let mut i32_0: i32 = 320i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i128_0: i128 = -15i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000000u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 10u8;
    let mut u8_5: u8 = 26u8;
    let mut i32_1: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_1: i64 = 1i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = 400i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut f64_0: f64 = -100.758032f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_5);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_2: i64 = 1000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = 172i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut u16_1: u16 = 367u16;
    let mut i32_3: i32 = 16i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_10);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    std::ops::AddAssign::add_assign(date_1_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8764() {
//    rusty_monitor::set_test_id(8764);
    let mut i32_0: i32 = 76i32;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 80i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u16_0: u16 = 999u16;
    let mut i32_1: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_2: i32 = 116i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_3);
    let mut i32_3: i32 = 1721425i32;
    let mut i64_3: i64 = -108i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_4: i32 = 122i32;
    let mut i64_4: i64 = 1000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 70u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_2);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_5: i64 = 86400i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i64_6: i64 = 24i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 60i8;
    let mut i8_10: i8 = 24i8;
    let mut i8_11: i8 = 0i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_7: i64 = 86400i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut i32_5: i32 = 150i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut month_0: month::Month = crate::date::Date::month(date_4);
    let mut i64_8: i64 = 253402300799i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_9: i64 = 0i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut u16_1: u16 = 10u16;
    let mut i32_6: i32 = 5i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_9);
    let mut time_2_ref_0: &mut crate::time::Time = &mut time_2;
    std::ops::SubAssign::sub_assign(time_2_ref_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7321() {
//    rusty_monitor::set_test_id(7321);
    let mut i32_0: i32 = 150i32;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 280i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut u32_0: u32 = 28u32;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 10000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i32_2: i32 = 3652425i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_2: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_6: i8 = 60i8;
    let mut i8_7: i8 = -46i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_2: u32 = 100000000u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 59u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut f64_0: f64 = -4.620003f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_9: i8 = -69i8;
    let mut i8_10: i8 = 3i8;
    let mut i8_11: i8 = 2i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_3: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut f64_1: f64 = 4794699203894837248.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_12: i8 = 1i8;
    let mut i8_13: i8 = 6i8;
    let mut i8_14: i8 = -47i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::DivAssign::div_assign(duration_3_ref_0, u32_0);
    let mut u8_6: u8 = crate::time::Time::hour(time_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_654() {
//    rusty_monitor::set_test_id(654);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 5119853i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 16i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_2: i32 = 1721425i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u16_1: u16 = 1u16;
    let mut i32_3: i32 = -17i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_3_ref_0: &mut crate::date::Date = &mut date_3;
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, f32_0);
    std::ops::SubAssign::sub_assign(primitivedatetime_1_ref_0, duration_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7522() {
//    rusty_monitor::set_test_id(7522);
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 59u16;
    let mut i32_0: i32 = 88i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_1: i32 = 20i32;
    let mut i64_2: i64 = 604800i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut i32_2: i32 = 130i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 3i8;
    let mut i32_3: i32 = 387i32;
    let mut i64_3: i64 = -4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut u16_1: u16 = 366u16;
    let mut i32_4: i32 = 400i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_3);
    let mut i32_5: i32 = 15i32;
    let mut i64_4: i64 = 6i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_5);
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 39u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_5: i64 = 2440588i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_6: i32 = 353i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i64_6: i64 = 3600i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i128_0: i128 = 1i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_7: i64 = 42i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut u16_2: u16 = 999u16;
    let mut i32_7: i32 = 229i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_2);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut u8_6: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut date_4_ref_0: &mut crate::date::Date = &mut date_4;
    std::ops::AddAssign::add_assign(date_4_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_95() {
//    rusty_monitor::set_test_id(95);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 62i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 0i8;
    let mut i8_7: i8 = 6i8;
    let mut i8_8: i8 = 127i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut i64_1: i64 = 242i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    std::ops::AddAssign::add_assign(offsetdatetime_1_ref_0, duration_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_477() {
//    rusty_monitor::set_test_id(477);
    let mut i8_0: i8 = -14i8;
    let mut i32_0: i32 = 133i32;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i8_1: i8 = 1i8;
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i8_2: i8 = 6i8;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i8_3: i8 = 1i8;
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i8_4: i8 = 127i8;
    let mut i32_1: i32 = 342i32;
    let mut i64_2: i64 = 1000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i8_5: i8 = 2i8;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::DivAssign::div_assign(duration_6_ref_0, i8_5);
    std::ops::DivAssign::div_assign(duration_5_ref_0, i8_4);
    std::ops::DivAssign::div_assign(duration_4_ref_0, i8_3);
    std::ops::DivAssign::div_assign(duration_3_ref_0, i8_2);
    std::ops::DivAssign::div_assign(duration_2_ref_0, i8_1);
    std::ops::DivAssign::div_assign(duration_1_ref_0, i8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_480() {
//    rusty_monitor::set_test_id(480);
    let mut u8_0: u8 = 30u8;
    let mut i32_0: i32 = 400i32;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u8_1: u8 = 60u8;
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u8_2: u8 = 31u8;
    let mut i32_1: i32 = 1721119i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut u8_3: u8 = 76u8;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = 1000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut u8_4: u8 = 5u8;
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut u8_5: u8 = 8u8;
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::DivAssign::div_assign(duration_8_ref_0, u8_5);
    std::ops::DivAssign::div_assign(duration_7_ref_0, u8_4);
    std::ops::DivAssign::div_assign(duration_6_ref_0, u8_3);
    std::ops::DivAssign::div_assign(duration_3_ref_0, u8_2);
    std::ops::DivAssign::div_assign(duration_1_ref_0, u8_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7840() {
//    rusty_monitor::set_test_id(7840);
    let mut i32_0: i32 = 4i32;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_1: i32 = 331i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut i32_2: i32 = 348i32;
    let mut i64_3: i64 = -57i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i32_3: i32 = 116i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_7);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_4: i32 = 201i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 11u8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i16_0: i16 = 5i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut i64_4: i64 = 3600i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_9_ref_0, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_10, duration_8);
    let mut duration_11: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    std::ops::SubAssign::sub_assign(duration_4_ref_0, duration_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_571() {
//    rusty_monitor::set_test_id(571);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut f32_0: f32 = -131.831925f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut f32_1: f32 = -66.571955f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 5u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 9u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 12u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_2_ref_0: &mut crate::time::Time = &mut time_2;
    std::ops::SubAssign::sub_assign(time_2_ref_0, duration_4);
    std::ops::SubAssign::sub_assign(time_1_ref_0, duration_3);
    std::ops::SubAssign::sub_assign(time_0_ref_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_454() {
//    rusty_monitor::set_test_id(454);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 144i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut f64_1: f64 = 4741671816366391296.000000f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut i64_3: i64 = 1000000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut f32_1: f32 = -24.133084f32;
    let mut i128_0: i128 = 0i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 224i32;
    let mut i64_4: i64 = 0i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_0);
    let mut duration_12_ref_0: &mut crate::duration::Duration = &mut duration_12;
    std::ops::AddAssign::add_assign(duration_10_ref_0, duration_9);
    std::ops::AddAssign::add_assign(duration_7_ref_0, duration_6);
    std::ops::AddAssign::add_assign(duration_4_ref_0, duration_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2429() {
//    rusty_monitor::set_test_id(2429);
    let mut u32_0: u32 = 1000000u32;
    let mut i64_0: i64 = 80i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i32_1: i32 = 116i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 1721425i32;
    let mut i64_1: i64 = -108i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut u32_1: u32 = 999999u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_3: i32 = 122i32;
    let mut i64_2: i64 = 1000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut u32_2: u32 = 15u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 70u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_2);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_3: i64 = 86402i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i64_4: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 60i8;
    let mut i8_10: i8 = 24i8;
    let mut i8_11: i8 = 0i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_5: i64 = 86400i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i32_4: i32 = 150i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut month_0: month::Month = crate::date::Date::month(date_3);
    let mut i64_6: i64 = 253402300799i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_7: i64 = 0i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut u16_1: u16 = 10u16;
    let mut i32_5: i32 = 5i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_6);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_470() {
//    rusty_monitor::set_test_id(470);
    let mut i16_0: i16 = 5i16;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i16_1: i16 = 7i16;
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i16_2: i16 = 6i16;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i16_3: i16 = 6i16;
    let mut i64_2: i64 = -61i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i16_4: i16 = 7i16;
    let mut i64_3: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i16_5: i16 = 60i16;
    let mut u16_0: u16 = 10u16;
    let mut i64_4: i64 = 604800i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    std::ops::MulAssign::mul_assign(duration_6_ref_0, i16_4);
    std::ops::MulAssign::mul_assign(duration_5_ref_0, i16_3);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i16_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i16_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2475() {
//    rusty_monitor::set_test_id(2475);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 398i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i32_1: i32 = 296i32;
    let mut i32_2: i32 = 7i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 70u8;
    let mut u8_1: u8 = 93u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i32_3: i32 = 133i32;
    let mut i64_4: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i64_5: i64 = 12i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_7);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u8_3: u8 = crate::date::Date::monday_based_week(date_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i32_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1706() {
//    rusty_monitor::set_test_id(1706);
    let mut i32_0: i32 = 48i32;
    let mut i64_0: i64 = 3600i64;
    let mut i32_1: i32 = 48i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i32_2: i32 = 189i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_3: i32 = 348i32;
    let mut i64_2: i64 = -57i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut i32_4: i32 = 116i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_5: i32 = 201i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 11u8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i16_0: i16 = 5i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut i64_3: i64 = 3600i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_7_ref_0, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_8, duration_6);
    let mut duration_9: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_2);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8354() {
//    rusty_monitor::set_test_id(8354);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 364i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 55u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_1: i32 = 116i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i32_2: i32 = 201i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 11u8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i16_0: i16 = 5i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i64_0: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_3_ref_0, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_4, duration_2);
    let mut duration_5: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
//    panic!("From RustyUnit with love");
}
}