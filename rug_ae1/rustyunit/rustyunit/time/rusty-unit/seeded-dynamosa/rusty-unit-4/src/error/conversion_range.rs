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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_374() {
//    rusty_monitor::set_test_id(374);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 23u8;
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 8u8;
    let mut u32_2: u32 = 10u32;
    let mut u8_6: u8 = 4u8;
    let mut u8_7: u8 = 23u8;
    let mut u8_8: u8 = 30u8;
    let mut u32_3: u32 = 92u32;
    let mut u8_9: u8 = 60u8;
    let mut u8_10: u8 = 81u8;
    let mut u8_11: u8 = 4u8;
    let mut u32_4: u32 = 1000u32;
    let mut u8_12: u8 = 53u8;
    let mut u8_13: u8 = 10u8;
    let mut u8_14: u8 = 24u8;
    let mut u32_5: u32 = 1000000000u32;
    let mut u8_15: u8 = 8u8;
    let mut u8_16: u8 = 6u8;
    let mut u8_17: u8 = 23u8;
    let mut u32_6: u32 = 100u32;
    let mut u8_18: u8 = 0u8;
    let mut u8_19: u8 = 31u8;
    let mut u8_20: u8 = 31u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_20, u8_19, u8_18, u32_6);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_17, u8_16, u8_15, u32_5);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_14, u8_13, u8_12, u32_4);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_11, u8_10, u8_9, u32_3);
    let mut result_4: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_8, u8_7, u8_6, u32_2);
    let mut result_5: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut result_6: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_253() {
//    rusty_monitor::set_test_id(253);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 86400i64;
    let mut i64_1: i64 = 3600i64;
    let mut i64_2: i64 = 1000000i64;
    let mut str_0: &str = "August";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 1000i64;
    let mut i64_4: i64 = 1000i64;
    let mut i64_5: i64 = 21i64;
    let mut str_1: &str = "Instant";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_1: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 9223372036854775807i64;
    let mut i64_7: i64 = 60i64;
    let mut i64_8: i64 = 2147483647i64;
    let mut str_2: &str = "October";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut error_2: error::Error = crate::error::Error::ComponentRange(componentrange_2);
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 60i64;
    let mut i64_10: i64 = 55i64;
    let mut i64_11: i64 = 24i64;
    let mut str_3: &str = "value";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut error_3: error::Error = crate::error::Error::ComponentRange(componentrange_3);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_3);
    let mut result_1: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_2);
    let mut result_2: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_1);
    let mut result_3: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
//    panic!("From RustyUnit with love");
}
}