//! Functions with a common interface that rely on system calls.

#![allow(unsafe_code)] // We're interfacing with system calls.

#[cfg(feature = "local-offset")]
mod local_offset_at;

#[cfg(feature = "local-offset")]
pub(crate) use local_offset_at::local_offset_at;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6103() {
//    rusty_monitor::set_test_id(6103);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = 57u8;
    let mut i32_0: i32 = 76i32;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_1: u8 = 52u8;
    let mut i32_1: i32 = 172i32;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_2: u8 = 5u8;
    let mut i32_3: i32 = 150i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_2, weekday_3);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_1, weekday_2);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_1);
//    panic!("From RustyUnit with love");
}
}