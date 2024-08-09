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
fn rusty_test_4555() {
    rusty_monitor::set_test_id(4555);
    let mut i128_0: i128 = 65i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = -62i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = 13i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut i64_2: i64 = -44i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = -48i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut u16_0: u16 = 25u16;
    let mut i32_1: i32 = -32i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_5);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -4i8;
    let mut i8_1: i8 = -84i8;
    let mut i8_2: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 71u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 39u8;
    let mut u8_2: u8 = 33u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 12u16;
    let mut i32_2: i32 = 54i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut i64_4: i64 = 155i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i32_3: i32 = 27i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 74u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 10u8;
    let mut u8_5: u8 = 28u8;
    let mut i64_5: i64 = -45i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i32_4: i32 = -124i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    panic!("From RustyUnit with love");
}
}