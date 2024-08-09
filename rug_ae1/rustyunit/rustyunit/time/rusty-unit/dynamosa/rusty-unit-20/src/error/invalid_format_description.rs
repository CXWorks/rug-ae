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
fn rusty_test_7153() {
    rusty_monitor::set_test_id(7153);
    let mut i32_0: i32 = 122i32;
    let mut i64_0: i64 = 303i64;
    let mut i8_0: i8 = 17i8;
    let mut i8_1: i8 = 56i8;
    let mut i8_2: i8 = 81i8;
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 74u32;
    let mut u8_3: u8 = 27u8;
    let mut u8_4: u8 = 77u8;
    let mut u8_5: u8 = 73u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 59u16;
    let mut i32_1: i32 = 71i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_2: i32 = -248i32;
    let mut i32_3: i32 = 44i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i64_1: i64 = -34i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_3: i8 = -91i8;
    let mut i8_4: i8 = 15i8;
    let mut i8_5: i8 = 87i8;
    let mut i32_4: i32 = -91i32;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_4);
    let mut i32_5: i32 = 57i32;
    let mut i64_2: i64 = 154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut i32_6: i32 = -207i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_3, duration_3);
    let mut i8_6: i8 = 8i8;
    let mut i8_7: i8 = 10i8;
    let mut i8_8: i8 = 102i8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    panic!("From RustyUnit with love");
}
}