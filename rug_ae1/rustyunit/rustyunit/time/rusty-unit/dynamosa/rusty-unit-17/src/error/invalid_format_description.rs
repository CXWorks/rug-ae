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
fn rusty_test_2497() {
    rusty_monitor::set_test_id(2497);
    let mut i64_0: i64 = 101i64;
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 11i8;
    let mut i8_2: i8 = -103i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = -115i8;
    let mut i8_5: i8 = 19i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 71u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 110i32;
    let mut i64_1: i64 = 16i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut f64_0: f64 = 2.700652f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = 25i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_6: i8 = -74i8;
    let mut i8_7: i8 = -54i8;
    let mut i8_8: i8 = 106i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_1: i32 = -155i32;
    let mut i64_3: i64 = 36i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut f32_0: f32 = -68.236125f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = -156i32;
    let mut i64_4: i64 = 3i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i8_9: i8 = -69i8;
    let mut i8_10: i8 = 40i8;
    let mut i8_11: i8 = 53i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_5: i64 = 47i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = -8i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut i8_12: i8 = -84i8;
    let mut i8_13: i8 = 2i8;
    let mut i8_14: i8 = -94i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_7: i64 = -190i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i64_8: i64 = -5i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut i64_9: i64 = -152i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut u32_1: u32 = 73u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 63u8;
    let mut u8_5: u8 = 71u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_15: i8 = 34i8;
    let mut i8_16: i8 = -31i8;
    let mut i8_17: i8 = -98i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_15: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_10: i64 = -85i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_16, duration_15);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut i64_11: i64 = 155i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::abs(duration_19);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_20);
    let mut i64_12: i64 = 6i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::minutes(i64_12);
    let mut i64_13: i64 = 40i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds(i64_13);
    let mut i8_18: i8 = -106i8;
    let mut i8_19: i8 = -77i8;
    let mut i8_20: i8 = -13i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i128_0: i128 = -11i128;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_26: std::time::Duration = crate::duration::Duration::abs_std(duration_25);
    let mut i64_14: i64 = 127i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::days(i64_14);
    let mut duration_28: std::time::Duration = crate::duration::Duration::abs_std(duration_27);
    let mut i8_21: i8 = -50i8;
    let mut i8_22: i8 = 5i8;
    let mut i8_23: i8 = -99i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i64_15: i64 = 81i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::minutes(i64_15);
    let mut u32_2: u32 = 15u32;
    let mut u8_6: u8 = 13u8;
    let mut u8_7: u8 = 23u8;
    let mut u8_8: u8 = 90u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    panic!("From RustyUnit with love");
}
}