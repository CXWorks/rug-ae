//! Error parsing an input into a [`Parsed`](crate::parsing::Parsed) struct

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// An error that occurred while parsing the input into a [`Parsed`](crate::parsing::Parsed) struct.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseFromDescription {
    /// A string literal was not what was expected.
    #[non_exhaustive]
    InvalidLiteral,
    /// A dynamic component was not valid.
    InvalidComponent(&'static str),
}

impl fmt::Display for ParseFromDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("a character literal was not valid"),
            Self::InvalidComponent(name) => {
                write!(f, "the '{}' component could not be parsed", name)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseFromDescription {}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<ParseFromDescription> for crate::Error {
    fn from(original: ParseFromDescription) -> Self {
        Self::ParseFromDescription(original)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<crate::Error> for ParseFromDescription {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ParseFromDescription(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8235() {
//    rusty_monitor::set_test_id(8235);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = 69i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = 25i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut i32_1: i32 = 36525i32;
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut i64_0: i64 = 124i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_2: i32 = -59i32;
    let mut i64_1: i64 = 1000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_2);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 11u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 81i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = 4i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 83i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = 140i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_9: i8 = 0i8;
    let mut i8_10: i8 = 24i8;
    let mut i8_11: i8 = 6i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut i8_12: i8 = 5i8;
    let mut i8_13: i8 = 6i8;
    let mut i8_14: i8 = 1i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_4: i64 = 9223372036854775807i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i8_15: i8 = 5i8;
    let mut i8_16: i8 = 2i8;
    let mut i8_17: i8 = 1i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 50u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_18: i8 = 5i8;
    let mut i8_19: i8 = 23i8;
    let mut i8_20: i8 = 127i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = 4i8;
    let mut i8_22: i8 = 4i8;
    let mut i8_23: i8 = -123i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_3: i32 = 3600i32;
    let mut i64_5: i64 = 253402300799i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_3);
    let mut i8_24: i8 = 4i8;
    let mut i8_25: i8 = 4i8;
    let mut i8_26: i8 = 5i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i128_2: i128 = 1i128;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut f32_2: f32 = 1065353216.000000f32;
    let mut i64_6: i64 = 3600i64;
    let mut u32_2: u32 = 10000u32;
    let mut u8_6: u8 = 52u8;
    let mut u8_7: u8 = 28u8;
    let mut u8_8: u8 = 59u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_0: u16 = 0u16;
    let mut i32_4: i32 = 144i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut u8_9: u8 = 28u8;
    let mut i32_5: i32 = 376i32;
    let mut i64_7: i64 = 604800i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_18: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut u32_3: u32 = 94u32;
    let mut u8_10: u8 = 6u8;
    let mut u8_11: u8 = 52u8;
    let mut u8_12: u8 = 5u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut i32_6: i32 = -21i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_6};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_4};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_3, duration_18);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_4, primitivedatetime_4);
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i64_8: i64 = -8i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::November;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_17);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_20, duration_19);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut month_3: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut i32_7: i32 = 195i32;
    let mut u8_13: u8 = 23u8;
    let mut month_4: month::Month = crate::month::Month::May;
    let mut i32_8: i32 = 6i32;
    let mut u8_14: u8 = 76u8;
    let mut month_5: month::Month = crate::month::Month::May;
    let mut i32_9: i32 = 3652425i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_7, month_4, u8_14);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_9, month_5, u8_13);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_8, month_3, u8_9);
//    panic!("From RustyUnit with love");
}
}