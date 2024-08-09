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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_310() {
//    rusty_monitor::set_test_id(310);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -79i8;
    let mut i8_1: i8 = 46i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 30u8;
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_2: u32 = 100000u32;
    let mut u8_6: u8 = 60u8;
    let mut u8_7: u8 = 98u8;
    let mut u8_8: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_1: i32 = 82i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_3: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_3);
    let mut u32_4: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_2);
    let mut u32_5: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_437() {
//    rusty_monitor::set_test_id(437);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_3: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_4: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_5: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_6: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_7: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_8: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_9: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_10: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_11: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_12: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_13: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_14: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_15: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_16: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_17: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_18: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_19: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_20: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_21: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_22: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_23: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_24: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_25: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_26: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_27: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_28: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_29: duration::Padding = crate::duration::Padding::Optimize;
//    panic!("From RustyUnit with love");
}
}