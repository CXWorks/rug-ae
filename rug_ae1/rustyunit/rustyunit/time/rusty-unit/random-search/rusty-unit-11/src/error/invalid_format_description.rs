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
fn rusty_test_4865() {
    rusty_monitor::set_test_id(4865);
    let mut i64_0: i64 = 135i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = -42i8;
    let mut i8_1: i8 = 11i8;
    let mut i8_2: i8 = 101i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 72i8;
    let mut i8_4: i8 = -96i8;
    let mut i8_5: i8 = 55i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 72i32;
    let mut i64_1: i64 = 157i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i8_6: i8 = 8i8;
    let mut i8_7: i8 = 20i8;
    let mut i8_8: i8 = -55i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -74i8;
    let mut i8_10: i8 = -85i8;
    let mut i8_11: i8 = 17i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_12: i8 = -55i8;
    let mut i8_13: i8 = 52i8;
    let mut i8_14: i8 = 26i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -113i8;
    let mut i8_16: i8 = -71i8;
    let mut i8_17: i8 = -19i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_1: u32 = 65u32;
    let mut u8_3: u8 = 87u8;
    let mut u8_4: u8 = 63u8;
    let mut u8_5: u8 = 43u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_18: i8 = -27i8;
    let mut i8_19: i8 = -56i8;
    let mut i8_20: i8 = 92i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i32_1: i32 = -26i32;
    let mut i64_2: i64 = 18i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i64_3: i64 = 112i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i8_21: i8 = -7i8;
    let mut i8_22: i8 = 80i8;
    let mut i8_23: i8 = 33i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f64_0: f64 = -51.464829f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 80i32;
    let mut i64_4: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut i32_3: i32 = 182i32;
    let mut i64_5: i64 = 65i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut u32_2: u32 = 31u32;
    let mut u8_6: u8 = 67u8;
    let mut u8_7: u8 = 10u8;
    let mut u8_8: u8 = 46u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_24: i8 = -114i8;
    let mut i8_25: i8 = 4i8;
    let mut i8_26: i8 = -84i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = -92i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i8_27: i8 = 19i8;
    let mut i8_28: i8 = 89i8;
    let mut i8_29: i8 = 0i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 30u32;
    let mut u8_9: u8 = 97u8;
    let mut u8_10: u8 = 22u8;
    let mut u8_11: u8 = 10u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_30: i8 = -27i8;
    let mut i8_31: i8 = -48i8;
    let mut i8_32: i8 = 29i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i64_7: i64 = -24i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut i8_33: i8 = -31i8;
    let mut i8_34: i8 = 37i8;
    let mut i8_35: i8 = -14i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = -16i8;
    let mut i8_37: i8 = -77i8;
    let mut i8_38: i8 = 87i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i32_4: i32 = 31i32;
    let mut i64_8: i64 = 89i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_4);
    let mut i8_39: i8 = 33i8;
    let mut i8_40: i8 = -10i8;
    let mut i8_41: i8 = -87i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u32_4: u32 = 92u32;
    let mut u8_12: u8 = 60u8;
    let mut u8_13: u8 = 38u8;
    let mut u8_14: u8 = 34u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i128_0: i128 = 110i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_42: i8 = -53i8;
    let mut i8_43: i8 = -35i8;
    let mut i8_44: i8 = -113i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut f32_0: f32 = -209.354280f32;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_45: i8 = 75i8;
    let mut i8_46: i8 = -13i8;
    let mut i8_47: i8 = -26i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i8_48: i8 = -71i8;
    let mut i8_49: i8 = 97i8;
    let mut i8_50: i8 = -55i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i128_1: i128 = 94i128;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_5: u32 = 8u32;
    let mut u8_15: u8 = 38u8;
    let mut u8_16: u8 = 2u8;
    let mut u8_17: u8 = 11u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i8_51: i8 = -115i8;
    let mut i8_52: i8 = 21i8;
    let mut i8_53: i8 = -25i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut i64_9: i64 = -43i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::microseconds(i64_9);
    let mut u32_6: u32 = 96u32;
    let mut u8_18: u8 = 24u8;
    let mut u8_19: u8 = 74u8;
    let mut u8_20: u8 = 78u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i8_54: i8 = 6i8;
    let mut i8_55: i8 = 32i8;
    let mut i8_56: i8 = 54i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut i64_10: i64 = -127i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::days(i64_10);
    let mut i8_57: i8 = 66i8;
    let mut i8_58: i8 = 113i8;
    let mut i8_59: i8 = 70i8;
    let mut utcoffset_19: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_59, i8_58, i8_57);
    let mut i8_60: i8 = -31i8;
    let mut i8_61: i8 = -121i8;
    let mut i8_62: i8 = -75i8;
    let mut utcoffset_20: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_62, i8_61, i8_60);
    let mut i8_63: i8 = 17i8;
    let mut i8_64: i8 = -117i8;
    let mut i8_65: i8 = -128i8;
    let mut utcoffset_21: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_65, i8_64, i8_63);
    let mut u32_7: u32 = 33u32;
    let mut u8_21: u8 = 83u8;
    let mut u8_22: u8 = 33u8;
    let mut u8_23: u8 = 31u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut i32_5: i32 = 110i32;
    let mut i64_11: i64 = -63i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new(i64_11, i32_5);
    let mut f32_1: f32 = 3.194902f32;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    panic!("From RustyUnit with love");
}
}