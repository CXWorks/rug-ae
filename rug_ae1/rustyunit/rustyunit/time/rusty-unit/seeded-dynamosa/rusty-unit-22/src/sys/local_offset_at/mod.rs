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
#[timeout(30000)]fn rusty_test_8325() {
//    rusty_monitor::set_test_id(8325);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut i64_0: i64 = 1i64;
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 303i32;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut u16_1: u16 = 10u16;
    let mut i32_1: i32 = 4i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i8_0: i8 = 15i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 1i8;
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 59i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 3i8;
    let mut i8_7: i8 = 82i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_6, i8_1);
    let mut i8_8: i8 = 2i8;
    let mut i8_9: i8 = 59i8;
    let mut i8_10: i8 = 4i8;
    let mut i8_11: i8 = 58i8;
    let mut i8_12: i8 = -80i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_9, i8_11);
    let mut i8_13: i8 = 5i8;
    let mut i8_14: i8 = 3i8;
    let mut i8_15: i8 = 23i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_12, i8_0);
    let mut i8_16: i8 = 2i8;
    let mut i8_17: i8 = -2i8;
    let mut i8_18: i8 = 0i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_13, i8_15, i8_10);
    let mut u8_3: u8 = 7u8;
    let mut u8_4: u8 = 3u8;
    let mut i8_19: i8 = 19i8;
    let mut i8_20: i8 = 24i8;
    let mut i8_21: i8 = 4i8;
    let mut i8_22: i8 = 127i8;
    let mut i8_23: i8 = 60i8;
    let mut i8_24: i8 = -24i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_22, i8_7);
    let mut u32_1: u32 = 1000u32;
    let mut u8_5: u8 = 7u8;
    let mut u8_6: u8 = 4u8;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_25: i8 = 127i8;
    let mut i8_26: i8 = 6i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_16, i8_24, i8_25);
    let mut i8_27: i8 = 4i8;
    let mut i8_28: i8 = 59i8;
    let mut i8_29: i8 = 4i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_27, i8_28);
    let mut u8_7: u8 = 12u8;
    let mut u8_8: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_3, u8_6, u32_1);
    let mut i8_30: i8 = 24i8;
    let mut i8_31: i8 = -26i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_19, i8_23);
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_31, i8_18, i8_17);
    let mut u32_2: u32 = 1000000000u32;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_4, u8_8, u32_2);
    let mut i8_32: i8 = 2i8;
    let mut i8_33: i8 = 127i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_30, i8_33, i8_32);
    let mut i8_34: i8 = 4i8;
    let mut i8_35: i8 = 2i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_34, i8_21, i8_35);
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut date_2_ref_0: &crate::date::Date = &mut date_2;
    let mut i64_3: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_2: i32 = 9i32;
    let mut i32_3: i32 = 246i32;
    let mut i64_4: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_3);
    let mut i64_5: i64 = 2440588i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_6: i64 = -75i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i64_7: i64 = 1000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_8);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i64_8: i64 = 12i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_4: i32 = 161i32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_8, i32_4);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_13);
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
//    panic!("From RustyUnit with love");
}
}