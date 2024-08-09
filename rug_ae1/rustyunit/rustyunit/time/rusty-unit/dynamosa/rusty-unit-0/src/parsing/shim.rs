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
fn rusty_test_6724() {
    rusty_monitor::set_test_id(6724);
    let mut i128_0: i128 = 83i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -31i8;
    let mut i8_1: i8 = -90i8;
    let mut i8_2: i8 = 73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -26i8;
    let mut i8_4: i8 = -6i8;
    let mut i8_5: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 85i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = 159i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = -120i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 179i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i8_6: i8 = 68i8;
    let mut i8_7: i8 = -4i8;
    let mut i8_8: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 89.067537f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i64_4: i64 = 3i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut u32_0: u32 = 22u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = -73i8;
    let mut i8_10: i8 = -4i8;
    let mut i8_11: i8 = -113i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_0: i32 = -13i32;
    let mut i64_5: i64 = 158i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut u32_1: u32 = 45u32;
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 13u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = -11i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_15);
    let mut i8_12: i8 = 83i8;
    let mut i8_13: i8 = 10i8;
    let mut i8_14: i8 = 56i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_7: i64 = 104i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i64_8: i64 = -50i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_17);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_19);
    let mut i64_9: i64 = 66i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut i32_1: i32 = -66i32;
    let mut i64_10: i64 = -50i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds(i64_10);
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_23, i32_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_25: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut i64_11: i64 = 2i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_11);
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_26, duration_25);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_12: i64 = 179i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_12);
    let mut duration_29: std::time::Duration = crate::duration::Duration::abs_std(duration_28);
    let mut i8_15: i8 = -54i8;
    let mut i8_16: i8 = 45i8;
    let mut i8_17: i8 = -114i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    panic!("From RustyUnit with love");
}
}