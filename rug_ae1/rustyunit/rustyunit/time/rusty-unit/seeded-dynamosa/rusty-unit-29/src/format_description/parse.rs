//! Parse a format description into a standardized representation.

use alloc::vec::Vec;

use crate::error::InvalidFormatDescription;
use crate::format_description::component::{Component, NakedComponent};
use crate::format_description::{helper, modifier, FormatItem};

/// The item parsed and remaining chunk of the format description after one iteration.
#[derive(Debug)]
pub(crate) struct ParsedItem<'a> {
    /// The item that was parsed.
    pub(crate) item: FormatItem<'a>,
    /// What is left of the input string after the item was parsed.
    pub(crate) remaining: &'a [u8],
}

/// Parse a component from the format description. Neither the leading nor trailing bracket should
/// be present in the parameter.
fn parse_component(mut s: &[u8], index: &mut usize) -> Result<Component, InvalidFormatDescription> {
    // Trim any whitespace between the opening bracket and the component name.
    s = helper::consume_whitespace(s, index);

    // Everything before the first whitespace is the component name.
    let component_index = *index;
    let whitespace_loc = s
        .iter()
        .position(u8::is_ascii_whitespace)
        .unwrap_or(s.len());
    *index += whitespace_loc;
    let component_name = &s[..whitespace_loc];
    s = &s[whitespace_loc..];
    s = helper::consume_whitespace(s, index);

    Ok(NakedComponent::parse(component_name, component_index)?
        .attach_modifiers(&modifier::Modifiers::parse(component_name, s, index)?))
}

/// Parse a literal string from the format description.
fn parse_literal<'a>(s: &'a [u8], index: &mut usize) -> ParsedItem<'a> {
    let loc = s.iter().position(|&c| c == b'[').unwrap_or(s.len());
    *index += loc;
    ParsedItem {
        item: FormatItem::Literal(&s[..loc]),
        remaining: &s[loc..],
    }
}

/// Parse either a literal or a component from the format description.
fn parse_item<'a>(
    s: &'a [u8],
    index: &mut usize,
) -> Result<ParsedItem<'a>, InvalidFormatDescription> {
    if let [b'[', b'[', remaining @ ..] = s {
        *index += 2;
        return Ok(ParsedItem {
            item: FormatItem::Literal(&[b'[']),
            remaining,
        });
    };

    if s.starts_with(&[b'[']) {
        if let Some(bracket_index) = s.iter().position(|&c| c == b']') {
            *index += 1; // opening bracket
            let ret_val = ParsedItem {
                item: FormatItem::Component(parse_component(&s[1..bracket_index], index)?),
                remaining: &s[bracket_index + 1..],
            };
            *index += 1; // closing bracket
            Ok(ret_val)
        } else {
            Err(InvalidFormatDescription::UnclosedOpeningBracket { index: *index })
        }
    } else {
        Ok(parse_literal(s, index))
    }
}

/// Parse a sequence of items from the format description.
///
/// The syntax for the format description can be found in [the
/// book](https://time-rs.github.io/book/api/format-description.html).
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
pub fn parse(s: &str) -> Result<Vec<FormatItem<'_>>, InvalidFormatDescription> {
    let mut compound = Vec::new();
    let mut loc = 0;

    let mut s = s.as_bytes();

    while !s.is_empty() {
        let ParsedItem { item, remaining } = parse_item(s, &mut loc)?;
        s = remaining;
        compound.push(item);
    }

    Ok(compound)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_163() {
//    rusty_monitor::set_test_id(163);
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_461() {
//    rusty_monitor::set_test_id(461);
    let mut u16_0: u16 = 367u16;
    let mut i32_0: i32 = 331i32;
    let mut u16_1: u16 = 365u16;
    let mut i32_1: i32 = 376i32;
    let mut u16_2: u16 = 365u16;
    let mut i32_2: i32 = 77i32;
    let mut u16_3: u16 = 60u16;
    let mut i32_3: i32 = 5i32;
    let mut u16_4: u16 = 0u16;
    let mut i32_4: i32 = 1000000i32;
    let mut u16_5: u16 = 34u16;
    let mut i32_5: i32 = 336i32;
    let mut u16_6: u16 = 0u16;
    let mut i32_6: i32 = -27i32;
    let mut u16_7: u16 = 10u16;
    let mut i32_7: i32 = 1000000000i32;
    let mut u16_8: u16 = 60u16;
    let mut i32_8: i32 = 88i32;
    let mut u16_9: u16 = 10u16;
    let mut i32_9: i32 = 100i32;
    let mut u16_10: u16 = 366u16;
    let mut i32_10: i32 = 36525i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_10, u16_10);
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_9, u16_9);
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_8);
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_7);
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_6);
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_5);
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_4);
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_3);
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_10: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_179() {
//    rusty_monitor::set_test_id(179);
    let mut i64_0: i64 = -14i64;
    let mut i64_1: i64 = 1000000000i64;
    let mut i64_2: i64 = 86400i64;
    let mut i64_3: i64 = 24i64;
    let mut i64_4: i64 = 253402300799i64;
    let mut i64_5: i64 = 12i64;
    let mut i64_6: i64 = 0i64;
    let mut i64_7: i64 = 3600i64;
    let mut i64_8: i64 = 60i64;
    let mut i64_9: i64 = 45i64;
    let mut i64_10: i64 = -68i64;
    let mut i64_11: i64 = 1000000000i64;
    let mut i64_12: i64 = 47i64;
    let mut i64_13: i64 = 1i64;
    let mut i64_14: i64 = 24i64;
    let mut i64_15: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_15);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_14);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_13);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_12);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_10);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
//    panic!("From RustyUnit with love");
}
}