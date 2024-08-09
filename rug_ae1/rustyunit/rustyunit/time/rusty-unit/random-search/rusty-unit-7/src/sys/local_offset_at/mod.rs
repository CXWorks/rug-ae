//! A method to obtain the local offset from UTC.

#[cfg_attr(target_family = "windows", path = "windows.rs")]
#[cfg_attr(target_family = "unix", path = "unix.rs")]
mod imp;

use crate::{OffsetDateTime, UtcOffset};

/// Attempt to obtain the system's UTC offset. If the offset cannot be determined, `None` is
/// returned.
pub(crate) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    imp::local_offset_at(datetime)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4980() {
    rusty_monitor::set_test_id(4980);
    let mut i64_0: i64 = -53i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 10i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_3: i64 = -16i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -154i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 54u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 44u8;
    let mut f32_0: f32 = -52.552966f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i32_0: i32 = 6i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_5: i64 = 89i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    panic!("From RustyUnit with love");
}
}