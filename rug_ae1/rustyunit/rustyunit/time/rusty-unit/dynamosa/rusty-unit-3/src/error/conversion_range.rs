//! Conversion range error

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// An error type indicating that a conversion failed because the target type could not store the
/// initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRange;

impl fmt::Display for ConversionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionRange {}

impl From<ConversionRange> for crate::Error {
    fn from(err: ConversionRange) -> Self {
        Self::ConversionRange(err)
    }
}

impl TryFrom<crate::Error> for ConversionRange {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ConversionRange(err) => Ok(err),
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
fn rusty_test_628() {
    rusty_monitor::set_test_id(628);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 49i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i64_0: i64 = -89i64;
    let mut f32_0: f32 = 195.381200f32;
    let mut u32_0: u32 = 79u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 91u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_1: i32 = -93i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_2: i64 = -245i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_3: i64 = 125i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_4: i64 = -26i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_5: i64 = -30i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_1: month::Month = crate::month::Month::July;
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    panic!("From RustyUnit with love");
}
}