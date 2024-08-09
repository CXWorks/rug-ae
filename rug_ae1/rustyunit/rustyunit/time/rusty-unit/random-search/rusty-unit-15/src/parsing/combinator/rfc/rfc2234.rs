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
fn rusty_test_2506() {
    rusty_monitor::set_test_id(2506);
    let mut u16_0: u16 = 22u16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 76i32;
    let mut i64_0: i64 = -52i64;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 2u8;
    let mut i16_0: i16 = 108i16;
    let mut i32_1: i32 = -85i32;
    let mut i64_1: i64 = 164i64;
    let mut i32_2: i32 = 290i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut f32_0: f32 = -59.500582f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = 73.564563f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_1: u32 = 72u32;
    let mut u8_3: u8 = 42u8;
    let mut u8_4: u8 = 87u8;
    let mut u8_5: u8 = 36u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i16_1: i16 = -73i16;
    let mut i128_0: i128 = -31i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = 64i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    panic!("From RustyUnit with love");
}
}