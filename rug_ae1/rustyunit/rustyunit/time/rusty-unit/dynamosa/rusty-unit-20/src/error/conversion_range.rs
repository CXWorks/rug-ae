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
	use std::convert::TryFrom;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6124() {
    rusty_monitor::set_test_id(6124);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 27u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -21i32;
    let mut i64_0: i64 = 43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_1: i64 = -102i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut i32_2: i32 = -8i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 30u32;
    let mut u8_3: u8 = 6u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 27u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_0: i8 = 9i8;
    let mut i8_1: i8 = -29i8;
    let mut i8_2: i8 = -40i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_1);
    let mut bool_0: bool = false;
    let mut i64_3: i64 = 34i64;
    let mut i64_4: i64 = -107i64;
    let mut i64_5: i64 = 60i64;
    let mut str_0: &str = "ZKN";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5808() {
    rusty_monitor::set_test_id(5808);
    let mut i64_0: i64 = 212i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_0: u32 = 27u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -21i32;
    let mut i64_1: i64 = 43i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut i32_1: i32 = -148i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i64_2: i64 = -102i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = -37i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_3);
    let mut u32_1: u32 = 71u32;
    let mut u8_3: u8 = 48u8;
    let mut u8_4: u8 = 15u8;
    let mut u8_5: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = -8i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_2: u32 = 30u32;
    let mut u8_6: u8 = 6u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 27u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_3: i64 = -14i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_0: i8 = 9i8;
    let mut i8_1: i8 = -29i8;
    let mut i8_2: i8 = -40i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_6, time: time_2};
    let mut bool_0: bool = false;
    let mut i64_4: i64 = 34i64;
    let mut i64_5: i64 = -107i64;
    let mut i64_6: i64 = 60i64;
    let mut str_0: &str = "ZKN";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_minutes(duration_0);
    let mut conversionrange_0: crate::error::conversion_range::ConversionRange = std::result::Result::unwrap(result_0);
    let mut u8_9: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_4);
    panic!("From RustyUnit with love");
}
}