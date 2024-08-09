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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7416() {
    rusty_monitor::set_test_id(7416);
    let mut i64_0: i64 = 91i64;
    let mut i8_0: i8 = -24i8;
    let mut i8_1: i8 = -112i8;
    let mut i8_2: i8 = -22i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 20i8;
    let mut i8_5: i8 = -121i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 92i32;
    let mut i64_1: i64 = -66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_2: i64 = -75i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut i64_3: i64 = -69i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut f32_0: f32 = -95.976739f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i32_1: i32 = 145i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 69i32;
    let mut f32_1: f32 = -6.775078f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_2);
    let mut i64_4: i64 = 44i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i32_3: i32 = 20i32;
    let mut i64_5: i64 = -13i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut i64_6: i64 = -96i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i64_7: i64 = 156i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i32_4: i32 = -44i32;
    let mut i32_5: i32 = -86i32;
    let mut i64_8: i64 = -121i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_5);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_4);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::hour(primitivedatetime_1);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    panic!("From RustyUnit with love");
}
}