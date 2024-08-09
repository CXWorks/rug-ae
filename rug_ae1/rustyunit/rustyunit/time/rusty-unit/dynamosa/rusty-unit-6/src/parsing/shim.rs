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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7171() {
    rusty_monitor::set_test_id(7171);
    let mut i64_0: i64 = 49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u32_0: u32 = 27u32;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 83i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = 28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 65i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = -56i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -22i8;
    let mut i8_7: i8 = -27i8;
    let mut i8_8: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 65i8;
    let mut i8_10: i8 = -124i8;
    let mut i8_11: i8 = 121i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 66u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 73u8;
    let mut u8_5: u8 = 86u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_12: i8 = -111i8;
    let mut i8_13: i8 = 40i8;
    let mut i8_14: i8 = -114i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -103i8;
    let mut i8_16: i8 = 15i8;
    let mut i8_17: i8 = 70i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 111i8;
    let mut i8_19: i8 = 28i8;
    let mut i8_20: i8 = 114i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut f32_0: f32 = -68.319723f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -9.289847f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = 183.654628f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_1: i64 = -234i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_2: u32 = 80u32;
    let mut u8_6: u8 = 77u8;
    let mut u8_7: u8 = 18u8;
    let mut u8_8: u8 = 77u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_0: i32 = 71i32;
    let mut i64_2: i64 = -129i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_1: i32 = -165i32;
    let mut i64_3: i64 = -76i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut u8_9: u8 = 72u8;
    let mut i32_2: i32 = -37i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut i32_3: i32 = -135i32;
    let mut i32_4: i32 = 187i32;
    let mut i64_4: i64 = 50i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_5: i32 = -18i32;
    let mut i64_5: i64 = 138i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_5);
    let mut i8_21: i8 = -5i8;
    let mut i8_22: i8 = 13i8;
    let mut i8_23: i8 = 90i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_7);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u32_3: u32 = 1u32;
    let mut u8_10: u8 = 86u8;
    let mut u8_11: u8 = 68u8;
    let mut u8_12: u8 = 65u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_3};
    let mut u8_13: u8 = 83u8;
    let mut u8_14: u8 = 18u8;
    let mut u8_15: u8 = 21u8;
    let mut u16_0: u16 = 88u16;
    let mut i32_6: i32 = -102i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut i32_7: i32 = -72i32;
    let mut i64_6: i64 = -17i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i32_8: i32 = 48i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut i8_24: i8 = 62i8;
    let mut i8_25: i8 = -43i8;
    let mut i8_26: i8 = -89i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_6, utcoffset_8);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut i64_7: i64 = -41i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i8_27: i8 = 65i8;
    let mut i8_28: i8 = -89i8;
    let mut i8_29: i8 = -7i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_9);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_9: i32 = crate::date::Date::year(date_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_11, i32_7);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_3, u8_15, u8_14, u8_13);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_9);
    panic!("From RustyUnit with love");
}
}