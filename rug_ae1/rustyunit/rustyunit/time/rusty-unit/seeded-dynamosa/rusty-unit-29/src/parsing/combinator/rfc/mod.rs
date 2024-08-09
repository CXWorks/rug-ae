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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_716() {
//    rusty_monitor::set_test_id(716);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 5i8;
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 5i8;
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = 24i8;
    let mut i8_9: i8 = 23i8;
    let mut i8_10: i8 = 59i8;
    let mut i8_11: i8 = 14i8;
    let mut i8_12: i8 = 0i8;
    let mut i8_13: i8 = 59i8;
    let mut i8_14: i8 = 1i8;
    let mut i8_15: i8 = 127i8;
    let mut i8_16: i8 = 0i8;
    let mut i8_17: i8 = 5i8;
    let mut i8_18: i8 = 23i8;
    let mut i8_19: i8 = 5i8;
    let mut i8_20: i8 = 24i8;
    let mut i8_21: i8 = 127i8;
    let mut i8_22: i8 = 5i8;
    let mut i8_23: i8 = 0i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_23, i8_22, i8_21);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_20, i8_19, i8_18);
    let mut result_2: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_17, i8_16, i8_15);
    let mut result_3: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_14, i8_13, i8_12);
    let mut result_4: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_11, i8_10, i8_9);
    let mut result_5: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_8, i8_7, i8_6);
    let mut result_6: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut result_7: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
//    panic!("From RustyUnit with love");
}
}