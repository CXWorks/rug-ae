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
fn rusty_test_4591() {
    rusty_monitor::set_test_id(4591);
    let mut i32_0: i32 = 101i32;
    let mut i64_0: i64 = 42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 63i8;
    let mut i8_2: i8 = 50i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 10i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 71i8;
    let mut i8_7: i8 = -1i8;
    let mut i8_8: i8 = -56i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 82u32;
    let mut u8_0: u8 = 33u8;
    let mut u8_1: u8 = 84u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -39i32;
    let mut i64_1: i64 = 105i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_2: i64 = 34i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i8_9: i8 = 1i8;
    let mut i8_10: i8 = 87i8;
    let mut i8_11: i8 = -38i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 69i8;
    let mut i8_13: i8 = -19i8;
    let mut i8_14: i8 = -21i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i32_2: i32 = -55i32;
    let mut i64_3: i64 = -187i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut i64_4: i64 = 81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i8_15: i8 = 28i8;
    let mut i8_16: i8 = -101i8;
    let mut i8_17: i8 = 63i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_1: u32 = 88u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_18: i8 = -47i8;
    let mut i8_19: i8 = -28i8;
    let mut i8_20: i8 = 105i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_5: i64 = 51i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = 130i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut f32_0: f32 = -67.360982f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i8_21: i8 = -6i8;
    let mut i8_22: i8 = -9i8;
    let mut i8_23: i8 = 43i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_2: u32 = 82u32;
    let mut u8_6: u8 = 6u8;
    let mut u8_7: u8 = 67u8;
    let mut u8_8: u8 = 42u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_24: i8 = -11i8;
    let mut i8_25: i8 = 65i8;
    let mut i8_26: i8 = 29i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i64_7: i64 = 111i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut u32_3: u32 = 27u32;
    let mut u8_9: u8 = 25u8;
    let mut u8_10: u8 = 58u8;
    let mut u8_11: u8 = 98u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_8: i64 = -63i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut i8_27: i8 = 24i8;
    let mut i8_28: i8 = 74i8;
    let mut i8_29: i8 = 108i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_4: u32 = 25u32;
    let mut u8_12: u8 = 43u8;
    let mut u8_13: u8 = 66u8;
    let mut u8_14: u8 = 74u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_9: i64 = -110i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_9);
    let mut i8_30: i8 = 81i8;
    let mut i8_31: i8 = -5i8;
    let mut i8_32: i8 = 79i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = -22i8;
    let mut i8_34: i8 = 72i8;
    let mut i8_35: i8 = -37i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = 59i8;
    let mut i64_10: i64 = -149i64;
    let mut i8_37: i8 = -38i8;
    let mut f32_1: f32 = -19.660331f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_11: i64 = -73i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_11);
    let mut i64_12: i64 = 152i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_12);
    let mut i64_13: i64 = 115i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::microseconds(i64_13);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_16, duration_15);
    let mut i16_0: i16 = -125i16;
    let mut i64_14: i64 = 132i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut i32_3: i32 = -156i32;
    let mut i64_15: i64 = -204i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::minutes(i64_15);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_19, i32_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_17);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_14);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds(i64_10);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_0);
    panic!("From RustyUnit with love");
}
}