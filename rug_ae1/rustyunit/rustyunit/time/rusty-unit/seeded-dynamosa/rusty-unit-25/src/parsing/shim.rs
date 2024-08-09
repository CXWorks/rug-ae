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
#[timeout(30000)]fn rusty_test_8091() {
//    rusty_monitor::set_test_id(8091);
    let mut i64_0: i64 = 12i64;
    let mut i128_0: i128 = 1i128;
    let mut i64_1: i64 = 1000000000i64;
    let mut i64_2: i64 = 60i64;
    let mut bool_0: bool = true;
    let mut i64_3: i64 = 2147483647i64;
    let mut i64_4: i64 = -30i64;
    let mut i64_5: i64 = 3600i64;
    let mut str_0: &str = "utc_datetime";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut error_0_ref_0: &error::Error = &mut error_0;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i128_1: i128 = 0i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i128_2: i128 = 1000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_6: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = 71i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = -11i8;
    let mut i8_2: i8 = 60i8;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i128_3: i128 = 1000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_7: i64 = -97i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 79i8;
    let mut i8_5: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_4, i8_2, i8_1);
    let mut i8_6: i8 = 2i8;
    let mut i8_7: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_5, i8_7);
    let mut i8_8: i8 = 21i8;
    let mut i8_9: i8 = 5i8;
    let mut i8_10: i8 = 127i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_9, i8_8);
    let mut i64_8: i64 = 1000000000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i8_11: i8 = -24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_11, i8_10);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_3);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut i32_2: i32 = 122i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
//    panic!("From RustyUnit with love");
}
}