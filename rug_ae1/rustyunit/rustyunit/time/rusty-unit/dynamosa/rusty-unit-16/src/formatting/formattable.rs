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
fn rusty_test_7490() {
    rusty_monitor::set_test_id(7490);
    let mut i64_0: i64 = -98i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i128_0: i128 = 72i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = -2.791953f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut i32_0: i32 = -269i32;
    let mut i64_1: i64 = 8i64;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 75u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i8_0: i8 = -54i8;
    let mut i8_1: i8 = 92i8;
    let mut i8_2: i8 = -30i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 2u32;
    let mut u8_2: u8 = 17u8;
    let mut u8_3: u8 = 54u8;
    let mut u8_4: u8 = 62u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_4, u8_3, u8_2, u32_0);
    let mut i64_2: i64 = -13i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = -46i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i8_3: i8 = 35i8;
    let mut i8_4: i8 = 30i8;
    let mut i8_5: i8 = -15i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = -20i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut f64_0: f64 = -127.884426f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut i64_5: i64 = 17i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = -84i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut u32_1: u32 = 67u32;
    let mut u8_5: u8 = 50u8;
    let mut u8_6: u8 = 12u8;
    let mut u8_7: u8 = 62u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_6, u8_5, u32_1);
    let mut i8_6: i8 = -6i8;
    let mut i8_7: i8 = 38i8;
    let mut i8_8: i8 = -10i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_1: i128 = 34i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_7: i64 = 56i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_2: u32 = 53u32;
    let mut u8_8: u8 = 55u8;
    let mut u8_9: u8 = 72u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_1, u8_0, u32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_9, weekday_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_2, duration_0);
    panic!("From RustyUnit with love");
}
}