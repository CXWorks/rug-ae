//! Rules defined in [RFC 2234].
//!
//! [RFC 2234]: https://datatracker.ietf.org/doc/html/rfc2234

use crate::parsing::ParsedItem;

/// Consume exactly one space or tab.
pub(crate) const fn wsp(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    match input {
        [b' ' | b'\t', rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4222() {
    rusty_monitor::set_test_id(4222);
    let mut i32_0: i32 = -159i32;
    let mut u32_0: u32 = 3u32;
    let mut u8_0: u8 = 92u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -74i8;
    let mut i8_1: i8 = -59i8;
    let mut i8_2: i8 = -99i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -119i8;
    let mut i8_4: i8 = 119i8;
    let mut i8_5: i8 = 104i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -34i8;
    let mut i8_7: i8 = 74i8;
    let mut i8_8: i8 = -32i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 95u32;
    let mut u8_3: u8 = 91u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 45u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = -50i8;
    let mut i8_10: i8 = 21i8;
    let mut i8_11: i8 = -39i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_2: u32 = 21u32;
    let mut u8_6: u8 = 18u8;
    let mut u8_7: u8 = 42u8;
    let mut u8_8: u8 = 34u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_12: i8 = -46i8;
    let mut i8_13: i8 = 34i8;
    let mut i8_14: i8 = -29i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -4i8;
    let mut i8_16: i8 = -24i8;
    let mut i8_17: i8 = 15i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = -92i8;
    let mut i8_19: i8 = 27i8;
    let mut i8_20: i8 = 63i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = -60i8;
    let mut i8_22: i8 = 95i8;
    let mut i8_23: i8 = -117i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_3: u32 = 35u32;
    let mut u8_9: u8 = 76u8;
    let mut u8_10: u8 = 26u8;
    let mut u8_11: u8 = 37u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_24: i8 = -55i8;
    let mut i8_25: i8 = -13i8;
    let mut i8_26: i8 = -114i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 3i8;
    let mut i8_28: i8 = 9i8;
    let mut i8_29: i8 = 39i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = 46i8;
    let mut i8_31: i8 = 70i8;
    let mut i8_32: i8 = -5i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut u32_4: u32 = 29u32;
    let mut u8_12: u8 = 51u8;
    let mut u8_13: u8 = 73u8;
    let mut u8_14: u8 = 42u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_33: i8 = 57i8;
    let mut i8_34: i8 = 39i8;
    let mut i8_35: i8 = 21i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = -8i8;
    let mut i8_37: i8 = 103i8;
    let mut i8_38: i8 = -10i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i8_39: i8 = -47i8;
    let mut i8_40: i8 = -60i8;
    let mut i8_41: i8 = 68i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u32_5: u32 = 82u32;
    let mut u8_15: u8 = 82u8;
    let mut u8_16: u8 = 67u8;
    let mut u8_17: u8 = 24u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_6: u32 = 24u32;
    let mut u8_18: u8 = 39u8;
    let mut u8_19: u8 = 87u8;
    let mut u8_20: u8 = 71u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i8_42: i8 = -76i8;
    let mut i8_43: i8 = 22i8;
    let mut i8_44: i8 = -88i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i8_45: i8 = -76i8;
    let mut i8_46: i8 = 63i8;
    let mut i8_47: i8 = 83i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i8_48: i8 = 42i8;
    let mut i8_49: i8 = -47i8;
    let mut i8_50: i8 = 89i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i8_51: i8 = -76i8;
    let mut i8_52: i8 = 61i8;
    let mut i8_53: i8 = -10i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut u32_7: u32 = 21u32;
    let mut u8_21: u8 = 92u8;
    let mut u8_22: u8 = 85u8;
    let mut u8_23: u8 = 29u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut i8_54: i8 = 59i8;
    let mut i8_55: i8 = -69i8;
    let mut i8_56: i8 = 35i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut i32_1: i32 = -122i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 36i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    panic!("From RustyUnit with love");
}
}