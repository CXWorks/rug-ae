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
fn rusty_test_8098() {
    rusty_monitor::set_test_id(8098);
    let mut u16_0: u16 = 41u16;
    let mut f64_0: f64 = 68.222982f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = 53.615208f32;
    let mut i64_0: i64 = 87i64;
    let mut i128_0: i128 = 27i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_1: f32 = 28.683509f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_1: i64 = -402i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_0: i32 = 36i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_1: i32 = -69i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut i8_0: i8 = -104i8;
    let mut i8_1: i8 = 87i8;
    let mut u8_0: u8 = 27u8;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 95u8;
    let mut i8_2: i8 = 108i8;
    let mut i8_3: i8 = 51i8;
    let mut i8_4: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_3, i8_0);
    let mut i64_2: i64 = 28i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_2: i32 = 24i32;
    let mut i64_3: i64 = -73i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut i32_3: i32 = 107i32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_3);
    let mut i8_5: i8 = -100i8;
    let mut i8_6: i8 = -72i8;
    let mut i8_7: i8 = -100i8;
    let mut i8_8: i8 = 74i8;
    let mut i8_9: i8 = -31i8;
    let mut i8_10: i8 = 42i8;
    let mut i8_11: i8 = 16i8;
    let mut i8_12: i8 = 8i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_7, i8_9, i8_10);
    let mut i32_4: i32 = -73i32;
    let mut i64_4: i64 = -53i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_4);
    let mut i8_13: i8 = -3i8;
    let mut i8_14: i8 = -20i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_5, i8_12);
    let mut u32_0: u32 = 36u32;
    let mut u8_3: u8 = 42u8;
    let mut u8_4: u8 = 96u8;
    let mut u8_5: u8 = 63u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_1, u8_2, u8_4, u32_0);
    let mut i64_5: i64 = -49i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_8);
    let mut i8_15: i8 = -70i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_4, i8_11, i8_14);
    let mut i64_6: i64 = -308i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i8_16: i8 = 14i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_15, i8_1);
    let mut u32_1: u32 = 88u32;
    let mut u8_6: u8 = 30u8;
    let mut u8_7: u8 = 8u8;
    let mut i64_7: i64 = 3i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i8_17: i8 = -43i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_13);
    let mut u32_2: u32 = 32u32;
    let mut u8_8: u8 = 45u8;
    let mut u8_9: u8 = 81u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_5, u8_3, u32_1);
    let mut i64_8: i64 = -21i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_13);
    let mut i64_9: i64 = 149i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f64_1: f64 = 127.741052f64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f64_2: f64 = 51.198282f64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_5: i32 = 43i32;
    let mut i64_10: i64 = 4i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_5);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_15, duration_18);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_5);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_10);
    let mut u8_10: u8 = 45u8;
    let mut u8_11: u8 = 43u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_0, u8_8, u8_11, u32_2);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_17);
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_6, u8_9, u8_10, u16_0);
    panic!("From RustyUnit with love");
}
}