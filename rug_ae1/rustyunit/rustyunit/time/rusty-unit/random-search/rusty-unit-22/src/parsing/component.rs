//! Parsing implementations for all [`Component`](crate::format_description::Component)s.

use core::num::{NonZeroU16, NonZeroU8};

use crate::format_description::modifier;
#[cfg(feature = "large-dates")]
use crate::parsing::combinator::n_to_m_digits_padded;
use crate::parsing::combinator::{
    any_digit, exactly_n_digits, exactly_n_digits_padded, first_match, opt, sign,
};
use crate::parsing::ParsedItem;
use crate::{Month, Weekday};

// region: date components
/// Parse the "year" component of a `Date`.
pub(crate) fn parse_year(input: &[u8], modifiers: modifier::Year) -> Option<ParsedItem<'_, i32>> {
    match modifiers.repr {
        modifier::YearRepr::Full => {
            let ParsedItem(input, sign) = opt(sign)(input);
            #[cfg(not(feature = "large-dates"))]
            let ParsedItem(input, year) =
                exactly_n_digits_padded::<u32, 4>(modifiers.padding)(input)?;
            #[cfg(feature = "large-dates")]
            let ParsedItem(input, year) =
                n_to_m_digits_padded::<u32, 4, 6>(modifiers.padding)(input)?;
            match sign {
                Some(b'-') => Some(ParsedItem(input, -(year as i32))),
                None if modifiers.sign_is_mandatory || year >= 10_000 => None,
                _ => Some(ParsedItem(input, year as i32)),
            }
        }
        modifier::YearRepr::LastTwo => {
            Some(exactly_n_digits_padded::<u32, 2>(modifiers.padding)(input)?.map(|v| v as i32))
        }
    }
}

/// Parse the "month" component of a `Date`.
pub(crate) fn parse_month(
    input: &[u8],
    modifiers: modifier::Month,
) -> Option<ParsedItem<'_, Month>> {
    use Month::*;
    let ParsedItem(remaining, value) = first_match(
        match modifiers.repr {
            modifier::MonthRepr::Numerical => {
                return exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)?
                    .flat_map(|n| Month::from_number(n).ok());
            }
            modifier::MonthRepr::Long => [
                (&b"January"[..], January),
                (&b"February"[..], February),
                (&b"March"[..], March),
                (&b"April"[..], April),
                (&b"May"[..], May),
                (&b"June"[..], June),
                (&b"July"[..], July),
                (&b"August"[..], August),
                (&b"September"[..], September),
                (&b"October"[..], October),
                (&b"November"[..], November),
                (&b"December"[..], December),
            ],
            modifier::MonthRepr::Short => [
                (&b"Jan"[..], January),
                (&b"Feb"[..], February),
                (&b"Mar"[..], March),
                (&b"Apr"[..], April),
                (&b"May"[..], May),
                (&b"Jun"[..], June),
                (&b"Jul"[..], July),
                (&b"Aug"[..], August),
                (&b"Sep"[..], September),
                (&b"Oct"[..], October),
                (&b"Nov"[..], November),
                (&b"Dec"[..], December),
            ],
        },
        modifiers.case_sensitive,
    )(input)?;
    Some(ParsedItem(remaining, value))
}

/// Parse the "week number" component of a `Date`.
pub(crate) fn parse_week_number(
    input: &[u8],
    modifiers: modifier::WeekNumber,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}

