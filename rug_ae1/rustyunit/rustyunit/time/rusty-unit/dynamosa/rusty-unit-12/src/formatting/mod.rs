//! Formatting for various types.

pub(crate) mod formattable;

use std::io;

pub use self::formattable::Formattable;
use crate::format_description::{modifier, Component};
use crate::{error, Date, Time, UtcOffset};

#[allow(clippy::missing_docs_in_private_items)]
const MONTH_NAMES: [&[u8]; 12] = [
    b"January",
    b"February",
    b"March",
    b"April",
    b"May",
    b"June",
    b"July",
    b"August",
    b"September",
    b"October",
    b"November",
    b"December",
];

#[allow(clippy::missing_docs_in_private_items)]
const WEEKDAY_NAMES: [&[u8]; 7] = [
    b"Monday",
    b"Tuesday",
    b"Wednesday",
    b"Thursday",
    b"Friday",
    b"Saturday",
    b"Sunday",
];

// region: extension trait
/// A trait that indicates the formatted width of the value can be determined.
///
/// Note that this should not be implemented for any signed integers. This forces the caller to
/// write the sign if desired.
pub(crate) trait DigitCount {
    /// The number of digits in the stringified value.
    fn num_digits(self) -> u8;
}
impl DigitCount for u8 {
    fn num_digits(self) -> u8 {
        // Using a lookup table as with u32 is *not* faster in standalone benchmarks.
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else {
            3
        }
    }
}
impl DigitCount for u16 {
    fn num_digits(self) -> u8 {
        // Using a lookup table as with u32 is *not* faster in standalone benchmarks.
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else if self < 1_000 {
            3
        } else if self < 10_000 {
            4
        } else {
            5
        }
    }
}

impl DigitCount for u32 {
    fn num_digits(self) -> u8 {
        /// Lookup table
        const TABLE: &[u64] = &[
            0x0001_0000_0000,
            0x0001_0000_0000,
            0x0001_0000_0000,
            0x0001_FFFF_FFF6,
            0x0002_0000_0000,
            0x0002_0000_0000,
            0x0002_FFFF_FF9C,
            0x0003_0000_0000,
            0x0003_0000_0000,
            0x0003_FFFF_FC18,
            0x0004_0000_0000,
            0x0004_0000_0000,
            0x0004_0000_0000,
            0x0004_FFFF_D8F0,
            0x0005_0000_0000,
            0x0005_0000_0000,
            0x0005_FFFE_7960,
            0x0006_0000_0000,
            0x0006_0000_0000,
            0x0006_FFF0_BDC0,
            0x0007_0000_0000,
            0x0007_0000_0000,
            0x0007_0000_0000,
            0x0007_FF67_6980,
            0x0008_0000_0000,
            0x0008_0000_0000,
            0x0008_FA0A_1F00,
            0x0009_0000_0000,
            0x0009_0000_0000,
            0x0009_C465_3600,
            0x000A_0000_0000,
            0x000A_0000_0000,
        ];
        ((self as u64 + TABLE[31_u32.saturating_sub(self.leading_zeros()) as usize]) >> 32) as _
    }
}
// endregion extension trait

/// Write all bytes to the output, returning the number of bytes written.
fn write(output: &mut impl io::Write, bytes: &[u8]) -> io::Result<usize> {
    output.write_all(bytes)?;
    Ok(bytes.len())
}

/// Format a number with the provided padding and width.
///
/// The sign must be written by the caller.
pub(crate) fn format_number<W: io::Write, V: itoa::Integer + DigitCount + Copy, const WIDTH: u8>(
    output: &mut W,
    value: V,
    padding: modifier::Padding,
) -> Result<usize, io::Error> {
    match padding {
        modifier::Padding::Space => format_number_pad_space::<_, _, WIDTH>(output, value),
        modifier::Padding::Zero => format_number_pad_zero::<_, _, WIDTH>(output, value),
        modifier::Padding::None => write(output, itoa::Buffer::new().format(value).as_bytes()),
    }
}

/// Format a number with the provided width and spaces as padding.
///
/// The sign must be written by the caller.
pub(crate) fn format_number_pad_space<
    W: io::Write,
    V: itoa::Integer + DigitCount + Copy,
    const WIDTH: u8,
