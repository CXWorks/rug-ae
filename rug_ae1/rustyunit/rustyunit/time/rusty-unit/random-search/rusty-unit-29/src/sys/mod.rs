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
fn rusty_test_4665() {
    rusty_monitor::set_test_id(4665);
    let mut i32_0: i32 = -51i32;
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 68u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -245i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i8_0: i8 = 42i8;
    let mut i8_1: i8 = -11i8;
    let mut i8_2: i8 = 39i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_2: i32 = 58i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u32_1: u32 = 42u32;
    let mut u8_3: u8 = 85u8;
    let mut u8_4: u8 = 46u8;
    let mut u8_5: u8 = 77u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = 102i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut u32_2: u32 = 17u32;
    let mut u8_6: u8 = 63u8;
    let mut u8_7: u8 = 4u8;
    let mut u8_8: u8 = 69u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u32_3: u32 = 25u32;
    let mut u8_9: u8 = 28u8;
    let mut u8_10: u8 = 47u8;
    let mut u8_11: u8 = 92u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i32_4: i32 = 99i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_2);
    let mut i8_3: i8 = -86i8;
    let mut i8_4: i8 = 19i8;
    let mut i8_5: i8 = -68i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_4: u32 = 80u32;
    let mut u8_12: u8 = 93u8;
    let mut u8_13: u8 = 68u8;
    let mut u8_14: u8 = 72u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i32_5: i32 = 193i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_5, offset: utcoffset_1};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_4);
    let mut i8_6: i8 = 94i8;
    let mut i8_7: i8 = -30i8;
    let mut i8_8: i8 = -44i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_5: u32 = 34u32;
    let mut u8_15: u8 = 49u8;
    let mut u8_16: u8 = 65u8;
    let mut u8_17: u8 = 89u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_5};
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_6, utcoffset_2);
    let mut u32_6: u32 = 88u32;
    let mut u8_18: u8 = 44u8;
    let mut u8_19: u8 = 69u8;
    let mut u8_20: u8 = 83u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_7: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut u16_0: u16 = 13u16;
    let mut i32_6: i32 = 40i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_6, time: time_7};
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_8, time_6);
    let mut i8_9: i8 = -31i8;
    let mut i8_10: i8 = 81i8;
    let mut i8_11: i8 = 23i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -106i8;
    let mut i8_13: i8 = -18i8;
    let mut i8_14: i8 = -123i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_6, utcoffset_4);
    let mut time_8: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut u16_1: u16 = 95u16;
    let mut i32_7: i32 = 47i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut primitivedatetime_10: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_7, time_8);
    let mut primitivedatetime_11: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_10, utcoffset_3);
    let mut i8_15: i8 = -34i8;
    let mut i8_16: i8 = 17i8;
    let mut i8_17: i8 = -84i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_8, utcoffset_5);
    let mut time_9: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut u32_7: u32 = 0u32;
    let mut u8_21: u8 = 57u8;
    let mut u8_22: u8 = 63u8;
    let mut u8_23: u8 = 70u8;
    let mut time_10: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_10, time_10);
    let mut date_8: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_11);
    let mut primitivedatetime_12: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_9);
    let mut u32_8: u32 = 12u32;
    let mut u8_24: u8 = 36u8;
    let mut u8_25: u8 = 82u8;
    let mut u8_26: u8 = 9u8;
    let mut time_11: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_26, u8_25, u8_24, u32_8);
    let mut i32_8: i32 = -172i32;
    let mut date_9: crate::date::Date = crate::date::Date {value: i32_8};
    let mut primitivedatetime_13: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_9, time_11);
    panic!("From RustyUnit with love");
}
}