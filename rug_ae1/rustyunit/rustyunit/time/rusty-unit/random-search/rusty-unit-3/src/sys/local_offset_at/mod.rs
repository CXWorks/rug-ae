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
fn rusty_test_4807() {
    rusty_monitor::set_test_id(4807);
    let mut i8_0: i8 = -75i8;
    let mut i32_0: i32 = 95i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i8_1: i8 = -29i8;
    let mut i8_2: i8 = 69i8;
    let mut i8_3: i8 = -60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u32_0: u32 = 57u32;
    let mut u8_0: u8 = 65u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 11u16;
    let mut i32_1: i32 = 74i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut i8_4: i8 = 64i8;
    let mut i8_5: i8 = -84i8;
    let mut i8_6: i8 = 33i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut u32_1: u32 = 88u32;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 70u8;
    let mut u8_5: u8 = 66u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 5u16;
    let mut i32_2: i32 = -97i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut i64_0: i64 = -28i64;
    let mut i8_7: i8 = -99i8;
    let mut i8_8: i8 = -38i8;
    let mut i8_9: i8 = 71i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut u32_2: u32 = 58u32;
    let mut u8_6: u8 = 49u8;
    let mut u8_7: u8 = 19u8;
    let mut u8_8: u8 = 24u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_3: i32 = -44i32;
    let mut i64_1: i64 = -25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i32_4: i32 = 175i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_2};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_2};
    let mut i64_2: i64 = -48i64;
    let mut f32_0: f32 = -130.934747f32;
    let mut i32_5: i32 = 32i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 63u32;
    let mut u8_9: u8 = 61u8;
    let mut u8_10: u8 = 85u8;
    let mut u8_11: u8 = 84u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    panic!("From RustyUnit with love");
}
}