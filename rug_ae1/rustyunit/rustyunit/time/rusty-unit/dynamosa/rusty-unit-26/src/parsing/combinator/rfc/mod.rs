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
fn rusty_test_7252() {
    rusty_monitor::set_test_id(7252);
    let mut u8_0: u8 = 27u8;
    let mut i64_0: i64 = 111i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 44i64;
    let mut u32_0: u32 = 0u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 27u8;
    let mut u8_3: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_5: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -11i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_6);
    panic!("From RustyUnit with love");
}
}