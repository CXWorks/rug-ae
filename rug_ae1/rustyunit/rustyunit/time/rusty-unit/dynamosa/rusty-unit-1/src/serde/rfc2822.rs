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
fn rusty_test_7868() {
    rusty_monitor::set_test_id(7868);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 68i8;
    let mut i8_1: i8 = -44i8;
    let mut i8_2: i8 = -49i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 18u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = -62i8;
    let mut i8_4: i8 = -106i8;
    let mut i8_5: i8 = 108i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_0: f32 = 128.187058f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -112i32;
    let mut i64_0: i64 = -19i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i8_6: i8 = -49i8;
    let mut i8_7: i8 = -118i8;
    let mut i8_8: i8 = 66i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = 128i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u32_1: u32 = 92u32;
    let mut u8_3: u8 = 97u8;
    let mut u8_4: u8 = 37u8;
    let mut u8_5: u8 = 0u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = 33i8;
    let mut i8_10: i8 = 124i8;
    let mut i8_11: i8 = 47i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = 63i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut f32_1: f32 = 145.890813f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i8_12: i8 = 19i8;
    let mut i8_13: i8 = 5i8;
    let mut i8_14: i8 = -59i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = 24i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i32_1: i32 = 14i32;
    let mut i64_4: i64 = -63i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i64_5: i64 = 23i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i32_2: i32 = -6i32;
    let mut i64_6: i64 = 25i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut i8_15: i8 = 23i8;
    let mut i8_16: i8 = -84i8;
    let mut i8_17: i8 = -46i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 41i8;
    let mut i8_19: i8 = -96i8;
    let mut i8_20: i8 = -48i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 63u32;
    let mut u8_6: u8 = 44u8;
    let mut u8_7: u8 = 49u8;
    let mut u8_8: u8 = 83u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_21: i8 = 68i8;
    let mut i8_22: i8 = 5i8;
    let mut i8_23: i8 = 55i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_3: u32 = 86u32;
    let mut u8_9: u8 = 95u8;
    let mut u8_10: u8 = 63u8;
    let mut u8_11: u8 = 28u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_7: i64 = 12i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i8_24: i8 = -59i8;
    let mut i8_25: i8 = -106i8;
    let mut i8_26: i8 = -34i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i64_8: i64 = -168i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut i8_27: i8 = -122i8;
    let mut i8_28: i8 = -59i8;
    let mut i8_29: i8 = -27i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = 58i8;
    let mut i8_31: i8 = -25i8;
    let mut i8_32: i8 = -52i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i128_0: i128 = -152i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_4: u32 = 34u32;
    let mut u8_12: u8 = 99u8;
    let mut u8_13: u8 = 66u8;
    let mut u8_14: u8 = 35u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i64_9: i64 = -100i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut i8_33: i8 = -52i8;
    let mut i8_34: i8 = -128i8;
    let mut i8_35: i8 = 10i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = -56i8;
    let mut i8_37: i8 = -31i8;
    let mut i8_38: i8 = 35i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i8_39: i8 = -66i8;
    let mut i8_40: i8 = 84i8;
    let mut i8_41: i8 = 58i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i32_3: i32 = 31i32;
    let mut i64_10: i64 = 25i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::minutes(i64_10);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_16, i32_3);
    let mut u32_5: u32 = 54u32;
    let mut u8_15: u8 = 5u8;
    let mut u8_16: u8 = 78u8;
    let mut u8_17: u8 = 27u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i64_11: i64 = 32i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::weeks(i64_11);
    let mut i8_42: i8 = 23i8;
    let mut i8_43: i8 = -57i8;
    let mut i8_44: i8 = -23i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i8_45: i8 = 85i8;
    let mut i8_46: i8 = 44i8;
    let mut i8_47: i8 = 90i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i32_4: i32 = 72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_0);
    let mut time_6: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i128_1: i128 = 44i128;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_5: i32 = 22i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_19);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_4, date_2);
    let mut time_7: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_20: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_20);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_7};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_6);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_6: i32 = -55i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i64_12: i64 = -37i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::microseconds(i64_12);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut i32_7: i32 = 56i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_7};
    let mut date_4_ref_0: &mut crate::date::Date = &mut date_4;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut u8_18: u8 = crate::util::days_in_year_month(i32_6, month_0);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_15};
    panic!("From RustyUnit with love");
}
}