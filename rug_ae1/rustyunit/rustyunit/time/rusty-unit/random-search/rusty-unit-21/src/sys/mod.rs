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
fn rusty_test_2350() {
    rusty_monitor::set_test_id(2350);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = -190i32;
    let mut f64_0: f64 = 21.470993f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut i32_1: i32 = -90i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u16_0: u16 = 13u16;
    let mut i32_2: i32 = -197i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    panic!("From RustyUnit with love");
}
}