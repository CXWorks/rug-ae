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
fn rusty_test_8459() {
    rusty_monitor::set_test_id(8459);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i8_0: i8 = -32i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = 2i8;
    let mut u16_0: u16 = 2u16;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 3u8;
    let mut u16_1: u16 = 1u16;
    let mut i32_0: i32 = -19i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 50u32;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut str_0: &str = "FXh4zlkx";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut month_1: month::Month = crate::month::Month::July;
    let mut u8_6: u8 = crate::time::Time::minute(time_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    panic!("From RustyUnit with love");
}
}