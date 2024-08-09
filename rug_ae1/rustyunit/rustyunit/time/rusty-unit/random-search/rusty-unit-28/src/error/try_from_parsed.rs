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
fn rusty_test_4758() {
    rusty_monitor::set_test_id(4758);
    let mut i32_0: i32 = -2i32;
    let mut i64_0: i64 = -17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut u32_0: u32 = 90u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 56u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -120i8;
    let mut i8_1: i8 = 39i8;
    let mut i8_2: i8 = -60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -62i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i8_3: i8 = -72i8;
    let mut i8_4: i8 = -123i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = 59i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i128_0: i128 = 50i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i8_6: i8 = 10i8;
    let mut i8_7: i8 = -81i8;
    let mut i8_8: i8 = 1i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -30i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i8_9: i8 = 19i8;
    let mut i8_10: i8 = 40i8;
    let mut i8_11: i8 = 103i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -86i8;
    let mut i8_13: i8 = -120i8;
    let mut i8_14: i8 = -43i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f32_0: f32 = -92.512932f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut u32_1: u32 = 35u32;
    let mut u8_3: u8 = 33u8;
    let mut u8_4: u8 = 58u8;
    let mut u8_5: u8 = 51u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_15: i8 = 31i8;
    let mut i8_16: i8 = 69i8;
    let mut i8_17: i8 = 91i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut f64_0: f64 = 6.022627f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_2: u32 = 42u32;
    let mut u8_6: u8 = 69u8;
    let mut u8_7: u8 = 65u8;
    let mut u8_8: u8 = 73u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_18: i8 = -9i8;
    let mut i8_19: i8 = 17i8;
    let mut i8_20: i8 = 27i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = -63i8;
    let mut i8_22: i8 = 112i8;
    let mut i8_23: i8 = -28i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i64_4: i64 = -2i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i8_24: i8 = -80i8;
    let mut i8_25: i8 = -128i8;
    let mut i8_26: i8 = 57i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = -38i8;
    let mut i8_28: i8 = 33i8;
    let mut i8_29: i8 = 57i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i32_1: i32 = 98i32;
    let mut f64_1: f64 = -75.828399f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_1);
    let mut u32_3: u32 = 99u32;
    let mut u8_9: u8 = 48u8;
    let mut u8_10: u8 = 99u8;
    let mut u8_11: u8 = 10u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_5: i64 = 110i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut i8_30: i8 = 38i8;
    let mut i8_31: i8 = 19i8;
    let mut i8_32: i8 = 2i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i32_2: i32 = -35i32;
    let mut u16_0: u16 = 20u16;
    let mut u8_12: u8 = 59u8;
    let mut u8_13: u8 = 9u8;
    let mut u8_14: u8 = 74u8;
    let mut i32_3: i32 = -118i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_6: i64 = 19i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i64_7: i64 = -17i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut i64_8: i64 = -167i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_14, u8_13, u8_12, u16_0);
    let mut u8_15: u8 = crate::date::Date::monday_based_week(date_0);
    panic!("From RustyUnit with love");
}
}