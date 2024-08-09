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
#[timeout(30000)]fn rusty_test_6505() {
//    rusty_monitor::set_test_id(6505);
    let mut i64_0: i64 = 1000i64;
    let mut i32_0: i32 = 93i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 70u16;
    let mut i32_1: i32 = 133i32;
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u16_1: u16 = 999u16;
    let mut i32_2: i32 = 252i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_3: i32 = 207i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut offsetdatetime_1_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_3: i64 = 86400i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_2: u16 = 366u16;
    let mut i32_4: i32 = 82i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_2_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_3;
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_4;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
//    panic!("From RustyUnit with love");
}
}