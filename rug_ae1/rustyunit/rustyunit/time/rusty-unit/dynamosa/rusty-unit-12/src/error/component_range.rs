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
	use std::cmp::Eq;
	use std::convert::From;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4516() {
    rusty_monitor::set_test_id(4516);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 243i64;
    let mut i64_1: i64 = 84i64;
    let mut i64_2: i64 = -139i64;
    let mut str_0: &str = "z3TuDguynT7T5r";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i64_3: i64 = -70i64;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_0: i32 = -67i32;
    let mut i64_4: i64 = 131i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_0);
    let mut i32_1: i32 = -2i32;
    let mut i64_5: i64 = 80i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = -111i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_2: i32 = 35i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut month_1: month::Month = crate::month::Month::December;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i32_3: i32 = -12i32;
    let mut i32_4: i32 = 80i32;
    let mut i64_6: i64 = -34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut i32_5: i32 = -141i32;
    let mut i64_7: i64 = -60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_4);
    let mut i32_6: i32 = -85i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_0_ref_0: &mut crate::date::Date = &mut date_0;
    let mut i32_7: i32 = 55i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_7);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    let mut month_2: month::Month = crate::month::Month::September;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_115() {
    rusty_monitor::set_test_id(115);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -82i64;
    let mut i64_1: i64 = -43i64;
    let mut i64_2: i64 = -89i64;
    let mut str_0: &str = "DXwiFJ3I46nUL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = 214i64;
    let mut i64_4: i64 = 14i64;
    let mut i64_5: i64 = -5i64;
    let mut str_1: &str = "U9Iv9Y";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -39i64;
    let mut i64_7: i64 = 11i64;
    let mut i64_8: i64 = 216i64;
    let mut str_2: &str = "A1GkBVW6uFkI";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut error_0: error::Error = std::convert::From::from(componentrange_2);
    let mut error_0_ref_0: &error::Error = &mut error_0;
    let mut i32_0: i32 = 8i32;
    let mut i64_9: i64 = 44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_0);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut bool_3: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_320() {
    rusty_monitor::set_test_id(320);
    let mut f64_0: f64 = 1.021307f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 98i8;
    let mut i8_1: i8 = 7i8;
    let mut i8_2: i8 = -25i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -119i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = -92i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = -33i64;
    let mut i64_2: i64 = 55i64;
    let mut i64_3: i64 = 75i64;
    let mut str_0: &str = "2Tc2sRt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_4: i64 = -78i64;
    let mut i64_5: i64 = 8i64;
    let mut i64_6: i64 = -30i64;
    let mut str_1: &str = "4a7EEEZsZ3X";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    panic!("From RustyUnit with love");
}
}