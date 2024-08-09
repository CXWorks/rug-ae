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
#[timeout(30000)]fn rusty_test_9062() {
//    rusty_monitor::set_test_id(9062);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut i128_1: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i32_0: i32 = 291i32;
    let mut i64_1: i64 = 604800i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = -139i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u16_0: u16 = 59u16;
    let mut i32_1: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i32_2: i32 = 4i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_3: i32 = 195i32;
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut i64_4: i64 = 253402300799i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i8_0: i8 = 39i8;
    let mut i8_1: i8 = 3i8;
    let mut i64_5: i64 = 604800i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i8_2: i8 = 23i8;
    let mut i8_3: i8 = -11i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_6: i64 = 12i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i128_2: i128 = 1000i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_7: i64 = -97i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 79i8;
    let mut i8_6: i8 = 6i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_3, i8_7);
    let mut i8_9: i8 = 21i8;
    let mut i8_10: i8 = 5i8;
    let mut i8_11: i8 = 127i8;
    let mut i64_8: i64 = 1000000000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_6);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_9, i8_10);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_11);
    let mut i32_4: i32 = 122i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_2_ref_0: &crate::date::Date = &mut date_2;
    let mut u16_1: u16 = 367u16;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_3_ref_0: &crate::date::Date = &mut date_3;
//    panic!("From RustyUnit with love");
}
}