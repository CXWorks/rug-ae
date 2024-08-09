//! Functions with a common interface that rely on system calls.

#![allow(unsafe_code)] // We're interfacing with system calls.

#[cfg(feature = "local-offset")]
mod local_offset_at;

#[cfg(feature = "local-offset")]
pub(crate) use local_offset_at::local_offset_at;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6393() {
//    rusty_monitor::set_test_id(6393);
    let mut i64_0: i64 = 24i64;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 42u8;
    let mut i64_1: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u16_0: u16 = 64u16;
    let mut i32_0: i32 = 370i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i32_1: i32 = 37i32;
    let mut i64_2: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_2: i32 = 32i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i64_3: i64 = 9223372036854775807i64;
    let mut str_0: &str = "second";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_3: i32 = 308i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut date_3: crate::date::Date = std::result::Result::unwrap(result_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_2, duration_1);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_0);
//    panic!("From RustyUnit with love");
}
}