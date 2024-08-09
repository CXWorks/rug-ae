//! Extension traits for things either not implemented or not yet stable in the MSRV.

/// Equivalent of `foo.parse()` for slices.
pub(crate) trait IntegerParseBytes<T> {
    #[allow(clippy::missing_docs_in_private_items)]
    fn parse_bytes(&self) -> Option<T>;
}

impl<T: Integer> IntegerParseBytes<T> for [u8] {
    fn parse_bytes(&self) -> Option<T> {
        T::parse_bytes(self)
    }
}

/// Marker trait for all integer types, including `NonZero*`
pub(crate) trait Integer: Sized {
    #[allow(clippy::missing_docs_in_private_items)]
    fn parse_bytes(src: &[u8]) -> Option<Self>;
}

/// Parse the given types from bytes.
macro_rules! impl_parse_bytes {
    ($($t:ty)*) => ($(
        impl Integer for $t {
            #[allow(trivial_numeric_casts)]
            fn parse_bytes(src: &[u8]) -> Option<Self> {
                src.iter().try_fold::<Self, _, _>(0, |result, c| {
                    result.checked_mul(10)?.checked_add((c - b'0') as Self)
                })
            }
        }
    )*)
}
impl_parse_bytes! { u8 u16 u32 }

/// Parse the given types from bytes.
macro_rules! impl_parse_bytes_nonzero {
    ($($t:ty)*) => {$(
        impl Integer for $t {
            fn parse_bytes(src: &[u8]) -> Option<Self> {
                Self::new(src.parse_bytes()?)
            }
        }
    )*}
}

impl_parse_bytes_nonzero! {
    core::num::NonZeroU8
    core::num::NonZeroU16
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6942() {
//    rusty_monitor::set_test_id(6942);
    let mut f32_0: f32 = 5.316463f32;
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut i64_0: i64 = 3600i64;
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u16_0: u16 = 367u16;
    let mut i32_0: i32 = 398i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    let mut u8_0: u8 = 60u8;
    let mut month_0: month::Month = crate::month::Month::February;
    let mut i32_1: i32 = 1000000i32;
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i8_3: i8 = 5i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 24i8;
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_2: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_3: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_2);
    let mut i8_6: i8 = 24i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = -47i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_7, i8_3);
    let mut i64_4: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i8_9: i8 = 60i8;
    let mut i8_10: i8 = 1i8;
    let mut i64_5: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 9u8;
    let mut u8_3: u8 = 31u8;
    let mut i64_6: i64 = -101i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_4);
    let mut i8_11: i8 = 1i8;
    let mut i8_12: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_5, i8_4);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 59u8;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_13: i8 = 1i8;
    let mut i8_14: i8 = 43i8;
    let mut i8_15: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_13, i8_11, i8_12);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_6: u8 = 7u8;
    let mut u8_7: u8 = 62u8;
    let mut u8_8: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_4, u8_1, u32_0);
    let mut i8_16: i8 = 4i8;
    let mut i8_17: i8 = 0i8;
    let mut i8_18: i8 = 0i8;
    let mut i8_19: i8 = 24i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_15, i8_19, i8_18);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_7: i64 = 9223372036854775807i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i8_20: i8 = 1i8;
    let mut i8_21: i8 = 1i8;
    let mut i8_22: i8 = 5i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_10, i8_20, i8_16);
    let mut i32_2: i32 = -54i32;
    let mut i64_8: i64 = 1000000000i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_14, i32_2);
    let mut i64_9: i64 = -159i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut i8_23: i8 = -22i8;
    let mut i8_24: i8 = 2i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_22, i8_17, i8_8);
    let mut u32_2: u32 = 100000000u32;
    let mut u8_9: u8 = 30u8;
    let mut u8_10: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_0, u8_3, u8_5, u32_1);
    let mut i8_25: i8 = 60i8;
    let mut i8_26: i8 = 6i8;
    let mut i8_27: i8 = 1i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_24, i8_14, i8_25);
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_6, u8_10, u32_2);
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_18: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_27, i8_21);
    let mut i32_3: i32 = 263i32;
    let mut i64_10: i64 = 12i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_3);
    let mut i8_28: i8 = 1i8;
    let mut i32_4: i32 = 305i32;
    let mut i64_11: i64 = 1000000i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_1);
    let mut i64_12: i64 = 1000000i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::hours(i64_11);
    let mut u8_11: u8 = 98u8;
    let mut u8_12: u8 = 4u8;
    let mut i8_29: i8 = 1i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_28, i8_29, i8_26);
    let mut i64_13: i64 = 1000i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::minutes(i64_12);
    let mut i64_14: i64 = 86400i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::days(i64_13);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_15: i64 = 1000000000i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::weeks(i64_14);
    let mut duration_26: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::seconds(i64_15);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i32_5: i32 = 36525i32;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut i32_6: i32 = -46i32;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_13: u8 = 78u8;
    let mut i32_7: i32 = 25i32;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_14: u8 = 12u8;
    let mut i32_8: i32 = 342i32;
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i32_9: i32 = 314i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_6, u8_2, weekday_2);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_7, u8_12, weekday_3);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_9, u8_13, weekday_0);
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_8, u8_9, weekday_4);
    let mut result_4: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_14, weekday_5);
    let mut result_5: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_5, u8_11, weekday_6);
//    panic!("From RustyUnit with love");
}
}