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
fn rusty_test_3511() {
    rusty_monitor::set_test_id(3511);
    let mut i32_0: i32 = 74i32;
    let mut i64_0: i64 = 79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = 32i8;
    let mut i8_2: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -323i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_1: i32 = -282i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i8_3: i8 = -59i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = -103i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_2);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u16_0: u16 = 56u16;
    let mut i32_2: i32 = -53i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i64_3: i64 = -63i64;
    let mut bool_0: bool = false;
    let mut i64_4: i64 = 132i64;
    let mut i64_5: i64 = 322i64;
    let mut i64_6: i64 = 256i64;
    let mut str_0: &str = "awneXEOfb";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::different_variant::DifferentVariant, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_2);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_1);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    panic!("From RustyUnit with love");
}
}