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
fn rusty_test_5475() {
    rusty_monitor::set_test_id(5475);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_0: u8 = 7u8;
    let mut i32_0: i32 = -93i32;
    let mut i32_1: i32 = -230i32;
    let mut i64_0: i64 = 50i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 120i8;
    let mut i8_1: i8 = 65i8;
    let mut i8_2: i8 = -91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -175i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i8_3: i8 = -75i8;
    let mut i8_4: i8 = 88i8;
    let mut i8_5: i8 = -89i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 4u32;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 88u8;
    let mut u8_3: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i8_6: i8 = 10i8;
    let mut i8_7: i8 = 36i8;
    let mut i8_8: i8 = 73i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 249i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut f32_0: f32 = -157.828369f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_9: i8 = 9i8;
    let mut i8_10: i8 = 62i8;
    let mut i8_11: i8 = -28i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 11i8;
    let mut i8_13: i8 = 24i8;
    let mut i8_14: i8 = -15i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i128_0: i128 = 75i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_1: u32 = 99u32;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 27u8;
    let mut u8_6: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i8_15: i8 = 48i8;
    let mut i8_16: i8 = 49i8;
    let mut i8_17: i8 = 117i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i32_2: i32 = -20i32;
    let mut i64_3: i64 = 9i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_2);
    let mut i8_18: i8 = -20i8;
    let mut i8_19: i8 = -71i8;
    let mut i8_20: i8 = -13i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 2u32;
    let mut u8_7: u8 = 89u8;
    let mut u8_8: u8 = 21u8;
    let mut u8_9: u8 = 78u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i64_4: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_3: i32 = -113i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_3);
    let mut i64_5: i64 = 76i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i64_6: i64 = 211i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i32_4: i32 = -70i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_11);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_10);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_9);
    let mut i32_5: i32 = 20i32;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut u32_3: u32 = 29u32;
    let mut u8_10: u8 = 2u8;
    let mut u8_11: u8 = 83u8;
    let mut u8_12: u8 = 9u8;
    let mut i64_7: i64 = 68i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_13);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut f64_0: f64 = 106.341160f64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_8: i64 = 64i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_15);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_14);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_3);
    let mut i64_9: i64 = -30i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_21: i8 = 39i8;
    let mut i8_22: i8 = 51i8;
    let mut i8_23: i8 = 91i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f64_1: f64 = 64.196562f64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u16_0: u16 = 0u16;
    let mut i32_6: i32 = 37i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_17);
    let mut i8_24: i8 = 38i8;
    let mut i8_25: i8 = -21i8;
    let mut i8_26: i8 = 65i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_4: u32 = 98u32;
    let mut u8_13: u8 = 45u8;
    let mut u8_14: u8 = 45u8;
    let mut u8_15: u8 = 37u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_15, u8_14, u8_13, u32_4);
    let mut u32_5: u32 = 21u32;
    let mut u8_16: u8 = 0u8;
    let mut u8_17: u8 = 59u8;
    let mut u8_18: u8 = 56u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_18, u8_17, u8_16, u32_5);
    let mut i32_7: i32 = 18i32;
    let mut date_8: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_5);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_6, time_4);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_7, utcoffset_8);
    let mut i64_10: i64 = -80i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_10);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_18);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_7);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_16);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_12, u8_11, u8_10, u32_3);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    let mut u8_19: u8 = crate::util::weeks_in_year(i32_5);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_3);
    panic!("From RustyUnit with love");
}
}