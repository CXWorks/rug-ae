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
fn rusty_test_4778() {
    rusty_monitor::set_test_id(4778);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 23u8;
    let mut i64_0: i64 = 188i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 78i8;
    let mut i8_1: i8 = -43i8;
    let mut i8_2: i8 = 12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -126i8;
    let mut i8_4: i8 = -29i8;
    let mut i8_5: i8 = 61i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -39i8;
    let mut i8_7: i8 = 28i8;
    let mut i8_8: i8 = 54i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i64_1: i64 = 190i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i128_1: i128 = 59i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i8_9: i8 = -49i8;
    let mut i8_10: i8 = -58i8;
    let mut i8_11: i8 = 42i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = -27i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i128_2: i128 = -163i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut i32_0: i32 = 68i32;
    let mut i64_3: i64 = 84i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = -108i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut f32_0: f32 = 155.060141f32;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_5: i64 = 21i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i8_12: i8 = 49i8;
    let mut i8_13: i8 = 104i8;
    let mut i8_14: i8 = -90i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 22i8;
    let mut i8_16: i8 = -19i8;
    let mut i8_17: i8 = -118i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i128_3: i128 = -120i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_3);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 56u8;
    let mut u8_4: u8 = 76u8;
    let mut u8_5: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = 137i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut i64_7: i64 = -42i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut f32_1: f32 = -157.322231f32;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_17);
    let mut i8_18: i8 = 30i8;
    let mut i8_19: i8 = 109i8;
    let mut i8_20: i8 = 30i8;
    let mut i8_21: i8 = 23i8;
    let mut i8_22: i8 = -8i8;
    let mut i8_23: i8 = -21i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f32_2: f32 = -54.798605f32;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_21: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut i64_8: i64 = 28i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_22, duration_21);
    let mut i64_9: i64 = -139i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut duration_25: std::time::Duration = crate::duration::Duration::abs_std(duration_24);
    let mut i32_1: i32 = 97i32;
    let mut i64_10: i64 = 130i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_1);
    let mut u32_2: u32 = 27u32;
    let mut u8_6: u8 = 37u8;
    let mut u8_7: u8 = 15u8;
    let mut u8_8: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_11: i64 = 116i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut i8_24: i8 = 86i8;
    let mut i8_25: i8 = 82i8;
    let mut i8_26: i8 = -19i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_2: i32 = 78i32;
    let mut i64_12: i64 = 34i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_12, i32_2);
    let mut duration_29: std::time::Duration = crate::duration::Duration::abs_std(duration_28);
    let mut i8_27: i8 = 12i8;
    let mut i8_28: i8 = -11i8;
    let mut i8_29: i8 = 120i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut f64_0: f64 = -69.120604f64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_31: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut u32_3: u32 = 35u32;
    let mut u8_9: u8 = 35u8;
    let mut u8_10: u8 = 9u8;
    let mut u8_11: u8 = 96u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_13: i64 = 147i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_13);
    let mut duration_33: std::time::Duration = crate::duration::Duration::abs_std(duration_32);
    let mut i8_30: i8 = -97i8;
    let mut i8_31: i8 = -70i8;
    let mut i8_32: i8 = -22i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 19i8;
    let mut i8_34: i8 = 55i8;
    let mut i8_35: i8 = 28i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i64_14: i64 = -123i64;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_14);
    let mut i32_3: i32 = -110i32;
    let mut i64_15: i64 = -60i64;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_15);
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_35, i32_3);
    let mut duration_37: std::time::Duration = crate::duration::Duration::abs_std(duration_36);
    let mut i64_16: i64 = -59i64;
    let mut duration_38: crate::duration::Duration = crate::duration::Duration::seconds(i64_16);
    let mut i8_36: i8 = -70i8;
    let mut i8_37: i8 = 66i8;
    let mut i8_38: i8 = -34i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_11);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut i8_39: i8 = 49i8;
    let mut i8_40: i8 = -11i8;
    let mut i8_41: i8 = 36i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u32_4: u32 = 65u32;
    let mut u8_12: u8 = 86u8;
    let mut u8_13: u8 = 81u8;
    let mut u8_14: u8 = 81u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i64_17: i64 = -94i64;
    let mut duration_39: crate::duration::Duration = crate::duration::Duration::minutes(i64_17);
    let mut i8_42: i8 = -38i8;
    let mut i8_43: i8 = -51i8;
    let mut i8_44: i8 = -13i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_40: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut i32_4: i32 = 38i32;
    let mut i64_18: i64 = -93i64;
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_18, i32_4);
    let mut duration_42: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_41, duration_40);
    let mut i8_45: i8 = 20i8;
    let mut i8_46: i8 = -8i8;
    let mut i8_47: i8 = -119i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    panic!("From RustyUnit with love");
}
}