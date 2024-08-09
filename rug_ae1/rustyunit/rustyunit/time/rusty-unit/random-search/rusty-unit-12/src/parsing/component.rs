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
fn rusty_test_4518() {
    rusty_monitor::set_test_id(4518);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -122i8;
    let mut i8_1: i8 = -23i8;
    let mut i8_2: i8 = -42i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -62i8;
    let mut i8_4: i8 = 103i8;
    let mut i8_5: i8 = -122i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_6: i8 = -3i8;
    let mut i8_7: i8 = -54i8;
    let mut i8_8: i8 = -64i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 21u32;
    let mut u8_3: u8 = 51u8;
    let mut u8_4: u8 = 60u8;
    let mut u8_5: u8 = 19u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = -74i8;
    let mut i8_10: i8 = 12i8;
    let mut i8_11: i8 = 49i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 20i8;
    let mut i8_13: i8 = 31i8;
    let mut i8_14: i8 = 47i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_2: u32 = 78u32;
    let mut u8_6: u8 = 77u8;
    let mut u8_7: u8 = 13u8;
    let mut u8_8: u8 = 74u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_15: i8 = -5i8;
    let mut i8_16: i8 = 109i8;
    let mut i8_17: i8 = -46i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 56i8;
    let mut i8_19: i8 = -7i8;
    let mut i8_20: i8 = 73i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = 74i8;
    let mut i8_22: i8 = -53i8;
    let mut i8_23: i8 = -101i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 79i8;
    let mut i8_25: i8 = -64i8;
    let mut i8_26: i8 = 104i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 100i8;
    let mut i8_28: i8 = -4i8;
    let mut i8_29: i8 = -103i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 24u32;
    let mut u8_9: u8 = 8u8;
    let mut u8_10: u8 = 80u8;
    let mut u8_11: u8 = 13u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = 35i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u16_1: u16 = 8u16;
    let mut i32_1: i32 = -54i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i32_2: i32 = 150i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_4: u32 = 32u32;
    let mut u8_12: u8 = 54u8;
    let mut u8_13: u8 = 41u8;
    let mut u8_14: u8 = 77u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_30: i8 = 26i8;
    let mut i8_31: i8 = -15i8;
    let mut i8_32: i8 = -54i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 118i8;
    let mut i8_34: i8 = 10i8;
    let mut i8_35: i8 = 19i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_3, utcoffset_12);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_0_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}
}