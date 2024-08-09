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
#[timeout(30000)]fn rusty_test_7677() {
//    rusty_monitor::set_test_id(7677);
    let mut i32_0: i32 = 376i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = 359i32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_2: i32 = 6i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_3: i32 = 150i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
//    panic!("From RustyUnit with love");
}
}