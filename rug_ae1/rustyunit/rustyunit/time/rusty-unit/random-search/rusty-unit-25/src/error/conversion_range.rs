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
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4482() {
    rusty_monitor::set_test_id(4482);
    let mut i32_0: i32 = 82i32;
    let mut i64_0: i64 = 100i64;
    let mut u32_0: u32 = 23u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 70u8;
    let mut u8_2: u8 = 97u8;
    let mut i8_0: i8 = 89i8;
    let mut i8_1: i8 = -110i8;
    let mut i8_2: i8 = 15i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 30i64;
    let mut i64_2: i64 = 5i64;
    let mut i64_3: i64 = -10i64;
    let mut str_0: &str = "HsU3ScdV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut conversionrange_0: crate::error::conversion_range::ConversionRange = std::result::Result::unwrap(result_0);
    let mut conversionrange_0_ref_0: &crate::error::conversion_range::ConversionRange = &mut conversionrange_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(conversionrange_0_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    panic!("From RustyUnit with love");
}
}