//! Error that occurred at some stage of parsing

use core::convert::TryFrom;
use core::fmt;

use crate::error::{self, ParseFromDescription, TryFromParsed};

/// An error that occurred at some stage of parsing.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[allow(variant_size_differences)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parse {
    #[allow(clippy::missing_docs_in_private_items)]
    TryFromParsed(TryFromParsed),
    #[allow(clippy::missing_docs_in_private_items)]
    ParseFromDescription(ParseFromDescription),
    /// The input should have ended, but there were characters remaining.
    #[non_exhaustive]
    UnexpectedTrailingCharacters,
}

impl fmt::Display for Parse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromParsed(err) => err.fmt(f),
            Self::ParseFromDescription(err) => err.fmt(f),
            Self::UnexpectedTrailingCharacters => f.write_str("unexpected trailing characters"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Parse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TryFromParsed(err) => Some(err),
            Self::ParseFromDescription(err) => Some(err),
            Self::UnexpectedTrailingCharacters => None,
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<TryFromParsed> for Parse {
    fn from(err: TryFromParsed) -> Self {
        Self::TryFromParsed(err)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<Parse> for TryFromParsed {
    type Error = error::DifferentVariant;

    fn try_from(err: Parse) -> Result<Self, Self::Error> {
        match err {
            Parse::TryFromParsed(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<ParseFromDescription> for Parse {
    fn from(err: ParseFromDescription) -> Self {
        Self::ParseFromDescription(err)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<Parse> for ParseFromDescription {
    type Error = error::DifferentVariant;

    fn try_from(err: Parse) -> Result<Self, Self::Error> {
        match err {
            Parse::ParseFromDescription(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<Parse> for crate::Error {
    fn from(err: Parse) -> Self {
        match err {
            Parse::TryFromParsed(err) => Self::TryFromParsed(err),
            Parse::ParseFromDescription(err) => Self::ParseFromDescription(err),
            Parse::UnexpectedTrailingCharacters => Self::UnexpectedTrailingCharacters,
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<crate::Error> for Parse {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ParseFromDescription(err) => Ok(Self::ParseFromDescription(err)),
            crate::Error::UnexpectedTrailingCharacters => Ok(Self::UnexpectedTrailingCharacters),
            crate::Error::TryFromParsed(err) => Ok(Self::TryFromParsed(err)),
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
fn rusty_test_4032() {
    rusty_monitor::set_test_id(4032);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i64_0: i64 = 23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 70u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 39i8;
    let mut i8_1: i8 = -72i8;
    let mut i8_2: i8 = -9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -195.578493f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_3: i8 = -19i8;
    let mut i8_4: i8 = -36i8;
    let mut i8_5: i8 = -38i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = 123i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = 17i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_6: i8 = -2i8;
    let mut i8_7: i8 = -28i8;
    let mut i8_8: i8 = -14i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -17i8;
    let mut i8_10: i8 = 29i8;
    let mut i8_11: i8 = -38i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 73u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 47u8;
    let mut u8_5: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_12: i8 = 92i8;
    let mut i8_13: i8 = -81i8;
    let mut i8_14: i8 = -31i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_2: u32 = 19u32;
    let mut u8_6: u8 = 77u8;
    let mut u8_7: u8 = 27u8;
    let mut u8_8: u8 = 84u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_15: i8 = 38i8;
    let mut i8_16: i8 = -41i8;
    let mut i8_17: i8 = -24i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i32_0: i32 = 33i32;
    let mut i64_3: i64 = 51i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut i8_18: i8 = -44i8;
    let mut i8_19: i8 = 47i8;
    let mut i8_20: i8 = -49i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i128_0: i128 = 34i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_21: i8 = 5i8;
    let mut i8_22: i8 = -15i8;
    let mut i8_23: i8 = -65i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_3: u32 = 3u32;
    let mut u8_9: u8 = 59u8;
    let mut u8_10: u8 = 9u8;
    let mut u8_11: u8 = 89u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_24: i8 = 74i8;
    let mut i8_25: i8 = 13i8;
    let mut i8_26: i8 = 64i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = -16i8;
    let mut i8_28: i8 = 3i8;
    let mut i8_29: i8 = -113i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i64_4: i64 = 235i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_1: i32 = -45i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_1);
    let mut i8_30: i8 = -80i8;
    let mut i8_31: i8 = -99i8;
    let mut i8_32: i8 = 23i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 22i8;
    let mut i8_34: i8 = -73i8;
    let mut i8_35: i8 = -35i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i64_5: i64 = 37i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i32_2: i32 = 73i32;
    let mut i64_6: i64 = -84i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_2);
    let mut i8_36: i8 = -69i8;
    let mut i8_37: i8 = -73i8;
    let mut i8_38: i8 = 8i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_12);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut f64_0: f64 = 169.366179f64;
    let mut i8_39: i8 = 12i8;
    let mut f32_1: f32 = -46.657613f32;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_7: i64 = -31i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut i32_3: i32 = -16i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_13);
    let mut time_5: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_5);
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i8_40: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_13);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    panic!("From RustyUnit with love");
}
}