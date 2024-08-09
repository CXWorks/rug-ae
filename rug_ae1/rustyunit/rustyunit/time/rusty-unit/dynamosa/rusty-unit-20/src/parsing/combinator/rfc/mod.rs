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
fn rusty_test_6719() {
    rusty_monitor::set_test_id(6719);
    let mut u16_0: u16 = 22u16;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 94u8;
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -159i64;
    let mut i64_1: i64 = -56i64;
    let mut i64_2: i64 = 75i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut str_1: &str = "kEj2n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = -132i64;
    let mut i64_4: i64 = 21i64;
    let mut i64_5: i64 = 97i64;
    let mut str_2: &str = "fR3B6HeRUZw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}
}