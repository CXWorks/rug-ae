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
#[timeout(30000)]fn rusty_test_6622() {
//    rusty_monitor::set_test_id(6622);
    let mut i32_0: i32 = -57i32;
    let mut i128_0: i128 = 1000i128;
    let mut i64_0: i64 = 1i64;
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 59u16;
    let mut i32_1: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i64_1: i64 = 1000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_3: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i64_4: i64 = 1000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = -201i32;
    let mut i64_5: i64 = 1i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_2);
    let mut i32_3: i32 = 161i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 1i8;
    let mut i64_6: i64 = 604800i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i64_7: i64 = -139i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut u16_1: u16 = 59u16;
    let mut i32_4: i32 = 268i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_8);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_4, i8_5, i8_3);
    let mut i64_8: i64 = 1000000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_5: i32 = 122i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_4_ref_0: &crate::date::Date = &mut date_4;
    let mut u16_2: u16 = 367u16;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_2);
    let mut date_5_ref_0: &crate::date::Date = &mut date_5;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
//    panic!("From RustyUnit with love");
}
}