//! Error converting a [`Parsed`](crate::parsing::Parsed) struct to another type

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// An error that occurred when converting a [`Parsed`](crate::parsing::Parsed) to another type.
#[non_exhaustive]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromParsed {
    /// The [`Parsed`](crate::parsing::Parsed) did not include enough information to construct the
    /// type.
    InsufficientInformation,
    /// Some component contained an invalid value for the type.
    ComponentRange(error::ComponentRange),
}

impl fmt::Display for TryFromParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientInformation => f.write_str(
                "the `Parsed` struct did not include enough information to construct the type",
            ),
            Self::ComponentRange(err) => err.fmt(f),
        }
    }
}

impl From<error::ComponentRange> for TryFromParsed {
    fn from(v: error::ComponentRange) -> Self {
        Self::ComponentRange(v)
    }
}

impl TryFrom<TryFromParsed> for error::ComponentRange {
    type Error = error::DifferentVariant;

    fn try_from(err: TryFromParsed) -> Result<Self, Self::Error> {
        match err {
            TryFromParsed::ComponentRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromParsed {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InsufficientInformation => None,
            Self::ComponentRange(err) => Some(err),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<TryFromParsed> for crate::Error {
    fn from(original: TryFromParsed) -> Self {
        Self::TryFromParsed(original)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<crate::Error> for TryFromParsed {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::TryFromParsed(err) => Ok(err),
            _ => Err(error::DifferentVariant),
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
fn rusty_test_2859() {
    rusty_monitor::set_test_id(2859);
    let mut u32_0: u32 = 59u32;
    let mut i8_0: i8 = 38i8;
    let mut i8_1: i8 = -21i8;
    let mut i8_2: i8 = -1i8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_3: i8 = -84i8;
    let mut i8_4: i8 = 12i8;
    let mut i8_5: i8 = 34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = 82i32;
    let mut i64_0: i64 = 52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u32_1: u32 = 85u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 43u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i8_6: i8 = -70i8;
    let mut i8_7: i8 = 52i8;
    let mut i8_8: i8 = -63i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -29i8;
    let mut i8_10: i8 = -4i8;
    let mut i8_11: i8 = 36i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_1: i64 = -226i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 47i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_12: i8 = 101i8;
    let mut i8_13: i8 = 11i8;
    let mut i8_14: i8 = 125i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = 75i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -51i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i8_15: i8 = -4i8;
    let mut i8_16: i8 = 101i8;
    let mut i8_17: i8 = 83i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 68u32;
    let mut u8_3: u8 = 89u8;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 16u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_5: i64 = -114i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i32_1: i32 = -104i32;
    let mut i64_6: i64 = 84i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_1);
    let mut i64_7: i64 = 46i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i8_18: i8 = -10i8;
    let mut i8_19: i8 = -67i8;
    let mut i8_20: i8 = -61i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_8: i64 = 1i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut i64_9: i64 = 94i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut i8_21: i8 = 12i8;
    let mut i8_22: i8 = -48i8;
    let mut i8_23: i8 = 83i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_3: u32 = 76u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 15u8;
    let mut u8_8: u8 = 50u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut i32_2: i32 = -98i32;
    let mut i64_10: i64 = 44i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_2);
    let mut i64_11: i64 = 28i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::hours(i64_11);
    let mut i64_12: i64 = 5i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::microseconds(i64_12);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i8_24: i8 = -85i8;
    let mut i8_25: i8 = -79i8;
    let mut i8_26: i8 = 11i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_3: i32 = 195i32;
    let mut i64_13: i64 = -195i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new(i64_13, i32_3);
    let mut i64_14: i64 = -19i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut i8_27: i8 = -27i8;
    let mut i8_28: i8 = -16i8;
    let mut i8_29: i8 = -81i8;
    let mut i128_0: i128 = 311i128;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_4: i32 = -1i32;
    let mut i32_5: i32 = 129i32;
    let mut i64_15: i64 = -86i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_15, i32_5);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_21, i32_4);
    let mut i64_16: i64 = 16i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::hours(i64_16);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_23);
    let mut i32_6: i32 = 34i32;
    let mut i64_17: i64 = -98i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_17, i32_6);
    let mut duration_26: std::time::Duration = crate::duration::Duration::abs_std(duration_25);
    let mut i64_18: i64 = 164i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::microseconds(i64_18);
    let mut i64_19: i64 = 123i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::microseconds(i64_19);
    let mut i8_30: i8 = 64i8;
    let mut i8_31: i8 = 42i8;
    let mut i8_32: i8 = -39i8;
    let mut i8_33: i8 = 30i8;
    let mut i8_34: i8 = 13i8;
    let mut i8_35: i8 = -46i8;
    let mut i32_7: i32 = -44i32;
    let mut i64_20: i64 = 298i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_20, i32_7);
    let mut u32_4: u32 = 19u32;
    let mut u8_9: u8 = 41u8;
    let mut u8_10: u8 = 86u8;
    let mut u8_11: u8 = 11u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_4);
    let mut i8_36: i8 = -77i8;
    let mut i8_37: i8 = 6i8;
    let mut i8_38: i8 = -90i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i32_8: i32 = 58i32;
    let mut i64_21: i64 = -13i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::new(i64_21, i32_8);
    let mut duration_31: std::time::Duration = crate::duration::Duration::abs_std(duration_30);
    let mut i32_9: i32 = -48i32;
    let mut i64_22: i64 = -162i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::hours(i64_22);
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_32, i32_9);
    let mut u32_5: u32 = 38u32;
    let mut u8_12: u8 = 9u8;
    let mut u8_13: u8 = 6u8;
    let mut u8_14: u8 = 52u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_5);
    let mut i64_23: i64 = -38i64;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_23);
    let mut duration_35: std::time::Duration = crate::duration::Duration::abs_std(duration_34);
    let mut f64_0: f64 = -117.529585f64;
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_24: i64 = -81i64;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::minutes(i64_24);
    let mut i64_25: i64 = -25i64;
    let mut duration_38: crate::duration::Duration = crate::duration::Duration::microseconds(i64_25);
    panic!("From RustyUnit with love");
}
}