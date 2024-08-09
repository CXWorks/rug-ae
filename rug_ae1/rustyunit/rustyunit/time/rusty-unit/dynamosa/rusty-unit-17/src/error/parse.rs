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
fn rusty_test_987() {
    rusty_monitor::set_test_id(987);
    let mut i64_0: i64 = 51i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = 40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut f64_0: f64 = 81.499310f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = -90i8;
    let mut i8_1: i8 = 52i8;
    let mut i8_2: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = -28i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut f32_0: f32 = -141.480545f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -214i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut u32_0: u32 = 55u32;
    let mut u8_0: u8 = 62u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -27i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut u32_1: u32 = 28u32;
    let mut u8_3: u8 = 92u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 73u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -32i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_4: i64 = -79i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut u32_2: u32 = 6u32;
    let mut u8_6: u8 = 81u8;
    let mut u8_7: u8 = 6u8;
    let mut u8_8: u8 = 49u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_3: i32 = 36i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_2);
    let mut i64_5: i64 = 76i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i64_6: i64 = -77i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    panic!("From RustyUnit with love");
}
}