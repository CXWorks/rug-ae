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
fn rusty_test_4545() {
    rusty_monitor::set_test_id(4545);
    let mut i128_0: i128 = 0i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 105i32;
    let mut i64_0: i64 = -66i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = 128.752531f64;
    let mut i64_1: i64 = -35i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_1: i128 = 17i128;
    let mut bool_0: bool = true;
    let mut i64_2: i64 = 49i64;
    let mut i64_3: i64 = -55i64;
    let mut i64_4: i64 = 104i64;
    let mut str_0: &str = "hU2ljagFX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_2);
    let mut option_1: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut f64_1: f64 = crate::duration::Duration::as_seconds_f64(duration_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    panic!("From RustyUnit with love");
}
}