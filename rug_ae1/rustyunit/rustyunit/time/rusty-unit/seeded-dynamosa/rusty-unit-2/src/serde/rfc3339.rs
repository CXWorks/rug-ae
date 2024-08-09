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
#[timeout(30000)]fn rusty_test_8546() {
//    rusty_monitor::set_test_id(8546);
    let mut i128_0: i128 = 0i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = 353i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 53u8;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut bool_0: bool = true;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut i64_2: i64 = 86400i64;
    let mut i64_3: i64 = 1i64;
    let mut str_0: &str = "month";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut u16_1: u16 = 365u16;
    let mut i32_1: i32 = 364i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut f32_1: f32 = 59.026492f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
//    panic!("From RustyUnit with love");
}
}