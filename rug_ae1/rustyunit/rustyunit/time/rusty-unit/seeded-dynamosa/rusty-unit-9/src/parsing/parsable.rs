//! A trait that can be used to parse an item from an input.

use core::convert::TryInto;
use core::ops::Deref;

use crate::error::TryFromParsed;
use crate::format_description::well_known::{Rfc2822, Rfc3339};
use crate::format_description::FormatItem;
use crate::parsing::{Parsed, ParsedItem};
use crate::{error, Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

/// A type that can be parsed.
#[cfg_attr(__time_03_docs, doc(notable_trait))]
pub trait Parsable: sealed::Sealed {}
impl Parsable for FormatItem<'_> {}
impl Parsable for [FormatItem<'_>] {}
impl Parsable for Rfc2822 {}
impl Parsable for Rfc3339 {}
impl<T: Deref> Parsable for T where T::Target: Parsable {}

/// Seal the trait to prevent downstream users from implementing it, while still allowing it to
/// exist in generic bounds.
mod sealed {

    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Parse the item using a format description and an input.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
    pub trait Sealed {
        /// Parse the item into the provided [`Parsed`] struct.
        ///
        /// This method can be used to parse a single component without parsing the full value.
        fn parse_into<'a>(
            &self,
            input: &'a [u8],
            parsed: &mut Parsed,
        ) -> Result<&'a [u8], error::Parse>;

        /// Parse the item into a new [`Parsed`] struct.
        ///
        /// This method can only be used to parse a complete value of a type. If any characters
        /// remain after parsing, an error will be returned.
        fn parse(&self, input: &[u8]) -> Result<Parsed, error::Parse> {
            let mut parsed = Parsed::new();
            if self.parse_into(input, &mut parsed)?.is_empty() {
                Ok(parsed)
            } else {
                Err(error::Parse::UnexpectedTrailingCharacters)
            }
        }

        /// Parse a [`Date`] from the format description.
        fn parse_date(&self, input: &[u8]) -> Result<Date, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`Time`] from the format description.
        fn parse_time(&self, input: &[u8]) -> Result<Time, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`UtcOffset`] from the format description.
        fn parse_offset(&self, input: &[u8]) -> Result<UtcOffset, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`PrimitiveDateTime`] from the format description.
        fn parse_date_time(&self, input: &[u8]) -> Result<PrimitiveDateTime, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`OffsetDateTime`] from the format description.
        fn parse_offset_date_time(&self, input: &[u8]) -> Result<OffsetDateTime, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }
    }
}

// region: custom formats
impl sealed::Sealed for FormatItem<'_> {
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        Ok(parsed.parse_item(input, self)?)
    }
}

impl sealed::Sealed for [FormatItem<'_>] {
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        Ok(parsed.parse_items(input, self)?)
    }
}

impl<T: Deref> sealed::Sealed for T
where
    T::Target: sealed::Sealed,
{
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        self.deref().parse_into(input, parsed)
    }
}
// endregion custom formats

// region: well-known formats
impl sealed::Sealed for Rfc2822 {
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        use crate::error::ParseFromDescription::{InvalidComponent, InvalidLiteral};
        use crate::parsing::combinator::rfc::rfc2822::{cfws, fws};
        use crate::parsing::combinator::{
            ascii_char, exactly_n_digits, first_match, n_to_m_digits, opt, sign,
        };

        let colon = ascii_char::<b':'>;
        let comma = ascii_char::<b','>;

