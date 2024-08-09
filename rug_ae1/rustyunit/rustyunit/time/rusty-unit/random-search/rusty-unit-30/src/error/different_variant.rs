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
fn rusty_test_110() {
    rusty_monitor::set_test_id(110);
    let mut i64_0: i64 = 106i64;
    let mut f64_0: f64 = -15.255768f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 128i32;
    let mut i64_1: i64 = 263i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -55i64;
    let mut i64_3: i64 = -126i64;
    let mut i64_4: i64 = 99i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_1, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    panic!("From RustyUnit with love");
}
}