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
fn rusty_test_4881() {
    rusty_monitor::set_test_id(4881);
    let mut i8_0: i8 = 82i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 95i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 8i64;
    let mut i64_1: i64 = -59i64;
    let mut i64_2: i64 = -23i64;
    let mut str_0: &str = "mYnFzJRn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i32_0: i32 = 40i32;
    let mut i64_3: i64 = 25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut i64_4: i64 = -33i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = crate::duration::Duration::whole_weeks(duration_0);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    panic!("From RustyUnit with love");
}
}