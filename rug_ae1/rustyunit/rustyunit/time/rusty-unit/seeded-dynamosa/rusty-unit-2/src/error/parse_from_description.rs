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
#[timeout(30000)]fn rusty_test_3300() {
//    rusty_monitor::set_test_id(3300);
    let mut i64_0: i64 = 2147483647i64;
    let mut i64_1: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 160i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 400i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i128_0: i128 = 0i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_1: i32 = 43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = 195i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = -38i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_2: u32 = 999999u32;
    let mut u8_6: u8 = 28u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 2u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_3: i64 = 39i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i32_3: i32 = -140i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut i64_4: i64 = 2147483647i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = crate::duration::Duration::whole_weeks(duration_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
//    panic!("From RustyUnit with love");
}
}