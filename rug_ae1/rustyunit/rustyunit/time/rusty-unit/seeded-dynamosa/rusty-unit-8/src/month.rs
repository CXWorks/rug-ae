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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::TryFrom;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_129() {
//    rusty_monitor::set_test_id(129);
    let mut i8_0: i8 = -116i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = -21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 94u8;
    let mut u8_2: u8 = 71u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 320i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = 3600i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_2: i32 = 274i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    let mut date_3: crate::date::Date = crate::primitive_date_time::PrimitiveDateTime::date(primitivedatetime_3);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_636() {
//    rusty_monitor::set_test_id(636);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::November;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut f32_0: f32 = 3.917696f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 9u16;
    let mut i32_0: i32 = 274i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut month_3: month::Month = crate::month::Month::November;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::October;
    let mut month_5: month::Month = crate::month::Month::next(month_4);
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut month_6: month::Month = std::clone::Clone::clone(month_5_ref_0);
    let mut month_7: month::Month = std::clone::Clone::clone(month_3_ref_0);
    let mut month_8: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut month_9: month::Month = std::clone::Clone::clone(month_1_ref_0);
    let mut month_10: month::Month = std::clone::Clone::clone(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_530() {
//    rusty_monitor::set_test_id(530);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::October;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::March;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::August;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::December;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 85u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut month_5: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut month_6: month::Month = crate::month::Month::October;
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::August;
    let mut month_7_ref_0: &month::Month = &mut month_7;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_7_ref_0, month_6_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(month_5_ref_0, month_4_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(month_3_ref_0, month_2_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_531() {
//    rusty_monitor::set_test_id(531);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::September;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::November;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_5: month::Month = crate::date::Date::month(date_0);
    let mut month_6: month::Month = crate::month::Month::previous(month_5);
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::February;
    let mut month_7_ref_0: &month::Month = &mut month_7;
    let mut month_8: month::Month = crate::month::Month::January;
    let mut month_8_ref_0: &month::Month = &mut month_8;
    let mut month_9: month::Month = crate::month::Month::December;
    let mut month_9_ref_0: &month::Month = &mut month_9;
    let mut month_10: month::Month = crate::month::Month::February;
    let mut month_10_ref_0: &month::Month = &mut month_10;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_10_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(month_9_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(month_8_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(month_7_ref_0);
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(month_6_ref_0);
    let mut tuple_5: () = std::cmp::Eq::assert_receiver_is_total_eq(month_4_ref_0);
    let mut tuple_6: () = std::cmp::Eq::assert_receiver_is_total_eq(month_3_ref_0);
    let mut tuple_7: () = std::cmp::Eq::assert_receiver_is_total_eq(month_2_ref_0);
    let mut tuple_8: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_635() {
//    rusty_monitor::set_test_id(635);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::May;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_3: month::Month = crate::month::Month::July;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::September;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::November;
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut f32_0: f32 = 3.917696f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u16_0: u16 = 9u16;
    let mut i32_0: i32 = 274i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut month_6: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::November;
    let mut month_7_ref_0: &month::Month = &mut month_7;
    let mut month_8: month::Month = crate::month::Month::October;
    let mut month_9: month::Month = crate::month::Month::next(month_2);
    let mut month_8_ref_0: &month::Month = &mut month_8;
    let mut month_10: month::Month = std::clone::Clone::clone(month_8_ref_0);
    let mut month_11: month::Month = std::clone::Clone::clone(month_7_ref_0);
    let mut month_12: month::Month = std::clone::Clone::clone(month_0_ref_0);
    let mut month_13: month::Month = std::clone::Clone::clone(month_5_ref_0);
    let mut month_14: month::Month = std::clone::Clone::clone(month_4_ref_0);
    let mut month_15: month::Month = std::clone::Clone::clone(month_6_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_529() {
//    rusty_monitor::set_test_id(529);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::November;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut f32_0: f32 = 3.917696f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u16_0: u16 = 9u16;
    let mut i32_0: i32 = 274i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut month_3: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::November;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::October;
    let mut month_6: month::Month = crate::month::Month::next(month_5);
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = std::clone::Clone::clone(month_6_ref_0);
    let mut month_8: month::Month = std::clone::Clone::clone(month_4_ref_0);
    let mut month_9: month::Month = std::clone::Clone::clone(month_3_ref_0);
    let mut month_10: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut month_11: month::Month = std::clone::Clone::clone(month_1_ref_0);
    let mut month_12: month::Month = std::clone::Clone::clone(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_278() {
//    rusty_monitor::set_test_id(278);
    let mut u8_0: u8 = 63u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 60u8;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 12u8;
    let mut u8_6: u8 = 4u8;
    let mut u8_7: u8 = 5u8;
    let mut u8_8: u8 = 24u8;
    let mut u8_9: u8 = 59u8;
    let mut u8_10: u8 = 7u8;
    let mut u8_11: u8 = 8u8;
    let mut u8_12: u8 = 23u8;
    let mut u8_13: u8 = 11u8;
    let mut u8_14: u8 = 11u8;
    let mut u8_15: u8 = 53u8;
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
}