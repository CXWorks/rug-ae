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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8237() {
//    rusty_monitor::set_test_id(8237);
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut i64_0: i64 = -152i64;
    let mut u32_0: u32 = 1000u32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_0: u8 = 0u8;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 59u8;
    let mut u8_3: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_1);
    let mut i64_1: i64 = 83i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_0: i32 = 331i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 60u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_0, u32_0);
    let mut i64_2: i64 = 1000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 11i8;
    let mut i8_2: i8 = 0i8;
    let mut i64_3: i64 = 1000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut f64_1: f64 = 4607182418800017408.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_0, duration_2);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 2i8;
    let mut i8_6: i8 = 0i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = -120i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_4, i8_5);
    let mut i8_9: i8 = 59i8;
    let mut i8_10: i8 = 5i8;
    let mut i8_11: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_10, i8_7, i8_3);
    let mut i32_1: i32 = 376i32;
    let mut i64_4: i64 = 1i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u8_6: u8 = 0u8;
    let mut u8_7: u8 = 52u8;
    let mut i8_12: i8 = 2i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_1, i8_0, i8_9);
    let mut i8_13: i8 = 43i8;
    let mut i8_14: i8 = 23i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_8, i8_11);
    let mut i32_2: i32 = 392i32;
    let mut i64_5: i64 = 60i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_2: u32 = 68u32;
    let mut u8_8: u8 = 59u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_6: i64 = 1000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
//    panic!("From RustyUnit with love");
}
}