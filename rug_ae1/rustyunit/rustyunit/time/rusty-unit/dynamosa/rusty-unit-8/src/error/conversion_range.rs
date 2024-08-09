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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6918() {
    rusty_monitor::set_test_id(6918);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -16i32;
    let mut i128_0: i128 = 22i128;
    let mut u8_0: u8 = 88u8;
    let mut i32_1: i32 = -55i32;
    let mut i64_0: i64 = -104i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut u32_0: u32 = 48u32;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = 72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_2: i32 = -123i32;
    let mut i64_3: i64 = -152i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 76i8;
    let mut i8_2: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -38.884742f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_4: i64 = 75i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_3: i32 = 11i32;
    let mut i64_5: i64 = -15i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_3);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_4: i32 = -41i32;
    let mut i64_6: i64 = 130i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_4);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i8_3: i8 = -76i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = -38i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i32_5: i32 = -16i32;
    let mut i32_6: i32 = -5i32;
    let mut i64_7: i64 = 74i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_6);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_5);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 97u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 36u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = -89i8;
    let mut i8_7: i8 = 97i8;
    let mut i8_8: i8 = -94i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_1: i128 = 64i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_9: i8 = -24i8;
    let mut i8_10: i8 = -109i8;
    let mut i8_11: i8 = -51i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_8: i64 = 1i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut f32_1: f32 = 86.079296f32;
    let mut i64_9: i64 = -186i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut i32_7: i32 = -105i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_14);
    let mut i64_10: i64 = 88i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_16);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_11: i64 = -62i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::microseconds(i64_10);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_3);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i64_12: i64 = 151i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds(i64_11);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_15);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut i32_8: i32 = 36i32;
    let mut i64_13: i64 = 119i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_12, i32_8);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds(i64_13);
    let mut i64_14: i64 = -109i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_4);
    let mut i64_15: i64 = 41i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut i32_9: i32 = -126i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_7};
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_15);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_9};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::next_day(date_2);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_5);
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_21_ref_0: &crate::duration::Duration = &mut duration_21;
    panic!("From RustyUnit with love");
}
}