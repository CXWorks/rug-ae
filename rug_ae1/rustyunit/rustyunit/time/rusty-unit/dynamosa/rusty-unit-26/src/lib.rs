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
fn rusty_test_1953() {
    rusty_monitor::set_test_id(1953);
    let mut i32_0: i32 = 3i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut f32_0: f32 = 12.612260f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_0: i64 = 173i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 63i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -75i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_3: i64 = -148i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_7);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_5);
    let mut i64_4: i64 = -180i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i32_2: i32 = 48i32;
    let mut i64_5: i64 = 21i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_10);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut u16_0: u16 = 11u16;
    let mut i32_3: i32 = 102i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut i8_0: i8 = -68i8;
    let mut i8_1: i8 = -40i8;
    let mut i8_2: i8 = 113i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_6, utcoffset_0);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_3);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_4: i32 = -20i32;
    let mut i8_3: i8 = -107i8;
    let mut i8_4: i8 = -31i8;
    let mut i8_5: i8 = -39i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_5: i32 = -24i32;
    let mut i64_6: i64 = -16i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_5);
    let mut i128_0: i128 = 105i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut i32_6: i32 = -20i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_13);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_1);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut i32_7: i32 = -62i32;
    let mut i64_7: i64 = -73i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_7);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_4, month_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_4);
    let mut primitivedatetime_6_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_6;
    std::ops::AddAssign::add_assign(primitivedatetime_6_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6347() {
    rusty_monitor::set_test_id(6347);
    let mut f32_0: f32 = 22.007516f32;
    let mut i32_0: i32 = -190i32;
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = 149i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = -84i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_1: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = -199i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut u16_0: u16 = 69u16;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 1u8;
    let mut i64_4: i64 = -192i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i8_0: i8 = -85i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -77i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut f32_1: f32 = 23.568211f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_2: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_9);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut f64_0: f64 = 146.854321f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = 19i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_10);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i32_3: i32 = 55i32;
    let mut i64_5: i64 = -113i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_8);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_4: i32 = 162i32;
    let mut i64_6: i64 = -122i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_4);
    let mut u8_6: u8 = crate::time::Time::second(time_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut duration_12_ref_0: &mut crate::duration::Duration = &mut duration_12;
    std::ops::SubAssign::sub_assign(duration_12_ref_0, duration_2);
    std::ops::DivAssign::div_assign(duration_0_ref_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2955() {
    rusty_monitor::set_test_id(2955);
    let mut i128_0: i128 = -108i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = -118.092880f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = 14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 64u32;
    let mut u8_3: u8 = 77u8;
    let mut u8_4: u8 = 34u8;
    let mut u8_5: u8 = 99u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_2: u32 = 95u32;
    let mut u8_6: u8 = 5u8;
    let mut u8_7: u8 = 62u8;
    let mut u8_8: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = 21i32;
    let mut i64_1: i64 = 153i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u16_0: u16 = 2u16;
    let mut i32_3: i32 = -53i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 69u16;
    let mut i32_4: i32 = 13i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4_ref_0: &crate::date::Date = &mut date_4;
    let mut i64_2: i64 = 156i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_5: i32 = -237i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut i8_3: i8 = 68i8;
    let mut i8_4: i8 = -104i8;
    let mut i8_5: i8 = -21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_3: i64 = -23i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -78i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_5: i64 = -130i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_10);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut u8_9: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    let mut time_2_ref_0: &mut crate::time::Time = &mut time_2;
    std::ops::SubAssign::sub_assign(time_2_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2563() {
    rusty_monitor::set_test_id(2563);
    let mut i64_0: i64 = 164i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = -12i8;
    let mut i8_1: i8 = 8i8;
    let mut i8_2: i8 = 79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = -39i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 17i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i64_2: i64 = -48i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut u16_0: u16 = 81u16;
    let mut i64_3: i64 = -190i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_3: i8 = 53i8;
    let mut i8_4: i8 = 91i8;
    let mut i8_5: i8 = -54i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -36i8;
    let mut i8_7: i8 = 28i8;
    let mut i8_8: i8 = 1i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 59i8;
    let mut i8_10: i8 = 37i8;
    let mut i8_11: i8 = -112i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_4: i64 = -5i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 61u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_5: i64 = -52i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i64_6: i64 = 154i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i32_1: i32 = -196i32;
    let mut i64_7: i64 = 60i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_1);
    let mut i64_8: i64 = -81i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    std::ops::DivAssign::div_assign(duration_7_ref_0, u16_0);
    let mut i32_2: i32 = crate::duration::Duration::subsec_nanoseconds(duration_4);
    std::ops::SubAssign::sub_assign(primitivedatetime_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5377() {
    rusty_monitor::set_test_id(5377);
    let mut u32_0: u32 = 60u32;
    let mut i128_0: i128 = -24i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i64_0: i64 = 149i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -84i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_0: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -199i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut u16_0: u16 = 69u16;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 1u8;
    let mut i64_3: i64 = -192i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i8_0: i8 = -85i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -77i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut f32_0: f32 = 23.568211f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_9);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut f64_0: f64 = 146.854321f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = 19i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_10);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i32_2: i32 = -39i32;
    let mut i32_3: i32 = 55i32;
    let mut i64_4: i64 = -113i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_8);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_4: i32 = 162i32;
    let mut i64_5: i64 = -122i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut u8_6: u8 = crate::time::Time::second(time_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut duration_13_ref_0: &mut crate::duration::Duration = &mut duration_13;
    std::ops::SubAssign::sub_assign(duration_13_ref_0, duration_2);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3340() {
    rusty_monitor::set_test_id(3340);
    let mut i8_0: i8 = -57i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 66i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -79i32;
    let mut i64_0: i64 = 43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 14u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 163i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_1: i32 = -205i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_2: i64 = 77i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut u32_1: u32 = 12u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 75u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    let mut i32_2: i32 = 2i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = 173i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = 63i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -75i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_8);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = -180i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut i32_3: i32 = 48i32;
    let mut i64_7: i64 = 21i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_5, duration_11);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut u16_0: u16 = 11u16;
    let mut i32_4: i32 = 102i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_3};
    let mut i8_3: i8 = -68i8;
    let mut i8_4: i8 = -40i8;
    let mut i8_5: i8 = 113i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_7, utcoffset_1);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_3);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_5: i32 = -20i32;
    let mut i8_6: i8 = -107i8;
    let mut i8_7: i8 = -31i8;
    let mut i8_8: i8 = -39i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_6: i32 = -24i32;
    let mut i64_8: i64 = -16i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_6);
    let mut i128_0: i128 = 105i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut i32_7: i32 = -20i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_7};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_14);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_9, utcoffset_2);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_10);
    let mut i32_8: i32 = -62i32;
    let mut i64_9: i64 = -73i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_8);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_5, month_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    std::ops::AddAssign::add_assign(time_1_ref_0, duration_5);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1239() {
    rusty_monitor::set_test_id(1239);
    let mut f32_0: f32 = -10.541270f32;
    let mut i64_0: i64 = -79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -89i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut u32_0: u32 = 54u32;
    let mut i64_2: i64 = 14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u32_1: u32 = 58u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut u32_2: u32 = 64u32;
    let mut u8_3: u8 = 77u8;
    let mut u8_4: u8 = 34u8;
    let mut u8_5: u8 = 99u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i32_0: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_3: u32 = 95u32;
    let mut u8_6: u8 = 5u8;
    let mut u8_7: u8 = 62u8;
    let mut u8_8: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut i32_2: i32 = 21i32;
    let mut i64_3: i64 = 153i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut u16_0: u16 = 2u16;
    let mut i32_3: i32 = -53i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 69u16;
    let mut i32_4: i32 = 13i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4_ref_0: &crate::date::Date = &mut date_4;
    let mut i64_4: i64 = 156i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i32_5: i32 = -237i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut i8_3: i8 = 68i8;
    let mut i8_4: i8 = -104i8;
    let mut i8_5: i8 = -21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_5: i64 = -23i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = -78i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_7: i64 = -130i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_10);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    std::ops::DivAssign::div_assign(duration_9_ref_0, u32_0);
    std::ops::MulAssign::mul_assign(duration_2_ref_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1890() {
    rusty_monitor::set_test_id(1890);
    let mut u8_0: u8 = 76u8;
    let mut i64_0: i64 = -190i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 53i8;
    let mut i8_1: i8 = 91i8;
    let mut i8_2: i8 = -54i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -36i8;
    let mut i8_4: i8 = 28i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 59i8;
    let mut i8_7: i8 = 37i8;
    let mut i8_8: i8 = -112i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = -5i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u32_0: u32 = 66u32;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 12u8;
    let mut u8_3: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i64_2: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = 154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_0: i32 = -196i32;
    let mut i64_4: i64 = 60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut i64_5: i64 = -81i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 70u32;
    let mut u8_4: u8 = 29u8;
    let mut u8_5: u8 = 78u8;
    let mut u8_6: u8 = 72u8;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::DivAssign::div_assign(duration_2_ref_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8080() {
    rusty_monitor::set_test_id(8080);
    let mut i64_0: i64 = -11i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u16_0: u16 = 69u16;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut i64_1: i64 = -192i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i8_0: i8 = -85i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -77i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 57u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut f32_0: f32 = 23.568211f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut f64_0: f64 = 146.854321f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = 19i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i32_1: i32 = -39i32;
    let mut i32_2: i32 = 55i32;
    let mut i64_2: i64 = -113i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: std::time::Instant = crate::instant::Instant::into_inner(instant_5);
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 162i32;
    let mut i64_3: i64 = -122i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut u8_6: u8 = crate::time::Time::second(time_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut instant_7_ref_0: &mut crate::instant::Instant = &mut instant_7;
    std::ops::AddAssign::add_assign(instant_7_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8046() {
    rusty_monitor::set_test_id(8046);
    let mut i32_0: i32 = 160i32;
    let mut i64_0: i64 = 149i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_1: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -199i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u16_0: u16 = 69u16;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 1u8;
    let mut i64_3: i64 = -192i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i8_0: i8 = -85i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -77i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut f32_0: f32 = 23.568211f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_8);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_7);
    let mut f64_0: f64 = 146.854321f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_3: i8 = -48i8;
    let mut i8_4: i8 = 19i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_9);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i32_3: i32 = -39i32;
    let mut i32_4: i32 = 55i32;
    let mut i64_4: i64 = -113i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_3);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_8);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_5: i32 = 162i32;
    let mut i64_5: i64 = -122i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_5);
    let mut u8_6: u8 = crate::time::Time::second(time_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut duration_12_ref_0: &mut crate::duration::Duration = &mut duration_12;
    std::ops::SubAssign::sub_assign(duration_12_ref_0, duration_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4451() {
    rusty_monitor::set_test_id(4451);
    let mut i16_0: i16 = 0i16;
    let mut f32_0: f32 = 12.612260f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_0: i64 = 173i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 63i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -75i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_3: i64 = -148i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut i64_4: i64 = -180i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i32_1: i32 = 48i32;
    let mut i64_5: i64 = 21i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_1);
    let mut u16_0: u16 = 11u16;
    let mut i32_2: i32 = 102i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i8_0: i8 = -68i8;
    let mut i8_1: i8 = -40i8;
    let mut i8_2: i8 = 113i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_3: i32 = -20i32;
    let mut i8_3: i8 = -107i8;
    let mut i8_4: i8 = -31i8;
    let mut i8_5: i8 = -39i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_4: i32 = -24i32;
    let mut i64_6: i64 = -16i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_4);
    let mut i128_0: i128 = 105i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut i32_5: i32 = -20i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_10);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_7, utcoffset_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut i32_6: i32 = -62i32;
    let mut i64_7: i64 = -73i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_3, month_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_1);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::DivAssign::div_assign(duration_5_ref_0, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6252() {
    rusty_monitor::set_test_id(6252);
    let mut i64_0: i64 = 149i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_0: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -199i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u16_0: u16 = 69u16;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 1u8;
    let mut i64_3: i64 = -192i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut u32_0: u32 = 0u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut f32_0: f32 = 23.568211f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_8);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut f64_0: f64 = 146.854321f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_2: i32 = -39i32;
    let mut i32_3: i32 = 55i32;
    let mut i64_4: i64 = -113i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: std::time::Instant = crate::instant::Instant::into_inner(instant_8);
    let mut instant_10: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_11: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_12: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_4: i32 = 162i32;
    let mut i64_5: i64 = -122i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut duration_12_ref_0: &mut crate::duration::Duration = &mut duration_12;
    std::ops::SubAssign::sub_assign(duration_12_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_120() {
    rusty_monitor::set_test_id(120);
    let mut i64_0: i64 = 31i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_1: i64 = -21i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u16_0: u16 = 96u16;
    let mut i32_0: i32 = -8i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut u16_1: u16 = 48u16;
    let mut i32_1: i32 = 169i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i8_0: i8 = 43i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = -16i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 89u8;
    let mut u8_2: u8 = 33u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_3: i8 = 56i8;
    let mut i8_4: i8 = 123i8;
    let mut i8_5: i8 = 27i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -47i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 17u32;
    let mut u8_3: u8 = 84u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 83u8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_1);
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    std::ops::SubAssign::sub_assign(instant_4_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4132() {
    rusty_monitor::set_test_id(4132);
    let mut i64_0: i64 = -213i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 14i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 64u32;
    let mut u8_3: u8 = 77u8;
    let mut u8_4: u8 = 34u8;
    let mut u8_5: u8 = 99u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_1: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_2: u32 = 95u32;
    let mut u8_6: u8 = 5u8;
    let mut u8_7: u8 = 62u8;
    let mut u8_8: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = 21i32;
    let mut i64_2: i64 = 153i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut u16_0: u16 = 2u16;
    let mut i32_3: i32 = -53i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 69u16;
    let mut i32_4: i32 = 13i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4_ref_0: &crate::date::Date = &mut date_4;
    let mut i64_3: i64 = 156i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i32_5: i32 = -237i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut i8_3: i8 = 68i8;
    let mut i8_4: i8 = -104i8;
    let mut i8_5: i8 = -21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = -78i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_5: i64 = -130i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_8);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_7);
    let mut time_3_ref_0: &mut crate::time::Time = &mut time_3;
    std::ops::SubAssign::sub_assign(time_3_ref_0, duration_1);
    panic!("From RustyUnit with love");
}
}