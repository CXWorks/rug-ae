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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4865() {
    rusty_monitor::set_test_id(4865);
    let mut i64_0: i64 = 135i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = -42i8;
    let mut i8_1: i8 = 11i8;
    let mut i8_2: i8 = 101i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 72i8;
    let mut i8_4: i8 = -96i8;
    let mut i8_5: i8 = 55i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 72i32;
    let mut i64_1: i64 = 157i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i8_6: i8 = 8i8;
    let mut i8_7: i8 = 20i8;
    let mut i8_8: i8 = -55i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -74i8;
    let mut i8_10: i8 = -85i8;
    let mut i8_11: i8 = 17i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_12: i8 = -55i8;
    let mut i8_13: i8 = 52i8;
    let mut i8_14: i8 = 26i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -113i8;
    let mut i8_16: i8 = -71i8;
    let mut i8_17: i8 = -19i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_1: u32 = 65u32;
    let mut u8_3: u8 = 87u8;
    let mut u8_4: u8 = 63u8;
    let mut u8_5: u8 = 43u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_18: i8 = -27i8;
    let mut i8_19: i8 = -56i8;
    let mut i8_20: i8 = 92i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i32_1: i32 = -26i32;
    let mut i64_2: i64 = 18i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i64_3: i64 = 112i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i8_21: i8 = -7i8;
    let mut i8_22: i8 = 80i8;
    let mut i8_23: i8 = 33i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f64_0: f64 = -51.464829f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 80i32;
    let mut i64_4: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut i32_3: i32 = 182i32;
    let mut i64_5: i64 = 65i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut u32_2: u32 = 31u32;
    let mut u8_6: u8 = 67u8;
    let mut u8_7: u8 = 10u8;
    let mut u8_8: u8 = 46u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_24: i8 = -114i8;
    let mut i8_25: i8 = 4i8;
    let mut i8_26: i8 = -84i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = -92i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i8_27: i8 = 19i8;
    let mut i8_28: i8 = 89i8;
    let mut i8_29: i8 = 0i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 30u32;
    let mut u8_9: u8 = 97u8;
    let mut u8_10: u8 = 22u8;
    let mut u8_11: u8 = 10u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_30: i8 = -27i8;
    let mut i8_31: i8 = -48i8;
    let mut i8_32: i8 = 29i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i64_7: i64 = -24i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut i8_33: i8 = -31i8;
    let mut i8_34: i8 = 37i8;
    let mut i8_35: i8 = -14i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = -16i8;
    let mut i8_37: i8 = -77i8;
    let mut i8_38: i8 = 87i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i32_4: i32 = 31i32;
    let mut i64_8: i64 = 89i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_4);
    let mut i8_39: i8 = 33i8;
    let mut i8_40: i8 = -10i8;
    let mut i8_41: i8 = -87i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u32_4: u32 = 92u32;
    let mut u8_12: u8 = 60u8;
    let mut u8_13: u8 = 38u8;
    let mut u8_14: u8 = 34u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i128_0: i128 = 110i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_42: i8 = -53i8;
    let mut i8_43: i8 = -35i8;
    let mut i8_44: i8 = -113i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut f32_0: f32 = -209.354280f32;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_45: i8 = 75i8;
    let mut i8_46: i8 = -13i8;
    let mut i8_47: i8 = -26i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i8_48: i8 = -71i8;
    let mut i8_49: i8 = 97i8;
    let mut i8_50: i8 = -55i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i128_1: i128 = 94i128;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_5: u32 = 8u32;
    let mut u8_15: u8 = 38u8;
    let mut u8_16: u8 = 2u8;
    let mut u8_17: u8 = 11u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i8_51: i8 = -115i8;
    let mut i8_52: i8 = 21i8;
    let mut i8_53: i8 = -25i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut i64_9: i64 = -43i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::microseconds(i64_9);
    let mut u32_6: u32 = 96u32;
    let mut u8_18: u8 = 24u8;
    let mut u8_19: u8 = 74u8;
    let mut u8_20: u8 = 78u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i8_54: i8 = 6i8;
    let mut i8_55: i8 = 32i8;
    let mut i8_56: i8 = 54i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut i64_10: i64 = -127i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::days(i64_10);
    let mut i8_57: i8 = 66i8;
    let mut i8_58: i8 = 113i8;
    let mut i8_59: i8 = 70i8;
    let mut utcoffset_19: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_59, i8_58, i8_57);
    let mut i8_60: i8 = -31i8;
    let mut i8_61: i8 = -121i8;
    let mut i8_62: i8 = -75i8;
    let mut utcoffset_20: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_62, i8_61, i8_60);
    let mut i8_63: i8 = 17i8;
    let mut i8_64: i8 = -117i8;
    let mut i8_65: i8 = -128i8;
    let mut utcoffset_21: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_65, i8_64, i8_63);
    let mut u32_7: u32 = 33u32;
    let mut u8_21: u8 = 83u8;
    let mut u8_22: u8 = 33u8;
    let mut u8_23: u8 = 31u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut i32_5: i32 = 110i32;
    let mut i64_11: i64 = -63i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new(i64_11, i32_5);
    let mut f32_1: f32 = 3.194902f32;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    panic!("From RustyUnit with love");
}
}