>(
    output: &mut W,
    value: V,
) -> Result<usize, io::Error> {
    let mut bytes = 0;
    for _ in 0..(WIDTH.saturating_sub(value.num_digits())) {
        bytes += write(output, &[b' '])?;
    }
    bytes += write(output, itoa::Buffer::new().format(value).as_bytes())?;
    Ok(bytes)
}

/// Format a number with the provided width and zeros as padding.
///
/// The sign must be written by the caller.
pub(crate) fn format_number_pad_zero<
    W: io::Write,
    V: itoa::Integer + DigitCount + Copy,
    const WIDTH: u8,
>(
    output: &mut W,
    value: V,
) -> Result<usize, io::Error> {
    let mut bytes = 0;
    for _ in 0..(WIDTH.saturating_sub(value.num_digits())) {
        bytes += write(output, &[b'0'])?;
    }
    bytes += write(output, itoa::Buffer::new().format(value).as_bytes())?;
    Ok(bytes)
}

/// Format the provided component into the designated output. An `Err` will be returned if the
/// component requires information that it does not provide or if the value cannot be output to the
/// stream.
pub(crate) fn format_component(
    output: &mut impl io::Write,
    component: Component,
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<UtcOffset>,
) -> Result<usize, error::Format> {
    use Component::*;
    Ok(match (component, date, time, offset) {
        (Day(modifier), Some(date), ..) => fmt_day(output, date, modifier)?,
        (Month(modifier), Some(date), ..) => fmt_month(output, date, modifier)?,
        (Ordinal(modifier), Some(date), ..) => fmt_ordinal(output, date, modifier)?,
        (Weekday(modifier), Some(date), ..) => fmt_weekday(output, date, modifier)?,
        (WeekNumber(modifier), Some(date), ..) => fmt_week_number(output, date, modifier)?,
        (Year(modifier), Some(date), ..) => fmt_year(output, date, modifier)?,
        (Hour(modifier), _, Some(time), _) => fmt_hour(output, time, modifier)?,
        (Minute(modifier), _, Some(time), _) => fmt_minute(output, time, modifier)?,
        (Period(modifier), _, Some(time), _) => fmt_period(output, time, modifier)?,
        (Second(modifier), _, Some(time), _) => fmt_second(output, time, modifier)?,
        (Subsecond(modifier), _, Some(time), _) => fmt_subsecond(output, time, modifier)?,
        (OffsetHour(modifier), .., Some(offset)) => fmt_offset_hour(output, offset, modifier)?,
        (OffsetMinute(modifier), .., Some(offset)) => fmt_offset_minute(output, offset, modifier)?,
        (OffsetSecond(modifier), .., Some(offset)) => fmt_offset_second(output, offset, modifier)?,
        _ => return Err(error::Format::InsufficientTypeInformation),
    })
}

// region: date formatters
/// Format the day into the designated output.
fn fmt_day(
    output: &mut impl io::Write,
    date: Date,
    modifier::Day { padding }: modifier::Day,
) -> Result<usize, io::Error> {
    format_number::<_, _, 2>(output, date.day(), padding)
}

/// Format the month into the designated output.
fn fmt_month(
    output: &mut impl io::Write,
    date: Date,
    modifier::Month {
        padding,
        repr,
        case_sensitive: _, // no effect on formatting
    }: modifier::Month,
) -> Result<usize, io::Error> {
    match repr {
        modifier::MonthRepr::Numerical => {
            format_number::<_, _, 2>(output, date.month() as u8, padding)
        }
        modifier::MonthRepr::Long => write(output, MONTH_NAMES[date.month() as usize - 1]),
        modifier::MonthRepr::Short => write(output, &MONTH_NAMES[date.month() as usize - 1][..3]),
    }
}

/// Format the ordinal into the designated output.
fn fmt_ordinal(
    output: &mut impl io::Write,
    date: Date,
    modifier::Ordinal { padding }: modifier::Ordinal,
) -> Result<usize, io::Error> {
    format_number::<_, _, 3>(output, date.ordinal(), padding)
}

