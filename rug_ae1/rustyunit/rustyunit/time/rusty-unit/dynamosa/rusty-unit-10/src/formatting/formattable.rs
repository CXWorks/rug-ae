//! A trait that can be used to format an item from its components.

use core::ops::Deref;
use std::io;

use crate::format_description::well_known::{Rfc2822, Rfc3339};
use crate::format_description::FormatItem;
use crate::formatting::{
    format_component, format_number_pad_zero, write, MONTH_NAMES, WEEKDAY_NAMES,
};
use crate::{error, Date, Time, UtcOffset};

/// A type that can be formatted.
#[cfg_attr(__time_03_docs, doc(notable_trait))]
pub trait Formattable: sealed::Sealed {}
impl Formattable for FormatItem<'_> {}
impl Formattable for [FormatItem<'_>] {}
impl Formattable for Rfc3339 {}
impl Formattable for Rfc2822 {}
impl<T: Deref> Formattable for T where T::Target: Formattable {}

/// Seal the trait to prevent downstream users from implementing it.
mod sealed {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Format the item using a format description, the intended output, and the various components.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
    pub trait Sealed {
        /// Format the item into the provided output, returning the number of bytes written.
        fn format_into(
            &self,
            output: &mut impl io::Write,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<usize, error::Format>;

        /// Format the item directly to a `String`.
        fn format(
            &self,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<String, error::Format> {
            let mut buf = Vec::new();
            self.format_into(&mut buf, date, time, offset)?;
            Ok(String::from_utf8_lossy(&buf).into_owned())
        }
    }
}

// region: custom formats
impl<'a> sealed::Sealed for FormatItem<'a> {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        Ok(match *self {
            Self::Literal(literal) => write(output, literal)?,
            Self::Component(component) => format_component(output, component, date, time, offset)?,
            Self::Compound(items) => items.format_into(output, date, time, offset)?,
            Self::Optional(item) => item.format_into(output, date, time, offset)?,
            Self::First(items) => match items {
                [] => 0,
                [item, ..] => item.format_into(output, date, time, offset)?,
            },
        })
    }
}

impl<'a> sealed::Sealed for [FormatItem<'a>] {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        let mut bytes = 0;
        for item in self.iter() {
            bytes += item.format_into(output, date, time, offset)?;
        }
        Ok(bytes)
    }
}

impl<T: Deref> sealed::Sealed for T
where
    T::Target: sealed::Sealed,
{
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        self.deref().format_into(output, date, time, offset)
    }
}
// endregion custom formats

// region: well-known formats
impl sealed::Sealed for Rfc2822 {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let mut bytes = 0;

        let (year, month, day) = date.to_calendar_date();

        if year < 1900 {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds_past_minute() != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        bytes += write(
            output,
            &WEEKDAY_NAMES[date.weekday().number_days_from_monday() as usize][..3],
        )?;
        bytes += write(output, b", ")?;
        bytes += format_number_pad_zero::<_, _, 2>(output, day)?;
        bytes += write(output, b" ")?;
        bytes += write(output, &MONTH_NAMES[month as usize - 1][..3])?;
        bytes += write(output, b" ")?;
        bytes += format_number_pad_zero::<_, _, 4>(output, year as u32)?;
        bytes += write(output, b" ")?;
        bytes += format_number_pad_zero::<_, _, 2>(output, time.hour())?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<_, _, 2>(output, time.minute())?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<_, _, 2>(output, time.second())?;
        bytes += write(output, b" ")?;
        bytes += write(output, if offset.is_negative() { b"-" } else { b"+" })?;
        bytes += format_number_pad_zero::<_, _, 2>(output, offset.whole_hours().unsigned_abs())?;
        bytes +=
            format_number_pad_zero::<_, _, 2>(output, offset.minutes_past_hour().unsigned_abs())?;

        Ok(bytes)
    }
}

impl sealed::Sealed for Rfc3339 {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let mut bytes = 0;

        let year = date.year();

        if !(0..10_000).contains(&year) {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds_past_minute() != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        bytes += format_number_pad_zero::<_, _, 4>(output, year as u32)?;
        bytes += write(output, &[b'-'])?;
        bytes += format_number_pad_zero::<_, _, 2>(output, date.month() as u8)?;
        bytes += write(output, &[b'-'])?;
        bytes += format_number_pad_zero::<_, _, 2>(output, date.day())?;
        bytes += write(output, &[b'T'])?;
        bytes += format_number_pad_zero::<_, _, 2>(output, time.hour())?;
        bytes += write(output, &[b':'])?;
        bytes += format_number_pad_zero::<_, _, 2>(output, time.minute())?;
        bytes += write(output, &[b':'])?;
        bytes += format_number_pad_zero::<_, _, 2>(output, time.second())?;

        #[allow(clippy::if_not_else)]
        if time.nanosecond() != 0 {
            let nanos = time.nanosecond();
            bytes += write(output, &[b'.'])?;
            bytes += if nanos % 10 != 0 {
                format_number_pad_zero::<_, _, 9>(output, nanos)
            } else if (nanos / 10) % 10 != 0 {
                format_number_pad_zero::<_, _, 8>(output, nanos / 10)
            } else if (nanos / 100) % 10 != 0 {
                format_number_pad_zero::<_, _, 7>(output, nanos / 100)
            } else if (nanos / 1_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 6>(output, nanos / 1_000)
            } else if (nanos / 10_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 5>(output, nanos / 10_000)
            } else if (nanos / 100_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 4>(output, nanos / 100_000)
            } else if (nanos / 1_000_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 3>(output, nanos / 1_000_000)
            } else if (nanos / 10_000_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 2>(output, nanos / 10_000_000)
            } else {
                format_number_pad_zero::<_, _, 1>(output, nanos / 100_000_000)
            }?;
        }

        if offset == UtcOffset::UTC {
            bytes += write(output, &[b'Z'])?;
            return Ok(bytes);
        }

        bytes += write(
            output,
            if offset.is_negative() {
                &[b'-']
            } else {
                &[b'+']
            },
        )?;
        bytes += format_number_pad_zero::<_, _, 2>(output, offset.whole_hours().unsigned_abs())?;
        bytes += write(output, &[b':'])?;
        bytes +=
            format_number_pad_zero::<_, _, 2>(output, offset.minutes_past_hour().unsigned_abs())?;

        Ok(bytes)
    }
}
// endregion well-known formats

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1578() {
    rusty_monitor::set_test_id(1578);
    let mut i64_0: i64 = -8i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = -7i64;
    let mut u32_0: u32 = 23u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 11u8;
    let mut u16_0: u16 = 25u16;
    let mut i32_0: i32 = -5i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: std::time::Instant = crate::instant::Instant::into_inner(instant_4);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = 37i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_8: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_9: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = 105i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -109i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_2: i32 = 142i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = -27i8;
    let mut i8_2: i8 = 21i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 31u32;
    let mut u8_3: u8 = 44u8;
    let mut u8_4: u8 = 46u8;
    let mut u8_5: u8 = 91u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_4: i64 = 19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_5: i64 = 19i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_6: i64 = -204i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i64_7: i64 = -154i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_4);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i32_3: i32 = 53i32;
    let mut i64_8: i64 = -46i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    panic!("From RustyUnit with love");
}
}