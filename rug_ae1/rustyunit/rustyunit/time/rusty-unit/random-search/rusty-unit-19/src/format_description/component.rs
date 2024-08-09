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
fn rusty_test_4820() {
    rusty_monitor::set_test_id(4820);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -103i32;
    let mut i64_0: i64 = 180i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = -42i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 108i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -7i8;
    let mut i8_4: i8 = -29i8;
    let mut i8_5: i8 = 59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -52i8;
    let mut i8_7: i8 = 15i8;
    let mut i8_8: i8 = 55i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 46u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 182.708835f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_9: i8 = -96i8;
    let mut i8_10: i8 = 125i8;
    let mut i8_11: i8 = -103i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 45u32;
    let mut u8_3: u8 = 66u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 98u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_12: i8 = -24i8;
    let mut i8_13: i8 = 21i8;
    let mut i8_14: i8 = -77i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_2: i64 = 169i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = -41i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i64_4: i64 = -80i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_5: i64 = 104i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_6: i64 = 82i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i64_7: i64 = 8i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut i64_8: i64 = -56i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_9);
    let mut i8_15: i8 = 71i8;
    let mut i8_16: i8 = 8i8;
    let mut i8_17: i8 = 7i8;
    let mut i8_18: i8 = -12i8;
    let mut i8_19: i8 = -54i8;
    let mut i8_20: i8 = -75i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = 52i8;
    let mut i8_22: i8 = -78i8;
    let mut i8_23: i8 = -48i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f64_0: f64 = 18.372666f64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_9: i64 = -229i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut i64_10: i64 = 20i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_13);
    let mut i64_11: i64 = 45i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i64_12: i64 = -47i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds(i64_12);
    let mut i32_1: i32 = 134i32;
    let mut i64_13: i64 = 145i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new(i64_13, i32_1);
    let mut u32_2: u32 = 87u32;
    let mut u8_6: u8 = 93u8;
    let mut u8_7: u8 = 15u8;
    let mut u8_8: u8 = 2u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut f64_1: f64 = 59.746849f64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_20);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_14: i64 = 59i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::hours(i64_14);
    let mut f32_1: f32 = -22.669966f32;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_23);
    let mut i64_15: i64 = -112i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::hours(i64_15);
    let mut i8_24: i8 = -50i8;
    let mut i8_25: i8 = -52i8;
    let mut i8_26: i8 = 74i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i64_16: i64 = 48i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_16);
    let mut duration_27: std::time::Duration = crate::duration::Duration::abs_std(duration_26);
    let mut i32_2: i32 = -193i32;
    let mut i64_17: i64 = 10i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::hours(i64_17);
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_28, i32_2);
    let mut i32_3: i32 = 81i32;
    let mut i64_18: i64 = 59i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_18, i32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_30);
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i8_27: i8 = -41i8;
    let mut i8_28: i8 = 11i8;
    let mut i8_29: i8 = -114i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_9);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut i64_19: i64 = -119i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_19);
    let mut i32_4: i32 = -43i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut i32_5: i32 = -68i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    panic!("From RustyUnit with love");
}
}