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
fn rusty_test_8172() {
    rusty_monitor::set_test_id(8172);
    let mut i64_0: i64 = 126i64;
    let mut i8_0: i8 = 52i8;
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_1: i8 = -68i8;
    let mut i8_2: i8 = 39i8;
    let mut i8_3: i8 = 116i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut f64_0: f64 = -15.879565f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 276i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 4u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = -43i32;
    let mut i64_1: i64 = 15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut i32_3: i32 = 14i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut i64_2: i64 = 27i64;
    let mut i32_4: i32 = -113i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_4);
    let mut i64_3: i64 = 76i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_5: i32 = -70i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut i32_6: i32 = 21i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_8);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut f64_1: f64 = 106.341160f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_4: i64 = 64i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_10);
    let mut date_8: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut date_9: crate::date::Date = crate::date::Date::saturating_add(date_8, duration_9);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_9);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_4, time_0);
    let mut i64_5: i64 = -30i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut f64_2: f64 = 64.196562f64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut u16_0: u16 = 0u16;
    let mut i32_7: i32 = 37i32;
    let mut date_10: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_10);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_6, duration_12);
    let mut i8_4: i8 = 38i8;
    let mut i8_5: i8 = -21i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_5, i8_4);
    let mut u32_1: u32 = 98u32;
    let mut u8_3: u8 = 45u8;
    let mut u8_4: u8 = 45u8;
    let mut u8_5: u8 = 37u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 21u32;
    let mut u8_6: u8 = 0u8;
    let mut u8_7: u8 = 2u8;
    let mut u8_8: u8 = 56u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_8: i32 = 18i32;
    let mut date_11: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_8, time_3);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_9, utcoffset_1);
    let mut i64_6: i64 = -80i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_13);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_11);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_5);
    let mut u8_9: u8 = crate::util::weeks_in_year(i32_6);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_3);
    panic!("From RustyUnit with love");
}
}