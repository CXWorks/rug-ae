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
fn rusty_test_7745() {
    rusty_monitor::set_test_id(7745);
    let mut i64_0: i64 = -14i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i128_0: i128 = -130i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i32_0: i32 = 87i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut u16_0: u16 = 69u16;
    let mut i32_1: i32 = -12i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i32_2: i32 = -2i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = -63i8;
    let mut i8_2: i8 = -118i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_3: i32 = -164i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_4: i32 = 17i32;
    let mut i64_1: i64 = -13i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut u16_1: u16 = 14u16;
    let mut i32_5: i32 = 134i32;
    let mut i64_2: i64 = -163i64;
    let mut f64_0: f64 = 52.531481f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 17u8;
    let mut u8_2: u8 = 74u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_6, time_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i32_6: i32 = 100i32;
    let mut date_8: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut u32_1: u32 = 79u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 60u8;
    let mut u8_5: u8 = 22u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = -87i8;
    let mut i8_5: i8 = 63i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_5, utcoffset_2);
    let mut date_9: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut date_10: crate::date::Date = crate::date::Date::saturating_sub(date_9, duration_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_10, time: time_2};
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_4, date_8);
    let mut i32_7: i32 = 88i32;
    let mut i64_3: i64 = -41i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u32_2: u32 = 12u32;
    let mut u8_6: u8 = 51u8;
    let mut u8_7: u8 = 16u8;
    let mut u8_8: u8 = 15u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut f64_1: f64 = 152.695896f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_8: i32 = -24i32;
    let mut date_11: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut date_12: crate::date::Date = crate::date::Date::saturating_add(date_11, duration_7);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_12, time: time_3};
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_6, duration_6);
    let mut i8_6: i8 = 84i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = -67i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_4: i64 = 109i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut i64_5: i64 = 61i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_12);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut u16_2: u16 = 70u16;
    let mut i32_9: i32 = -37i32;
    let mut date_13: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_9, u16_2);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_13, time_4);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_8, duration_10);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_5, duration_0);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_7);
    panic!("From RustyUnit with love");
}
}