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
fn rusty_test_2636() {
    rusty_monitor::set_test_id(2636);
    let mut f64_0: f64 = -72.562110f64;
    let mut i32_0: i32 = -73i32;
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = -20i8;
    let mut i8_1: i8 = 55i8;
    let mut i8_2: i8 = 100i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i16_0: i16 = -32i16;
    let mut f64_1: f64 = -96.259625f64;
    let mut f32_0: f32 = -151.289052f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 87u32;
    let mut f32_1: f32 = -89.658370f32;
    let mut i128_0: i128 = 19i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
    panic!("From RustyUnit with love");
}
}