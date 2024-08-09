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
fn rusty_test_7512() {
    rusty_monitor::set_test_id(7512);
    let mut u16_0: u16 = 47u16;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 11u8;
    let mut i64_0: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut f64_0: f64 = -184.369323f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_1: i64 = -24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = -4i8;
    let mut i8_1: i8 = -100i8;
    let mut i8_2: i8 = -48i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -65i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = -33i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 62i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut u32_0: u32 = 28u32;
    let mut u8_3: u8 = 73u8;
    let mut u8_4: u8 = 43u8;
    let mut u8_5: u8 = 91u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = -138i32;
    let mut i64_5: i64 = -35i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i8_3: i8 = -67i8;
    let mut i8_4: i8 = -88i8;
    let mut i8_5: i8 = 79i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_6: i64 = 112i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = 15i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f32_0: f32 = 66.604061f32;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_7: i64 = -7i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_15: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_15);
    let mut u32_1: u32 = 55u32;
    let mut u8_6: u8 = 23u8;
    let mut u8_7: u8 = 81u8;
    let mut u8_8: u8 = 55u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i64_8: i64 = -74i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut i8_9: i8 = -40i8;
    let mut i8_10: i8 = 18i8;
    let mut i8_11: i8 = -16i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_1: i32 = 15i32;
    let mut i64_9: i64 = 63i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_1);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_19);
    let mut i8_12: i8 = -8i8;
    let mut i8_13: i8 = 106i8;
    let mut i8_14: i8 = -46i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i32_2: i32 = -80i32;
    let mut i64_10: i64 = 141i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_2);
    let mut i64_11: i64 = 90i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut i64_12: i64 = 60i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::hours(i64_12);
    let mut i8_15: i8 = 24i8;
    let mut i8_16: i8 = -58i8;
    let mut i8_17: i8 = -29i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 23u32;
    let mut u8_9: u8 = 50u8;
    let mut u8_10: u8 = 57u8;
    let mut u8_11: u8 = 99u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_2);
    let mut i32_3: i32 = -103i32;
    let mut i64_13: i64 = 8i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_13, i32_3);
    let mut i64_14: i64 = 15i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::microseconds(i64_14);
    let mut i64_15: i64 = 83i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::minutes(i64_15);
    let mut i64_16: i64 = -70i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::seconds(i64_16);
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_28, duration_27);
    let mut duration_30: std::time::Duration = crate::duration::Duration::abs_std(duration_29);
    let mut i8_18: i8 = 110i8;
    let mut i8_19: i8 = 21i8;
    let mut i8_20: i8 = 117i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_17: i64 = 51i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::microseconds(i64_17);
    let mut i64_18: i64 = 174i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::days(i64_18);
    let mut i64_19: i64 = 8i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::seconds(i64_19);
    let mut duration_34: std::time::Duration = crate::duration::Duration::abs_std(duration_33);
    let mut i128_0: i128 = 209i128;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_20: i64 = 18i64;
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_20);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_36);
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut u32_3: u32 = 21u32;
    let mut u8_12: u8 = 94u8;
    let mut u8_13: u8 = 32u8;
    let mut u8_14: u8 = 22u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_3);
    let mut i8_21: i8 = 20i8;
    let mut i8_22: i8 = -57i8;
    let mut i8_23: i8 = -85i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f64_1: f64 = 29.002327f64;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_38: std::time::Duration = crate::duration::Duration::abs_std(duration_37);
    let mut i64_21: i64 = -117i64;
    let mut duration_39: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_21);
    let mut i64_22: i64 = 41i64;
    let mut duration_40: crate::duration::Duration = crate::duration::Duration::microseconds(i64_22);
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::abs(duration_40);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_41);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut i32_4: i32 = -36i32;
    let mut i8_24: i8 = 35i8;
    let mut i8_25: i8 = -41i8;
    let mut i8_26: i8 = -15i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_5: i32 = -116i32;
    let mut i64_23: i64 = -111i64;
    let mut duration_42: crate::duration::Duration = crate::duration::Duration::new(i64_23, i32_5);
    let mut duration_43: crate::duration::Duration = crate::duration::Duration::abs(duration_42);
    let mut duration_44: std::time::Duration = crate::duration::Duration::abs_std(duration_43);
    let mut u16_1: u16 = 13u16;
    let mut i32_6: i32 = 31i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut u16_2: u16 = crate::util::days_in_year(i32_4);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}
}