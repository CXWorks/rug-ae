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
fn rusty_test_4274() {
    rusty_monitor::set_test_id(4274);
    let mut u32_0: u32 = 59u32;
    let mut i64_0: i64 = 37i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 96u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 67u8;
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    panic!("From RustyUnit with love");
}
}