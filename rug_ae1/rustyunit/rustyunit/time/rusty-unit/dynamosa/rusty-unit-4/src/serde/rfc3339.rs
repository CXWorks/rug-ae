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
fn rusty_test_4334() {
    rusty_monitor::set_test_id(4334);
    let mut i32_0: i32 = -123i32;
    let mut f32_0: f32 = 87.451552f32;
    let mut i64_0: i64 = 40i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 46i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = -8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_3: i8 = 58i8;
    let mut i8_4: i8 = -120i8;
    let mut i8_5: i8 = 86i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
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
    let mut i64_2: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = 49i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i8_9: i8 = -15i8;
    let mut i8_10: i8 = 97i8;
    let mut i64_4: i64 = 32i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut u8_3: u8 = 60u8;
    let mut i64_5: i64 = 37i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_6: i64 = -111i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i64_7: i64 = 154i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i8_11: i8 = 5i8;
    let mut i8_12: i8 = -87i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_10, i8_12, i8_9);
    let mut i32_1: i32 = 65i32;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_13, i32_1);
    let mut i8_13: i8 = 38i8;
    let mut i8_14: i8 = -55i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_13, i8_11, i8_14);
    let mut i32_2: i32 = 4i32;
    let mut i64_8: i64 = -344i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_2);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_14);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut i32_3: i32 = 5i32;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u8_4: u8 = crate::time::Time::minute(time_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_3);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    panic!("From RustyUnit with love");
}
}