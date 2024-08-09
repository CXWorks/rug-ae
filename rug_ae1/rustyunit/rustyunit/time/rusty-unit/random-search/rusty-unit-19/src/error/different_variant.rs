//! Different variant error

use core::convert::TryFrom;
use core::fmt;

/// An error type indicating that a [`TryFrom`](core::convert::TryFrom) call failed because the
/// original value was of a different variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentVariant;

impl fmt::Display for DifferentVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was of a different variant than required")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DifferentVariant {}

impl From<DifferentVariant> for crate::Error {
    fn from(err: DifferentVariant) -> Self {
        Self::DifferentVariant(err)
    }
}

impl TryFrom<crate::Error> for DifferentVariant {
    type Error = Self;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::DifferentVariant(err) => Ok(err),
            _ => Err(Self),
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
fn rusty_test_964() {
    rusty_monitor::set_test_id(964);
    let mut i128_0: i128 = -14i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_0: u32 = 98u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 45u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u16_0: u16 = 49u16;
    let mut i32_0: i32 = -30i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut bool_0: bool = true;
    let mut i64_1: i64 = 12i64;
    let mut i64_2: i64 = 63i64;
    let mut i64_3: i64 = -135i64;
    let mut str_0: &str = "FoGD58vZ3oLFM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut i32_1: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4876() {
    rusty_monitor::set_test_id(4876);
    let mut i8_0: i8 = 122i8;
    let mut i8_1: i8 = 14i8;
    let mut i8_2: i8 = 19i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 23i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 55u16;
    let mut i32_1: i32 = 196i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_0: i64 = -82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 81i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_2: i32 = 92i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_3: i32 = -78i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_0);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = -14i64;
    let mut i64_3: i64 = -60i64;
    let mut i64_4: i64 = -102i64;
    let mut str_0: &str = "iO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut differentvariant_0: crate::error::different_variant::DifferentVariant = std::result::Result::unwrap(result_0);
    let mut u32_0: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_3);
    let mut u8_0: u8 = crate::time::Time::hour(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4279() {
    rusty_monitor::set_test_id(4279);
    let mut i8_0: i8 = 19i8;
    let mut i8_1: i8 = 7i8;
    let mut i8_2: i8 = 0i8;
    let mut i32_0: i32 = 86i32;
    let mut i32_1: i32 = -89i32;
    let mut i64_0: i64 = 198i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 65u16;
    let mut i32_2: i32 = -142i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i8_3: i8 = -19i8;
    let mut i8_4: i8 = -42i8;
    let mut i8_5: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 16i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i128_0: i128 = 77i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = -131.855336f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut u16_1: u16 = 23u16;
    let mut i32_3: i32 = 69i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut u32_1: u32 = 57u32;
    let mut u8_3: u8 = 54u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 53u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_6: u8 = 53u8;
    let mut i32_4: i32 = 43i32;
    let mut u32_2: u32 = 33u32;
    let mut u8_7: u8 = 40u8;
    let mut u8_8: u8 = 80u8;
    let mut u8_9: u8 = 16u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut u16_2: u16 = 28u16;
    let mut i32_5: i32 = -40i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut u8_10: u8 = crate::primitive_date_time::PrimitiveDateTime::hour(primitivedatetime_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_6, weekday_0);
    let mut date_3: crate::date::Date = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}
}