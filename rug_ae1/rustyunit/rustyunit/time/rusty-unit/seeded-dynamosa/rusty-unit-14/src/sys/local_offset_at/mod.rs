//! A method to obtain the local offset from UTC.

#[cfg_attr(target_family = "windows", path = "windows.rs")]
#[cfg_attr(target_family = "unix", path = "unix.rs")]
mod imp;

use crate::{OffsetDateTime, UtcOffset};

/// Attempt to obtain the system's UTC offset. If the offset cannot be determined, `None` is
/// returned.
pub(crate) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    imp::local_offset_at(datetime)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_540() {
//    rusty_monitor::set_test_id(540);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::February;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::June;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::May;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::September;
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut month_6: month::Month = crate::month::Month::September;
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::July;
    let mut month_8: month::Month = crate::month::Month::previous(month_7);
    let mut month_8_ref_0: &month::Month = &mut month_8;
    let mut month_9: month::Month = crate::month::Month::July;
    let mut month_9_ref_0: &month::Month = &mut month_9;
    let mut month_10: month::Month = crate::month::Month::July;
    let mut month_10_ref_0: &month::Month = &mut month_10;
    let mut month_11: month::Month = crate::month::Month::November;
    let mut month_11_ref_0: &month::Month = &mut month_11;
    let mut month_12: month::Month = crate::month::Month::September;
    let mut month_12_ref_0: &month::Month = &mut month_12;
    let mut month_13: month::Month = crate::month::Month::September;
    let mut month_13_ref_0: &month::Month = &mut month_13;
    let mut month_14: month::Month = crate::month::Month::May;
    let mut month_14_ref_0: &month::Month = &mut month_14;
    let mut month_15: month::Month = crate::month::Month::May;
    let mut month_15_ref_0: &month::Month = &mut month_15;
//    panic!("From RustyUnit with love");
}
}