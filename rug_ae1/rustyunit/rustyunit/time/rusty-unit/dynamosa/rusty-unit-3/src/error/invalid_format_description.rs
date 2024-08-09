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
fn rusty_test_6432() {
    rusty_monitor::set_test_id(6432);
    let mut i32_0: i32 = -125i32;
    let mut i32_1: i32 = 0i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut i32_2: i32 = 66i32;
    let mut i8_0: i8 = 17i8;
    let mut i8_1: i8 = 46i8;
    let mut i8_2: i8 = 123i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 108i8;
    let mut i8_4: i8 = -83i8;
    let mut i8_5: i8 = 49i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = 2i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_6: i8 = 48i8;
    let mut i8_7: i8 = 25i8;
    let mut i8_8: i8 = -31i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = -10i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = 235i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i32_3: i32 = -15i32;
    let mut i64_3: i64 = 43i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut i32_4: i32 = -59i32;
    let mut i64_4: i64 = 14i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut i128_0: i128 = -101i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_5: i64 = -192i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut u32_0: u32 = 57u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 33u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 7i8;
    let mut i8_10: i8 = -54i8;
    let mut i8_11: i8 = -98i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f64_0: f64 = -110.901507f64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 73u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut i64_6: i64 = 177i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i8_12: i8 = 91i8;
    let mut i8_13: i8 = 4i8;
    let mut i8_14: i8 = -45i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_7: i64 = 42i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut i128_1: i128 = 200i128;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_15: i8 = -52i8;
    let mut i8_16: i8 = -16i8;
    let mut i8_17: i8 = -95i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = std::result::Result::unwrap(result_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_0);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    panic!("From RustyUnit with love");
}
}