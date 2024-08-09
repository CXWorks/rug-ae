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
	use std::cmp::Eq;
	use std::convert::From;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6171() {
    rusty_monitor::set_test_id(6171);
    let mut u32_0: u32 = 33u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 82u8;
    let mut u8_2: u8 = 30u8;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 23u8;
    let mut u16_0: u16 = 62u16;
    let mut i32_0: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = 206i64;
    let mut i64_3: i64 = 199i64;
    let mut i64_4: i64 = 0i64;
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_1: bool = false;
    let mut i64_5: i64 = 2i64;
    let mut i64_6: i64 = -47i64;
    let mut i64_7: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_5, u8_4, u8_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6242() {
    rusty_monitor::set_test_id(6242);
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 23u8;
    let mut u16_0: u16 = 62u16;
    let mut i32_0: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = 2i64;
    let mut i64_3: i64 = -47i64;
    let mut i64_4: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8430() {
    rusty_monitor::set_test_id(8430);
    let mut i32_0: i32 = 91i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 22u16;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 94u8;
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -159i64;
    let mut i64_1: i64 = -56i64;
    let mut i64_2: i64 = 75i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 88i64;
    let mut i64_4: i64 = 78i64;
    let mut i64_5: i64 = -60i64;
    let mut str_1: &str = "kEj2n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -132i64;
    let mut i64_7: i64 = 21i64;
    let mut i64_8: i64 = 97i64;
    let mut str_2: &str = "fR3B6HeRUZw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = std::cmp::PartialEq::ne(componentrange_2_ref_0, componentrange_1_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut u8_3: u8 = crate::date::Date::day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4248() {
    rusty_monitor::set_test_id(4248);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -9i64;
    let mut i64_1: i64 = -22i64;
    let mut i64_2: i64 = -11i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = -49i64;
    let mut i64_4: i64 = 53i64;
    let mut i64_5: i64 = 39i64;
    let mut str_1: &str = "KjVrx5YD0I73";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 23u8;
    let mut u16_0: u16 = 62u16;
    let mut i32_0: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_6: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i64_7: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut bool_2: bool = false;
    let mut i64_8: i64 = 206i64;
    let mut i64_9: i64 = 199i64;
    let mut i64_10: i64 = 0i64;
    let mut str_2: &str = "ckEhmq0dArl0GW";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_10, maximum: i64_9, value: i64_8, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_3: bool = false;
    let mut i64_11: i64 = 2i64;
    let mut i64_12: i64 = -47i64;
    let mut i64_13: i64 = -83i64;
    let mut str_3: &str = "jbvxl";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_13, maximum: i64_12, value: i64_11, conditional_range: bool_3};
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_3_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut month_0: month::Month = crate::month::Month::October;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6774() {
    rusty_monitor::set_test_id(6774);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u16_0: u16 = 22u16;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 94u8;
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -159i64;
    let mut i64_1: i64 = -56i64;
    let mut i64_2: i64 = 75i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 88i64;
    let mut i64_4: i64 = 78i64;
    let mut i64_5: i64 = -60i64;
    let mut str_1: &str = "kEj2n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -132i64;
    let mut i64_7: i64 = 21i64;
    let mut i64_8: i64 = 97i64;
    let mut str_2: &str = "fR3B6HeRUZw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = std::cmp::PartialEq::ne(componentrange_2_ref_0, componentrange_0_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3210() {
    rusty_monitor::set_test_id(3210);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -22i64;
    let mut i64_1: i64 = -95i64;
    let mut i64_2: i64 = 85i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut u16_0: u16 = 22u16;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 94u8;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = -159i64;
    let mut i64_4: i64 = -56i64;
    let mut i64_5: i64 = 75i64;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = 88i64;
    let mut i64_7: i64 = 78i64;
    let mut i64_8: i64 = -60i64;
    let mut str_2: &str = "kEj2n";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_3: bool = true;
    let mut i64_9: i64 = -132i64;
    let mut i64_10: i64 = 21i64;
    let mut i64_11: i64 = 97i64;
    let mut str_3: &str = "fR3B6HeRUZw";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_4: bool = std::cmp::PartialEq::ne(componentrange_2_ref_0, componentrange_0_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut componentrange_3_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_3;
    let mut componentrange_4: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_197() {
    rusty_monitor::set_test_id(197);
    let mut i64_0: i64 = 32i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 3u16;
    let mut i32_0: i32 = 257i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i32_1: i32 = 7i32;
    let mut i32_2: i32 = 212i32;
    let mut i64_1: i64 = -97i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut i64_2: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut bool_0: bool = false;
    let mut i64_3: i64 = 136i64;
    let mut i64_4: i64 = 27i64;
    let mut i64_5: i64 = -64i64;
    let mut str_0: &str = "o0zqalEnC1D";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_add(time_1, duration_2);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6197() {
    rusty_monitor::set_test_id(6197);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 23u8;
    let mut u16_0: u16 = 62u16;
    let mut i32_0: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = 206i64;
    let mut i64_3: i64 = 199i64;
    let mut i64_4: i64 = 0i64;
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_1: bool = false;
    let mut i64_5: i64 = 2i64;
    let mut i64_6: i64 = -47i64;
    let mut i64_7: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut tuple_1: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4829() {
    rusty_monitor::set_test_id(4829);
    let mut i8_0: i8 = -59i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 94i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i32_0: i32 = -126i32;
    let mut i64_1: i64 = 88i64;
    let mut i8_3: i8 = 76i8;
    let mut i8_4: i8 = -109i8;
    let mut i8_5: i8 = -60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 89i8;
    let mut i8_7: i8 = -69i8;
    let mut i8_8: i8 = -113i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 95u32;
    let mut u8_3: u8 = 32u8;
    let mut u8_4: u8 = 16u8;
    let mut u8_5: u8 = 2u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -48i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_1: i32 = -44i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut u8_6: u8 = 87u8;
    let mut u8_7: u8 = 6u8;
    let mut u8_8: u8 = 4u8;
    let mut u16_0: u16 = 62u16;
    let mut i32_2: i32 = -139i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i64_3: i64 = 58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -38i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_3);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i8_9: i8 = 73i8;
    let mut i8_10: i8 = 66i8;
    let mut i8_11: i8 = -41i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut bool_0: bool = false;
    let mut i64_5: i64 = 206i64;
    let mut i64_6: i64 = 199i64;
    let mut i64_7: i64 = 0i64;
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_1: bool = false;
    let mut i64_8: i64 = 2i64;
    let mut i64_9: i64 = -47i64;
    let mut i64_10: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_10, maximum: i64_9, value: i64_8, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_3, u8_8, u8_7, u8_6);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5100() {
    rusty_monitor::set_test_id(5100);
    let mut i128_0: i128 = 132i128;
    let mut u16_0: u16 = 22u16;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 94u8;
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -159i64;
    let mut i64_1: i64 = -56i64;
    let mut i64_2: i64 = 75i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 88i64;
    let mut i64_4: i64 = 78i64;
    let mut i64_5: i64 = -60i64;
    let mut str_1: &str = "kEj2n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -132i64;
    let mut i64_7: i64 = 21i64;
    let mut i64_8: i64 = 97i64;
    let mut str_2: &str = "fR3B6HeRUZw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = std::cmp::PartialEq::ne(componentrange_2_ref_0, componentrange_1_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7823() {
    rusty_monitor::set_test_id(7823);
    let mut i32_0: i32 = -83i32;
    let mut u16_0: u16 = 62u16;
    let mut i32_1: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_0: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = 2i64;
    let mut i64_3: i64 = -47i64;
    let mut i64_4: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut u8_0: u8 = crate::util::weeks_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4194() {
    rusty_monitor::set_test_id(4194);
    let mut i8_0: i8 = -43i8;
    let mut i8_1: i8 = -47i8;
    let mut i8_2: i8 = -120i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -139i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u16_0: u16 = 87u16;
    let mut i32_0: i32 = 64i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_1: i64 = -3i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_1: i32 = -6i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 4u8;
    let mut u16_1: u16 = 62u16;
    let mut i32_2: i32 = -139i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i64_2: i64 = 58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = -38i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 66i8;
    let mut i8_5: i8 = -41i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut bool_0: bool = false;
    let mut i64_4: i64 = 206i64;
    let mut i64_5: i64 = 199i64;
    let mut i64_6: i64 = 0i64;
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut bool_1: bool = false;
    let mut i64_7: i64 = 2i64;
    let mut i64_8: i64 = -47i64;
    let mut i64_9: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_9, maximum: i64_8, value: i64_7, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_4, u8_2, u8_1, u8_0);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4279() {
    rusty_monitor::set_test_id(4279);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -159i64;
    let mut i64_1: i64 = -56i64;
    let mut i64_2: i64 = 75i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 88i64;
    let mut i64_4: i64 = 78i64;
    let mut i64_5: i64 = -60i64;
    let mut str_1: &str = "kEj2n";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = false;
    let mut i64_6: i64 = -132i64;
    let mut i64_7: i64 = 21i64;
    let mut i64_8: i64 = 97i64;
    let mut str_2: &str = "fR3B6HeRUZw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_3: bool = std::cmp::PartialEq::ne(componentrange_2_ref_0, componentrange_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2484() {
    rusty_monitor::set_test_id(2484);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 25u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i64_0: i64 = 27i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = -45i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = 135i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut u8_3: u8 = 87u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 23u8;
    let mut u16_1: u16 = 62u16;
    let mut i32_1: i32 = -139i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i64_2: i64 = 58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = -40i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut bool_0: bool = false;
    let mut i64_4: i64 = 206i64;
    let mut i64_5: i64 = 199i64;
    let mut i64_6: i64 = 0i64;
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_1: bool = false;
    let mut i64_7: i64 = 2i64;
    let mut i64_8: i64 = -47i64;
    let mut i64_9: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_9, maximum: i64_8, value: i64_7, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_5, u8_4, u8_3);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut u8_6: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut u8_7: u8 = crate::time::Time::second(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5254() {
    rusty_monitor::set_test_id(5254);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut i32_0: i32 = 6i32;
    let mut i64_0: i64 = -52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 4u8;
    let mut u16_0: u16 = 62u16;
    let mut i32_1: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 58i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = -38i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -115.877560f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut bool_0: bool = false;
    let mut i64_3: i64 = 206i64;
    let mut i64_4: i64 = 199i64;
    let mut i64_5: i64 = 0i64;
    let mut str_0: &str = "ckEhmq0dArl0GW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut bool_1: bool = false;
    let mut i64_6: i64 = 2i64;
    let mut i64_7: i64 = -47i64;
    let mut i64_8: i64 = -83i64;
    let mut str_1: &str = "jbvxl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_1_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut i32_2: i32 = crate::duration::Duration::subsec_microseconds(duration_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}
}