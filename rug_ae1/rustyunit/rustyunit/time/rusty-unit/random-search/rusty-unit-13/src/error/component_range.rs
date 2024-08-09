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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::TryFrom;
	use std::cmp::Eq;
	use std::convert::From;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2442() {
    rusty_monitor::set_test_id(2442);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 172i64;
    let mut i64_1: i64 = 28i64;
    let mut i64_2: i64 = -49i64;
    let mut str_0: &str = "c2zci";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = -7i64;
    let mut i64_4: i64 = -58i64;
    let mut i64_5: i64 = 31i64;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -23i64;
    let mut i64_7: i64 = 106i64;
    let mut i64_8: i64 = -31i64;
    let mut str_2: &str = "0rt";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut componentrange_3: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_2_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2889() {
    rusty_monitor::set_test_id(2889);
    let mut i8_0: i8 = -7i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = -93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -94i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 16i32;
    let mut i32_2: i32 = 57i32;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = -35i8;
    let mut i8_5: i8 = 45i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -11i64;
    let mut i64_1: i64 = -127i64;
    let mut i64_2: i64 = 2i64;
    let mut str_0: &str = "Pg4CG8q5mkise";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut tuple_1: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    let mut i8_7: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3229() {
    rusty_monitor::set_test_id(3229);
    let mut f32_0: f32 = -183.399701f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -180i64;
    let mut i64_1: i64 = 78i64;
    let mut i64_2: i64 = -33i64;
    let mut str_0: &str = "tvSko";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 61i64;
    let mut i64_4: i64 = -101i64;
    let mut i64_5: i64 = -116i64;
    let mut str_1: &str = "pMpHZVuHvN2RE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3016() {
    rusty_monitor::set_test_id(3016);
    let mut i8_0: i8 = 89i8;
    let mut i8_1: i8 = 78i8;
    let mut i8_2: i8 = 55i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 61u32;
    let mut u8_0: u8 = 47u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 117i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 64i64;
    let mut i64_1: i64 = 94i64;
    let mut i64_2: i64 = 6i64;
    let mut str_0: &str = "Wu8eKqA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_0: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3079() {
    rusty_monitor::set_test_id(3079);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 4i64;
    let mut i64_1: i64 = 268i64;
    let mut i64_2: i64 = -54i64;
    let mut str_0: &str = "76r141E";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 114i64;
    let mut i64_4: i64 = 64i64;
    let mut i64_5: i64 = -67i64;
    let mut str_1: &str = "woa7A";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_1: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3862() {
    rusty_monitor::set_test_id(3862);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -29i64;
    let mut i64_1: i64 = -126i64;
    let mut i64_2: i64 = 149i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = -24i64;
    let mut i64_4: i64 = -12i64;
    let mut i64_5: i64 = 0i64;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_0: error::Error = std::convert::From::from(componentrange_1);
    let mut componentrange_2: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1199() {
    rusty_monitor::set_test_id(1199);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -90i64;
    let mut i64_1: i64 = -36i64;
    let mut i64_2: i64 = -147i64;
    let mut str_0: &str = "auNXTD20";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut u32_0: u32 = 64u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 36u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_1);
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_120() {
    rusty_monitor::set_test_id(120);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -7i64;
    let mut i64_1: i64 = -31i64;
    let mut i64_2: i64 = 15i64;
    let mut str_0: &str = "ioCY63B3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i64_3: i64 = -110i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i64_4: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i32_0: i32 = -73i32;
    let mut i64_5: i64 = -2i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_0);
    let mut i32_1: i32 = -214i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_0_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}
}