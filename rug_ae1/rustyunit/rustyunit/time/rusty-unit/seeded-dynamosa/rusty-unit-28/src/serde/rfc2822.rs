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
#[timeout(30000)]fn rusty_test_8701() {
//    rusty_monitor::set_test_id(8701);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 24u8;
    let mut i64_0: i64 = 2440588i64;
    let mut i64_1: i64 = 17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = -49i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_1: i32 = 150i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 212i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 43u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 7u16;
    let mut i32_3: i32 = 314i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i64_2: i64 = -1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut f64_0: f64 = -56.692536f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut u16_1: u16 = 365u16;
    let mut i32_4: i32 = 184i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_7);
    let mut i32_5: i32 = 99i32;
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut u16_2: u16 = 0u16;
    let mut i32_6: i32 = 66i32;
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_2);
    let mut date_9: crate::date::Date = crate::date::Date::saturating_sub(date_6, duration_6);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_7, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}
}