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
fn rusty_test_8352() {
    rusty_monitor::set_test_id(8352);
    let mut i32_0: i32 = 2i32;
    let mut i64_0: i64 = 98i64;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut f32_0: f32 = -144.838189f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 98u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 46u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 76i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = -116i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i8_0: i8 = -63i8;
    let mut i8_1: i8 = -17i8;
    let mut i8_2: i8 = 28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -63i8;
    let mut i8_4: i8 = 118i8;
    let mut i8_5: i8 = 50i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = 121i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u32_1: u32 = 77u32;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 10u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = -142i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_2: i32 = 19i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_minutes(duration_2);
    let mut u16_0: u16 = crate::util::days_in_year(i32_0);
    panic!("From RustyUnit with love");
}
}