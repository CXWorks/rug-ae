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
fn rusty_test_8141() {
    rusty_monitor::set_test_id(8141);
    let mut i8_0: i8 = 101i8;
    let mut i8_1: i8 = 102i8;
    let mut i8_2: i8 = -40i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -188i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 38u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 75i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 92u16;
    let mut i32_1: i32 = -155i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i32_2: i32 = -98i32;
    let mut i64_0: i64 = -146i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i8_3: i8 = 42i8;
    let mut i8_4: i8 = -36i8;
    let mut i8_5: i8 = -45i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_3: i32 = -94i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_3, utcoffset_1);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_4);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i32_4: i32 = -93i32;
    let mut i32_5: i32 = -230i32;
    let mut i64_1: i64 = 50i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_2);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut i8_6: i8 = 120i8;
    let mut i8_7: i8 = 65i8;
    let mut i8_8: i8 = -91i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = -175i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i8_9: i8 = -75i8;
    let mut i8_10: i8 = 88i8;
    let mut i8_11: i8 = -89i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 10i8;
    let mut i8_13: i8 = 36i8;
    let mut i8_14: i8 = 73i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = 249i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut f32_0: f32 = -157.828369f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_15: i8 = 9i8;
    let mut i8_16: i8 = 62i8;
    let mut i8_17: i8 = -28i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 11i8;
    let mut i8_19: i8 = 24i8;
    let mut i8_20: i8 = -15i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i128_1: i128 = 75i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_1: u32 = 99u32;
    let mut u8_3: u8 = 35u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_21: i8 = 48i8;
    let mut i8_22: i8 = 49i8;
    let mut i8_23: i8 = 117i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_6: i32 = -20i32;
    let mut i64_4: i64 = 9i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_6);
    let mut i8_24: i8 = -20i8;
    let mut i8_25: i8 = -71i8;
    let mut i8_26: i8 = -13i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_2: u32 = 2u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 21u8;
    let mut u8_8: u8 = 78u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_5: i64 = -28i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i32_7: i32 = -113i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_7);
    let mut i64_6: i64 = 76i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut i64_7: i64 = 211i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i32_8: i32 = -70i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_8};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_13);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_3, duration_12);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_4);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_11);
    let mut i32_9: i32 = 20i32;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_9: u8 = 9u8;
    let mut i64_8: i64 = 68i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::abs(duration_14);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_7: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_add(date_7, duration_15);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut f64_0: f64 = 106.341160f64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_9: i64 = 64i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::minutes(i64_9);
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_12: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_10, duration_5);
    let mut date_9: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_12);
    let mut date_10: crate::date::Date = crate::date::Date::saturating_add(date_9, duration_16);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_10);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_5, time_3);
    let mut i64_10: i64 = -30i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_27: i8 = 39i8;
    let mut i8_28: i8 = 51i8;
    let mut i8_29: i8 = 91i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut f64_1: f64 = 64.196562f64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_18);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_9, weekday_1);
    let mut u8_10: u8 = crate::util::weeks_in_year(i32_9);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_8);
    panic!("From RustyUnit with love");
}
}