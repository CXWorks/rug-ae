//! Functions with a common interface that rely on system calls.

#![allow(unsafe_code)] // We're interfacing with system calls.

#[cfg(feature = "local-offset")]
mod local_offset_at;

#[cfg(feature = "local-offset")]
pub(crate) use local_offset_at::local_offset_at;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1021() {
    rusty_monitor::set_test_id(1021);
    let mut u16_0: u16 = 26u16;
    let mut i64_0: i64 = -115i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = -131i32;
    let mut i64_1: i64 = -99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut u8_0: u8 = 79u8;
    let mut f32_0: f32 = 257.553194f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = -19i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    panic!("From RustyUnit with love");
}
}