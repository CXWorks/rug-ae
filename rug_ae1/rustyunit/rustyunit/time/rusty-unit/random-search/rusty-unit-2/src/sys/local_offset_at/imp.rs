//! A fallback for any OS not covered.

use crate::{OffsetDateTime, UtcOffset};

pub(super) fn local_offset_at(_datetime: OffsetDateTime) -> Option<UtcOffset> {
    None
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3926() {
    rusty_monitor::set_test_id(3926);
    let mut i8_0: i8 = 92i8;
    let mut i8_1: i8 = -1i8;
    let mut i8_2: i8 = 10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -123i8;
    let mut i8_4: i8 = 32i8;
    let mut i8_5: i8 = 69i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 7u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 33i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut offsetdatetime_1_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i32_1: i32 = 14i32;
    let mut i64_0: i64 = -153i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_2: i32 = -101i32;
    let mut i64_1: i64 = -12i64;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i64_2: i64 = crate::duration::Duration::whole_days(duration_1);
    panic!("From RustyUnit with love");
}
}