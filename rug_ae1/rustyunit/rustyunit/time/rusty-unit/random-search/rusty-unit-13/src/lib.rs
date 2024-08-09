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
fn rusty_test_2385() {
    rusty_monitor::set_test_id(2385);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut i32_0: i32 = -156i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut u16_0: u16 = 3u16;
    let mut i32_1: i32 = 189i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u16_1: u16 = 43u16;
    let mut i32_2: i32 = -24i32;
    let mut i32_3: i32 = -135i32;
    let mut i32_4: i32 = -264i32;
    let mut i64_0: i64 = -22i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_4);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::DivAssign::div_assign(duration_2_ref_0, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_2, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_0);
    std::ops::SubAssign::sub_assign(time_0_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2994() {
    rusty_monitor::set_test_id(2994);
    let mut i64_0: i64 = 81i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 43i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -59i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 82u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 96u16;
    let mut i32_0: i32 = 34i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    std::ops::SubAssign::sub_assign(offsetdatetime_0_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2308() {
    rusty_monitor::set_test_id(2308);
    let mut f32_0: f32 = 135.192089f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 62i32;
    let mut i64_0: i64 = -99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u16_0: u16 = 48u16;
    let mut i32_1: i32 = -39i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i32_2: i32 = 102i32;
    let mut i64_1: i64 = -58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i64_2: i64 = 132i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_3: i64 = -123i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -31i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut i16_0: i16 = 161i16;
    let mut i16_1: i16 = -116i16;
    let mut i64_5: i64 = -29i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = 116i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut u32_0: u32 = 40u32;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 25u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u16_1: u16 = 89u16;
    let mut i64_7: i64 = 228i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_1, duration_10);
    let mut month_0: month::Month = crate::month::Month::June;
    std::ops::SubAssign::sub_assign(instant_0_ref_0, duration_7);
    let mut i32_3: i32 = crate::duration::Duration::subsec_microseconds(duration_11);
    let mut i16_2: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    let mut u8_4: u8 = crate::primitive_date_time::PrimitiveDateTime::hour(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_868() {
    rusty_monitor::set_test_id(868);
    let mut i64_0: i64 = 142i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -89i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i64_2: i64 = 148i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut f32_0: f32 = 89.048924f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -71i32;
    let mut i64_3: i64 = -31i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut u16_0: u16 = 1u16;
    let mut i64_4: i64 = -121i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::DivAssign::div_assign(duration_8_ref_0, u16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_7, duration_4);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2123() {
    rusty_monitor::set_test_id(2123);
    let mut i8_0: i8 = -114i8;
    let mut i8_1: i8 = -21i8;
    let mut i8_2: i8 = -7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = 13i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 24u32;
    let mut u8_0: u8 = 91u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 48u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut i16_0: i16 = 93i16;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut i32_0: i32 = 17i32;
    let mut i64_0: i64 = 55i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut bool_0: bool = crate::util::is_leap_year(i32_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::DivAssign::div_assign(duration_2_ref_0, i16_0);
    std::ops::SubAssign::sub_assign(time_0_ref_0, duration_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4215() {
    rusty_monitor::set_test_id(4215);
    let mut f64_0: f64 = 298.458006f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = -39i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_0: i32 = -6i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_1: i64 = 107i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u16_0: u16 = 12u16;
    let mut i32_1: i32 = -74i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    std::ops::AddAssign::add_assign(date_1_ref_0, duration_3);
    let mut u8_0: u8 = crate::time::Time::hour(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3148() {
    rusty_monitor::set_test_id(3148);
    let mut i64_0: i64 = 109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_0: u32 = 61u32;
    let mut u8_0: u8 = 14u8;
    let mut u8_1: u8 = 17u8;
    let mut u8_2: u8 = 38u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 0i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -53i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut f64_0: f64 = 95.001564f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_1: i128 = -5i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 32u16;
    let mut i32_1: i32 = -30i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut u8_3: u8 = 41u8;
    let mut i32_2: i32 = -74i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_3);
    let mut month_0: month::Month = crate::month::Month::February;
    std::ops::SubAssign::sub_assign(primitivedatetime_2_ref_0, duration_2);
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3037() {
    rusty_monitor::set_test_id(3037);
    let mut i8_0: i8 = -29i8;
    let mut i8_1: i8 = -3i8;
    let mut i8_2: i8 = 70i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i64_0: i64 = 77i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_3: i8 = 68i8;
    let mut i8_4: i8 = 91i8;
    let mut i8_5: i8 = 39i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = -56i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 31u16;
    let mut i32_1: i32 = 54i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
    let mut i32_2: i32 = 6i32;
    let mut i64_1: i64 = 63i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = -48i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::SubAssign::sub_assign(duration_3_ref_0, duration_2);
    std::ops::AddAssign::add_assign(time_1_ref_0, duration_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1066() {
    rusty_monitor::set_test_id(1066);
    let mut i32_0: i32 = 13i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_1: i32 = 20i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut f32_0: f32 = -145.662766f32;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 83i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_2: i32 = -13i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut bool_0: bool = false;
    let mut i64_1: i64 = -170i64;
    let mut i64_2: i64 = -124i64;
    let mut i64_3: i64 = -42i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i128_0: i128 = 1i128;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_1);
    let mut month_1: month::Month = crate::month::Month::January;
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_0);
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::MulAssign::mul_assign(duration_1_ref_0, f32_0);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3357() {
    rusty_monitor::set_test_id(3357);
    let mut u16_0: u16 = 45u16;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 30u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i8_0: i8 = 15i8;
    let mut i8_1: i8 = 37i8;
    let mut i8_2: i8 = -24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u16_1: u16 = 62u16;
    let mut i64_0: i64 = 200i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    std::ops::MulAssign::mul_assign(duration_0_ref_0, u16_1);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4340() {
    rusty_monitor::set_test_id(4340);
    let mut i128_0: i128 = 109i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u16_0: u16 = 29u16;
    let mut i32_0: i32 = -125i32;
    let mut i8_0: i8 = 49i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i128_1: i128 = -20i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i8_1: i8 = -127i8;
    let mut f32_0: f32 = 83.535020f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, i8_1);
    std::ops::SubAssign::sub_assign(duration_3_ref_0, duration_2);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i8_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i8_2: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_691() {
    rusty_monitor::set_test_id(691);
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut i32_0: i32 = 40i32;
    let mut i64_1: i64 = 36i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_2: i64 = -53i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_3: i64 = -21i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut f64_0: f64 = 164.771744f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 67i32;
    let mut i64_4: i64 = 44i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_1);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i8_0: i8 = -34i8;
    let mut i64_5: i64 = -20i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::MulAssign::mul_assign(duration_8_ref_0, i8_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i32_2: i32 = crate::duration::Duration::subsec_microseconds(duration_7);
    let mut f64_1: f64 = crate::duration::Duration::as_seconds_f64(duration_6);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_5, duration_2);
    std::ops::AddAssign::add_assign(instant_0_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_463() {
    rusty_monitor::set_test_id(463);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u16_0: u16 = 82u16;
    let mut i32_0: i32 = -110i32;
    let mut i32_1: i32 = -23i32;
    let mut i64_0: i64 = -169i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut f64_0: f64 = 78.936254f64;
    let mut i64_1: i64 = -81i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i32_2: i32 = 107i32;
    let mut i32_3: i32 = -2i32;
    let mut i64_2: i64 = 27i64;
    std::ops::MulAssign::mul_assign(duration_1_ref_0, f64_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_days(duration_0);
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2183() {
    rusty_monitor::set_test_id(2183);
    let mut i32_0: i32 = 52i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_1: i32 = 111i32;
    let mut i64_0: i64 = 98i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i64_1: i64 = -176i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 126i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    std::ops::SubAssign::sub_assign(primitivedatetime_1_ref_0, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2462() {
    rusty_monitor::set_test_id(2462);
    let mut u16_0: u16 = 94u16;
    let mut i64_0: i64 = -41i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i32_0: i32 = -55i32;
    let mut i64_1: i64 = 44i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 35i32;
    let mut f32_0: f32 = -274.674696f32;
    let mut i32_2: i32 = 61i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i16_0: i16 = 5i16;
    let mut i64_2: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i128_0: i128 = 96i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 75i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, f32_0);
    let mut i32_3: i32 = crate::duration::Duration::subsec_nanoseconds(duration_1);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3839() {
    rusty_monitor::set_test_id(3839);
    let mut f32_0: f32 = 119.164122f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 51u32;
    let mut u8_3: u8 = 50u8;
    let mut u8_4: u8 = 25u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 83i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut u16_0: u16 = 31u16;
    let mut i64_0: i64 = -64i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_1: i32 = -70i32;
    let mut i64_1: i64 = 247i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_2: i64 = crate::duration::Duration::whole_days(duration_4);
    std::ops::DivAssign::div_assign(duration_3_ref_0, u16_0);
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1782() {
    rusty_monitor::set_test_id(1782);
    let mut i32_0: i32 = 8i32;
    let mut i64_0: i64 = 19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = -18i32;
    let mut i64_2: i64 = -48i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 53u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i32_2: i32 = 3i32;
    let mut i128_0: i128 = 72i128;
    let mut i8_0: i8 = -35i8;
    let mut i64_3: i64 = 27i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i64_4: i64 = -9i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i32_3: i32 = -153i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_4);
    std::ops::MulAssign::mul_assign(duration_3_ref_0, i8_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = crate::util::days_in_year(i32_2);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1424() {
    rusty_monitor::set_test_id(1424);
    let mut i32_0: i32 = -174i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = -61i32;
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut u32_0: u32 = 66u32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 163i32;
    let mut i64_1: i64 = 20i64;
    let mut i16_0: i16 = -35i16;
    let mut i64_2: i64 = -41i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut dateadjustment_1: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u32_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_seconds(duration_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3290() {
    rusty_monitor::set_test_id(3290);
    let mut i8_0: i8 = -35i8;
    let mut i8_1: i8 = 22i8;
    let mut i8_2: i8 = 85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 52u16;
    let mut i32_0: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i32_1: i32 = 56i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2338() {
    rusty_monitor::set_test_id(2338);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 60i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut f32_0: f32 = -25.574782f32;
    let mut i32_1: i32 = 60i32;
    let mut i64_1: i64 = 116i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut i64_2: i64 = 35i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i64_3: i64 = -47i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_2: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    std::ops::DivAssign::div_assign(duration_6_ref_0, f32_0);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1318() {
    rusty_monitor::set_test_id(1318);
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 63u8;
    let mut u8_1: u8 = 93u8;
    let mut u8_2: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 63i32;
    let mut i64_1: i64 = -34i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut u16_0: u16 = 49u16;
    let mut i32_1: i32 = 114i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i64_2: i64 = 13i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_2: i32 = 63i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2_ref_0: &mut crate::date::Date = &mut date_2;
    std::ops::SubAssign::sub_assign(date_2_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4849() {
    rusty_monitor::set_test_id(4849);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i128_0: i128 = 18i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut f64_0: f64 = -0.519514f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = 220i32;
    let mut i64_0: i64 = 9i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = -106i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    std::ops::AddAssign::add_assign(duration_2_ref_0, duration_1);
    let mut u32_0: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_949() {
    rusty_monitor::set_test_id(949);
    let mut i128_0: i128 = -166i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_0: u32 = 24u32;
    let mut u8_0: u8 = 18u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 26u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 136i32;
    let mut i64_0: i64 = 170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = 232i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = 104i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut i16_0: i16 = -32i16;
    let mut i64_3: i64 = 45i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, i16_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_399() {
    rusty_monitor::set_test_id(399);
    let mut u16_0: u16 = 48u16;
    let mut i32_0: i32 = -69i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_0: i64 = 16i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_1: i32 = -179i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -104i8;
    let mut i8_1: i8 = 93i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -101i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 23i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i32_2: i32 = -17i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_3: i64 = 4i64;
    let mut f64_0: f64 = 51.854659f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_3: i32 = -23i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_5_ref_0: &mut crate::date::Date = &mut date_5;
    std::ops::AddAssign::add_assign(date_5_ref_0, duration_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_2);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_1);
    let mut tuple_1: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3773() {
    rusty_monitor::set_test_id(3773);
    let mut i32_0: i32 = -147i32;
    let mut f32_0: f32 = 2.166174f32;
    let mut f32_1: f32 = 55.613667f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 46u16;
    let mut i32_1: i32 = 80i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
    let mut i8_0: i8 = 48i8;
    let mut i32_2: i32 = -22i32;
    let mut i64_0: i64 = 36i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i8_0);
    std::ops::SubAssign::sub_assign(date_0_ref_0, duration_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1992() {
    rusty_monitor::set_test_id(1992);
    let mut u8_0: u8 = 56u8;
    let mut i64_0: i64 = 20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i8_0: i8 = -37i8;
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = -13i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut f64_0: f64 = 15.559318f64;
    let mut i64_3: i64 = -88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_1: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::DivAssign::div_assign(duration_2_ref_0, i8_0);
    std::ops::DivAssign::div_assign(duration_0_ref_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1451() {
    rusty_monitor::set_test_id(1451);
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = -39i32;
    let mut i64_0: i64 = 105i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_1: i32 = 40i32;
    let mut i64_1: i64 = -71i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -77i8;
    let mut i8_1: i8 = -45i8;
    let mut i8_2: i8 = -50i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 218.304740f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut u16_1: u16 = 38u16;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    std::ops::DivAssign::div_assign(duration_5_ref_0, u16_1);
    std::ops::AddAssign::add_assign(instant_0_ref_0, duration_4);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_0, duration_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2799() {
    rusty_monitor::set_test_id(2799);
    let mut i32_0: i32 = 11i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 18u16;
    let mut i32_1: i32 = 14i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 95u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 29u16;
    let mut i32_2: i32 = 117i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut i64_0: i64 = 179i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut f64_0: f64 = -34.617599f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::SubAssign::sub_assign(duration_2_ref_0, duration_1);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_1);
    let mut u8_3: u8 = crate::date::Date::sunday_based_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3804() {
    rusty_monitor::set_test_id(3804);
    let mut u16_0: u16 = 19u16;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 66u8;
    let mut i128_0: i128 = -9i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = -20i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut f64_0: f64 = 54.098367f64;
    let mut i32_0: i32 = 13i32;
    let mut i64_1: i64 = -63i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    std::ops::DivAssign::div_assign(duration_3_ref_0, f64_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3700() {
    rusty_monitor::set_test_id(3700);
    let mut i8_0: i8 = 37i8;
    let mut i8_1: i8 = -61i8;
    let mut i8_2: i8 = -7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 84u8;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i64_0: i64 = 52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i16_0: i16 = 64i16;
    let mut i64_1: i64 = 73i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, i16_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4630() {
    rusty_monitor::set_test_id(4630);
    let mut i64_0: i64 = -98i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_0: u32 = 56u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 70u8;
    let mut u8_2: u8 = 88u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 9u16;
    let mut i32_0: i32 = -50i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = -105i8;
    let mut f32_0: f32 = -55.336401f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u16_1: u16 = 98u16;
    let mut i32_1: i32 = -26i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_1);
    std::ops::DivAssign::div_assign(duration_1_ref_0, i8_0);
    let mut i8_1: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1786() {
    rusty_monitor::set_test_id(1786);
    let mut i64_0: i64 = 95i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = -144i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = 135i32;
    let mut i64_2: i64 = 193i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 13i8;
    let mut i8_2: i8 = -55i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    std::ops::SubAssign::sub_assign(time_0_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4817() {
    rusty_monitor::set_test_id(4817);
    let mut i64_0: i64 = -207i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 13i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = 0i32;
    let mut i64_2: i64 = -152i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut u16_1: u16 = 35u16;
    let mut u8_0: u8 = 49u8;
    let mut i64_3: i64 = 208i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut u16_2: u16 = 73u16;
    let mut i32_1: i32 = -32i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_2);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    std::ops::SubAssign::sub_assign(instant_0_ref_0, duration_4);
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1418() {
    rusty_monitor::set_test_id(1418);
    let mut i8_0: i8 = -15i8;
    let mut i8_1: i8 = -63i8;
    let mut i8_2: i8 = 56i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -108i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut i64_1: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i8_3: i8 = -85i8;
    let mut i8_4: i8 = -54i8;
    let mut i8_5: i8 = 22i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -81i8;
    let mut i8_7: i8 = 82i8;
    let mut i8_8: i8 = 51i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_0: i32 = -88i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 3i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut u8_0: u8 = 9u8;
    std::ops::AddAssign::add_assign(offsetdatetime_1_ref_0, duration_2);
    std::ops::SubAssign::sub_assign(instant_0_ref_0, duration_1);
    let mut i8_9: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut i64_0: i64 = -6i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = -54i8;
    let mut i8_1: i8 = -42i8;
    let mut i8_2: i8 = 38i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 45i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i128_0: i128 = -105i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 25u16;
    let mut i64_1: i64 = -29i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_2: i32 = 12i32;
    let mut i64_2: i64 = 198i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u16_1: u16 = 46u16;
    let mut i32_3: i32 = 56i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u16_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3896() {
    rusty_monitor::set_test_id(3896);
    let mut i64_0: i64 = 43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 64i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u16_0: u16 = 71u16;
    let mut i32_1: i32 = -15i32;
    let mut i32_2: i32 = -34i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut u32_0: u32 = 7u32;
    let mut i16_0: i16 = -202i16;
    let mut i64_1: i64 = 178i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = -24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, u32_0);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_2);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_0);
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2341() {
    rusty_monitor::set_test_id(2341);
    let mut i128_0: i128 = 58i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 57u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 34i8;
    let mut i8_1: i8 = 102i8;
    let mut i8_2: i8 = 78i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_1: u32 = 93u32;
    let mut u8_3: u8 = 58u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 97u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = -23i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    std::ops::AddAssign::add_assign(offsetdatetime_2_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_59() {
    rusty_monitor::set_test_id(59);
    let mut i32_0: i32 = -19i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i8_0: i8 = -27i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -188i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = -123i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut month_0: month::Month = crate::month::Month::December;
    let mut f64_0: f64 = -32.378367f64;
    let mut i128_0: i128 = 102i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_2: i32 = 25i32;
    let mut f64_1: f64 = -12.622852f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    std::ops::DivAssign::div_assign(duration_4_ref_0, f64_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::July;
    std::ops::AddAssign::add_assign(time_0_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2012() {
    rusty_monitor::set_test_id(2012);
    let mut i64_0: i64 = -93i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &mut crate::instant::Instant = &mut instant_0;
    let mut u16_0: u16 = 37u16;
    let mut i128_0: i128 = 92i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = -235i32;
    let mut i64_1: i64 = 115i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 8i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = -114i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i8_0: i8 = -26i8;
    let mut i8_1: i8 = 65i8;
    let mut i8_2: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_2: i32 = -117i32;
    let mut f64_0: f64 = 44.596489f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_2);
    let mut i8_3: i8 = 96i8;
    let mut i8_4: i8 = -41i8;
    let mut i8_5: i8 = 18i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 24i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    std::ops::AddAssign::add_assign(instant_0_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_243() {
    rusty_monitor::set_test_id(243);
    let mut u8_0: u8 = 39u8;
    let mut i8_0: i8 = -44i8;
    let mut i8_1: i8 = -105i8;
    let mut i8_2: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 78u32;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 82u8;
    let mut u8_3: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_0: i32 = -72i32;
    let mut i64_0: i64 = -89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 199i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_2: i64 = -198i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i64_3: i64 = 145i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = 212i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i8_3: i8 = -8i8;
    let mut i8_4: i8 = -17i8;
    let mut i8_5: i8 = 8i8;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u8_0);
    panic!("From RustyUnit with love");
}
}