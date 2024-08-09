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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7733() {
//    rusty_monitor::set_test_id(7733);
    let mut i64_0: i64 = 2147483647i64;
    let mut i32_0: i32 = -114i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_1: i32 = 268i32;
    let mut i32_2: i32 = 174i32;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_2);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_3: i32 = 303i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_4: i32 = 1000i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_1: i64 = 86400i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 19u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i64_4: i64 = 86400i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_5: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_6: i64 = -63i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_6);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i64_7: i64 = 131i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i64_8: i64 = 60i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i32_5: i32 = 3i32;
    let mut i64_9: i64 = 1000000000i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_5);
    let mut i64_10: i64 = 1000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_11: i64 = 7i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_10);
    let mut i64_12: i64 = 12i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_11);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_4);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut i64_13: i64 = -10i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::hours(i64_12);
    let mut u16_1: u16 = 1u16;
    let mut i32_6: i32 = 280i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_8);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_13);
    let mut i32_7: i32 = 25i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut u16_2: u16 = 367u16;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_2);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_4);
    let mut option_1: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_5);
    let mut option_2: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_6);
    let mut option_3: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_7);
//    panic!("From RustyUnit with love");
}
}