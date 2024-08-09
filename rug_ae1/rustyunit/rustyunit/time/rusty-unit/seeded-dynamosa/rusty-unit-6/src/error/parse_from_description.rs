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
#[timeout(30000)]fn rusty_test_7529() {
//    rusty_monitor::set_test_id(7529);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 303i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    let mut i32_1: i32 = 100i32;
    let mut u16_0: u16 = 1u16;
    let mut i32_2: i32 = 3652425i32;
    let mut u8_0: u8 = 6u8;
    let mut i32_3: i32 = 195i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_4: i32 = -55i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_5: i32 = 20i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_2);
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 12u8;
    let mut i64_2: i64 = 3600i64;
    let mut i64_3: i64 = 131i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_4: i64 = 60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_6: i32 = 3i32;
    let mut i64_5: i64 = 1000000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_6);
    let mut i64_6: i64 = 1000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_7: i64 = 7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut i64_8: i64 = 12i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_9: i64 = -10i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut u16_1: u16 = 1u16;
    let mut i32_7: i32 = 280i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_8);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut u16_2: u16 = 367u16;
    let mut i32_8: i32 = 25i32;
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut date_9: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_12);
    let mut u16_3: u16 = 367u16;
    let mut date_10: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_3);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_9);
    let mut option_1: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_8);
    let mut option_2: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_5);
    let mut option_3: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_6);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_2);
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_1, month_0);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_0);
//    panic!("From RustyUnit with love");
}
}