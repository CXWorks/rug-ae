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
fn rusty_test_1451() {
    rusty_monitor::set_test_id(1451);
    let mut u8_0: u8 = 16u8;
    let mut f64_0: f64 = -38.661845f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 38u32;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 92u8;
    let mut u8_3: u8 = 29u8;
    let mut u32_1: u32 = 99u32;
    let mut i32_0: i32 = -33i32;
    let mut i64_0: i64 = 42i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = -19i8;
    let mut i8_1: i8 = 12i8;
    let mut i8_2: i8 = 107i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::October;
    panic!("From RustyUnit with love");
}
}