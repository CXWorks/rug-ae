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
fn rusty_test_4563() {
    rusty_monitor::set_test_id(4563);
    let mut i64_0: i64 = -63i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 55u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 67u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -71i8;
    let mut i8_1: i8 = -15i8;
    let mut i8_2: i8 = 45i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 97u32;
    let mut u8_3: u8 = 29u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 61u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 78u16;
    let mut i32_0: i32 = -48i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i32_1: i32 = 94i32;
    let mut i64_1: i64 = 21i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_2: i64 = -73i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut bool_0: bool = true;
    let mut i64_3: i64 = 94i64;
    let mut i64_4: i64 = 4i64;
    let mut i64_5: i64 = 34i64;
    let mut str_0: &str = "RxhPp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4591() {
    rusty_monitor::set_test_id(4591);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = 124i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -59i8;
    let mut i8_2: i8 = 35i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 169i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 92i32;
    let mut i64_1: i64 = -3i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut u16_0: u16 = 30u16;
    let mut i32_1: i32 = -79i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    panic!("From RustyUnit with love");
}
}