/// Parse the "weekday" component of a `Date`.
pub(crate) fn parse_weekday(
    input: &[u8],
    modifiers: modifier::Weekday,
) -> Option<ParsedItem<'_, Weekday>> {
    first_match(
        match (modifiers.repr, modifiers.one_indexed) {
            (modifier::WeekdayRepr::Short, _) => [
                (&b"Mon"[..], Weekday::Monday),
                (&b"Tue"[..], Weekday::Tuesday),
                (&b"Wed"[..], Weekday::Wednesday),
                (&b"Thu"[..], Weekday::Thursday),
                (&b"Fri"[..], Weekday::Friday),
                (&b"Sat"[..], Weekday::Saturday),
                (&b"Sun"[..], Weekday::Sunday),
            ],
            (modifier::WeekdayRepr::Long, _) => [
                (&b"Monday"[..], Weekday::Monday),
                (&b"Tuesday"[..], Weekday::Tuesday),
                (&b"Wednesday"[..], Weekday::Wednesday),
                (&b"Thursday"[..], Weekday::Thursday),
                (&b"Friday"[..], Weekday::Friday),
                (&b"Saturday"[..], Weekday::Saturday),
                (&b"Sunday"[..], Weekday::Sunday),
            ],
            (modifier::WeekdayRepr::Sunday, false) => [
                (&b"1"[..], Weekday::Monday),
                (&b"2"[..], Weekday::Tuesday),
                (&b"3"[..], Weekday::Wednesday),
                (&b"4"[..], Weekday::Thursday),
                (&b"5"[..], Weekday::Friday),
                (&b"6"[..], Weekday::Saturday),
                (&b"0"[..], Weekday::Sunday),
            ],
            (modifier::WeekdayRepr::Sunday, true) => [
                (&b"2"[..], Weekday::Monday),
                (&b"3"[..], Weekday::Tuesday),
                (&b"4"[..], Weekday::Wednesday),
                (&b"5"[..], Weekday::Thursday),
                (&b"6"[..], Weekday::Friday),
                (&b"7"[..], Weekday::Saturday),
                (&b"1"[..], Weekday::Sunday),
            ],
            (modifier::WeekdayRepr::Monday, false) => [
                (&b"0"[..], Weekday::Monday),
                (&b"1"[..], Weekday::Tuesday),
                (&b"2"[..], Weekday::Wednesday),
                (&b"3"[..], Weekday::Thursday),
                (&b"4"[..], Weekday::Friday),
                (&b"5"[..], Weekday::Saturday),
                (&b"6"[..], Weekday::Sunday),
            ],
            (modifier::WeekdayRepr::Monday, true) => [
                (&b"1"[..], Weekday::Monday),
                (&b"2"[..], Weekday::Tuesday),
                (&b"3"[..], Weekday::Wednesday),
                (&b"4"[..], Weekday::Thursday),
                (&b"5"[..], Weekday::Friday),
                (&b"6"[..], Weekday::Saturday),
                (&b"7"[..], Weekday::Sunday),
            ],
        },
        modifiers.case_sensitive,
    )(input)
}

/// Parse the "ordinal" component of a `Date`.
pub(crate) fn parse_ordinal(
    input: &[u8],
    modifiers: modifier::Ordinal,
) -> Option<ParsedItem<'_, NonZeroU16>> {
    exactly_n_digits_padded::<_, 3>(modifiers.padding)(input)
}

/// Parse the "day" component of a `Date`.
pub(crate) fn parse_day(
    input: &[u8],
    modifiers: modifier::Day,
) -> Option<ParsedItem<'_, NonZeroU8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}
// endregion date components

// region: time components
/// Indicate whether the hour is "am" or "pm".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Period {
    #[allow(clippy::missing_docs_in_private_items)]
    Am,
    #[allow(clippy::missing_docs_in_private_items)]
    Pm,
}

/// Parse the "hour" component of a `Time`.
pub(crate) fn parse_hour(input: &[u8], modifiers: modifier::Hour) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}

/// Parse the "minute" component of a `Time`.
pub(crate) fn parse_minute(
    input: &[u8],
    modifiers: modifier::Minute,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}

/// Parse the "second" component of a `Time`.
pub(crate) fn parse_second(
    input: &[u8],
    modifiers: modifier::Second,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}

/// Parse the "period" component of a `Time`. Required if the hour is on a 12-hour clock.
pub(crate) fn parse_period(
    input: &[u8],
    modifiers: modifier::Period,
) -> Option<ParsedItem<'_, Period>> {
    first_match(
        if modifiers.is_uppercase {
            [(&b"AM"[..], Period::Am), (&b"PM"[..], Period::Pm)]
        } else {
            [(&b"am"[..], Period::Am), (&b"pm"[..], Period::Pm)]
        },
        modifiers.case_sensitive,
    )(input)
}

