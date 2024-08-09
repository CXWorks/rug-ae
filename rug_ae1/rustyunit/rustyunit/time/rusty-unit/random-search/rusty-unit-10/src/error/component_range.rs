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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2222() {
    rusty_monitor::set_test_id(2222);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -13i64;
    let mut i64_1: i64 = -57i64;
    let mut i64_2: i64 = 115i64;
    let mut str_0: &str = "6kPnA8zBWqE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = -65i64;
    let mut i64_4: i64 = 71i64;
    let mut i64_5: i64 = 56i64;
    let mut str_1: &str = "hT8aohrB";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 3i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_0);
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2942() {
    rusty_monitor::set_test_id(2942);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 35i64;
    let mut i64_1: i64 = 17i64;
    let mut i64_2: i64 = -185i64;
    let mut str_0: &str = "rTEDtS7";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = -59i64;
    let mut i64_4: i64 = -142i64;
    let mut i64_5: i64 = 147i64;
    let mut str_1: &str = "DXtyPjjUJ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut bool_2: bool = true;
    let mut i64_6: i64 = -107i64;
    let mut i64_7: i64 = 49i64;
    let mut i64_8: i64 = -92i64;
    let mut str_2: &str = "X6OVFYTqtA";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_2_ref_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_570() {
    rusty_monitor::set_test_id(570);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 56i64;
    let mut i64_1: i64 = 45i64;
    let mut i64_2: i64 = 14i64;
    let mut str_0: &str = "JNAgLUe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 119i64;
    let mut i64_4: i64 = -36i64;
    let mut i64_5: i64 = 34i64;
    let mut str_1: &str = "8I7PUEv64prz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1719() {
    rusty_monitor::set_test_id(1719);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 128i64;
    let mut i64_1: i64 = -131i64;
    let mut i64_2: i64 = -106i64;
    let mut str_0: &str = "lf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i64_3: i64 = 232i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_4: i64 = -115i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3677() {
    rusty_monitor::set_test_id(3677);
    let mut u32_0: u32 = 4u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 8u8;
    let mut i64_0: i64 = -13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_0: i32 = 135i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut bool_0: bool = true;
    let mut i64_1: i64 = -199i64;
    let mut i64_2: i64 = 110i64;
    let mut i64_3: i64 = 98i64;
    let mut str_0: &str = "tNa0Aa";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1595() {
    rusty_monitor::set_test_id(1595);
    let mut u16_0: u16 = 52u16;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 16u8;
    let mut u32_0: u32 = 48u32;
    let mut u8_3: u8 = 81u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 32u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i8_0: i8 = 45i8;
    let mut i8_1: i8 = -19i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_3: i8 = -56i8;
    let mut i8_4: i8 = -74i8;
    let mut i8_5: i8 = 108i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 160i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -78i64;
    let mut i64_3: i64 = 189i64;
    let mut i64_4: i64 = 60i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut u32_1: u32 = 98u32;
    let mut u8_6: u8 = 66u8;
    let mut u8_7: u8 = 33u8;
    let mut u8_8: u8 = 98u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut componentrange_1: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_0_ref_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    let mut month_0: month::Month = crate::month::Month::November;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut u16_0: u16 = 69u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 31i64;
    let mut i64_1: i64 = 223i64;
    let mut i64_2: i64 = 15i64;
    let mut str_0: &str = "YMT6omt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut f64_0: f64 = -214.695656f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut bool_1: bool = true;
    let mut i64_3: i64 = -203i64;
    let mut i64_4: i64 = 199i64;
    let mut i64_5: i64 = -120i64;
    let mut str_1: &str = "oUdF2vEQjmplR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut componentrange_2: crate::error::component_range::ComponentRange = std::clone::Clone::clone(componentrange_1_ref_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_0);
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_2: bool = std::cmp::PartialEq::ne(componentrange_2_ref_0, componentrange_0_ref_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2489() {
    rusty_monitor::set_test_id(2489);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 41i64;
    let mut i64_1: i64 = 6i64;
    let mut i64_2: i64 = -136i64;
    let mut str_0: &str = "iZBE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 0i64;
    let mut i64_4: i64 = -53i64;
    let mut i64_5: i64 = -241i64;
    let mut str_1: &str = "bqmV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut componentrange_1_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_1;
    let mut i8_0: i8 = 28i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = -15i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_1_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1434() {
    rusty_monitor::set_test_id(1434);
    let mut i64_0: i64 = -34i64;
    let mut i64_1: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_0: i32 = -36i32;
    let mut i64_2: i64 = 34i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut bool_0: bool = true;
    let mut i64_3: i64 = -13i64;
    let mut i64_4: i64 = 135i64;
    let mut i64_5: i64 = -1i64;
    let mut str_0: &str = "w5tEeY2OBQW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(componentrange_0_ref_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_1, duration_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4647() {
    rusty_monitor::set_test_id(4647);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_0: i64 = 41i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i8_0: i8 = 41i8;
    let mut i8_1: i8 = 71i8;
    let mut i8_2: i8 = 13i8;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = -23i64;
    let mut i64_2: i64 = -16i64;
    let mut i64_3: i64 = -132i64;
    let mut str_0: &str = "v5XUBEH4T4dn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4912() {
    rusty_monitor::set_test_id(4912);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 61i64;
    let mut i64_1: i64 = -66i64;
    let mut i64_2: i64 = -14i64;
    let mut str_0: &str = "SbciwGaVm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_3: i64 = -1i64;
    let mut i64_4: i64 = -292i64;
    let mut i64_5: i64 = 79i64;
    let mut str_1: &str = "SV6oW2rEdsYrN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut result_0: std::result::Result<crate::error::component_range::ComponentRange, crate::error::different_variant::DifferentVariant> = std::convert::TryFrom::try_from(error_0);
    let mut componentrange_2: crate::error::component_range::ComponentRange = std::result::Result::unwrap(result_0);
    let mut componentrange_2_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_2;
    let mut bool_2: bool = std::cmp::PartialEq::eq(componentrange_2_ref_0, componentrange_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3817() {
    rusty_monitor::set_test_id(3817);
    let mut i32_0: i32 = 76i32;
    let mut i64_0: i64 = -116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u16_0: u16 = 56u16;
    let mut i32_1: i32 = 66i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 85i64;
    let mut i64_2: i64 = 127i64;
    let mut i64_3: i64 = -132i64;
    let mut str_0: &str = "EeIcCQiZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut error_0: error::Error = std::convert::From::from(componentrange_0);
    let mut u8_0: u8 = crate::date::Date::iso_week(date_1);
    panic!("From RustyUnit with love");
}
}