/// Format the weekday into the designated output.
fn fmt_weekday(
    output: &mut impl io::Write,
    date: Date,
    modifier::Weekday {
        repr,
        one_indexed,
        case_sensitive: _, // no effect on formatting
    }: modifier::Weekday,
) -> Result<usize, io::Error> {
    match repr {
        modifier::WeekdayRepr::Short => write(
            output,
            &WEEKDAY_NAMES[date.weekday().number_days_from_monday() as usize][..3],
        ),
        modifier::WeekdayRepr::Long => write(
            output,
            WEEKDAY_NAMES[date.weekday().number_days_from_monday() as usize],
        ),
        modifier::WeekdayRepr::Sunday => format_number::<_, _, 1>(
            output,
            date.weekday().number_days_from_sunday() + one_indexed as u8,
            modifier::Padding::None,
        ),
        modifier::WeekdayRepr::Monday => format_number::<_, _, 1>(
            output,
            date.weekday().number_days_from_monday() + one_indexed as u8,
            modifier::Padding::None,
        ),
    }
}

/// Format the week number into the designated output.
fn fmt_week_number(
    output: &mut impl io::Write,
    date: Date,
    modifier::WeekNumber { padding, repr }: modifier::WeekNumber,
) -> Result<usize, io::Error> {
    format_number::<_, _, 2>(
        output,
        match repr {
            modifier::WeekNumberRepr::Iso => date.iso_week(),
            modifier::WeekNumberRepr::Sunday => date.sunday_based_week(),
            modifier::WeekNumberRepr::Monday => date.monday_based_week(),
        },
        padding,
    )
}

/// Format the year into the designated output.
fn fmt_year(
    output: &mut impl io::Write,
    date: Date,
    modifier::Year {
        padding,
        repr,
        iso_week_based,
        sign_is_mandatory,
    }: modifier::Year,
) -> Result<usize, io::Error> {
    let full_year = if iso_week_based {
        date.iso_year_week().0
    } else {
        date.year()
    };
    let value = match repr {
        modifier::YearRepr::Full => full_year,
        modifier::YearRepr::LastTwo => (full_year % 100).abs(),
    };
    let format_number = match repr {
        #[cfg(feature = "large-dates")]
        modifier::YearRepr::Full if value.abs() >= 100_000 => format_number::<_, _, 6>,
        #[cfg(feature = "large-dates")]
        modifier::YearRepr::Full if value.abs() >= 10_000 => format_number::<_, _, 5>,
        modifier::YearRepr::Full => format_number::<_, _, 4>,
        modifier::YearRepr::LastTwo => format_number::<_, _, 2>,
    };
    let mut bytes = 0;
    if repr != modifier::YearRepr::LastTwo {
        if full_year < 0 {
            bytes += write(output, &[b'-'])?;
        } else if sign_is_mandatory || cfg!(feature = "large-dates") && full_year >= 10_000 {
            bytes += write(output, &[b'+'])?;
        }
    }
    bytes += format_number(output, value.unsigned_abs(), padding)?;
    Ok(bytes)
}
// endregion date formatters

// region: time formatters
/// Format the hour into the designated output.
fn fmt_hour(
    output: &mut impl io::Write,
    time: Time,
    modifier::Hour {
        padding,
        is_12_hour_clock,
    }: modifier::Hour,
) -> Result<usize, io::Error> {
    let value = match (time.hour(), is_12_hour_clock) {
        (hour, false) => hour,
        (0 | 12, true) => 12,
        (hour, true) if hour < 12 => hour,
        (hour, true) => hour - 12,
    };
    format_number::<_, _, 2>(output, value, padding)
}

/// Format the minute into the designated output.
fn fmt_minute(
    output: &mut impl io::Write,
    time: Time,
    modifier::Minute { padding }: modifier::Minute,
) -> Result<usize, io::Error> {
    format_number::<_, _, 2>(output, time.minute(), padding)
}

/// Format the period into the designated output.
fn fmt_period(
    output: &mut impl io::Write,
    time: Time,
    modifier::Period {
        is_uppercase,
        case_sensitive: _, // no effect on formatting
    }: modifier::Period,
) -> Result<usize, io::Error> {
    match (time.hour() >= 12, is_uppercase) {
        (false, false) => write(output, b"am"),
        (false, true) => write(output, b"AM"),
        (true, false) => write(output, b"pm"),
        (true, true) => write(output, b"PM"),
    }
}

/// Format the second into the designated output.
fn fmt_second(
    output: &mut impl io::Write,
    time: Time,
    modifier::Second { padding }: modifier::Second,
) -> Result<usize, io::Error> {
    format_number::<_, _, 2>(output, time.second(), padding)
}

