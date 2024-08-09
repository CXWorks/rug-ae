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
#[timeout(30000)]fn rusty_test_7814() {
//    rusty_monitor::set_test_id(7814);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i32_0: i32 = 189i32;
    let mut month_1: month::Month = crate::month::Month::April;
    let mut i32_1: i32 = 0i32;
    let mut month_2: month::Month = crate::month::Month::May;
    let mut i32_2: i32 = 1000i32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 0u8;
    let mut i32_3: i32 = 5i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 246i32;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_4);
    let mut i32_5: i32 = 88i32;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_5);
    let mut i64_2: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i32_6: i32 = 54i32;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_6);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut u32_0: u32 = 999999u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i128_0: i128 = 1000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_4: i64 = 604800i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_7);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_5: i64 = 1i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i64_6: i64 = 76i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut u8_6: u8 = 12u8;
    let mut month_3: month::Month = crate::month::Month::February;
    let mut i32_7: i32 = 511i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_2, u8_1);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_7, month_3, u8_0);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_1, month_1, u8_2);
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_6);
//    panic!("From RustyUnit with love");
}
}