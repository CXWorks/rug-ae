//! Error formatting a struct

use core::convert::TryFrom;
use core::fmt;
use std::io;

use crate::error;

/// An error occurred when formatting.
#[non_exhaustive]
#[allow(missing_copy_implementations)]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
#[derive(Debug)]
pub enum Format {
    /// The type being formatted does not contain sufficient information to format a component.
    #[non_exhaustive]
    InsufficientTypeInformation,
    /// The component named has a value that cannot be formatted into the requested format.
    ///
    /// This variant is only returned when using well-known formats.
    InvalidComponent(&'static str),
    /// A value of `std::io::Error` was returned internally.
    StdIo(io::Error),
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientTypeInformation => f.write_str(
                "The type being formatted does not contain sufficient information to format a \
                 component.",
            ),
            Self::InvalidComponent(component) => write!(
                f,
                "The {} component cannot be formatted into the requested format.",
                component
            ),
            Self::StdIo(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Format {
    fn from(err: io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl TryFrom<Format> for io::Error {
    type Error = error::DifferentVariant;

    fn try_from(err: Format) -> Result<Self, Self::Error> {
        match err {
            Format::StdIo(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Format {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::InsufficientTypeInformation | Self::InvalidComponent(_) => None,
            Self::StdIo(ref err) => Some(err),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
impl From<Format> for crate::Error {
    fn from(original: Format) -> Self {
        Self::Format(original)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
impl TryFrom<crate::Error> for Format {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::Format(err) => Ok(err),
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
fn rusty_test_8297() {
    rusty_monitor::set_test_id(8297);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i32_0: i32 = -251i32;
    let mut i64_0: i64 = -107i64;
    let mut i8_0: i8 = -5i8;
    let mut i64_1: i64 = 89i64;
    let mut i32_1: i32 = -55i32;
    let mut i64_2: i64 = -104i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = 72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i8_1: i8 = 51i8;
    let mut i8_2: i8 = 76i8;
    let mut i8_3: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut f32_0: f32 = -38.884742f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = 11i32;
    let mut i64_5: i64 = -15i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_2);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_3: i32 = -41i32;
    let mut i64_6: i64 = 130i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_3);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i8_4: i8 = -76i8;
    let mut i8_5: i8 = 2i8;
    let mut i8_6: i8 = -38i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i32_4: i32 = -16i32;
    let mut i32_5: i32 = -5i32;
    let mut i64_7: i64 = 74i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_4);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 97u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 36u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_7: i8 = -89i8;
    let mut i8_8: i8 = 97i8;
    let mut i8_9: i8 = -94i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i128_0: i128 = 64i128;
    let mut i64_8: i64 = 12i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut i8_10: i8 = 52i8;
    let mut i8_11: i8 = 90i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_10, i8_0, i8_11);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u8_6: u8 = 2u8;
    let mut i32_6: i32 = -182i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_6, u8_6, weekday_0);
    panic!("From RustyUnit with love");
}
}