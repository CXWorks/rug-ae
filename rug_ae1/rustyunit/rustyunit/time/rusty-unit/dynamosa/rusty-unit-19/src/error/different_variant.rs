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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6527() {
    rusty_monitor::set_test_id(6527);
    let mut u16_0: u16 = 79u16;
    let mut u16_1: u16 = 66u16;
    let mut u16_2: u16 = 10u16;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 5u8;
    let mut i64_0: i64 = -216i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = -39i32;
    let mut i32_1: i32 = 30i32;
    let mut i64_1: i64 = 37i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut i64_2: i64 = 69i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i32_2: i32 = -23i32;
    let mut i64_3: i64 = 88i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i128_0: i128 = -161i128;
    let mut i32_3: i32 = -57i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_3};
    let mut u8_3: u8 = crate::date::Date::iso_week(date_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i128_1: i128 = crate::duration::Duration::whole_microseconds(duration_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_0);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_7);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_2);
    panic!("From RustyUnit with love");
}
}