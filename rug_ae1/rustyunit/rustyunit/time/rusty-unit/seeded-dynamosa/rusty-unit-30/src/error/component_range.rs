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
	use std::convert::TryFrom;
	use std::cmp::Eq;
	use std::convert::From;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_105() {
//    rusty_monitor::set_test_id(105);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 9999i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = -33i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 50u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut u16_0: u16 = 59u16;
    let mut i32_1: i32 = 376i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 0i64;
    let mut i64_2: i64 = 0i64;
    let mut i64_3: i64 = 12i64;
    let mut str_0: &str = "December";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_4: i64 = 1000i64;
    let mut i64_5: i64 = 222i64;
    let mut i64_6: i64 = -41i64;
    let mut str_1: &str = "X";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_691() {
//    rusty_monitor::set_test_id(691);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 61i64;
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 12i64;
    let mut str_0: &str = "ChXYkanJR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i32_0: i32 = -59i32;
    let mut i64_3: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 59i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_492() {
//    rusty_monitor::set_test_id(492);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 1000i64;
    let mut i64_1: i64 = 3600i64;
    let mut i64_2: i64 = -60i64;
    let mut str_0: &str = "week";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 0i64;
    let mut i64_4: i64 = 1i64;
    let mut i64_5: i64 = 3600i64;
    let mut str_1: &str = "overflow converting `std::time::Duration` to `time::Duration`";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = false;
    let mut i64_6: i64 = 1i64;
    let mut i64_7: i64 = 9223372036854775807i64;
    let mut i64_8: i64 = 604800i64;
    let mut str_2: &str = "March";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 66i64;
    let mut i64_10: i64 = 2147483647i64;
    let mut i64_11: i64 = 604800i64;
    let mut str_3: &str = "nanosecond";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut bool_4: bool = std::cmp::PartialEq::eq(componentrange_3_ref_0, componentrange_2_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_461() {
//    rusty_monitor::set_test_id(461);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 2147483647i64;
    let mut i64_1: i64 = 1000i64;
    let mut i64_2: i64 = -26i64;
    let mut str_0: &str = "January";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 253402300799i64;
    let mut i64_4: i64 = 2440588i64;
    let mut i64_5: i64 = 2440588i64;
    let mut str_1: &str = "Date";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_1: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 12i64;
    let mut i64_7: i64 = 0i64;
    let mut i64_8: i64 = 59i64;
    let mut str_2: &str = "43Ja2f";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut error_2: error::Error = std::convert::From::from(componentrange_2);
    let mut bool_3: bool = true;
    let mut i64_9: i64 = 1i64;
    let mut i64_10: i64 = 85i64;
    let mut i64_11: i64 = 95i64;
    let mut str_3: &str = "overflow subtracting duration from date";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut error_3: error::Error = std::convert::From::from(componentrange_3);
    let mut result_0: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_3);
    let mut result_1: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_2);
    let mut result_2: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_1);
    let mut result_3: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5900() {
//    rusty_monitor::set_test_id(5900);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 365i32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 72u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 387i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_3: u8 = 68u8;
    let mut i32_2: i32 = 353i32;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 1000000000i64;
    let mut i64_2: i64 = 253402300799i64;
    let mut i64_3: i64 = 0i64;
    let mut str_0: &str = "qTaCBNPUDSKjkr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_4: i64 = 0i64;
    let mut i64_5: i64 = 23i64;
    let mut i64_6: i64 = -30i64;
    let mut str_1: &str = "Tuesday";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut i8_3: i8 = 3i8;
    let mut i64_7: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i32_3: i32 = 9991i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = -33i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_5, i8_4);
    let mut u32_1: u32 = 10u32;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 63u8;
    let mut u8_6: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u16_1: u16 = 59u16;
    let mut i32_4: i32 = 376i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_2);
    let mut bool_2: bool = false;
    let mut i64_8: i64 = 0i64;
    let mut i64_9: i64 = 0i64;
    let mut i64_10: i64 = 12i64;
    let mut str_2: &str = "December";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_10, maximum: i64_9, value: i64_8, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = true;
    let mut i64_11: i64 = 1000i64;
    let mut i64_12: i64 = 222i64;
    let mut i64_13: i64 = -41i64;
    let mut str_3: &str = "X";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_13, maximum: i64_12, value: i64_11, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut bool_4: bool = std::cmp::PartialEq::ne(componentrange_3_ref_0, componentrange_2_ref_0);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_3);
    let mut bool_5: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_3, weekday_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_226() {
//    rusty_monitor::set_test_id(226);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 253402300799i64;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut i64_2: i64 = 2147483647i64;
    let mut str_0: &str = "5d";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 1i64;
    let mut i64_4: i64 = -13i64;
    let mut i64_5: i64 = 0i64;
    let mut str_1: &str = "millisecond";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 1000000000i64;
    let mut i64_7: i64 = 1i64;
    let mut i64_8: i64 = 2147483647i64;
    let mut str_2: &str = "hours";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 1000i64;
    let mut i64_10: i64 = 2147483647i64;
    let mut i64_11: i64 = 83i64;
    let mut str_3: &str = "February";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut bool_4: bool = std::cmp::PartialEq::ne(componentrange_3_ref_0, componentrange_2_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_460() {
//    rusty_monitor::set_test_id(460);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 60i64;
    let mut i64_1: i64 = 604800i64;
    let mut i64_2: i64 = 0i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_356() {
//    rusty_monitor::set_test_id(356);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 146i64;
    let mut i64_1: i64 = 2147483647i64;
    let mut i64_2: i64 = -1i64;
    let mut str_0: &str = "March";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 9223372036854775807i64;
    let mut i64_4: i64 = 253402300799i64;
    let mut i64_5: i64 = 47i64;
    let mut str_1: &str = ", given values of other parameters";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 60i64;
    let mut i64_7: i64 = 604800i64;
    let mut i64_8: i64 = 604800i64;
    let mut str_2: &str = "Saturday";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = true;
    let mut i64_9: i64 = -92i64;
    let mut i64_10: i64 = 41i64;
    let mut i64_11: i64 = 12i64;
    let mut str_3: &str = "date";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut componentrange_4: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_3_ref_0);
    let mut componentrange_5: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_2_ref_0);
    let mut componentrange_6: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_1_ref_0);
    let mut componentrange_7: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}
}