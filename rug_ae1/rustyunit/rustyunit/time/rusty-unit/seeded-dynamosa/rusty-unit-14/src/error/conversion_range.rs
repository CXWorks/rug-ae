//! Conversion range error

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// An error type indicating that a conversion failed because the target type could not store the
/// initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRange;

impl fmt::Display for ConversionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionRange {}

impl From<ConversionRange> for crate::Error {
    fn from(err: ConversionRange) -> Self {
        Self::ConversionRange(err)
    }
}

impl TryFrom<crate::Error> for ConversionRange {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ConversionRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5916() {
//    rusty_monitor::set_test_id(5916);
    let mut i32_0: i32 = -227i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i8_0: i8 = 41i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i8_3: i8 = 127i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 604800i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i8_6: i8 = 6i8;
    let mut i8_7: i8 = 23i8;
    let mut i8_8: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_2: i64 = 1000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_1: i32 = 207i32;
    let mut i64_3: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i8_9: i8 = 6i8;
    let mut i8_10: i8 = 59i8;
    let mut i8_11: i8 = 0i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_12: i8 = 23i8;
    let mut i8_13: i8 = 2i8;
    let mut i8_14: i8 = 127i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i32_2: i32 = 50i32;
    let mut i64_4: i64 = 9223372036854775807i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut i32_3: i32 = 257i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_3);
    let mut i8_15: i8 = 6i8;
    let mut i8_16: i8 = 6i8;
    let mut i8_17: i8 = 24i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 2i8;
    let mut i8_19: i8 = 23i8;
    let mut i8_20: i8 = 1i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 6u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_21: i8 = 60i8;
    let mut i8_22: i8 = 6i8;
    let mut i8_23: i8 = 6i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 83i8;
    let mut i8_25: i8 = 39i8;
    let mut i8_26: i8 = 127i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_2: u32 = 1000000000u32;
    let mut u8_6: u8 = 0u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_5: i64 = 1i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = 9223372036854775807i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_9);
    let mut i64_7: i64 = 604800i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i8_27: i8 = 60i8;
    let mut i8_28: i8 = 23i8;
    let mut i8_29: i8 = 59i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i32_4: i32 = 139i32;
    let mut i64_8: i64 = 1000000000i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_4);
    let mut i8_30: i8 = 1i8;
    let mut i8_31: i8 = 23i8;
    let mut i8_32: i8 = -49i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_33: i8 = 24i8;
    let mut i8_34: i8 = 6i8;
    let mut i8_35: i8 = 4i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut u32_3: u32 = 32u32;
    let mut u8_9: u8 = 52u8;
    let mut u8_10: u8 = 23u8;
    let mut u8_11: u8 = 11u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut u16_0: u16 = 37u16;
    let mut i32_5: i32 = 365i32;
    let mut i64_9: i64 = 2147483647i64;
    let mut i32_6: i32 = 392i32;
    let mut i64_10: i64 = 86400i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_15);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_11: i64 = 0i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_11);
    let mut i8_36: i8 = 60i8;
    let mut i8_37: i8 = 127i8;
    let mut i8_38: i8 = 59i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i64_12: i64 = 24i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::hours(i64_12);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut u32_4: u32 = 1000000u32;
    let mut u8_12: u8 = 3u8;
    let mut u8_13: u8 = 10u8;
    let mut u8_14: u8 = 23u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
//    panic!("From RustyUnit with love");
}
}