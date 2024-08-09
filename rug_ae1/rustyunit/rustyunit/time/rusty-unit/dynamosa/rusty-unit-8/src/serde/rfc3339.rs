//! Use the well-known [RFC3339 format] when serializing and deserializing an [`OffsetDateTime`].
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! [RFC3339 format]: https://tools.ietf.org/html/rfc3339#section-5.6
//! [with]: https://serde.rs/field-attrs.html#with

use core::marker::PhantomData;

use serde::ser::Error as _;
use serde::{Deserializer, Serialize, Serializer};

use super::Visitor;
use crate::format_description::well_known::Rfc3339;
use crate::OffsetDateTime;

/// Serialize an [`OffsetDateTime`] using the well-known RFC3339 format.
pub fn serialize<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    datetime
        .format(&Rfc3339)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

/// Deserialize an [`OffsetDateTime`] from its RFC3339 representation.
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    deserializer.deserialize_any(Visitor::<Rfc3339>(PhantomData))
}

/// Use the well-known [RFC3339 format] when serializing and deserializing an
/// [`Option<OffsetDateTime>`].
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// [RFC3339 format]: https://tools.ietf.org/html/rfc3339#section-5.6
/// [with]: https://serde.rs/field-attrs.html#with
pub mod option {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Serialize an [`Option<OffsetDateTime>`] using the well-known RFC3339 format.
    pub fn serialize<S: Serializer>(
        option: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        option
            .map(|odt| odt.format(&Rfc3339))
            .transpose()
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    /// Deserialize an [`Option<OffsetDateTime>`] from its RFC3339 representation.
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        deserializer.deserialize_option(Visitor::<Option<Rfc3339>>(PhantomData))
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4834() {
    rusty_monitor::set_test_id(4834);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i32_0: i32 = -158i32;
    let mut i32_1: i32 = -55i32;
    let mut i64_0: i64 = -104i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = 72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_2: i32 = -123i32;
    let mut i64_3: i64 = -152i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 76i8;
    let mut i8_2: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -38.884742f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_4: i64 = 75i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_3: i32 = 11i32;
    let mut i64_5: i64 = -15i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_3);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_4: i32 = -41i32;
    let mut i64_6: i64 = 130i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_4);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i8_3: i8 = -76i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = -38i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i32_5: i32 = -16i32;
    let mut i32_6: i32 = -5i32;
    let mut i64_7: i64 = 74i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_6);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_5);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 97u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 36u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = -89i8;
    let mut i8_7: i8 = 97i8;
    let mut i8_8: i8 = -94i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_0: i128 = 64i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_9: i8 = -24i8;
    let mut i8_10: i8 = -109i8;
    let mut i8_11: i8 = -51i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_8: i64 = 1i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i64_9: i64 = -56i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut i64_10: i64 = 18i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_10);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_17, duration_16);
    let mut duration_19: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut i64_11: i64 = -10i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut i32_7: i32 = 33i32;
    let mut i64_12: i64 = 90i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds(i64_12);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_21, i32_7);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_5);
    let mut u8_6: u8 = 2u8;
    let mut i32_8: i32 = -180i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_8, u8_6, weekday_0);
    panic!("From RustyUnit with love");
}
}