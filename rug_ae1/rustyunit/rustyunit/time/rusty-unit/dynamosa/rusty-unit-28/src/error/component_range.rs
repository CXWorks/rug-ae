//! Component range error

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// An error type indicating that a component provided to a method was out of range, causing a
/// failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need for a type parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub(crate) name: &'static str,
    /// Minimum allowed value, inclusive.
    pub(crate) minimum: i64,
    /// Maximum allowed value, inclusive.
    pub(crate) maximum: i64,
    /// Value that was provided.
    pub(crate) value: i64,
    /// The minimum and/or maximum value is conditional on the value of other
    /// parameters.
    pub(crate) conditional_range: bool,
}

impl ComponentRange {
    /// Obtain the name of the component whose value was out of range.
    pub const fn name(self) -> &'static str {
        self.name
    }
}

impl fmt::Display for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.name, self.minimum, self.maximum
        )?;

        if self.conditional_range {
            f.write_str(", given values of other parameters")?;
        }

        Ok(())
    }
}

impl From<ComponentRange> for crate::Error {
    fn from(original: ComponentRange) -> Self {
        Self::ComponentRange(original)
    }
}

impl TryFrom<crate::Error> for ComponentRange {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ComponentRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

/// **This trait implementation is deprecated and will be removed in a future breaking release.**
#[cfg(feature = "serde")]
impl serde::de::Expected for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a value in the range {}..={}",
            self.minimum, self.maximum
        )
    }
}

#[cfg(feature = "serde")]
impl ComponentRange {
    /// Convert the error to a deserialization error.
    pub(crate) fn into_de_error<E: serde::de::Error>(self) -> E {
        E::invalid_value(serde::de::Unexpected::Signed(self.value), &self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ComponentRange {}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8250() {
    rusty_monitor::set_test_id(8250);
    let mut i64_0: i64 = -99i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = -94i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 97u16;
    let mut i32_1: i32 = 88i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 37i64;
    let mut i64_2: i64 = 86i64;
    let mut i64_3: i64 = 207i64;
    let mut str_0: &str = "sVRlXK9vAPI";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i8_0: i8 = -24i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_2: i32 = -118i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut i64_4: i64 = -1i64;
    let mut bool_1: bool = false;
    let mut i64_5: i64 = -182i64;
    let mut i64_6: i64 = -140i64;
    let mut i64_7: i64 = 75i64;
    let mut str_1: &str = "rIz9Kgm3";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_3: i32 = -62i32;
    let mut i64_8: i64 = 31i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_8, i32_3);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i32_4: i32 = 15i32;
    let mut i64_9: i64 = 105i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_4);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_1};
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_2);
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2530() {
    rusty_monitor::set_test_id(2530);
    let mut i64_0: i64 = -14i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = -204i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = -106i64;
    let mut i64_2: i64 = 68i64;
    let mut i64_3: i64 = -77i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 33u8;
    let mut u8_1: u8 = 34u8;
    let mut u8_2: u8 = 19u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_4: i64 = -41i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut u16_1: u16 = 42u16;
    let mut i32_1: i32 = 92i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_5: i64 = -60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i32_2: i32 = 63i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_2);
    let mut i64_6: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i8_0: i8 = -128i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_7: i64 = 58i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i32_3: i32 = -78i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_3};
    let mut u8_3: u8 = crate::date::Date::iso_week(date_5);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
    let mut componentrange_1: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
    let mut u8_5: u8 = crate::date::Date::monday_based_week(date_1);
    panic!("From RustyUnit with love");
}
}