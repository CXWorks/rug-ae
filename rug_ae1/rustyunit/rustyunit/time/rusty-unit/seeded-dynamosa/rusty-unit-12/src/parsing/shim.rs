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
#[timeout(30000)]fn rusty_test_1622() {
//    rusty_monitor::set_test_id(1622);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 392i32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 8u8;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u16_1: u16 = 60u16;
    let mut i32_1: i32 = 331i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_2: i32 = 212i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_2: i64 = -38i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_0: u32 = 10u32;
    let mut u8_2: u8 = 11u8;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_4, u8_3, u8_2, u32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut i128_1: i128 = 35i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_3: i64 = 17i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_3: i32 = 71i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u8_5: u8 = crate::date::Date::day(date_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_0, u8_1, u8_5);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1635() {
//    rusty_monitor::set_test_id(1635);
    let mut i32_0: i32 = 5i32;
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 55u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 150i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut u16_0: u16 = 33u16;
    let mut i32_2: i32 = 252i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = 195i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut str_0: &str = "month";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
//    panic!("From RustyUnit with love");
}
}