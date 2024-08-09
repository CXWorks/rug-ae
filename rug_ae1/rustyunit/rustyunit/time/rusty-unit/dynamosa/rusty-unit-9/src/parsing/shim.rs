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
fn rusty_test_6604() {
    rusty_monitor::set_test_id(6604);
    let mut f32_0: f32 = 11.848537f32;
    let mut u32_0: u32 = 38u32;
    let mut f32_1: f32 = -79.400052f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u16_0: u16 = 58u16;
    let mut i32_0: i32 = -6i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = -116i8;
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = 50i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -29i8;
    let mut i8_4: i8 = -62i8;
    let mut i8_5: i8 = 17i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_2: f32 = -37.594879f32;
    let mut i64_0: i64 = -122i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut f32_3: f32 = -39.371380f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_3);
    let mut i8_6: i8 = -4i8;
    let mut i8_7: i8 = -19i8;
    let mut i8_8: i8 = 92i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f32_4: f32 = 25.697432f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_4);
    let mut i8_9: i8 = -97i8;
    let mut i8_10: i8 = 79i8;
    let mut i8_11: i8 = -15i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_1: i32 = 89i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 150i32;
    let mut i64_2: i64 = 67i64;
    let mut i64_3: i64 = 79i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i64_4: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut i8_12: i8 = -58i8;
    let mut i32_3: i32 = -121i32;
    let mut i64_5: i64 = 24i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_3);
    let mut u16_1: u16 = 3u16;
    let mut i32_4: i32 = -13i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_8);
    let mut i32_5: i32 = -89i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut i32_6: i32 = -117i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_6};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_4);
    let mut i64_6: i64 = 15i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut f32_5: f32 = 150.380696f32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_5);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_2);
    panic!("From RustyUnit with love");
}
}