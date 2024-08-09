//! Invalid format description

use alloc::string::String;
use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// The format description provided was not valid.
#[cfg_attr(
    __time_03_docs,
    doc(cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc")))
)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidFormatDescription {
    /// There was a bracket pair that was opened but not closed.
    #[non_exhaustive]
    UnclosedOpeningBracket {
        /// The zero-based index of the opening bracket.
        index: usize,
    },
    /// A component name is not valid.
    #[non_exhaustive]
    InvalidComponentName {
        /// The name of the invalid component name.
        name: String,
        /// The zero-based index the component name starts at.
        index: usize,
    },
    /// A modifier is not valid.
    #[non_exhaustive]
    InvalidModifier {
        /// The value of the invalid modifier.
        value: String,
        /// The zero-based index the modifier starts at.
        index: usize,
    },
    /// A component name is missing.
    #[non_exhaustive]
    MissingComponentName {
        /// The zero-based index where the component name should start.
        index: usize,
    },
}

#[cfg_attr(
    __time_03_docs,
    doc(cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc")))
)]
impl From<InvalidFormatDescription> for crate::Error {
    fn from(original: InvalidFormatDescription) -> Self {
        Self::InvalidFormatDescription(original)
    }
}

#[cfg_attr(
    __time_03_docs,
    doc(cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc")))
)]
impl TryFrom<crate::Error> for InvalidFormatDescription {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::InvalidFormatDescription(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

impl fmt::Display for InvalidFormatDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InvalidFormatDescription::*;
        match self {
            UnclosedOpeningBracket { index } => {
                write!(f, "unclosed opening bracket at byte index {}", index)
            }
            InvalidComponentName { name, index } => write!(
                f,
                "invalid component name `{}` at byte index {}",
                name, index
            ),
            InvalidModifier { value, index } => {
                write!(f, "invalid modifier `{}` at byte index {}", value, index)
            }
            MissingComponentName { index } => {
                write!(f, "missing component name at byte index {}", index)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidFormatDescription {}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8505() {
    rusty_monitor::set_test_id(8505);
    let mut i64_0: i64 = 10i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = 80i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_1: i64 = 72i64;
    let mut i64_2: i64 = 105i64;
    let mut i64_3: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i8_0: i8 = 30i8;
    let mut i8_1: i8 = 51i8;
    let mut i8_2: i8 = -1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 20i8;
    let mut i8_4: i8 = 59i8;
    let mut i8_5: i8 = -2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_0: i128 = -2i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 51i32;
    let mut i64_4: i64 = -155i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_5: i64 = -40i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_6: i64 = 54i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    panic!("From RustyUnit with love");
}
}