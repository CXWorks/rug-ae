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
#[timeout(30000)]fn rusty_test_652() {
//    rusty_monitor::set_test_id(652);
    let mut u8_0: u8 = 30u8;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_1: month::Month = crate::month::Month::July;
    let mut u8_1: u8 = 60u8;
    let mut month_2: month::Month = crate::month::Month::October;
    let mut month_3: month::Month = crate::month::Month::next(month_2);
    let mut u8_2: u8 = 24u8;
    let mut month_4: month::Month = crate::month::Month::December;
    let mut u8_3: u8 = 28u8;
    let mut month_5: month::Month = crate::month::Month::March;
    let mut month_6: month::Month = crate::month::Month::next(month_5);
    let mut month_7: month::Month = crate::month::Month::January;
    let mut u8_4: u8 = 28u8;
    let mut month_8: month::Month = crate::month::Month::November;
    let mut u8_5: u8 = 10u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_4, u8_3, u8_1);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_0, u8_5, u8_2);
//    panic!("From RustyUnit with love");
}
}