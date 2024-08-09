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
#[timeout(30000)]fn rusty_test_7612() {
//    rusty_monitor::set_test_id(7612);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut f64_0: f64 = 48.520235f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut f64_1: f64 = 4607182418800017408.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_0: i32 = 370i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_1: i32 = 1000000i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_sunday(weekday_5);
    let mut u8_1: u8 = crate::weekday::Weekday::number_from_sunday(weekday_4);
    let mut u8_2: u8 = crate::weekday::Weekday::number_from_sunday(weekday_3);
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_sunday(weekday_1);
    let mut u8_4: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
//    panic!("From RustyUnit with love");
}
}