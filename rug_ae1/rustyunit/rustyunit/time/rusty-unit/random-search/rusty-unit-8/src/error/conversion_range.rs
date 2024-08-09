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
	use std::clone::Clone;
	use std::convert::TryFrom;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut i32_0: i32 = -113i32;
    let mut i64_0: i64 = 2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u16_0: u16 = 57u16;
    let mut u8_0: u8 = 33u8;
    let mut u8_1: u8 = 13u8;
    let mut u8_2: u8 = 16u8;
    let mut i32_1: i32 = -137i32;
    let mut i64_1: i64 = -64i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -122i64;
    let mut i64_3: i64 = -69i64;
    let mut i64_4: i64 = 33i64;
    let mut str_0: &str = "hdmphyg";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut conversionrange_0: crate::error::conversion_range::ConversionRange = std::result::Result::unwrap(result_0);
    let mut bool_1: bool = crate::duration::Duration::is_zero(duration_1);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut conversionrange_0_ref_0: &crate::error::conversion_range::ConversionRange = &mut conversionrange_0;
    let mut conversionrange_1: crate::error::conversion_range::ConversionRange = std::clone::Clone::clone(conversionrange_0_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4772() {
    rusty_monitor::set_test_id(4772);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -23i64;
    let mut i64_1: i64 = 177i64;
    let mut i64_2: i64 = -39i64;
    let mut str_0: &str = "snYc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut i32_0: i32 = 218i32;
    let mut i64_3: i64 = -37i64;
    let mut f64_0: f64 = -14.038500f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u16_0: u16 = 1u16;
    let mut i32_1: i32 = 63i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut u32_0: u32 = 80u32;
    let mut f64_1: f64 = 60.629629f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    panic!("From RustyUnit with love");
}
}