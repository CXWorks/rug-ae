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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_676() {
    rusty_monitor::set_test_id(676);
    let mut i128_0: i128 = 83i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -31i8;
    let mut i8_1: i8 = -90i8;
    let mut i8_2: i8 = 73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -26i8;
    let mut i8_4: i8 = -6i8;
    let mut i8_5: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 85i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = 159i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = -120i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = 179i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i8_6: i8 = 68i8;
    let mut i8_7: i8 = -4i8;
    let mut i8_8: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 89.067537f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i64_4: i64 = 3i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut u32_0: u32 = 22u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = -73i8;
    let mut i8_10: i8 = -4i8;
    let mut i8_11: i8 = -113i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_0: i32 = -13i32;
    let mut i64_5: i64 = 158i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut u32_1: u32 = 45u32;
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 13u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = -11i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_15);
    let mut i8_12: i8 = 83i8;
    let mut i8_13: i8 = 10i8;
    let mut i8_14: i8 = 56i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_7: i64 = 104i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i64_8: i64 = -50i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_17);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_19);
    let mut i64_9: i64 = 66i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut i32_1: i32 = -66i32;
    let mut i64_10: i64 = -50i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds(i64_10);
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_23, i32_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_25: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut i64_11: i64 = 2i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_11);
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_26, duration_25);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_12: i64 = 179i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_12);
    let mut duration_29: std::time::Duration = crate::duration::Duration::abs_std(duration_28);
    let mut i8_15: i8 = -54i8;
    let mut i8_16: i8 = 45i8;
    let mut i8_17: i8 = -114i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut f64_1: f64 = 21.463842f64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::abs(duration_30);
    let mut duration_32: std::time::Duration = crate::duration::Duration::abs_std(duration_31);
    let mut f64_2: f64 = -123.019627f64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut i64_13: i64 = -62i64;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::hours(i64_13);
    let mut i128_1: i128 = -83i128;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_35, duration_34);
    let mut i64_14: i64 = -88i64;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_14);
    let mut duration_38: crate::duration::Duration = crate::duration::Duration::abs(duration_37);
    let mut i8_18: i8 = -10i8;
    let mut i8_19: i8 = 13i8;
    let mut i8_20: i8 = -33i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i128_2: i128 = 124i128;
    let mut duration_39: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut u32_2: u32 = 27u32;
    let mut u8_6: u8 = 29u8;
    let mut u8_7: u8 = 4u8;
    let mut u8_8: u8 = 49u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_21: i8 = 6i8;
    let mut i8_22: i8 = 51i8;
    let mut i8_23: i8 = 35i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i64_15: i64 = 88i64;
    let mut duration_40: crate::duration::Duration = crate::duration::Duration::weeks(i64_15);
    let mut u32_3: u32 = 42u32;
    let mut u8_9: u8 = 11u8;
    let mut u8_10: u8 = 53u8;
    let mut u8_11: u8 = 61u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i128_3: i128 = 81i128;
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_3);
    let mut i8_24: i8 = 35i8;
    let mut i8_25: i8 = 93i8;
    let mut i8_26: i8 = -21i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 52i8;
    let mut i8_28: i8 = 62i8;
    let mut i8_29: i8 = 83i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i64_16: i64 = 170i64;
    let mut duration_42: crate::duration::Duration = crate::duration::Duration::microseconds(i64_16);
    let mut i64_17: i64 = -209i64;
    let mut duration_43: crate::duration::Duration = crate::duration::Duration::days(i64_17);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_44: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut duration_45: crate::duration::Duration = crate::duration::Duration::abs(duration_44);
    let mut duration_46: std::time::Duration = crate::duration::Duration::abs_std(duration_45);
    let mut f64_3: f64 = -179.549394f64;
    let mut duration_47: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut duration_48: std::time::Duration = crate::duration::Duration::abs_std(duration_47);
    let mut i8_30: i8 = 59i8;
    let mut i8_31: i8 = 35i8;
    let mut i8_32: i8 = -126i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 75i8;
    let mut i8_34: i8 = -50i8;
    let mut i8_35: i8 = -127i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut u32_4: u32 = 20u32;
    let mut u8_12: u8 = 96u8;
    let mut u8_13: u8 = 1u8;
    let mut u8_14: u8 = 2u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i32_2: i32 = 54i32;
    let mut i64_18: i64 = 42i64;
    let mut duration_49: crate::duration::Duration = crate::duration::Duration::seconds(i64_18);
    let mut duration_50: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_49, i32_2);
    let mut f32_0: f32 = 99.539884f32;
    let mut duration_51: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_19: i64 = -24i64;
    let mut duration_52: crate::duration::Duration = crate::duration::Duration::days(i64_19);
    let mut i64_20: i64 = -102i64;
    let mut duration_53: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_20);
    let mut duration_54: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_53, duration_52);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_55: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut duration_56: crate::duration::Duration = crate::duration::Duration::abs(duration_55);
    let mut i32_3: i32 = -201i32;
    let mut i64_21: i64 = 25i64;
    let mut duration_57: crate::duration::Duration = crate::duration::Duration::new(i64_21, i32_3);
    let mut duration_58: std::time::Duration = crate::duration::Duration::abs_std(duration_57);
    let mut i64_22: i64 = -71i64;
    let mut duration_59: crate::duration::Duration = crate::duration::Duration::seconds(i64_22);
    let mut duration_60: crate::duration::Duration = crate::duration::Duration::abs(duration_59);
    let mut duration_61: std::time::Duration = crate::duration::Duration::abs_std(duration_60);
    let mut i8_36: i8 = -28i8;
    let mut i8_37: i8 = 23i8;
    let mut i8_38: i8 = -7i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_23: i64 = 44i64;
    let mut duration_62: crate::duration::Duration = crate::duration::Duration::minutes(i64_23);
    let mut i8_39: i8 = 86i8;
    let mut i8_40: i8 = -25i8;
    let mut i8_41: i8 = 4i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i64_24: i64 = 207i64;
    let mut duration_63: crate::duration::Duration = crate::duration::Duration::weeks(i64_24);
    let mut duration_64: std::time::Duration = crate::duration::Duration::abs_std(duration_63);
    let mut i64_25: i64 = 51i64;
    let mut duration_65: crate::duration::Duration = crate::duration::Duration::hours(i64_25);
    let mut duration_66: crate::duration::Duration = crate::duration::Duration::abs(duration_65);
    let mut duration_67: std::time::Duration = crate::duration::Duration::abs_std(duration_66);
    let mut i64_26: i64 = -29i64;
    let mut duration_68: crate::duration::Duration = crate::duration::Duration::hours(i64_26);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut f32_1: f32 = -154.318852f32;
    let mut duration_69: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_27: i64 = -144i64;
    let mut duration_70: crate::duration::Duration = crate::duration::Duration::weeks(i64_27);
    let mut u32_5: u32 = 57u32;
    let mut u8_15: u8 = 2u8;
    let mut u8_16: u8 = 53u8;
    let mut u8_17: u8 = 98u8;
    let mut i64_28: i64 = 106i64;
    let mut duration_71: crate::duration::Duration = crate::duration::Duration::minutes(i64_28);
    let mut i32_4: i32 = -78i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_71);
    let mut i64_29: i64 = -145i64;
    let mut duration_72: crate::duration::Duration = crate::duration::Duration::weeks(i64_29);
    let mut duration_73: crate::duration::Duration = crate::duration::Duration::abs(duration_72);
    let mut duration_74: std::time::Duration = crate::duration::Duration::abs_std(duration_73);
    let mut u32_6: u32 = 34u32;
    let mut u8_18: u8 = 99u8;
    let mut u8_19: u8 = 91u8;
    let mut u8_20: u8 = 34u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut u16_0: u16 = 89u16;
    let mut i32_5: i32 = -11i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_2, u8_17, u8_16, u8_15, u32_5);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    panic!("From RustyUnit with love");
}
}