/// Parse the "subsecond" component of a `Time`.
pub(crate) fn parse_subsecond(
    input: &[u8],
    modifiers: modifier::Subsecond,
) -> Option<ParsedItem<'_, u32>> {
    use modifier::SubsecondDigits::*;
    Some(match modifiers.digits {
        One => exactly_n_digits::<u32, 1>(input)?.map(|v| v * 100_000_000),
        Two => exactly_n_digits::<u32, 2>(input)?.map(|v| v * 10_000_000),
        Three => exactly_n_digits::<u32, 3>(input)?.map(|v| v * 1_000_000),
        Four => exactly_n_digits::<u32, 4>(input)?.map(|v| v * 100_000),
        Five => exactly_n_digits::<u32, 5>(input)?.map(|v| v * 10_000),
        Six => exactly_n_digits::<u32, 6>(input)?.map(|v| v * 1_000),
        Seven => exactly_n_digits::<u32, 7>(input)?.map(|v| v * 100),
        Eight => exactly_n_digits::<u32, 8>(input)?.map(|v| v * 10),
        Nine => exactly_n_digits::<u32, 9>(input)?,
        OneOrMore => {
            let ParsedItem(mut input, mut value) =
                any_digit(input)?.map(|v| (v - b'0') as u32 * 100_000_000);

            let mut multiplier = 10_000_000;
            while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                value += (digit - b'0') as u32 * multiplier;
                input = new_input;
                multiplier /= 10;
            }

            ParsedItem(input, value)
        }
    })
}
// endregion time components

// region: offset components
/// Parse the "hour" component of a `UtcOffset`.
pub(crate) fn parse_offset_hour(
    input: &[u8],
    modifiers: modifier::OffsetHour,
) -> Option<ParsedItem<'_, i8>> {
    let ParsedItem(input, sign) = opt(sign)(input);
    let ParsedItem(input, hour) = exactly_n_digits_padded::<u8, 2>(modifiers.padding)(input)?;
    match sign {
        Some(b'-') => Some(ParsedItem(input, -(hour as i8))),
        None if modifiers.sign_is_mandatory => None,
        _ => Some(ParsedItem(input, hour as i8)),
    }
}

/// Parse the "minute" component of a `UtcOffset`.
pub(crate) fn parse_offset_minute(
    input: &[u8],
    modifiers: modifier::OffsetMinute,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}

/// Parse the "second" component of a `UtcOffset`.
pub(crate) fn parse_offset_second(
    input: &[u8],
    modifiers: modifier::OffsetSecond,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<_, 2>(modifiers.padding)(input)
}
// endregion offset components

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4653() {
    rusty_monitor::set_test_id(4653);
    let mut f64_0: f64 = 64.202169f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_0: i64 = 47i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = 98i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 63i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 45i8;
    let mut i8_4: i8 = 82i8;
    let mut i8_5: i8 = -35i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -66i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u16_0: u16 = 18u16;
    let mut i32_1: i32 = 96i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_6: i8 = 64i8;
    let mut i8_7: i8 = 45i8;
    let mut i8_8: i8 = -104i8;
    let mut i8_9: i8 = 58i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i8_10: i8 = 110i8;
    let mut i8_11: i8 = 81i8;
    let mut i8_12: i8 = 19i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_11, i8_10);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_2: i32 = 78i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_2);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 99u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_13: i8 = 43i8;
    let mut i8_14: i8 = 74i8;
    let mut i8_15: i8 = -5i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_15, i8_14, i8_13);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_4);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut u16_1: u16 = 10u16;
    let mut i32_3: i32 = -31i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_3);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_4, time_2);
    let mut f64_1: f64 = 166.339369f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_1);
    panic!("From RustyUnit with love");
}
}