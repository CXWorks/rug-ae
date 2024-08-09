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
fn rusty_test_7591() {
    rusty_monitor::set_test_id(7591);
    let mut i64_0: i64 = 207i64;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 122i8;
    let mut i8_1: i8 = -85i8;
    let mut i8_2: i8 = 68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut month_1: month::Month = crate::month::Month::October;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_2: month::Month = crate::month::Month::December;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut i32_0: i32 = -12i32;
    let mut i32_1: i32 = 80i32;
    let mut i64_1: i64 = -34i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_2: i32 = -141i32;
    let mut i64_2: i64 = -60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut i32_3: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i32_4: i32 = 55i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_4);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    panic!("From RustyUnit with love");
}
}