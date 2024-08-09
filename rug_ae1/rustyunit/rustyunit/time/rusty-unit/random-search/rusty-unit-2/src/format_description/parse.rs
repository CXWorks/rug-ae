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
fn rusty_test_4945() {
    rusty_monitor::set_test_id(4945);
    let mut i64_0: i64 = -22i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 11i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = -148i32;
    let mut i64_1: i64 = 17i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_2: i64 = 133i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 56u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = -60i32;
    let mut i64_3: i64 = -225i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_6);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i64_4: i64 = -87i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = 67i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut u32_1: u32 = 1u32;
    let mut u8_3: u8 = 65u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 33u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 3u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 92u8;
    let mut u8_8: u8 = 84u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_2);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut f32_0: f32 = 35.970508f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_6: i64 = -95i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i64_7: i64 = 172i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = 71i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut i64_9: i64 = 29i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut i32_3: i32 = -85i32;
    let mut i64_10: i64 = 109i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_3);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i8_0: i8 = 93i8;
    let mut i8_1: i8 = 14i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    panic!("From RustyUnit with love");
}
}