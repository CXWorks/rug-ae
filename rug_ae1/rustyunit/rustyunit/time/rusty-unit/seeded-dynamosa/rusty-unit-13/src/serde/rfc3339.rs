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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7634() {
//    rusty_monitor::set_test_id(7634);
    let mut i64_0: i64 = 2147483647i64;
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut i64_1: i64 = 24i64;
    let mut i64_2: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u32_0: u32 = 3u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 96i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 246i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 34u8;
    let mut i64_3: i64 = 1000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 11i8;
    let mut i8_2: i8 = 0i8;
    let mut i64_4: i64 = 1000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut f64_1: f64 = 4607182418800017408.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_2);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_3, i8_4);
    let mut i8_6: i8 = 0i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = -120i8;
    let mut i8_9: i8 = 59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_7, i8_2, i8_6);
    let mut i32_2: i32 = 376i32;
    let mut i64_5: i64 = 1i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 12u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_3, u8_4, u32_1);
    let mut i8_10: i8 = 2i8;
    let mut i8_11: i8 = 5i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_1, i8_5, i8_10);
    let mut i8_12: i8 = 43i8;
    let mut i8_13: i8 = 23i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_11, i8_12);
    let mut i32_3: i32 = 392i32;
    let mut i64_6: i64 = 60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut i8_14: i8 = 3i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_8);
    let mut i64_7: i64 = 1000000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_7);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
//    panic!("From RustyUnit with love");
}
}