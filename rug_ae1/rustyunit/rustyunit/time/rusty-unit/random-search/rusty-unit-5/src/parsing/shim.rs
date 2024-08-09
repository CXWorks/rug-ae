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
fn rusty_test_3883() {
    rusty_monitor::set_test_id(3883);
    let mut i8_0: i8 = -2i8;
    let mut i8_1: i8 = -18i8;
    let mut i8_2: i8 = 119i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_0: i64 = 87i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut f64_0: f64 = 65.648860f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_3: i8 = -85i8;
    let mut i8_4: i8 = -17i8;
    let mut i8_5: i8 = 34i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 56u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 43u8;
    let mut u8_2: u8 = 36u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_6: i8 = -6i8;
    let mut i8_7: i8 = -98i8;
    let mut i8_8: i8 = 18i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -55i8;
    let mut i8_10: i8 = 112i8;
    let mut i8_11: i8 = 18i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_1: i64 = -89i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u32_1: u32 = 54u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 76u8;
    let mut u8_5: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 91i32;
    let mut i64_2: i64 = 16i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i64_3: i64 = -38i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i8_12: i8 = -56i8;
    let mut i8_13: i8 = -6i8;
    let mut i8_14: i8 = -45i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -100i8;
    let mut i8_16: i8 = -29i8;
    let mut i8_17: i8 = 14i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_4: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut u32_2: u32 = 52u32;
    let mut u8_6: u8 = 30u8;
    let mut u8_7: u8 = 32u8;
    let mut u8_8: u8 = 50u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_18: i8 = -8i8;
    let mut i8_19: i8 = -88i8;
    let mut i8_20: i8 = -103i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_5: i64 = -40i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i8_21: i8 = 26i8;
    let mut i8_22: i8 = -56i8;
    let mut i8_23: i8 = 6i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_24: i8 = -113i8;
    let mut i8_25: i8 = -118i8;
    let mut i8_26: i8 = -3i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = -79i8;
    let mut i8_28: i8 = 123i8;
    let mut i8_29: i8 = 22i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i64_6: i64 = 139i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut u32_3: u32 = 22u32;
    let mut u8_9: u8 = 75u8;
    let mut u8_10: u8 = 12u8;
    let mut u8_11: u8 = 29u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_7: i64 = -215i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i8_30: i8 = -9i8;
    let mut i8_31: i8 = 83i8;
    let mut i8_32: i8 = 93i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 8i8;
    let mut i8_34: i8 = -86i8;
    let mut i8_35: i8 = 94i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = 17i8;
    let mut i8_37: i8 = 97i8;
    let mut i8_38: i8 = 121i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i32_1: i32 = -5i32;
    let mut i64_8: i64 = 110i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_1);
    let mut i64_9: i64 = 43i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut i128_0: i128 = -28i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut u32_4: u32 = 88u32;
    let mut u8_12: u8 = 10u8;
    let mut u8_13: u8 = 80u8;
    let mut u8_14: u8 = 23u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_39: i8 = 107i8;
    let mut i8_40: i8 = -16i8;
    let mut i8_41: i8 = -52i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u8_15: u8 = 10u8;
    let mut i64_10: i64 = 231i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_10);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = -13i32;
    let mut i64_11: i64 = -25i64;
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    panic!("From RustyUnit with love");
}
}