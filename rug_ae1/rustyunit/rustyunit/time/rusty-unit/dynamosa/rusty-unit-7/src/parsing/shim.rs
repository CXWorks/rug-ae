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
fn rusty_test_3522() {
    rusty_monitor::set_test_id(3522);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -44i8;
    let mut i8_2: i8 = 55i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -17i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = -8i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 83i8;
    let mut i8_7: i8 = 12i8;
    let mut i8_8: i8 = -50i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 40u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -154i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_1: u32 = 93u32;
    let mut u8_3: u8 = 63u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 74u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 71u16;
    let mut i32_1: i32 = 36i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_0: i64 = -52i64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 255.644765f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_0: f32 = -224.997763f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u16_1: u16 = 78u16;
    let mut i32_2: i32 = 14i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut i32_3: i32 = 171i32;
    let mut i64_1: i64 = -14i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_3);
    let mut u16_2: u16 = 83u16;
    let mut i32_4: i32 = -20i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_5);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i8_9: i8 = -21i8;
    let mut i8_10: i8 = -51i8;
    let mut i8_11: i8 = -38i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u16_3: u16 = 66u16;
    let mut i32_5: i32 = -61i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_3);
    let mut i8_12: i8 = -60i8;
    let mut i8_13: i8 = 14i8;
    let mut i8_14: i8 = -124i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_2: i64 = -123i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 105i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i32_6: i32 = -122i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_7: crate::date::Date = crate::date::Date::saturating_sub(date_6, duration_8);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_7);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u16_4: u16 = 4u16;
    let mut i32_7: i32 = -87i32;
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_2);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_5, utcoffset_4);
    let mut f64_1: f64 = 139.709176f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut f32_1: f32 = 297.287187f32;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut duration_12_ref_0: &std::time::Duration = &mut duration_12;
    let mut i64_4: i64 = -90i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_13_ref_0: &crate::duration::Duration = &mut duration_13;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_14);
    let mut date_9: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut i8_15: i8 = -114i8;
    let mut i8_16: i8 = 35i8;
    let mut i8_17: i8 = 35i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    panic!("From RustyUnit with love");
}
}