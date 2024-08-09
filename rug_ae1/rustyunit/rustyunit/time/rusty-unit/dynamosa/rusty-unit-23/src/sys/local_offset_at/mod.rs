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
fn rusty_test_585() {
    rusty_monitor::set_test_id(585);
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i64_1: i64 = 65i64;
    let mut u32_0: u32 = 96u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 3u8;
    let mut i16_0: i16 = 4i16;
    let mut i64_2: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut u32_1: u32 = 99u32;
    let mut u8_3: u8 = 68u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 98u8;
    let mut i32_0: i32 = 90i32;
    let mut i32_1: i32 = 69i32;
    let mut i64_3: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut i8_0: i8 = -50i8;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_4: i64 = 8i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i8_1: i8 = -7i8;
    let mut i8_2: i8 = -55i8;
    let mut i8_3: i8 = 19i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u32_2: u32 = 1u32;
    let mut u8_6: u8 = 50u8;
    let mut u8_7: u8 = 53u8;
    let mut u8_8: u8 = 70u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_0: u16 = 28u16;
    let mut i32_2: i32 = -31i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_3: i32 = -27i32;
    let mut u8_9: u8 = 85u8;
    let mut i64_5: i64 = -42i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut f32_0: f32 = 43.333512f32;
    let mut u32_3: u32 = 60u32;
    let mut u8_10: u8 = 38u8;
    let mut i64_6: i64 = 127i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_1);
    let mut month_1: month::Month = crate::month::Month::June;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut month_2: month::Month = crate::month::Month::November;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}
}