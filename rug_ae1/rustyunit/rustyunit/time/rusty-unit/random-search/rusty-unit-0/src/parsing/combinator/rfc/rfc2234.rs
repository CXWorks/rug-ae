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
fn rusty_test_4989() {
    rusty_monitor::set_test_id(4989);
    let mut f64_0: f64 = 3.373019f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 16i8;
    let mut i8_2: i8 = -76i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 44i8;
    let mut i8_4: i8 = -39i8;
    let mut i8_5: i8 = 67i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -77i8;
    let mut i8_7: i8 = 9i8;
    let mut i8_8: i8 = -46i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 42u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 89u8;
    let mut u8_2: u8 = 79u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 34i8;
    let mut i8_10: i8 = -38i8;
    let mut i8_11: i8 = -123i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i128_0: i128 = -38i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut f64_1: f64 = 11.542333f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_1: u32 = 34u32;
    let mut u8_3: u8 = 66u8;
    let mut u8_4: u8 = 84u8;
    let mut u8_5: u8 = 47u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 31i32;
    let mut i64_0: i64 = 40i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i8_12: i8 = -70i8;
    let mut i8_13: i8 = -53i8;
    let mut i8_14: i8 = 55i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_1: i64 = 2i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_2: i64 = -20i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut f32_0: f32 = 79.688103f32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i128_1: i128 = -91i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_15: i8 = 89i8;
    let mut i8_16: i8 = 56i8;
    let mut i8_17: i8 = 88i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_add(date_0, duration_0);
    panic!("From RustyUnit with love");
}
}