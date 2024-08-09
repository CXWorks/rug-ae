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
fn rusty_test_2312() {
    rusty_monitor::set_test_id(2312);
    let mut i32_0: i32 = 116i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -39i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_2: i32 = -172i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 85u8;
    let mut u8_4: u8 = 81u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 13u16;
    let mut i32_3: i32 = -38i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_0, i32_1);
    let mut u32_2: u32 = crate::time::Time::microsecond(time_0);
    panic!("From RustyUnit with love");
}
}