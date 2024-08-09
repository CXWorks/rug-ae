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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut i64_0: i64 = -34i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 40u32;
    let mut u8_0: u8 = 21u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 36u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 43u32;
    let mut u8_3: u8 = 7u8;
    let mut u8_4: u8 = 19u8;
    let mut u8_5: u8 = 75u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 59i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i32_0: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut u32_2: u32 = 48u32;
    let mut u8_6: u8 = 19u8;
    let mut u8_7: u8 = 67u8;
    let mut u8_8: u8 = 93u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_1: i32 = 89i32;
    let mut i32_2: i32 = 142i32;
    let mut i64_3: i64 = -242i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::DivAssign::div_assign(duration_5_ref_0, i32_1);
    let mut u32_3: u32 = crate::time::Time::microsecond(time_2);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4686() {
    rusty_monitor::set_test_id(4686);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 34u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -57i32;
    let mut i64_0: i64 = 127i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u16_0: u16 = 4u16;
    let mut i32_1: i32 = -7i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i64_1: i64 = -96i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_1: u32 = 85u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -129i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_2: i32 = -14i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_2);
    let mut i32_3: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i32_4: i32 = 106i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u32_2: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, u32_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_4);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_2);
    let mut i32_5: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8102() {
    rusty_monitor::set_test_id(8102);
    let mut i64_0: i64 = 44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    let mut i64_1: i64 = 128i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 41i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut u8_0: u8 = 87u8;
    let mut i32_1: i32 = -156i32;
    let mut i32_2: i32 = 80i32;
    let mut i64_2: i64 = -43i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut i64_3: i64 = -90i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 27i32;
    let mut i64_4: i64 = 8i64;
    let mut i32_4: i32 = 101i32;
    let mut i64_5: i64 = 41i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_4);
    let mut u16_1: u16 = 66u16;
    let mut i32_5: i32 = -131i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_7);
    let mut i32_6: i32 = 37i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    std::ops::SubAssign::sub_assign(instant_4_ref_0, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_8, duration_1);
    std::ops::SubAssign::sub_assign(offsetdatetime_0_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6672() {
    rusty_monitor::set_test_id(6672);
    let mut f64_0: f64 = 134.213577f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 122i8;
    let mut i8_2: i8 = 8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -14i8;
    let mut i8_4: i8 = 80i8;
    let mut i8_5: i8 = -73i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 64i8;
    let mut i8_7: i8 = -51i8;
    let mut i8_8: i8 = 4i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_0: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_2);
    let mut i8_9: i8 = 47i8;
    let mut i8_10: i8 = -7i8;
    let mut i8_11: i8 = -5i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 47u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -177i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut f32_0: f32 = -74.222949f32;
    let mut i64_0: i64 = -112i64;
    let mut i8_12: i8 = 4i8;
    let mut i8_13: i8 = -27i8;
    let mut i8_14: i8 = 44i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f64_1: f64 = 8.033375f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_2: i32 = -16i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 60i32;
    let mut i64_1: i64 = -45i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 27u16;
    let mut i32_4: i32 = 11i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut i64_2: i64 = 113i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_5: i32 = 19i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_7);
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut date_5_ref_0: &mut crate::date::Date = &mut date_5;
    std::ops::SubAssign::sub_assign(date_5_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8257() {
    rusty_monitor::set_test_id(8257);
    let mut i64_0: i64 = -96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -129i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = -9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i32_1: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_2: i32 = 106i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1685() {
    rusty_monitor::set_test_id(1685);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i64_0: i64 = -96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -129i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_2: i32 = 106i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7671() {
    rusty_monitor::set_test_id(7671);
    let mut i64_0: i64 = 74i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = 117i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -13i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i64_3: i64 = -96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_4: i64 = -129i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_0: i32 = -14i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_5);
    let mut i32_1: i32 = 96i32;
    let mut i64_5: i64 = -121i64;
    let mut i64_6: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i64_7: i64 = -34i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut i32_2: i32 = 106i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 17u32;
    let mut i64_8: i64 = -17i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    std::ops::MulAssign::mul_assign(duration_9_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_4);
    std::ops::AddAssign::add_assign(duration_8_ref_0, duration_7);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_1);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_2);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_4);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3562() {
    rusty_monitor::set_test_id(3562);
    let mut f64_0: f64 = 23.661337f64;
    let mut i32_0: i32 = -7i32;
    let mut i64_0: i64 = -173i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 21u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 79i8;
    let mut i8_1: i8 = -36i8;
    let mut i8_2: i8 = 101i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 27i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_2: i32 = 114i32;
    let mut i8_3: i8 = 115i8;
    let mut i8_4: i8 = -37i8;
    let mut i8_5: i8 = 57i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 119i8;
    let mut i8_7: i8 = 11i8;
    let mut i8_8: i8 = 122i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 60u32;
    let mut u8_3: u8 = 20u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 92u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_2: u32 = 17u32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u32_2);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    std::ops::DivAssign::div_assign(duration_0_ref_0, f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4203() {
    rusty_monitor::set_test_id(4203);
    let mut f32_0: f32 = 71.713671f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    let mut i64_0: i64 = -28i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -96i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -129i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = -16i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut i32_1: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i32_2: i32 = 106i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    std::ops::SubAssign::sub_assign(offsetdatetime_0_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6692() {
    rusty_monitor::set_test_id(6692);
    let mut i64_0: i64 = -42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 27i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut u8_0: u8 = 96u8;
    let mut i8_0: i8 = 7i8;
    let mut i8_1: i8 = -114i8;
    let mut i8_2: i8 = -2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 63u32;
    let mut u8_1: u8 = 66u8;
    let mut u8_2: u8 = 57u8;
    let mut u8_3: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u16_0: u16 = 47u16;
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i32_1: i32 = -66i32;
    let mut i64_2: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 141i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_2: i32 = -31i32;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_3: i32 = -75i32;
    let mut i64_4: i64 = -19i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut i32_4: i32 = 180i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = -36i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_1);
    let mut bool_0: bool = crate::util::is_leap_year(i32_2);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_4);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_3, i32_1);
    let mut duration_6: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, u8_0);
    std::ops::SubAssign::sub_assign(duration_2_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6631() {
    rusty_monitor::set_test_id(6631);
    let mut u16_0: u16 = 70u16;
    let mut f32_0: f32 = 60.944504f32;
    let mut i64_0: i64 = -38i64;
    let mut i16_0: i16 = -102i16;
    let mut i32_0: i32 = 48i32;
    let mut i64_1: i64 = -51i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i32_1: i32 = 60i32;
    let mut i8_0: i8 = -28i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = -13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -45i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 27u16;
    let mut i32_2: i32 = 11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i64_3: i64 = 113i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 19i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_0_ref_0, i16_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5094() {
    rusty_monitor::set_test_id(5094);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -34i32;
    let mut i64_0: i64 = -139i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 18i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i16_0: i16 = 147i16;
    let mut f32_0: f32 = -145.588208f32;
    let mut i64_1: i64 = -160i64;
    let mut i64_2: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i32_2: i32 = 60i32;
    let mut i64_3: i64 = -45i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 27u16;
    let mut i32_3: i32 = 11i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i64_4: i64 = 113i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_4: i32 = 19i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_4);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_1_ref_0, i16_0);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2516() {
    rusty_monitor::set_test_id(2516);
    let mut i32_0: i32 = -111i32;
    let mut i64_0: i64 = -13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 54i8;
    let mut i8_2: i8 = -11i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 7i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_0: u32 = 81u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 38u8;
    let mut u8_2: u8 = 83u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 79u16;
    let mut i32_1: i32 = 12i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut u16_1: u16 = 65u16;
    let mut i64_2: i64 = -107i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = -91i32;
    let mut i64_3: i64 = -8i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_3: i8 = -73i8;
    let mut i8_4: i8 = 68i8;
    let mut i8_5: i8 = -47i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = 113i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut u16_2: u16 = 95u16;
    let mut i32_3: i32 = 4i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_1};
    let mut i64_5: i64 = -53i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = 36i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i32_4: i32 = 108i32;
    let mut i64_7: i64 = -35i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_4);
    let mut i32_5: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_2);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    std::ops::MulAssign::mul_assign(duration_9_ref_0, u16_1);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5190() {
    rusty_monitor::set_test_id(5190);
    let mut i64_0: i64 = 103i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut i64_2: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i8_0: i8 = -70i8;
    let mut i8_1: i8 = -17i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -54i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_1: i32 = -118i32;
    let mut i64_3: i64 = 147i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f32_0: f32 = -76.468316f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_4: i64 = -54i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i32_2: i32 = 2i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    std::ops::SubAssign::sub_assign(date_1_ref_0, duration_3);
    std::ops::AddAssign::add_assign(time_0_ref_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3824() {
    rusty_monitor::set_test_id(3824);
    let mut i32_0: i32 = -48i32;
    let mut i64_0: i64 = 13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u16_0: u16 = 66u16;
    let mut i32_1: i32 = 24i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
    let mut u8_0: u8 = 49u8;
    let mut i64_1: i64 = -4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut u16_1: u16 = 20u16;
    let mut i32_2: i32 = -117i32;
    let mut i8_0: i8 = -6i8;
    let mut i8_1: i8 = 97i8;
    let mut i8_2: i8 = -93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 73i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut u32_0: u32 = 48u32;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 72u8;
    let mut u8_3: u8 = 73u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u16_2: u16 = 22u16;
    let mut i32_3: i32 = 144i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_0};
    let mut i8_3: i8 = -123i8;
    let mut i8_4: i8 = 105i8;
    let mut i8_5: i8 = -27i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_4: i32 = 15i32;
    let mut i64_3: i64 = -99i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_2, u16_1);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u8_0);
    std::ops::SubAssign::sub_assign(time_0_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8096() {
    rusty_monitor::set_test_id(8096);
    let mut i64_0: i64 = 4i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = -71i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 19i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i64_1: i64 = -96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -129i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_2: i32 = -14i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_1);
    let mut i32_3: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_4: i32 = 106i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u32_1: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_4);
    std::ops::AddAssign::add_assign(duration_4_ref_0, duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    std::ops::AddAssign::add_assign(primitivedatetime_1_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_498() {
    rusty_monitor::set_test_id(498);
    let mut u32_0: u32 = 77u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -21i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = 85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut i64_1: i64 = -3i64;
    let mut i64_2: i64 = 17i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = -96i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut u32_1: u32 = 85u32;
    let mut u8_3: u8 = 6u8;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_5: i64 = -129i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i32_1: i32 = -14i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_3);
    let mut i32_2: i32 = 96i32;
    let mut i64_6: i64 = -121i64;
    let mut i64_7: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut i64_8: i64 = -34i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i32_3: i32 = 106i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_2: u32 = 17u32;
    let mut i64_9: i64 = -17i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    std::ops::MulAssign::mul_assign(duration_7_ref_0, u32_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_4);
    std::ops::AddAssign::add_assign(duration_6_ref_0, duration_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_2);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5366() {
    rusty_monitor::set_test_id(5366);
    let mut f32_0: f32 = 21.303355f32;
    let mut i64_0: i64 = -39i64;
    let mut i16_0: i16 = -102i16;
    let mut i32_0: i32 = 48i32;
    let mut i64_1: i64 = -51i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i32_1: i32 = 60i32;
    let mut i64_2: i64 = -45i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 27u16;
    let mut i32_2: i32 = 11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_3: i64 = 113i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 19i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    std::ops::DivAssign::div_assign(duration_0_ref_0, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2345() {
    rusty_monitor::set_test_id(2345);
    let mut i64_0: i64 = 13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i128_0: i128 = 181i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -21i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_1: i32 = -14i32;
    let mut i64_1: i64 = -215i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut i32_2: i32 = 41i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut date_2_ref_0: &mut crate::date::Date = &mut date_2;
    let mut i64_2: i64 = -96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = -129i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_3: i32 = -14i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_5);
    let mut i32_4: i32 = 96i32;
    let mut i64_4: i64 = -121i64;
    let mut i64_5: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = -34i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut i32_5: i32 = 106i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut u32_1: u32 = 17u32;
    let mut i64_7: i64 = -17i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    std::ops::MulAssign::mul_assign(duration_9_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_5);
    std::ops::AddAssign::add_assign(duration_8_ref_0, duration_7);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_3);
    std::ops::AddAssign::add_assign(date_2_ref_0, duration_3);
    std::ops::SubAssign::sub_assign(primitivedatetime_1_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3118() {
    rusty_monitor::set_test_id(3118);
    let mut i32_0: i32 = -233i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i32_1: i32 = 175i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i32_2: i32 = -16i32;
    let mut i64_0: i64 = -76i64;
    let mut i64_1: i64 = 301i64;
    let mut i32_3: i32 = 130i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_2: i64 = 5i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_4: i32 = -62i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 87u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 130i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_3: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 17u32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    std::ops::MulAssign::mul_assign(duration_1_ref_0, u32_1);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7981() {
    rusty_monitor::set_test_id(7981);
    let mut f32_0: f32 = 21.051641f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 41i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut u8_0: u8 = 87u8;
    let mut i32_1: i32 = -156i32;
    let mut i32_2: i32 = 80i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_1: i64 = -90i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 27i32;
    let mut i64_2: i64 = 8i64;
    let mut i32_4: i32 = 101i32;
    let mut i64_3: i64 = 41i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_4);
    let mut u16_1: u16 = 66u16;
    let mut i32_5: i32 = -131i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_5);
    let mut i32_6: i32 = 37i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = crate::time::Time::nanosecond(time_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut instant_4_ref_0: &mut crate::instant::Instant = &mut instant_4;
    std::ops::AddAssign::add_assign(instant_4_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2024() {
    rusty_monitor::set_test_id(2024);
    let mut i64_0: i64 = -67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = -81i8;
    let mut i8_1: i8 = -99i8;
    let mut i8_2: i8 = 79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i64_1: i64 = -117i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_0: i32 = 306i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut i64_2: i64 = -18i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_1: i32 = -127i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i8_3: i8 = 115i8;
    let mut i8_4: i8 = -37i8;
    let mut i8_5: i8 = 57i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 119i8;
    let mut i8_7: i8 = 11i8;
    let mut i8_8: i8 = 122i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 35u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 0i64;
    let mut i32_2: i32 = 96i32;
    let mut i64_4: i64 = -121i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 17u32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_1);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_6);
    std::ops::AddAssign::add_assign(offsetdatetime_1_ref_0, duration_1);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4054() {
    rusty_monitor::set_test_id(4054);
    let mut i64_0: i64 = -40i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -129i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_1: i32 = 106i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::SubAssign::sub_assign(duration_2_ref_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3162() {
    rusty_monitor::set_test_id(3162);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 68i32;
    let mut i32_1: i32 = -68i32;
    let mut i64_0: i64 = -173i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_1: u32 = 15u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 64u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = -68i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut u32_2: u32 = 37u32;
    let mut u8_6: u8 = 90u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 27u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_0: u16 = 85u16;
    let mut i32_2: i32 = 3i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_3: i32 = 4i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i8_0: i8 = -107i8;
    let mut i8_1: i8 = -34i8;
    let mut i8_2: i8 = -96i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_4: i32 = 159i32;
    let mut i64_2: i64 = 86i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_4);
    let mut i32_5: i32 = 121i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_5, utcoffset_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_3, primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut u32_3: u32 = 99u32;
    let mut u8_9: u8 = 25u8;
    let mut u8_10: u8 = 22u8;
    let mut u8_11: u8 = 29u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = 34i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut time_4_ref_0: &mut crate::time::Time = &mut time_4;
    std::ops::AddAssign::add_assign(time_4_ref_0, duration_2);
    let mut u8_12: u8 = crate::time::Time::second(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7652() {
    rusty_monitor::set_test_id(7652);
    let mut i64_0: i64 = -96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -129i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut u32_1: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_1);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5863() {
    rusty_monitor::set_test_id(5863);
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 69u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 41u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_1: u32 = 85u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = -132i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_2: i32 = 106i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_2: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    let mut tuple_2: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_weeks(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3119() {
    rusty_monitor::set_test_id(3119);
    let mut i32_0: i32 = 128i32;
    let mut i64_0: i64 = 9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = -96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_3: i32 = 106i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_1);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5367() {
    rusty_monitor::set_test_id(5367);
    let mut f32_0: f32 = 130.098828f32;
    let mut i64_0: i64 = 129i64;
    let mut i16_0: i16 = -102i16;
    let mut i32_0: i32 = 48i32;
    let mut i64_1: i64 = -51i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u32_0: u32 = 99u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 6u8;
    let mut i32_1: i32 = 60i32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 27u16;
    let mut i32_2: i32 = 11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_2: i64 = 113i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_3: i32 = 19i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    std::ops::DivAssign::div_assign(duration_0_ref_0, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1333() {
    rusty_monitor::set_test_id(1333);
    let mut f32_0: f32 = -146.476157f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -42i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
    let mut i64_0: i64 = -96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -129i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_1: i32 = -14i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut i32_2: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_3: i32 = 106i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_1: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_3);
    std::ops::AddAssign::add_assign(duration_4_ref_0, duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    std::ops::AddAssign::add_assign(date_0_ref_0, duration_0);
    let mut month_0: month::Month = crate::month::Month::September;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1006() {
    rusty_monitor::set_test_id(1006);
    let mut f32_0: f32 = -167.739398f32;
    let mut i64_0: i64 = -101i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut f64_0: f64 = -12.547120f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 84i8;
    let mut i8_1: i8 = -72i8;
    let mut i8_2: i8 = -93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 52u16;
    let mut i32_0: i32 = 131i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut i64_1: i64 = -96i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -129i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_1: i32 = -14i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut i32_2: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i32_3: i32 = 106i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_1: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_3);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_3);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::hour(primitivedatetime_1);
    std::ops::MulAssign::mul_assign(duration_0_ref_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3858() {
    rusty_monitor::set_test_id(3858);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = 151i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 70i32;
    let mut i128_0: i128 = -196i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i32_2: i32 = -90i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f32_0: f32 = 75.651443f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -22.443206f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -152i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_6);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_2: i64 = -94i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut u32_1: u32 = 46u32;
    let mut u8_3: u8 = 49u8;
    let mut u8_4: u8 = 79u8;
    let mut u8_5: u8 = 62u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 9u16;
    let mut i32_3: i32 = 94i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut i32_4: i32 = 130i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i64_3: i64 = 5i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_5: i32 = -62i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_9);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut u32_2: u32 = 43u32;
    let mut u8_6: u8 = 78u8;
    let mut u8_7: u8 = 4u8;
    let mut u8_8: u8 = 87u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i128_1: i128 = 130i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_6: i32 = -68i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_10);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_6, time_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_2);
    let mut i64_4: i64 = 89i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut u32_3: u32 = 84u32;
    let mut u8_9: u8 = 39u8;
    let mut u8_10: u8 = 8u8;
    let mut u8_11: u8 = 80u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_6, time_4);
    let mut offsetdatetime_7_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_7;
    let mut i32_7: i32 = 44i32;
    let mut i64_5: i64 = -123i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_7);
    let mut f64_1: f64 = -1.180734f64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_13, duration_12);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_14);
    let mut f64_2: f64 = crate::duration::Duration::as_seconds_f64(duration_11);
    let mut u8_12: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_5);
    let mut primitivedatetime_2_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    std::ops::AddAssign::add_assign(primitivedatetime_2_ref_0, duration_8);
    let mut u8_13: u8 = crate::date::Date::day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7189() {
    rusty_monitor::set_test_id(7189);
    let mut i32_0: i32 = 59i32;
    let mut i64_0: i64 = -120i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_1: i64 = -96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -129i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_1: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i32_2: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_3: i32 = 106i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_1: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(duration_4_ref_0, duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut tuple_1: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut i32_4: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6994() {
    rusty_monitor::set_test_id(6994);
    let mut i8_0: i8 = 32i8;
    let mut i64_0: i64 = 68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut i8_1: i8 = 40i8;
    let mut u16_0: u16 = 20u16;
    let mut i32_0: i32 = -117i32;
    let mut i8_2: i8 = -6i8;
    let mut i8_3: i8 = 97i8;
    let mut i8_4: i8 = -93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_1, i8_3, i8_4);
    let mut i64_1: i64 = 73i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 73u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 22u16;
    let mut i32_1: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i8_5: i8 = -123i8;
    let mut i8_6: i8 = -27i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_2, i8_5);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_2: i32 = 15i32;
    let mut i64_2: i64 = -99i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    std::ops::DivAssign::div_assign(duration_0_ref_0, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8032() {
    rusty_monitor::set_test_id(8032);
    let mut i8_0: i8 = -73i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_0: u32 = 77u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 85u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -21i32;
    let mut i32_1: i32 = -11i32;
    let mut i64_0: i64 = 31i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut i64_1: i64 = -114i64;
    let mut i64_2: i64 = -67i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut f32_0: f32 = 25.878963f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_2: i32 = -35i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u8_3: u8 = 21u8;
    let mut u8_4: u8 = 90u8;
    let mut u8_5: u8 = 49u8;
    let mut i64_3: i64 = -18i64;
    let mut u16_0: u16 = 6u16;
    let mut i32_3: i32 = -138i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_3);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut time_1: crate::time::Time = std::result::Result::unwrap(result_1);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::DivAssign::div_assign(duration_6_ref_0, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4545() {
    rusty_monitor::set_test_id(4545);
    let mut i8_0: i8 = 70i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut f32_0: f32 = 184.440486f32;
    let mut i64_0: i64 = -75i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_1: i8 = 69i8;
    let mut i8_2: i8 = -25i8;
    let mut i8_3: i8 = 95i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f64_0: f64 = -24.862251f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -73i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i64_1: i64 = 90i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i32_1: i32 = 39i32;
    let mut i64_2: i64 = -48i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i32_2: i32 = -149i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_7);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut u32_0: u32 = 93u32;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 9u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 63u8;
    let mut u8_5: u8 = 48u8;
    let mut i64_3: i64 = -68i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = 14i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i32_3: i32 = 60i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut u8_6: u8 = crate::time::Time::second(time_1);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    std::ops::MulAssign::mul_assign(duration_8_ref_0, f32_0);
    std::ops::MulAssign::mul_assign(duration_1_ref_0, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4710() {
    rusty_monitor::set_test_id(4710);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i64_0: i64 = -96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -129i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_2: i32 = 106i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1838() {
    rusty_monitor::set_test_id(1838);
    let mut i64_0: i64 = 80i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 30u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 91u16;
    let mut i32_0: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_0_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut i64_1: i64 = -96i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_1: u32 = 85u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -129i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_1: i32 = -14i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_2);
    let mut i32_2: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i32_3: i32 = 106i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_2: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::MulAssign::mul_assign(duration_6_ref_0, u32_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_3);
    std::ops::AddAssign::add_assign(duration_5_ref_0, duration_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_2);
    std::ops::SubAssign::sub_assign(primitivedatetime_0_ref_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_78() {
    rusty_monitor::set_test_id(78);
    let mut i64_0: i64 = -96i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -129i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = 96i32;
    let mut i64_2: i64 = -121i64;
    let mut i64_3: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i32_1: i32 = 106i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_0: u32 = 17u32;
    let mut i64_5: i64 = -17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    std::ops::MulAssign::mul_assign(duration_4_ref_0, u32_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    std::ops::AddAssign::add_assign(duration_3_ref_0, duration_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_803() {
    rusty_monitor::set_test_id(803);
    let mut f32_0: f32 = 36.399754f32;
    let mut i8_0: i8 = 45i8;
    let mut i8_1: i8 = -13i8;
    let mut i8_2: i8 = 54i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 75u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 43u8;
    let mut u8_2: u8 = 22u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -87i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 36u8;
    let mut u8_4: u8 = 65u8;
    let mut u8_5: u8 = 41u8;
    let mut f64_0: f64 = 43.438408f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_0: i64 = -71i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i16_0: i16 = -33i16;
    let mut i64_1: i64 = -110i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    std::ops::DivAssign::div_assign(duration_1_ref_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7850() {
    rusty_monitor::set_test_id(7850);
    let mut i64_0: i64 = 98i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i16_0: i16 = -93i16;
    let mut i64_1: i64 = -96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = -129i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = 96i32;
    let mut i64_3: i64 = -121i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i32_1: i32 = 106i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_0: u32 = 17u32;
    let mut i64_6: i64 = -17i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    std::ops::MulAssign::mul_assign(duration_5_ref_0, u32_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    std::ops::AddAssign::add_assign(duration_4_ref_0, duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    std::ops::MulAssign::mul_assign(duration_2_ref_0, i16_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7502() {
    rusty_monitor::set_test_id(7502);
    let mut u16_0: u16 = 72u16;
    let mut i64_0: i64 = -75i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 69i8;
    let mut i8_1: i8 = -25i8;
    let mut i8_2: i8 = 95i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f64_0: f64 = -24.862251f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -73i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i64_1: i64 = 90i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_3);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i32_1: i32 = 39i32;
    let mut i64_2: i64 = -48i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut i32_2: i32 = -149i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 48u8;
    let mut i64_3: i64 = -68i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = 14i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_3: i32 = 60i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    std::ops::DivAssign::div_assign(duration_6_ref_0, u16_0);
    panic!("From RustyUnit with love");
}
}