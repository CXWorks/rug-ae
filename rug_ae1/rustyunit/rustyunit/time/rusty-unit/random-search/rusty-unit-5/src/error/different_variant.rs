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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1643() {
    rusty_monitor::set_test_id(1643);
    let mut i64_0: i64 = -144i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 25i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut u16_0: u16 = 59u16;
    let mut i32_0: i32 = 14i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut f32_0: f32 = -6.292746f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i8_0: i8 = -59i8;
    let mut i8_1: i8 = 73i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = 19i64;
    let mut i64_3: i64 = -52i64;
    let mut i64_4: i64 = 41i64;
    let mut str_0: &str = "FFJsDj6BxrkbD6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut f32_1: f32 = -36.338730f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_5);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut differentvariant_0: crate::error::different_variant::DifferentVariant = std::result::Result::unwrap(result_0);
    let mut differentvariant_0_ref_0: &crate::error::different_variant::DifferentVariant = &mut differentvariant_0;
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut bool_2: bool = crate::duration::Duration::is_negative(duration_4);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    panic!("From RustyUnit with love");
}
}