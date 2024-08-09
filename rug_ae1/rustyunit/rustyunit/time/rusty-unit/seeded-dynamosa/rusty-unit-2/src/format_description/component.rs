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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_521() {
//    rusty_monitor::set_test_id(521);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut month_2: month::Month = crate::month::Month::June;
    let mut month_3: month::Month = crate::month::Month::June;
    let mut month_4: month::Month = crate::month::Month::June;
    let mut month_5: month::Month = crate::month::Month::June;
    let mut month_6: month::Month = crate::month::Month::June;
    let mut month_7: month::Month = crate::month::Month::June;
    let mut month_8: month::Month = crate::month::Month::June;
    let mut month_9: month::Month = crate::month::Month::June;
    let mut month_10: month::Month = crate::month::Month::June;
    let mut month_11: month::Month = crate::month::Month::June;
    let mut month_12: month::Month = crate::month::Month::June;
    let mut month_13: month::Month = crate::month::Month::June;
    let mut month_14: month::Month = crate::month::Month::June;
    let mut month_15: month::Month = crate::month::Month::June;
    let mut month_16: month::Month = crate::month::Month::June;
    let mut month_17: month::Month = crate::month::Month::June;
    let mut month_18: month::Month = crate::month::Month::June;
    let mut month_19: month::Month = crate::month::Month::June;
    let mut month_20: month::Month = crate::month::Month::June;
    let mut month_21: month::Month = crate::month::Month::June;
    let mut month_22: month::Month = crate::month::Month::June;
    let mut month_23: month::Month = crate::month::Month::June;
    let mut month_24: month::Month = crate::month::Month::June;
    let mut month_25: month::Month = crate::month::Month::June;
    let mut month_26: month::Month = crate::month::Month::June;
    let mut month_27: month::Month = crate::month::Month::June;
    let mut month_28: month::Month = crate::month::Month::June;
    let mut month_29: month::Month = crate::month::Month::June;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_485() {
//    rusty_monitor::set_test_id(485);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 1000000000i64;
    let mut i64_1: i64 = 2147483647i64;
    let mut i64_2: i64 = 1i64;
    let mut str_0: &str = "May";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 1000i64;
    let mut i64_4: i64 = 3600i64;
    let mut i64_5: i64 = 2147483647i64;
    let mut str_1: &str = "April";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_2: bool = false;
    let mut i64_6: i64 = 30i64;
    let mut i64_7: i64 = 60i64;
    let mut i64_8: i64 = 24i64;
    let mut str_2: &str = "DifferentVariant";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 86400i64;
    let mut i64_10: i64 = 245i64;
    let mut i64_11: i64 = 1000i64;
    let mut str_3: &str = "ComponentRange";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_4: bool = false;
    let mut i64_12: i64 = 24i64;
    let mut i64_13: i64 = 9223372036854775807i64;
    let mut i64_14: i64 = 1000000i64;
    let mut str_4: &str = "second";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_4_ref_0, minimum: i64_14, maximum: i64_13, value: i64_12, conditional_range: bool_4};
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_4: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_587() {
//    rusty_monitor::set_test_id(587);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_8: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_9: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_10: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_11: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_12: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_13: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_14: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_15: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_16: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_17: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_18: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_19: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_20: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_21: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_22: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_23: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_24: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_25: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_26: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_27: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_28: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_29: weekday::Weekday = crate::weekday::Weekday::Saturday;
//    panic!("From RustyUnit with love");
}
}