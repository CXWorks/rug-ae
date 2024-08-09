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
fn rusty_test_4664() {
    rusty_monitor::set_test_id(4664);
    let mut i64_0: i64 = 10i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = 142i32;
    let mut i64_1: i64 = 9i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -89i8;
    let mut i8_1: i8 = -13i8;
    let mut i8_2: i8 = -1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -42i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 61i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_1: i32 = -133i32;
    let mut i64_4: i64 = -5i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_3: i8 = -49i8;
    let mut i8_4: i8 = 47i8;
    let mut i8_5: i8 = 74i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -27i8;
    let mut i8_7: i8 = -37i8;
    let mut i8_8: i8 = -106i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 14u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = -3i8;
    let mut i8_10: i8 = -82i8;
    let mut i8_11: i8 = 51i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 25u32;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 81u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_5: i64 = -9i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i8_12: i8 = 33i8;
    let mut i8_13: i8 = -8i8;
    let mut i8_14: i8 = 106i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f64_0: f64 = 43.059350f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_15: i8 = 58i8;
    let mut i8_16: i8 = 32i8;
    let mut i8_17: i8 = -12i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 47i8;
    let mut i8_19: i8 = -113i8;
    let mut i8_20: i8 = 11i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i32_2: i32 = -26i32;
    let mut i64_6: i64 = -183i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut i8_21: i8 = -25i8;
    let mut i8_22: i8 = 86i8;
    let mut i8_23: i8 = 96i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_2: u32 = 32u32;
    let mut u8_6: u8 = 76u8;
    let mut u8_7: u8 = 20u8;
    let mut u8_8: u8 = 15u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u8_9: u8 = crate::date::Date::day(date_0);
    panic!("From RustyUnit with love");
}
}