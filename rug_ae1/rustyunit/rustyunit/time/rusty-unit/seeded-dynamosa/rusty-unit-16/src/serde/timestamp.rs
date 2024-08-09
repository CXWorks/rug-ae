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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7949() {
//    rusty_monitor::set_test_id(7949);
    let mut i32_0: i32 = 161i32;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut i64_0: i64 = 24i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = -99i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_2: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 111i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_3: i64 = -31i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_2: i32 = 365i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i64_4: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i32_3: i32 = 6i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_4: i32 = 4i32;
    let mut i64_5: i64 = 1000000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut i64_6: i64 = 12i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_7: i64 = 1i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_3: i8 = 36i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 3i8;
    let mut i8_6: i8 = -6i8;
    let mut i8_7: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_7, i8_4, i8_3);
    let mut i32_5: i32 = 115i32;
    let mut i64_8: i64 = 0i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_5);
    let mut i64_9: i64 = 0i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_6: i32 = 3652425i32;
    let mut i64_10: i64 = 24i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_6);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_11: i64 = 60i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::hours(i64_10);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut i8_8: i8 = -30i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_8, i8_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_0: u32 = 10000000u32;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::minutes(i64_11);
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 1u8;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_0};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_1, u8_2, u8_0, u32_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_10);
//    panic!("From RustyUnit with love");
}
}