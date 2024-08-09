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
#[timeout(30000)]fn rusty_test_8243() {
//    rusty_monitor::set_test_id(8243);
    let mut u16_0: u16 = 97u16;
    let mut i32_0: i32 = 3i32;
    let mut i64_0: i64 = 24i64;
    let mut i32_1: i32 = 37i32;
    let mut i64_1: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = -19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_2: i32 = 9i32;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
//    panic!("From RustyUnit with love");
}
}