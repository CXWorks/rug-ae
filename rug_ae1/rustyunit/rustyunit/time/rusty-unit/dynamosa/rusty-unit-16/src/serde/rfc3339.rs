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
fn rusty_test_8171() {
    rusty_monitor::set_test_id(8171);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 56i64;
    let mut i64_1: i64 = -80i64;
    let mut i64_2: i64 = -203i64;
    let mut str_0: &str = "9G";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut f64_0: f64 = -18.714761f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -94i32;
    let mut i128_0: i128 = -144i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_1: i32 = -97i32;
    let mut i64_3: i64 = 54i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut u16_0: u16 = 16u16;
    let mut i32_2: i32 = 91i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u16_1: u16 = 53u16;
    let mut i32_3: i32 = 61i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_4: i64 = 178i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = 63i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_7);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut i64_6: i64 = -5i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i64_7: i64 = -187i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut i32_4: i32 = 85i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_9);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_8);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -53i8;
    let mut i8_1: i8 = 69i8;
    let mut i8_2: i8 = -75i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 37u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 11u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 15u8;
    let mut u8_5: u8 = 1u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_8: i64 = 59i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut u16_2: u16 = 22u16;
    let mut i32_5: i32 = 53i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_2);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_10);
    let mut i32_6: i32 = 134i32;
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut i32_7: i32 = -74i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_7, u16_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i64_9: i64 = crate::duration::Duration::whole_weeks(duration_0);
    panic!("From RustyUnit with love");
}
}