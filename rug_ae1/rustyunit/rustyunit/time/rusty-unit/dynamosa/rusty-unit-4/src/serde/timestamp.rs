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
fn rusty_test_7143() {
    rusty_monitor::set_test_id(7143);
    let mut i8_0: i8 = 46i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = -8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 58i8;
    let mut i8_4: i8 = -120i8;
    let mut i8_5: i8 = 86i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut f64_0: f64 = 13.636505f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = -30i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_0: u32 = 30u32;
    let mut u8_0: u8 = 37u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_6: i8 = -25i8;
    let mut i8_7: i8 = 83i8;
    let mut i8_8: i8 = 95i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_2: i64 = 49i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i128_1: i128 = 3i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut i8_9: i8 = 95i8;
    let mut i8_10: i8 = -121i8;
    let mut i8_11: i8 = -116i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -9i8;
    let mut i8_13: i8 = 82i8;
    let mut i8_14: i8 = -23i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 10i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 33u8;
    let mut u8_5: u8 = 85u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_15: i8 = 91i8;
    let mut i8_16: i8 = 65i8;
    let mut i8_17: i8 = -84i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_5: i64 = -8i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i8_18: i8 = -81i8;
    let mut i8_19: i8 = -76i8;
    let mut i8_20: i8 = -58i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_6: i64 = 6i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut i32_0: i32 = -82i32;
    let mut i64_7: i64 = 39i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_17, i32_0);
    let mut i8_21: i8 = 112i8;
    let mut i8_22: i8 = 17i8;
    let mut i8_23: i8 = -28i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_1: i32 = -94i32;
    let mut i64_8: i64 = 99i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_1);
    let mut i64_9: i64 = 167i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::microseconds(i64_9);
    let mut i8_24: i8 = -64i8;
    let mut i8_25: i8 = 31i8;
    let mut i8_26: i8 = -59i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i64_10: i64 = 77i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::microseconds(i64_10);
    let mut i8_27: i8 = 35i8;
    let mut i8_28: i8 = -21i8;
    let mut i8_29: i8 = 38i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i32_2: i32 = -65i32;
    let mut i64_11: i64 = 36i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_11, i32_2);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut i64_12: i64 = -100i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_12);
    let mut duration_25: std::time::Duration = crate::duration::Duration::abs_std(duration_24);
    let mut i64_13: i64 = -113i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::days(i64_13);
    let mut u32_2: u32 = 86u32;
    let mut u8_6: u8 = 79u8;
    let mut u8_7: u8 = 40u8;
    let mut u8_8: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i64_14: i64 = crate::duration::Duration::whole_days(duration_10);
    let mut u8_9: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    panic!("From RustyUnit with love");
}
}