/// Format the subsecond into the designated output.
fn fmt_subsecond<W: io::Write>(
    output: &mut W,
    time: Time,
    modifier::Subsecond { digits }: modifier::Subsecond,
) -> Result<usize, io::Error> {
    use modifier::SubsecondDigits::*;
    let nanos = time.nanosecond();

    if digits == Nine || (digits == OneOrMore && nanos % 10 != 0) {
        format_number_pad_zero::<_, _, 9>(output, nanos)
    } else if digits == Eight || (digits == OneOrMore && (nanos / 10) % 10 != 0) {
        format_number_pad_zero::<_, _, 8>(output, nanos / 10)
    } else if digits == Seven || (digits == OneOrMore && (nanos / 100) % 10 != 0) {
        format_number_pad_zero::<_, _, 7>(output, nanos / 100)
    } else if digits == Six || (digits == OneOrMore && (nanos / 1_000) % 10 != 0) {
        format_number_pad_zero::<_, _, 6>(output, nanos / 1_000)
    } else if digits == Five || (digits == OneOrMore && (nanos / 10_000) % 10 != 0) {
        format_number_pad_zero::<_, _, 5>(output, nanos / 10_000)
    } else if digits == Four || (digits == OneOrMore && (nanos / 100_000) % 10 != 0) {
        format_number_pad_zero::<_, _, 4>(output, nanos / 100_000)
    } else if digits == Three || (digits == OneOrMore && (nanos / 1_000_000) % 10 != 0) {
        format_number_pad_zero::<_, _, 3>(output, nanos / 1_000_000)
    } else if digits == Two || (digits == OneOrMore && (nanos / 10_000_000) % 10 != 0) {
        format_number_pad_zero::<_, _, 2>(output, nanos / 10_000_000)
    } else {
        format_number_pad_zero::<_, _, 1>(output, nanos / 100_000_000)
    }
}
// endregion time formatters

// region: offset formatters
/// Format the offset hour into the designated output.
fn fmt_offset_hour(
    output: &mut impl io::Write,
    offset: UtcOffset,
    modifier::OffsetHour {
        padding,
        sign_is_mandatory,
    }: modifier::OffsetHour,
) -> Result<usize, io::Error> {
    let mut bytes = 0;
    if offset.is_negative() {
        bytes += write(output, &[b'-'])?;
    } else if sign_is_mandatory {
        bytes += write(output, &[b'+'])?;
    }
    bytes += format_number::<_, _, 2>(output, offset.whole_hours().unsigned_abs(), padding)?;
    Ok(bytes)
}

/// Format the offset minute into the designated output.
fn fmt_offset_minute(
    output: &mut impl io::Write,
    offset: UtcOffset,
    modifier::OffsetMinute { padding }: modifier::OffsetMinute,
) -> Result<usize, io::Error> {
    format_number::<_, _, 2>(output, offset.minutes_past_hour().unsigned_abs(), padding)
}

/// Format the offset second into the designated output.
fn fmt_offset_second(
    output: &mut impl io::Write,
    offset: UtcOffset,
    modifier::OffsetSecond { padding }: modifier::OffsetSecond,
) -> Result<usize, io::Error> {
    format_number::<_, _, 2>(output, offset.seconds_past_minute().unsigned_abs(), padding)
}
// endregion offset formatters

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7921() {
    rusty_monitor::set_test_id(7921);
    let mut i32_0: i32 = -34i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = 179i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 48u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -63i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 41u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 80u8;
    let mut u8_5: u8 = 90u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 78i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut f64_0: f64 = -9.422799f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = 91i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = -46i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 247i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut u32_2: u32 = 84u32;
    let mut u8_6: u8 = 35u8;
    let mut u8_7: u8 = 9u8;
    let mut u8_8: u8 = 99u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = 11i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i64_3: i64 = 11i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i32_3: i32 = 150i32;
    let mut i64_4: i64 = 15i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 35u32;
    let mut u8_9: u8 = 11u8;
    let mut u8_10: u8 = 67u8;
    let mut u8_11: u8 = 17u8;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_4: i32 = -8i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_9);
    let mut i64_5: i64 = 75i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 28u32;
    let mut u8_12: u8 = 57u8;
    let mut u8_13: u8 = 10u8;
    let mut u8_14: u8 = 99u8;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_5: u32 = 87u32;
    let mut u8_15: u8 = 99u8;
    let mut u8_16: u8 = 0u8;
    let mut u8_17: u8 = 69u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_5: i32 = crate::duration::Duration::subsec_microseconds(duration_3);
    let mut offsetdatetime_0_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    panic!("From RustyUnit with love");
}
}