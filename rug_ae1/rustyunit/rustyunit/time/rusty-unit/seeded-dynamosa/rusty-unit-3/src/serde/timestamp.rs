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
#[timeout(30000)]fn rusty_test_130() {
//    rusty_monitor::set_test_id(130);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 28u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 7u16;
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_0_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 235i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_6: u8 = crate::time::Time::minute(time_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_158() {
//    rusty_monitor::set_test_id(158);
    let mut i32_0: i32 = 178i32;
    let mut i32_1: i32 = 4i32;
    let mut i32_2: i32 = 93i32;
    let mut i32_3: i32 = 195i32;
    let mut i32_4: i32 = 65i32;
    let mut i32_5: i32 = 4i32;
    let mut i32_6: i32 = 128i32;
    let mut i32_7: i32 = 398i32;
    let mut i32_8: i32 = 6i32;
    let mut i32_9: i32 = 1000000i32;
    let mut i32_10: i32 = 268i32;
    let mut i32_11: i32 = 4i32;
    let mut i32_12: i32 = 359i32;
    let mut i32_13: i32 = 156i32;
    let mut i32_14: i32 = 5119853i32;
    let mut i32_15: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_15);
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_14);
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_13);
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_12);
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_11);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_10);
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_9);
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut date_8: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut date_9: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut date_10: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_11: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_12: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_13: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_14: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_15: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_528() {
//    rusty_monitor::set_test_id(528);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_1: month::Month = crate::month::Month::October;
    let mut month_2: month::Month = crate::month::Month::October;
    let mut month_3: month::Month = crate::month::Month::October;
    let mut month_4: month::Month = crate::month::Month::October;
    let mut month_5: month::Month = crate::month::Month::October;
    let mut month_6: month::Month = crate::month::Month::October;
    let mut month_7: month::Month = crate::month::Month::October;
    let mut month_8: month::Month = crate::month::Month::October;
    let mut month_9: month::Month = crate::month::Month::October;
    let mut month_10: month::Month = crate::month::Month::October;
    let mut month_11: month::Month = crate::month::Month::October;
    let mut month_12: month::Month = crate::month::Month::October;
    let mut month_13: month::Month = crate::month::Month::October;
    let mut month_14: month::Month = crate::month::Month::October;
    let mut month_15: month::Month = crate::month::Month::October;
    let mut month_16: month::Month = crate::month::Month::October;
    let mut month_17: month::Month = crate::month::Month::October;
    let mut month_18: month::Month = crate::month::Month::October;
    let mut month_19: month::Month = crate::month::Month::October;
    let mut month_20: month::Month = crate::month::Month::October;
    let mut month_21: month::Month = crate::month::Month::October;
    let mut month_22: month::Month = crate::month::Month::October;
    let mut month_23: month::Month = crate::month::Month::October;
    let mut month_24: month::Month = crate::month::Month::October;
    let mut month_25: month::Month = crate::month::Month::October;
    let mut month_26: month::Month = crate::month::Month::October;
    let mut month_27: month::Month = crate::month::Month::October;
    let mut month_28: month::Month = crate::month::Month::October;
    let mut month_29: month::Month = crate::month::Month::October;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_183() {
//    rusty_monitor::set_test_id(183);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 6u8;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 400i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 24u8;
    let mut i32_1: i32 = 20i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_2: u32 = 1000u32;
    let mut u8_6: u8 = 12u8;
    let mut u8_7: u8 = 4u8;
    let mut u8_8: u8 = 4u8;
    let mut i32_2: i32 = -66i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_3: u32 = 44u32;
    let mut u8_9: u8 = 1u8;
    let mut u8_10: u8 = 49u8;
    let mut u8_11: u8 = 97u8;
    let mut u16_0: u16 = 59u16;
    let mut i32_3: i32 = 178i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut u32_4: u32 = 999999999u32;
    let mut u8_12: u8 = 29u8;
    let mut u8_13: u8 = 6u8;
    let mut u8_14: u8 = 2u8;
    let mut i32_4: i32 = 161i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_5, u8_14, u8_13, u8_12, u32_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_4, u8_11, u8_10, u8_9, u32_3);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_3, u8_8, u8_7, u8_6, u32_2);
    let mut result_3: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut result_4: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}
}