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
#[timeout(30000)]fn rusty_test_268() {
//    rusty_monitor::set_test_id(268);
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 98u8;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 53u8;
    let mut u8_6: u8 = 53u8;
    let mut u8_7: u8 = 12u8;
    let mut u8_8: u8 = 11u8;
    let mut u8_9: u8 = 59u8;
    let mut u8_10: u8 = 6u8;
    let mut u8_11: u8 = 6u8;
    let mut u8_12: u8 = 7u8;
    let mut u8_13: u8 = 2u8;
    let mut u8_14: u8 = 29u8;
    let mut u8_15: u8 = 14u8;
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
#[timeout(30000)]fn rusty_test_4310() {
//    rusty_monitor::set_test_id(4310);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 11u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = 161i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i64_0: i64 = 2440588i64;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 353i32;
    let mut i64_2: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_3: u8 = 9u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut month_2: month::Month = crate::month::Month::November;
    let mut month_3: month::Month = crate::month::Month::next(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut month_4: month::Month = crate::month::Month::July;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut month_5: month::Month = crate::month::Month::next(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_594() {
//    rusty_monitor::set_test_id(594);
    let mut i8_0: i8 = 127i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::May;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::January;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::April;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::June;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::November;
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut month_6: month::Month = crate::month::Month::July;
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::March;
    let mut month_7_ref_0: &month::Month = &mut month_7;
    let mut month_8: month::Month = crate::month::Month::August;
    let mut month_8_ref_0: &month::Month = &mut month_8;
    let mut month_9: month::Month = crate::month::Month::June;
    let mut month_10: month::Month = crate::month::Month::next(month_9);
    let mut month_10_ref_0: &month::Month = &mut month_10;
    let mut month_11: month::Month = crate::month::Month::September;
    let mut month_11_ref_0: &month::Month = &mut month_11;
    let mut i32_0: i32 = 365i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_12: month::Month = crate::date::Date::month(date_1);
    let mut month_12_ref_0: &month::Month = &mut month_12;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_12_ref_0, month_11_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(month_10_ref_0, month_8_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(month_7_ref_0, month_6_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(month_5_ref_0, month_4_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(month_3_ref_0, month_2_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_696() {
//    rusty_monitor::set_test_id(696);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut u8_0: u8 = 53u8;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = 207i32;
    let mut i32_1: i32 = -111i32;
    let mut i32_2: i32 = 55i32;
    let mut i32_3: i32 = 3652425i32;
    let mut i32_4: i32 = 280i32;
    let mut i32_5: i32 = 1000000000i32;
    let mut i32_6: i32 = 133i32;
    let mut i32_7: i32 = 48i32;
    let mut i32_8: i32 = 99i32;
    let mut i32_9: i32 = -142i32;
    let mut i32_10: i32 = -140i32;
    let mut i32_11: i32 = 291i32;
    let mut i32_12: i32 = 7i32;
    let mut i32_13: i32 = 4i32;
    let mut i32_14: i32 = 1i32;
    let mut i32_15: i32 = 93i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_15};
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_14};
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_13};
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_12};
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_11};
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_10};
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_9};
    let mut date_7: crate::date::Date = crate::date::Date {value: i32_8};
    let mut date_8: crate::date::Date = crate::date::Date {value: i32_7};
    let mut date_9: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_10: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_11: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_12: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_13: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_14: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_15: crate::date::Date = crate::date::Date {value: i32_0};
    let mut month_1: month::Month = crate::month::Month::next(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3551() {
//    rusty_monitor::set_test_id(3551);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i64_1: i64 = 2440588i64;
    let mut i64_2: i64 = 86400i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 353i32;
    let mut i64_3: i64 = 1000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut u8_3: u8 = 9u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::next(weekday_3);
    let mut i32_1: i32 = crate::duration::Duration::subsec_nanoseconds(duration_4);
    let mut month_1: month::Month = crate::month::Month::November;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_3);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut month_3: month::Month = crate::month::Month::July;
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    let mut u8_4: u8 = crate::weekday::Weekday::number_from_sunday(weekday_1);
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_112() {
//    rusty_monitor::set_test_id(112);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f64_0: f64 = -19.236902f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 76u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 65u16;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 2u8;
    let mut u8_5: u8 = 52u8;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u16_1: u16 = 60u16;
    let mut i32_0: i32 = 100i32;
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_5, u8_4, u8_3, u16_0);
    let mut month_1: month::Month = crate::month::Month::March;
    let mut tuple_1: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_add(time_0, duration_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1899() {
//    rusty_monitor::set_test_id(1899);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 11u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = 161i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i64_0: i64 = 2440588i64;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 353i32;
    let mut i64_2: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_6: u8 = 9u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut i32_2: i32 = crate::duration::Duration::subsec_nanoseconds(duration_3);
    let mut month_1: month::Month = crate::month::Month::November;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut month_3: month::Month = crate::month::Month::July;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7022() {
//    rusty_monitor::set_test_id(7022);
    let mut i64_0: i64 = 1000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 88u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 9u8;
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut i64_1: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_2: i64 = 108i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 100000000u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 24u8;
    let mut u8_8: u8 = 10u8;
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = std::clone::Clone::clone(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1322() {
//    rusty_monitor::set_test_id(1322);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = 161i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 2440588i64;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 350i32;
    let mut i64_2: i64 = 999999i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_0: u8 = 9u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut i32_2: i32 = crate::duration::Duration::subsec_nanoseconds(duration_3);
    let mut month_1: month::Month = crate::month::Month::November;
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut month_3: month::Month = crate::month::Month::July;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Sunday;
//    panic!("From RustyUnit with love");
}
}
