//! Combinators for rules as defined in an RFC.
//!
//! These rules have been converted strictly following the ABNF syntax as specified in [RFC 2234].
//!
//! [RFC 2234]: https://datatracker.ietf.org/doc/html/rfc2234

pub(crate) mod rfc2234;
pub(crate) mod rfc2822;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4159() {
    rusty_monitor::set_test_id(4159);
    let mut i8_0: i8 = 21i8;
    let mut u16_0: u16 = 3u16;
    let mut i32_0: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 139i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 130i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_1: i8 = 10i8;
    let mut i8_2: i8 = -84i8;
    let mut i8_3: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i8_4: i8 = 99i8;
    let mut i8_5: i8 = -77i8;
    let mut i8_6: i8 = 120i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i8_7: i8 = -53i8;
    let mut i8_8: i8 = -94i8;
    let mut i8_9: i8 = -102i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut u32_1: u32 = 93u32;
    let mut u8_3: u8 = 39u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_10: i8 = 54i8;
    let mut i8_11: i8 = -52i8;
    let mut i8_12: i8 = -66i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_11, i8_10);
    let mut u32_2: u32 = 5u32;
    let mut u8_6: u8 = 72u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 17u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_13: i8 = -40i8;
    let mut i8_14: i8 = -106i8;
    let mut i8_15: i8 = 95i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_15, i8_14, i8_13);
    let mut i8_16: i8 = -73i8;
    let mut i8_17: i8 = 51i8;
    let mut i8_18: i8 = -18i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_18, i8_17, i8_16);
    let mut i8_19: i8 = 69i8;
    let mut i8_20: i8 = 69i8;
    let mut i8_21: i8 = 92i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_21, i8_20, i8_19);
    let mut u32_3: u32 = 9u32;
    let mut u8_9: u8 = 81u8;
    let mut u8_10: u8 = 7u8;
    let mut u8_11: u8 = 54u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_22: i8 = -3i8;
    let mut i8_23: i8 = 80i8;
    let mut i8_24: i8 = -62i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_24, i8_23, i8_22);
    let mut i8_25: i8 = -68i8;
    let mut i8_26: i8 = 99i8;
    let mut i8_27: i8 = -101i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_27, i8_26, i8_25);
    let mut i8_28: i8 = -123i8;
    let mut i8_29: i8 = 39i8;
    let mut i8_30: i8 = 13i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_30, i8_29, i8_28);
    let mut u32_4: u32 = 41u32;
    let mut u8_12: u8 = 42u8;
    let mut u8_13: u8 = 7u8;
    let mut u8_14: u8 = 11u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_31: i8 = 14i8;
    let mut i8_32: i8 = -66i8;
    let mut i8_33: i8 = -11i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_33, i8_32, i8_31);
    let mut i8_34: i8 = 49i8;
    let mut i8_35: i8 = -107i8;
    let mut i8_36: i8 = 40i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_36, i8_35, i8_34);
    panic!("From RustyUnit with love");
}
}