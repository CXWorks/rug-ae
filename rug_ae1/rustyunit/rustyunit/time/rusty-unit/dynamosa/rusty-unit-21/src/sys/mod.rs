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
fn rusty_test_8268() {
    rusty_monitor::set_test_id(8268);
    let mut i64_0: i64 = -58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 135i32;
    let mut i64_1: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 20i32;
    let mut i64_2: i64 = 15i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = -156i32;
    let mut i64_3: i64 = 20i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 130i32;
    let mut i64_4: i64 = 128i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    panic!("From RustyUnit with love");
}
}