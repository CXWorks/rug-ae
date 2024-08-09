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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7863() {
    rusty_monitor::set_test_id(7863);
    let mut i32_0: i32 = -17i32;
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 21i8;
    let mut i8_4: i8 = 51i8;
    let mut i8_5: i8 = 101i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = -58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_6: i8 = -100i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = -104i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 22u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 86u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 6i8;
    let mut i8_10: i8 = -34i8;
    let mut i8_11: i8 = -8i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i8_12: i8 = -50i8;
    let mut i8_13: i8 = -120i8;
    let mut i8_14: i8 = 12i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -48i8;
    let mut i8_16: i8 = 28i8;
    let mut i8_17: i8 = 37i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 68u8;
    let mut u8_4: u8 = 53u8;
    let mut i8_18: i8 = 63i8;
    let mut i8_19: i8 = -38i8;
    let mut i8_20: i8 = -89i8;
    let mut i8_21: i8 = 65i8;
    let mut i8_22: i8 = -70i8;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_23: i8 = 112i8;
    let mut i8_24: i8 = -1i8;
    let mut i8_25: i8 = 27i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_22, i8_21, i8_20);
    let mut i8_26: i8 = -17i8;
    let mut i8_27: i8 = 73i8;
    let mut i8_28: i8 = -116i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_27, i8_25, i8_26);
    let mut i8_29: i8 = 111i8;
    let mut u32_2: u32 = 89u32;
    let mut u8_5: u8 = 43u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_4, u8_5, u8_3, u32_1);
    let mut u8_6: u8 = 55u8;
    let mut u8_7: u8 = 82u8;
    let mut i8_30: i8 = 125i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_30, i8_29);
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_18, i8_24, i8_28);
    let mut i8_31: i8 = 59i8;
    let mut u8_8: u8 = 22u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_32: i8 = 3i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_19, i8_32, i8_31);
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    panic!("From RustyUnit with love");
}
}