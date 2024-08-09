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
	use std::convert::TryFrom;
	use std::convert::From;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2662() {
    rusty_monitor::set_test_id(2662);
    let mut i128_0: i128 = -54i128;
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i128_1: i128 = -18i128;
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -24i64;
    let mut i64_1: i64 = -135i64;
    let mut i64_2: i64 = 142i64;
    let mut str_0: &str = "52zRX3clU4n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = -123i64;
    let mut i64_4: i64 = -129i64;
    let mut i64_5: i64 = -32i64;
    let mut str_1: &str = "4t";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_0: month::Month = crate::month::Month::September;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7121() {
    rusty_monitor::set_test_id(7121);
    let mut i8_0: i8 = -27i8;
    let mut i8_1: i8 = 30i8;
    let mut i8_2: i8 = 9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut i8_3: i8 = 19i8;
    let mut i8_4: i8 = -23i8;
    let mut i8_5: i8 = 120i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 56i8;
    let mut i8_7: i8 = 37i8;
    let mut i8_8: i8 = -76i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 89i8;
    let mut i8_10: i8 = 91i8;
    let mut i8_11: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_0: i64 = 136i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 25u8;
    let mut u8_2: u8 = 33u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -43i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_0: i32 = 105i32;
    let mut i64_2: i64 = -73i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i128_0: i128 = -54i128;
    let mut i64_3: i64 = -108i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_1: i32 = -72i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i128_1: i128 = -18i128;
    let mut bool_0: bool = true;
    let mut i64_4: i64 = -24i64;
    let mut i64_5: i64 = -135i64;
    let mut i64_6: i64 = 142i64;
    let mut str_0: &str = "52zRX3clU4n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_7: i64 = -123i64;
    let mut i64_8: i64 = -129i64;
    let mut i64_9: i64 = -32i64;
    let mut str_1: &str = "4t";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_9, maximum: i64_8, value: i64_7, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u8_3: u8 = crate::time::Time::second(time_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_0);
    let mut u8_4: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8019() {
    rusty_monitor::set_test_id(8019);
    let mut i32_0: i32 = 81i32;
    let mut i128_0: i128 = -54i128;
    let mut i64_0: i64 = -108i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_1: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i128_1: i128 = -18i128;
    let mut bool_0: bool = true;
    let mut i64_1: i64 = -24i64;
    let mut i64_2: i64 = -135i64;
    let mut i64_3: i64 = 142i64;
    let mut str_0: &str = "52zRX3clU4n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_4: i64 = -123i64;
    let mut i64_5: i64 = -129i64;
    let mut i64_6: i64 = -32i64;
    let mut str_1: &str = "4t";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut bool_3: bool = crate::duration::Duration::is_negative(duration_2);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7047() {
    rusty_monitor::set_test_id(7047);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 174i64;
    let mut i64_1: i64 = -140i64;
    let mut i64_2: i64 = -63i64;
    let mut str_0: &str = "n8NBt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    let mut i128_0: i128 = -119i128;
    let mut i128_1: i128 = -54i128;
    let mut i64_3: i64 = -108i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut bool_1: bool = true;
    let mut i64_4: i64 = -24i64;
    let mut i64_5: i64 = -135i64;
    let mut i64_6: i64 = 142i64;
    let mut str_1: &str = "52zRX3clU4n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = false;
    let mut i64_7: i64 = -123i64;
    let mut i64_8: i64 = -129i64;
    let mut i64_9: i64 = -32i64;
    let mut str_2: &str = "4t";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_9, maximum: i64_8, value: i64_7, conditional_range: bool_2};
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut month_0: month::Month = crate::month::Month::September;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7879() {
    rusty_monitor::set_test_id(7879);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u8_0: u8 = 74u8;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_0: i32 = 60i32;
    let mut i128_0: i128 = -54i128;
    let mut i64_0: i64 = -108i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_1: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i128_1: i128 = -18i128;
    let mut bool_0: bool = true;
    let mut i64_1: i64 = -24i64;
    let mut i64_2: i64 = -135i64;
    let mut i64_3: i64 = 142i64;
    let mut str_0: &str = "52zRX3clU4n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_4: i64 = -123i64;
    let mut i64_5: i64 = -129i64;
    let mut i64_6: i64 = -32i64;
    let mut str_1: &str = "4t";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u8_1: u8 = crate::time::Time::second(time_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5728() {
    rusty_monitor::set_test_id(5728);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 94i64;
    let mut i64_1: i64 = -54i64;
    let mut i64_2: i64 = -168i64;
    let mut str_0: &str = "Gy7Gm05mye";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    let mut i128_0: i128 = -54i128;
    let mut i64_3: i64 = -108i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i128_1: i128 = -18i128;
    let mut bool_1: bool = true;
    let mut i64_4: i64 = -24i64;
    let mut i64_5: i64 = -135i64;
    let mut i64_6: i64 = 142i64;
    let mut str_1: &str = "52zRX3clU4n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = false;
    let mut i64_7: i64 = -123i64;
    let mut i64_8: i64 = -129i64;
    let mut i64_9: i64 = -32i64;
    let mut str_2: &str = "4t";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_9, maximum: i64_8, value: i64_7, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = std::cmp::PartialEq::eq(componentrange_2_ref_0, componentrange_1_ref_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut result_0: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7635() {
    rusty_monitor::set_test_id(7635);
    let mut i64_0: i64 = -20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = 152i128;
    let mut i128_1: i128 = -54i128;
    let mut i64_1: i64 = -108i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i128_2: i128 = -18i128;
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -24i64;
    let mut i64_3: i64 = -135i64;
    let mut i64_4: i64 = 142i64;
    let mut str_0: &str = "52zRX3clU4n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_5: i64 = -123i64;
    let mut i64_6: i64 = -129i64;
    let mut i64_7: i64 = -32i64;
    let mut str_1: &str = "4t";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_0);
    panic!("From RustyUnit with love");
}
}