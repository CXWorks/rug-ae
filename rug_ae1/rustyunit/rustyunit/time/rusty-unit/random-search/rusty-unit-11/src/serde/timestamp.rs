//! Treat an [`OffsetDateTime`] as a [Unix timestamp] for the purposes of serde.
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! When deserializing, the offset is assumed to be UTC.
//!
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
//! [with]: https://serde.rs/field-attrs.html#with

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::OffsetDateTime;

/// Serialize an `OffsetDateTime` as its Unix timestamp
pub fn serialize<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    datetime.unix_timestamp().serialize(serializer)
}

/// Deserialize an `OffsetDateTime` from its Unix timestamp
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    OffsetDateTime::from_unix_timestamp(<_>::deserialize(deserializer)?)
        .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))
}

/// Treat an `Option<OffsetDateTime>` as a [Unix timestamp] for the purposes of
/// serde.
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// When deserializing, the offset is assumed to be UTC.
///
/// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
/// [with]: https://serde.rs/field-attrs.html#with
pub mod option {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Serialize an `Option<OffsetDateTime>` as its Unix timestamp
    pub fn serialize<S: Serializer>(
        option: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        option
            .map(OffsetDateTime::unix_timestamp)
            .serialize(serializer)
    }

    /// Deserialize an `Option<OffsetDateTime>` from its Unix timestamp
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        Option::deserialize(deserializer)?
            .map(OffsetDateTime::from_unix_timestamp)
            .transpose()
            .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3680() {
    rusty_monitor::set_test_id(3680);
    let mut u16_0: u16 = 50u16;
    let mut u8_0: u8 = 69u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 37u8;
    let mut i64_0: i64 = 73i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -22i8;
    let mut i8_1: i8 = -35i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 47i8;
    let mut i8_4: i8 = -85i8;
    let mut i8_5: i8 = -85i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -124i8;
    let mut i8_7: i8 = -16i8;
    let mut i8_8: i8 = 38i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 78u32;
    let mut u8_3: u8 = 76u8;
    let mut u8_4: u8 = 79u8;
    let mut u8_5: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i64_1: i64 = -105i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut f32_0: f32 = -8.370128f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_9: i8 = 41i8;
    let mut i8_10: i8 = 21i8;
    let mut i8_11: i8 = -8i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 19u32;
    let mut u8_6: u8 = 58u8;
    let mut u8_7: u8 = 19u8;
    let mut u8_8: u8 = 84u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i8_12: i8 = 107i8;
    let mut i8_13: i8 = 94i8;
    let mut i8_14: i8 = 114i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f32_1: f32 = 28.455541f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_0: i32 = -137i32;
    let mut i64_2: i64 = 11i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i8_15: i8 = -92i8;
    let mut i8_16: i8 = 102i8;
    let mut i8_17: i8 = 35i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i32_1: i32 = 134i32;
    let mut i32_2: i32 = -215i32;
    let mut i64_3: i64 = 110i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_1);
    let mut u32_2: u32 = 99u32;
    let mut u8_9: u8 = 69u8;
    let mut u8_10: u8 = 40u8;
    let mut u8_11: u8 = 79u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_2);
    let mut i64_4: i64 = 30i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i8_18: i8 = 67i8;
    let mut i8_19: i8 = 21i8;
    let mut i8_20: i8 = -113i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = -11i8;
    let mut i8_22: i8 = 67i8;
    let mut i8_23: i8 = 37i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 9i8;
    let mut i8_25: i8 = -64i8;
    let mut i8_26: i8 = -16i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_3: u32 = 3u32;
    let mut u8_12: u8 = 15u8;
    let mut u8_13: u8 = 40u8;
    let mut u8_14: u8 = 66u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_3);
    let mut i8_27: i8 = 7i8;
    let mut i8_28: i8 = -128i8;
    let mut i8_29: i8 = -43i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = -14i8;
    let mut i8_31: i8 = -3i8;
    let mut i8_32: i8 = -19i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut f32_2: f32 = 34.040051f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut i64_5: i64 = -60i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i8_33: i8 = 39i8;
    let mut i8_34: i8 = 24i8;
    let mut i8_35: i8 = 48i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut u32_4: u32 = 55u32;
    let mut u8_15: u8 = 8u8;
    let mut u8_16: u8 = 25u8;
    let mut u8_17: u8 = 51u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_4);
    let mut i64_6: i64 = -60i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut f32_3: f32 = 209.000613f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_3);
    let mut i8_36: i8 = -61i8;
    let mut i8_37: i8 = -66i8;
    let mut i8_38: i8 = -54i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut u32_5: u32 = 33u32;
    let mut i32_3: i32 = 27i32;
    let mut i64_7: i64 = 26i64;
    let mut u32_6: u32 = 8u32;
    let mut u8_18: u8 = 85u8;
    let mut u8_19: u8 = 1u8;
    let mut u8_20: u8 = 73u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i64_8: i64 = -114i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut i32_4: i32 = -3i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i64_9: i64 = 95i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut f32_4: f32 = -4.657839f32;
    let mut i64_10: i64 = 33i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_18: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_19: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut u32_7: u32 = 91u32;
    let mut u8_21: u8 = 83u8;
    let mut u8_22: u8 = 77u8;
    let mut u8_23: u8 = 28u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_17, duration_16);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}
}