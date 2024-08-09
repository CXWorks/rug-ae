//! Error converting a [`Parsed`](crate::parsing::Parsed) struct to another type

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// An error that occurred when converting a [`Parsed`](crate::parsing::Parsed) to another type.
#[non_exhaustive]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromParsed {
    /// The [`Parsed`](crate::parsing::Parsed) did not include enough information to construct the
    /// type.
    InsufficientInformation,
    /// Some component contained an invalid value for the type.
    ComponentRange(error::ComponentRange),
}

impl fmt::Display for TryFromParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientInformation => f.write_str(
                "the `Parsed` struct did not include enough information to construct the type",
            ),
            Self::ComponentRange(err) => err.fmt(f),
        }
    }
}

impl From<error::ComponentRange> for TryFromParsed {
    fn from(v: error::ComponentRange) -> Self {
        Self::ComponentRange(v)
    }
}

impl TryFrom<TryFromParsed> for error::ComponentRange {
    type Error = error::DifferentVariant;

    fn try_from(err: TryFromParsed) -> Result<Self, Self::Error> {
        match err {
            TryFromParsed::ComponentRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromParsed {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InsufficientInformation => None,
            Self::ComponentRange(err) => Some(err),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<TryFromParsed> for crate::Error {
    fn from(original: TryFromParsed) -> Self {
        Self::TryFromParsed(original)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<crate::Error> for TryFromParsed {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::TryFromParsed(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4161() {
    rusty_monitor::set_test_id(4161);
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 66u8;
    let mut u8_2: u8 = 26u8;
    let mut i64_0: i64 = -104i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 33i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = 35i8;
    let mut i8_1: i8 = -103i8;
    let mut i8_2: i8 = 29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 124i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_3: i64 = 27i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_3: i8 = 44i8;
    let mut i8_4: i8 = 32i8;
    let mut i8_5: i8 = 96i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 12i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = -6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_0: u32 = 8u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 94u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i64_4: i64 = -17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i64_5: i64 = 110i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut i32_0: i32 = 231i32;
    let mut i64_6: i64 = 31i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_0);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i8_9: i8 = -88i8;
    let mut i8_10: i8 = -43i8;
    let mut i8_11: i8 = -56i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f64_0: f64 = 68.478115f64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut i8_12: i8 = 77i8;
    let mut i8_13: i8 = -120i8;
    let mut i8_14: i8 = -83i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f32_0: f32 = -0.313653f32;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut f32_1: f32 = 153.704012f32;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i64_7: i64 = 135i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = -95i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i32_1: i32 = 12i32;
    let mut i64_9: i64 = 4i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_1);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_20, duration_19);
    let mut i8_15: i8 = 11i8;
    let mut i8_16: i8 = 33i8;
    let mut i8_17: i8 = 4i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i32_2: i32 = -28i32;
    let mut i8_18: i8 = 54i8;
    let mut i8_19: i8 = 41i8;
    let mut i8_20: i8 = -78i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_6);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i32_3: i32 = 55i32;
    let mut i64_10: i64 = 43i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::hours(i64_10);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_22, i32_3);
    let mut u16_0: u16 = 27u16;
    let mut i32_4: i32 = 87i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_23);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut i64_11: i64 = -124i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::minutes(i64_11);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_25: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_25, duration_24);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_12: i64 = -282i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::hours(i64_12);
    let mut i32_5: i32 = -50i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_27);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_4, date_6);
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i8_21: i8 = 31i8;
    let mut i8_22: i8 = 72i8;
    let mut i8_23: i8 = 90i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_6, utcoffset_8);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut i32_6: i32 = -26i32;
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_7, time_1);
    let mut f32_2: f32 = 30.997933f32;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut i32_7: i32 = 132i32;
    let mut date_8: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut i32_8: i32 = -134i32;
    let mut i64_13: i64 = -69i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::new(i64_13, i32_8);
    let mut i32_9: i32 = 29i32;
    let mut i64_14: i64 = 93i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_14, i32_9);
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_30, duration_29);
    let mut u32_1: u32 = 97u32;
    let mut u8_6: u8 = 31u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 1u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut f64_1: f64 = 38.844097f64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut date_9: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_10: crate::date::Date = crate::date::Date::saturating_sub(date_9, duration_32);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_10, time_2);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_4, duration_26);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}
}