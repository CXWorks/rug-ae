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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_86() {
    rusty_monitor::set_test_id(86);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -81i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_0_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 64u8;
    let mut i32_1: i32 = 23i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i8_0: i8 = 33i8;
    let mut i8_1: i8 = 71i8;
    let mut i8_2: i8 = 19i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut i8_3: i8 = -58i8;
    let mut i8_4: i8 = -13i8;
    let mut i8_5: i8 = 12i8;
    let mut i64_0: i64 = 13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_2: i32 = 152i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_3;
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_2);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}
}