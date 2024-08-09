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
fn rusty_test_4712() {
    rusty_monitor::set_test_id(4712);
    let mut f64_0: f64 = 69.756690f64;
    let mut i64_0: i64 = -12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i8_0: i8 = 44i8;
    let mut i8_1: i8 = -121i8;
    let mut i8_2: i8 = -19i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 39i8;
    let mut i8_5: i8 = -20i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = -43.360369f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_0: u32 = 52u32;
    let mut u8_0: u8 = 33u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 54u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 60.137575f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_6: i8 = 31i8;
    let mut i8_7: i8 = -99i8;
    let mut i8_8: i8 = -20i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 127i8;
    let mut i8_10: i8 = -110i8;
    let mut i8_11: i8 = -115i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 13u32;
    let mut u8_3: u8 = 88u8;
    let mut u8_4: u8 = 74u8;
    let mut u8_5: u8 = 39u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u8_6: u8 = 97u8;
    let mut i64_1: i64 = -50i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = -55i32;
    let mut i32_1: i32 = 180i32;
    let mut i64_2: i64 = -22i64;
    let mut i8_12: i8 = 16i8;
    let mut i8_13: i8 = -107i8;
    let mut i8_14: i8 = 38i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -50i8;
    let mut i8_16: i8 = -112i8;
    let mut i8_17: i8 = 106i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_18: i8 = -41i8;
    let mut i8_19: i8 = -1i8;
    let mut i8_20: i8 = 40i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = 24i8;
    let mut i8_22: i8 = -91i8;
    let mut i8_23: i8 = -17i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = -4i8;
    let mut i8_25: i8 = 47i8;
    let mut i8_26: i8 = -38i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_2: i32 = 158i32;
    let mut i64_3: i64 = -94i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut i64_4: i64 = -36i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut u32_2: u32 = 86u32;
    let mut u8_7: u8 = 60u8;
    let mut u8_8: u8 = 42u8;
    let mut u8_9: u8 = 46u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = 144i32;
    let mut i64_5: i64 = 146i64;
    let mut i8_27: i8 = 0i8;
    let mut i8_28: i8 = -77i8;
    let mut i8_29: i8 = 60i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 42u32;
    let mut u8_10: u8 = 94u8;
    let mut u8_11: u8 = 70u8;
    let mut u8_12: u8 = 30u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut i8_30: i8 = -33i8;
    let mut i8_31: i8 = 94i8;
    let mut i8_32: i8 = 6i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = -38i8;
    let mut i8_34: i8 = 47i8;
    let mut i8_35: i8 = 5i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i64_6: i64 = -68i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut u32_4: u32 = 22u32;
    let mut i64_7: i64 = 46i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut i64_8: i64 = -48i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut i32_4: i32 = 165i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_8);
    let mut month_0: month::Month = crate::month::Month::November;
    panic!("From RustyUnit with love");
}
}