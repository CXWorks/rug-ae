//! The `Month` enum and its associated `impl`s.

use core::convert::TryFrom;
use core::fmt;
use core::num::NonZeroU8;

use self::Month::*;
use crate::error;

/// Months of the year.
#[allow(clippy::missing_docs_in_private_items)] // variants
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    /// Create a `Month` from its numerical value.
    pub(crate) const fn from_number(n: NonZeroU8) -> Result<Self, error::ComponentRange> {
        match n.get() {
            1 => Ok(January),
            2 => Ok(February),
            3 => Ok(March),
            4 => Ok(April),
            5 => Ok(May),
            6 => Ok(June),
            7 => Ok(July),
            8 => Ok(August),
            9 => Ok(September),
            10 => Ok(October),
            11 => Ok(November),
            12 => Ok(December),
            n => Err(error::ComponentRange {
                name: "month",
                minimum: 1,
                maximum: 12,
                value: n as _,
                conditional_range: false,
            }),
        }
    }

    /// Get the previous month.
    ///
    /// ```rust
    /// # use time::Month;
    /// assert_eq!(Month::January.previous(), Month::December);
    /// ```
    pub const fn previous(self) -> Self {
        match self {
            January => December,
            February => January,
            March => February,
            April => March,
            May => April,
            June => May,
            July => June,
            August => July,
            September => August,
            October => September,
            November => October,
            December => November,
        }
    }

    /// Get the next month.
    ///
    /// ```rust
    /// # use time::Month;
    /// assert_eq!(Month::January.next(), Month::February);
    /// ```
    pub const fn next(self) -> Self {
        match self {
            January => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => January,
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            January => "January",
            February => "February",
            March => "March",
            April => "April",
            May => "May",
            June => "June",
            July => "July",
            August => "August",
            September => "September",
            October => "October",
            November => "November",
            December => "December",
        })
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month as _
    }
}

impl TryFrom<u8> for Month {
    type Error = error::ComponentRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match NonZeroU8::new(value) {
            Some(value) => Self::from_number(value),
            None => Err(error::ComponentRange {
                name: "month",
                minimum: 1,
                maximum: 12,
                value: 0,
                conditional_range: false,
            }),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::convert::TryFrom;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_554() {
//    rusty_monitor::set_test_id(554);
    let mut u8_0: u8 = 75u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 4u8;
    let mut u8_3: u8 = 84u8;
    let mut u8_4: u8 = 15u8;
    let mut u8_5: u8 = 23u8;
    let mut u8_6: u8 = 11u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 12u8;
    let mut u8_9: u8 = 52u8;
    let mut u8_10: u8 = 4u8;
    let mut u8_11: u8 = 30u8;
    let mut u8_12: u8 = 4u8;
    let mut u8_13: u8 = 9u8;
    let mut u8_14: u8 = 5u8;
    let mut u8_15: u8 = 23u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_15);
    let mut result_1: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_14);
    let mut result_2: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_13);
    let mut result_3: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_12);
    let mut result_4: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_11);
    let mut result_5: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_10);
    let mut result_6: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut result_7: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_8);
    let mut result_8: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_7);
    let mut result_9: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut result_10: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_5);
    let mut result_11: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_4);
    let mut result_12: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut result_13: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_2);
    let mut result_14: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_1);
    let mut result_15: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1096() {
//    rusty_monitor::set_test_id(1096);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut u16_0: u16 = 365u16;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 45i64;
    let mut month_1: month::Month = crate::month::Month::May;
    let mut i32_0: i32 = 78i32;
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut i64_2: i64 = 92i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_1: i32 = 189i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 60u16;
    let mut i32_2: i32 = -250i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i64_3: i64 = 1000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut duration_5_ref_0: &std::time::Duration = &mut duration_5;
    let mut i64_4: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_3: i32 = 285i32;
    let mut u32_1: u32 = 19u32;
    let mut u8_3: u8 = 37u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 31u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_2: month::Month = crate::month::Month::December;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut u8_6: u8 = crate::util::weeks_in_year(i32_3);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_1);
    let mut month_3: month::Month = crate::month::Month::next(month_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_3_ref_0, month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_280() {
//    rusty_monitor::set_test_id(280);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 93u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 60i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::May;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::September;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::February;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::March;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::November;
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut month_6: month::Month = crate::month::Month::April;
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::May;
    let mut month_7_ref_0: &month::Month = &mut month_7;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_7_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(month_6_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(month_5_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(month_4_ref_0);
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(month_3_ref_0);
    let mut tuple_5: () = std::cmp::Eq::assert_receiver_is_total_eq(month_2_ref_0);
    let mut tuple_6: () = std::cmp::Eq::assert_receiver_is_total_eq(month_1_ref_0);
    let mut tuple_7: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_404() {
//    rusty_monitor::set_test_id(404);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::October;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::October;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut i32_0: i32 = 381i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_3: month::Month = crate::date::Date::month(date_0);
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::May;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::April;
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut month_6: month::Month = crate::month::Month::March;
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::January;
    let mut month_8: month::Month = crate::month::Month::previous(month_7);
    let mut month_8_ref_0: &month::Month = &mut month_8;
    let mut month_9: month::Month = crate::month::Month::March;
    let mut month_9_ref_0: &month::Month = &mut month_9;
    let mut month_10: month::Month = crate::month::Month::May;
    let mut month_10_ref_0: &month::Month = &mut month_10;
    let mut month_11: month::Month = std::clone::Clone::clone(month_10_ref_0);
    let mut month_12: month::Month = std::clone::Clone::clone(month_9_ref_0);
    let mut month_13: month::Month = std::clone::Clone::clone(month_8_ref_0);
    let mut month_14: month::Month = std::clone::Clone::clone(month_6_ref_0);
    let mut month_15: month::Month = std::clone::Clone::clone(month_5_ref_0);
    let mut month_16: month::Month = std::clone::Clone::clone(month_4_ref_0);
    let mut month_17: month::Month = std::clone::Clone::clone(month_3_ref_0);
    let mut month_18: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut month_19: month::Month = std::clone::Clone::clone(month_1_ref_0);
    let mut month_20: month::Month = std::clone::Clone::clone(month_0_ref_0);
//    panic!("From RustyUnit with love");
}
}