        let input = opt(fws)(input).into_inner();
        let input = first_match(
            [
                (&b"Mon"[..], Weekday::Monday),
                (&b"Tue"[..], Weekday::Tuesday),
                (&b"Wed"[..], Weekday::Wednesday),
                (&b"Thu"[..], Weekday::Thursday),
                (&b"Fri"[..], Weekday::Friday),
                (&b"Sat"[..], Weekday::Saturday),
                (&b"Sun"[..], Weekday::Sunday),
            ],
            false,
        )(input)
        .ok_or(InvalidComponent("weekday"))?
        .assign_value_to(&mut parsed.weekday);
        let input = comma(input).ok_or(InvalidLiteral)?.into_inner();
        let input = cfws(input).ok_or(InvalidLiteral)?.into_inner();
        let input = n_to_m_digits::<_, 1, 2>(input)
            .ok_or(InvalidComponent("day"))?
            .assign_value_to(&mut parsed.day);
        let input = cfws(input).ok_or(InvalidLiteral)?.into_inner();
        let input = first_match(
            [
                (&b"Jan"[..], Month::January),
                (&b"Feb"[..], Month::February),
                (&b"Mar"[..], Month::March),
                (&b"Apr"[..], Month::April),
                (&b"May"[..], Month::May),
                (&b"Jun"[..], Month::June),
                (&b"Jul"[..], Month::July),
                (&b"Aug"[..], Month::August),
                (&b"Sep"[..], Month::September),
                (&b"Oct"[..], Month::October),
                (&b"Nov"[..], Month::November),
                (&b"Dec"[..], Month::December),
            ],
            false,
        )(input)
        .ok_or(InvalidComponent("month"))?
        .assign_value_to(&mut parsed.month);
        let input = cfws(input).ok_or(InvalidLiteral)?.into_inner();
        let input = match exactly_n_digits::<u32, 4>(input) {
            Some(item) => {
                let input = item
                    .flat_map_res(|year| {
                        if year >= 1900 {
                            Ok(year)
                        } else {
                            Err(InvalidComponent("year"))
                        }
                    })?
                    .map(|year| year as _)
                    .assign_value_to(&mut parsed.year);
                let input = fws(input).ok_or(InvalidLiteral)?.into_inner();
                input
            }
            None => {
                let input = exactly_n_digits::<u32, 2>(input)
                    .ok_or(InvalidComponent("year"))?
                    .map(|year| if year < 50 { year + 2000 } else { year + 1900 })
                    .map(|year| year as _)
                    .assign_value_to(&mut parsed.year);
                let input = cfws(input).ok_or(InvalidLiteral)?.into_inner();
                input
            }
        };

        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("hour"))?
            .assign_value_to(&mut parsed.hour_24);
        let input = opt(cfws)(input).into_inner();
        let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
        let input = opt(cfws)(input).into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("minute"))?
            .assign_value_to(&mut parsed.minute);

        let input = if let Some(input) = colon(opt(cfws)(input).into_inner()) {
            let input = input.into_inner(); // discard the colon
            let input = opt(cfws)(input).into_inner();
            let input = exactly_n_digits::<_, 2>(input)
                .ok_or(InvalidComponent("second"))?
                .assign_value_to(&mut parsed.second);
            let input = cfws(input).ok_or(InvalidLiteral)?.into_inner();
            input
        } else {
            cfws(input).ok_or(InvalidLiteral)?.into_inner()
        };

        // The RFC explicitly allows leap seconds. We don't currently support them, so treat it as
        // the previous moment.
        if parsed.second == Some(60) {
            parsed.second = Some(59);
            parsed.subsecond = Some(999_999_999);
        }

        let zone_literal = first_match(
            [
                (&b"UT"[..], 0),
                (&b"GMT"[..], 0),
                (&b"EST"[..], -5),
                (&b"EDT"[..], -4),
                (&b"CST"[..], -6),
                (&b"CDT"[..], -5),
                (&b"MST"[..], -7),
                (&b"MDT"[..], -6),
                (&b"PST"[..], -8),
                (&b"PDT"[..], -7),
            ],
            false,
        )(input)
        .or_else(|| match input {
            [
                b'a'..=b'i' | b'k'..=b'z' | b'A'..=b'I' | b'K'..=b'Z',
                rest @ ..,
            ] => Some(ParsedItem(rest, 0)),
            _ => None,
        });
        if let Some(zone_literal) = zone_literal {
            let input = zone_literal.assign_value_to(&mut parsed.offset_hour);
            parsed.offset_minute = Some(0);
            parsed.offset_second = Some(0);
            return Ok(input);
        }

        let ParsedItem(input, offset_sign) = sign(input).ok_or(InvalidComponent("offset hour"))?;
        let input = exactly_n_digits::<u8, 2>(input)
            .ok_or(InvalidComponent("offset hour"))?
            .map(|offset_hour| {
                if offset_sign == b'-' {
                    -(offset_hour as i8)
                } else {
                    offset_hour as _
                }
            })
            .assign_value_to(&mut parsed.offset_hour);
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("offset minute"))?
            .assign_value_to(&mut parsed.offset_minute);

        Ok(input)
    }
}

