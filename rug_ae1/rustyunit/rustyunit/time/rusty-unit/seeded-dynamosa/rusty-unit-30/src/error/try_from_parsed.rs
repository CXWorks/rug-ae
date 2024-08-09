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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8804() {
//    rusty_monitor::set_test_id(8804);
    let mut str_0: &str = "Cannot represent a resulting duration in std. Try `let x = x - rhs;`, which will change the type.";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = 342i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -44i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = 119i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 24u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 3600i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 78u8;
    let mut u8_4: u8 = 8u8;
    let mut u8_5: u8 = 29u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut u16_0: u16 = 7u16;
    let mut i32_2: i32 = 128i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_3);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut i32_3: i32 = 3600i32;
    let mut i128_0: i128 = 1000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_4: i32 = 365i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}
}