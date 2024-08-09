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
fn rusty_test_4158() {
    rusty_monitor::set_test_id(4158);
    let mut i64_0: i64 = 75i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 67u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 52u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = -2i8;
    let mut i8_1: i8 = -59i8;
    let mut i8_2: i8 = -106i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u32_1: u32 = 20u32;
    let mut u8_3: u8 = 7u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_3: i8 = -106i8;
    let mut i8_4: i8 = 115i8;
    let mut i8_5: i8 = 77i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 9i8;
    let mut i8_7: i8 = -40i8;
    let mut i8_8: i8 = -15i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_0: i32 = 59i32;
    let mut i64_3: i64 = -144i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut i8_9: i8 = -1i8;
    let mut i8_10: i8 = -14i8;
    let mut i8_11: i8 = 120i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_1: i32 = 47i32;
    let mut i64_4: i64 = -232i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_1);
    let mut i8_12: i8 = -43i8;
    let mut i8_13: i8 = -28i8;
    let mut i8_14: i8 = 54i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_5: i64 = -29i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i8_15: i8 = 122i8;
    let mut i8_16: i8 = 68i8;
    let mut i8_17: i8 = 111i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 0u32;
    let mut u8_6: u8 = 28u8;
    let mut u8_7: u8 = 57u8;
    let mut u8_8: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_18: i8 = 46i8;
    let mut i64_6: i64 = -70i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut i32_2: i32 = 200i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    panic!("From RustyUnit with love");
}
}