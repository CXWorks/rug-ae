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
	use std::convert::TryFrom;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4026() {
    rusty_monitor::set_test_id(4026);
    let mut u16_0: u16 = 90u16;
    let mut i32_0: i32 = -31i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = -3i32;
    let mut i64_0: i64 = 75i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i32_2: i32 = 73i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 1i64;
    let mut i64_2: i64 = 19i64;
    let mut i64_3: i64 = -58i64;
    let mut str_0: &str = "y6QxoEJHdez";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut u32_0: u32 = 17u32;
    let mut u8_0: u8 = 37u8;
    let mut u8_1: u8 = 85u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = -194i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut f32_0: f32 = -67.534366f32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 21i8;
    let mut i8_4: i8 = 51i8;
    let mut i8_5: i8 = 101i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = -58i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i64_5: i64 = -40i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i8_6: i8 = -100i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = -104i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 22u32;
    let mut u8_3: u8 = 7u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 86u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = 6i8;
    let mut i8_10: i8 = -34i8;
    let mut i8_11: i8 = -8i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_6: i64 = -6i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut i8_12: i8 = -50i8;
    let mut i8_13: i8 = -120i8;
    let mut i8_14: i8 = 12i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -48i8;
    let mut i8_16: i8 = 28i8;
    let mut i8_17: i8 = 37i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i128_0: i128 = 2i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u8_6: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u8_7: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_1, duration_4);
    let mut result_0: std::result::Result<crate::error::conversion_range::ConversionRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut i32_4: i32 = crate::date::Date::to_julian_day(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    panic!("From RustyUnit with love");
}
}