//! Combinators for rules as defined in an RFC.
//!
//! These rules have been converted strictly following the ABNF syntax as specified in [RFC 2234].
//!
//! [RFC 2234]: https://datatracker.ietf.org/doc/html/rfc2234

pub(crate) mod rfc2234;
pub(crate) mod rfc2822;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2030() {
    rusty_monitor::set_test_id(2030);
    let mut i8_0: i8 = 41i8;
    let mut u16_0: u16 = 87u16;
    let mut i64_0: i64 = 34i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 115i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u16_1: u16 = 5u16;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 62u8;
    let mut i16_0: i16 = 81i16;
    let mut i32_0: i32 = 262i32;
    let mut i64_2: i64 = -58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_1);
    panic!("From RustyUnit with love");
}
}