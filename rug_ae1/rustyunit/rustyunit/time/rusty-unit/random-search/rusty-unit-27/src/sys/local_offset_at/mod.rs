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
fn rusty_test_3450() {
    rusty_monitor::set_test_id(3450);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_0: u8 = 29u8;
    let mut i32_0: i32 = 0i32;
    let mut i64_0: i64 = -38i64;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = -42i64;
    let mut i64_2: i64 = -29i64;
    let mut i64_3: i64 = 100i64;
    let mut str_0: &str = "r0xDHJFF3lh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    panic!("From RustyUnit with love");
}
}