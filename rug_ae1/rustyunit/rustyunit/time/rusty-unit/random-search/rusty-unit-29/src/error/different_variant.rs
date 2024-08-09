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
fn rusty_test_395() {
    rusty_monitor::set_test_id(395);
    let mut i64_0: i64 = 108i64;
    let mut u16_0: u16 = 13u16;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 73u8;
    let mut i64_1: i64 = 20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = 28i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_2, u8_1, u8_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = std::result::Result::unwrap(result_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_417() {
    rusty_monitor::set_test_id(417);
    let mut i64_0: i64 = -18i64;
    let mut month_0: month::Month = crate::month::Month::March;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 16u8;
    let mut bool_0: bool = true;
    let mut i64_1: i64 = 202i64;
    let mut i64_2: i64 = -74i64;
    let mut i64_3: i64 = 36i64;
    let mut str_0: &str = "8s8pEw8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut i8_0: i8 = -85i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -67i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut result_1: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    let mut time_0: crate::time::Time = std::result::Result::unwrap(result_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut month_2: month::Month = crate::month::Month::July;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2283() {
    rusty_monitor::set_test_id(2283);
    let mut u32_0: u32 = 22u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 208.140635f32;
    let mut f64_0: f64 = 63.153587f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut u8_3: u8 = 26u8;
    let mut u8_4: u8 = 21u8;
    let mut u8_5: u8 = 11u8;
    let mut i32_1: i32 = -7i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_6: u8 = 14u8;
    let mut i32_2: i32 = -146i32;
    let mut i32_3: i32 = -24i32;
    let mut i64_0: i64 = 27i64;
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 119i8;
    let mut i8_2: i8 = 35i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_5, u8_4, u8_3);
    panic!("From RustyUnit with love");
}
}