impl sealed::Sealed for Rfc3339 {
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        use crate::error::ParseFromDescription::{InvalidComponent, InvalidLiteral};
        use crate::parsing::combinator::{
            any_digit, ascii_char, ascii_char_ignore_case, exactly_n_digits, sign,
        };

        let dash = ascii_char::<b'-'>;
        let colon = ascii_char::<b':'>;

        let input = exactly_n_digits::<_, 4>(input)
            .ok_or(InvalidComponent("year"))?
            .map(|year: u32| year as _)
            .assign_value_to(&mut parsed.year);
        let input = dash(input).ok_or(InvalidLiteral)?.into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("month"))?
            .flat_map_res(Month::from_number)
            .map_err(error::TryFromParsed::ComponentRange)?
            .assign_value_to(&mut parsed.month);
        let input = dash(input).ok_or(InvalidLiteral)?.into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("day"))?
            .assign_value_to(&mut parsed.day);
        let input = ascii_char_ignore_case::<b'T'>(input)
            .ok_or(InvalidLiteral)?
            .into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("hour"))?
            .assign_value_to(&mut parsed.hour_24);
        let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("minute"))?
            .assign_value_to(&mut parsed.minute);
        let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("second"))?
            .assign_value_to(&mut parsed.second);
        let input = if let Some(ParsedItem(input, ())) = ascii_char::<b'.'>(input) {
            let ParsedItem(mut input, mut value) = any_digit(input)
                .ok_or(InvalidComponent("subsecond"))?
                .map(|v| (v - b'0') as u32 * 100_000_000);

            let mut multiplier = 10_000_000;
            while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                value += (digit - b'0') as u32 * multiplier;
                input = new_input;
                multiplier /= 10;
            }

            ParsedItem(input, value).assign_value_to(&mut parsed.subsecond)
        } else {
            input
        };

        // The RFC explicitly allows leap seconds. We don't currently support them, so treat it as
        // the previous moment.
        if parsed.second == Some(60) {
            parsed.second = Some(59);
            parsed.subsecond = Some(999_999_999);
        }

        if let Some(ParsedItem(input, ())) = ascii_char_ignore_case::<b'Z'>(input) {
            parsed.offset_hour = Some(0);
            parsed.offset_minute = Some(0);
            parsed.offset_second = Some(0);
            return Ok(input);
        }

        let ParsedItem(input, offset_sign) = sign(input).ok_or(InvalidComponent("offset hour"))?;
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("offset hour"))?
            .map(|offset_hour: u8| {
                if offset_sign == b'-' {
                    -(offset_hour as i8)
                } else {
                    offset_hour as _
                }
            })
            .assign_value_to(&mut parsed.offset_hour);
        let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
        let input = exactly_n_digits::<_, 2>(input)
            .ok_or(InvalidComponent("offset minute"))?
            .assign_value_to(&mut parsed.offset_minute);

        Ok(input)
    }

    fn parse_offset_date_time(&self, input: &[u8]) -> Result<OffsetDateTime, error::Parse> {
        use crate::error::ParseFromDescription::{InvalidComponent, InvalidLiteral};
        use crate::parsing::combinator::{
            any_digit, ascii_char, ascii_char_ignore_case, exactly_n_digits, sign,
        };

        let dash = ascii_char::<b'-'>;
        let colon = ascii_char::<b':'>;

        let ParsedItem(input, year) =
            exactly_n_digits::<u32, 4>(input).ok_or(InvalidComponent("year"))?;
        let input = dash(input).ok_or(InvalidLiteral)?.into_inner();
        let ParsedItem(input, month) =
            exactly_n_digits::<_, 2>(input).ok_or(InvalidComponent("month"))?;
        let input = dash(input).ok_or(InvalidLiteral)?.into_inner();
        let ParsedItem(input, day) =
            exactly_n_digits::<_, 2>(input).ok_or(InvalidComponent("day"))?;
        let input = ascii_char_ignore_case::<b'T'>(input)
            .ok_or(InvalidLiteral)?
            .into_inner();
        let ParsedItem(input, hour) =
            exactly_n_digits::<_, 2>(input).ok_or(InvalidComponent("hour"))?;
        let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
        let ParsedItem(input, minute) =
            exactly_n_digits::<_, 2>(input).ok_or(InvalidComponent("minute"))?;
        let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
        let ParsedItem(input, mut second) =
            exactly_n_digits::<_, 2>(input).ok_or(InvalidComponent("second"))?;
        let ParsedItem(input, mut nanosecond) =
            if let Some(ParsedItem(input, ())) = ascii_char::<b'.'>(input) {
                let ParsedItem(mut input, mut value) = any_digit(input)
                    .ok_or(InvalidComponent("subsecond"))?
                    .map(|v| (v - b'0') as u32 * 100_000_000);

                let mut multiplier = 10_000_000;
                while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                    value += (digit - b'0') as u32 * multiplier;
                    input = new_input;
                    multiplier /= 10;
                }

                ParsedItem(input, value)
            } else {
                ParsedItem(input, 0)
            };
        let ParsedItem(input, offset) = {
            if let Some(ParsedItem(input, ())) = ascii_char_ignore_case::<b'Z'>(input) {
                ParsedItem(input, UtcOffset::UTC)
            } else {
                let ParsedItem(input, offset_sign) =
                    sign(input).ok_or(InvalidComponent("offset hour"))?;
                let ParsedItem(input, offset_hour) =
                    exactly_n_digits::<u8, 2>(input).ok_or(InvalidComponent("offset hour"))?;
                let input = colon(input).ok_or(InvalidLiteral)?.into_inner();
                let ParsedItem(input, offset_minute) =
                    exactly_n_digits::<u8, 2>(input).ok_or(InvalidComponent("offset minute"))?;
                UtcOffset::from_hms(
                    if offset_sign == b'-' {
                        -(offset_hour as i8)
                    } else {
                        offset_hour as _
                    },
                    offset_minute as _,
                    0,
                )
                .map(|offset| ParsedItem(input, offset))
                .map_err(|mut err| {
                    // Provide the user a more accurate error.
                    if err.name == "hours" {
                        err.name = "offset hour";
                    } else if err.name == "minutes" {
                        err.name = "offset minute";
                    }
                    err
                })
                .map_err(TryFromParsed::ComponentRange)?
            }
        };

        if !input.is_empty() {
            return Err(error::Parse::UnexpectedTrailingCharacters);
        }

        // The RFC explicitly allows leap seconds. We don't currently support them, so treat it as
        // the previous moment.
        if second == 60 {
            second = 59;
            nanosecond = 999_999_999;
        }

        Ok(Month::from_number(month)
            .and_then(|month| Date::from_calendar_date(year as _, month, day))
            .and_then(|date| date.with_hms_nano(hour, minute, second, nanosecond))
            .map(|date| date.assume_offset(offset))
            .map_err(TryFromParsed::ComponentRange)?)
    }
}
// endregion well-known formats

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8263() {
//    rusty_monitor::set_test_id(8263);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut u8_0: u8 = 12u8;
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 10i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 999999u32;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 30u8;
    let mut u8_3: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_4: u8 = 96u8;
    let mut u8_5: u8 = 20u8;
    let mut u8_6: u8 = 91u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_0: i32 = 280i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut u32_2: u32 = 999999u32;
    let mut u8_7: u8 = 50u8;
    let mut u8_8: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_0, u8_8, u32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_2);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 2i32;
    let mut month_1: month::Month = crate::month::Month::September;
    let mut i32_2: i32 = 218i32;
    let mut month_2: month::Month = crate::month::Month::February;
    let mut i32_3: i32 = 178i32;
    let mut month_3: month::Month = crate::month::Month::July;
    let mut i32_4: i32 = 111i32;
    let mut u8_9: u8 = crate::util::days_in_year_month(i32_2, month_1);
    let mut u8_10: u8 = crate::util::days_in_year_month(i32_3, month_2);
    let mut u8_11: u8 = crate::util::days_in_year_month(i32_4, month_3);
    let mut u8_12: u8 = crate::util::days_in_year_month(i32_1, month_0);
//    panic!("From RustyUnit with love");
}
}