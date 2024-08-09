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
fn rusty_test_4875() {
    rusty_monitor::set_test_id(4875);
    let mut i8_0: i8 = -127i8;
    let mut i8_1: i8 = 70i8;
    let mut i8_2: i8 = -89i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -83i8;
    let mut i8_4: i8 = 18i8;
    let mut i8_5: i8 = -32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = -114i8;
    let mut i8_8: i8 = -42i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 16u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 71u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -68i32;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = -4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_9: i8 = -21i8;
    let mut i8_10: i8 = -30i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut f32_0: f32 = 115.551029f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_12: i8 = -96i8;
    let mut i8_13: i8 = -122i8;
    let mut i8_14: i8 = 21i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_2: i64 = -149i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i128_0: i128 = -141i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = -86i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = 95i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = -34.946590f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut i8_15: i8 = -48i8;
    let mut i8_16: i8 = -1i8;
    let mut i8_17: i8 = -82i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_18: i8 = -72i8;
    let mut i8_19: i8 = 62i8;
    let mut i8_20: i8 = -60i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 3u32;
    let mut u8_6: u8 = 83u8;
    let mut u8_7: u8 = 37u8;
    let mut u8_8: u8 = 22u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_5: i64 = 68i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut f32_1: f32 = -42.168155f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_13, duration_12);
    let mut i8_21: i8 = -100i8;
    let mut i8_22: i8 = 38i8;
    let mut i8_23: i8 = -19i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f32_2: f32 = 3.837842f32;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_15);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i64_6: i64 = -163i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::abs(duration_18);
    let mut i64_7: i64 = 36i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_20);
    let mut i64_8: i64 = -196i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut i8_24: i8 = 20i8;
    let mut i8_25: i8 = 22i8;
    let mut i8_26: i8 = 62i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_1: i32 = -176i32;
    let mut i64_9: i64 = -248i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_1);
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::abs(duration_23);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    panic!("From RustyUnit with love");
}
}