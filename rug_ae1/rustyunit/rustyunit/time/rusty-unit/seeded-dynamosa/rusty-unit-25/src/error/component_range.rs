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
	use std::convert::From;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_436() {
//    rusty_monitor::set_test_id(436);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 2440588i64;
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 1000i64;
    let mut str_0: &str = "overflow subtracting duration from date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 9223372036854775807i64;
    let mut i64_4: i64 = 1000i64;
    let mut i64_5: i64 = 9i64;
    let mut str_1: &str = "second";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_1: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut bool_2: bool = false;
    let mut i64_6: i64 = 0i64;
    let mut i64_7: i64 = -102i64;
    let mut i64_8: i64 = 1i64;
    let mut str_2: &str = "second";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut error_2: error::Error = crate::error::Error::ComponentRange(componentrange_2);
    let mut bool_3: bool = true;
    let mut i64_9: i64 = 1000i64;
    let mut i64_10: i64 = 12i64;
    let mut i64_11: i64 = 8i64;
    let mut str_3: &str = "Instant";
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
#[timeout(30000)]fn rusty_test_2870() {
//    rusty_monitor::set_test_id(2870);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 88i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 29u8;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 296i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 12i64;
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 123i64;
    let mut str_0: &str = "conditional_range";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i32_2: i32 = 43i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut componentrange_1: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_762() {
//    rusty_monitor::set_test_id(762);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 1000000000i64;
    let mut i64_1: i64 = 12i64;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut str_0: &str = "Wednesday";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 2147483647i64;
    let mut i64_4: i64 = 1000000000i64;
    let mut i64_5: i64 = 12i64;
    let mut str_1: &str = "UtcOffset";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 9u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_6: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i64_7: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_268() {
//    rusty_monitor::set_test_id(268);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut i64_1: i64 = 86400i64;
    let mut i64_2: i64 = 1000000000i64;
    let mut str_0: &str = "ordinal";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1179() {
//    rusty_monitor::set_test_id(1179);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = -44i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = -7i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 21i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 84u32;
    let mut u8_3: u8 = 4u8;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = -112i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_1: i32 = 111i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_1);
    let mut i8_6: i8 = 47i8;
    let mut i8_7: i8 = 6i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 60i8;
    let mut i8_10: i8 = 1i8;
    let mut i8_11: i8 = 32i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_2: u32 = 1000u32;
    let mut u8_6: u8 = 8u8;
    let mut u8_7: u8 = 77u8;
    let mut u8_8: u8 = 23u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u16_1: u16 = 10u16;
    let mut i32_2: i32 = 280i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_3};
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_2);
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_3);
    let mut f32_0: f32 = -58.232583f32;
    let mut i8_12: i8 = -72i8;
    let mut i8_13: i8 = 127i8;
    let mut i8_14: i8 = -93i8;
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut bool_0: bool = false;
    let mut i64_4: i64 = 9223372036854775807i64;
    let mut i64_5: i64 = 3600i64;
    let mut i64_6: i64 = 29i64;
    let mut str_0: &str = "July";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut componentrange_1: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut u16_2: u16 = crate::time::Time::millisecond(time_3);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_4);
    let mut u8_9: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_211() {
//    rusty_monitor::set_test_id(211);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 3600i64;
    let mut i64_1: i64 = 78i64;
    let mut i64_2: i64 = 1000000i64;
    let mut str_0: &str = "Duration";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 60i64;
    let mut i64_4: i64 = 604800i64;
    let mut i64_5: i64 = 1i64;
    let mut str_1: &str = "ambrBD7";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -102i64;
    let mut i64_7: i64 = 0i64;
    let mut i64_8: i64 = 2440588i64;
    let mut str_2: &str = "offset";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = true;
    let mut i64_9: i64 = 2147483647i64;
    let mut i64_10: i64 = 12i64;
    let mut i64_11: i64 = 1000i64;
    let mut str_3: &str = "June";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_3_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_2_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1193() {
//    rusty_monitor::set_test_id(1193);
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = 48i32;
    let mut i64_1: i64 = 1000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i64_2: i64 = 604800i64;
    let mut i32_1: i32 = 93i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut bool_0: bool = true;
    let mut i64_3: i64 = 86400i64;
    let mut i64_4: i64 = 9223372036854775807i64;
    let mut i64_5: i64 = 0i64;
    let mut str_0: &str = "ErKgUtQOMUKvB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_6: i64 = 2147483647i64;
    let mut i64_7: i64 = 604800i64;
    let mut i64_8: i64 = 60i64;
    let mut str_1: &str = "day";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1159() {
//    rusty_monitor::set_test_id(1159);
    let mut i8_0: i8 = 47i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 280i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_0: f32 = -58.232583f32;
    let mut i8_6: i8 = -66i8;
    let mut i8_7: i8 = 127i8;
    let mut i8_8: i8 = -93i8;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut i64_2: i64 = 3600i64;
    let mut i64_3: i64 = 29i64;
    let mut str_0: &str = "July";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut componentrange_1: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_2);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
//    panic!("From RustyUnit with love");
}
}