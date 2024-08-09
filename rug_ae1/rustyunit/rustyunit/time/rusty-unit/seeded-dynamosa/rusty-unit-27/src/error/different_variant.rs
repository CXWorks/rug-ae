//! Different variant error

use core::convert::TryFrom;
use core::fmt;

/// An error type indicating that a [`TryFrom`](core::convert::TryFrom) call failed because the
/// original value was of a different variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentVariant;

impl fmt::Display for DifferentVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was of a different variant than required")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DifferentVariant {}

impl From<DifferentVariant> for crate::Error {
    fn from(err: DifferentVariant) -> Self {
        Self::DifferentVariant(err)
    }
}

impl TryFrom<crate::Error> for DifferentVariant {
    type Error = Self;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::DifferentVariant(err) => Ok(err),
            _ => Err(Self),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::convert::TryFrom;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_103() {
//    rusty_monitor::set_test_id(103);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 83u8;
    let mut u8_5: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 0i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_0: i16 = 0i16;
    let mut i64_0: i64 = -74i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = 161i32;
    let mut i64_1: i64 = 1000000000i64;
    let mut u32_2: u32 = 999999999u32;
    let mut u32_3: u32 = 10u32;
    let mut u8_6: u8 = 0u8;
    let mut u8_7: u8 = 60u8;
    let mut u8_8: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_2);
    let mut i32_2: i32 = crate::primitive_date_time::PrimitiveDateTime::to_julian_day(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_156() {
//    rusty_monitor::set_test_id(156);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 60i64;
    let mut i64_1: i64 = -134i64;
    let mut i64_2: i64 = 86400i64;
    let mut str_0: &str = "date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 12i64;
    let mut i64_4: i64 = 1i64;
    let mut i64_5: i64 = 253402300799i64;
    let mut str_1: &str = "IHW";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_1: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 2440588i64;
    let mut i64_7: i64 = 604800i64;
    let mut i64_8: i64 = 253402300799i64;
    let mut str_2: &str = "Source value is out of range for the target type";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut error_2: error::Error = crate::error::Error::ComponentRange(componentrange_2);
    let mut bool_3: bool = true;
    let mut i64_9: i64 = 86400i64;
    let mut i64_10: i64 = 86400i64;
    let mut i64_11: i64 = 3600i64;
    let mut str_3: &str = "second";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut error_3: error::Error = crate::error::Error::ComponentRange(componentrange_3);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_3);
    let mut result_1: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_2);
    let mut result_2: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_1);
    let mut result_3: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
//    panic!("From RustyUnit with love");
}
}