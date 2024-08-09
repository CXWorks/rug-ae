//! Part of a format description.

#[cfg(feature = "alloc")]
use alloc::string::String;

use crate::format_description::modifier;
#[cfg(feature = "alloc")]
use crate::{error::InvalidFormatDescription, format_description::modifier::Modifiers};

/// A component of a larger format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    /// Day of the month.
    Day(modifier::Day),
    /// Month of the year.
    Month(modifier::Month),
    /// Ordinal day of the year.
    Ordinal(modifier::Ordinal),
    /// Day of the week.
    Weekday(modifier::Weekday),
    /// Week within the year.
    WeekNumber(modifier::WeekNumber),
    /// Year of the date.
    Year(modifier::Year),
    /// Hour of the day.
    Hour(modifier::Hour),
    /// Minute within the hour.
    Minute(modifier::Minute),
    /// AM/PM part of the time.
    Period(modifier::Period),
    /// Second within the minute.
    Second(modifier::Second),
    /// Subsecond within the second.
    Subsecond(modifier::Subsecond),
    /// Hour of the UTC offset.
    OffsetHour(modifier::OffsetHour),
    /// Minute within the hour of the UTC offset.
    OffsetMinute(modifier::OffsetMinute),
    /// Second within the minute of the UTC offset.
    OffsetSecond(modifier::OffsetSecond),
}

/// A component with no modifiers present.
#[cfg(feature = "alloc")]
pub(crate) enum NakedComponent {
    /// Day of the month.
    Day,
    /// Month of the year.
    Month,
    /// Ordinal day of the year.
    Ordinal,
    /// Day of the week.
    Weekday,
    /// Week within the year.
    WeekNumber,
    /// Year of the date.
    Year,
    /// Hour of the day.
    Hour,
    /// Minute within the hour.
    Minute,
    /// AM/PM part of the time.
    Period,
    /// Second within the minute.
    Second,
    /// Subsecond within the second.
    Subsecond,
    /// Hour of the UTC offset.
    OffsetHour,
    /// Minute within the hour of the UTC offset.
    OffsetMinute,
    /// Second within the minute of the UTC offset.
    OffsetSecond,
}

#[cfg(feature = "alloc")]
impl NakedComponent {
    /// Parse a component (without its modifiers) from the provided name.
    pub(crate) fn parse(
        component_name: &[u8],
        component_index: usize,
    ) -> Result<Self, InvalidFormatDescription> {
        match component_name {
            b"day" => Ok(Self::Day),
            b"month" => Ok(Self::Month),
            b"ordinal" => Ok(Self::Ordinal),
            b"weekday" => Ok(Self::Weekday),
            b"week_number" => Ok(Self::WeekNumber),
            b"year" => Ok(Self::Year),
            b"hour" => Ok(Self::Hour),
            b"minute" => Ok(Self::Minute),
            b"period" => Ok(Self::Period),
            b"second" => Ok(Self::Second),
            b"subsecond" => Ok(Self::Subsecond),
            b"offset_hour" => Ok(Self::OffsetHour),
            b"offset_minute" => Ok(Self::OffsetMinute),
            b"offset_second" => Ok(Self::OffsetSecond),
            b"" => Err(InvalidFormatDescription::MissingComponentName {
                index: component_index,
            }),
            _ => Err(InvalidFormatDescription::InvalidComponentName {
                name: String::from_utf8_lossy(component_name).into_owned(),
                index: component_index,
            }),
        }
    }

    /// Attach the necessary modifiers to the component.
    pub(crate) fn attach_modifiers(self, modifiers: &Modifiers) -> Component {
        match self {
            Self::Day => Component::Day(modifier::Day {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Month => Component::Month(modifier::Month {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.month_repr.unwrap_or_default(),
                case_sensitive: modifiers.case_sensitive.unwrap_or(true),
            }),
            Self::Ordinal => Component::Ordinal(modifier::Ordinal {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Weekday => Component::Weekday(modifier::Weekday {
                repr: modifiers.weekday_repr.unwrap_or_default(),
                one_indexed: modifiers.weekday_is_one_indexed.unwrap_or(true),
                case_sensitive: modifiers.case_sensitive.unwrap_or(true),
            }),
            Self::WeekNumber => Component::WeekNumber(modifier::WeekNumber {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.week_number_repr.unwrap_or_default(),
            }),
            Self::Year => Component::Year(modifier::Year {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.year_repr.unwrap_or_default(),
                iso_week_based: modifiers.year_is_iso_week_based.unwrap_or_default(),
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
            }),
            Self::Hour => Component::Hour(modifier::Hour {
                padding: modifiers.padding.unwrap_or_default(),
                is_12_hour_clock: modifiers.hour_is_12_hour_clock.unwrap_or_default(),
            }),
            Self::Minute => Component::Minute(modifier::Minute {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Period => Component::Period(modifier::Period {
                is_uppercase: modifiers.period_is_uppercase.unwrap_or(true),
                case_sensitive: modifiers.case_sensitive.unwrap_or(true),
            }),
            Self::Second => Component::Second(modifier::Second {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Subsecond => Component::Subsecond(modifier::Subsecond {
                digits: modifiers.subsecond_digits.unwrap_or_default(),
            }),
            Self::OffsetHour => Component::OffsetHour(modifier::OffsetHour {
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::OffsetMinute => Component::OffsetMinute(modifier::OffsetMinute {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::OffsetSecond => Component::OffsetSecond(modifier::OffsetSecond {
                padding: modifiers.padding.unwrap_or_default(),
            }),
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
fn rusty_test_8057() {
    rusty_monitor::set_test_id(8057);
    let mut i64_0: i64 = -35i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 12i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i32_1: i32 = -90i32;
    let mut i64_1: i64 = 141i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut u32_0: u32 = 1u32;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 82u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut i64_2: i64 = -116i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut f32_0: f32 = 154.671164f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_3: i64 = -184i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut f32_1: f32 = 24.060743f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_6);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut f64_0: f64 = -65.419486f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_2: f32 = -241.087496f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut f64_1: f64 = -15.432639f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut u32_1: u32 = 92u32;
    let mut u8_3: u8 = 94u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 87u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -126i32;
    let mut i64_4: i64 = -116i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_2);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut i32_3: i32 = -33i32;
    let mut i32_4: i32 = -94i32;
    let mut i64_5: i64 = -80i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_4);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_15, i32_3);
    let mut i64_6: i64 = 114i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::abs(duration_16);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_18);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i8_0: i8 = 104i8;
    let mut u16_0: u16 = 64u16;
    let mut i32_5: i32 = -39i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = -48i8;
    let mut i8_3: i8 = 16i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u32_2: u32 = 42u32;
    let mut u8_6: u8 = 92u8;
    let mut u8_7: u8 = 42u8;
    let mut u8_8: u8 = 3u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_4: i8 = -29i8;
    let mut i8_5: i8 = -37i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_4, i8_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_19: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_19);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    panic!("From RustyUnit with love");
}
}