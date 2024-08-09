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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3508() {
//    rusty_monitor::set_test_id(3508);
    let mut str_0: &str = "eZPEEUnuVnhtj";
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 1721119i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 229i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_1: i32 = 212i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = 1000000i64;
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut str_0_ref_0: &str = &mut str_0;
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_2);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_496() {
//    rusty_monitor::set_test_id(496);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_0: i32 = 1721119i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 3600i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i64_3: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = 195i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i64_5: i64 = 1000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i8_0: i8 = 19i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_6: i64 = 0i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i64_7: i64 = 24i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_11);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i128_0: i128 = 1000i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::next(weekday_3);
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::next(weekday_2);
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
//    panic!("From RustyUnit with love");
}
}