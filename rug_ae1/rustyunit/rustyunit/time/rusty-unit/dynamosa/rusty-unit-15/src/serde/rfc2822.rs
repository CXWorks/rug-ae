//! Use the well-known [RFC2822 format] when serializing and deserializing an [`OffsetDateTime`].
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! [RFC2822 format]: https://tools.ietf.org/html/rfc2822#section-3.3
//! [with]: https://serde.rs/field-attrs.html#with

use core::marker::PhantomData;

use serde::ser::Error as _;
use serde::{Deserializer, Serialize, Serializer};

use super::Visitor;
use crate::format_description::well_known::Rfc2822;
use crate::OffsetDateTime;

/// Serialize an [`OffsetDateTime`] using the well-known RFC2822 format.
pub fn serialize<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    datetime
        .format(&Rfc2822)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

/// Deserialize an [`OffsetDateTime`] from its RFC2822 representation.
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    deserializer.deserialize_any(Visitor::<Rfc2822>(PhantomData))
}

/// Use the well-known [RFC2822 format] when serializing and deserializing an
/// [`Option<OffsetDateTime>`].
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// [RFC2822 format]: https://tools.ietf.org/html/rfc2822#section-3.3
/// [with]: https://serde.rs/field-attrs.html#with
pub mod option {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Serialize an [`Option<OffsetDateTime>`] using the well-known RFC2822 format.
    pub fn serialize<S: Serializer>(
        option: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        option
            .map(|odt| odt.format(&Rfc2822))
            .transpose()
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    /// Deserialize an [`Option<OffsetDateTime>`] from its RFC2822 representation.
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        deserializer.deserialize_option(Visitor::<Option<Rfc2822>>(PhantomData))
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6585() {
    rusty_monitor::set_test_id(6585);
    let mut i64_0: i64 = 55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 94u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f64_0: f64 = 51.564389f64;
    let mut i128_0: i128 = -122i128;
    let mut i64_2: i64 = 71i64;
    let mut i64_3: i64 = -55i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i32_1: i32 = -186i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut i64_4: i64 = 90i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i64_5: i64 = 42i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i64_6: i64 = 2i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_6);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_2: i32 = 255i32;
    let mut i64_7: i64 = 13i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_7);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 14u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = 34i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_8: i64 = -31i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_4: i32 = -22i32;
    let mut i64_9: i64 = -52i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_4);
    let mut i64_10: i64 = 14i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_10);
    let mut i64_11: i64 = -58i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut i32_5: i32 = -181i32;
    let mut i64_12: i64 = 229i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new(i64_11, i32_5);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_8, duration_13);
    let mut i64_13: i64 = 143i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::microseconds(i64_12);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut i64_14: i64 = 157i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds(i64_13);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut u8_6: u8 = 41u8;
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i32_6: i32 = 92i32;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::microseconds(i64_14);
    let mut u16_0: u16 = 13u16;
    let mut i32_7: i32 = -212i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_23);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_7, u8_6, weekday_0);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}
}