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
#[timeout(30000)]fn rusty_test_7547() {
//    rusty_monitor::set_test_id(7547);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 35u8;
    let mut u8_2: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 76.736472f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut f64_1: f64 = 4741671816366391296.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_0: i64 = 3600i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = -19i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u8_6: u8 = crate::date::Date::day(date_0);
//    panic!("From RustyUnit with love");
}
}