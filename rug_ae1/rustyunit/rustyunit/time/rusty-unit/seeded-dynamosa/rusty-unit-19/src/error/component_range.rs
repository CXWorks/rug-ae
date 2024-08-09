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
#[timeout(30000)]fn rusty_test_204() {
//    rusty_monitor::set_test_id(204);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 1000000i64;
    let mut i64_1: i64 = 604800i64;
    let mut i64_2: i64 = 2147483647i64;
    let mut str_0: &str = "ZK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 604800i64;
    let mut i64_4: i64 = 86400i64;
    let mut i64_5: i64 = 1i64;
    let mut str_1: &str = "A7q15ZEE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = false;
    let mut i64_6: i64 = 60i64;
    let mut i64_7: i64 = 1000i64;
    let mut i64_8: i64 = 1i64;
    let mut str_2: &str = "hours";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 0i64;
    let mut i64_10: i64 = -195i64;
    let mut i64_11: i64 = 2440588i64;
    let mut str_3: &str = "j";
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
#[timeout(30000)]fn rusty_test_246() {
//    rusty_monitor::set_test_id(246);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 24i64;
    let mut i64_1: i64 = 168i64;
    let mut i64_2: i64 = 1i64;
    let mut str_0: &str = "seconds";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 60i64;
    let mut i64_4: i64 = 1000000000i64;
    let mut i64_5: i64 = 9223372036854775807i64;
    let mut str_1: &str = "gwnORUR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 2147483647i64;
    let mut i64_7: i64 = -130i64;
    let mut i64_8: i64 = 1000000000i64;
    let mut str_2: &str = "OffsetDateTime";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 2440588i64;
    let mut i64_10: i64 = 9223372036854775807i64;
    let mut i64_11: i64 = 3600i64;
    let mut str_3: &str = "utc_datetime";
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
#[timeout(30000)]fn rusty_test_90() {
//    rusty_monitor::set_test_id(90);
    let mut u32_0: u32 = 74u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut f32_0: f32 = 30.416925f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 3600i64;
    let mut i64_2: i64 = 60i64;
    let mut i64_3: i64 = -11i64;
    let mut str_0: &str = "OffsetDateTime";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_4: i64 = 63i64;
    let mut i64_5: i64 = 604800i64;
    let mut i64_6: i64 = 91i64;
    let mut str_1: &str = "December";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_391() {
//    rusty_monitor::set_test_id(391);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 24i64;
    let mut i64_1: i64 = 604800i64;
    let mut i64_2: i64 = 253402300799i64;
    let mut str_0: &str = "overflow converting `std::time::Duration` to `time::Duration`";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 60i64;
    let mut i64_4: i64 = 68i64;
    let mut i64_5: i64 = 604800i64;
    let mut str_1: &str = "PrimitiveDateTime";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 60i64;
    let mut i64_7: i64 = 2440588i64;
    let mut i64_8: i64 = 9223372036854775807i64;
    let mut str_2: &str = "year";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 1000i64;
    let mut i64_10: i64 = 3600i64;
    let mut i64_11: i64 = 52i64;
    let mut str_3: &str = "9vQN";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut componentrange_4: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_3_ref_0);
    let mut componentrange_5: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_2_ref_0);
    let mut componentrange_6: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_1_ref_0);
    let mut componentrange_7: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_300() {
//    rusty_monitor::set_test_id(300);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 1000i64;
    let mut i64_1: i64 = 1000000i64;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut str_0: &str = "2YMxmb";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    let mut error_0_ref_0: &error::Error = &mut error_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_368() {
//    rusty_monitor::set_test_id(368);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 253402300799i64;
    let mut i64_1: i64 = 12i64;
    let mut i64_2: i64 = 2147483647i64;
    let mut str_0: &str = "resulting value is out of range";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 2147483647i64;
    let mut i64_4: i64 = -15i64;
    let mut i64_5: i64 = 253402300799i64;
    let mut str_1: &str = "date";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_1: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut bool_2: bool = false;
    let mut i64_6: i64 = 1000000000i64;
    let mut i64_7: i64 = 86400i64;
    let mut i64_8: i64 = 12i64;
    let mut str_2: &str = "August";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut error_2: error::Error = crate::error::Error::ComponentRange(componentrange_2);
    let mut bool_3: bool = true;
    let mut i64_9: i64 = 126i64;
    let mut i64_10: i64 = -38i64;
    let mut i64_11: i64 = 1000i64;
    let mut str_3: &str = "minutes";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut error_3: error::Error = crate::error::Error::ComponentRange(componentrange_3);
    let mut result_0: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_3);
    let mut result_1: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_2);
    let mut result_2: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_1);
    let mut result_3: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_93() {
//    rusty_monitor::set_test_id(93);
    let mut i64_0: i64 = 133i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 157i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_0: i32 = 331i32;
    let mut i64_2: i64 = 68i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut i64_3: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut bool_0: bool = false;
    let mut i64_4: i64 = 60i64;
    let mut i64_5: i64 = 24i64;
    let mut i64_6: i64 = 1000000000i64;
    let mut str_0: &str = "utc_datetime";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_7: i64 = 1000i64;
    let mut i64_8: i64 = 2440588i64;
    let mut i64_9: i64 = 2440588i64;
    let mut str_1: &str = "year";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_9, maximum: i64_8, value: i64_7, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i128_1: i128 = crate::duration::Duration::whole_nanoseconds(duration_0);
//    panic!("From RustyUnit with love");
}
}